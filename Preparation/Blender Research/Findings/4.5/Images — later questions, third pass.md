# Images — later questions, third pass (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual (`Blender Sources/manual_4.5`), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), the 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All four are a snapshot taken **2026-07-30**, which is the date read for everything quoted below. I did not use the web.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project. One qualification so the line is exactly true: to confirm the folder I was told to write into exists, I listed the file *names* in `Preparation/Blender Research/Findings/4.5/`. I opened none of them, and the names carry no claims about Blender. I did not open the two earlier findings documents named in my inputs, nor `CLAUDE.md`, nor `Start Here.md`, nor anything they point to.

Only question 9 was assigned. The rest of the sheet is untouched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

### The reading I answered, and the other one

The question is ambiguous in one place, so I say plainly which way I took it.

**The reading I answered:** a blend-file refers to an image file on disk, that file is not there, and the image texture's output is wired to an input that is not a colour input — roughness, a normal, a displacement height, an alpha, and so on.

**The other reading:** "feeding something other than colour" means the image's own **Color Space** is set to *Non-Color* — the setting used for maps that hold numbers rather than colours.

As it happens the finding below settles both, because the substitution Cycles makes for a missing image happens *before* any colour-space handling and *before* the value reaches whatever it feeds. So the same answer covers either reading. I say where that is visible in the quotes.

### LABEL: observed (the substituted value and where it is substituted) + inferred (what that value does once it reaches a non-colour input) + not in the sources (the manual and the API say nothing at all about this)

### The core of it

Cycles does not have a "missing" state that travels down the node tree. When the image behind an Image Texture or Environment Texture node is not there, the node hands out a **fixed magenta colour with alpha 1** — red 1, green 0, blue 1, alpha 1 — and that is all any downstream node ever sees. Nothing further along knows the image was missing. So a missing image feeding roughness, or a normal map, or a displacement height, does not produce a warning stripe, a black patch, or a fallback to the socket's own default value: it produces whatever those three numbers happen to mean to the thing they were plugged into, applied evenly across every point the texture covers.

### What the files say, exactly

**1. The colour Cycles substitutes, and the comment that names its purpose.**

File: `cycles_4.5/source/util/texture.h`, lines 11–17.

```
/* Color to use when textures are not found. */
enum {
  TEX_IMAGE_MISSING_R = 1,
  TEX_IMAGE_MISSING_G = 0,
  TEX_IMAGE_MISSING_B = 1,
  TEX_IMAGE_MISSING_A = 1
};
```

Read on 2026-07-30. (Cycles' own files have no web address; cited by file and line.)

**2. Where it is substituted — at the texture lookup itself, before anything else the node does.**

File: `cycles_4.5/source/kernel/svm/image.h`, lines 20–41.

```
ccl_device float4
svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
{
  if (id == -1) {
    return make_float4(
        TEX_IMAGE_MISSING_R, TEX_IMAGE_MISSING_G, TEX_IMAGE_MISSING_B, TEX_IMAGE_MISSING_A);
  }

  float4 r = kernel_tex_image_interp(kg, id, x, y);
  const float alpha = r.w;

  if ((flags & NODE_IMAGE_ALPHA_UNASSOCIATE) && alpha != 1.0f && alpha != 0.0f) {
    r /= alpha;
    r.w = alpha;
  }

  if (flags & NODE_IMAGE_COMPRESS_AS_SRGB) {
    r = color_srgb_to_linear_v4(r);
  }

  return r;
}
```

Read on 2026-07-30. Two things are visible here and both matter.

- The missing case returns straight away, on line 24. It never reaches the alpha handling on line 31 or the colour-space conversion on line 36. **This is what settles the second reading of the question:** the substituted magenta is identical whether the image's Color Space is sRGB or *Non-Color*, because the colour-space step is skipped entirely.
- The function takes the image `flags` but ignores them in the missing case. There is no branch anywhere in it for "this image is being used as data rather than as colour".

**3. Where those numbers go — the node's two outputs.**

File: `cycles_4.5/source/kernel/svm/image.h`, lines 119–126 (inside `svm_node_tex_image`).

```
  const float4 f = svm_image_texture(kg, id, tex_co.x, tex_co.y, flags);

  if (stack_valid(out_offset)) {
    stack_store_float3(stack, out_offset, make_float3(f.x, f.y, f.z));
  }
  if (stack_valid(alpha_offset)) {
    stack_store_float(stack, alpha_offset, f.w);
  }
```

Read on 2026-07-30. So for a missing image the node's **Color** output is (1, 0, 1) and its **Alpha** output is **1** — fully opaque. If the missing image was feeding an Alpha or transmission input, what arrives there is a solid 1.

**4. The same substitution serves the Environment Texture node.**

File: `cycles_4.5/source/kernel/svm/image.h`, line 263 (inside `svm_node_tex_environment`):

```
  const float4 f = svm_image_texture(kg, id, uv.x, uv.y, flags);
```

Read on 2026-07-30. The Box projection path in the same file also routes through `svm_image_texture` (lines 218, 222 and 226, once per blended side).

**5. There is exactly one such handler in the whole kernel.** Searching every file under `cycles_4.5/` for `TEX_IMAGE_MISSING` returns two files only: the four lines that define it in `util/texture.h`, and the one use of it in `kernel/svm/image.h` line 25. Searching the same tree for `id == -1` returns one hit, line 23 of `kernel/svm/image.h`. So there is no second, different fallback for images used as data. Observed, 2026-07-30.

### What happens next, once magenta reaches a non-colour input

Everything in this section is **inferred**. The step I am taking each time is the same one: the missing image's output is the fixed value (1, 0, 1) with alpha 1, so I read the quoted arithmetic of the receiving node with those numbers put in. The files state the arithmetic; they do not state this outcome.

**Into a plain number input (roughness, metallic, a mix factor, a bump height, an IOR).** The manual states the conversion:

> "Between color and float – the color data is converted to its grayscale equivalent."

Source: manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/parts.html — read 2026-07-30.

Cycles does it like this — file `cycles_4.5/source/kernel/svm/convert.h`, lines 33–37:

```
    case NODE_CONVERT_CF: {
      const float3 f = stack_load_float3(stack, from);
      const float g = linear_rgb_to_gray(kg, f);
      stack_store_float(stack, to, g);
      break;
    }
```

and `cycles_4.5/source/kernel/util/colorspace.h`, lines 34–37:

```
ccl_device float linear_rgb_to_gray(KernelGlobals kg, const float3 c)
{
  return dot(c, make_float3(kernel_data.film.rgb_to_y));
}
```

Both read 2026-07-30. Inferred outcome: the input receives **one constant number, the same at every point the texture covered**, being the scene's luminance weighting applied to (1, 0, 1) — that is, the red weight plus the blue weight, with green contributing nothing. The surface therefore looks uniformly rough (or uniformly metallic, or whatever the input was) rather than showing the pattern the map was meant to carry. I deliberately do not give the number: the weights come from `kernel_data.film.rgb_to_y`, which is scene colour-management data and is not in the files I was given.

**Into a Normal Map node.** File `cycles_4.5/source/kernel/svm/tex_coord.h`, lines 347–348:

```
  float3 color = stack_load_float3(stack, color_offset);
  color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);
```

Read 2026-07-30. Inferred outcome: (1, 0, 1) decodes to the tangent-space direction (1, −1, 1) — a strongly tilted normal rather than the (0, 0, 1) a flat area of a real normal map would give. Once normalised and put into world space (lines 394–396 of the same file), every point of the surface gets the *same* tilted normal, so the surface shades as though it were uniformly canted away from its true orientation. It does not go flat, and it does not go black.

**Into a Displacement node's Height.** The colour first becomes one constant number as above, then file `cycles_4.5/source/kernel/svm/displace.h`, lines 151–168:

```
    const float height = stack_load_float(stack, height_offset);
    const float midlevel = stack_load_float(stack, midlevel_offset);
    const float scale = stack_load_float(stack, scale_offset);
...
      dP *= (height - midlevel) * scale;
```

Read 2026-07-30. Inferred outcome: because `height` is the same everywhere, the surface is pushed along its normal by a single constant amount — an even swell or shrink with no relief in it. Whether it swells or shrinks depends on that constant against the node's Midlevel, which I cannot pin down from these files.

**Into an alpha or transparency input.** Alpha comes out as 1 (quote 3 above), so an object that should have been cut out by an alpha map stays whole and opaque. Inferred from the stored value.

**Does the render stop, or report?** In the quoted code the missing case returns a value and shading continues; there is no error branch, no counter, and nothing written back. Inferred: the render runs to completion and produces a picture. I searched Cycles' Python files (`cycles_4.5/*.py`) for `report` and `Error`; every hit concerns Open Shading Language compilation, the denoise operator and baking — none concerns a missing image. I found nothing anywhere in the sources saying a missing image is reported during a render.

### What the sources do *not* say, and where I looked for it

This is the part that must be on the record, so it is set out in full.

**Neither the manual nor the Python API describes what a render produces for a missing image texture. Not anywhere.**

Searched across the whole of `manual_4.5` (case-insensitive):

- `missing` — 43 hits, none about image textures in materials. The relevant-looking ones: the topbar's *Report Missing Files* and *Find Missing Files*, which describe finding broken links, not rendering them (quoted below); the Video Sequencer's *Show Missing Media* (ruled out, see below); linked-library placeholder data-blocks; the Video Sequencer's image-strip placeholders.
- `magenta`, `pink` — 22 hits. Every one is either interface colouring (socket colours, selection highlights, node-editor colours), anaglyph glasses, or a colour-balance control. The single behavioural hit is the Video Sequencer one.
- `purple` — 8 hits, all interface colouring.
- `placeholder` — 19 hits: glTF export, the render output *Placeholders* option, empties, geometry-nodes string formatting, linked libraries, sequencer image strips. None about textures.
- `not found`, `cannot be found`, `can't be found`, `unable to open`, `unable to load`, `unable to find`, `fails to load`, `failed to load`, `broken link`, `invalid path` — the only behavioural hits are the Attribute node ("If a suitable property is not found, all sockets of the node, including Alpha, output 0") and Dynamic Paint layers, neither of which concerns images.
- `does not exist`, `no longer exist`, `moved or deleted`, `fallback`, `falls back`, `substitut`, `default color` — restricted to `manual_4.5/render`, `manual_4.5/files` and `manual_4.5/editors/image`. Nothing about a missing image.

Pages I opened in full and ruled out:

- **Image Texture Node**, https://docs.blender.org/manual/en/4.5/render/shader_nodes/textures/image.html — read 2026-07-30. Describes Vector, Image, Interpolation, Projection, Extension, Source, Frames, Color Space, Alpha, and the two outputs. It says of the outputs only: "Color — RGB color from image" and "Alpha — Alpha channel from image." It says nothing about the image being absent. Its pictures (`node-types_ShaderNodeTexImage.webp`, the four projection examples, the example render) show a working texture; none shows a missing one, so they add nothing here.
- **Node Parts**, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/parts.html — read 2026-07-30. Used for the colour-to-float and colour-to-vector conversion rules quoted above. Nothing about missing data.
- **Topbar → File → External Data**, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30. This is the nearest the manual comes, and it is about repair, not rendering: "This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files." Nothing about what a render does in the meantime.
- **Video Sequencer preview sidebar**, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read 2026-07-30. This is the *only* place in the manual that documents a magenta stand-in: "Show Missing Media — Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." **I am ruling this out as an answer.** It is a Video Sequencer switch for strips, a different part of Blender from shading, it is optional where the Cycles substitution is not, and it has an off state that makes missing media transparent. The matching API entry says the same thing (`SequenceEditor.show_missing_media`, https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html, read 2026-07-30). Nothing suggests it governs image textures in materials.
- **Normal Map Node**, https://docs.blender.org/manual/en/4.5/render/shader_nodes/vector/normal_map.html and **Color Management**, https://docs.blender.org/manual/en/4.5/render/color_management.html — read 2026-07-30, checked for anything tying *Non-Color* images to missing-file behaviour. They describe when to use *Non-Color* and nothing about absent files.

Searched across the whole of `api_4.5`:

- `missing` — the hits are `ID.is_missing` (about *linked data-blocks*, not image files: "This data-block is a place-holder for missing linked data"), the four `ImagePaint.missing_*` flags (texture-paint readiness warnings, not rendering), and `bpy.ops.file.find_missing_files` / `bpy.ops.file.report_missing_files` ("Try to find missing external files" / "Report all missing external files"). None says what a render produces.
- `magenta`, `pink` — only the sequencer property and the anaglyph enum.
- `bpy.types.Image` — read 2026-07-30, https://docs.blender.org/api/4.5/bpy.types.Image.html. It offers `has_data` ("True if the image data is loaded into memory"), which tells a script whether pixels are present, but the page says nothing about render behaviour.

Searched across the whole of `release_notes_4.5` for `missing`, `magenta`, `pink`, `placeholder`. Nothing on point. The only image-adjacent entries are bug-fix list lines — "Internal mode in IES texture node flagged in missing file report" and "Placeholder output files wrong name" — neither of which concerns what a render produces for a missing image texture.

**Things I could not settle at all from what I was given:**

- **EEVEE and Workbench.** The only renderer whose own files I was given is Cycles. There is no Blender or EEVEE source in `Blender Sources/`, and the manual and API are silent, so I cannot say what EEVEE puts in place of a missing image, nor whether it matches Cycles' magenta. This is a real gap and I will not guess at it.
- **The Open Shading Language path in Cycles.** `cycles_4.5/shader/` holds compiled shader objects (`.oso`), not their source. `node_image_texture.oso` shows the shader calling OSL's own `texture()` (line 505, referencing `node_image_texture.osl:67`), and `cycles_4.5/source/kernel/osl/services_gpu.h` lines 1380 and 1385 declare `osl_texture_set_missingcolor_arena` and `osl_texture_set_missingcolor_alpha` — but nothing in the files I have shows what missing colour, if any, is set. So I cannot say whether a scene rendered with Open Shading Language turned on gives the same magenta.
- **How an image comes to be "missing" as far as the kernel is concerned.** The quoted code reacts to an image slot of −1. The code that decides a file could not be loaded and hands over −1 lives in Cycles' scene-side files, which are not in `cycles_4.5/source/` (only `kernel/` and `util/` are present). So I have the consequence, not the trigger.
- **The exact grey value** a missing image becomes when it feeds a number input, for the reason given above.
- **Volumes.** `cycles_4.5/source/kernel/svm/voxel.h` — the Volume/voxel lookup — has no missing-image branch at all, unlike the 2D path. I note this only to mark the edge of my answer; a missing volume grid is not an image file pointed at by an image texture, and I did not chase it.

### Not covered

This answer settles what Cycles' render kernel substitutes for a missing image, that it substitutes it at the texture node rather than at whatever the node feeds, and that the substitution is the same whatever the image was going to be used for. It does not settle: what EEVEE or Workbench do; what happens under Open Shading Language; whether anything is written to a log, the Info editor, or the process's output when a render meets a missing image; whether the render's exit status changes; whether a missing image affects any pass other than the shading it feeds; the precise numeric outcome at a number input; or what happens to a missing image behind a legacy Texture data-block (a Displace modifier, a force field) as opposed to a shader node.

### Outside the interface

The substitution sits in Cycles' render kernel — the shading code that runs for every sample — and there is nothing in the quoted lines that consults the interface, a window, a preference, or a user action. **Inferred:** a render started and driven from outside Blender's own window produces exactly the same magenta stand-in, and the same downstream consequences, as one started by a person clicking Render. The step I took: the code path from `svm_node_tex_image` through `svm_image_texture` has no interface dependency in it.

One difference is worth naming and it is not about the picture. The two documented ways of learning that files are missing — *Report Missing Files*, which "will appear in the Info editor's header", and *Find Missing Files*, which "A File Browser will show up" — are both interface actions, and the second is interactive by description. Their Python forms exist (`bpy.ops.file.report_missing_files`, `bpy.ops.file.find_missing_files`), but the sources do not say where the report goes when there is no Info editor on screen. So: the picture is the same, and the sources do not say whether the notice is.

**Settled further by hand test, 2026-08-09 — observed.** The three tests in the *Hand-test results, second batch* section below were all run with Blender driven from a terminal, with no window at all, on both computers — including the rented Linux machine, which has no display server of any kind. Every one of them finished, wrote its frame and gave the numbers recorded there. Workbench, which is the viewport's own renderer, rendered there too. So what a render puts on the surface for a missing file does not depend on a window being open.

### Operating system

The missing-colour constant is in `util/texture.h` and the branch that uses it is in `kernel/svm/image.h`; both are shared kernel files with no operating-system condition anywhere near the quoted lines, and they are compiled into every device back-end in `cycles_4.5/source/kernel/device/` (CUDA, HIP, HIP-RT, Metal, OptiX, and the CPU path). **Inferred:** the behaviour is the same on every operating system Blender runs on, and on graphics cards as well as processors. The step I took: one shared constant, one shared branch, no platform-conditional code around either. The sources do not state this in words.

**Settled further by hand test, 2026-08-09 — observed.** All three tests in the *Hand-test results, second batch* section below were run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every one. Where any figure differed it differed in its last digits only, and each such difference is given where it occurs. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Hand-test results, second batch — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md` under the heading *Added from the citation check — a second batch*; the entry numbers below are that list's numbers.

These three are a second batch. The topic's first fourteen hand tests were run on the same day and are written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`; they are not repeated here. The citation check found three of those fourteen recorded too thinly for anybody to repeat and wrote out what each should have been. Those three are entries 15, 16 and 17.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before that run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, so pictures were compared as pixels and never as files. "Missing" below always means the file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 4.5.12 LTS, from `/opt/blender/4.5/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 4.5.12 LTS.

### 15. A missing picture feeding a normal map, a bump height and a displacement height, rebuilt so it can be repeated (entry 15)

> **Why there are two of these.** The citation check found that hand-test result 2 — which is written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`, not here — cannot be repeated from its own words and wrote out a test with every measurement in it. That test is entry 15 in the hand-test list, and this is what it gave. **That earlier result is untouched and its figures stand.** This is a second run over the same ground, not a correction of it, and the two set-ups are different so the figures are not meant to match.

**What was done.** A 64 by 64 black-and-white checker of 8-pixel squares, colour space `Non-Color`. Three UV spheres of 128 segments by 64 rings, radius 0.4, shaded smooth, on a black world, lit by one sun lamp at angle 0, energy 3, turned 45 degrees about X and 30 degrees about Z. An orthographic camera at (0, 0, 5). The frame 240 by 80 with a transparent film, Cycles at 4 samples with denoising off and every bounce count set to 0, output OpenEXR at full float. Each sphere carries a Diffuse BSDF with colour 0.5, 0.5, 0.5 and roughness 0. The left sphere takes the picture through a Normal Map node into the Normal input; the middle one through a Bump node's Height with Distance 5; the right one through a Displacement node's Height at Scale 0.4 into the Material Output, with the material's displacement method set to `DISPLACEMENT`.

Four renders of the same file, not three: **present** (picture there, everything wired), **missing** (picture moved out of the folder), **unwired** (the link out of each Image Texture node removed), and **nonode** (as unwired, and the Normal Map, Bump and Displacement node taken out of the chain as well, so each sphere is plain and untouched).

The fourth render is there because of something the run itself printed: the Normal Map node's own default Colour is **0.5, 0.5, 1**. So *unwired* is not "no normal map" — it is "a normal map holding 0.5, 0.5, 1", and without a plain sphere to compare against, nothing in the run fixes what "unchanged" means. The Displacement node's own default Height printed as **0.0** and its Midlevel as **0.5**; the Bump node's own default Height printed as **1.0**.

**What came back, first — with the framing exactly as it was written out.** The check's framing puts the three spheres at x = −1, 0, +1 in an orthographic width of 3. Run that way, the displaced sphere runs off the picture. Counting lit pixels on each edge row of the frame, the picture-present render gave top 51, bottom 52, left 0, right 52, on both computers; the other three renders touched no edge at all. In the same render the middle third of the frame held 3469 lit pixels against 3217 in every other state, because part of the displaced sphere had grown into its neighbour's third. The displaced sphere's own third counted 5878 present, 1968 missing, 800 nothing-connected, 3207 plain, and those counts are cut short by the frame edge rather than measured. So the figures from that framing are recorded here as what happened and are not used to answer anything.

**What came back, second — with the framing widened.** Two numbers were changed and nothing else: the orthographic width to 4.5 and the spheres to x = −1.5, 0, +1.5. Nothing touched any edge of the frame in any of the four renders, on either computer. The figures below are the mean and the largest single-channel difference over each third of the frame; the two computers agreed except in the last digits of one mean.

```
DIFF normalmap  present_vs_unwired mean 0.039300  max 0.476329
DIFF normalmap  missing_vs_unwired mean 0.031217  max 0.385979
DIFF normalmap  unwired_vs_nonode  mean 0.000000  max 0.000000
DIFF normalmap  missing_vs_nonode  mean 0.031217  max 0.385979
DIFF bump       present_vs_unwired mean 0.010682  max 0.474189
DIFF bump       missing_vs_unwired mean 0.000001  max 0.000035
DIFF bump       unwired_vs_nonode  mean 0.000000  max 0.000000
DIFF displace   present_vs_unwired mean 0.095682  max 0.476674
DIFF displace   missing_vs_unwired mean 0.020101  max 0.476927
DIFF displace   unwired_vs_nonode  mean 0.040953  max 0.477084
DIFF displace   missing_vs_nonode  mean 0.021028  max 0.470619
SILHOUETTE normalmap  present 1416  missing 1416  unwired 1416  nonode 1416
SILHOUETTE bump       present 1415  missing 1415  unwired 1415  nonode 1415
SILHOUETTE displace   present 3045  missing 868  unwired 348  nonode 1424
```

That block is the Creator's Mac. The rented Linux machine gave `present_vs_unwired mean 0.039284` for the normal map where the Mac gave 0.039300, and every other figure above to the digit printed. The rented Linux machine's one other difference was in the bump group, `missing_vs_unwired max 0.000046` instead of `0.000035`.

**The controls.** `present_vs_unwired` in each group is the first control: each of the three wirings changes the picture when the file is really there. `unwired_vs_nonode` for the normal map is the second: a Normal Map node fed 0.5, 0.5, 1 gives a picture identical to no Normal Map node at all, mean 0.000000 and largest 0.000000, so "unchanged" has a fixed meaning in this run.

**So, with the picture missing:**

- **The normal map.** The surface is not left as it was. The missing-picture render differs from the plain sphere by mean 0.031217 and largest 0.385979 of a channel, against a control that shows what "identical" looks like here — exactly zero.
- **The bump height.** The surface is left as it was. The largest single-channel difference anywhere on that sphere between the missing-picture render and the nothing-connected render was 0.000035, against 0.474189 for the control.
- **The displacement height.** The sphere gets smaller. Its outline held 1424 pixels untouched, 868 with the picture missing, 348 with nothing connected to the Displacement node, and 3045 with the picture present. The other two spheres, which are not displaced, held 1416 and 1415 in every state.

### 16. Workbench with a missing picture, with the two settings it needs (entry 16)

> **Why there is a second one.** The citation check found that the Workbench half of hand-test result 5 — written up in this version's first and second pass documents, not here — cannot produce its own numbers from its own words, and wrote the test out again with the two missing settings in it. That test is entry 16 in the hand-test list. The earlier result is untouched.

**What was done.** The `col` blend-file from hand-test result 1, built again by the same script — a plane filling an orthographic camera, an Emission-only material, the Image Texture node's Colour into Emission Colour, 32 by 32 pixels, OpenEXR at full float, view transform `Standard`. The render engine was set to `BLENDER_WORKBENCH` and the frame rendered twice in the same run: once with nothing else touched, and once with `scene.display.shading.color_type` set to `TEXTURE` and `scene.display.shading.light` set to `FLAT`. Both were run with the picture present and again with it moved out of the folder. Four renders. The two settings were printed before and after each change.

**What came back.** Both settings read `MATERIAL` and `STUDIO` before the change, in every version on both computers.

The Creator's Mac:

```
RESULT bare     present [0.357666015625, 0.366943359375, 0.37060546875, 0.99755859375]
RESULT bare     missing [0.357666015625, 0.366943359375, 0.37060546875, 0.99755859375]
RESULT textured present [0.0, 0.99755859375, 0.0, 0.99755859375]
RESULT textured missing [0.99755859375, 0.0, 0.99755859375, 0.99755859375]
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

Then the scene, with every number recorded: a Volume data-block pointing at `//vol.vdb`, on an object at location (0, 0, 0), rotation 0, scale (1, 1, 1); a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; the frame 64 by 64 with the film **not** transparent; a world data-block with its nodes switched off and its colour black; Cycles at 128 samples with denoising off. With the file loaded, the object's dimensions read (3.5938, 3.5938, 4.8438). In 4.5 the world's nodes really were switched off, so the empty frame is black: mean 0.0 and largest 0.0.

Three renders: file present; the Volume object deleted altogether; the file moved out of the folder.

**What came back, the same on both computers**, as the mean and the largest pixel value over the frame:

```
RESULT present   mean 0.5932814478874207  max 2.500030279159546
RESULT novolume  mean 0.0  max 0.0
RESULT missing   mean 0.0  max 0.0
```

**So:** the file-present render against the object-deleted render is the control, and it holds — the volume plainly changes the frame when its file is there. With the file gone, the frame matched the render with no Volume object at all, to every digit printed, on both computers. Searching the whole of every run's output for `ERROR`, `WARNING`, `Error` and `Traceback` matched no line, in either state, on either computer. A missing volume file draws nothing and says nothing.

### What this second batch did not settle

- **Windows.** No Windows machine has run it. The three results above settle macOS on Apple silicon and Linux on an NVIDIA card, and leave Windows untested rather than answered.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Each of these closes a gap named above. They are written so somebody who cannot see my reasoning can carry them out.

1. **Confirm the magenta stand-in in a Cycles render.** New file. Add a plane facing the camera and a light. Give the plane a Principled BSDF. Add an Image Texture node, open any image file, save the blend, quit Blender, rename or move that image file on disk, reopen the blend. Wire the Image Texture's **Color** to **Base Color** and render with Cycles. Then use the render's colour picker (or save it as OpenEXR and read a pixel) on the plane. Expect red 1, green 0, blue 1. Note whether it is exactly 1/0/1 or whether view transform has shifted it — set the view transform to *Standard* first to avoid that confusion.

2. **Confirm nothing changes when the image is not feeding colour.** Same file. Move the wire from **Base Color** to **Roughness** and render. Read whether the plane is uniformly rough (one constant value everywhere) rather than showing any pattern. Then move the wire to **Metallic**, then through a **Normal Map** node into **Normal**, then through a **Displacement** node into the material output's **Displacement**, rendering each time. In every case check that the result is uniform across the plane, with no magenta patch and no warning marking, and record what each looks like. **The Normal Map, Bump and Displacement parts of this are answered** — see result 15 in the second batch of hand tests below, run 2026-08-09 on both computers. The Roughness and Metallic parts are not covered there.

3. **Read the actual grey value.** With the missing image's Color output going into an **Emission** shader's Strength (or straight into a Value-to-Color chain you can view), render and read the pixel. That gives the number the colour-to-float conversion produces in that scene, which my sources could not supply.

4. **Check the Color Space setting makes no difference.** With the wire in **Base Color**, switch the missing Image Texture node's Color Space between *sRGB* and *Non-Color* and render each way. The two renders should be identical. If they are not, my reading of the early return in `svm_image_texture` is wrong and should be reported.

5. **Settle EEVEE and Workbench.** Repeat step 1 and step 2 with the render engine set to EEVEE, then to Workbench. Record the colour each produces, since my sources say nothing about either. **The Workbench half is answered** — see result 16 in the second batch of hand tests below, run 2026-08-09 on both computers. Workbench needs its colour set to Texture and its lighting set to Flat before it shows a picture at all; with those set it gives the picture's own colour when the file is there and magenta when it is not. The EEVEE half is not covered there.

6. **Settle Open Shading Language.** In Cycles, turn on Open Shading Language in Render Properties (CPU, and again on OptiX if a suitable card is available) and repeat step 1. Record whether the colour is still magenta.

7. **Find out whether anything is said out loud.** Render the missing-image file from a terminal with `blender -b thefile.blend -f 1` and capture everything the process prints, plus its exit code. Do the same again from inside the interface and look at the Info editor and the system console. Record whether the missing image is named anywhere, and whether the exit code differs from a clean render.

8. **Check the Alpha output.** Wire the missing Image Texture's **Alpha** into the Principled BSDF's **Alpha**, set the material to use alpha, and render over a contrasting background. Expect the plane to stay fully opaque, which is what an Alpha of 1 would give.

9. **Check a legacy texture user.** Add a Displace modifier to a subdivided plane, give it a Texture data-block whose image is the missing file, and render. Record whether the geometry moves, stays put, or shows something else — my sources cover only shader nodes.

---

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 4.5

# Images — later questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual, Blender 5.1 Python API reference, Blender 5.1 release notes, and Cycles' own files as shipped with 5.1 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is my date read for everything cited below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 9 was assigned to me. The rest of the sheet is untouched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

> **Changed by the citation check, 2026-08-09 (Opus 5).** Seven things were changed, each marked *[citation check]* where it sits: one address corrected (the displacement lines); two claims about what a flat normal map and the usual Midlevel would be, cut, because no source here carries either; "the weights sum to one" narrowed; "not zero and not one" cut; and two places where a step from the code's stack offsets and flags to Blender's own settings and sockets was presented as needing no step. Three hand-test entries were annotated — see entries 2, 5 and 14.

### The question was ambiguous; here is the reading I answered

I read "feeding something other than colour" as: **the missing image's output is wired into an input that is not a colour** — roughness, metallic, bump height, displacement height, a normal map, and so on.

The other reading available is: **the image is marked as data rather than colour** (its Colour Space set to "Non-Color"). As it happens, the same lines of Cycles settle that reading too, and I say so below, because the substitute value is produced *before* any colour-space handling runs.

### Short answer

Cycles replaces every look-up of an image it cannot find with one fixed value: red 1, green 0, blue 1, alpha 1 — opaque magenta. It does this inside the texture node itself, before anything downstream sees it. So a missing image feeding a non-colour input does **not** put a magenta patch in the picture. It puts that one fixed value into whatever the input drives, identically at every point of the surface, and the picture shows whatever that constant does.

Because the value is the same everywhere, anything that works off *variation* in the image produces nothing at all, while anything that works off the *level* is pushed to a fixed setting.

The manual, the Python API reference and the 5.1 release notes are silent on all of this. Nothing in them says what a render produces for a missing image in a material, and nothing says whether EEVEE behaves the same way. I searched hard for it; the record is below.

### LABEL

Mixed, and split deliberately:

- **observed** — the substitute value, where it is produced, and that it is produced before colour-space handling. Read out of Cycles' own files.
- **inferred** — everything about what that value then *does* to a non-colour input. Each step is written out.
- **not in the sources** — the manual, the API reference and the release notes say nothing about it; EEVEE is not covered; nothing says a warning is raised or a render is stopped.

### What Cycles' own files say (observed)

**The substitute value, and the comment that names its purpose.**

File: `Blender Sources/cycles_5.1/source/util/types_image.h`, lines 12–13:

```
/* Color to use when images are not found. */
#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
```

Read: 2026-07-30. The four numbers are red, green, blue, alpha.

**Where it is substituted — the Image Texture node's look-up.**

File: `Blender Sources/cycles_5.1/source/kernel/svm/image.h`, lines 21–41:

```
ccl_device float4
svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
{
  if (id == -1) {
    return IMAGE_MISSING_RGBA;
  }

  float4 r = kernel_image_interp(kg, id, x, y);
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

Read: 2026-07-30.

Two things are visible in those lines. First, the substitution happens at the very top and returns immediately. Second, the two adjustments guarded by `NODE_IMAGE_ALPHA_UNASSOCIATE` and `NODE_IMAGE_COMPRESS_AS_SRGB` sit *after* that return, so neither touches the substituted value. *[citation check: this said the reader need supply nothing, and then named the two flags as the image's Alpha setting and its Colour Space setting. Nothing quoted here ties either flag to a setting a person can change, so that is a step the reader supplies.]* Taking that step, setting an image to "Non-Color" changes nothing about what a missing image gives you.

**What the node hands out.**

Same file, lines 119–126:

```
  const float4 f = svm_image_texture(kg, id, tex_co.x, tex_co.y, flags);

  if (stack_valid(out_offset)) {
    stack_store_float3(stack, out_offset, make_float3(f.x, f.y, f.z));
  }
  if (stack_valid(alpha_offset)) {
    stack_store_float(stack, alpha_offset, f.w);
  }
```

So the first three numbers are stored at `out_offset` and the fourth at `alpha_offset`; that those are the node's Colour and Alpha outputs is a step the reader supplies, as no quoted line names them. *[citation check]*

**The same substitution covers four more cases.**

- *A missing UDIM tile.* Same file, lines 78–110: `int id = -1;` is the starting point, and the loop over the node's tiles only replaces it if a matching tile is found — `if (tile_node.x == tile) { id = tile_node.y; break; }`. A coordinate that lands on a tile the node does not carry keeps `id` at -1 and gets the substitute.
- *Box projection.* Same file, lines 215–223, three weighted look-ups summed: `f += weight.x * svm_image_texture(kg, id, uv.x, uv.y, flags);` and the same for `weight.y` and `weight.z`. Each look-up returns the substitute, so the blend is a weighted sum of three copies of one value. *[citation check: "the weights sum to one, so the blend is still the substitute" was narrowed — the weights are worked out higher up the function, which is not quoted here.]*
- *The Environment Texture node.* Same file, line 260: `const float4 f = svm_image_texture(kg, id, uv.x, uv.y, flags);` — the same function, so the same substitution for a missing environment image.
- *Three-dimensional images (volumes).* File `Blender Sources/cycles_5.1/source/kernel/util/image_3d.h`, line 258, the closing line of `kernel_image_interp_3d`: `return IMAGE_MISSING_RGBA;`.

Read: 2026-07-30.

### What that value then does to a non-colour input (inferred)

Everything in this section is a step beyond what the files state. I have written each step out.

**Into a plain number — roughness, metallic, bump height, displacement height, and the like.**

Cycles turns a colour into a single number with a conversion node.

File: `Blender Sources/cycles_5.1/source/kernel/svm/convert.h`, lines 29–33:

```
    case NODE_CONVERT_CF: {
      const float3 f = stack_load_float3(stack, from);
      const float g = linear_rgb_to_gray(kg, f);
      stack_store_float(stack, to, g);
      break;
    }
```

File: `Blender Sources/cycles_5.1/source/kernel/util/colorspace.h`, lines 34–37:

```
ccl_device float linear_rgb_to_gray(KernelGlobals kg, const float3 c)
{
  return dot(c, make_float3(kernel_data.film.rgb_to_y));
}
```

Read: 2026-07-30.

*The step I took:* the colour is (1, 0, 1), so the number handed to the input is the red weight plus the blue weight of whatever the scene's working colour space uses for brightness, with the green weight dropped. Those weights are not written down in the files I was given, so I cannot give a figure. What I can say is that it is a single fixed number, the same at every point. *[citation check: "and it is not zero and not one" was cut — nothing quoted here fixes the weights, so nothing quoted rules out any size.]*

**Into a Normal Map node.**

File: `Blender Sources/cycles_5.1/source/kernel/svm/tex_coord.h`, lines 339–340:

```
  float3 color = stack_load_float3(stack, color_offset);
  color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);
```

Read: 2026-07-30.

*The step I took:* putting (1, 0, 1) through that arithmetic gives (1, -1, 1), and the same three numbers at every point of the surface, because the look-up ignores the position it is handed. So every point of the surface is shaded as if it were leaning the same way, whatever the shape of the object. *[citation check: "rather than the straight-up direction a flat normal map would give" was cut — no source quoted in this document says what a flat normal map contains. Sent back as a question for a fresh researcher.]*

**Into a Bump node's Height.**

File: `Blender Sources/cycles_5.1/source/kernel/svm/displace.h`, lines 72–78 and 98–99:

```
    const float h_c = stack_load_float(stack, c_offset);
    const float h_x = stack_load_float(stack, x_offset);
    const float h_y = stack_load_float(stack, y_offset);

    /* compute surface gradient and determinant */
    const float det = dot(dP.dx, Rx);
    const float3 surfgrad = (h_x - h_c) * Rx + (h_y - h_c) * Ry;
```

```
    float3 normal_out = safe_normalize(bump_filter_width * absdet * normal_in -
                                       scale * signf(det) * surfgrad);
```

Read: 2026-07-30.

*The step I took:* the node reads the height at a point and at two neighbouring points and works from the differences. With a missing image the three are the same number, so the differences are zero, so the second term drops out and the normal comes back unchanged. A missing bump map therefore produces no bumps at all — flat shading, not wrong shading.

**Into a Displacement node's Height.**

File: `Blender Sources/cycles_5.1/source/kernel/svm/displace.h`, lines 161–168 *[citation check: was 158–167]*:

```
      object_inverse_normal_transform(kg, sd, &dP);
      dP *= (height - midlevel) * scale;
      object_dir_transform(kg, sd, &dP);
    }
    else {
      /* World space. */
      dP *= (height - midlevel) * scale;
    }
```

Read: 2026-07-30.

*The step I took:* the height is a single fixed number, so every point moves along its own normal by the same distance. The shape is inflated or shrunk uniformly rather than given any relief. *[citation check: "With the usual midlevel of 0.5 the number is above the midlevel, so the movement is outward" was cut. Nothing quoted here says what the Midlevel default is, and this answer has already said it cannot give the number, so it cannot know which side of the midlevel the number falls. Sent back as a question for a fresh researcher.]*

### What the documentation does and does not say

**Not in the sources**, on all of the following:

- Any statement anywhere in the 5.1 manual, the 5.1 Python API reference or the 5.1 release notes about what a render produces when a material's image file is missing.
- Whether EEVEE substitutes the same value, a different one, or nothing. The EEVEE page listing where its nodes fall short of Cycles covers the Image Texture node but only for interpolation, projection and extension: "Smart Interpolation always uses Cubic interpolation. Artifact present using Tube or Sphere projection with linear interpolation." — manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/nodes_support.html, read 2026-07-30. Missing images are not mentioned.
- Whether a render writes a warning, marks the frame, or stops when an image is missing.
- Whether the substituted value is affected by view transform or by the file format written out. (It is a shading value like any other, so it goes through the same pipeline, but the sources do not address it and I will not assert it.)

**The one nearby thing the documentation does state**, and it is a different case, so I am recording it as ruled out rather than as the answer. In the Video Sequencer, a missing strip file is handled by an explicit switch:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent."

Manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html, read 2026-07-30. The same switch in the Python API reference:

> "Render missing images/movies with a solid magenta color (default False)"

Python API reference, https://docs.blender.org/api/5.1/bpy.types.SequenceEditor.html, read 2026-07-30.

This is out of scope for the question because a sequencer strip only ever feeds colour, and because the default is off, which gives transparency rather than magenta. I include it because it is the only place in the whole of the 5.1 documentation that states what a render does with a missing image at all, and because its magenta is a *separate* mechanism from Cycles' — different switch, different default, different part of Blender.

The two related commands in the File menu are also documented, and they are about finding missing files rather than about what a render does with them:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30.

### Not covered

- I cannot give the actual number a missing image hands to a plain-number input, because the brightness weights it depends on are not written down in the files I was given.
- I did not establish the link between "the file on disk is gone" and "the look-up gets -1" from the sources. The part of Cycles that loads images from disk and assigns those numbers is not among the files I was given; only the render kernel is. What I have is the kernel's own comment, "Color to use when images are not found", and the behaviour it triggers. The step from a missing file to a not-found image is one I did not see written down.
- EEVEE is entirely unaddressed. If your renders use EEVEE, nothing above is established for them.
- Nothing here covers what a packed image does, or an image that exists but cannot be decoded, or an image whose frame number falls outside a sequence. Those may or may not take the same path; the sources I was given do not say.
- Nothing here covers the Compositor's Image node or the Geometry Nodes Image Texture node with a missing file. The Geometry Nodes page describes the node's outputs — "The image data is sampled with the input Vector and outputs a Color and Alpha value" (manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/texture/image.html, read 2026-07-30) — but says nothing about a missing one.
- Whether the Open Shading Language path in Cycles behaves the same way is unsettled. See the record below.

### Outside the interface

Does this hold when Blender is driven from outside its own window?

For the Cycles part: the lines quoted are the render kernel — the code that produces the picture. Nothing in or around them tests whether Blender has a window open, and there is no alternative branch for a windowless run. On that basis I expect the substitution to be identical in a windowless run. **Inferred**, and the step is: the quoted code contains no such test, and the files I was given contain no second version of it. I did not find a statement to that effect anywhere in the sources.

Where it plainly does differ: the two repair commands are built around a person clicking. "A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it" (manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30). The Python API reference does show that the search command takes a directory as an argument — `bpy.ops.file.find_missing_files(*, find_all=False, directory='', ...)`, "Try to find missing external files" (Python API reference, https://docs.blender.org/api/5.1/bpy.ops.file.html, read 2026-07-30) — and that the reporting command takes none at all: `bpy.ops.file.report_missing_files()`, "Report all missing external files". Where the report goes without a window is not stated; the manual says it appears in the Info editor's header, which is part of the interface.

**Settled by hand test, 2026-08-09 — observed.** Every result in the *Hand-test results* section below was produced with Blender driven from a terminal with no window, on both computers. The substitution happened, the render finished, the frame was written, and the command ended with exit status 0. Standard error was empty. The reporting side is settled too: `bpy.ops.file.report_missing_files()` returned `{'FINISHED'}` and printed to the ordinary output — `Info: No missing files` where the file was there, and `Warning: Path '<path>/flat.png' not found, from local data-block 'IMflat.png'` where it was not — so the report does reach a terminal when there is no Info editor. The render itself printed one line to the ordinary output, `cycles | ERROR Image file <path>/flat.png does not exist.`

### Operating system

Does this hold on every operating system Blender runs on?

The sources do not say. The lines quoted from Cycles carry no branch on operating system at the point where the substitute value is produced, so I have no reason from the files to expect a difference in the value itself — **inferred**, step: no such branch appears in the quoted code. Whether a given path counts as missing in the first place could plainly differ between systems, but I found nothing in the 5.1 manual, the API reference or the release notes that addresses how image paths are matched per system, so I will not assert anything either way.

**Settled by hand test, 2026-08-09 — observed.** Every test in the *Hand-test results* section below was run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every test. The substitute values, the lines printed, the exit statuses and the pictures written matched; the only differences were in the last digits of a few figures, and those are given where they occur. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Record of where I looked

Everything below was searched inside `Blender Sources/` only, on the 2026-07-30 snapshot.

**Whole-manual, whole-API, whole-release-notes text searches** (case-insensitive), with what came back:

| Term | Manual | API reference | Release notes |
|---|---|---|---|
| missing | 24 files | 86 files | 3 files |
| magenta | 5 files | 2 files | 0 |
| pink | 14 files | 0 | 0 |
| purple | several, all unrelated (bones, curve handles, strip colours) | — | — |
| not found | 7 files | 1710 files (nearly all the phrase "not found" in unrelated API prose) | 0 |
| missing image / missing images / missing texture / missing textures / missing media | 1 manual file, 1 API file — both the Video Sequencer switch | — | 0 |
| missing file / Find Missing / Report Missing | 1 manual file — the File menu | — | 0 |
| broken | 16 files, none about images | — | — |
| placeholder | 9 files, none about a missing image in a render | — | — |
| fallback | 15 files — tools, ray tracing, light probes, denoising, glTF; none about images | — | — |
| failed to load / unable to load / could not be loaded / cannot load | 0 | 0 | 0 |
| does not exist / no longer exist | only the File menu entry and unrelated node prose | — | — |
| invalid image / empty image / deleted file / moved file / absent | nothing relevant | — | — |
| no image | 2 files — UV packing, and the Compositor Crop node | — | — |
| case-sensitive / path separator / backslash | nothing about image paths differing by operating system | — | — |

**Manual pages opened in full or in part, and what I ruled out:**

- `render/shader_nodes/textures/image.txt` — the Image Texture node. Documents every property and both outputs; says nothing about a missing image. This is the single page where an answer would most naturally live, and it is not there.
- `render/shader_nodes/textures/environment.txt`, `render/shader_nodes/displacement/normal_map.txt` — checked for the same, found nothing (they surfaced only on the "Non-Color" search).
- `render/eevee/limitations/nodes_support.txt` — read the Texture Nodes section in full. Image Texture is listed, but only for interpolation and projection shortfalls.
- `interface/window_system/topbar.txt` — read in full. Has Report Missing Files and Find Missing Files. Both are about locating files, not about rendering with them absent.
- `editors/video_sequencer/preview/sidebar.txt` — read the Show Missing Media entry in full. Ruled out as a different mechanism, in a different part of Blender, for strips that only carry colour.
- `modeling/geometry_nodes/texture/image.txt` — read in full. Nothing about a missing image.
- `files/blend/open_save.txt` — the "cannot be found" hit turned out to be about pruning the recent-files list.
- `advanced/command_line/arguments.txt` and `advanced/command_line/render.txt` — searched for "missing" and "texture". Only hit was an unrelated setting about a graphics-card memory pool. Nothing about missing files in a windowless run.
- `render/color_management/*`, `render/output/properties/stereoscopy/usage.txt`, `compositing/types/color/adjust/color_balance.txt`, `modeling/meshes/tools/loop.txt` — the remaining "magenta" hits. All unrelated (a colour-shift slider, anaglyph glasses, a colour wheel, a tool description).

**Python API reference pages checked:**

- `bpy.types.Image.txt` — read the properties list around `has_data` ("True if the image data is loaded into memory (default False, readonly)") and `source`. Nothing about what a render does when the data is not loaded.
- `bpy.types.ID.txt` — `is_missing` turned out to be about linked data-blocks: "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)". Ruled out — that is broken links between blend-files, not a missing image file.
- `bpy.ops.file.txt` — the two repair commands, quoted above.
- `bpy.types.SequenceEditor.txt` — the sequencer switch, quoted above.
- `bpy.types.ShaderNodeTexImage.txt`, `bpy.types.ImageUser.txt`, `bpy.types.ImageTexture.txt` — searched for "missing"; only the inherited `ID.is_missing` entry.
- Filtered every "missing" line across the whole API reference down to those also mentioning image, texture, file or media. That produced nothing beyond the items above.

**Release notes for 5.1:** searched every subject file for image and texture lines that also mention missing, failure, error, not found, magenta or pink. Five hits, all in the fixed-bugs list and none of them about a missing image in a render (an SGI decoding fault, a Python error in a texture panel, a Python error in the image editor, a Compositor link-drag issue, and a graphics-card test failure). No entry announces any change to how missing images are handled.

**Cycles' own files:**

- Searched every file for "MISSING". Exactly three hits, all quoted above: the definition in `source/util/types_image.h`, and the two places it is returned, in `source/kernel/svm/image.h` and `source/kernel/util/image_3d.h`.
- Read `source/kernel/svm/image.h` in full — the two-dimensional image node, its UDIM tile search, box projection, and the environment node.
- Read `source/kernel/svm/convert.h` in full, `source/kernel/util/colorspace.h` around the brightness function, `source/kernel/svm/displace.h` for the bump and displacement nodes, and `source/kernel/svm/tex_coord.h` for the normal map node.
- Searched the Python side of the add-on (`__init__.py`, `engine.py`, `properties.py`, `operators.py`, `ui.py` and the rest) for "missing", "magenta" and "not found". Two hits, neither relevant: a message about a missing frame when joining frames together, and an unrelated device check.
- Checked the pre-compiled Open Shading Language shaders in `shader/`. Searched all of them for "missing" — nothing. `shader/node_image_texture.oso` reaches the image through a general-purpose texture instruction (`texture ___419_rgb filename ...`, line 505) with no substitute colour supplied. What that instruction does with a missing file is decided outside these files, so I could not settle the Open Shading Language path. The rest of Cycles' handling above is for its ordinary path, which is the default.

**Pictures.** No picture in the manual was relevant here. The one candidate, `_images/node-types_ShaderNodeTexImage.webp` on the Image Texture node page, is a panel screenshot of the node's settings; it shows no missing-image state and would not settle the question either way, so I did not treat it as a source.

---

## Hand-test results — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md`; the entry numbers below are that list's numbers.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before the run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, because Blender writes text into a saved picture that nothing controls, so pictures were compared as pixels and never as files. "Missing" below always means the picture file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 5.1.2, from `/opt/blender/5.1/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.1.2.

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
DIFF normalmap  present_vs_unwired mean 0.083611  max 0.661448
DIFF normalmap  missing_vs_unwired mean 0.056779  max 0.525167
DIFF normalmap  missing_vs_present mean 0.049017  max 0.603775
SILHOUETTE normalmap  present  pixels 2224
SILHOUETTE normalmap  missing  pixels 2224
SILHOUETTE normalmap  unwired  pixels 2224
DIFF bump       present_vs_unwired mean 0.021260  max 0.640052
DIFF bump       missing_vs_unwired mean 0.000040  max 0.006260
DIFF bump       missing_vs_present mean 0.021222  max 0.640052
SILHOUETTE bump       present  pixels 2225
SILHOUETTE bump       missing  pixels 2225
SILHOUETTE bump       unwired  pixels 2225
DIFF displace   present_vs_unwired mean 0.109437  max 0.655591
DIFF displace   missing_vs_unwired mean 0.022522  max 0.645490
DIFF displace   missing_vs_present mean 0.101933  max 0.655591
SILHOUETTE displace   present  pixels 3126
SILHOUETTE displace   missing  pixels 1848
SILHOUETTE displace   unwired  pixels 1424
```

The rented Linux machine printed the same, with three lines differing in their last digits: `DIFF normalmap present_vs_unwired mean 0.083579 max 0.661448`, `DIFF normalmap missing_vs_present mean 0.048984 max 0.603775`, and `DIFF bump missing_vs_unwired mean 0.000040 max 0.006259`. Every silhouette count matched.

The `present_vs_unwired` line in each group is the control showing that each of the three wirings does change the picture when the file is there.

So: with the picture missing, the normal-map sphere differed from the same sphere with nothing connected. The bump sphere did not — the largest difference of any single channel anywhere on it was 0.006260 on the Mac and 0.006259 on the rented Linux machine, against 0.64 for the control. The displacement sphere's outline held 1848 pixels with the picture missing, 3126 with it present, and 1424 with nothing connected; the undisplaced spheres beside it held 2224 and 2225.

### 3. What the render says, whether it finishes, and what it writes (entry 3)

**What was done.** The Emission plane from entry 1, with its Color wired to Emission Color, was rendered from a terminal with the ordinary output and the error output captured separately, once with the picture present and once with it moved away. The exact command was `BLENDER_USER_RESOURCES=<fresh folder> <blender> --background --factory-startup <work>/col.blend --render-output <work>/missing_ --render-frame 1`.

**What came back, on both computers.** Both runs ended with exit status 0 and both wrote their frame. Standard error was empty, 0 bytes, in both runs. The missing-picture run printed one line that the picture-present run did not. On the Creator's Mac:

```
00:00.326  cycles           | ERROR Image file <work folder>/flat.png does not exist.
```

and on the rented Linux machine:

```
00:00.870  cycles           | ERROR Image file /root/ht-images/linuxC/e3-5.1/flat.png does not exist.
```

It sits on standard output, not on standard error, and it names the file. There are no quotation marks around the path.

Both runs also printed the line `Fra: 1 | Mem: 1M | Updating Images | Loading flat.png` — that line appears whether or not the file is there, so it is not the report.

That the file really had not been read is shown by the frames themselves, on the Creator's Mac:

```
CENTREPIXEL present_0001.exr [0.0, 1.0, 0.0, 1.0] bytes 1608
CENTREPIXEL missing_0001.exr [1.0, 0.0, 1.0, 1.0] bytes 1442
```

and on the rented Linux machine, the same two centre pixels at 1504 and 1438 bytes.

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

**What came back.** In 5.1, `BLENDER_EEVEE_NEXT` was refused:

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
EEVEE DIFF normalmap  present_vs_unwired mean 0.094679  max 0.637192
EEVEE DIFF normalmap  missing_vs_unwired mean 0.052932  max 0.515213
EEVEE DIFF bump       present_vs_unwired mean 0.027924  max 0.627426
EEVEE DIFF bump       missing_vs_unwired mean 0.000000  max 0.000000
EEVEE SILHOUETTE displace   present  pixels 3140
EEVEE SILHOUETTE displace   missing  pixels 1852
EEVEE SILHOUETTE displace   unwired  pixels 1436
```

and on the rented Linux machine `EEVEE DIFF normalmap missing_vs_unwired mean 0.052806 max 0.513809`, `EEVEE DIFF bump missing_vs_unwired mean 0.000000 max 0.000000`, and the same three silhouette counts, 3140, 1852 and 1436.

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

5.1 printed a line in four of the six states, and the wording is not the same for all four:

```
moved:     cycles | ERROR Image file <path>/flat.png does not exist.
badfolder: cycles | ERROR Image file <path>/nosuchfolder/flat.png does not exist.
zerobyte:  cycles | ERROR Image file <path>/flat.png failed to load.
garbage:   cycles | ERROR Image file <path>/flat.png failed to load.
```

The `present` and `wrongext` states printed no such line.

A seventh state was run only on the rented Linux machine, because everything there otherwise runs as root and root reads past permissions without complaint. The work folder was handed to the ordinary user `tester` and Blender was run as that user. With the picture set to `chmod 644` the run printed `RESULT permok [0.0, 1.0, 0.0, 1.0]`; with it set to `chmod 600` and owned by root it printed `RESULT permdenied [1.0, 0.0, 1.0, 1.0]`, together with `cycles | ERROR Image file <path>/flat.png failed to load.`. That the permission really bit was shown in the same run:

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

**What was then done.** 5.1 holds the compositor as a node group on `scene.compositing_node_group`, so the scene was built that way instead: an Image node loading the picture, its Image output into the `Factor` input of a Color Ramp node left at its default black-to-white, and the Color Ramp's Color into a Group Output whose interface socket is a colour. `CompositorNodeValToRGB` and `CompositorNodeComposite` do not exist in 5.1; the node types used were printed:

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
RESULT present   mean/max: (0.23868240416049957, 2.003127098083496)
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

**What came back, first — with the framing exactly as it was written out.** The check's framing puts the three spheres at x = −1, 0, +1 in an orthographic width of 3. Run that way, the displaced sphere runs off the picture. Counting lit pixels on each edge row of the frame, the picture-present render gave top 52, bottom 52, left 0, right 52, on both computers; the other three renders touched no edge at all. In the same render the middle third of the frame held 3464 lit pixels against 3209 in every other state, because part of the displaced sphere had grown into its neighbour's third. The displaced sphere's own third counted 5871 present, 1973 missing, 804 nothing-connected, 3214 plain, and those counts are cut short by the frame edge rather than measured. So the figures from that framing are recorded here as what happened and are not used to answer anything.

**What came back, second — with the framing widened.** Two numbers were changed and nothing else: the orthographic width to 4.5 and the spheres to x = −1.5, 0, +1.5. Nothing touched any edge of the frame in any of the four renders, on either computer. The figures below are the mean and the largest single-channel difference over each third of the frame; the two computers agreed except in the last digits of one mean.

```
DIFF normalmap  present_vs_unwired mean 0.042157  max 0.501700
DIFF normalmap  missing_vs_unwired mean 0.033042  max 0.398359
DIFF normalmap  unwired_vs_nonode  mean 0.000000  max 0.000000
DIFF normalmap  missing_vs_nonode  mean 0.033042  max 0.398359
DIFF bump       present_vs_unwired mean 0.011695  max 0.487683
DIFF bump       missing_vs_unwired mean 0.000036  max 0.012500
DIFF bump       unwired_vs_nonode  mean 0.000070  max 0.012500
DIFF displace   present_vs_unwired mean 0.103587  max 0.499832
DIFF displace   missing_vs_unwired mean 0.022044  max 0.501909
DIFF displace   unwired_vs_nonode  mean 0.045120  max 0.502020
DIFF displace   missing_vs_nonode  mean 0.023251  max 0.495387
SILHOUETTE normalmap  present 1416  missing 1416  unwired 1416  nonode 1416
SILHOUETTE bump       present 1416  missing 1416  unwired 1416  nonode 1416
SILHOUETTE displace   present 3048  missing 874  unwired 350  nonode 1424
```

That block is the Creator's Mac. The rented Linux machine gave `present_vs_unwired mean 0.042147` for the normal map where the Mac gave 0.042157, and every other figure above to the digit printed.

**The controls.** `present_vs_unwired` in each group is the first control: each of the three wirings changes the picture when the file is really there. `unwired_vs_nonode` for the normal map is the second: a Normal Map node fed 0.5, 0.5, 1 gives a picture identical to no Normal Map node at all, mean 0.000000 and largest 0.000000, so "unchanged" has a fixed meaning in this run.

**So, with the picture missing:**

- **The normal map.** The surface is not left as it was. The missing-picture render differs from the plain sphere by mean 0.033042 and largest 0.398359 of a channel, against a control that shows what "identical" looks like here — exactly zero.
- **The bump height.** The surface is left as it was. The largest single-channel difference anywhere on that sphere between the missing-picture render and the nothing-connected render was 0.012500, against 0.487683 for the control.
- **The displacement height.** The sphere gets smaller. Its outline held 1424 pixels untouched, 874 with the picture missing, 350 with nothing connected to the Displacement node, and 3048 with the picture present. The other two spheres, which are not displaced, held 1416 and 1416 in every state.

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

Then the scene, with every number recorded: a Volume data-block pointing at `//vol.vdb`, on an object at location (0, 0, 0), rotation 0, scale (1, 1, 1); a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; the frame 64 by 64 with the film **not** transparent; a world data-block with its nodes switched off and its colour black; Cycles at 128 samples with denoising off. With the file loaded, the object's dimensions read (3.75, 3.75, 5.0). In 5.1 setting the world's nodes off did not take — the property read back as True straight after it was set — so the empty frame carries the world's own default grey rather than black.

Three renders: file present; the Volume object deleted altogether; the file moved out of the folder.

**What came back, the same on both computers**, as the mean and the largest pixel value over the frame:

```
RESULT present   mean 0.23868240416049957  max 2.003127098083496
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

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 5.1

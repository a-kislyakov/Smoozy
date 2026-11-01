# Images — later questions (Blender 4.5)

Sources consulted, all inside `Blender Sources/`, all taken on 2026-07-30 and read by me on 2026-08-09:

- Blender's manual for 4.5 (`manual_4.5`), searched in full and read in places
- Blender's Python API documentation for 4.5 (`api_4.5`), searched in full
- Blender's release notes for 4.5 (`release_notes_4.5`), searched in full
- Cycles' own files for 4.5 (`cycles_4.5`), searched and read in places

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 9 was assigned to me. The rest of the sheet is untouched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

> **Changed by the citation check, 2026-08-09 (Opus 5).** Five things in this answer were changed against the files themselves, each marked *[citation check]* where it sits: the address for the colour-to-number conversion was corrected from lines 32–37 to lines 33–38; "the only thing the function is told" was corrected to name the flags argument the quote also shows; the step from the two stack offsets to "the node's Color output" and "its Alpha output" was relabelled inferred rather than observed; "a full-strength sideways tilt" was narrowed to what the arithmetic carries; and the Image Texture node's Color description, which had been quoted mid-sentence, was restored whole. Three hand-test entries were annotated — see entries 2, 5 and 14.

**Short answer.** This is about Cycles only, and it is read out of Cycles' own files rather than watched in a render. Those files say the picture-reading step, when it is told the picture is not available, returns the fixed value red 1, green 0, blue 1, alpha 1 — magenta at full opacity — and returns it before anything else in that step runs. The step is told which picture to read and where, and nothing about what the result will feed. So an image feeding a normal map, a bump, a displacement, a roughness or any other non-colour input is handed the same four numbers it would be handed if it were feeding colour, and the finished picture shows not magenta but whatever the shader makes of those numbers. Two steps stand between the files and a render — that these files are what a render runs, and what the shader then does with the numbers — and both are marked inferred below.

**A note on the question.** It can be read two ways. I answer the reading I think is meant: the image is plugged into something that is not a colour input. The other reading is that the image's own Color Space is set to Non-Color. The finding below bears on both, because the lines quoted show the substitute returned before the colour-space conversion in that same step. Whether the Color Space setting reaches that step only through the flags named in those lines is not shown by anything I quote.

### The substitute value — observed

The value is a named constant in Cycles' own files.

Exact words, from `Blender Sources/cycles_4.5/source/util/texture.h`, lines 11 to 17:

```
/* Color to use when textures are not found. */
enum {
  TEX_IMAGE_MISSING_R = 1,
  TEX_IMAGE_MISSING_G = 0,
  TEX_IMAGE_MISSING_B = 1,
  TEX_IMAGE_MISSING_A = 1
};
```

Date read: 2026-07-30 (snapshot date).

### Where it is used, and why the destination makes no difference — observed

Exact words, from `Blender Sources/cycles_4.5/source/kernel/svm/image.h`, lines 20 to 41:

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

Three things, the first two visible in the lines above. First, the missing case returns before the alpha handling and before the sRGB-to-linear conversion — so in those lines the substitute is those four raw numbers, with neither of the two treatments in that step applied to them. Second, what the function is told is which picture to read, where on it to read, and a set of flags *[citation check]*; it is told nothing about what the result will feed. Third — and this one is not in the lines above — the same function is called by all three of Cycles' picture nodes: the Image Texture node in its ordinary projections, the Image Texture node in Box projection, and the Environment Texture node. That rests on line references in the same file (119, 218, 222, 226 and 263); only the first is quoted below, so the other four carry no words here.

Exact words, same file, lines 119 to 126 (this is the Image Texture node handing its result out):

```
  const float4 f = svm_image_texture(kg, id, tex_co.x, tex_co.y, flags);

  if (stack_valid(out_offset)) {
    stack_store_float3(stack, out_offset, make_float3(f.x, f.y, f.z));
  }
  if (stack_valid(alpha_offset)) {
    stack_store_float(stack, alpha_offset, f.w);
  }
```

What the quoted lines show is that the first three numbers are stored at `out_offset` and the fourth at `alpha_offset`. That those two are the node's Color and Alpha outputs, and that they then reach whatever is connected, colour or not, is a step a reader supplies — nothing in the quoted lines says it. **Inferred**, not observed. *[citation check]*

The same file also bears on the tiled (UDIM) case: the tile index starts at `int id = -1;` (line 78) and, on my reading of lines 93 to 103, changes only if a matching tile is found. I have not quoted lines 93 to 103, so that part carries no words here. A tile that is absent therefore takes the same path as a picture that is absent — a step from those line references, not something the words quoted above say.

Date read: 2026-07-30 (snapshot date).

### From what the files say to what a render does — inferred

Everything above is read out of files, not watched in a render. The step is from the words in those files to the claim that a render behaves that way. Two things make it a step rather than a reading. Cycles is supplied as its own files, and I did not establish that the ones I read are the ones a render runs rather than a copy kept for building graphics-card kernels. And reading an instruction is not the same as seeing it carried out. Nothing in this document was rendered.

### From "the file on disk is not there" to "id is -1" — inferred

The files supplied here show what happens once the lookup is told the picture is not available. They do not contain the part of Cycles that opens files and decides that. The step I am taking is from the comment `/* Color to use when textures are not found. */` in `texture.h` line 11 to the claim that a picture whose file is absent arrives at the lookup as the missing case. The comment says the constant is for textures that are not found; the only place it is used is the `id == -1` branch. A reader has to join those two, so this is inferred, not documented.

### What the value feeds into — inferred

The step: the numbers 1, 0, 1 (and alpha 1) are what the destination receives, and the destination then does its own arithmetic on them. Worked through for the most common non-colour case, a tangent-space normal map:

Exact words, from `Blender Sources/cycles_4.5/source/kernel/svm/tex_coord.h`, lines 347 and 348:

```
  float3 color = stack_load_float3(stack, color_offset);
  color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);
```

Putting 1, 0, 1 through that arithmetic gives 1, -1, 1, the same three numbers at every point on the surface (the lookup quoted above ignores the position it is handed), which is then combined with the surface's tangent and normal (same file, lines 390 to 396, which I have not quoted). What those three numbers amount to as a direction — how far from the surface's own normal, and in which sense — is not carried by the quoted line, so I do not say. *[citation check]* That is not magenta, and it is the same value at every point on the surface. What that looks like in a finished frame I did not see; I rendered nothing.

For a single-number input, such as roughness or metallic, the colour is reduced to one number by a weighted grey:

Exact words, from `Blender Sources/cycles_4.5/source/kernel/svm/convert.h`, lines 33 to 38 *[citation check: was 32 to 37]*:

```
    case NODE_CONVERT_CF: {
      const float3 f = stack_load_float3(stack, from);
      const float g = linear_rgb_to_gray(kg, f);
      stack_store_float(stack, to, g);
      break;
    }
```

The weights themselves are taken from scene data (`Blender Sources/cycles_4.5/source/kernel/util/colorspace.h`, lines 34 to 37: `return dot(c, make_float3(kernel_data.film.rgb_to_y));`), so I cannot state the resulting number from these files. What I can say is that it is a single constant value applied uniformly, not magenta.

So: **on this reading of Cycles' files, a missing non-colour texture is not flagged by turning that part of the picture magenta.** What reaches the surface is a constant value instead of the map's own. Whether that looks obviously broken depends on what the map was doing, and that part I did not see.

### Whether the render says anything about it

Not in the sources. Nothing in the supplied Cycles files, the manual, the API documentation or the release notes says whether a render prints, logs or otherwise reports that a picture was missing. See the record of looking below.

### What the sources say about other renderers

Not in the sources. The finding above rests entirely on Cycles' own files. Nothing I found describes what EEVEE or Workbench put in a render where a picture is missing, for a colour input or any other. The only place the manual names a magenta stand-in at all is the Video Sequencer, which is a different thing and is quoted in the record of looking.

### Not covered

- Why the lookup is told the picture is missing, and whether every kind of absence (file deleted, path wrong, drive not mounted, unreadable file, unsupported format, wrong permissions) reaches it the same way. The deciding code is not in the supplied files.
- Whether the shipped files I read are the ones that actually run during a render, or a copy kept for compiling graphics-card kernels. They are Cycles' own files as supplied with Blender; I did not verify which role they play at render time.
- EEVEE and Workbench, entirely.
- Whether anything is written to a log or console.
- Volumes read from external volume files, which take a different path in the same folder (`source/kernel/svm/voxel.h`) and have no missing-value branch of their own.
- What the final picture looks like, in any concrete case. I can say what value is fed in; I cannot say what the finished frame looks like without rendering one, and I rendered nothing.
- Whether alpha 1 rather than 0 matters for a picture feeding transparency — the numbers are stated above, the consequence is not traced.

### Outside the interface

The finding holds. The step: the substitute value lives in the render kernel — the part that computes the picture — and not in any window, panel or editor. Nothing in the lines quoted refers to the interface, to a user, or to anything being displayed. A render started from a command line runs that same code. This is inferred; the sources do not state it in so many words.

What does differ is everything around it. The manual's two aids for this problem are menu items that need a person: "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files." and "Find Missing Files — In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up." (manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html, read 2026-07-30). The API documentation lists `bpy.ops.file.report_missing_files()`, "Report all missing external files", and `bpy.ops.file.find_missing_files(...)`, "Try to find missing external files" (API documentation, https://docs.blender.org/api/4.5/bpy.ops.file.html, read 2026-07-30) — but the sources do not say where the report of the first goes when there is no Info editor to put it in, and the second opens a file browser.

**Settled by hand test, 2026-08-09 — observed.** Every result in the *Hand-test results* section below was produced with Blender driven from a terminal with no window, on both computers. The substitution happened, the render finished, the frame was written, and the command ended with exit status 0. Standard error was empty. The reporting side is settled too: `bpy.ops.file.report_missing_files()` returned `{'FINISHED'}` and printed to the ordinary output — `Info: No missing files` where the file was there, and `Warning: Path '<path>/flat.png' not found, from local data-block 'IMflat.png'` where it was not — so the report does reach a terminal when there is no Info editor. The render itself said nothing at all about the absent file: with the timing and memory figures stripped out, the whole of its output differed from a picture-present render's in the name of the file it saved and nothing else.

### Operating system

The sources do not say. The searches recorded below were for the subject of the question and included no operating-system words; what they turned up is no statement of any kind about a missing picture in a render, on any operating system. Beyond that, nothing in the lines I quoted is written differently for different operating systems — the substitute is a plain fixed value in a shared file with no conditions on it. No source states that it holds everywhere.

**Settled by hand test, 2026-08-09 — observed.** Every test in the *Hand-test results* section below was run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every test. The substitute values, the lines printed, the exit statuses and the pictures written matched; the only differences were in the last digits of a few figures, and those are given where they occur. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Record of looking

Searched the whole of `manual_4.5` and `api_4.5` for: magenta, pink, purple, missing, missing image, missing texture, missing file, image is missing, file is missing, texture is missing, not found, cannot be found, could not be found, no longer exist, does not exist, doesn't exist, deleted or moved, unable to load, fails to load, failed to load, cannot load, can't load, invalid image, empty image, no image, broken, broken path, unresolved, placeholder, fallback, falls back, default color, default colour, checkerboard, unavailable.

What that turned up, and why each was ruled out:

- The only statement in the whole manual that a missing picture is drawn magenta is about the Video Sequencer, not about textures in a shader: "Show Missing Media — Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." (manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html, read 2026-07-30). The matching API entry is `show_missing_media`, "Render missing images/movies with a solid magenta color", "boolean, default False" (API documentation, https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html, read 2026-07-30). A sequencer strip feeds the picture directly, not a non-colour input, so this does not answer the question.
- The Image Texture node page describes its two outputs — "Color — RGB color from image. If the image has transparency, the color is premultiplied if the Alpha output is used, and unpremultiplied (straight) otherwise." *[citation check: the second sentence had been cut]* and "Alpha — Alpha channel from image." — and says nothing anywhere about the image being absent (manual, https://docs.blender.org/manual/en/4.5/render/shader_nodes/textures/image.html, read 2026-07-30).
- The Image Settings page covers Color Space and says "Some images such as normal, bump or stencil maps do not strictly contain 'colors' and should never have a color conversion applied to them. For such images, the color space should be set to Non-Color." — the closest the manual comes to the non-colour case, with nothing about absence (manual, https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html, read 2026-07-30).
- The Color Management page defines the setting — "Non-Color: Generic data that is not color, will not apply any color transform (e.g. normal maps)." — again nothing about absence (manual, https://docs.blender.org/manual/en/4.5/render/color_management.html, read 2026-07-30).
- The Normal Map node page says "the image texture should be set to Non-Color mode to give correct results" and describes its Color input as "RGB color that encodes the normal map in the specified space", with nothing about the image being absent (manual, https://docs.blender.org/manual/en/4.5/render/shader_nodes/vector/normal_map.html, read 2026-07-30).
- `ID.is_missing` appears on many types including `bpy.types.Image`, but it belongs to linked data-blocks from other blend-files, not to an image file on disk; `Image.has_data`, "True if the image data is loaded into memory", says nothing about what a render then does (API documentation, https://docs.blender.org/api/4.5/bpy.types.Image.html, read 2026-07-30).
- Searched all of `release_notes_4.5` for magenta, missing, "texture not", "image not". The only hits were unrelated (a Cycles line "Support previously missing features", and bug-fix list entries). Nothing about missing pictures in renders.
- Searched all of `cycles_4.5` for magenta, missing, invalid, TEX_IMAGE_MISSING, missingcolor. The five hits for TEX_IMAGE_MISSING are the two files quoted above and nothing else. `missingcolor` appears only as two do-nothing stubs for the graphics-card Open Shading Language path (`source/kernel/osl/services_gpu.h`, lines 1380 and 1385), which set no value; I could not follow that path further because the code that would use it is not in the supplied files, so I make no claim about renders using Open Shading Language.
- I did not use the web, and did not need to.

---

## Hand-test results — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md`; the entry numbers below are that list's numbers.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before the run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, because Blender writes text into a saved picture that nothing controls, so pictures were compared as pixels and never as files. "Missing" below always means the picture file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 4.5.12 LTS, from `/opt/blender/4.5/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 4.5.12 LTS.

### 1. The numbers a missing picture hands out (entry 1)

**What was done.** A 64 by 64 picture file was written with every pixel red 0, green 1, blue 0, alpha 1, and a second one with every pixel white and alpha 0.25. A plane filling an orthographic camera was given an Emission-only material with an Image Texture node, in three wirings: the node's **Color** into Emission Color at strength 1; the node's **Alpha** into Emission Strength with a white colour; and the node's **Color** into Emission Strength with a white colour, which is a colour output plugged into a socket that takes one number. Cycles, one sample, 32 by 32 pixels, filter width 0. Each was rendered with the picture present and again with it moved away.

**What came back.** The Creator's Mac printed:

```
RESULT col_cpu_default present [0.0, 1.0, 0.0, 1.0]
RESULT alpha_cpu_default present [0.250980406999588, 0.250980406999588, 0.250980406999588, 1.0]
RESULT grey_cpu_default present [0.7151514887809753, 0.7151514887809753, 0.7151514887809753, 1.0]
RESULT col_gpu_default present [0.0, 1.0, 0.0, 1.0]
RESULT col_cpu_noncolor present [0.0, 1.0, 0.0, 1.0]
RESULT col_cpu_default missing [1.0, 0.0, 1.0, 1.0]
RESULT alpha_cpu_default missing [1.0, 1.0, 1.0, 1.0]
RESULT grey_cpu_default missing [0.2848494350910187, 0.2848494350910187, 0.2848494350910187, 1.0]
RESULT col_gpu_default missing [1.0, 0.0, 1.0, 1.0]
RESULT col_cpu_noncolor missing [1.0, 0.0, 1.0, 1.0]
```

The rented Linux machine printed the same, with these two lines differing in their last digits:

```
RESULT grey_cpu_default present [0.7151516079902649, 0.7151516079902649, 0.7151516079902649, 1.0]
RESULT grey_cpu_default missing [0.28484946489334106, 0.28484946489334106, 0.28484946489334106, 1.0]
```

The two machines agreed. On both, the picture-present runs gave red 0, green 1, blue 0 and an alpha of 0.250980406999588, which is the control showing the file really was being read.

So, with the picture missing: the node's Color output carried red 1, green 0, blue 1; its Alpha output carried 1; and a colour output plugged into a socket taking one number gave 0.284849 on the Mac and 0.2848495 on the rented Linux machine. The graphics card gave the same numbers as the processor. Setting the node's Colour Space to `Non-Color` gave the same numbers as leaving it at `sRGB`.

### 2. A missing picture feeding a normal map, a bump height and a displacement height (entry 2)

> **Note from the citation check, 2026-08-09 (Opus 5).** This entry cannot be repeated from its own words. It does not say how big the spheres are, where they sit, how the camera is framed, which way the sun points or how bright it is, or what colour the surfaces are — and every number below depends on all of those. Re-running it with those details invented reproduces the three results in kind (the normal-map sphere differs from the unwired one; the bump sphere does not; the displacement sphere's outline sits between the two) but not the figures. A test somebody can follow is written out in the hand-test list under this check's heading.

**What was done.** A 64 by 64 black-and-white checker was written. Three UV-unwrapped spheres of 128 by 64 segments sat side by side under one sun lamp with its angle set to 0, against a black world, with Cycles at four samples and no light bounces, so that two identical set-ups give identical pixels. The left sphere took the picture through a Normal Map node into a Diffuse BSDF's Normal input; the middle one through a Bump node's Height with Distance 5; the right one through a Displacement node's Height at scale 0.4 into the Material Output, with the material's displacement method set to `DISPLACEMENT`. Three renders were made: picture present and wired; picture moved away and wired; picture present with the link out of each Image Texture node removed. The frame was 240 by 80 with a transparent film, so each sphere has its own third of the frame and the number of pixels inside its outline can be counted.

**What came back.** Both computers printed the same numbers:

```
DIFF normalmap  present_vs_unwired mean 0.079000  max 0.636317
DIFF normalmap  missing_vs_unwired mean 0.053971  max 0.514908
DIFF normalmap  missing_vs_present mean 0.046174  max 0.587494
SILHOUETTE normalmap  present  pixels 2227
SILHOUETTE normalmap  missing  pixels 2227
SILHOUETTE normalmap  unwired  pixels 2227
DIFF bump       present_vs_unwired mean 0.020014  max 0.621900
DIFF bump       missing_vs_unwired mean 0.000001  max 0.000026
DIFF bump       missing_vs_present mean 0.020015  max 0.621899
SILHOUETTE bump       present  pixels 2227
SILHOUETTE bump       missing  pixels 2227
SILHOUETTE bump       unwired  pixels 2227
DIFF displace   present_vs_unwired mean 0.103296  max 0.632193
DIFF displace   missing_vs_unwired mean 0.021025  max 0.625831
DIFF displace   missing_vs_present mean 0.096593  max 0.632193
SILHOUETTE displace   present  pixels 3124
SILHOUETTE displace   missing  pixels 1849
SILHOUETTE displace   unwired  pixels 1424
```

The rented Linux machine's one difference was `DIFF bump missing_vs_unwired mean 0.000001 max 0.000029`.

The `present_vs_unwired` line in each group is the control showing that each of the three wirings does change the picture when the file is there.

So: with the picture missing, the normal-map sphere differed from the same sphere with nothing connected. The bump sphere did not — the largest difference of any single channel anywhere on it was 0.000026 on the Mac and 0.000029 on the rented Linux machine, against 0.62 for the control. The displacement sphere's outline held 1849 pixels with the picture missing, 3124 with it present, and 1424 with nothing connected; the undisplaced spheres beside it held 2227.

### 3. What the render says, whether it finishes, and what it writes (entry 3)

**What was done.** The Emission plane from entry 1, with its Color wired to Emission Color, was rendered from a terminal with the ordinary output and the error output captured separately, once with the picture present and once with it moved away. The exact command was `BLENDER_USER_RESOURCES=<fresh folder> <blender> --background --factory-startup <work>/col.blend --render-output <work>/missing_ --render-frame 1`.

**What came back, on both computers.** Both runs ended with exit status 0 and both wrote their frame. Standard error was empty, 0 bytes, in both runs. With the timing and memory figures stripped out of Blender's progress lines, the whole of the missing-picture run's output differed from the picture-present run's in two lines only:

```
< Saved: 'WORK/present_0001.exr'
< Time: 00:00.01 (Saving: 00:00.00)
---
> Saved: 'WORK/missing_0001.exr'
> Time: 00:00.00 (Saving: 00:00.00)
```

Searching the missing-picture run's output for `missing`, `cannot`, `can't`, `unable`, `error`, `warn`, `fail`, `not found`, `no such` and `invalid` matched one line, the `Saved:` line. **Nothing named the absent file and nothing reported it.** Both runs printed the line `Updating Images | Loading flat.png` — that line appears whether or not the file is there.

That silence is a result rather than an absence of looking for two reasons recorded in the same runs: the picture-present run shows this stream does carry Blender's own messages (its version banner, thirty-odd progress lines and a `Saved:` line), and the frame written by the missing-picture run had a centre pixel of `[1.0, 0.0, 1.0, 1.0]` against the control's `[0.0, 1.0, 0.0, 1.0]`, so the file really had not been read.

```
CENTREPIXEL present_0001.exr [0.0, 1.0, 0.0, 1.0] bytes 1488
CENTREPIXEL missing_0001.exr [1.0, 0.0, 1.0, 1.0] bytes 1322
```

(That pair is from the Creator's Mac; the rented Linux machine gave the same two centre pixels, at 1384 and 1318 bytes.)

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

**What came back.** In 4.5, `BLENDER_EEVEE` was refused:

```
ENGINE NOT ACCEPTED BLENDER_EEVEE TypeError('bpy_struct: item.attr = val: enum "BLENDER_EEVEE" not found in (\'BLENDER_EEVEE_NEXT\', \'BLENDER_WORKBENCH\', \'CYCLES\')')
```

With `BLENDER_EEVEE_NEXT`, the Creator's Mac and the rented Linux machine both printed:

```
RESULT col BLENDER_EEVEE_NEXT present [0.0, 1.0, 0.0, 1.0]
RESULT col BLENDER_EEVEE_NEXT missing [1.0, 0.0, 1.0, 1.0]
```

With `BLENDER_WORKBENCH`, the Creator's Mac printed:

```
RESULT col BLENDER_WORKBENCH present [0.0, 0.99755859375, 0.0, 0.99755859375]
RESULT col BLENDER_WORKBENCH missing [0.99755859375, 0.0, 0.99755859375, 0.99755859375]
```

and the rented Linux machine printed:

```
RESULT col BLENDER_WORKBENCH present [0.0, 0.9990234375, 0.0, 1.0]
RESULT col BLENDER_WORKBENCH missing [0.9990234375, 0.0, 0.9990234375, 1.0]
```

The three-sphere file rendered in `BLENDER_EEVEE_NEXT` gave, on the Creator's Mac:

```
EEVEE DIFF normalmap  present_vs_unwired mean 0.094623  max 0.637207
EEVEE DIFF normalmap  missing_vs_unwired mean 0.052901  max 0.515234
EEVEE DIFF bump       present_vs_unwired mean 0.027909  max 0.627441
EEVEE DIFF bump       missing_vs_unwired mean 0.000000  max 0.000000
EEVEE SILHOUETTE displace   present  pixels 3140
EEVEE SILHOUETTE displace   missing  pixels 1852
EEVEE SILHOUETTE displace   unwired  pixels 1436
```

and on the rented Linux machine `EEVEE DIFF normalmap missing_vs_unwired mean 0.052804 max 0.513775`, `EEVEE DIFF bump missing_vs_unwired mean 0.000000 max 0.000000`, and the same three silhouette counts, 3140, 1852 and 1436.

### 6. Which kinds of absence give the same result (entry 6)

**What was done.** Six copies of the entry-1 blend-file, each in its own folder, with the picture in a different state, plus a seventh run with the picture correct as the control.

**What came back, the same on both computers.** The centre pixel in each state:

```
CASE present   blend points at //flat.png | that path exists on disk: True  | size: 303
RESULT present   [0.0, 1.0, 0.0, 1.0]
CASE moved     blend points at //flat.png | that path exists on disk: False | size: -
RESULT moved     [1.0, 0.0, 1.0, 1.0]
CASE badfolder blend points at <work>/nosuchfolder/flat.png | that path exists on disk: False | size: -
RESULT badfolder [1.0, 0.0, 1.0, 1.0]
CASE zerobyte  blend points at //flat.png | that path exists on disk: True  | size: 0
RESULT zerobyte  [1.0, 0.0, 1.0, 1.0]
CASE wrongext  blend points at <work>/flat.xyz | that path exists on disk: True | size: 303
RESULT wrongext  [0.0, 1.0, 0.0, 1.0]
CASE garbage   blend points at //flat.png | that path exists on disk: True  | size: 200
RESULT garbage   [1.0, 0.0, 1.0, 1.0]
```

`wrongext` is a working PNG file whose name ends `.xyz`, with the blend-file pointing at that name; `garbage` is a file named `flat.png` holding two hundred bytes of zeros.

A seventh state was run only on the rented Linux machine, because everything there otherwise runs as root and root reads past permissions without complaint. The work folder was handed to the ordinary user `tester` and Blender was run as that user. With the picture set to `chmod 644` the run printed `RESULT permok [0.0, 1.0, 0.0, 1.0]`; with it set to `chmod 600` and owned by root it printed `RESULT permdenied [1.0, 0.0, 1.0, 1.0]`. That the permission really bit was shown in the same run:

```
proof the permission really bites — tester trying to read it:
cat: <work>/flat.png: Permission denied
(that command's exit status: 1)
```

In none of the seven states did 4.5 print anything about the picture; searching each run's output for `ERROR`, `WARNING`, `does not exist`, `Failed`, `Cannot` and `cannot` matched no line.

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

The colour is the same as the ordinary shading path's. The single number differs in its fourth decimal place onward: 0.2847999930381775 here against 0.2848494350910187 on the Mac and 0.28484946489334106 on the rented Linux machine in entry 1.

### 12. The compositor (entry 12)

**What was done.** A scene with compositing switched on, an Image node loading the picture, its Image output into the `Fac` input of a Color Ramp node left at its default black-to-white, and the Color Ramp into the Composite node. In 4.5 this was built through `scene.use_nodes` and `scene.node_tree`, and the node types available were printed: `['CompositorNodeImage', 'CompositorNodeValToRGB', 'CompositorNodeComposite']`.

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

**What was done.** A real OpenVDB file was produced first, by baking a small smoke simulation with the cache format set to OpenVDB, so that the test has a control showing a present volume file does render: `BAKE RETURNED {'FINISHED'}` and `CACHE FILE .../fluid_data_0001.vdb 57570`. That file was given to a Volume object with a Principled Volume material at emission strength 1, in front of an orthographic camera, rendered with Cycles at 128 samples. Three renders: file present, volume object removed altogether, file moved away.

**What came back, the same on both computers**, as the mean and largest pixel value over the frame:

```
RESULT present   mean/max: (0.5932814478874207, 2.500030279159546)
RESULT novolume  mean/max: (0.0, 0.0)
RESULT missing   mean/max: (0.0, 0.0)
```

The missing-file render matched the render with no volume object at all. Nothing in either run's output matched a search for `ERROR`, `WARNING`, `Error` or `Traceback`.

### The second batch of hand tests — 2026-08-09

The three results that follow were run on the same day as the fourteen above, after the citation check found three of those fourteen recorded too thinly for anybody to repeat. They are entries 15, 16 and 17 in the hand-test list. Nothing above is changed or withdrawn by them. Both computers are the two named at the head of this section, and both were used for all three.

### 15. A missing picture feeding a normal map, a bump height and a displacement height, rebuilt so it can be repeated (entry 15)

> **Why there are two of these.** The citation check found that result 2 above cannot be repeated from its own words and wrote out a test with every measurement in it. That test is entry 15 in the hand-test list, and this is what it gave. **Result 2 above is untouched and its figures stand.** This is a second run over the same ground, not a correction of it, and the two set-ups are different so the figures are not meant to match.

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

> **Why there is a second one.** The citation check found that the Workbench half of result 5 above cannot produce its own numbers from its own words, and wrote the test out again with the two missing settings in it. That test is entry 16 in the hand-test list. Result 5 above is untouched.

**What was done.** The `col` blend-file from result 1 above — a plane filling an orthographic camera, an Emission-only material, the Image Texture node's Colour into Emission Colour, 32 by 32 pixels, OpenEXR at full float, view transform `Standard`. The render engine was set to `BLENDER_WORKBENCH` and the frame rendered twice in the same run: once with nothing else touched, and once with `scene.display.shading.color_type` set to `TEXTURE` and `scene.display.shading.light` set to `FLAT`. Both were run with the picture present and again with it moved out of the folder. Four renders. The two settings were printed before and after each change.

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

> **Why there is a second one.** The citation check found that result 14 above does not record the frame, the camera, the world or where the volume sits, and did not re-run it. It asked for a test with all of that written down. That test is entry 17 in the hand-test list. Result 14 above is untouched.

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

### What these tests did not settle

- **Windows.** No Windows machine has run it. Everything above settles macOS on Apple silicon and Linux on an NVIDIA card, and leaves Windows untested rather than answered.
- **The second substitute value a tile can be given when it is asked for but not yet in memory.** No set-up is known that reliably holds a tile in that state at the moment a lookup happens; it is a passing state during a render, not something a scene can be built to sit in. Entry 7 reaches the case where the tile's file is absent, and not that one.
- **What a panel shows.** Where the report of `File ▸ External Data ▸ Report Missing Files` appears when there **is** a window was dropped this round: it needs eyes on a window, the rented Linux machine has no display server, and the Creator ran nothing. What the same command does with no window is a different test and is result 4 above.

---

## Things a person with Blender open could settle in a few minutes

Every item that stood here has been run, and the results are in the section above. Nothing from this list is still open.

Two things are still open and are not on this list: Windows, which no Windows machine has run, and the second substitute value a tile can be given when it is asked for but not yet in memory, which no machine this project has can reach. Both are named under *What these tests did not settle* above.

---

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 4.5

# Images — later questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual, Blender 5.0 Python API documentation, Blender 5.0 release notes, and Cycles' own files for 5.0 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is my date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 9 was assigned. Questions 1–8 are untouched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

> **Changed by the citation check, 2026-08-09 (Opus 5).** Two things were changed, each marked *[citation check]* where it sits: the address for the colour-to-number conversion was corrected from lines 28–33 to lines 29–34 (the first pass for 5.0 had it right); and a sentence claiming the render still completes was cut, because nothing quoted here carries it. Three hand-test entries were annotated — see entries 2, 5 and 14.

**How I read the question.** A blend-file names an image file on disk; that file is not there; and the image texture's output is wired to a material input that is not a colour — roughness, metallic, a displacement height, a normal map, and so on. I answer what appears in the rendered picture at that spot.

The question is ambiguous in one place. "Feeding something other than colour" could also mean the image is marked as **Non-Color** data (Blender's setting for images that hold numbers rather than colours). Both readings land on the same finding below, and I say where each one shows up, so nothing is lost either way. I flag it rather than choose silently.

**LABEL: observed** (from Cycles' own files), with one **inferred** step named explicitly below.

### What the files say

Cycles keeps a single fixed colour for this case, and the comment above it says what it is for.

> ```
> /* Color to use when textures are not found. */
> enum {
>   TEX_IMAGE_MISSING_R = 1,
>   TEX_IMAGE_MISSING_G = 0,
>   TEX_IMAGE_MISSING_B = 1,
>   TEX_IMAGE_MISSING_A = 1
> };
> ```

Cycles' own files, `source/util/texture.h`, lines 12–18. Read 2026-07-30.

That is red 1, green 0, blue 1 — magenta — with alpha 1.

The image texture lookup returns exactly that whenever there is no image behind the node:

> ```
> ccl_device float4
> svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
> {
>   if (id == -1) {
>     return make_float4(
>         TEX_IMAGE_MISSING_R, TEX_IMAGE_MISSING_G, TEX_IMAGE_MISSING_B, TEX_IMAGE_MISSING_A);
>   }
> ```

Cycles' own files, `source/kernel/svm/image.h`, lines 20–26. Read 2026-07-30.

Three things follow from those lines that matter to the question.

**One — the substitution happens at the lookup, not at the destination.** The same result is then written to whatever the node's outputs feed:

> ```
>   const float4 f = svm_image_texture(kg, id, tex_co.x, tex_co.y, flags);
>
>   if (stack_valid(out_offset)) {
>     stack_store_float3(stack, out_offset, make_float3(f.x, f.y, f.z));
>   }
>   if (stack_valid(alpha_offset)) {
>     stack_store_float(stack, alpha_offset, f.w);
>   }
> ```

Cycles' own files, `source/kernel/svm/image.h`, lines 119–126. Read 2026-07-30.

Nothing in that code asks where the value is going. So the substitute value is the same whether the texture feeds base colour, roughness, a displacement height or a normal — it is not a colour-only behaviour. The Environment Texture node runs through the same lookup (`source/kernel/svm/image.h`, line 263), as does the box-projection variant (lines 218, 222, 226).

**Two — the colour-space handling is skipped.** The missing-image branch returns before the two colour adjustments in the same function:

> ```
>   if ((flags & NODE_IMAGE_ALPHA_UNASSOCIATE) && alpha != 1.0f && alpha != 0.0f) {
>     r /= alpha;
>     r.w = alpha;
>   }
>
>   if (flags & NODE_IMAGE_COMPRESS_AS_SRGB) {
>     r = color_srgb_to_linear_v4(r);
>   }
> ```

Cycles' own files, `source/kernel/svm/image.h`, lines 31–38. Read 2026-07-30.

This is where the second reading of the question lands: an image marked Non-Color gets the same substitute value as any other, because the branch that would have treated the two differently is never reached. The substitute is 1, 0, 1, 1 either way.

**Three — what a non-colour input then receives.** A colour arriving at a single-value input is converted. Cycles has that conversion in two places, and they agree in shape.

In the compiled shading path:

> ```
>     case NODE_CONVERT_CF: {
>       const float3 f = stack_load_float3(stack, from);
>       const float g = linear_rgb_to_gray(kg, f);
>       stack_store_float(stack, to, g);
>       break;
>     }
> ```

Cycles' own files, `source/kernel/svm/convert.h`, lines 29–34 *[citation check: was 28–33]*. Read 2026-07-30. And:

> ```
> ccl_device float linear_rgb_to_gray(KernelGlobals kg, const float3 c)
> {
>   return dot(c, make_float3(kernel_data.film.rgb_to_y));
> }
> ```

Cycles' own files, `source/kernel/util/colorspace.h`, lines 34–37. Read 2026-07-30. The three weights (`rgb_to_y`) are filled in from the scene at render time; the value they take is not in the files I was given.

In the Open Shading Language path the weights are compiled in, and the original source line is preserved as a comment inside the compiled file:

> ```
> #   value_float = value_color[0] * 0.2126 + value_color[1] * 0.7152 + value_color[2] * 0.0722;
> ```
> ```
> #   value_vector = vector(value_color[0], value_color[1], value_color[2]);
> ```
> ```
> #   value_normal = normal(value_color[0], value_color[1], value_color[2]);
> ```

Cycles' own files, `shader/node_convert_from_color.oso`, in the `___main___` code block (the three commented source lines 15, 17 and 19 of `node_convert_from_color.osl`). Read 2026-07-30.

Put the substitute value through those: a single-value input receives 0.2126 × 1 + 0.7152 × 0 + 0.0722 × 1 = **0.2848**. A vector, point or normal input receives **(1, 0, 1)** unchanged, component for component.

### The answer

Where the image file is missing, Cycles renders that spot using a fixed stand-in value of red 1, green 0, blue 1 with alpha 1 — magenta — in place of every pixel it would have read from the file. It does this at the point of the lookup, so it does not matter what the texture feeds. If it feeds a single-value input such as roughness, that input gets the grey value of magenta (0.2848 under the weights compiled into the Open Shading Language path). If it feeds a vector or normal input, that input gets 1, 0, 1. *[citation check: "The render still completes; nothing is left blank and no pixel is left unshaded" was cut — no line quoted in this document bears on whether a render finishes. The hand tests below settle it.]*

So a missing image on a non-colour input does not usually show up as visible magenta. It shows up as a constant, wrong number applied evenly across the whole surface — for example a uniformly medium-rough surface, or a uniform displacement — with no patterning, because the stand-in is one value everywhere rather than an image.

### The inferred step, named

The files show: *if the lookup has no image behind it (`id == -1`), return magenta*, under a comment reading "Color to use when textures are not found." They do **not** show the code that decides to hand the kernel `id == -1`. That code lives in the part of Cycles that loads images and talks to Blender, and that part is not in the folder I was given (the folder holds only Cycles' kernel, its utilities and its Open Shading Language shaders — 403 files, no image-loading or scene-building code). Reading "the file on disk is missing" as the case that produces `id == -1` is a step I took from the comment's wording, not something the files state. Label that step **inferred**.

The step about the single-value conversion is also partly inferred: the conversion code exists and is named for converting a colour to a value, but the code that inserts it into a shader when a colour output is wired to a value input is not in the folder either.

### Not covered

- What each specific non-colour consumer then renders. I can say a normal-map input receives (1, 0, 1) and a height input receives 0.2848; I cannot say from these files what surface that produces, because the nodes that consume those values do more work that I did not trace.
- Whether Blender prints a warning, writes a line to the console, or fails the render when an image file is missing. Nothing in the manual, the API documentation, the release notes or the Cycles files I was given says.
- EEVEE. Every finding above is from Cycles' own files. EEVEE's code is not in the folder, and the manual does not describe EEVEE's handling of missing images. Whether EEVEE substitutes the same value is unknown to me.
- The compositor, the video sequencer and Grease Pencil are separate paths and are not covered by the Cycles code above. (The sequencer has its own, differently-controlled behaviour — see "what I ruled out" below.)
- Images that are *packed* into the blend-file, or images whose file exists but cannot be decoded. The comment says "not found", which I read as absent; a corrupt-but-present file is a different case the files do not distinguish.
- The value of `rgb_to_y` in the compiled shading path. The Open Shading Language weights are fixed at 0.2126 / 0.7152 / 0.0722; the compiled path's weights come from the scene and are not in these files, so the two paths may not give identical numbers.

### Outside the interface

Yes, this holds. Everything quoted above is in Cycles' render kernel — the code that computes pixels. It runs the same way whether a person pressed a button or the render was started by something else, because nothing in that code path consults the interface. There is no setting in any of these files that switches the substitution on or off.

What differs outside the interface is only the *remedy*, not the behaviour. The manual's two tools for missing files are menu commands:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files.
>
> Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up."

Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html. Read 2026-07-30.

Both also exist as commands that can be driven from outside:

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"

and

> "Try to find missing external files"

Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.file.html. Read 2026-07-30. The reporting one puts its warning in an editor header, which nothing is watching when there is no window; the sources do not say where that warning goes in that case.

**Settled by hand test, 2026-08-09 — observed.** Every result in the *Hand-test results* section below was produced with Blender driven from a terminal with no window, on both computers. The substitution happened, the render finished, the frame was written, and the command ended with exit status 0. Standard error was empty. The reporting side is settled too: `bpy.ops.file.report_missing_files()` returned `{'FINISHED'}` and printed to the ordinary output — `Info: No missing files` where the file was there, and `Warning: Path '<path>/flat.png' not found, from local data-block 'IMflat.png'` where it was not — so the report does reach a terminal when there is no Info editor. The render itself printed one line to the ordinary output, `cycles | ERROR Image file '<path>/flat.png' does not exist.`

### Operating system

The sources do not distinguish. The constant and the branch are in code that is compiled for every processor and graphics card Cycles supports — the same header is used by the graphics-card variants (`source/kernel/util/texture_3d.h` line 225 uses the same four values), and nothing near it is conditional on an operating system. I found no statement anywhere in the manual, the API documentation or the release notes tying this behaviour to a particular operating system. One caution the files do not resolve: what counts as a missing file depends on how a path is resolved, and path handling does differ between operating systems; that part is decided by code not in the folder I was given.

**Settled by hand test, 2026-08-09 — observed.** Every test in the *Hand-test results* section below was run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every test. The substitute values, the lines printed, the exit statuses and the pictures written matched; the only differences were in the last digits of a few figures, and those are given where they occur. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked

Searched the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` from the top, not file by file.

**Terms searched, across the manual and the API documentation:** `missing`, `magenta`, `pink`, `purple`, `not found`, `cannot be found`, `could not be found`, `does not exist`, `no longer exist`, `not exist`, `absent`, `unable to load`, `fails to load`, `failed to load`, `cannot load`, `could not open`, `cannot be opened`, `broken`, `unresolved`, `placeholder`, `invalid path`, `no image`, `empty image`, `solid color`, `fallback`, `255, 0, 255`, `1, 0, 1`, `Non-Color`, `find_missing_files`, `report_missing_files`, `has_data`, `is_missing`.

**Terms searched in Cycles' own files:** `missing`, `TEX_IMAGE_MISSING`, `magenta`, `id == -1`, `id < 0`, `linear_rgb_to_gray`, `svm_image_texture`, `missingcolor`, `missingalpha`, `texture`, `not found`, `error`.

**Pages I opened and what they gave:**

- https://docs.blender.org/manual/en/5.0/render/shader_nodes/textures/image.html — the Image Texture node's own page. Read in full. It describes inputs, projection, extension, interpolation, colour space, alpha and the two outputs. It says nothing about a missing file. Ruled out.
- https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/texture/image.html — the geometry-nodes Image Texture node, read in full because that node can drive geometry rather than colour, which fits "something other than colour". Nothing about missing files. Ruled out.
- https://docs.blender.org/manual/en/5.0/render/eevee/limitations/nodes_support.html — read in full in case EEVEE documented different handling. It lists unsupported features; missing images are not among them. Ruled out.
- https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — the External Data menu. Documents Report Missing Files and Find Missing Files, quoted above. Tells you how to find and fix missing links; says nothing about what a render shows meanwhile.
- https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html — the Image data-block settings, including Source and Color Space. Nothing about a source file being absent.
- https://docs.blender.org/api/5.0/bpy.types.Image.html — checked `source` (values FILE, SEQUENCE, MOVIE, GENERATED, VIEWER, TILED — none of which is a "missing" state) and `has_data` ("True if the image data is loaded into memory"). Neither says what a render does.
- https://docs.blender.org/api/5.0/bpy.ops.file.html — the two missing-file commands, quoted above.
- https://docs.blender.org/api/5.0/bpy.types.ShaderNodeTexImage.html and https://docs.blender.org/api/5.0/bpy.types.ShaderNodeTexEnvironment.html — the only "not found" text on these pages is boilerplate about looking up Python types. Ruled out.
- Release notes for 5.0, all 23 subject files, searched for `missing`, `magenta`, `texture not found`, `placeholder`. Every hit was in the bug-fix list and about something else (a crash with missing linked data-blocks, missing shadows in EEVEE, missing defaults in UV relax, and so on). Nothing about missing image textures in a render. Ruled out.
- https://docs.blender.org/manual/en/5.0/advanced/command_line/ — searched the whole command-line section for `missing` and `texture`. No hits. Nothing there about missing files during an unattended render.
- The troubleshooting section of the manual — searched for `missing` and `image`. Only startup and Python problems. Ruled out.

**The near-miss I ruled out, and why.** The one place in the whole manual that ties missing files to magenta is the Video Sequencer:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."

Manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html. Read 2026-07-30. The same wording appears in the API documentation as `show_missing_media` on https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html.

This is **not** an answer to the question. It is a display option belonging to the video sequencer's preview, about strips of footage, not about an image texture feeding a material input in a 3D render. It has an off switch; the Cycles behaviour above has none. I mention it because anyone searching the manual for "missing" and "magenta" will land here first and could easily mistake it for the answer.

**Where the answer is not.** The manual never states what a render produces for a missing image texture — not on the node's page, not in the render section, not in troubleshooting, not in the colour-management pages, not in the file-handling pages. Neither does the API documentation, nor the release notes. That silence is why the answer above rests on Cycles' own files.

---

## Hand-test results — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md`; the entry numbers below are that list's numbers.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before the run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, because Blender writes text into a saved picture that nothing controls, so pictures were compared as pixels and never as files. "Missing" below always means the picture file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 5.0.1, from `/opt/blender/5.0/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.0.1.

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

The rented Linux machine printed the same, with one line differing in its last digits:

```
RESULT grey_cpu_default missing [0.2848318815231323, 0.2848318815231323, 0.2848318815231323, 1.0]
```

The two machines agreed. On both, the picture-present runs gave red 0, green 1, blue 0 and an alpha of 0.250980406999588, which is the control showing the file really was being read.

So, with the picture missing: the node's Color output carried red 1, green 0, blue 1; its Alpha output carried 1; and a colour output plugged into a socket taking one number gave 0.2848318 on the Mac and 0.2848319 on the rented Linux machine. The graphics card gave the same numbers as the processor. Setting the node's Colour Space to `Non-Color` gave the same numbers as leaving it at `sRGB`.

### 2. A missing picture feeding a normal map, a bump height and a displacement height (entry 2)

> **Note from the citation check, 2026-08-09 (Opus 5).** This entry cannot be repeated from its own words. It does not say how big the spheres are, where they sit, how the camera is framed, which way the sun points or how bright it is, or what colour the surfaces are — and every number below depends on all of those. Re-running it with those details invented reproduces the three results in kind (the normal-map sphere differs from the unwired one; the bump sphere does not; the displacement sphere's outline sits between the two) but not the figures. A test somebody can follow is written out in the hand-test list under this check's heading.

**What was done.** A 64 by 64 black-and-white checker was written. Three UV-unwrapped spheres of 128 by 64 segments sat side by side under one sun lamp with its angle set to 0, against a black world, with Cycles at four samples and no light bounces, so that two identical set-ups give identical pixels. The left sphere took the picture through a Normal Map node into a Diffuse BSDF's Normal input; the middle one through a Bump node's Height with Distance 5; the right one through a Displacement node's Height at scale 0.4 into the Material Output, with the material's displacement method set to `DISPLACEMENT`. Three renders were made: picture present and wired; picture moved away and wired; picture present with the link out of each Image Texture node removed. The frame was 240 by 80 with a transparent film, so each sphere has its own third of the frame and the number of pixels inside its outline can be counted.

**What came back.** The Creator's Mac printed:

```
DIFF normalmap  present_vs_unwired mean 0.083631  max 0.661449
DIFF normalmap  missing_vs_unwired mean 0.056781  max 0.525166
DIFF normalmap  missing_vs_present mean 0.048984  max 0.603759
SILHOUETTE normalmap  present  pixels 2224
SILHOUETTE normalmap  missing  pixels 2224
SILHOUETTE normalmap  unwired  pixels 2224
DIFF bump       present_vs_unwired mean 0.021260  max 0.640054
DIFF bump       missing_vs_unwired mean 0.000041  max 0.006264
DIFF bump       missing_vs_present mean 0.021222  max 0.640054
SILHOUETTE bump       present  pixels 2225
SILHOUETTE bump       missing  pixels 2225
SILHOUETTE bump       unwired  pixels 2225
DIFF displace   present_vs_unwired mean 0.109438  max 0.655591
DIFF displace   missing_vs_unwired mean 0.022522  max 0.645492
DIFF displace   missing_vs_present mean 0.101934  max 0.655591
SILHOUETTE displace   present  pixels 3126
SILHOUETTE displace   missing  pixels 1848
SILHOUETTE displace   unwired  pixels 1424
```

The rented Linux machine printed the same, with three lines differing in their last digits: `DIFF normalmap present_vs_unwired mean 0.083615 max 0.661449`, `DIFF normalmap missing_vs_present mean 0.048968 max 0.603759`, and `DIFF bump missing_vs_unwired mean 0.000041 max 0.006257`. Every silhouette count matched.

The `present_vs_unwired` line in each group is the control showing that each of the three wirings does change the picture when the file is there.

So: with the picture missing, the normal-map sphere differed from the same sphere with nothing connected. The bump sphere did not — the largest difference of any single channel anywhere on it was 0.006264 on the Mac and 0.006257 on the rented Linux machine, against 0.64 for the control. The displacement sphere's outline held 1848 pixels with the picture missing, 3126 with it present, and 1424 with nothing connected; the undisplaced spheres beside it held 2224 and 2225.

### 3. What the render says, whether it finishes, and what it writes (entry 3)

**What was done.** The Emission plane from entry 1, with its Color wired to Emission Color, was rendered from a terminal with the ordinary output and the error output captured separately, once with the picture present and once with it moved away. The exact command was `BLENDER_USER_RESOURCES=<fresh folder> <blender> --background --factory-startup <work>/col.blend --render-output <work>/missing_ --render-frame 1`.

**What came back, on both computers.** Both runs ended with exit status 0 and both wrote their frame. Standard error was empty, 0 bytes, in both runs. The missing-picture run printed one line that the picture-present run did not. On the Creator's Mac:

```
00:00.268  cycles           | ERROR Image file '<work folder>/flat.png' does not exist.
```

and on the rented Linux machine:

```
00:00.572  cycles           | ERROR Image file '/root/ht-images/linuxC/e3-5.0/flat.png' does not exist.
```

It sits on standard output, not on standard error, and it names the file. Note the single quotes around the path.

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

**What came back.** In 5.0, `BLENDER_EEVEE_NEXT` was refused:

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

5.0 printed a line in four of the six states, and the wording is not the same for all four:

```
moved:     cycles | ERROR Image file '<path>/flat.png' does not exist.
badfolder: cycles | ERROR Image file '<path>/nosuchfolder/flat.png' does not exist.
zerobyte:  cycles | ERROR Image file '<path>/flat.png' failed to load.
garbage:   cycles | ERROR Image file '<path>/flat.png' failed to load.
```

The `present` and `wrongext` states printed no such line.

A seventh state was run only on the rented Linux machine, because everything there otherwise runs as root and root reads past permissions without complaint. The work folder was handed to the ordinary user `tester` and Blender was run as that user. With the picture set to `chmod 644` the run printed `RESULT permok [0.0, 1.0, 0.0, 1.0]`; with it set to `chmod 600` and owned by root it printed `RESULT permdenied [1.0, 0.0, 1.0, 1.0]`, together with `cycles | ERROR Image file '<path>/flat.png' failed to load.`. That the permission really bit was shown in the same run:

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

The colour is the same as the ordinary shading path's. The single number differs in its fifth decimal place onward: 0.2847999930381775 here against 0.28483182191848755 on the Mac and 0.2848318815231323 on the rented Linux machine in entry 1.

### 12. The compositor (entry 12)

**What was first tried, and what came back.** The 4.5 route was tried first and failed on both computers:

```
Traceback (most recent call last):
AttributeError: 'Scene' object has no attribute 'node_tree'
```

**What was then done.** 5.0 holds the compositor as a node group on `scene.compositing_node_group`, so the scene was built that way instead: an Image node loading the picture, its Image output into the `Factor` input of a Color Ramp node left at its default black-to-white, and the Color Ramp's Color into a Group Output whose interface socket is a colour. `CompositorNodeValToRGB` and `CompositorNodeComposite` do not exist in 5.0; the node types used were printed:

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
RESULT present   mean/max: (0.23706698417663574, 2.003127098083496)
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

**What came back, first — with the framing exactly as it was written out.** The check's framing puts the three spheres at x = −1, 0, +1 in an orthographic width of 3. Run that way, the displaced sphere runs off the picture. Counting lit pixels on each edge row of the frame, the picture-present render gave top 52, bottom 52, left 0, right 52, on both computers; the other three renders touched no edge at all. In the same render the middle third of the frame held 3464 lit pixels against 3209 in every other state, because part of the displaced sphere had grown into its neighbour's third. The displaced sphere's own third counted 5873 present, 1973 missing, 804 nothing-connected, 3214 plain, and those counts are cut short by the frame edge rather than measured. So the figures from that framing are recorded here as what happened and are not used to answer anything.

**What came back, second — with the framing widened.** Two numbers were changed and nothing else: the orthographic width to 4.5 and the spheres to x = −1.5, 0, +1.5. Nothing touched any edge of the frame in any of the four renders, on either computer. The figures below are the mean and the largest single-channel difference over each third of the frame; the two computers agreed except in the last digits of one mean.

```
DIFF normalmap  present_vs_unwired mean 0.042176  max 0.501700
DIFF normalmap  missing_vs_unwired mean 0.033043  max 0.398391
DIFF normalmap  unwired_vs_nonode  mean 0.000000  max 0.000000
DIFF normalmap  missing_vs_nonode  mean 0.033043  max 0.398391
DIFF bump       present_vs_unwired mean 0.011695  max 0.487685
DIFF bump       missing_vs_unwired mean 0.000036  max 0.012493
DIFF bump       unwired_vs_nonode  mean 0.000070  max 0.012500
DIFF displace   present_vs_unwired mean 0.103587  max 0.499832
DIFF displace   missing_vs_unwired mean 0.022044  max 0.501910
DIFF displace   unwired_vs_nonode  mean 0.045120  max 0.502020
DIFF displace   missing_vs_nonode  mean 0.023251  max 0.495387
SILHOUETTE normalmap  present 1416  missing 1416  unwired 1416  nonode 1416
SILHOUETTE bump       present 1416  missing 1416  unwired 1416  nonode 1416
SILHOUETTE displace   present 3048  missing 874  unwired 350  nonode 1424
```

That block is the Creator's Mac. The rented Linux machine gave `present_vs_unwired mean 0.042180` for the normal map where the Mac gave 0.042176, and every other figure above to the digit printed.

**The controls.** `present_vs_unwired` in each group is the first control: each of the three wirings changes the picture when the file is really there. `unwired_vs_nonode` for the normal map is the second: a Normal Map node fed 0.5, 0.5, 1 gives a picture identical to no Normal Map node at all, mean 0.000000 and largest 0.000000, so "unchanged" has a fixed meaning in this run.

**So, with the picture missing:**

- **The normal map.** The surface is not left as it was. The missing-picture render differs from the plain sphere by mean 0.033043 and largest 0.398391 of a channel, against a control that shows what "identical" looks like here — exactly zero.
- **The bump height.** The surface is left as it was. The largest single-channel difference anywhere on that sphere between the missing-picture render and the nothing-connected render was 0.012493, against 0.487685 for the control.
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

Then the scene, with every number recorded: a Volume data-block pointing at `//vol.vdb`, on an object at location (0, 0, 0), rotation 0, scale (1, 1, 1); a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; the frame 64 by 64 with the film **not** transparent; a world data-block with its nodes switched off and its colour black; Cycles at 128 samples with denoising off. With the file loaded, the object's dimensions read (3.75, 3.75, 5.0). In 5.0 setting the world's nodes off did not take — the property read back as True straight after it was set — so the empty frame carries the world's own default grey rather than black.

Three renders: file present; the Volume object deleted altogether; the file moved out of the folder.

**What came back, the same on both computers**, as the mean and the largest pixel value over the frame:

```
RESULT present   mean 0.23706698417663574  max 2.003127098083496
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

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 5.0

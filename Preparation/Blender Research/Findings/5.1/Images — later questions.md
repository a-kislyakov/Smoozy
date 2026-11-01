# Images — later questions (Blender 5.1)

Sources consulted, all inside `Blender Sources/`: the Blender 5.1 manual (`manual_5.1`), the Blender 5.1 Python API documentation (`api_5.1`), the Blender 5.1 release notes (`release_notes_5.1`), and Cycles' own files (`cycles_5.1`). All four are a snapshot taken on 2026-07-30, and that is my date read for everything below. I did the reading on 2026-08-09.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

> **Changed by the citation check, 2026-08-09 (Opus 5).** One thing was changed, marked *[citation check]* where it sits: `#ifdef __KERNEL_GPU__` was corrected to `#ifndef __KERNEL_GPU__`, which is what the file says, with line numbers added. Everything else in this document checked out against the 5.1 files, quote for quote and line for line. Three hand-test entries were annotated — see entries 2, 5 and 14.

### How I read the question

The question can be read two ways, and I flag that rather than choose silently.

- **The reading I answer.** A blend-file points at an image file on disk, the file is not there when the render runs, and that image texture is wired into an input that is not a colour — a normal map, a height for bump or displacement, a roughness or metallic value, an alpha, and so on. What comes out in the rendered picture?
- **The other reading.** "Feeding something other than colour" could mean the image's own **Color Space** property is set to something non-colour rather than where it is wired. My answer happens to cover that reading too, and I say where below.

### The short answer

Nothing I read says whether the render stops, carries on, or is marked as failed — see *Not covered* below.

What Cycles' own files do show is one fixed value — full red, no green, full blue, alpha one, which is magenta — handed back in place of a picture whenever the picture slot is empty. That handing back happens **before** anything to do with colour is applied, and it comes out of one function that is the same whatever the value will be used for.

Everything after that point is arithmetic I did on those lines, not reading. On that arithmetic: a missing normal map would not give a magenta patch on screen but a wrongly tilted surface, and a missing height would give no bump at all, because the same value comes back at every point.

Below, each piece separately, with its own label. This summary carries no label of its own and is not a finding — take the labels below, not this paragraph.

---

### 9a. The substitution itself

**Label: observed** (read out of Cycles' own files).

Cycles' files hold one named value, with a comment above it saying what it is for.

> ```
> /* Color to use when images are not found. */
> #define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
> ```

Source: Cycles' own files, `cycles_5.1/source/util/types_image.h`, lines 12–13. Date read: 2026-07-30.

The four numbers are red 1, green 0, blue 1, alpha 1. That is magenta, fully opaque.

Every image lookup in the shading kernel goes through one function, and that function hands back this value the moment the picture slot is empty:

> ```
> ccl_device float4
> svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
> {
>   if (id == -1) {
>     return IMAGE_MISSING_RGBA;
>   }
>
>   float4 r = kernel_image_interp(kg, id, x, y);
> ```

Source: Cycles' own files, `cycles_5.1/source/kernel/svm/image.h`, lines 21–28. Date read: 2026-07-30.

Three things are visible in those lines and matter for the question.

1. The `x` and `y` — the position on the picture being asked for — are ignored entirely. The same value comes back at every point of the surface.
2. The return happens before the rest of the function. The rest of the function is where colour handling lives:

   > ```
   >   if (flags & NODE_IMAGE_COMPRESS_AS_SRGB) {
   >     r = color_srgb_to_linear_v4(r);
   >   }
   > ```

   Source: same file, lines 36–38. So the magenta is not put through any colour conversion. It is the same magenta whether the image's Color Space is set to a colour space or to a non-colour one. That is the second reading of the question, answered.
3. The fourth of the four numbers that come back is 1. **Inferred, not read:** if the node's **Alpha** output is that fourth number, then a missing image feeding an alpha input gives fully opaque. The quoted lines do not show where the fourth number goes, so that last step is mine.

Two more places in the same file appeared to me to call this same function — the box-projection path (`cycles_5.1/source/kernel/svm/image.h`, lines 215–223) and the Environment Texture node (same file, line 260). I did not quote their words, so that rests on my reading rather than on anything on this page, and it does not carry the further claim that a missing world image behaves the same way.

The same value is returned in the file for three-dimensional lookups: `return IMAGE_MISSING_RGBA;` at `cycles_5.1/source/kernel/util/image_3d.h`, line 258. What uses that file is not shown by the quoted line.

**One step I did not take.** These files tell me what happens when the picture slot is empty (`id == -1`). They do not contain the part of Cycles that loads image files and decides to leave the slot empty — that code is not in the folder I was given. The link from "the file on disk is missing" to "the slot is `-1`" rests on the comment above the value, which says in the source's own words that this is the "Color to use when images are not found." Treat that link as **inferred**, not observed.

---

### 9b. Feeding a normal map

**Label: inferred** — from quoted lines plus arithmetic that I write out.

The Normal Map node takes the colour and shifts it into a direction:

> ```
>   float3 color = stack_load_float3(stack, color_offset);
>   color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);
> ```

Source: Cycles' own files, `cycles_5.1/source/kernel/svm/tex_coord.h`, lines 339–340. Date read: 2026-07-30.

The step: put the missing value (1, 0, 1) into that line. You get 2 × (0.5, −0.5, 0.5) = (1, −1, 1). That is one fixed direction, the same at every point of the surface. In the tangent-space case with strength left at 1, it is then turned into the shading normal:

> ```
>     const float3 B = sign * cross(normal, tangent);
>     N = safe_normalize(to_global(color, tangent, B, normal));
> ```

Source: same file, lines 408–409.

In that line the three numbers are the amounts of `tangent`, of `B` and of `normal` that make up the new direction. So (0, 0, 1) would be the plain surface normal, and (1, −1, 1) is not it. The whole surface is therefore shaded as if tilted away from where it really faces, everywhere and by the same amount. I put no figure on the angle: working one out would need facts about the `tangent` and `B` directions that the quoted lines do not give. In the picture, the object keeps its shape and silhouette but its shading, and anything mirror-like on it, is uniformly skewed — highlights and reflections sit in the wrong place. It is not a magenta patch.

There is one escape hatch in the same function: if the tangent attribute the node needs is not present, it gives up and uses the plain surface normal instead (`stack_store_float3(stack, normal_offset, sd->N); return;`, lines 363–366). That is about a missing tangent, not a missing image, but it means the visible result also depends on whether the mesh has the map it needs.

---

### 9c. Feeding a bump height

**Label: inferred** — from quoted lines plus one step I write out.

The Bump node reads the height at three nearby points and works out the slope:

> ```
>     const float h_c = stack_load_float(stack, c_offset);
>     const float h_x = stack_load_float(stack, x_offset);
>     const float h_y = stack_load_float(stack, y_offset);
>
>     /* compute surface gradient and determinant */
>     const float det = dot(dP.dx, Rx);
>     const float3 surfgrad = (h_x - h_c) * Rx + (h_y - h_c) * Ry;
> ```

Source: Cycles' own files, `cycles_5.1/source/kernel/svm/displace.h`, lines 72–78. Date read: 2026-07-30.

The step: the missing-image lookup ignores the position asked for (9a, point 1), so all three heights are the same number. `h_x - h_c` and `h_y - h_c` are both zero, so `surfgrad` is zero. The line that then makes the new normal is:

> ```
>     float3 normal_out = safe_normalize(bump_filter_width * absdet * normal_in -
>                                        scale * signf(det) * surfgrad);
>     if (is_zero(normal_out)) {
>       normal_out = normal_in;
>     }
> ```

Source: same file, lines 98–102. With `surfgrad` zero, what is left is the incoming normal scaled by a positive number, which normalises back to the incoming normal; and if that scale is zero, the next line hands back the incoming normal anyway.

So a missing bump height produces **no bump at all**. The surface renders perfectly smooth. This is the case most likely to go unnoticed: there is no magenta and no obvious wrongness, just missing detail.

---

### 9d. Feeding a plain number — roughness, metallic, an amount

**Label: inferred** — from quoted lines; one number I cannot supply.

Cycles has a step that turns a colour into a single number. **The step I supply:** that this is what stands between an image texture's colour output and an input that wants one number. The lines below show the conversion existing; they do not show that it is what gets used here.

> ```
>     case NODE_CONVERT_CF: {
>       const float3 f = stack_load_float3(stack, from);
>       const float g = linear_rgb_to_gray(kg, f);
>       stack_store_float(stack, to, g);
> ```

Source: Cycles' own files, `cycles_5.1/source/kernel/svm/convert.h`, lines 29–33. And:

> ```
> ccl_device float linear_rgb_to_gray(KernelGlobals kg, const float3 c)
> {
>   return dot(c, make_float3(kernel_data.film.rgb_to_y));
> }
> ```

Source: Cycles' own files, `cycles_5.1/source/kernel/util/colorspace.h`, lines 34–37. Date read: 2026-07-30.

So the input receives the brightness of magenta — the red weight plus the blue weight, with green counting for nothing. That is one fixed number, the same over the whole surface. In the picture: a uniform wrong value where a varying one was intended. A missing roughness map gives one roughness everywhere, a missing metallic map one metallic value everywhere, and so on.

**What I cannot give you.** The actual number. The weights (`rgb_to_y`) are set outside the files I was given, and they follow the scene's colour settings, so I will not invent a figure.

---

### 9e. Feeding a height into displacement

**Label: inferred** — from quoted lines.

The lines below are the displacement part of that file. They do not say which of Blender's displacement settings reaches them, so nothing here separates displacement that moves the mesh from displacement done in the shading alone.

> ```
>     float3 dP = normal;
>
>     if (space == NODE_NORMAL_MAP_OBJECT) {
>       /* Object space. */
>       object_inverse_normal_transform(kg, sd, &dP);
>       dP *= (height - midlevel) * scale;
>       object_dir_transform(kg, sd, &dP);
>     }
>     else {
>       /* World space. */
>       dP *= (height - midlevel) * scale;
>     }
> ```

Source: Cycles' own files, `cycles_5.1/source/kernel/svm/displace.h`, lines 157–168. Date read: 2026-07-30.

The height is the same everywhere (9a, 9d), so every point moves along its own normal by the same distance: `(that fixed number − midlevel) × scale`. I cannot give that fixed number (9d), so I cannot say how far, in which direction, or whether it is zero. What does follow from the quoted lines alone is that the move is identical at every point: a uniform swelling or shrinking rather than any relief, and no change at all if the number happens to equal the midlevel. The shape stays smooth either way.

---

### Not covered

- **The other render engine.** Everything above is Cycles. EEVEE's own files were not among my sources, and the manual and API documentation say nothing about missing images in either engine. What EEVEE puts on screen for a missing image feeding a non-colour input is **not in the sources**.
- **The Open Shading Language path.** Cycles has a setting `shading_system`, named "Open Shading Language", described as "Use Open Shading Language" (`cycles_5.1/properties.py`, lines 388–391). That path does not use the function quoted in 9a. Its image texture shader calls a texture lookup (`cycles_5.1/shader/node_image_texture.oso`, line 505, beginning `texture ___419_rgb filename`). I read the rest of that line as setting no missing-colour option, but I have not quoted the rest of it here, so treat that as my reading rather than as words on this page. The code that would decide what a missing file returns there is not in the folder I was given. So the answer above may not hold when Open Shading Language is switched on. Not settled.
- **The link from "file gone" to "empty slot".** See the note at the end of 9a. The loading code is not in these files.
- **Whether anything is written to the log.** I did not find any statement about a message, warning, or error being produced when an image file is missing at render time. Not in the sources.
- **Whether the render is marked as failed.** Nothing in what I read suggests the render stops or is flagged, and nothing states that it does not. Not settled by the sources.
- **Packed images, image sequences with only some frames missing, UDIM tile sets with only some tiles missing.** The `id == -1` line is per-lookup and the tile-picking code sits above it in the same file, but I did not follow those branches far enough to state what happens, so I do not.
- **Exact pixel values.** I did not run a render. Everything in 9b–9e is arithmetic on quoted lines, not measurement.

### Outside the interface

The lines quoted above are all in Cycles' render kernel — the part that computes the picture — and in the shared value file next to it. None of them refers to Blender's window, to a user, or to anything about how Blender was started. **Inferred:** the behaviour is the same when Blender is driven from outside its own window as when a person clicks. The step: the code that decides this sits in the rendering path, and there is no branch in it that tests for an interface. The sources do not state this in words.

One difference I can point at, though it is not the render itself. **Documented:** the manual describes two commands for finding and fixing missing files — "Report Missing Files", which produces "a warning message … in the Info editor's header", and "Find Missing Files", which opens "A File Browser" (manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30). **Inferred** from those words: both are things a person picks from a menu and looks at, so neither happens by itself when Blender is driven from outside. What the render does meanwhile is not stated in those words, and is covered by the labels above.

**Settled by hand test, 2026-08-09 — observed.** Every result in the *Hand-test results* section below was produced with Blender driven from a terminal with no window, on both computers. The substitution happened, the render finished, the frame was written, and the command ended with exit status 0. Standard error was empty. The reporting side is settled too: `bpy.ops.file.report_missing_files()` returned `{'FINISHED'}` and printed to the ordinary output — `Info: No missing files` where the file was there, and `Warning: Path '<path>/flat.png' not found, from local data-block 'IMflat.png'` where it was not — so the report does reach a terminal when there is no Info editor. The render itself printed one line to the ordinary output, `cycles | ERROR Image file <path>/flat.png does not exist.`

### Operating system

Nothing in the quoted lines is conditional on the operating system. The one place the code does branch is on the kind of processor doing the work — for example `#ifndef __KERNEL_GPU__` (lines 20 and 261) and the Metal-specific attribute at lines 178 to 179 in `cycles_5.1/source/kernel/util/image_3d.h` *[citation check: this said `#ifdef`; the file says `#ifndef`]* — and the missing-image value is defined once, in a shared file, for all of them. **Inferred:** the answer holds on every operating system Blender runs on. The step: a single shared definition with no operating-system branch around it. The sources do not state this in words. I found no statement anywhere in the manual, the API documentation or the release notes that ties missing-image behaviour to an operating system.

**Settled by hand test, 2026-08-09 — observed.** Every test in the *Hand-test results* section below was run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every test. The substitute values, the lines printed, the exit statuses and the pictures written matched; the only differences were in the last digits of a few figures, and those are given where they occur. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked

Searches run across the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` (case-insensitive), plus `cycles_5.1`:

- `magenta`, `pink`, `purple` — in the manual, the only hits about a missing thing are in the video sequencer: "Render missing images/movies with a solid magenta color." and "When disabled, missing content will render fully transparent." (`editors/video_sequencer/preview/sidebar.txt`, lines 103–104, page https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html). That is the sequencer, not shading, so I did not use it. The same words appear at https://docs.blender.org/api/5.1/bpy.types.SequenceEditor.html. All other hits are about interface colours (curve handles, bone display, keyframes) and are unrelated.
- `missing file`, `missing image`, `missing texture`, `file not found`, `not found`, `cannot be found`, `could not be found`, `broken path`, `broken link` — the only manual pages about missing external files are "Report Missing Files" and "Find Missing Files" (https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html), and "Removes files from the list that cannot be found" on https://docs.blender.org/manual/en/5.1/files/blend/open_save.html. None says what a render produces.
- Every occurrence of the word `missing` in the whole manual, read through. Nothing about image textures during rendering.
- `placeholder`, `fallback`, `unable to load`, `can't load`, `cannot load`, `failed to load` — nothing about missing image textures. "Placeholder" in a render context is only the output option "Create empty placeholder frames while rendering" (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html), which is about output files, not input images.
- The Image Texture node page in full — https://docs.blender.org/manual/en/5.1/render/shader_nodes/textures/image.html. It describes the inputs, the projections, the extension modes, the colour space and the two outputs. It says nothing about the image being absent.
- The Normal Map node page — https://docs.blender.org/manual/en/5.1/render/shader_nodes/displacement/normal_map.html — searched for the same terms; nothing.
- API documentation: `bpy.types.Image` (https://docs.blender.org/api/5.1/bpy.types.Image.html). The nearest thing is `has_data`, "True if the image data is loaded into memory (default False, readonly)", which reports the state but says nothing about rendering. `ID.is_missing` is about linked data-blocks, not image files: "This data-block is a place-holder for missing linked data" (https://docs.blender.org/api/5.1/bpy.types.ID.html). `bpy.ops.file.find_missing_files` and `bpy.ops.file.report_missing_files` exist in the index but describe the commands, not render behaviour.
- Release notes for 5.1 (https://developer.blender.org/docs/release_notes/5.1/): searched `missing`, `image texture`, `magenta`. The only image-texture entry is a fix, "Cycles does not respect `Straight` alpha option in image texture", which is a different subject.
- Cycles' own files: searched `magenta`, `pink`, `missing image`, `missing texture`, `missing file` — no hits. Searched `IMAGE_MISSING`, which found the three places quoted above. Read `source/kernel/svm/image.h` in full, `source/util/types_image.h` in full, `source/kernel/svm/displace.h`, `source/kernel/svm/convert.h`, `source/kernel/util/colorspace.h`, the normal-map part of `source/kernel/svm/tex_coord.h`, and the relevant part of `source/kernel/util/image_3d.h`.
- I looked for the image-loading side of Cycles (the part that would decide a slot is empty). It is not in `cycles_5.1` — that folder holds the render kernel, the compiled shaders and the add-on's Python, not the scene-building code.

I did not use the web at all, and did not need to.

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

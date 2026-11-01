# Images — later questions, third pass (Blender 5.0)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- Blender 5.0 manual (plain-text copy), pages under `https://docs.blender.org/manual/en/5.0/`
- Blender 5.0 Python API documentation (plain-text copy), pages under `https://docs.blender.org/api/5.0/`
- Blender 5.0 release notes, pages under `https://developer.blender.org/docs/release_notes/5.0/`
- Cycles' own files as shipped with Blender 5.0 (`cycles_5.0`), which have no web address and are cited by file and line

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Only question **9** was assigned to me. The rest of the sheet is untouched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

**How I read the question.** It is ambiguous in two places, so I say plainly which reading I answered.

- *"an image a file points at is missing"* — I read this as: the blend-file refers to an image file that is not on disk when the render runs. The other reading, which I did not centre on, is that one file out of a set (one frame of an image sequence, one tile of a tiled image) is missing while the rest are present. I say what the Cycles files show about the tiled case below, because the same code covers it.
- *"feeding something other than colour"* — I read this as: the image texture's output is wired into an input that is not a base colour — roughness, metallic, a normal map, a bump height, a displacement height, a mask, and so on. The other reading is that the image's own Color Space setting is not a colour space (the "Non-Color" setting). The Cycles files happen to answer both, and I cover both.

**LABEL: observed** for what Cycles' files contain; **inferred** for what a finished picture then looks like. Both parts are separated below. The manual, the Python API documentation and the release notes are **not in the sources** on this question — my record of looking is at the end of this answer.

### What the files say (observed)

Cycles carries a named colour whose only stated purpose is to stand in for a texture that could not be found. The whole of the declaration:

```
/* Color to use when textures are not found. */
enum {
  TEX_IMAGE_MISSING_R = 1,
  TEX_IMAGE_MISSING_G = 0,
  TEX_IMAGE_MISSING_B = 1,
  TEX_IMAGE_MISSING_A = 1
};
```

— `cycles_5.0/source/util/texture.h`, lines 12–18. Read 2026-07-30.

Red 1, green 0, blue 1, alpha 1. That is magenta, fully opaque.

The place it is handed out is the single function every image-texture read in Cycles' own shader machinery goes through:

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

— `cycles_5.0/source/kernel/svm/image.h`, lines 20–41. Read 2026-07-30.

Three things are visible in those lines and matter here.

1. The substitution happens **at the read**, before anything else. The four numbers are returned and the function ends.
2. The substitution happens **before** the two flag branches. The alpha handling and the colour-space conversion (`NODE_IMAGE_COMPRESS_AS_SRGB`) are never reached on that path.
3. What comes back is a four-part value, not a colour socket. The same function's result is split into the node's two outputs:

```
  const float4 f = svm_image_texture(kg, id, tex_co.x, tex_co.y, flags);

  if (stack_valid(out_offset)) {
    stack_store_float3(stack, out_offset, make_float3(f.x, f.y, f.z));
  }
  if (stack_valid(alpha_offset)) {
    stack_store_float(stack, alpha_offset, f.w);
  }
```

— `cycles_5.0/source/kernel/svm/image.h`, lines 119–126 (inside `svm_node_tex_image`). Read 2026-07-30.

So the Color output carries (1, 0, 1) and the Alpha output carries 1.

The same `svm_image_texture` is what the box projection and the environment texture use, so they are covered by the same substitution:

- box projection: three weighted calls to `svm_image_texture`, `cycles_5.0/source/kernel/svm/image.h`, lines 218, 222 and 226, with the same two-output store at lines 230 and 233.
- environment texture: `const float4 f = svm_image_texture(kg, id, uv.x, uv.y, flags);`, `cycles_5.0/source/kernel/svm/image.h`, line 263, inside `svm_node_tex_environment`, with the same two-output store at lines 266 and 269.

For a **tiled (UDIM-style) image**, the same file shows the tile search starting from the missing value and staying there when no tile matches:

```
  int id = -1;
  const int num_nodes = (int)node.y;
```

and, after the search loop,

```
      /* If we found the tile, offset the UVs to be relative to it. */
      if (id != -1) {
```

— `cycles_5.0/source/kernel/svm/image.h`, lines 78–79 and 105–106. Read 2026-07-30. The value `id` is only set from a matching tile; where no tile matches, it is still `-1` when it reaches `svm_image_texture`.

A separate fallback of the same colour exists for three-dimensional (volume) texture reads:

```
  return make_float4(
      TEX_IMAGE_MISSING_R, TEX_IMAGE_MISSING_G, TEX_IMAGE_MISSING_B, TEX_IMAGE_MISSING_A);
```

— `cycles_5.0/source/kernel/util/texture_3d.h`, lines 224–225, at the end of `kernel_tex_image_interp_3d`. Read 2026-07-30. I did not trace which conditions reach it, so I claim nothing further about it.

### What that means for the finished picture (inferred)

The step I am taking: the numbers above are what the shader receives, and I then follow those numbers through the code of the nodes they would feed, which is in the same files. Nothing here is from running anything.

**The short answer:** a missing image does not paint magenta pixels when it is feeding something other than colour. It hands the rest of the shader a *constant* value — 1, 0, 1 with alpha 1 — across the whole surface, and the picture then shows whatever that constant does in the place it was plugged into. The magenta is only literally visible when the missing image was feeding something that shows colour directly.

Traced through the code, for the common cases:

- **Into a plain number input (roughness, metallic, transmission, a mask factor).** The colour is turned into one number by `NODE_CONVERT_CF`: `const float g = linear_rgb_to_gray(kg, f);` — `cycles_5.0/source/kernel/svm/convert.h`, line 31. That function is `return dot(c, make_float3(kernel_data.film.rgb_to_y));` — `cycles_5.0/source/kernel/util/colorspace.h`, line 36. With green at 0, the result is the red weight plus the blue weight. Those weights come from scene data that is not in the files I was given, so I cannot give the exact number; what I can say is that the whole surface gets one constant value, not a varying one. The surface renders perfectly uniform in whatever that input controls.
- **Into a Normal Map node.** The node does `color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);` — `cycles_5.0/source/kernel/svm/tex_coord.h`, line 335. Magenta becomes the vector (1, −1, 1), a strongly tilted constant normal rather than the usual straight-up (0, 0, 1). Every point of the surface is shaded as if tilted the same way, so the surface looks uniformly and wrongly lit rather than magenta.
- **Into a Bump node.** Bump works from the difference between neighbouring samples: `const float3 surfgrad = (h_x - h_c) * Rx + (h_y - h_c) * Ry;` — `cycles_5.0/source/kernel/svm/displace.h`, line 78. A constant makes those differences zero, so the gradient is zero and the perturbed normal reduces to the unperturbed one. The bump simply disappears; the render looks smooth.
- **Into a Displacement node.** The height is a constant, so `sd->P += dP` — `cycles_5.0/source/kernel/svm/displace.h`, line 131 — shifts the whole surface by one uniform amount along its normal rather than sculpting it. There is no shape detail, only a uniform offset (whose size depends on the Midlevel and Scale inputs).
- **Into an alpha or transparency input from the node's Alpha output.** The Alpha output is 1 (`f.w`, above). Nothing is cut out; the surface is fully opaque where the map would have made holes.
- **Where the image was a world environment.** The environment texture returns the same magenta, so the whole background and all the light coming from it become flat magenta.

**On the colour-space reading of the question.** Because the substitution returns before the `NODE_IMAGE_COMPRESS_AS_SRGB` branch (lines 23–26 come before lines 36–38 of `cycles_5.0/source/kernel/svm/image.h`), the same four numbers come out whatever the image's Color Space is set to. The step I am taking here: that flag is the node's colour-space handling, so skipping it means the colour-space setting has no effect on the substituted value. An image set to a non-colour space — the normal case for roughness, normal and displacement maps — therefore yields exactly 1, 0, 1, 1 in the renderer's own linear space, not a converted version of it.

**What I could not establish.** The files I was given contain Cycles' render kernel and some of its utility code. They do **not** contain the part of Cycles that loads image files and decides what number `id` gets when a file cannot be opened. So the chain "file absent on disk → `id == -1`" is not shown anywhere in what I read. What is shown is the comment naming this colour as the one "to use when textures are not found", which is Cycles' own statement of the purpose, and the guard that hands it out. I treat the connection as strongly indicated by the comment but not demonstrated line by line.

### Not covered

- Nothing here covers EEVEE. The only renderer whose files I was given is Cycles, and the manual, the API documentation and the release notes say nothing about what EEVEE draws for a missing image. EEVEE could behave differently and I have no way to tell from these sources.
- Nothing here covers the Open Shading Language path inside Cycles. That path calls out to a general texture system: the compiled shader `cycles_5.0/shader/node_image_texture.oso` shows the call as `color rgb = (color)texture(` at line 504 with no missing-colour option among its arguments, and the only missing-colour entry points I found, `osl_texture_set_missingcolor_arena` and `osl_texture_set_missingcolor_alpha` in `cycles_5.0/source/kernel/osl/services_gpu.h`, lines 1307 and 1312, have empty bodies on that device. The code that would set a missing colour for OSL is not in the files I have. So a render with Open Shading Language switched on is not settled by this answer.
- The exact single number a missing image produces when it feeds a plain-number input is not settled, because the weights come from scene data not present in these files.
- Whether Blender writes a warning to the console, marks the render as failed, or stops, is not settled. Nothing I read says.
- Whether the picture Blender saves to disk differs in any way from what is shown on screen is not settled.
- The three-dimensional (volume) fallback at `texture_3d.h` line 225 is noted but not traced.

### Outside the interface

**Sources do not say, with one thing that follows from them.** No page I read distinguishes a render started by a person from one started from outside Blender's window on this point. The code that performs the substitution is the render kernel itself — `svm_image_texture` in `cycles_5.0/source/kernel/svm/image.h` — and it takes no account of any window, editor or user action; it only sees a texture identifier. On that basis I infer the substituted value is the same however the render was started. That is an inference from where the code sits, not a statement in any source.

What is clearly interface-bound is the *warning* about missing files. The manual describes it as something a person clicks and then reads:

> "This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

— Blender manual, `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`, "Report Missing Files". Read 2026-07-30.

That check is also reachable without the window, as `bpy.ops.file.report_missing_files()`, described as "Report all missing external files" — Blender Python API, `https://docs.blender.org/api/5.0/bpy.ops.file.html`, read 2026-07-30 — but where its message goes when there is no Info editor is not stated anywhere I read.

**Settled further by hand test, 2026-08-09 — observed.** The three tests in the *Hand-test results, second batch* section below were all run with Blender driven from a terminal, with no window at all, on both computers — including the rented Linux machine, which has no display server of any kind. Every one of them finished, wrote its frame and gave the numbers recorded there. Workbench, which is the viewport's own renderer, rendered there too. So what a render puts on the surface for a missing file does not depend on a window being open.

### Operating system

**Sources do not say, with one thing that follows from them.** Nothing I read ties any of this to an operating system. The constant in `cycles_5.0/source/util/texture.h` and the guard in `cycles_5.0/source/kernel/svm/image.h` are in files shared by every device Cycles builds for — the per-device folders `cuda`, `hip`, `metal`, `optix` sit under `cycles_5.0/source/kernel/device/` and contain no copy of that constant (I searched the whole of `source` for `TEX_IMAGE_MISSING`; the only three hits are the declaration and the two uses cited above). On that basis I infer the substituted value is the same on every operating system. Again, that is an inference from where the code sits.

One caveat I can see rather than guess: the processor-side image reading code (`kernel/device/cpu/image.h`) is not present in the files I was given, only the graphics-card side (`kernel/device/gpu/image.h`). So I checked the shared path, not both halves.

**Settled further by hand test, 2026-08-09 — observed.** All three tests in the *Hand-test results, second batch* section below were run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every one. Where any figure differed it differed in its last digits only, and each such difference is given where it occurs. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked, and what I ruled out

Searched the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` for each of these words and phrases, case-insensitively: *magenta*, *pink*, *purple*, *missing*, *missing image*, *missing texture*, *image missing*, *textures are missing*, *missing file*, *Missing Files*, *not found*, *cannot be found*, *can't be found*, *does not exist*, *no longer exist*, *unable to load*, *failed to load*, *cannot open*, *could not open*, *unreadable*, *invalid image*, *broken*, *broken link*, *placeholder*, *fallback*, *falls back*, *renders as*, *empty image*, *no image*, *unloaded*, *error color*, *checkerboard*, *path is invalid*, *absolute path*, *relink*.

What that turned up, and why each was ruled out:

- **The only place in the whole manual that describes a magenta stand-in is the Video Sequencer, not shading.** "Show Missing Media / Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." — Blender manual, `https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html`, read 2026-07-30. The same setting in the API: "Render missing images/movies with a solid magenta color", "Type: boolean, default False" — `https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html`, read 2026-07-30. Ruled out as an answer: it is a switch on the sequencer's own preview for whole missing clips, it is off by default, and it has no bearing on an image feeding a non-colour input in a material. I record it because it is the one documented magenta, and it would be easy to mistake for this one.
- **Topbar → External Data** (`https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`), read in full around "Report Missing Files" and "Find Missing Files". It says how to detect and repair missing links. It never says what a render does with one.
- **Image Texture node page** (`https://docs.blender.org/manual/en/5.0/render/shader_nodes/textures/image.html`), read in full, all 183 lines. Every input, property and output is described. The Color output is "RGB color from image" and the Alpha output "Alpha channel from image". There is no sentence about the image being absent. I also opened the sibling pages in `render/shader_nodes/textures/` (`environment.txt`, `index.txt`) — the same silence.
- **`bpy.types.ShaderNodeTexImage` and `bpy.types.ShaderNodeTexEnvironment`** (`https://docs.blender.org/api/5.0/bpy.types.ShaderNodeTexImage.html`, `…ShaderNodeTexEnvironment.html`) — searched for *missing*, *not found*, *magenta*; the only hits are boilerplate about looking up a class. Ruled out.
- **`bpy.types.Image`** (`https://docs.blender.org/api/5.0/bpy.types.Image.html`) — the nearest thing is `has_data`, "True if the image data is loaded into memory", read 2026-07-30. That reports a state; it says nothing about what a render produces. The `ID.is_missing` entry listed on the same page belongs to linked data-blocks from other blend-files, a different subject, so I ruled it out.
- **`bpy.ops.file`** (`https://docs.blender.org/api/5.0/bpy.ops.file.html`) — `find_missing_files` ("Try to find missing external files") and `report_missing_files` ("Report all missing external files"). Repair and detection, not render behaviour.
- **Image Settings / Image editor pages** (`https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html` and neighbours) — searched for *miss*, *exist*, *gone*, *deleted*, *reload*. Only "Reload / Reloads the file. Useful when it has been reworked in an external application." Nothing about absence.
- **Release notes for 5.0** — searched every subject file for *missing*, *magenta*, *image texture*. The `missing` hits are all in `bugfixes.md` and concern unrelated things (UV relax defaults, a node group picker, USDZ export, a VSE media type, and so on). The one image-texture note is "EEVEE Image Texture node now do not clamp 32 bits HDR values to 16 bits." — `https://developer.blender.org/docs/release_notes/5.0/eevee/`, read 2026-07-30 — which is about value range, not absence. Nothing in `cycles.md`, `rendering.md`, `pipeline_io.md` or `core.md` touches this.
- **Cycles' own files** — searched the whole of `cycles_5.0/source` for `MISSING`, `missing`, `magenta`, `1.0f, 0.0f, 1.0f`, and `TEX_IMAGE_MISSING`. Three hits for the last: the declaration and the two uses quoted above. Searched `cycles_5.0/*.py` (the add-on's own Python) for *missing*, *not found*, *magenta*: the only relevant-looking hit is a message about a missing animation frame in `operators.py` line 86, `"Frame '%s' not found, animation must be complete"`, which belongs to the "bake"/animation-denoising operator and not to image textures, so I ruled it out.
- **Pictures.** I looked for a manual picture that might show a missing texture and found none to open. The pictures named on the Image Texture node page are `node-types_ShaderNodeTexImage.webp` and four projection examples; none is about a missing file, so there was nothing to read from them on this question.

That is why I record the manual, the API documentation and the release notes as silent, and rest the answer on Cycles' files.

---

## Hand-test results, second batch — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md` under the heading *Added from the citation check — a second batch*; the entry numbers below are that list's numbers.

These three are a second batch. The topic's first fourteen hand tests were run on the same day and are written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`; they are not repeated here. The citation check found three of those fourteen recorded too thinly for anybody to repeat and wrote out what each should have been. Those three are entries 15, 16 and 17.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before that run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, so pictures were compared as pixels and never as files. "Missing" below always means the file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 5.0.1, from `/opt/blender/5.0/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.0.1.

### 15. A missing picture feeding a normal map, a bump height and a displacement height, rebuilt so it can be repeated (entry 15)

> **Why there are two of these.** The citation check found that hand-test result 2 — which is written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`, not here — cannot be repeated from its own words and wrote out a test with every measurement in it. That test is entry 15 in the hand-test list, and this is what it gave. **That earlier result is untouched and its figures stand.** This is a second run over the same ground, not a correction of it, and the two set-ups are different so the figures are not meant to match.

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

Then the scene, with every number recorded: a Volume data-block pointing at `//vol.vdb`, on an object at location (0, 0, 0), rotation 0, scale (1, 1, 1); a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; the frame 64 by 64 with the film **not** transparent; a world data-block with its nodes switched off and its colour black; Cycles at 128 samples with denoising off. With the file loaded, the object's dimensions read (3.75, 3.75, 5.0). In 5.0 setting the world's nodes off did not take — the property read back as True straight after it was set — so the empty frame carries the world's own default grey rather than black.

Three renders: file present; the Volume object deleted altogether; the file moved out of the folder.

**What came back, the same on both computers**, as the mean and the largest pixel value over the frame:

```
RESULT present   mean 0.23706698417663574  max 2.003127098083496
RESULT novolume  mean 0.049999963492155075  max 0.049999967217445374
RESULT missing   mean 0.049999963492155075  max 0.049999967217445374
```

**So:** the file-present render against the object-deleted render is the control, and it holds — the volume plainly changes the frame when its file is there. With the file gone, the frame matched the render with no Volume object at all, to every digit printed, on both computers. Searching the whole of every run's output for `ERROR`, `WARNING`, `Error` and `Traceback` matched no line, in either state, on either computer. A missing volume file draws nothing and says nothing.

### What this second batch did not settle

- **Windows.** No Windows machine has run it. The three results above settle macOS on Apple silicon and Linux on an NVIDIA card, and leave Windows untested rather than answered.

---

## Things a person with Blender 5.0 open could settle in a few minutes

1. **What a missing map actually renders as, for each kind of input.** Make a new file. Add a Principled BSDF. Add an Image Texture node, open any image file, save the blend-file, quit Blender, rename or move that image file on disk, reopen the blend-file. Then render (F12) with Cycles, once for each of these wirings, and look at the result: (a) Color → Base Color; (b) Color → Roughness; (c) Color → a Normal Map node → Normal; (d) Color → a Bump node's Height → Normal; (e) Color → a Displacement node's Height → the Material Output's Displacement; (f) Alpha → the Principled BSDF's Alpha. Note in each case whether the surface is magenta, uniformly wrong, or unchanged. **Wirings (c), (d) and (e) are answered** — see result 15 in the second batch of hand tests below, run 2026-08-09 on both computers. Wirings (a), (b) and (f) are not covered there.
2. **The exact number a missing map gives a plain-number input.** With the same broken file, wire the Image Texture's Color into an Emission shader's Strength, render, and read the pixel value in the Image editor. That gives the single number the missing colour turns into.
3. **Whether the Color Space setting changes it.** Repeat test 1(a) with the image's Color Space set to sRGB, then to Non-Color, and compare the rendered pixel values. The files predict no difference.
4. **Whether EEVEE does the same.** Repeat tests 1 and 2 with the render engine set to EEVEE and compare. Nothing in the sources covers this.
5. **Whether Open Shading Language changes it.** Repeat test 1 with Cycles set to CPU and Open Shading Language switched on in the Render properties, and compare.
6. **Whether the render warns.** Start Blender from a terminal so its console output is visible, render the broken file, and read what, if anything, is printed. Then repeat with the blend-file rendered head-less (`blender -b broken.blend -f 1`) and compare the output, and check the process's exit code.
7. **Whether the saved file matches the screen.** In the same head-less run, write the frame out and open the written file; check that the surface looks the same as the render shown on screen.
8. **A missing tile of a tiled image.** Set up an image with UDIM-style tiles, delete one tile file, and render a surface whose coordinates fall in the deleted tile. Check whether that region is magenta while the intact tiles are normal.
9. **A missing frame of an image sequence.** Set an Image Texture's Source to Image Sequence, delete one frame from disk, and render that frame and its neighbours. Check whether only the missing frame is substituted.
10. **A missing world environment.** Point an Environment Texture at a file, break the path, render, and check whether the background and the lighting both turn magenta.

---

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 5.0

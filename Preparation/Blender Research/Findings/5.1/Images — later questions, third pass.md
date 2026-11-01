# Images — later questions, third pass (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual, Blender 5.1 Python API reference, Blender 5.1 release notes, and the Cycles files supplied with this version — all as the copies held in `Blender Sources/`, taken on 2026-07-30. That snapshot date is my date read for everything below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One disclosure, so the line above is exact: before writing this file I ran a plain directory listing of the folder it goes in, to check the folder existed. That returned file names only. I did not open any file in it, and no other part of the project was read, searched, or listed.

Only question 9 was assigned to me. The rest of the sheet is untouched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

### How I read the question

Two things in it are open to more than one reading, so I say plainly which I answered.

*"An image a file points at is missing."* I read this as: the blend-file refers to an image file on disk, and at render time that file is not there. The other reading — a single frame of an image sequence, or a single tile of a tiled texture set, being absent while the rest are present — I touch on only where the files I read happen to settle it.

*"Feeding something other than colour."* I read this as: the picture's output is wired into a shader input that is not a colour input — roughness, metallic, a bump height, a normal map, a displacement, and so on. There is a second reading available, because Blender also has a setting on every image called Color Space with a "Non-Color" choice, and "feeding something other than colour" could mean that instead. As it happens the same finding answers both, and I say why below.

### LABEL

**Observed** for the substituted value itself and for the point in the pipeline where it is substituted — these come from reading Cycles' own supplied files.

**Inferred** for everything downstream of the substitution — what a non-colour input then does with that value. The steps are written out.

**Not in the sources** for: any statement of this behaviour in the manual, the Python API reference or the release notes; for EEVEE; for whether anything is written to a log or reported when it happens; and for the Open Shading Language path.

### The finding

**Cycles has one fixed colour it uses when an image is not found, and it is bright magenta.** From Cycles' own files, `source/util/types_image.h`, lines 12–13, quoted exactly:

```
/* Color to use when images are not found. */
#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
```

That is red 1, green 0, blue 1, alpha 1.

**The substitution happens at the picture lookup, not at whatever the picture is feeding.** From `source/kernel/svm/image.h`, lines 21–37, quoted exactly:

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

Three things follow from those exact lines, and they matter for this question:

1. The magenta is returned and the function ends. The colour-space handling (`NODE_IMAGE_COMPRESS_AS_SRGB`) and the alpha handling (`NODE_IMAGE_ALPHA_UNASSOCIATE`) sit *after* the early return, so neither is applied to it. **This is the answer to the second reading of the question**: setting a picture's Color Space to Non-Color, which is what one does for a picture feeding roughness or a normal map, changes nothing about the substituted value. It is (1, 0, 1, 1) either way.
2. The alpha that comes out alongside is 1, not 0.
3. The same function is used by the ordinary picture lookup, by the box-projected lookup and by the environment lookup. In the same file, `svm_node_tex_image` (declared line 49, its call at line 119), `svm_node_tex_image_box` (declared line 130, its calls at lines 215, 219 and 223) and `svm_node_tex_environment` (declared line 234, its call at line 260) all obtain their result from `svm_image_texture`, and all of them then write the red, green and blue onto the Color output and the fourth number onto the Alpha output. So the substitution is not particular to one kind of projection or one node.

**What the picture is feeding makes no difference to the substitution.** This is the direct answer to the question as asked. Nothing in the quoted code inspects what the result is wired into. The picture lookup produces magenta, and magenta is then handed to whatever is downstream exactly as a real pixel would be.

**What the render then produces therefore depends on the downstream node, and there is no magenta in the picture.** This part is **inferred**. Below are the three commonest non-colour destinations, each with the step written out, from Cycles' own files.

*Into a plain numeric input (roughness, metallic, strength, a bump height, and so on).* Cycles has a conversion step for turning a colour into a single number. From `source/kernel/svm/convert.h`, lines 29–34:

```
    case NODE_CONVERT_CF: {
      const float3 f = stack_load_float3(stack, from);
      const float g = linear_rgb_to_gray(kg, f);
      stack_store_float(stack, to, g);
      break;
    }
```

and `linear_rgb_to_gray` is, from `source/kernel/util/colorspace.h`, lines 34–37:

```
ccl_device float linear_rgb_to_gray(KernelGlobals kg, const float3 c)
{
  return dot(c, make_float3(kernel_data.film.rgb_to_y));
}
```

The step I am taking: *if* that conversion is what stands between a colour output and a numeric input — and the files I was given contain the conversion but not the code that decides to insert it — then the number arriving at the numeric input is a weighted sum of 1, 0 and 1 using three weights called `rgb_to_y`. Those three weights are not fixed in the files I read; they are computed elsewhere and handed to the kernel, so I cannot state the resulting number. What I can say from the arithmetic is that it is a sum of the red and blue weights only, the green contributing nothing. It is neither zero nor one. So a roughness fed by a missing picture does not go black or white; it lands somewhere in between and looks like a deliberate value.

*Into a Normal Map node.* From `source/kernel/svm/tex_coord.h`, lines 339–340:

```
  float3 color = stack_load_float3(stack, color_offset);
  color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);
```

The step: putting (1, 0, 1) through that line gives (1, −1, 1). A picture that was doing nothing would have to be (0.5, 0.5, 1), giving (0, 0, 1). So the decoded direction is not the flat one — it is tilted hard along two axes at once. Further down the same function that vector is combined with the surface's tangent, bitangent and normal and normalised (line 409, `N = safe_normalize(to_global(color, tangent, B, normal));`). The render therefore produces a surface lit as though it were uniformly slanted, not a magenta surface and not an unchanged one.

*Into a Displacement node.* From `source/kernel/svm/displace.h`, the Displacement node reads a single height, subtracts a midlevel and multiplies by a scale (lines 162 and 167, `dP *= (height - midlevel) * scale;`). The step: the height arriving is the converted number from the first case above, which is a fixed value everywhere on the surface. A displacement that is the same everywhere pushes the whole surface out or in by a constant amount rather than shaping it, so what the render produces is a uniformly inflated or deflated object, again with no magenta anywhere.

**In one line:** where the picture feeds colour you see bright magenta and know at once; where it feeds anything else you see no magenta at all, only a surface that is wrong in a way that looks intentional.

**Two more things the Cycles files settle.**

A tiled picture set behaves the same way through the same route, but it can reach it for a second reason. In `source/kernel/svm/image.h`, `svm_node_tex_image` starts with `int id = -1;` (line 78) and only replaces it if the coordinate lands inside a tile that is present in the list; if no tile matches, `id` is still −1 when it reaches `svm_image_texture`. The step I am taking is only that the value stays −1 through that stretch of code, which the lines show. So a missing tile, and a coordinate outside every tile, both give the same magenta.

There is a second place the same constant appears, and it is *not* the missing-file case, so I flag it rather than fold it in. `source/kernel/util/image_3d.h` line 258 returns `IMAGE_MISSING_RGBA` at the end of the three-dimensional lookup used for volume grids, reached when the data type is not one of the recognised ones or when the build has no NanoVDB support. That is a build and data-type fallback, not the same thing as a file being absent, and I did not find the missing-file branch for volumes in the files I was given.

### One link in the chain I could not close

`svm_image_texture` substitutes magenta when the identifier it is handed is −1. The code that decides an image file could not be loaded and hands out −1 in its place is Cycles' image manager, and that file is not among the ones I was given — the supplied Cycles material is the render kernel, some shared utilities, the compiled shader files and the add-on's Python. So the connection between "the file on disk is missing" and "the identifier is −1" rests on the comment on the constant itself — `Color to use when images are not found` — and on the constant's name, `IMAGE_MISSING_RGBA`. That is strong, and it is written in the file, but it is one step short of watching it happen. I have labelled the substitution observed and this link inferred, and this is the step.

### Not covered

- **Nothing in the manual, the API reference or the release notes says any of this.** The magenta substitution for missing pictures in a render is, in the sources I was given, visible only in Cycles' own files. A user reading the documentation would not learn it.
- **EEVEE is not settled at all.** EEVEE's code was not among my sources, and the manual does not say. Whether EEVEE substitutes the same colour, a different one, or nothing, I cannot say.
- **Whether anything is reported.** I found no statement anywhere that a render logs, warns or errors when a picture file is absent. The render does not appear to stop — the kernel simply has a value to use — but that "does not stop" is inferred from the fact that the code has a defined result and no error path, not stated anywhere.
- **The Open Shading Language path is not settled.** With OSL, picture lookups go through a different route. In the supplied compiled shader `shader/node_image_texture.oso` I found no missing-colour option being set (searched the file for "missing": no matches). On the graphics-card OSL path, `source/kernel/osl/services_gpu.h` lines 1025–1051 route a picture lookup to the same `kernel_image_interp` for one kind of handle and `return false;` for anything unrecognised, with no use of the magenta constant. What a `false` return produces in the finished picture is decided by the OSL library, which is not among my sources. So for OSL I cannot say the answer is the same.
- **The exact number** a numeric input receives, because the three weights it depends on are not in the files I read.
- **A missing frame within an image sequence, and a missing picture used somewhere other than a material** — the compositor, the video sequencer, a texture read inside geometry nodes — are outside what I checked, except that the video sequencer case is documented separately and I quote it below only because it is the one place the documentation does speak.

### The one place the documentation does speak, and why it is not the answer

The manual documents a magenta substitution, but for the video sequencer, not for a render's materials. From the manual, `https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html`, read 2026-07-30:

> Show Missing Media

> Render missing images/movies with a solid magenta color.

> When disabled, missing content will render fully transparent.

> Strips with missing content will be displayed as red in the timeline.

The same setting appears in the API reference, `https://docs.blender.org/api/5.1/bpy.types.SequenceEditor.html`, read 2026-07-30: "Render missing images/movies with a solid magenta color (default False)".

I include this because it is the nearest documented thing and somebody checking my work will find it. It is a different part of Blender, it is a switch that can be turned off, and it says nothing about pictures feeding a material. The Cycles substitution I quoted above has no switch anywhere in the files I read.

### The only documented way of learning a file is missing

From the manual, `https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html`, read 2026-07-30:

> Report Missing Files

> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files.

The matching Python entry, from `https://docs.blender.org/api/5.1/bpy.ops.file.html`, read 2026-07-30: "bpy.ops.file.report_missing_files()" — "Report all missing external files".

Also relevant, from `https://docs.blender.org/api/5.1/bpy.types.Image.html`, read 2026-07-30, on the property `has_data`: "True if the image data is loaded into memory (default False, readonly)".

Neither of these is part of rendering. They are checks a person or a script runs separately.

### Outside the interface

The substitution itself is in the render kernel — the code that runs per shading point during a render — and nothing in the quoted lines consults any window, editor or user setting. So it holds identically whether Blender is driven by a person clicking or started and controlled by something else. That is **inferred**, from the fact that the branch depends only on the picture identifier; the sources do not state it.

Where it does differ: the *finding out* differs. The one documented report, quoted above, puts "a warning message ... in the Info editor's header" — an editor that a run with no window does not display. The operator `bpy.ops.file.report_missing_files()` can be called from Python, so the check itself is reachable from outside the interface, but where its warning goes when there is no Info editor is **not in the sources**. And `has_data` on an image is a plain property readable from Python at any time. So: the render behaves the same, but the only documented way of being told is tied to an editor.

**Settled further by hand test, 2026-08-09 — observed.** The three tests in the *Hand-test results, second batch* section below were all run with Blender driven from a terminal, with no window at all, on both computers — including the rented Linux machine, which has no display server of any kind. Every one of them finished, wrote its frame and gave the numbers recorded there. Workbench, which is the viewport's own renderer, rendered there too. So what a render puts on the surface for a missing file does not depend on a window being open.

### Operating system

The magenta constant is a single definition in a shared header with no operating-system condition around it. I read `source/util/types_image.h` in full for this and there is no platform test near lines 12–13; the same is true of the branch in `source/kernel/svm/image.h`. So it holds on every operating system. **Observed**, from those files.

One caution that is about the build rather than the operating system: the three-dimensional lookup in `source/kernel/util/image_3d.h` reaches its `IMAGE_MISSING_RGBA` return through an `#ifdef WITH_NANOVDB` — so a build made without that support takes that path always. That is a build option, and the sources do not say which builds on which operating systems set it.

**Settled further by hand test, 2026-08-09 — observed.** All three tests in the *Hand-test results, second batch* section below were run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every one. Where any figure differed it differed in its last digits only, and each such difference is given where it occurs. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked, and what I ruled out

Searched across the whole of the 5.1 manual, the whole of the 5.1 API reference and the whole of the 5.1 release notes, case-insensitively, for: `magenta`, `pink`, `purple`, `missing`, `missing file`, `missing image`, `missing texture`, `find missing`, `report missing`, `missing media`, `broken path`, `broken link`, `file not found`, `cannot be found`, `could not be found`, `not found`, `does not exist`, `doesn't exist`, `no longer exists`, `moved or deleted`, `unavailable`, `invalid path`, `placeholder`, `fallback`, `substitute`, `unresolved`, `texture is missing`, `cannot load`, `unable to load`, `load fail`.

- `magenta` across all three sources returns nine lines. Only the video sequencer one is about a missing picture; the rest are colour management, anaglyph glasses, a loop-cut highlight and a colour balance control. Quoted the relevant one above; ruled the others out as unrelated.
- `missing` across the whole manual returns 41 lines, and I read every one. They are: the topbar's Report/Find Missing Files (quoted above), the video sequencer setting (quoted above), missing caustics and missing highlights in the Cycles noise page, missing shadow updates in EEVEE, missing node support in EEVEE, missing NLA, an add-on's missing features table, missing data layers in two modifiers, missing linked data in the linked-libraries pages, missing frames in a video-sequencer image strip, missing directories being created by the File Output node, and a handful in contributor and history pages. None of them concerns a picture feeding a material during a render. Ruled out.
- `pink` and `purple` restricted to the manual's `render/`, `files/`, `editors/image/` and `troubleshooting/` folders return nothing at all.
- Read in full the manual page for the Image Texture node, `https://docs.blender.org/manual/en/5.1/render/shader_nodes/textures/image.html`. It describes every input, property and output including Color Space, Alpha, Extension and Source, and says nothing about the file being absent. Ruled out — this was the likeliest page and it is silent.
- Checked the manual's image settings and image editor introduction pages, and the image formats page, for `missing`, `exist`, `not found`, `error`, `fail`. Nothing.
- Checked the EEVEE node-support page's Image Texture entry, `https://docs.blender.org/manual/en/5.1/render/eevee/limitations/nodes_support.html`. It lists interpolation and projection limitations only. Nothing about a missing file. Ruled out.
- Checked the command-line arguments page for `missing` and `texture`: only a graphics-card debugging switch. Ruled out.
- Searched the 5.1 release notes for `missing`, `magenta`, `pink`. The hits are bug-fix list entries about unrelated things — an eye-dropper tool, hair node groups, an export error, missing API descriptions, a multiresolution file path. Nothing about picture substitution in a render, and nothing saying this behaviour changed. Ruled out.
- Searched all supplied Cycles files for `pink`, `magenta`, `missing`, and for the literal magenta value written several ways. That is what surfaced `IMAGE_MISSING_RGBA`, and I then traced all three places it appears (`source/util/types_image.h` line 13, `source/kernel/svm/image.h` line 25, `source/kernel/util/image_3d.h` line 258) and read the surrounding function in each.
- Read in full: `source/kernel/svm/image.h`, `source/kernel/svm/convert.h`, `source/kernel/svm/displace.h`, `source/util/types_image.h`; and the relevant stretches of `source/kernel/svm/tex_coord.h` (the Normal Map node), `source/kernel/util/colorspace.h`, `source/kernel/util/image_3d.h`, `source/kernel/osl/services_gpu.h`.
- Looked for the image manager (the code that would load a file and assign or refuse an identifier) among the supplied Cycles files. It is not there — the supplied set is `source/kernel`, `source/util`, the compiled `shader` files, and the add-on's Python. That absence is what leaves the one open link described above.
- Searched the compiled shader `shader/node_image_texture.oso` for `missing`: no matches.
- I did not need the web and did not go there.

---

## Hand-test results, second batch — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md` under the heading *Added from the citation check — a second batch*; the entry numbers below are that list's numbers.

These three are a second batch. The topic's first fourteen hand tests were run on the same day and are written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`; they are not repeated here. The citation check found three of those fourteen recorded too thinly for anybody to repeat and wrote out what each should have been. Those three are entries 15, 16 and 17.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before that run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, so pictures were compared as pixels and never as files. "Missing" below always means the file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 5.1.2, from `/opt/blender/5.1/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.1.2.

### 15. A missing picture feeding a normal map, a bump height and a displacement height, rebuilt so it can be repeated (entry 15)

> **Why there are two of these.** The citation check found that hand-test result 2 — which is written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`, not here — cannot be repeated from its own words and wrote out a test with every measurement in it. That test is entry 15 in the hand-test list, and this is what it gave. **That earlier result is untouched and its figures stand.** This is a second run over the same ground, not a correction of it, and the two set-ups are different so the figures are not meant to match.

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

Then the scene, with every number recorded: a Volume data-block pointing at `//vol.vdb`, on an object at location (0, 0, 0), rotation 0, scale (1, 1, 1); a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; the frame 64 by 64 with the film **not** transparent; a world data-block with its nodes switched off and its colour black; Cycles at 128 samples with denoising off. With the file loaded, the object's dimensions read (3.75, 3.75, 5.0). In 5.1 setting the world's nodes off did not take — the property read back as True straight after it was set — so the empty frame carries the world's own default grey rather than black.

Three renders: file present; the Volume object deleted altogether; the file moved out of the folder.

**What came back, the same on both computers**, as the mean and the largest pixel value over the frame:

```
RESULT present   mean 0.23868240416049957  max 2.003127098083496
RESULT novolume  mean 0.049999963492155075  max 0.049999967217445374
RESULT missing   mean 0.049999963492155075  max 0.049999967217445374
```

**So:** the file-present render against the object-deleted render is the control, and it holds — the volume plainly changes the frame when its file is there. With the file gone, the frame matched the render with no Volume object at all, to every digit printed, on both computers. Searching the whole of every run's output for `ERROR`, `WARNING`, `Error` and `Traceback` matched no line, in either state, on either computer. A missing volume file draws nothing and says nothing.

### What this second batch did not settle

- **Windows.** No Windows machine has run it. The three results above settle macOS on Apple silicon and Linux on an NVIDIA card, and leave Windows untested rather than answered.

---

## Things a person with Blender 5.1 open could settle in a few minutes

Each of these is a gap above. They are written so somebody who has not read my reasoning can carry them out.

1. **Confirm the magenta substitution, and confirm it is invisible on a non-colour input.** New file. Give the default cube a Principled BSDF. Add an Image Texture node, open any picture from disk, and plug its Color into Base Color. Save the blend-file. Quit Blender. Rename the picture file on disk. Reopen the blend-file and render with Cycles. Look at the cube. Then, in the same file, unplug the texture from Base Color and plug it into Roughness instead, set Base Color to a plain mid grey, and render again. Report both: what colour the cube is in the first render, and whether the second render shows any magenta at all or merely a cube with an unexpected shininess.

2. **Read off the actual number a numeric input receives.** Two renders, both with the same broken texture and colour management left at its default.
   - First: put an Emission shader on the cube, strength 1, and plug the Image Texture's Color into the Emission's Color. Render. Sample the cube in the Image Editor with the colour picker and report the three numbers.
   - Second: set the Emission's Color to pure white and plug the Image Texture's Color into the Emission's *Strength* instead — a colour going into a numeric input, which is the case in question. Render, sample the cube, and report the brightness.
   The second number is the one this document could not state.

3. ~~**Settle the normal map case.**~~ **Answered** — see result 15 in the second batch of hand tests below, run 2026-08-09 in all four versions on both computers. A Normal Map node fed 0.5, 0.5, 1 renders identically to no Normal Map node at all; a missing picture on the same input does not.

4. **Settle EEVEE.** Repeat item 1 exactly, with the render engine set to EEVEE instead of Cycles. Report the colour of the cube in the colour case, and whether the roughness case differs from Cycles.

5. **Settle whether anything is reported.** Take the file from item 1 with the picture renamed, and render it from a command line with no window: `blender --background <file>.blend --render-frame 1`. Copy out everything the command prints, in full. Report whether the missing picture is mentioned anywhere in it, and whether the command's exit status is zero.

6. **Settle the Open Shading Language path.** Same file as item 1, Cycles, device set to CPU, and Open Shading Language switched on in the Render properties. Render. Report the colour of the cube, and say whether it matches the non-OSL result.

7. ~~**Settle the volume case.**~~ **Answered** — see result 17 in the second batch of hand tests below, run 2026-08-09 in all four versions on both computers. With the OpenVDB file gone the frame matched the render with no Volume object at all, to every digit, and nothing was printed about it.

8. **Settle the missing-tile case.** Use a tiled (UDIM) texture set with at least two tiles, delete one tile file from disk, and render an object whose coordinates cross both tiles. Report whether the missing tile's area is magenta while the present tile renders normally.

9. **Settle what a headless run does with the missing-file report.** From a command line: `blender --background <file>.blend --python-expr "import bpy; bpy.ops.file.report_missing_files()"`. Report everything printed.

---

9 August 2026 — Opus 5 (Claude Code) — Images, Blender 5.1

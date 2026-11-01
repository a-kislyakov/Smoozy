# The compositor — Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including three images from its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All were read on **2026-07-30** (the snapshot date given to me), which is the date of reading recorded against every finding below.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

I worked alone. I started no helper agents, so there is nobody else's reading for me to vouch for.

---

## 1. What decides whether a file's compositor runs at all?

**Answer.** Three separate conditions appear in the sources. Two are settings a person can flip; the third is a property of the node tree itself. The sources place Use Nodes and the node tree on `Scene`, and question 7's interface file reads Compositing from `context.scene.render`. The conclusion that all three conditions are per scene is **inferred** from those locations.

**(a) The scene's Use Nodes switch.** Ticking it activates nodes for compositing and enables the Compositor; the property is described as enabling the compositing node tree and starts off. *(Citation check, 2026-08-03: this line previously read "Until this is on, the scene has no compositing node tree in play." The quoted words do not carry that — they say what ticking the switch does and what the property's default is, not what is or is not in play before then. Narrowed to what the words carry.)*

> "Access the Compositor and activate nodes for compositing by clicking the Use Nodes checkbox in the header
> (see Introduction)."
>
> "After clicking Use Nodes the Compositor is enabled, however,
> it can also be disabled in the Post Processing."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/introduction.html
- Date read: 2026-07-30

The same switch in the Python API:

> "use_nodes
>
> Enable the compositing node tree
>
> Type:
>
> boolean, default False"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html
- Date read: 2026-07-30

Note the default is **off**. A brand-new scene does not have it enabled.

**(b) The Compositing checkbox in Output ‣ Post Processing.**

> "Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Composite Output node."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html
- Date read: 2026-07-30

> "use_compositing
>
> Process the render result through the compositing pipeline, if compositing nodes are enabled
>
> Type:
>
> boolean, default True"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

Default is **on**. The wording "if compositing nodes are enabled" makes this setting conditional on compositing nodes being enabled; the source does not name the Use Nodes property in that sentence.

I also looked at the picture of that panel. It shows a panel headed "Post Processing" containing exactly two checkboxes, "Compositing" and "Sequencer", both ticked, and a slider "Dither" reading 1.000.

- Source: manual, image `_images/render_output_properties_post-processing_panel.png`, shown on https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html
- Label for what I take from the picture: **documented**
- Date read: 2026-07-30

**(c) Whether the node tree contains an active output node.**

> "The compositor searches for the active Composite node, if none was found, it searches for the active Viewer node.
> If none was found, the compositor doesn't run altogether."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html
- Date read: 2026-07-30

**A separate switch exists for the viewport compositor**, and it is not any of the above:

> "use_compositor
>
> When to preview the compositor output inside the viewport
>
> DISABLED
> Disabled – The compositor is disabled.
>
> CAMERA
> Camera – The compositor is enabled only in camera view.
>
> ALWAYS
> Always – The compositor is always enabled regardless of the view.
>
> Type:
>
> enum in ['DISABLED', 'CAMERA', 'ALWAYS'], default 'DISABLED'"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.View3DShading.html
- Date read: 2026-07-30

**LABEL: documented** for the three conditions quoted beside them; **inferred** for the per-scene summary, with the step above.

**Not covered.** I cannot show from the sources that this list is complete — no page says "the compositor runs if and only if…". The sentence in (c) sits on a page section about how the compositor picks its output target, and that section discusses viewport compositing alongside final-render compositing; I cannot tell from its wording alone whether "doesn't run altogether" is meant for both. Nothing I found says what the render result contains when the compositor does not run (whether the raw render is written instead) — see question 2. Nothing says what happens when Use Nodes is on but the tree is empty of nodes entirely, beyond (c).

**Outside the interface.** The sources do not say. All three switches are stored in the blend-file (two as scene properties, one as node-tree contents), and the command-line render options listed for 4.5 contain no switch for compositing — the Render Options are `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads` (source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). That the file's stored settings therefore govern a background render is **inferred**, not stated: the step is "the settings live in the file, and no command-line option overrides them, so the file's settings are what applies".

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run twice: once on macOS, and once on a second computer — the Linux results are in their own section further down, and the two runs agreed on every figure. The sources still do not say this differs by operating system. Windows was not tried, because no Windows machine has run it. The only operating-system-specific statement I found anywhere in the compositing chapter concerns the GPU compositor: "TDR on Windows … resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit" (manual — https://docs.blender.org/manual/en/4.5/compositing/limitations.html, read 2026-07-30). That is about the GPU compositor failing, not about whether the compositor runs — and it names the one operating system neither machine runs.

### Observed — hand tests, 2026-08-03

All the tests below were run on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, using Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, started with no window (`-b`) and with the Creator's own add-ons and preferences kept out of it (`--factory-startup`). The full set-up for each is written out in `Audits/Blender Research/The compositor/Hand tests.md`.

**Hand test 1 — what the switches are called, and what they start as.** Blender was started on its own empty startup file and asked to print the value of each named property, or `MISSING` where the property does not exist. What came back, word for word:

```
BLENDER_VERSION 4.5.12 LTS
Scene.use_nodes = False
Scene.node_tree = None
Scene.compositing_node_group = MISSING
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = MISSING
RenderSettings.compositor_device = 'CPU'
RenderSettings.compositor_precision = 'AUTO'
```

So in 4.5 the compositing tree hangs off the scene as `node_tree`, starting empty, with `use_nodes` starting off — which matches (a) and (b) above.

**Hand test 3 — whether Use Nodes still decides anything.** A 64×64 scene rendering a flat green slab was given a compositor of Render Layers into an Invert node into the Composite node, with the Compositing switch on. `scene.use_nodes` was set to each value in turn, read back, and the frame rendered and its first pixel read. What came back, word for word:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back False -> written pixel [0.4275, 0.7686, 0.3137, 1.0]
```

Green is `[0.4275, 0.7686, 0.3137, 1.0]` and its inverse is `[0.7882, 0.4588, 0.7882, 1.0]`. So in 4.5 the switch keeps the value it is given, and turning it off leaves the raw render in the file. The two `in RNA` lines record that `bl_use_postprocess` and `bl_use_image_save` are both present on the render-engine type in 4.5.

**Hand test 4 — what makes the compositor run in a render started with no window.** A 64×64 Workbench scene with a cube and a camera, whose raw render is black, was rendered five ways: with a compositor of Render Layers into Invert into Composite, once with the Compositing switch on and once off; with the Composite node deleted; with a plain red colour fed to the Composite node and no Render Layers node at all; with the Composite node muted; and with no compositor tree at all. What came back, word for word:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Black is `[0.0, 0.0, 0.0, 1.0]`, inverted black is `[0.7686, 0.7686, 0.7686, 1.0]`, and the red fed to the output is `[0.8549, 0.2196, 0.1255, 1.0]`.

**Hand test 5 — the compositor device.** The same 64×64 flat-green scene with an Invert compositor was rendered with the Compositor Device set each way:

```
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
```

**Hand test 5 — the Enable Output node.** Blender was asked whether the node type exists:

```
RESULT Enable Output node -> bpy.types.NodeEnableOutput MISSING in this version
```

**Hand test 14 — the compositor above the size 5.2's documentation names.** A 9000×400 flat-green scene with an Invert compositor was rendered with each device:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
```

**Outside the interface, from the tests.** Every one of the cases above was a render started from a terminal with no window open, so the three conditions and the device setting all take effect in that case. The step marked inferred above — that the file's stored settings are what applies to a background render, since no command-line option overrides them — is now observed for the Compositing switch, the Use Nodes switch and the node tree's contents.

### Observed — window tests, 2026-08-03

These three are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 4.5's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are written out in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15, 17 and 21. What follows is what the Creator reported, in their own words.

**Hand test 15 — what the Post Processing panel holds.** The Properties editor's Output tab was opened, and the panel called Post Processing opened with it. The Creator's report:

> Two checkboxes — Compositing and Sequencer, both on by default. One slider called "Dither" set to 1.00 by default, a float, adjustable from 0 to 2. Compositing tooltip: "Process the render result through the compositing pipeline, if compositing nodes are enabled." Sequencer tooltip: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

So the panel really does hold what the manual's screenshot shows, and the Compositing tooltip is the same sentence as the API description of `use_compositing` quoted above, word for word. The test also asked whether the panel's own header carries a checkbox beside the words "Post Processing". The Creator did not say, so that part is not answered.

**Hand test 17 — what the Compositor editor's header offers.** The Compositing tab along the top of the window was clicked, in a file where nothing had been set up. The Creator's report:

> There is a "Use Nodes" checkbox, off by default. The View, Select, Add and Node menus are present but everything in Add is greyed out until it's ticked. Ticking it creates the standard setup: Render Layers connected through a reroute to a Viewer node and to a Composite node. Right under the workspace menu bar it reads "Scene > Compositing Node Tree".

The switch named under (a) above is in that header, and it starts off, which is the default the API entry quoted above gives.

**Hand test 21 — how a second output node is marked.** In the Compositing editor a Render Layers node was wired to a Composite node, a second Composite node was added, and the Render Layers node wired to that one too. The Creator's report:

> The active node has a dark red header, the inactive one grey. Clicking a node makes it active. Clicking empty space or another node leaves the last-clicked one active.

That is the same marking the manual sentence quoted under question 2 describes — "The active Composite node is highlighted with a red header and is the last selected among them." The test also asked whether either node's own header carries a control for making it the live one. The Creator did not name one, so that part is not answered.

**Outside the interface.** These three settle nothing about it. Each is a look at what a Blender window shows, so none of them speaks to a render driven from outside.

**Operating system.** All three were done on macOS. All three were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Observed — the same tests on a rented Linux machine, 2026-08-06

Every test above was run again, unchanged, on a second computer: a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own. The Blender used was 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`, started with no window (`-b`) and with `--factory-startup`, except where the test says a window was opened. The scripts were the same files as on the Mac, copied across. The full set-up is in `Audits/Blender Research/The compositor/Hand tests — Linux.md`. **These are second results, not replacements for the ones above.**

One thing to know before reading the figures. Two of the reference colours come back one step of 255 lower in one channel on this machine than on the Mac: magenta reads `[0.7882, 0.4549, 0.7882, 1.0]` here against `[0.7882, 0.4588, 0.7882, 1.0]` there, and yellow reads `[0.7804, 0.7529, 0.4196, 1.0]` here against `[0.7804, 0.7529, 0.4235, 1.0]` there. Every other colour is identical to the last figure. Which colour arrived is the same on both machines in every test.

**Hand test 1 — what the switches are called, and what they start as.** What came back, word for word:

```
BLENDER_VERSION 4.5.12 LTS
Scene.use_nodes = False
Scene.node_tree = None
Scene.compositing_node_group = MISSING
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = MISSING
RenderSettings.compositor_device = 'CPU'
RenderSettings.compositor_precision = 'AUTO'
```

Identical to the Mac's, line for line.

**Hand test 3 — whether Use Nodes still decides anything.** What came back, word for word:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4549, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back False -> written pixel [0.4275, 0.7686, 0.3137, 1.0]
```

The same as the Mac: the switch keeps the value it is given, and turning it off leaves the raw green render in the file.

**Hand test 4 — what makes the compositor run in a render started with no window.** What came back, word for word:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Identical to the Mac's, figure for figure.

**Hand test 5 — the compositor device, and the Enable Output node.** What came back, word for word:

```
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0]
RESULT Enable Output node -> bpy.types.NodeEnableOutput MISSING in this version
```

**Hand test 14 — the compositor above the size 5.2's documentation names.** What came back, word for word:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
```

The same as the Mac: 4.5 writes the composited picture at 9000 pixels wide on either device.

**Hand test 15 on the Linux list — whether a compositor tree gives the same picture on the card as on the processor.** This test has no Mac counterpart. A 320×240 Workbench scene holding a sphere, a cube and a cone under the studio light was given four compositor trees in turn — an Invert; a Blur of size 12; a Scale set to absolute 160×120; and a chain of Invert into a Blur of size 6 into an Alpha Over blending the blurred copy back over the original at half strength. Each was rendered twice, once with the Compositor Device set to CPU and once to GPU, and the two written files compared value by value, all 307,200 of them. The first three trees were run by one command and the fourth by a second command; what came back is quoted below as one block, the two commands run one after the other. Word for word: *(Linux citation check, 2026-08-06: the block was previously headed "word for word" with no sign that it stitches the output of two separate commands. Said plainly.)*

```
RESULT tree=invert device=CPU -> 320x240 first pixel [0.7686, 0.7686, 0.7686, 1.0]
RESULT tree=invert device=GPU -> 320x240 first pixel [0.7686, 0.7686, 0.7686, 1.0]
COMPARE tree=invert -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
RESULT tree=blur device=CPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
RESULT tree=blur device=GPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
COMPARE tree=blur -> 307200 values compared, largest difference 0.003922, 0 values differ by more than one step of 255, identical=False
RESULT tree=scale device=CPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
RESULT tree=scale device=GPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
COMPARE tree=scale -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
ALPHAOVER inputs=['Fac', 'Image', 'Image', 'Straight Alpha'] outputs=['Image']
RESULT tree=invert+blur+alphaover device=CPU -> 320x240 first pixel [0.6627, 0.6627, 0.6627, 1.0]
RESULT tree=invert+blur+alphaover device=GPU -> 320x240 first pixel [0.6627, 0.6627, 0.6627, 1.0]
COMPARE tree=invert+blur+alphaover -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
```

Three of the four trees came back identical to the last figure on the two devices. The fourth, the blur, differed by at most `0.003922` — one step of 255 — with no value differing by more than that. This settles Linux and one NVIDIA card at 320×240; it says nothing about macOS, about Apple's own graphics, or about a card of another make.

### Observed — the same window tests on the rented Linux machine, 2026-08-06

These are interface observations, like the three above. They were driven over SSH on the rented Linux machine, which has a display of its own, and photographed with Blender's own screenshot command. The pictures are on that machine under `/root/smoozy/The compositor/work/shots/`, and what each shows is written out in words here, so nothing depends on the picture surviving.

**Hand test 15 — what the Post Processing panel holds.** The Properties editor was put on the Output tab, made to fill the window, and the Post Processing panel popped open on its own. The panel holds a row labelled **Pipeline** with two checkboxes, **Compositing** and **Sequencer**, both ticked; and below it a slider labelled **Dither** reading **1.00**. Nothing else. That is what the Creator reported on the Mac.

**And the part the Mac left unanswered is now answered.** The same editor was scrolled to the bottom and photographed. The Post Processing panel's own header reads `> Post Processing` with only the fold arrow beside it — **no checkbox**. The Stereoscopy header two rows above it does carry a checkbox in the same picture, so a checkbox in a header shows up in these pictures where one exists.

**Hand test 17 — what the Compositor editor's header offers.** A Node Editor was set to the compositor and made to fill the window, in a file where nothing had been set up. Left to right the header holds: the editor-type dropdown; the menus **View**, **Select**, **Add** and **Node**; and a checkbox labelled **Use Nodes**, unticked. On the right of the same strip: a pin, a greyed-out **Backdrop** button and three small toggles. That matches the Creator's report on the Mac.

**Hand test 21 — how a second output node is marked.** A Render Layers node was wired to two Composite nodes. The first has a **dark red header**; the second has a **grey header**. Neither carries any word or badge.

**And the part the Mac left unanswered is now answered.** At three times magnification, **neither node's own header carries any control for making it the live one**. The header holds the fold arrow and the node's name, and nothing else.

**Outside the interface.** These three settle nothing about it, exactly as their Mac counterparts do not.

**Operating system.** These three were done on Linux and settle Linux. Together with the Mac results above them, macOS and Linux now both have an answer and the two agree. Windows stays untested.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**This question is ambiguous and I flag it.** I answer the reading I believe is meant: *the scene's render output path* — Properties ‣ Output ‣ Output ‣ File Path, the setting that decides where rendered frames land on disk. The other reading would be *the Composite node* — "the node where the image result is sent"; I answer that one too, at the end, because it is cheap to do and I would rather over-cover than pick silently.

### Reading A — the render output path

**Answer: no switch for it appears anywhere in the sources.** The path itself has no on/off companion. What does exist is a per-render choice, for single frames only, of whether the rendered image is saved at all.

The panel is described as:

> "This panel provides options for setting the location of rendered frames for animations,
> and the quality of the saved images.
>
> File Path
>
> Choose the location to save rendered frames."

and its only nearby toggles are named and described as Overwrite, Placeholders, File Extensions and Cache Result — none of which disable writing:

> "Overwrite
>
> Overwrite existing files when rendering.
>
> Placeholders
>
> Create empty placeholder frames while rendering."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

I looked at the picture of that panel. Visible in it: a text field reading `/tmp/` with a folder button; checkboxes "Overwrite" (ticked), "Placeholders" (unticked), "File Extensions" (ticked), "Cache Result" (unticked); then "File Format: PNG", "Color: BW / RGB / RGBA" with RGBA selected, "Color Depth: 8 / 16" with 8 selected, and "Compression 15%". There is no checkbox on the panel header or beside the path field.

- Source: manual, image `_images/render_output_properties_output_panel.png`, shown on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Label for what I take from the picture: **documented**
- Date read: 2026-07-30

For an **animation** render, the sources state that writing happens automatically:

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Date read: 2026-07-30

For a **single frame**, whether it is written is an argument of the render operator, not a scene setting:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')
>
> Render active scene
>
> …
>
> write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.ops.render.html
- Date read: 2026-07-30

Its default is `False`, i.e. a single-frame render started this way writes nothing unless asked to.

To show the silence rather than assert it, here is where I looked. I read the whole property list of `RenderSettings` (the block that holds `filepath`) and there is no boolean anywhere in it that switches output off; the booleans it does carry are `use_bake_*`, `use_border`, `use_compositing`, `use_crop_to_border`, `use_file_extension`, `use_freestyle`, `use_high_quality_normals`, `use_lock_interface`, `use_motion_blur`, `use_multiview`, `use_overwrite`, `use_persistent_data`, `use_placeholder`, `use_render_cache`, `use_sequencer`, `use_sequencer_override_scene_strip`, `use_simplify*`, `use_single_layer`, `use_spherical_stereo`, `use_stamp*` (source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). I searched the whole manual and the whole API documentation for the phrases "null output", "no output file", "skip writing", "do not save", "don't save", "disable the output", "turn off the output", "without writing", "not written to disk" and "nothing is written" — no hit relevant to this. I searched the manual for "output path" and read every hit (command-line arguments, the animation page, the File Output node page, the path-templates page, the troubleshooting mention). I read the full list of command-line Render Options and Format Options on the command-line arguments page; there is `-o/--render-output <path>` to set the path and `-x/--use-extension` for the extension, and nothing to suppress writing.

**LABEL: not in the sources** — for a switch that turns the output path off. **documented** — for the two things quoted above (animation frames are saved automatically; a single-frame render's saving is controlled by the operator's `write_still` argument).

**Not covered.** Whether setting the File Path to an empty string, or to an unwritable location, counts as "switched off" — the sources do not describe either. Whether an animation render can be made to write nothing at all. Whether the render still happens (and the compositor still runs) when nothing is written — not stated. The File Output node's own paths are a separate matter and are described as "treated the same as the render output path" (manual — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30); I found no switch for those either.

### Reading B — the Composite node

The sources carry three narrower facts about the compositor's own end point: the Composite node defines the final output; only the active Composite node is used; and a node can be muted.

> "The Composite node defines the final output of the Compositor.
> It is the node where the image result is sent to the renderer or image output after rendering."
>
> "If multiple Composite nodes are present in the node tree, only the active one is used.
> The active Composite node is highlighted with a red header and is the last selected among them."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html
- Date read: 2026-07-30

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change."

- Source: manual — https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html
- Date read: 2026-07-30

As quoted in question 1, if no active Composite node and no active Viewer node exists, "the compositor doesn't run altogether". These facts are **documented**. The quoted words do not establish that deselecting or muting a Composite node leaves no active output node, nor what the render writes when there is none. Those parts are **not in the sources**. The record of looking is the Composite node page, the node-editing page and the Compositor System page quoted here and in question 1.

**Outside the interface.** The sources do not say for reading A beyond the command-line facts already given: `-f, --render-frame <frame>` is described as "Render frame <frame> and save it", which is a statement that a command-line single-frame render does write (source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). That is the opposite of the `write_still=False` default of the Python operator, so the two routes differ here. For reading B, nothing is said about outside-the-interface behaviour.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and again on a rented Linux machine, and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. One related fact from the sources: the command-line page lists render formats that are "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP" (same page and date), which bears on what can be written, not on whether writing can be switched off.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 1 — is there an off switch at all.** Blender was asked for the value of `save_output` on the render settings, the property that Blender 5.2's manual describes as turning automatic saving off. What came back:

```
RenderSettings.save_output = MISSING
```

The property does not exist in 4.5. That confirms the answer above is a fact about 4.5 rather than a search that stopped early: the same question was asked of all four versions and the property appears only in 5.2.

**Hand test 10 — what happens with the File Path emptied.** A two-frame 64×64 file was saved with `render.filepath` set to the empty string, then rendered as an animation from a terminal, with the working folder set to the folder holding the file. The command was `blender -b --factory-startup emptypath.blend -a`. What came back, word for word:

```
Read blend: ".../out_write/4.5/emptypath.blend"
Saved: '0001.png'
Time: 00:00.13 (Saving: 00:00.05)
Saved: '0002.png'
Time: 00:00.00 (Saving: 00:00.00)
```

Both files appeared in the folder the command was run from. So an empty File Path does not stop writing in 4.5; the frames land in the working folder under their frame numbers.

**Hand test 10 — a muted File Output node.** Two otherwise identical two-frame files were saved, one with a File Output node aimed at an empty folder and muted, one with the same node unmuted, and each rendered as an animation from a terminal. The folder listings afterwards:

```
muted_fileoutput  -> [end]
unmuted_fileoutput-> Image0001.png Image0002.png [end]
```

Muting the node stops it writing.

**Hand test 4 — what is written when the compositor has no output node.** With the Compositing switch on, every Composite and Viewer node deleted, and a Render Layers node feeding an Invert node that goes nowhere, a still was rendered from a terminal:

```
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

`[0.0, 0.0, 0.0, 1.0]` is the scene's raw render. A file was written, and it held the raw render rather than nothing. The same happened with the Composite node present but muted, and with no compositor tree at all.

**Outside the interface, from the tests.** Every case above was driven from a terminal with no window. The finding that a command-line animation render writes is now observed as well as documented, and the empty-path case above shows where it writes when the path gives it nothing to go on.

### Observed — window tests, 2026-08-03

This one is an **interface observation**, unlike the terminal tests above. The Creator ran it by hand in Blender 4.5's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-up is written out in `Audits/Blender Research/The compositor/Hand tests.md`, entry 16.

**Hand test 16 — what the Output panel holds, and whether its header has a checkbox.** The Properties editor's Output tab was opened and the panel called Output looked at, header first. The Creator answered this one with a screenshot rather than words, saved as `4.5.png` in a folder called `Test 16` on their own computer. What the picture shows, top to bottom:

- The panel header reads **Output**, with the ordinary collapse arrow to its left and the drag handle on the right. **There is no checkbox beside the word "Output".**
- A file path field reading `/tmp/`, with a folder button at its right end.
- **Saving** — "File Extensions", ticked; "Cache Result", unticked.
- **File Format** — "PNG".
- **Color** — three buttons, "BW", "RGB", "RGBA", with RGBA chosen.
- **Color Depth** — two buttons, "8" and "16", with 8 chosen.
- **Compression** — a slider reading "15%".
- **Image Sequence** — "Overwrite", ticked; "Placeholders", unticked.

Below the Output panel the picture also shows two further panels, "Color Management" and "Pixel Density". There is **no control called Media Type** anywhere in the panel.

So the running program agrees with the manual's own picture of this panel, which is quoted above: no on/off control on the header, and no Media Type row. The same look in Blender 5.0, 5.1 and 5.2 does show a Media Type row, and 5.2 alone shows a checkbox in the header.

**Outside the interface.** This settles nothing about it. It is a look at what a Blender window shows.

**Operating system.** It was done on macOS. It was done again on the rented Linux machine, and the two agreed; the Linux result is in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`. **Second results, not replacements.**

**Hand test 1 — whether an output-off switch exists.** `RenderSettings.save_output = MISSING`, the same as on the Mac. There is no such property in 4.5.

**Hand test 4, case 2 — removing the compositor's output point.** What came back, word for word:

```
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

The raw render still reached the file. The same as on the Mac.

**Hand test 10 — whether the place frames are written can be switched off.** Four files were built and rendered from the command line. What came back, word for word — the build lines, then each render's `Saved:` lines, then the folder listings:

```
BLENDER_VERSION 4.5.12 LTS
SAVED emptypath.blend with render.filepath = ''
render.save_output MISSING in this version; file saved with the same tree and no such switch
SAVED saveoff.blend
SAVED mutedfo.blend with the File Output node muted
SAVED unmutedfo.blend
END_OF_BUILD
----- emptypath.blend -a
Saved: '0001.png'
Saved: '0002.png'
----- saveoff.blend -o cli_saveoff_ -F PNG -a
Saved: '/root/smoozy/The compositor/work/out_write/4.5/saveoff_fileoutput/Image0001.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/cli_saveoff_0001.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/saveoff_fileoutput/Image0002.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/cli_saveoff_0002.png'
----- mutedfo.blend -a
Saved: '/root/smoozy/The compositor/work/out_write/4.5/muted_main_0001.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/muted_main_0002.png'
----- unmutedfo.blend -a
Saved: '/root/smoozy/The compositor/work/out_write/4.5/unmuted_fileoutput/Image0001.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/unmuted_main_0001.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/unmuted_fileoutput/Image0002.png'
Saved: '/root/smoozy/The compositor/work/out_write/4.5/unmuted_main_0002.png'
----- listings
0001.png
0002.png
cli_saveoff_0001.png
cli_saveoff_0002.png
emptypath.blend
muted_fileoutput
muted_main_0001.png
muted_main_0002.png
mutedfo.blend
saveoff.blend
saveoff_fileoutput
unmuted_fileoutput
unmuted_main_0001.png
unmuted_main_0002.png
unmutedfo.blend
--- saveoff_fileoutput:
Image0001.png
Image0002.png
--- muted_fileoutput:
--- unmuted_fileoutput:
Image0001.png
Image0002.png
```

The same as on the Mac on every point: an emptied File Path does not stop the writing, it moves it — the frames land in the folder Blender was started in, named for the frame number alone. A muted File Output node writes nothing while the scene's own output still writes. And there is no switch that turns the writing off.

### Observed — the same window test on the rented Linux machine, 2026-08-06

**Hand test 16 — whether the Output panel's header carries a checkbox.** The Properties editor was put on the Output tab, made to fill the window, scrolled so the panel and its header were both visible, and photographed with Blender's own screenshot command. The header reads `∨ Output` with **no checkbox** beside it. The panel holds, top to bottom: the file path field reading `/tmp/` with a folder button; **Saving** with **File Extensions** (ticked) and **Cache Result** (unticked); **File Format** reading **PNG**; **Color** as **BW / RGB / RGBA** with RGBA chosen; **Color Depth** as **8 / 16** with 8 chosen; **Compression** reading **15%**; **Image Sequence** with **Overwrite** (ticked) and **Placeholders** (unticked); and two closed sub-panels, **Color Management** and **Pixel Density**. There is **no Media Type** control. That is what the Creator reported on the Mac.

**Outside the interface.** This settles nothing about it.

**Operating system.** It was done on Linux and settles Linux. With the Mac result above it, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Answer: to one scene.** The compositing node tree is a property of a scene, and a file can hold many scenes.

> "node_tree
>
> Compositing node tree
>
> Type:
>
> NodeTree, (readonly)"

and, on the same page:

> "use_nodes
>
> Enable the compositing node tree"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html
- Date read: 2026-07-30

> "Scenes are a way to organize your work.
> Each blend-file can contain multiple scenes, which share other data such as objects and materials."

- Source: manual — https://docs.blender.org/manual/en/4.5/scene_layout/scene/introduction.html
- Date read: 2026-07-30

That one scene's compositor can pull from another scene is also stated — the Render Layers node carries a Scene setting:

> "Scene
>
> The scene for which to render a view layer."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html
- Date read: 2026-07-30

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render
>
> layer
>
> …
>
> scene
>
> Type:
>
> Scene"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.CompositorNodeRLayers.html
- Date read: 2026-07-30

**Node groups are a different thing.** They are node-group data-blocks listed in the blend-file, and may be linked or appended from another file:

> "node_groups
>
> Node group data-blocks
>
> Type:
>
> BlendDataNodeTrees bpy_prop_collection of NodeTree, (readonly)"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.BlendData.html
- Date read: 2026-07-30

> "Node Groups
>
> This section lists all the node groups, both those in the current blend-file and those
> Linked or Appended from another blend-file."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/types/groups.html
- Date read: 2026-07-30

**LABEL: documented.**

**Not covered.** Whether two scenes can be made to share one and the same compositing tree is not stated outright. The API says `Scene.node_tree` is read-only, from which I would take that you cannot point a second scene at an existing tree through the API — but that is **inferred**, and the step is "read-only means the property cannot be assigned", which the source does not spell out for this property. Copying a scene ("Linked Copy", "Full Copy") is described on the scene page in terms of objects and object data; it says nothing about what happens to the compositing tree.

**Outside the interface.** The sources do not say this differs. The scene is chosen for a command-line render with `-S, --scene <name> — Set the active scene <name> for rendering` (manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30), so which scene is rendered — and therefore which compositor is the one in play — is selectable from outside. That last clause is **inferred** from this answer plus that argument's description.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and again on a rented Linux machine, and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. Nothing in the pages above mentions any operating system.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 5 — whether two scenes can be made to share one compositing tree.** Two scenes were made, A rendering blue and B rendering green. A was given a compositing tree, and the script then tried to point B at the same tree. What came back, word for word:

```
RESULT sharing one tree between two scenes -> REFUSED: bpy_struct: attribute "node_tree" from "Scene" is read-only
RESULT shared tree, rendering scene A -> [0.7804, 0.7529, 0.4235, 1.0]
RESULT shared tree, rendering scene B -> [0.4275, 0.7686, 0.3137, 1.0]
```

`[0.7804, 0.7529, 0.4235, 1.0]` is blue inverted; `[0.4275, 0.7686, 0.3137, 1.0]` is plain green. So in 4.5 the sharing was refused outright, and B went on rendering through its own compositor. This turns the step marked inferred above — that a second scene cannot be pointed at an existing tree, because the property is read-only — into an observed one, with the exact refusal quoted.

**Hand test 12 — whether choosing a scene from outside picks that scene's compositor.** One file, `scenes.blend`, was saved holding two scenes: A rendering blue with a compositor that inverts it to yellow, and B rendering green with a compositor that inverts it to magenta, both with their Compositing switch on. Two renders were then run from a terminal, `blender -b --factory-startup scenes.blend -S A -o .../scene_A_ -F PNG -f 1` and the same with `-S B`, and the written frames read back. What came back, word for word:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [0.7804, 0.7529, 0.4235, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
```

Yellow for A, magenta for B. So `-S` picks the scene, and that scene's own compositor is the one that runs.

**Outside the interface, from the tests.** The clause marked inferred above — that which scene is rendered from the command line decides which compositor is in play — is now observed, by hand test 12.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 4.5's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 17 and 20.

**Hand test 17 — what the Compositor editor's header offers.** The Compositing tab along the top of the window was clicked, in a file where nothing had been set up. The Creator's report:

> There is a "Use Nodes" checkbox, off by default. The View, Select, Add and Node menus are present but everything in Add is greyed out until it's ticked. Ticking it creates the standard setup: Render Layers connected through a reroute to a Viewer node and to a Composite node. Right under the workspace menu bar it reads "Scene > Compositing Node Tree".

So the editor's header offers no way to name or choose a node tree in 4.5. What it shows instead is a path reading "Scene > Compositing Node Tree", which is the tree hanging off the scene — the arrangement this answer describes from the sources.

**Hand test 20 — what the Render menu offers.** The Video Editing tab along the top of the window was clicked, then the Render menu in the very top bar was opened. The Creator reported the menu is identical in all four versions and answered with one screenshot, saved as `all versions.png` in a folder called `Test 20` on their own computer. What the picture shows, in order:

- Render Image — F12
- Render Animation — ⌘ F12
- Render Audio…
- View Render — F11
- View Animation — ⌘ F11
- Lock Interface — a checkbox, unticked

with dividing lines after "Render Animation", after "Render Audio…" and after "View Animation". **No entry in it names a Sequencer Scene.**

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 4.5.12 LTS. **Second results, not replacements.**

**Hand test 5 — whether one tree can serve two scenes.** What came back, word for word:

```
RESULT sharing one tree between two scenes -> REFUSED: bpy_struct: attribute "node_tree" from "Scene" is read-only
RESULT shared tree, rendering scene A -> [0.7804, 0.7529, 0.4196, 1.0]
RESULT shared tree, rendering scene B -> [0.4275, 0.7686, 0.3137, 1.0]
```

The same as on the Mac: 4.5 refuses to hand one scene's tree to another, and each scene keeps its own — A composites to yellow, B is left as its raw green.

**Hand test 12 — whether the command line's scene choice picks that scene's compositor.** Two scenes were built into one file, each with its own compositor, and each rendered by naming it with `-S` on the command line. What came back, word for word:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [0.7804, 0.7529, 0.4196, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0]
```

A comes back yellow and B magenta, so naming a scene picks that scene's own compositor. The same as on the Mac.

### Observed — the same window tests on the rented Linux machine, 2026-08-06

**Hand test 17 — what the Compositor editor's header offers.** Written out in full under question 1: a **Use Nodes** checkbox in 4.5, and the breadcrumb below the header reading the scene's name and its compositing node tree. The same as the Creator's Mac report.

**Hand test 20 — what the Render menu offers.** The top bar's Render menu was opened and photographed. It is headed **Render** and holds, in order: **Render Image** (F12), **Render Animation** (Ctrl F12), **Render Audio…**, a separator, **View Render** (F11), **View Animation** (Ctrl F11), a separator, and **Lock Interface** as an unticked checkbox. **No entry names a Sequencer Scene.** One thing to say plainly about the picture: a tooltip reading "Render active scene." had appeared over the item under the pointer and covers part of the Render Animation row; the shortcut column is still legible behind it. The menu is the same as the Creator reported on the Mac.

**Outside the interface.** These two settle nothing about it.

**Operating system.** Both were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**Answer: no — the second scene's compositor is not run; what comes across is that scene's raw render output.**

The carrying words are a Hint on the Render Layers node page:

> "Hint
>
> To use the compositing output from another scene rather than its “raw” render output,
> first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
> then load those images into the Compositor of the current scene using the
> Image Node."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html
- Date read: 2026-07-30

**LABEL: inferred.** The step a reader has to take: the source does not say "the other scene's compositor does not run". It says that to get the other scene's *compositing output* you must first render that scene out to files and read the files back in. If pointing a Render Layers node at another scene already gave you that scene's compositing output, this instruction would have nothing to fix. Hence the node gives the raw render, and the other scene's compositor is not part of the current scene's render. I flag this as the kind of inference that is easy to over-read, and it is one of the things I list at the end as settleable by hand in a few minutes.

Related facts are **documented**:

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html (the same sentence also appears on https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch_stereo_view.html)
- Date read: 2026-07-30

The 4.5 fixed-bug list names a compositor case that references another scene:

> "Crash when compositor references another scene"

- Source: release notes — https://developer.blender.org/docs/release_notes/4.5/bugfixes/
- Date read: 2026-07-30

The render-cache setting also mentions indirectly rendered scenes:

> "use_render_cache
>
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**Not covered.** Whether the second scene's own Post Processing settings (its Compositing and Sequencer checkboxes) have any effect at all in this situation. Whether the second scene's video sequence editor runs. Whether the second scene's own Composite node is even looked at. Whether the second scene's frame range, resolution or output path play any part. None of these is stated. Also not settled: what "raw render output" means precisely in terms of which passes cross over — the node's outputs are described as Image, Alpha and "Additional outputs for any enabled render passes" on the page quoted above.

**Outside the interface. Now observed.** The tests below were run from a terminal with no window open, and gave the result described.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and again on a rented Linux machine, and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. Nothing on these pages mentions any operating system.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 7 — a Render Layers node pointing at another scene.** Two 64×64 scenes were built. B renders a flat green slab, and its compositor sends its Render Layers node through an Invert node to its Composite node — so B's own composite is magenta — and also straight into a File Output node aimed at an empty folder, so that any work B's compositor does leaves a file behind. A renders blue on its own, and its whole compositor is one Render Layers node whose Scene is set to B, wired to A's Composite node. A was rendered twice, with B's Compositing switch on and then off, and B's File Output folder was checked each time. Three controls were rendered as well. What came back, word for word:

```
RESULT B.use_compositing=True -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=True -> files in B's File Output folder: []
RESULT B.use_compositing=False -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=False -> files in B's File Output folder: []
CONTROL A alone, compositing off -> [0.0745, 0.3333, 0.8196, 1.0]
CONTROL B alone, compositing off -> [0.4275, 0.7686, 0.3137, 1.0]
CONTROL B alone, compositing on  -> [0.7882, 0.4588, 0.7882, 1.0]
```

The controls give the three colours to read the result by: A alone is `[0.0745, 0.3333, 0.8196, 1.0]`, B's raw render is `[0.4275, 0.7686, 0.3137, 1.0]`, and B's composite is `[0.7882, 0.4588, 0.7882, 1.0]`. What A wrote was B's raw render, both times. B's File Output folder was empty both times, although rendering B directly does fill it.

**Hand test 8 — a Scene strip pointing at another scene.** The same two scenes, except that A takes B in through a Video Sequencer Scene strip with A's Sequencer switch on, and A's own compositor is a plain pass-through that changes nothing. Rendered four times, once for each combination of the two scenes' Compositing switches, clearing B's File Output folder before each run. What came back, word for word:

```
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['Image0001.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['Image0001.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

The written frame follows B's switch and not A's, and B's File Output node writes a file exactly when B's switch is on.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Both tests were run again, unchanged, on the rented Linux machine, with Blender 4.5.12 LTS started with no window and with `--factory-startup`. **Second results, not replacements.**

**Hand test 7 — a Render Layers node pointing at another scene.** What came back, word for word:

```
RESULT B.use_compositing=True -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=True -> files in B's File Output folder: []
RESULT B.use_compositing=False -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=False -> files in B's File Output folder: []
CONTROL A alone, compositing off -> [0.0745, 0.3333, 0.8196, 1.0]
CONTROL B alone, compositing off -> [0.4275, 0.7686, 0.3137, 1.0]
CONTROL B alone, compositing on  -> [0.7882, 0.4549, 0.7882, 1.0]
```

Green is `[0.4275, 0.7686, 0.3137, 1.0]` and B's own composite is magenta. A got B's raw green render both times, and B's File Output folder stayed empty both times. The same as on the Mac.

**Hand test 8 — a Scene strip pointing at another scene.** What came back, word for word:

```
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0] ; files in B's File Output folder: ['Image0001.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0] ; files in B's File Output folder: ['Image0001.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

The frame follows B's switch and not A's, and B's File Output node writes exactly when B's switch is on. The same as on the Mac.

**Outside the interface.** Both were run from a terminal with no window, as on the Mac.

**Operating system.** Both were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

Three parts, answered in turn. There is an ambiguity in the third part which I flag below.

### (a) What it arrives as

It arrives at a **Render Layers node**, as an image on that node's sockets, plus a socket per enabled pass.

> "Renders a View Layer and reads its
> Passes into the compositing node graph."
>
> "Outputs
>
> Image
>
> Rendered image.
>
> Alpha
>
> Alpha channel.
>
> Render pass sockets
>
> Additional outputs for any enabled render passes."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html
- Date read: 2026-07-30

What the main image is, in pass terms:

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/layers/passes.html
- Date read: 2026-07-30

Once inside, it is image data of one of exactly three kinds, and it carries a position and size in the compositing space:

> "Compositing nodes operate on data that is either an image or a dimensionless single value. For
> instance, the Levels node outputs a single value, while the
> Render Layers node outputs an image."
>
> "Three types of data exist, all of which are stored in half precision formats:
>
> Float
> …
> Vector
> …
> Color
>
> A 4D vector storing the Red, Green, Blue, and Alpha of the color."
>
> "Consequently, images are not only represented by their size, but also by their transformation in
> that space, much like 3D objects have transformations. … The rectangular area occupied by an image in that space as defined
> by its transformation and size is called the Domain of the image."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html
- Date read: 2026-07-30

**LABEL for (a): documented.**

### (b) Whether it can be told apart from anything else there

Two different answers depending on what "it" means.

**The node can be told apart.** A Render Layers node is a distinct node type with a Scene and a View Layer named on it — quoted in question 3 (`CompositorNodeRLayers`, "Input render passes from a scene render", with `scene` and `layer`). The source facts are **documented**. The conclusion that they make a render's point of entry identifiable as belonging to a named scene and view layer is **inferred**. The step is that the node type identifies the entry point and its two properties name the scene and view layer.

**The data itself carries no marker.** The compositor's data description above admits only Float, Vector and Color, with a domain (a transformation and a size). Nothing in it records where an image came from. I searched the whole compositing chapter and the compositor node-tree API page for "provenance", "identify the image", "which node produced" and "tag the image" and found nothing; I read the Compositor System page in full, the Image node page, the Image Info node page (whose outputs are Dimensions, Resolution, Location, Rotation and Scale — all geometry, no origin), and the Image Coordinates and Cryptomatte pages. **LABEL for this part: not in the sources.**

### (c) Whether it can be followed forward

**The ambiguity.** "nodes that only pass it along unchanged or split it into more than one" reads most naturally to me as *reroute nodes and muted nodes* (pass through unchanged) and *one output feeding several inputs* (split into more than one). That is the reading I answer. The other reading would be nodes that split an image into its components — Separate Color, Separate XYZ — and I do not answer that one, beyond noting that such nodes exist in the node list on https://docs.blender.org/manual/en/4.5/compositing/index.html.

Muted nodes are documented as passing links through unchanged. Reroute nodes are documented as taking one input and allowing several output connections:

> "A node used primarily for organization.
> Reroute looks and behaves much like a socket on other nodes in that it supports one input
> connection while allowing multiple output connections."

- Source: manual — https://docs.blender.org/manual/en/4.5/interface/controls/nodes/reroute.html
- Date read: 2026-07-30

> "class bpy.types.NodeReroute(NodeInternal)
>
> A single-socket organization tool that supports one input and multiple outputs"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.NodeReroute.html
- Date read: 2026-07-30

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change."

- Source: manual — https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html
- Date read: 2026-07-30

And links are readable end to end, in both directions, with muting visible:

> "links
>
> List of node links from or to this socket.
>
> Type:
>
> NodeLinks
>
> Note
>
> Takes O(len(nodetree.links)) time."

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html
- Date read: 2026-07-30

> "class bpy.types.NodeLink(bpy_struct)
>
> Link between nodes in a node tree
>
> from_node
> …
> from_socket
> …
> is_muted
>
> Link is muted and can be ignored"

(the same page also lists `to_node` and `to_socket`)

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.NodeLink.html
- Date read: 2026-07-30

Muted links are described the same way in the manual: "A muted link acts as though it's no longer there" (https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html, read 2026-07-30). A single node also has a mute flag of its own: `mute`, boolean, default False (https://docs.blender.org/api/4.5/bpy.types.Node.html, read 2026-07-30).

Node groups expose named boundary points — Group Input "Exposes the inputs of the node group" and Group Output "Receives the outputs of the node group" (https://docs.blender.org/manual/en/4.5/compositing/types/groups.html, read 2026-07-30). The quoted words do not say that what passes between those points is unchanged.

**LABEL for (c): documented** for each fact quoted; **inferred** for the conclusion that these facts together let you walk a render forward from its Render Layers node to whatever it eventually reaches. The step: each socket lists the links attached to it, each link names the node and socket at both ends, reroutes and muted nodes pass the signal along, so repeated lookups from the Render Layers node's outputs trace the whole downstream path. The sources do not describe anyone doing this.

**Not covered.** Nothing in the sources tells you whether following links in this way matches what the compositor actually evaluates — for instance whether a link that is muted, hidden or invalid (`is_hidden`, `is_valid` on `NodeLink`) is skipped by the compositor in the same way, or how a Switch node's chosen branch relates to the links that exist. Nothing settles the second reading of "split it into more than one". Nothing says whether pass sockets and the Image socket behave the same way when traced. Nothing addresses whether the data can be told apart *after* it has been mixed with other images.

**Outside the interface.** The link and socket properties quoted are Python API properties. That they are available in a background session is **inferred** (the step: they are data-block properties, and nothing on those pages ties them to a window or an editor). The Render Layers node's re-render button is described as "The button next to the dropdown re-renders it immediately" (manual — https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html, read 2026-07-30). That it is an interface-only affordance is **inferred** from it being a button beside a dropdown.

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The test below was run on macOS and again on a rented Linux machine, and the two runs agreed; the Linux result is in its own section further down. **The exception is hand test 19, what hovering a link shows.** That was attempted on the Linux machine and could not be settled there, so that reading rests on the Mac alone — see the two sections further down. Windows was not tried, because no Windows machine has run it. Nothing on these pages mentions any operating system. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. Hand test 19 is part of this answer and is not settled on Linux, as this document says a hundred lines further down. Narrowed to name the exception.)*

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 11 — telling a render apart, and following it forward, with no window open.** One 64×64 scene was given a compositor holding a Render Layers node named `TheRender`, an Image node named `TheImage`, a Reroute named `TheReroute`, a Blur node named `MutedBlur` that was muted, a Viewer named `TheViewer`, and the Composite node. The render's Image socket goes into the Reroute; the Reroute feeds the muted Blur, which feeds the Composite node; the Reroute also feeds the Viewer through a link that was itself muted. Blender was then asked to print the nodes' identifiers, every link, a walk forward from the render's Image socket, and what the muted node exposes. What came back, word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
RenderLayers output sockets = ['Image', 'Alpha', 'Depth', 'Normal', 'UV', 'Vector', 'Position', 'Deprecated', 'Deprecated', 'Shadow', 'AO', 'Deprecated', 'Deprecated', 'Deprecated', 'IndexOB', 'IndexMA', 'Mist', 'Emit', 'Env', 'DiffDir', 'DiffInd', 'DiffCol', 'GlossDir', 'GlossInd', 'GlossCol', 'TransDir', 'TransInd', 'TransCol', 'SubsurfaceDir', 'SubsurfaceInd', 'SubsurfaceCol']
--- every link in the tree
LINK TheRender.Image -> TheReroute.Input  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> MutedBlur.Image  is_muted=False is_valid=True is_hidden=False
LINK MutedBlur.Image -> Composite.Image  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> TheViewer.Image  is_muted=True is_valid=True is_hidden=False
--- walking forward from the Render Layers Image socket
WALK TheRender.Image -> TheReroute.Input (muted link=False, muted node=False)
WALK   TheReroute.Output -> MutedBlur.Image (muted link=False, muted node=True)
WALK     MutedBlur.Image -> Composite.Image (muted link=False, muted node=False)
WALK   TheReroute.Output -> TheViewer.Image (muted link=True, muted node=False)
--- what the muted node exposes
MutedBlur.mute = True ; internal_links = [('Image', 'Image')]
--- is_active_output on output nodes
```

Three things this settles. The two node types carry different identifiers, so a program can tell the render's entry point from an image file's. The `layer` property reports `'ViewLayer'`, not the `'PLACEHOLDER'` the reference page prints. And the walk ran to completion, reaching both destinations through the Reroute, with the muted link and the muted node each flagged on the way. The `is_active_output` section printed nothing, because no node in a 4.5 compositing tree carries that property.

**Hand test 2 — what the Split node offers.** Blender was asked to add each named node type to a compositing tree and print its sockets. For the Split node:

```
ADD CompositorNodeSplit -> inputs=['Factor', 'Image', 'Image'] outputs=['Image']
```

The node has one output socket. The 4.5 manual's Split page says "This node has no output sockets", so the running program and that page do not agree.

**Outside the interface. Now observed.** The whole of hand test 11 ran in a session with no window open, so the link and socket properties quoted above are readable that way. The step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal test above. The Creator ran them by hand in Blender 4.5's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 19 and 21.

**Hand test 19 — what hovering a link shows.** A small tree was built in the Compositing editor — a Render Layers node and an Image node with an image loaded, each feeding its own Viewer node — one frame was rendered and the render window closed, and then the link out of each node was hovered. The Creator's report for 4.5:

> Hovering shows "Image." — with a trailing period.

So something does appear in 4.5, and it is the socket's name. The Creator reported one wording and did not say it differed between the two links, so this record cannot say whether the Render Layers link and the Image link showed different things. Nothing in what was reported names where the image came from.

**Hand test 21 — how a second output node is marked.** Quoted in full under question 1. A Render Layers node was wired to two Composite nodes; the active one carries a dark red header and the inactive one grey, and the last node clicked is the active one.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Test 21 was done again on the rented Linux machine and agreed; test 19 was attempted there and could not be settled. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 4.5.12 LTS. **Second results, not replacements.**

**Hand test 2 — which compositor nodes exist, and what the Split node's sockets are.** The printed block is long; the lines that carry the answer came back word for word as:

```
BLENDER_VERSION 4.5.12 LTS
TREE_TYPE CompositorNodeTree
TREE_STARTING_NODES ['CompositorNodeComposite', 'CompositorNodeRLayers', 'CompositorNodeViewer', 'NodeReroute']
ADD CompositorNodeSplit -> inputs=['Factor', 'Image', 'Image'] outputs=['Image']
ADD CompositorNodeSwitch -> inputs=['Switch', 'Off', 'On'] outputs=['Image']
ADD CompositorNodeRLayers -> inputs=[] outputs=['Image', 'Alpha', 'Depth', 'Normal', 'UV', 'Vector', 'Position', 'Deprecated', 'Deprecated', 'Shadow', 'AO', 'Deprecated', 'Deprecated', 'Deprecated', 'IndexOB', 'IndexMA', 'Mist', 'Emit', 'Env', 'DiffDir', 'DiffInd', 'DiffCol', 'GlossDir', 'GlossInd', 'GlossCol', 'TransDir', 'TransInd', 'TransCol', 'SubsurfaceDir', 'SubsurfaceInd', 'SubsurfaceCol']
ADD CompositorNodeOutputFile -> inputs=['Image'] outputs=[]
ADD CompositorNodeViewer -> inputs=['Image'] outputs=[]
ADD CompositorNodeComposite -> inputs=['Image'] outputs=[]
```

Identical to the Mac's, socket for socket, including the Split node having one output rather than none.

**Hand test 11 — telling a render apart and following it forward.** What came back, word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
--- every link in the tree
LINK TheRender.Image -> TheReroute.Input  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> MutedBlur.Image  is_muted=False is_valid=True is_hidden=False
LINK MutedBlur.Image -> Composite.Image  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> TheViewer.Image  is_muted=True is_valid=True is_hidden=False
--- walking forward from the Render Layers Image socket
WALK TheRender.Image -> TheReroute.Input (muted link=False, muted node=False)
WALK   TheReroute.Output -> MutedBlur.Image (muted link=False, muted node=True)
WALK     MutedBlur.Image -> Composite.Image (muted link=False, muted node=False)
WALK   TheReroute.Output -> TheViewer.Image (muted link=True, muted node=False)
--- what the muted node exposes
MutedBlur.mute = True ; internal_links = [('Image', 'Image')]
--- is_active_output on output nodes
```

Identical to the Mac's, line for line, including the view-layer property reporting a real name rather than a placeholder, and no `is_active_output` line in 4.5.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 21 — how a second output node is marked.** Written out in full under question 1: the first Composite node has a dark red header and the second a grey one, and neither node's header carries any control for making it the live one. That agrees with the Creator's Mac report and closes the part they left unanswered.

**Hand test 19 — what hovering a link shows. This could not be settled here, and no answer is written in.** A Blender window was opened on the rented Linux machine with the same tree the Mac test used, one frame was rendered, and the pointer was put over the link out of the Render Layers node, over that node's own Image output socket, and over the link out of the Image node. It was tried four ways: with Blender moving its own pointer to coordinates it worked out from the node positions; with the pointer moved from outside in several small steps and held; with dwells of 2.5 seconds and of 8 seconds; and with the picture taken by Blender itself on a signal, so the pointer was checked to be in place first. No display appeared in any of them. As a check on the method, the pointer was then put on the **Use Nodes** checkbox in the same header — a control that certainly has a tooltip — for 2.5 seconds with its position confirmed, and no display appeared there either. **That is a fault in the run, not a finding about Blender.** The machine's display is shared with other topics being tested at the same moment: one check printed `AT 438,227 -> x:573 y:68`, meaning something else moved the pointer between putting it there and reading it back. So the socket-inspection reading for 4.5 still rests on the Mac alone. What to run when the machine is free is written out in `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

**Outside the interface.** These settle nothing about it.

**Operating system.** Test 21 now has an answer from both machines and they agree. Test 19 has an answer from the Mac only; Linux and Windows are both open for it.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Answer: yes — at least the render region with Crop to Render Region, and the stereo Views Format.** In the other direction, the sources are explicit that compositor transforms do not change the final compositor output size. They do not settle the size written by every file-writing node.

**Render region.** Two settings sit alongside Resolution X/Y and Percentage in the same panel:

> "Render Region
>
> Renders just a portion of the view instead of the entire frame.
> See the Render Region
> documentation to see how to define the size of the render region.
>
> Crop to Render Region
>
> Crops the rendered image to the size of the render region,
> instead of rendering a transparent background around it."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/format.html
- Date read: 2026-07-30

The API says the same in fewer words:

> "use_border
>
> Render a user-defined render region, within the frame size"
>
> "use_crop_to_border
>
> Crop the rendered frame to the defined render region size"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

The region itself is drawn in the viewport and applies to the final render:

> "Allows you to limit rendering to a 2D rectangular area. … If you define one while in Camera View,
> it will apply not just to the viewport, but also to the final render."
>
> "Render regions only apply to the viewport when using Cycles, not when using EEVEE.
> However, they always affect the final render."

- Source: manual — https://docs.blender.org/manual/en/4.5/editors/3dview/navigate/regions.html
- Date read: 2026-07-30

I looked at the picture of the Format panel. Visible in it: "Resolution X 1920 px", "Y 1080 px", "% 100%", "Aspect X 1.000", "Y 1.000", an unticked checkbox "Render Region", a greyed-out checkbox "Crop to Render Region", and "Frame Rate 24 fps".

- Source: manual, image `_images/render_output_properties_format_panel.png`, shown on https://docs.blender.org/manual/en/4.5/render/output/properties/format.html
- Label for what I take from the picture: **documented**
- Date read: 2026-07-30

**Stereo output format.** When stereoscopy is on and the views are written as a single Stereo 3D image, the two views are combined into one image:

> "SIDEBYSIDE
> Side-by-Side – Render views for left and right eyes side-by-side.
>
> TOPBOTTOM
> Top-Bottom – Render views for left and right eyes one above another."
>
> "use_squeezed_frame
>
> Combine both views in a squeezed image"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Stereo3dFormat.html
- Date read: 2026-07-30

> "The Top-Bottom compressed will discard half of your vertical resolution data."
>
> "For the Stereo 3D formats the only lossless options are
> Top-Bottom and Side-by-Side without the Squeezed Frame option."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html
- Date read: 2026-07-30

That an unsqueezed Side-by-Side or Top-Bottom file therefore has different pixel dimensions from the scene's resolution is **inferred**. The step: two full-resolution views placed beside or above one another, with no data discarded, cannot fit in one frame of the scene's own resolution. The sources state the losslessness, not the dimensions.

**What does *not* decide the final compositor output size.** The compositor's own transforms do not change that size, because the final compositor output is pinned to the render size:

> "The operation domain of output nodes like the Viewer node is
> a domain with an identity transformation and the same size as the final compositor output. For
> viewport compositing, that size would be the viewport size, and for
> final render compositing, that size would be the scene render size."

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html
- Date read: 2026-07-30

and, said again from the stereoscopy side:

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html
- Date read: 2026-07-30

This holds even though individual nodes plainly do resize images — the Crop node's Alpha Crop option "Sets the areas outside of the crop region to be transparent instead of actually cropping the size of the image", so with it off the image is genuinely made smaller (https://docs.blender.org/manual/en/4.5/compositing/types/transform/crop.html, read 2026-07-30), and the Scale node offers a "Render Size" space described as "Image dimensions set in the Render panel" (https://docs.blender.org/manual/en/4.5/compositing/types/transform/scale.html, read 2026-07-30). The final compositor output is still the render size.

**Things that look like they might and do not.** Aspect X/Y is described as controlling pixel shape — "Older televisions may have non-square pixels, so this can be used to control the shape of the pixels along the respective axis" — and pixel density "is meta-data which doesn't impact the quality of the image" (manual — https://docs.blender.org/manual/en/4.5/render/output/properties/format.html and https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30). The conclusion that neither changes the pixel dimensions is **inferred**: the descriptions concern pixel shape and metadata, not pixel count. Burning metadata into the frame is described as "Add metadata as text to the render", which I read as drawing over the frame rather than adding to it — that reading is **inferred** (https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html, read 2026-07-30).

**LABEL: documented** that Crop to Render Region crops *the rendered image* to the region's size, and for compositor transforms not changing the final compositor output size; **inferred** for the step from "the rendered image is cropped" to "the file written to disk is that size" — the quoted words say "rendered image" and "rendered frame", never "written" or "saved"; **inferred** for the stereo dimensions conclusion, with the step written out above. *(Citation check, 2026-08-03: the render-region half was previously labelled documented outright. The one-step gap between the rendered image and the written file was not marked. The hand test below measures the written file and settles it.)*

**Not covered.** Whether the File Output node writes at the scene render size too. The passage quoted above says "output nodes like the Viewer node", and the manual files the File Output node under Output Nodes alongside Composite and Viewer (https://docs.blender.org/manual/en/4.5/compositing/types/output/index.html, read 2026-07-30) — but extending the rule to it is **inferred**, and I flag it as the weakest thing in this answer and one I have put in the hand-test list. Its own page says nothing about size at all. Also not covered: whether a codec forces the size ("some codecs do not work with arbitrary dimensions. So, try to stick with common dimensions" — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30, which warns but does not say Blender changes the size); what size is written when the render is produced by the video sequence editor instead (question 7); and the exact pixel dimensions of any of the stereo formats.

**Outside the interface.** The sources do not say this differs. All of the deciding settings quoted are stored in the file, and the render region is noted as defined in the viewport but applying "always" to the final render. Path templates give a sideways confirmation that the final numbers are file-side: `resolution_x` / `resolution_y` as template variables are described as "The x and y resolution of the rendered image. This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively" (https://docs.blender.org/manual/en/4.5/files/file_paths.html, read 2026-07-30). Note that this template description accounts for resolution and percentage only — it says nothing about the render region.

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The tests below were run on macOS and again on a rented Linux machine, and the two runs agreed; the Linux results are in their own section further down. **The exception is the second half of hand test 18** — ticking Render Region and looking at Crop to Render Region again. Ticking a box needs the pointer, which could not be relied on on the Linux machine, so that half rests on the Mac alone. Windows was not tried, because no Windows machine has run it. The only operating-system caveat nearby in the sources is the list of render formats "not available on all systems" quoted under question 2. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. The second half of hand test 18 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)*

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 6 — three things that might decide the written size.** A 400×200 Workbench scene with a cube and a camera was rendered three ways. First, with a Scale node set to Absolute 120×90 between the Render Layers node and a File Output node, while the unscaled render went to the scene's own output. Second, with the render region set to the middle half of the frame — 0.25 to 0.75 on both axes — and Crop to Render Region off and then on. Third, with stereoscopy on and both views written into one file, Side-by-Side and Top-Bottom, squeezed and unsqueezed. What came back, word for word:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> Image0001.png is 120x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

This settles the item flagged above as the weakest thing in the answer. The File Output node wrote at the size of the image plugged into it, 120×90, while the scene's own output stayed at 400×200 in the same render. So the rule about output nodes taking the final compositor size does not extend to the File Output node. It also turns the stereo step marked inferred above into a measurement: unsqueezed Side-by-Side doubles the width and unsqueezed Top-Bottom doubles the height, while the squeezed forms keep the scene's own dimensions.

**Hand test 13 — what the output-path template reports under a crop.** The same 400×200 scene, with the render region cropped to the middle half and the output path set to `{resolution_x}x{resolution_y}_`. What came back:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The template reports the scene's own Resolution values, not the size actually written.

**Hand test 9 — whether the sequencer route changes the written size.** A 400×200 scene was rendered six ways across the Sequencer and Compositing switches, with and without a Scene strip. Every one of the six wrote a 400×200 file; the six lines are quoted under question 7.

**Hand test 14 — a very large frame.** A 9000×400 scene with an Invert compositor was rendered with the Compositor Device set each way:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
```

`[0.7882, 0.4588, 0.7882, 1.0]` is the inverted green, so the compositor ran and the size was not changed, on either device.

**Outside the interface. Now observed.** All of the above ran from a terminal with no window. The step marked inferred above — that the file's stored sizing settings are what applies — is observed for the render region, the crop, the stereo formats and the File Output node.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 4.5's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 16 and 18.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor's Output tab was opened at the Format panel, the two checkboxes looked at with Render Region unticked, then Render Region ticked and them looked at again. The Creator's report, which they gave for all four versions at once:

> All four versions: Render Region is clickable, off by default; while off, Crop to Render Region is greyed out. Ticking Render Region makes Crop to Render Region available, but it stays off by default.

That is what the manual's Format panel picture described above shows, seen now in the running program: both start off, and the second is unavailable until the first is on.

**Hand test 16 — what the Output panel holds.** Quoted in full under question 2. In 4.5 the panel has a path field, Saving with File Extensions and Cache Result, File Format, Color, Color Depth, Compression, and Image Sequence with Overwrite and Placeholders. There is no Media Type control and no checkbox in the panel header. Nothing in the panel offers a second way of deciding the written pixel size.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 4.5.12 LTS. **Second results, not replacements.**

**Hand test 6 — what decides the pixel size of what gets written.** What came back, word for word:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> Image0001.png is 120x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

Identical to the Mac's, figure for figure.

**Hand test 13 — the path template under a crop.** What came back, word for word:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The same as on the Mac: the template reports the scene's own numbers on a file that is half those in each direction.

**Hand test 14 — the compositor above the size 5.2's documentation names.** Written out in full under question 1: 4.5 writes the composited picture at 9000×400 on either device.

**Hand test 15 on the Linux list — the card against the processor.** Written out in full under question 1. Of four compositor trees rendered on both devices and compared value by value at 320×240, three matched to the last figure and the fourth, a blur, differed by at most one step of 255.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 16 — the Output panel.** Written out in full under question 2. No Media Type control in 4.5, and no checkbox in the panel's header.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor was put on the Output tab, made to fill the window, and photographed. The Format panel holds two checkboxes one above the other, labelled exactly **Render Region** and **Crop to Render Region**. Both start unticked, and **Crop to Render Region is drawn greyed out** — its label and its box both dimmed — while Render Region is unticked. **Only the first half of the Mac's test was run here.** The Mac's version also ticked Render Region and looked again, and found Crop to Render Region became available; ticking a box needs the pointer, which could not be relied on here, so the second half stays as the Mac has it.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux, except the second half of test 18, which stays macOS only. With the Mac results above them, macOS and Linux now both have an answer for the rest and they agree. Windows stays untested.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Answer: yes.** The same Post Processing panel that switches compositing off has a Sequencer switch beside it, and the sources describe the two separately.

> "Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

- Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html
- Date read: 2026-07-30

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist
>
> Type:
>
> boolean, default True"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

So: leave Sequencer on, have strips in the scene's sequence editor, and turn Compositing off (`use_compositing`, quoted under question 1). Both switches sit in one panel; I confirmed that from Cycles' own interface file, which builds that panel:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False

        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")

        layout.prop(rd, "dither_intensity", text="Dither", slider=True)
```

- Source: Cycles' own files — `Blender Sources/cycles_4.5/ui.py`, class `CYCLES_PT_post_processing`, lines 1102–1118 (the class statement is line 1102; the `dither_intensity` line is 1118)
- LABEL for this: **observed** — this is what the file says the panel contains: exactly `use_compositing`, `use_sequencer` and `dither_intensity`, under a heading "Pipeline", in the Output tab (`bl_context = "output"`). What the render then *does* is a step beyond the file and is not observed here.
- Date read: 2026-07-30

**What it changes about the render output.** Documented, from the quote above: the render output is the video sequence editor's result rather than the view through the scene's active camera. With Compositing off, the Scene strips inside that sequence contribute their own raw renders rather than their compositing output — that last clause is **inferred** from the sentence "If Compositing is also enabled, the Scene strip will be the output of the Compositor", read the other way round; the step is "the source states the compositor's output is what a Scene strip carries *when* compositing is enabled, so with it disabled the strip carries something else". *(Citation check, 2026-08-03: the step previously ended "…something else, namely the scene's render." Reading the sentence backwards reaches "something else"; it does not identify what. That last identification is struck from the step. Hand test 8 below measures it.)*

One further documented consequence, about content rather than pixels:

> "Limitations
>
> Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

- Source: manual — https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html
- Date read: 2026-07-30

And a scene's sequence cannot feed on itself:

> "Scene strips cannot be used to reference the sequence's own scene; a secondary scene must be used instead."

- Source: same page and date

Metadata can also be taken from the strips rather than the scene — "Sequencer Strips: Use metadata from the strips in the Sequencer" (https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html, read 2026-07-30).

There is one more sequencer switch whose description I record because it names the sequencer and scene strips together:

> "use_sequencer_override_scene_strip
>
> Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip"

- Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**LABEL: documented** that the switch exists and that it substitutes the sequencer's output for the camera view; **inferred** for what turning Compositing off does to Scene strips, with the step written out.

**Not covered.** The sources do not say what pixel size the sequencer's output is written at, nor whether the scene's own 3D render still happens when the sequencer supplies the image, nor what is written if Sequencer is on but the scene has no strips (the API only says the sequencer pipeline is used "if sequencer strips exist"), nor whether multi-layer output such as EXR passes survives this route. The Proxy Render Size setting is described as affecting the preview — "Controls the preview resolution" (https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html, read 2026-07-30) — and I found no statement either way about the final render, so I do not claim one.

**Outside the interface.** The sources do not say. Both switches are scene properties stored in the file, and no command-line option in the list on the command-line arguments page touches either — but note that a viewport-snapshot operator does take a sequencer flag: `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)`, with `sequencer` described as "Sequencer, Render using the sequencer's OpenGL display" (https://docs.blender.org/api/4.5/bpy.ops.render.html, read 2026-07-30). That is a different thing from the Post Processing switch and I do not claim they are related.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and again on a rented Linux machine, and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. Nothing on these pages mentions any operating system.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 9 — what the sequencer route writes.** A 400×200 scene was built whose camera view is blue, `[0.0745, 0.3333, 0.8196, 1.0]`, and whose compositor replaces the picture with flat red, `[0.8549, 0.2196, 0.1255, 1.0]`, so the two are told apart at a glance. A second scene rendering green, `[0.4275, 0.7686, 0.3137, 1.0]`, was added as a Scene strip for half the runs. Six renders were made across the Sequencer and Compositing switches. What came back, word for word:

```
RESULT no strips, sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer OFF, compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT strip,     sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer OFF, compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
```

This answers the two things listed above as not covered. With Sequencer on and no strips, a file is written and it holds the scene's ordinary picture — the camera view when Compositing is off, the compositor's result when it is on. With a strip present, the strip's picture is written whether the scene's own Compositing switch is on or off. And the written size stayed at the scene's own 400×200 in all six.

**Hand test 8 — what a Scene strip carries.** Quoted in full under question 4. The four runs show that the picture follows the strip's own scene's Compositing switch, not the rendering scene's, and that the strip's scene's File Output node writes a file exactly when that scene's switch is on. That settles which reading of "If Compositing is also enabled" is meant, and it settles the clause marked inferred above.

**Hand test 13 — what survives into a multi-layer EXR.** A scene with the Depth and Mist passes turned on was rendered to multi-layer OpenEXR twice: once plainly, and once with a Scene strip supplying the picture and the Sequencer switch on. The channel names in the two files:

```
exr_seqFalse.exr: ViewLayer.Combined.A ViewLayer.Combined.B ViewLayer.Combined.G ViewLayer.Combined.R ViewLayer.Depth.Z
exr_seqTrue.exr:  Composite.Combined.A Composite.Combined.B Composite.Combined.G Composite.Combined.R
```

Without the sequencer the file holds a view layer with its Combined and Depth passes. With the sequencer supplying the picture it holds one layer called Composite with only the combined image; the Depth pass is gone.

**Outside the interface. Now observed.** Every one of the runs above was started from a terminal with no window. Both switches take effect that way, and the step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 4.5's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15 and 20.

**Hand test 15 — what the Post Processing panel holds.** Quoted in full under question 1. The Creator's report on the Sequencer half:

> Two checkboxes — Compositing and Sequencer, both on by default. … Sequencer tooltip: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

That tooltip is the same sentence as the API description of `use_sequencer` quoted above, word for word, and both switches were seen on by default in the running program, which is the default the API gives. The two switches are the only checkboxes in the panel; the third control is the Dither slider.

**Hand test 20 — what the Render menu offers.** Quoted in full under question 3. The Creator opened the Video Editing tab and then the Render menu, reported it identical in all four versions, and supplied one screenshot. It holds Render Image (F12), Render Animation (⌘ F12), Render Audio…, View Render (F11), View Animation (⌘ F11) and a Lock Interface checkbox. **No entry in it names a Sequencer Scene.** So there is no separate menu command for rendering a sequencer's own scene here; the ordinary Render Image and Render Animation are what the menu offers.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 4.5.12 LTS. **Second results, not replacements.**

**Hand test 4 — the Compositing switch on its own.** Written out in full under question 1: with the switch off, the raw render reached the file; with it on, the compositor's result did.

**Hand test 8 — whose Compositing switch a Scene strip follows.** Written out in full under question 4: the strip carries the strip's own scene's composite when that scene's Compositing switch is on and its raw camera view when it is off, and the rendering scene's own switch makes no difference.

**Hand test 9 — what the sequencer route writes.** What came back, word for word:

```
RESULT no strips, sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer OFF, compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT strip,     sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer OFF, compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
```

Blue is the camera view, red is the compositor's result, green is the strip's. Identical to the Mac's, figure for figure: with the Sequencer switch on and no strips, the ordinary camera view is written; with a strip, the strip wins over the compositor; and the written size never changes.

**Hand test 13 — a multi-layer EXR through the sequencer.** What came back, word for word:

```
RESULT multi-layer EXR, sequencer=False -> file exists: True size 2058
RESULT multi-layer EXR, sequencer=True -> file exists: True size 1612
--- channels in exr_seqFalse.exr
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
ViewLayer.Depth.Z
--- channels in exr_seqTrue.exr
Composite.Combined.A
Composite.Combined.B
Composite.Combined.G
Composite.Combined.R
```

The same channel names as on the Mac: going through the sequencer collapses the file to one `Composite` layer and loses the depth pass.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 15 — what the Post Processing panel holds.** Written out in full under question 1: two checkboxes, Compositing and Sequencer, both ticked, and a Dither slider reading 1.00, and nothing else — and no checkbox in the panel's own header.

**Hand test 20 — what the Render menu offers.** Written out in full under question 3: Render Image, Render Animation, Render Audio…, View Render, View Animation and Lock Interface, and no entry naming a Sequencer Scene.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## Things only a running Blender can settle

Every test that was on this list has been run, and the results are written into the answers above. The exact set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`. Nothing is left on it.

The seven that needed somebody looking at a Blender window were run by the Creator and are recorded under the questions they answer: what the Post Processing panel holds (entry 15, question 1 and question 7), what the Output panel holds (entry 16, questions 2 and 6), what the Compositor editor's header offers (entry 17, questions 1 and 3), whether Crop to Render Region is greyed out until Render Region is on (entry 18, question 6), what hovering a link shows (entry 19, question 5), what the Render menu offers (entry 20, questions 3 and 7), and how a second Composite node is marked as the unused one (entry 21, questions 1 and 5).

Two small parts of those seven came back unanswered on the Mac, and **both have since been answered on a second machine**: the Post Processing panel's own header carries no checkbox, and neither Composite node's header carries a control for making it the live one. Both are written in beside their findings.

**A second machine, 2026-08-06.** Every test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are written in beside the Mac's, never merged with them. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`. Every test agreed with the Mac except one, which could not be settled there at all: what hovering a link shows (entry 19). One new test was added that only the second machine could carry: whether a compositor tree gives the same picture on the graphics card as on the processor. It does, at 320×240, for the four trees tried.

**Still cannot be run with the available equipment.** Two things stay untested. **Windows** — no Windows machine has run it, so every "operating system" line above covers macOS and Linux and leaves Windows open, and that is why the compositor's documented driver-reset limit on Windows, which the manual heads "TDR on Windows", stays untested. **A graphics card of another make** — the two machines have Apple's own graphics and one NVIDIA card between them, so nothing here speaks for an AMD or Intel card. Three smaller things need the rented machine to itself, because its display is shared: what hovering a link shows, what the tooltip on 5.2's Output header checkbox says, and what Crop to Render Region looks like once Render Region is ticked.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 4.5

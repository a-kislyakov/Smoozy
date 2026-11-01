# The compositor — Blender 5.2

This document records the sourced answers to the compositor questions for Blender 5.2.

**Sources consulted, and the date.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including pictures in its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All read on **2026-07-30** (the snapshot date given to me).

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

No helper agents were used. Every finding below is my own reading of the files named above.

---

## 1. What decides whether a file's compositor runs at all?

**Answer.** Three things have to line up, and the sources name all three.

*First*, the scene must have a compositing node group attached to it. The API lists this on the scene itself:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree | None"

*Second*, the Compositing pipeline switch must be on. The manual says:

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

and the API gives that switch its meaning and default:

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

*Third*, the node tree must contain an output the compositor can find:

> "The compositor searches for the active Group Output node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether."

One thing that does **not** decide it any more, despite its name:

> "use_nodes
>
> Enable the compositing node group. (default False)
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

Separately, whether the compositor runs *in the 3D Viewport as a preview* is decided by its own setting, which is not the same switch:

> "Compositor
>
> When to preview the result of compositing in the 3D Viewport.
>
> Disabled:
>
> Never show the compositing output.
>
> Camera:
>
> Only show the compositing output when in Camera View, which gives the best vantage point for previewing the final result.
>
> Always:
>
> Always show the compositing output."

**Label: documented** for each of the three conditions taken separately, and for the "Use Nodes" and viewport-preview entries. **Inferred** for the opening claim that the three "have to line up" and that the sources name all three — each is stated on its own page, and no source presents them as one joint condition or as a complete list. *(Citation check, 2026-08-03: the conjunction and the completeness claim were previously covered by the documented label; both are a step past the quoted words. This topic's 5.1 first-pass findings already mark the same step for the same question.)*

**Not covered.** The cited compositor pages do not say what happens when the Compositing switch is on, a node group is assigned, and the only output found is a Viewer node — whether a *file render* then produces the Viewer's image, the raw render, or nothing. The sentence about the active output is written in a section that also discusses viewport render. **Label: not in the sources.**

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/compositing/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/3dview/display/shading.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30

**Supporting finding from Cycles' own files (observed).** Cycles draws the Post Processing panel itself, and the two switches it draws are exactly these two properties:

> ```
> class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
>     bl_label = "Post Processing"
>     bl_options = {'DEFAULT_CLOSED'}
>     bl_context = "output"
>
>     def draw(self, context):
>         layout = self.layout
>         layout.use_property_split = True
>         layout.use_property_decorate = False
>
>         rd = context.scene.render
>
>         col = layout.column(align=True, heading="Pipeline")
>         col.prop(rd, "use_compositing")
>         col.prop(rd, "use_sequencer")
> ```

File: `Blender Sources/cycles_5.2/ui.py`, lines 1169–1183. Read 2026-07-30. This is what the file says. What a render *does* with those settings is a step beyond the file, and I have not taken it here.

**Supporting finding from a picture (documented).** The picture `_images/render_output_properties_post-processing_panel.png`, shown on the Post Processing manual page, shows a panel titled "Post Processing" containing two ticked checkboxes labelled "Compositing" and "Sequencer", and a "Dither" slider set to 1.000. Read 2026-07-30. What is visible is those two checkboxes side by side; nothing about what they do is visible in the picture.

**Outside the interface.** All three things are stored in the file (a scene property, a render setting, and the node tree's contents), and the render setting is reachable from Python as `scene.render.use_compositing`, so setting them does not require a person clicking. **Label: inferred.** The step is that the documented scene data and Python property can be read and changed without the interface. The command-line arguments page contains no argument for turning compositing on or off — I searched the whole page and found `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, `-F`, `-x` and the Cycles device options, and none of them touch compositing. That page alone does not establish whether compositor evaluation differs in background mode. **Label for that limit: not in the sources.**

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and the conditions behave there as described. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. One neighbouring fact is named for one system only: the compositor's Limitations page says that on Windows, "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." (https://docs.blender.org/manual/en/5.2/compositing/limitations.html, read 2026-07-30). That is about the GPU compositor being interrupted, not about whether it is asked to run — and it names the one operating system neither machine runs. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.**

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, using Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`, started with no window (`-b`) and with the Creator's own add-ons and preferences kept out of it (`--factory-startup`). Full set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

**Hand test 1 — what the switches are called, and what they start as.** Blender was started on its own empty startup file and asked to print each named property's value, or `MISSING` where it does not exist:

```
BLENDER_VERSION 5.2.0 LTS
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = True
RenderSettings.compositor_device = 'GPU'
RenderSettings.compositor_precision = 'AUTO'
```

Two things worth pinning down. `save_output` exists here and nowhere else — the same question was asked of 4.5, 5.0 and 5.1 and came back `MISSING` in all three, so the answer to question 2 below is a change in Blender rather than three researchers missing the same thing. And `compositor_device` starts at `'GPU'` in 5.2, where 4.5, 5.0 and 5.1 all start at `'CPU'`.

**Hand test 3 — whether Use Nodes still decides anything.** A 64×64 scene rendering flat green, `[0.4275, 0.7686, 0.3137, 1.0]`, was given a compositor of Render Layers into Invert into Group Output, with Compositing on. `scene.use_nodes` was set each way, read back, and the frame rendered:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
```

`[0.7882, 0.4588, 0.7882, 1.0]` is the inverted green. Setting the switch off did not take and the compositor ran either way.

**Hand test 4 — what makes the compositor run in a render started with no window.** A 64×64 Workbench scene with a cube and a camera, whose raw render is black, rendered five ways: a compositor of Render Layers into Invert into Group Output with Compositing on and then off; the Group Output node deleted; a plain red colour fed to the Group Output with no Render Layers node at all; the Group Output node muted; and no compositing node group assigned:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Black `[0.0, 0.0, 0.0, 1.0]` is the raw render, `[0.7686, 0.7686, 0.7686, 1.0]` is black inverted, `[0.8549, 0.2196, 0.1255, 1.0]` is the red. Case 2 answers what this answer records as not covered: with no active output node the compositor does not run and the raw render is what lands in the file. Case 3 shows a group with no Render Layers node still runs. Case 4 shows muting the Group Output node does not stop it counting as the active output.

**Hand test 5 — the compositor device, and the Enable Output node.**

```
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
Enable Output node inputs: ['Enable', 'Value'] outputs: ['Value']
RESULT Enable Output set False -> [0.0, 0.0, 0.0, 1.0]
```

With an Enable Output node between the Invert node and the Group Output and its Enable input turned off, the written frame was black rather than the scene's green camera view — the compositor ran and delivered nothing, rather than being skipped.

**Hand test 14 — the GPU compositor above the size the Limitations page names.** A 9000×400 flat-green scene with an Invert compositor, rendered with each device:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
```

The compositor produced its inverted green on both devices and the frame came out at the full 9000×400. This is on Apple's own graphics; another maker's card cannot be tested with the available equipment.

**Outside the interface. Now observed.** Every case above was a render started from a terminal with no window. The step marked inferred above — taking the command-line and background-job facts together to conclude the same gates apply headlessly — is now observed for the Compositing switch, the assigned node group and the node group's contents.

### Observed — window tests, 2026-08-03

These three are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.2's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are written out in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15, 17 and 21. What follows is what the Creator reported, in their own words.

**Hand test 15 — what the Post Processing panel holds.** The Properties editor's Output tab was opened and the panel called Post Processing opened with it. The Creator gave their 4.5 answer in full and then said 5.0, 5.1 and 5.2 are:

> Identical to 4.5 except the Compositing tooltip reads: "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene."

Their 4.5 answer, which the rest of this therefore stands on, was:

> Two checkboxes — Compositing and Sequencer, both on by default. One slider called "Dither" set to 1.00 by default, a float, adjustable from 0 to 2. … Sequencer tooltip: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

So the panel really does hold what the manual's picture and Cycles' own file say it holds — the two switches and the Dither slider, nothing else — and the Compositing switch is on by default, which is the API default quoted above. Its tooltip is that API sentence word for word, condition and all. The test also asked whether the panel's own header carries a checkbox beside the words "Post Processing". The Creator did not say, so that part is not answered.

**Hand test 17 — what the Compositor editor's header offers.** The Compositing tab along the top of the window was clicked, in a file where nothing had been set up. The Creator's report, given for 5.0, 5.1 and 5.2 together as identical across all three:

> No checkbox. Same four menus, but now there's a Node Tree Sub-Type dropdown to their left with two options, "Scene" (default) and "Sequencer". Center-top is the ordinary data-block selector plus a "+New" button (the standard pattern used throughout Blender's node editors). Nothing exists until you click +New, which creates a real data-block called "Compositor Nodes" with the same default setup as 4.5, except the final node is called Group Output instead of Composite.

"Same four menus" refers to the View, Select, Add and Node menus the Creator listed for 4.5. So there is no "Use Nodes" checkbox in this header, which fits the API entry quoted above calling that property unused. The first of the three conditions — a compositing node group attached to the scene — appears here as a data-block selector with a New button, and nothing is attached in a fresh file until New is clicked. The tree that New makes has a Group Output as its final node, which is the output the third condition looks for.

**Hand test 21 — how a second output node is marked.** In the Compositing editor a Render Layers node was wired to a Group Output node, a second Group Output node was added, and the Render Layers node wired to that one too. The Creator's report, given for 5.0, 5.1 and 5.2 together as identical:

> The last-selected node is the active one. The inactive one shows a warning above its header: a triangle with an exclamation mark, followed by "Unused Output".

So the "active Group Output node" of the third condition is picked by last selection, and the one that is not active carries a warning reading "Unused Output". The test also asked whether either node's own header carries a control for making it the live one. The Creator did not name one, so that part is not answered.

**Outside the interface.** These three settle nothing about it. Each is a look at what a Blender window shows, so none of them speaks to a render driven from outside.

**Operating system.** All three were done on macOS. All three were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Observed — the same tests on a rented Linux machine, 2026-08-06

Every test above was run again, unchanged, on a second computer: a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own. The Blender used was 5.2.0 LTS at `/opt/blender/blender-5.2.0-linux-x64/blender`, started with no window (`-b`) and with `--factory-startup`, except where the test says a window was opened. The scripts were the same files as on the Mac, copied across. The full set-up is in `Audits/Blender Research/The compositor/Hand tests — Linux.md`. **These are second results, not replacements for the ones above.**

One thing to know before reading the figures. Two of the reference colours come back one step of 255 lower in one channel on this machine than on the Mac: magenta reads `[0.7882, 0.4549, 0.7882, 1.0]` here against `[0.7882, 0.4588, 0.7882, 1.0]` there, and yellow reads `[0.7804, 0.7529, 0.4196, 1.0]` here against `[0.7804, 0.7529, 0.4235, 1.0]` there. Every other colour is identical to the last figure. Which colour arrived is the same on both machines in every test.

**Hand test 1 — what the switches are called, and what they start as.** What came back, word for word:

```
BLENDER_VERSION 5.2.0 LTS
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = True
RenderSettings.compositor_device = 'GPU'
RenderSettings.compositor_precision = 'AUTO'
```

Identical to the Mac's, line for line — including `save_output` existing and starting on, and `compositor_device` starting at `GPU`, which no earlier version does.

**Hand test 3 — whether Use Nodes still decides anything.** What came back, word for word:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4549, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back True -> written pixel [0.7882, 0.4549, 0.7882, 1.0]
```

The same as the Mac: the switch refuses to go off, and the compositor runs either way.

**Hand test 4 — what makes the compositor run in a render started with no window.** What came back, word for word:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Identical to the Mac's, figure for figure — including case 4, where muting the Group Output node does **not** stop the compositor's result reaching the file.

**Hand test 5 — the compositor device, and the Enable Output node.** What came back, word for word:

```
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0]
Enable Output node inputs: ['Enable', 'Value'] outputs: ['Value']
RESULT Enable Output set False -> [0.0, 0.0, 0.0, 1.0]
```

**Hand test 14 — the compositor above the size the documentation names.** The manual for this version names an 8k limit for the GPU compositor. What came back, word for word:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
```

The same as the Mac: 5.2 writes the composited picture at 9000 pixels wide on either device, on an NVIDIA card as on Apple's own graphics. That is above the size the documentation names.

**Hand test 15 on the Linux list — whether a compositor tree gives the same picture on the card as on the processor. This matters most in 5.2, because 5.2 is the version whose compositor device starts on the card.** A 320×240 Workbench scene holding a sphere, a cube and a cone under the studio light was given four compositor trees in turn — an Invert; a Blur of size 12; a Scale set to absolute 160×120; and a chain of Invert into a Blur of size 6 into an Alpha Over blending the blurred copy back over the original at half strength. Each was rendered twice, once on each device, and the two written files compared value by value, all 307,200 of them. The first three trees were run by one command and the fourth by a second command; what came back is quoted below as one block, the two commands run one after the other. Word for word: *(Linux citation check, 2026-08-06: the block was previously headed "word for word" with no sign that it stitches the output of two separate commands. Said plainly.)*

```
RESULT tree=invert device=CPU -> 320x240 first pixel [0.7686, 0.7686, 0.7686, 1.0]
RESULT tree=invert device=GPU -> 320x240 first pixel [0.7686, 0.7686, 0.7686, 1.0]
COMPARE tree=invert -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
RESULT tree=blur device=CPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
RESULT tree=blur device=GPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
COMPARE tree=blur -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
RESULT tree=scale device=CPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
RESULT tree=scale device=GPU -> 320x240 first pixel [0.0, 0.0, 0.0, 1.0]
COMPARE tree=scale -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
ALPHAOVER inputs=['Background', 'Foreground', 'Factor', 'Type', 'Straight Alpha'] outputs=['Image']
RESULT tree=invert+blur+alphaover device=CPU -> 320x240 first pixel [0.6627, 0.6627, 0.6627, 1.0]
RESULT tree=invert+blur+alphaover device=GPU -> 320x240 first pixel [0.6627, 0.6627, 0.6627, 1.0]
COMPARE tree=invert+blur+alphaover -> 307200 values compared, largest difference 0.000000, 0 values differ by more than one step of 255, identical=True
```

All four trees came back identical to the last figure on the two devices. This settles Linux and one NVIDIA card at 320×240; it says nothing about macOS, about Apple's own graphics, or about a card of another make.

**One error is recorded here because it happened before that run.** The fourth tree was first built around a Mix node named `CompositorNodeMixRGB`, and came back:

```
Traceback (most recent call last):
RuntimeError: Error: Node type CompositorNodeMixRGB undefined
```

A probe then asked which mix-like nodes exist and printed:

```
BLENDER_VERSION 5.2.0 LTS
bpy.types.CompositorNodeMixRGB = MISSING
bpy.types.CompositorNodeMix = MISSING
bpy.types.ShaderNodeMix = PRESENT
bpy.types.ShaderNodeMixRGB = PRESENT
bpy.types.CompositorNodeAlphaOver = PRESENT
```

The tree was rebuilt around an Alpha Over node, and that failed too, because the node's two picture inputs are named differently from 4.5's:

```
Traceback (most recent call last):
IndexError: list index out of range
ALPHAOVER inputs=['Background', 'Foreground', 'Factor', 'Type', 'Straight Alpha'] outputs=['Image']
```

The script was then changed to ask for whichever pair of inputs the version has, and it is that third version whose results are quoted above.

### Observed — the same window tests on the rented Linux machine, 2026-08-06

These are interface observations, like the three above. They were driven over SSH on the rented Linux machine, which has a display of its own, and photographed with Blender's own screenshot command. **No click or keystroke was used** — everything was done by asking Blender to change an area, pop a panel or open a menu from inside itself. The pictures are on that machine under `/root/smoozy/The compositor/work/shots/`, and what each shows is written out in words here, so nothing depends on the picture surviving.

**Hand test 15 — what the Post Processing panel holds.** The panel holds a row labelled **Pipeline** with two checkboxes, **Compositing** and **Sequencer**, both ticked; and below it a slider labelled **Dither** reading **1.00**. Nothing else. That is what the Creator reported on the Mac.

**And the part the Mac left unanswered is now answered.** The same editor was scrolled to the bottom and photographed. The Post Processing panel's own header reads `> Post Processing` with only the fold arrow beside it — **no checkbox**. The Stereoscopy header higher up carries a checkbox in the same picture, and the Output header carries one too, so a checkbox in a header shows up in these pictures where one exists.

**Hand test 17 — what the Compositor editor's header offers.** Left to right the header holds: the editor-type dropdown; a second dropdown reading **Scene**; the menus **View**, **Select**, **Add** and **Node**; then a node-group chooser with a small browse button, a **+** and a button reading **New**; then a pin. **There is no Use Nodes checkbox.** Below the header the editor showed a row of node-group preview thumbnails along the bottom of the area. That matches the Creator's report on the Mac.

**Hand test 21 — how a second output node is marked.** A Render Layers node was wired to two Group Output nodes. Both have the same grey header, and the second one carries a small strip directly above it holding a warning triangle and the words **Unused Output**. The first carries nothing.

**And the part the Mac left unanswered is now answered.** At twice magnification, **neither node's own header carries any control for making it the live one**. The header holds the fold arrow and the node's name, and nothing else.

**Outside the interface.** These three settle nothing about it, exactly as their Mac counterparts do not.

**Operating system.** These three were done on Linux and settle Linux. Together with the Mac results above them, macOS and Linux now both have an answer and the two agree. Windows stays untested.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Answer.** Yes. The Output panel has a checkbox in its header that turns off writing from that panel, and the manual spells out what happens instead:

> "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node.
>
> The automatic saving of renders can be toggled using the checkbox in the panel header.
>
> When enabled, Blender behaves in the standard way: still renders (F12) are not saved automatically, while animation renders (Ctrl-F12) are written to the configured output path.
>
> When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported.
>
> Playblast operations also require an active output method. Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."

The API names the property and its default:

> "save_output
>
> Write frames to disk for animation renders (default True)"

The 5.2 release notes state this as a change:

> "* Saving render output can now be disabled. (blender/blender@342f37566f)"

**Label: documented.**

**Not covered.** The manual's own picture of the Output panel does not show this checkbox — see below — so I cannot confirm from a picture where in the header it sits or how it is labelled. The cited Output manual and API pages do not settle what happens to a still render that is explicitly asked to write when the panel is off, beyond the general statement that Blender does not write from this panel. **Label: not in the sources.**

**A note on the picture, and its limit.** I opened `_images/render_output_properties_output_panel.png` (shown on the Output manual page). It shows a panel headed "Output" with a path field reading `/tmp/`, and checkboxes for Overwrite, Placeholders, File Extensions and Cache Result, then File Format PNG, Color BW/RGB/RGBA, Color Depth 8/16, and Compression 15%. There is no checkbox visible in the panel header, and no "Media Type", "Pixel Density" or "Color Management" rows that the page's text describes. So this picture shows fewer controls than the text describes. What is visible is visible; I am not inferring from it that the checkbox is absent in 5.2, because the text says plainly that it is there. Read 2026-07-30.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/rendering/ — read 2026-07-30

**Outside the interface.** The switch is a render setting, `scene.render.save_output`, so it is readable and writable from Python without a person clicking. **Label: documented.** The manual's description is written in terms of F12 and Ctrl-F12. The command line's `-o, --render-output <path>` sets the path, but the documented list contains no argument that switches writing off. The cited pages do not settle whether the reported error conditions stop a background render or only print a message. **Label for that limit: not in the sources.**

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.**

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 1 — that the switch is real, and new.** Blender was asked for the value of `save_output` on the render settings, in each of the four researched versions:

```
5.2.0 LTS: RenderSettings.save_output = True
5.1.2:     RenderSettings.save_output = MISSING
5.0.1:     RenderSettings.save_output = MISSING
4.5.12 LTS: RenderSettings.save_output = MISSING
```

The property exists only in 5.2, and starts on. That confirms the release-note line quoted above as a change in Blender, not as three other researchers missing something.

**Hand test 10 — what Save Output off does when the command line supplies a path.** A two-frame 64×64 file was saved with `render.save_output` set to False, an output path of its own, and a compositor holding a Render Layers node feeding both the Group Output and a File Output node aimed at a separate empty folder. It was then rendered as an animation from a terminal with an `-o` override, the exact command being `blender -b --factory-startup saveoff.blend -o .../cli_saveoff_ -F PNG -a`. What came back, word for word:

```
00:00.392  render           | Rendering frame 1
00:00.543  render           | Saved: '.../saveoff_fileoutput/saveoffBeauty0001.png'
00:00.543  render           | Rendering frame 2
00:00.550  render           | Saved: '.../saveoff_fileoutput/saveoffBeauty0002.png'
```

and the folder counts afterwards:

```
saveoff_main_*: 0 ; cli_saveoff_*: 0 ; saveoff_fileoutput: saveoffBeauty0001.png saveoffBeauty0002.png
```

Nothing was written from the Output panel — not to the path stored in the file and not to the path given on the command line — while the File Output node wrote both frames. So `-o` does not override the switch, and this answers what the answer above records as not covered. The same run in 4.5, 5.0 and 5.1, which have no such switch, wrote both the `cli_saveoff_` frames and the File Output frames.

**Hand test 10 — what happens with the File Path emptied.** A two-frame file with `render.filepath` set to the empty string, rendered as an animation from a terminal with the working folder set to the folder holding the file, by `blender -b --factory-startup emptypath.blend -a`:

```
00:00.382  blend            | Read blend: ".../out_write/5.2/emptypath.blend"
00:00.386  render           | Rendering animation (frames 1..2)
00:00.386  render           | Rendering frame 1
00:00.386  render           | Start rendering: Scene, ViewLayer
00:00.386  render           | Engine: Workbench
00:00.508  image            | ERROR Couldn't create directory for file 0001.png: No such file or directory
00:00.508  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
00:00.508  render           | Time: 00:00.12
```

No file was written and the run stopped after frame 1. That answers the other reading this answer flags: emptying the File Path field is a failure, not a supported way of switching writing off. The proper way is the Save Output switch above.

**Hand test 10 — a muted File Output node.** Two otherwise identical two-frame files, one with a File Output node aimed at an empty folder and muted, one with it unmuted, each rendered as an animation from a terminal:

```
muted_fileoutput  -> [end]
unmuted_fileoutput-> unmutedfoBeauty0001.png unmutedfoBeauty0002.png [end]
```

Muting the node stops it writing. That matters here, because with Save Output off the File Output nodes are what the manual says Blender falls back on.

**Outside the interface. Now observed.** Every case above was driven from a terminal with no window, so the switch is honoured in a headless render and the step marked inferred above is observed.

### Observed — window tests, 2026-08-03

This one is an **interface observation**, unlike the terminal tests above. The Creator ran it by hand in Blender 5.2's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-up is in `Audits/Blender Research/The compositor/Hand tests.md`, entry 16.

**Hand test 16 — what the Output panel holds, and whether its header has a checkbox.** The Properties editor's Output tab was opened and the panel called Output looked at, header first. The Creator answered in three parts: words for the Media Type control, a screenshot for the panel, saved as `5.2.png` in a folder called `Test 16` on their own computer, and one extra thing they offered without being asked.

Their words:

> 5.0, 5.1, 5.2: The Media Type dropdown has three items: Image; Multi-Layer EXR; Video.

> Bonus, not asked for but offered: in 5.2, "Copy Full Data Path" on the Output header checkbox gives `bpy.data.scenes["Scene"].render.save_output`.

What the picture shows, top to bottom:

- The panel header reads **Output**, with the ordinary collapse arrow at the far left, then **a ticked checkbox**, then the word "Output". The checkbox carries no words of its own.
- A file path field reading `/tmp/`, with a folder button at its right end.
- **Saving** — "File Extensions", ticked; "Cache Result", unticked.
- **Media Type** — "Image".
- **File Format** — "PNG (.png)".
- **Color** — three buttons, "BW", "RGB", "RGBA", with RGBA chosen.
- **Color Depth** — two buttons, "8" and "16", with 8 chosen.
- **Compression** — a slider reading "15%".
- **Image Sequence** — "Overwrite", ticked; "Placeholders", unticked.

Below the Output panel the picture also shows two further panels, "Color Management" and "Pixel Density".

**This closes the gap this answer records as not covered.** The checkbox the manual's text describes is real, it sits immediately to the left of the word "Output" in the panel header, and it starts ticked — which is the `save_output` default of True quoted above. The manual's own picture of this panel, which does not show it, is the part that is out of date. The data path the Creator copied off the checkbox names it as `render.save_output`, the same property hand test 1 and hand test 10 above exercised from a terminal, so the control in the window and the property in the file are one thing and not two.

One part of the test came back differently from how it was asked. The Creator was asked for the checkbox's tooltip and gave its full data path instead. So the tooltip's wording is still not recorded here; what the checkbox is has been settled by its data path rather than by its tooltip.

The same look in Blender 4.5, 5.0 and 5.1 shows no checkbox in this header at all, which is what those versions' findings say. 5.2 alone has it.

**Outside the interface.** This settles nothing about it. It is a look at what a Blender window shows. What the switch does to a render driven from outside was settled separately, by hand test 10 above.

**Operating system.** It was done on macOS. It was done again on the rented Linux machine, and the two agreed; the Linux result is in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.2.0 LTS at `/opt/blender/blender-5.2.0-linux-x64/blender`. **Second results, not replacements.** This is the question the Save Output switch belongs to, and it is the one place where 5.2 differs from every earlier version.

**Hand test 1 — whether an output-off switch exists.** `RenderSettings.save_output = True`. It exists in 5.2 and starts on. In 4.5, 5.0 and 5.1 the same probe printed `MISSING`. The same as on the Mac.

**Hand test 10 — whether the place frames are written can be switched off.** Four files were built and rendered from the command line, one of them with `save_output` set to `False` and an `-o` on the command line naming somewhere else to write. What came back, word for word:

```
BLENDER_VERSION 5.2.0 LTS
SAVED emptypath.blend with render.filepath = ''
SET render.save_output = False
SAVED saveoff.blend
SAVED mutedfo.blend with the File Output node muted
SAVED unmutedfo.blend
END_OF_BUILD
----- emptypath.blend -a
00:00.883  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
----- saveoff.blend -o cli_saveoff_ -F PNG -a
00:00.926  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/saveoff_fileoutput/saveoffBeauty0001.png'
00:00.931  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/saveoff_fileoutput/saveoffBeauty0002.png'
----- mutedfo.blend -a
00:00.925  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/muted_main_0001.png'
00:00.929  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/muted_main_0002.png'
----- unmutedfo.blend -a
00:00.989  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/unmuted_fileoutput/unmutedfoBeauty0001.png'
00:00.990  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/unmuted_main_0001.png'
00:00.995  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/unmuted_fileoutput/unmutedfoBeauty0002.png'
00:00.997  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.2/unmuted_main_0002.png'
----- listings
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
saveoffBeauty0001.png
saveoffBeauty0002.png
--- muted_fileoutput:
--- unmuted_fileoutput:
unmutedfoBeauty0001.png
unmutedfoBeauty0002.png
```

The same as on the Mac on every point. With `save_output` off, no `cli_saveoff_` file appears in the listing at all, even though the command line named a place with `-o` — so the switch beats the command-line override. The File Output node inside the compositor kept writing regardless. An emptied File Path is not a way to switch the writing off: the render fails with an error and writes nothing. And a muted File Output node writes nothing while the scene's own output still writes.

**Hand test 4, case 2 — removing the compositor's output point.** Written out in full under question 1: the raw render still reached the file.

### Observed — the same window test on the rented Linux machine, 2026-08-06

**Hand test 16 — whether the Output panel's header carries a checkbox.** The Properties editor was put on the Output tab, made to fill the window, scrolled so the panel and its header were both visible, and photographed with Blender's own screenshot command. The header reads `∨ ☑ Output` — **a ticked checkbox to the left of the word Output**. The panel holds, top to bottom: the file path field reading `/tmp/` with a folder button; **Saving** with **File Extensions** (ticked) and **Cache Result** (unticked); a **Media Type** dropdown reading **Image**; **File Format** reading **PNG (.png)**; **Color** as **BW / RGB / RGBA** with RGBA chosen; **Color Depth** as **8 / 16** with 8 chosen; **Compression** reading **15%**; **Image Sequence** with **Overwrite** (ticked) and **Placeholders** (unticked); and two closed sub-panels, **Color Management** and **Pixel Density**. That is what the Creator reported on the Mac, and the checkbox the manual describes is really there in the running program.

**What is still not answered** is what the tooltip on that checkbox says. Reading a tooltip needs the pointer held on the control, and on the rented machine the display is shared and a click does not reach Blender, so it could not be done there. The Creator's Mac record gives the control's full data path instead, `bpy.data.scenes["Scene"].render.save_output`, and hand test 1 above shows that property reading `True` in 5.2 and missing in the other three versions.

**Outside the interface.** This settles nothing about it.

**Operating system.** It was done on Linux and settles Linux. With the Mac result above it, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Answer.** Both, in different senses, and the sources are explicit about the split. The node tree itself is a data-block belonging to the file; the *assignment* of a node tree belongs to a scene.

The manual, on creating one:

> "This adds a compositing node tree data block to the scene, which can then be reused across different scenes or linked/appended from other blend-files."

Node Tree is listed as one of the file's data-block types in the data-blocks table, described as:

> "Groups of re-usable nodes.
> Used in the node editors."

and the general rule for data-blocks applies to it:

> "Data-blocks can be shared among other data-blocks."

The per-scene pointer is the scene property already quoted in answer 1, `compositing_node_group`, of type `NodeTree | None` — one scene, one assigned tree, and the type allows None.

So: a blend-file can hold many compositing node trees; each scene points at most one of them; and the same tree can be pointed at by more than one scene.

**Label: documented** for the node tree being a shareable file-level data-block and the assignment being a scene property. **Label: inferred** for the summary sentence "the same tree can be pointed at by more than one scene" — the step is: the manual says the tree "can then be reused across different scenes", and the scene property holds a reference to a `NodeTree` rather than owning it.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/compositing/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/files/data_blocks.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30

**Outside the interface.** This is file structure, not interface behaviour: the tree is a data-block in the file and the pointer is a scene property, both readable from Python. The command line's `-S, --scene <name>` ("Set the active scene <name> for rendering") selects which scene renders, and therefore which assigned tree is the one in play. **Label: inferred.** The step is the per-scene assignment described above.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.**

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 5 — two scenes pointing at one node group.** Two scenes were made, A rendering blue `[0.0745, 0.3333, 0.8196, 1.0]` and B rendering green `[0.4275, 0.7686, 0.3137, 1.0]`. A was given a compositing node group holding a Render Layers node whose Scene was set explicitly to A, feeding an Invert node into the Group Output. B was then pointed at that same node group, and each scene rendered in turn with Compositing on:

```
RESULT sharing one tree between two scenes -> True
RESULT shared tree, rendering scene A -> [0.7804, 0.7529, 0.4235, 1.0]
RESULT shared tree, rendering scene B -> [0.7804, 0.7529, 0.4235, 1.0]
```

`[0.7804, 0.7529, 0.4235, 1.0]` is blue inverted. Both scenes wrote the same picture: A's render, put through the shared tree. The step marked inferred above — that the same tree can be pointed at by more than one scene — is now observed, and a further thing is settled with it: the Render Layers node inside a shared tree follows the scene named on the node, not the scene being rendered.

**Hand test 12 — whether choosing a scene from outside picks that scene's compositor.** One file, `scenes.blend`, holding two scenes: A rendering blue with a compositor that inverts it to yellow, B rendering green with a compositor that inverts it to magenta, both with Compositing on. Two renders from a terminal, `blender -b --factory-startup scenes.blend -S A -o .../scene_A_ -F PNG -f 1` and the same with `-S B`, and the written frames read back:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [0.7804, 0.7529, 0.4235, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
```

Yellow for A, magenta for B. So `-S` picks the scene and that scene's own assigned tree is the one that runs — the step marked inferred above.

**Outside the interface. Now observed.** Both tests ran from a terminal with no window.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.2's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 17 and 20.

**Hand test 17 — what the Compositor editor's header offers.** Quoted in full under question 1. The Creator's report, given for 5.0, 5.1 and 5.2 together:

> No checkbox. Same four menus, but now there's a Node Tree Sub-Type dropdown to their left with two options, "Scene" (default) and "Sequencer". Center-top is the ordinary data-block selector plus a "+New" button (the standard pattern used throughout Blender's node editors). Nothing exists until you click +New, which creates a real data-block called "Compositor Nodes" with the same default setup as 4.5, except the final node is called Group Output instead of Composite.

That is the split this answer describes, seen in the window. The header's middle is a data-block selector with a New button — the ordinary way a shared data-block is picked in Blender — and clicking New makes one with a name of its own, "Compositor Nodes". The scene points at whatever is in that selector, and nothing is pointed at in a fresh file. The dropdown to the left offering "Scene" and "Sequencer" shows that a compositing node tree is used in two places, not only by scenes.

**Hand test 20 — what the Render menu offers.** The Video Editing tab along the top of the window was clicked, then the Render menu in the very top bar was opened, in a file where nothing had been set up. The Creator reported the menu is identical in all four versions and answered with one screenshot, saved as `all versions.png` in a folder called `Test 20` on their own computer. What the picture shows, in order:

- Render Image — F12
- Render Animation — ⌘ F12
- Render Audio…
- View Render — F11
- View Animation — ⌘ F11
- Lock Interface — a checkbox, unticked

with dividing lines after "Render Animation", after "Render Audio…" and after "View Animation". **No entry in it names a Sequencer Scene.** Note what that does and does not show: the manual sentence quoted under question 7 makes those entries conditional — "When the active scene is different from the Sequencer Scene in the current workspace, the 'Render' menu will include separate options to render the Sequencer Scene" — and this look was taken in a fresh file where the two were not made different. So it records that the entries are not always in the menu; it does not show that they never are.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.2.0 LTS. **Second results, not replacements.**

**Hand test 5 — whether one tree can serve two scenes, and whose render it pulls.** What came back, word for word:

```
RESULT sharing one tree between two scenes -> True
RESULT shared tree, rendering scene A -> [0.7804, 0.7529, 0.4196, 1.0]
RESULT shared tree, rendering scene B -> [0.7804, 0.7529, 0.4196, 1.0]
```

The same as on the Mac: one node group can be given to two scenes, and when it is, both scenes write the picture the shared tree's Render Layers node names — scene A's blue render inverted to yellow — rather than their own.

**Hand test 12 — whether the command line's scene choice picks that scene's compositor.** What came back, word for word:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [0.7804, 0.7529, 0.4196, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0]
```

A comes back yellow and B magenta, so naming a scene with `-S` picks that scene's own compositor. The same as on the Mac.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 17 — what the Compositor editor's header offers.** Written out in full under question 1: a scene dropdown, the four menus, a node-group chooser with a **New** button, and no Use Nodes checkbox.

**Hand test 20 — what the Render menu offers.** The top bar's Render menu was opened and photographed. It is headed **Render** and holds, in order: **Render Image** (F12), **Render Animation** (Ctrl F12), **Render Audio…**, a separator, **View Render** (F11), **View Animation** (Ctrl F11), a separator, and **Lock Interface** as an unticked checkbox. **No entry names a Sequencer Scene.** One thing to say plainly about the picture: a tooltip reading "Render active scene." had appeared over the item under the pointer and covers part of the Render Animation row; the shortcut column is still legible behind it. The menu is the same as the Creator reported on the Mac.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were both run with no window.

**Operating system.** All were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**This question is ambiguous** — there are two different ways one scene takes in another's rendered result, and the sources answer them differently. I have answered both. The reading I take as meant is the first, because the question is on a compositor sheet.

### 4a. Through a Render Layers node pointed at another scene — its composited result does not arrive; whether its compositor runs is not established.

The Render Layers node has a Scene property:

> "Scene
>
> The scene for which to render a view layer."

and the API confirms the node holds a scene reference:

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render"
>
> "scene
>
> Type:
>
> Scene | None"

The same manual page then states, in so many words, that this route does not give you the other scene's composited result:

> "To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node."

**Label: documented** that what arrives is the other scene's raw render output and not its compositing output. **Label: not in the sources** for whether the second scene's compositor runs. The quoted workaround establishes which result arrives, but it does not distinguish a compositor that was skipped from one that ran and whose result was discarded.

### 4b. Through a Scene strip in the Video Sequencer — the sources say something, but it is ambiguous.

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

and, on the Post Processing page:

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

**Label: documented** that scene strips are rendered as part of the pipeline, and that compositing being enabled makes the scene strip carry the Compositor's output. **Label: not in the sources** for which scene's Compositing switch "also enabled" refers to. The panel described is the rendering scene's Post Processing panel, but the compositor whose output a scene strip would carry could be the strip's own scene's. I am not choosing between those readings; the sentence supports either.

**Not covered.** Neither route is described in terms of what actually executes. Nothing in the recorded search says whether the second scene's node tree is evaluated, what happens if the second scene has Compositing switched off, or whether a Render Layers node pointing at another scene triggers a render of that scene during a file render (as opposed to the manual's button "The button next to the dropdown re-renders it immediately", which is an interface action). **Label: not in the sources.**

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeRLayers.html — read 2026-07-30

**Where I looked, for the parts that came back empty.** I searched the whole 5.2 manual for "Scene strip" (33 hits, all read or ruled out by page), "composit" across the whole `video_editing/` and `editors/video_sequencer/` trees (all hits were about the Compositor strip and Compositor modifier, which are a different feature), "render pipeline" and "compositing pipeline" (2 hits, both already quoted), and "viewport compositing"/"viewport compositor" (4 hits, none about cross-scene behaviour). Nothing in those results describes whether one scene's render triggers another scene's compositor.

**Outside the interface. Now observed.** The tests below were run from a terminal with no window and gave the results described, so cross-scene evaluation in background mode is no longer unestablished.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.**

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 7 — a Render Layers node pointing at another scene.** Two 64×64 scenes. B renders a flat green slab; its compositing node group sends its Render Layers node through an Invert node to its Group Output — so B's own composite is magenta — and also straight into a File Output node aimed at an empty folder, so any work B's compositor does leaves a file behind. A renders blue on its own, and its whole compositing node group is one Render Layers node whose Scene is set to B, wired to A's Group Output. A was rendered twice, with B's Compositing switch on and then off, and B's File Output folder checked each time. Three controls were rendered as well:

```
RESULT B.use_compositing=True -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=True -> files in B's File Output folder: []
RESULT B.use_compositing=False -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=False -> files in B's File Output folder: []
CONTROL A alone, compositing off -> [0.0745, 0.3333, 0.8196, 1.0]
CONTROL B alone, compositing off -> [0.4275, 0.7686, 0.3137, 1.0]
CONTROL B alone, compositing on  -> [0.7882, 0.4588, 0.7882, 1.0]
```

The controls give the three colours to read the result by. What A wrote was B's raw render, both times — which is what the documented Hint says. B's File Output folder was empty both times, though rendering B directly fills it, and B's own Compositing switch made no difference to either signal. That is the test this answer's 4a asks for: it keeps the result that arrived separate from whether B's compositor did any work, and on this evidence it did none.

**Hand test 8 — a Scene strip pointing at another scene.** The same two scenes, except A takes B in through a Video Sequencer Scene strip with A's Sequencer switch on, and A's own compositor is a plain pass-through. Rendered four times, once for each combination of the two scenes' Compositing switches, with B's File Output folder cleared before each run:

```
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['UnsavedBeauty.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['UnsavedBeauty.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

This settles the question 4b would not guess at. "If Compositing is also enabled" means the **strip's own scene's** switch: the written frame follows B's and is unaffected by A's, and B's File Output node writes exactly when B's switch is on, which shows B's compositor genuinely runs in that case.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Both tests were run again, unchanged, on the rented Linux machine, with Blender 5.2.0 LTS started with no window and with `--factory-startup`. **Second results, not replacements.**

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

A got B's raw green render both times, and B's File Output folder stayed empty both times. The same as on the Mac.

**Hand test 8 — a Scene strip pointing at another scene.** What came back, word for word:

```
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0] ; files in B's File Output folder: ['UnsavedBeauty.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0] ; files in B's File Output folder: ['UnsavedBeauty.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

The frame follows B's switch and not A's, and B's File Output node writes exactly when B's switch is on. The same as on the Mac.

**Outside the interface.** Both were run from a terminal with no window, as on the Mac.

**Operating system.** Both were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

**This question has three parts and I have answered each separately.** I read "told apart from anything else there" as: can the render be distinguished from other images inside the node tree.

### What it arrives as

Through the Render Layers node, as a set of output sockets:

> "Renders a View Layer and reads its Passes into the compositing node graph."
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

The node's picture (`_images/node-types_CompositorNodeRLayers.webp`, on the Render Layers manual page, read 2026-07-30) shows a node headed "Render Layers" with two outputs on the right labelled "Image" and "Alpha", and two selectors below them: a data-block field reading "Scene" and a dropdown reading "ViewLayer". That is what is visible; the pass sockets described in the text are not visible in this picture, presumably because none are enabled in the shot — that last clause is my assumption, not the picture's content. **Label for the picture: documented.**

Once inside, it is one of only three kinds of data the compositor knows:

> "Three types of data exist, all of which are stored in half precision formats:
>
> Float
> ...
> Vector
> ...
> Color
>
> A 4D vector storing the Red, Green, Blue, and Alpha of the color."

and it carries a position and size in the compositing space, not just pixels:

> "images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations."

**Label: documented.**

### Can it be told apart

By its *source node*, yes: the Render Layers node is a distinct node type (`CompositorNodeRLayers`, "Input render passes from a scene render") carrying `scene` and view-layer properties, so inspecting the node tree tells you which node the render entered through and from which scene.

By the *data on a link*, no marker is described. The compositor's data types are Float, Vector and Color, with nothing recording where a value came from, and the socket inspection panel — the one facility the manual gives for looking at what a socket holds — reports only:

> "Resolution – Displays the image resolution. If the image has a display window, both the data window and display window are shown instead.
>
> Transform – Displays the image transform. If no transformation has been applied, Identity is shown. Otherwise, the translation, rotation, and scale are displayed.
>
> Sampling – Displays the interpolation and sampling modes used by the image.
>
> Precision – Indicates whether the image is stored using Full or Half precision."

**Label: documented** that the Render Layers node is a distinct, identifiable node type and that socket inspection reports the four listed properties. **Label: inferred** for "the data itself carries no marker of being a render" and for the inspection panel having no origin field — the step is: the documented data model lists three types with no provenance field, and the documented inspection panel lists four properties, none of which is provenance. A reader has to supply the assumption that these lists are complete.

### Can it be followed forward

Through the structure of the node tree, yes, as an inference from the documented links. The API describes a link as:

> "class bpy.types.NodeLink(bpy_struct)
>
> Link between nodes in a node tree"

and each socket exposes:

> "links
>
> List of node links from or to this socket."

A node that only passes data along unchanged is the Reroute node:

> "A single-socket organization tool that supports one input and multiple outputs"

which the manual restates as:

> "Reroute looks and behaves much like a socket on other nodes in that it supports one input connection while allowing multiple output connections."

That covers both halves of the question: pass-along and split-into-more-than-one are the same node.

The Switch node also passes an input through untouched:

> "Output
>
> One of the two inputs without any modifications."

**Label: documented** for links between nodes, for sockets listing their links, for Reroute's one-in-many-out behaviour, and for Switch passing an input through unmodified. **Label: inferred** for "therefore a render can be traced from the Render Layers node forward to wherever it ends up" — the step is: starting at the Render Layers node and walking the linked sockets follows the structure of the graph. Three points complicate the walk, and I name them rather than glossing over them:

- Muted links: "A muted link acts as though it's no longer there" and "Muted links are kept in place but ignored in evaluation" — so a link that exists in the file may carry nothing.
- Muting on a reroute propagates: "When muting links on the input side of a reroute node, the links on its output side will be muted too."
- A Switch outputs "one of the two inputs", so which branch carries the render depends on its condition rather than on the links alone. **Label: inferred.** The 5.2 release notes add that some conditions are known before evaluation: "The compositor now supports lazy evaluation of Switch nodes whose condition is known statically, that is, not connected or connected directly to a Group Input socket."

**Not covered.** The cited node-system pages do not describe any way to ask, at render time, "which socket currently holds the render". They do not describe tracing across a group boundary beyond saying that Group Input and Group Output carry values in and out, or whether a render's identity survives a node that combines it with another image. **Label: not in the sources.**

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/types/layout/reroute.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/switch.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.NodeLink.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.NodeSocket.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.NodeTree.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.NodeReroute.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeRLayers.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/compositor/ — read 2026-07-30

**Outside the interface.** Node trees, nodes, sockets and links are data readable from Python, so the structural trace described above does not require the interface. **Label: inferred.** One caution the API gives is about cost, not availability: `NodeSocket.links` carries the note "Takes O(len(nodetree.links)) time." Socket inspection, by contrast, is described as an interface panel and the manual says "Socket values are only available after the node tree has been evaluated. Nodes that do not contribute to the final output are generally not evaluated and therefore have no inspection data available." The cited pages do not settle whether inspection data exists outside the interface. **Label for that limit: not in the sources.**

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The test below was run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. **The exception is hand test 19, what socket inspection shows.** That was attempted on the Linux machine and could not be settled there, so that reading rests on the Mac alone — see the two sections further down. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.** *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. Hand test 19 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)*

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 11 — telling a render apart, and following it forward, with no window open.** One 64×64 scene was given a compositing node group holding a Render Layers node named `TheRender`, an Image node named `TheImage`, a Reroute named `TheReroute`, a Blur node named `MutedBlur` that was muted, a Viewer named `TheViewer`, and the Group Output. The render's Image socket goes into the Reroute; the Reroute feeds the muted Blur, which feeds the Group Output; the Reroute also feeds the Viewer through a link that was itself muted. What came back, word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
RenderLayers output sockets = ['Image', 'Alpha']
--- every link in the tree
LINK TheRender.Image -> TheReroute.Input  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> MutedBlur.Image  is_muted=False is_valid=True is_hidden=False
LINK MutedBlur.Image -> Group Output.Image  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> TheViewer.Image  is_muted=True is_valid=True is_hidden=False
--- walking forward from the Render Layers Image socket
WALK TheRender.Image -> TheReroute.Input (muted link=False, muted node=False)
WALK   TheReroute.Output -> MutedBlur.Image (muted link=False, muted node=True)
WALK     MutedBlur.Image -> Group Output.Image (muted link=False, muted node=False)
WALK   TheReroute.Output -> TheViewer.Image (muted link=True, muted node=False)
--- what the muted node exposes
MutedBlur.mute = True ; internal_links = [('Image', 'Image')]
--- is_active_output on output nodes
ACTIVE Group Output.is_active_output = True
```

Three things this settles. The two node types carry different identifiers, so a program can tell the render's entry point from an image file's — the `layer` property reports `'ViewLayer'` rather than the `'PLACEHOLDER'` the reference page prints. The walk ran to completion, reaching both destinations through the Reroute, with the muted link and the muted node each flagged on the way, so the structural trace this answer marks inferred does work. And `is_active_output` reads `True` on the Group Output node, so which output node is live is a property of the node itself.

**Hand test 2 — what the Split node offers.** The other reading of "split it into more than one" is flagged above. Adding the node and printing its sockets gives:

```
ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']
```

It has one output socket, so something can be followed forward past it after all.

**Outside the interface. Now observed.** The whole of hand test 11 ran in a session with no window open, so the structural trace is available there. Socket inspection was not exercised there — it is a panel display and needs somebody looking at a window, and it is written up in the next section.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal test above. The Creator ran them by hand in Blender 5.2's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 19 and 21.

**Hand test 19 — what socket inspection shows, and whether it names where an image came from.** A small tree was built in the Compositing editor — a Render Layers node and an Image node with an image loaded, each feeding its own Viewer node — one frame was rendered and the render window closed, and then the link out of each node was hovered. The Creator's report for 5.2:

> For an ordinary Image node it always shows "Image", no period. For the Render Layers node it's richer — screenshots in the `Test 19` folder show what appears, and the file names mark whether they were taken before or after rendering. This detailed info only appears if you click the eye icon in the node's upper-left corner, and that holds both before and after rendering.

So the two links do not show the same thing. Hovering the Image node's link gives the socket's name and nothing more — the same one word the Creator saw in 5.0 and 5.1. Hovering the Render Layers node's link gives the fuller display, and it only appears once the eye icon in that node's upper-left corner has been clicked.

The two screenshots are saved in a folder called `Test 19` on the Creator's own computer, named for whether they were taken before or after rendering. **After rendering**, the display reads, line for line:

```
Image
Resolution: 1920x1080
Transformation: Identity
Sampling:
• Interpolation: Bilinear
• Extension: Clip
Precision: Full
Type: Image
```

**Before rendering**, the same hover gives a different display entirely, line for line:

```
Image
Value: 0 0 0 0 (Linear)
Type: Float Color
[a colour swatch, black on the left half, a transparency checkerboard on the right]
Display RGB:  0.000  0.000  0.000
HSV:          0.000  0.000  0.000
Alpha:        0.000
Hex:          #00000000
```

This settles what the answer above could only take from the manual's list. After a render, the four things the manual names are all there — Resolution, Transform (shown as "Transformation", reading "Identity"), Sampling (broken into Interpolation and Extension) and Precision — plus one line the manual's list does not mention, "Type: Image". **Not one line in either display names where the image came from.** So the step this answer marks inferred — that the inspection panel has no origin field — is now observed, for both the before and after states.

The before-rendering display also bears on the manual sentence quoted above, "Socket values are only available after the node tree has been evaluated." Before a render, the socket did show something, and what it showed was a single black transparent colour rather than an image. That is what appeared; this record does not say what it means.

**Hand test 21 — how a second output node is marked.** Quoted in full under question 1. With two Group Output nodes in one tree, the last-selected one is the active one, and the inactive one carries a warning above its header — a triangle with an exclamation mark, then the words "Unused Output". That is a second way of telling which output is the live one, beside the `is_active_output` reading in hand test 11 above, and this one is visible on the node itself.

**Outside the interface.** These two settle nothing about it, and one of them narrows it. Both are looks at what a Blender window shows, so neither speaks to a render driven from outside — and socket inspection in particular was reached only by clicking an icon on a node, so nothing here shows it is available without a window.

**Operating system.** Both were done on macOS. Test 21 was done again on the rented Linux machine and agreed; test 19 was attempted there and could not be settled. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.2.0 LTS. **Second results, not replacements.**

**Hand test 2 — which compositor nodes exist, and what the Split node's sockets are.** The printed block is long; the lines that carry the answer came back word for word as:

```
BLENDER_VERSION 5.2.0 LTS
bpy.types.CompositorNodeComposite = MISSING
TREE_TYPE CompositorNodeTree
TREE_STARTING_NODES []
ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']
ADD CompositorNodeSwitch -> inputs=['Switch', 'Off', 'On'] outputs=['Image']
ADD CompositorNodeRLayers -> inputs=[] outputs=['Image', 'Alpha']
ADD CompositorNodeOutputFile -> inputs=[''] outputs=[]
ADD CompositorNodeViewer -> inputs=['Image'] outputs=[]
ADD CompositorNodeComposite -> ERROR Error: Node type CompositorNodeComposite undefined
ADD NodeGroupOutput -> inputs=[''] outputs=[]
```

Identical to the Mac's, socket for socket, including the Split node producing one output image.

**Hand test 11 — telling a render apart and following it forward.** What came back, word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
RenderLayers output sockets = ['Image', 'Alpha']
--- every link in the tree
LINK TheRender.Image -> TheReroute.Input  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> MutedBlur.Image  is_muted=False is_valid=True is_hidden=False
LINK MutedBlur.Image -> Group Output.Image  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> TheViewer.Image  is_muted=True is_valid=True is_hidden=False
--- walking forward from the Render Layers Image socket
WALK TheRender.Image -> TheReroute.Input (muted link=False, muted node=False)
WALK   TheReroute.Output -> MutedBlur.Image (muted link=False, muted node=True)
WALK     MutedBlur.Image -> Group Output.Image (muted link=False, muted node=False)
WALK   TheReroute.Output -> TheViewer.Image (muted link=True, muted node=False)
--- what the muted node exposes
MutedBlur.mute = True ; internal_links = [('Image', 'Image')]
--- is_active_output on output nodes
ACTIVE Group Output.is_active_output = True
```

Identical to the Mac's, line for line, including the view-layer property reporting a real name rather than a placeholder.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 21 — how a second output node is marked.** Written out in full under question 1: both Group Output nodes have a grey header and the second carries an **Unused Output** warning above it, and neither node's header carries any control for making it the live one. That agrees with the Creator's Mac report and closes the part they left unanswered.

**Hand test 19 — what socket inspection shows. This could not be settled here, and no answer is written in.** A Blender window was opened on the rented Linux machine with the same tree the Mac test used, one frame was rendered, and the pointer was put over the link out of the Render Layers node, over that node's own Image output socket, and over the link out of the Image node. It was tried four ways: with Blender moving its own pointer to coordinates it worked out from the node positions; with the pointer moved from outside in several small steps and held; with dwells of 2.5 seconds and of 8 seconds; and with the picture taken by Blender itself on a signal, so the pointer was checked to be in place first. No display appeared in any of them, and none appeared over a control that certainly has a tooltip either. **That is a fault in the run, not a finding about Blender.** The machine's display is shared with other topics being tested at the same moment: one check printed `AT 438,227 -> x:573 y:68`, meaning something else moved the pointer between putting it there and reading it back. The 5.2 case also needs the eye icon on the Render Layers node clicked first, and on that machine a click does not appear to reach Blender at all. So the socket-inspection reading for 5.2 still rests on the Mac alone. What to run when the machine is free is written out in `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

**Outside the interface.** These settle nothing about it.

**Operating system.** Test 21 now has an answer from both machines and they agree. Test 19 has an answer from the Mac only; Linux and Windows are both open for it.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Answer.** Yes — at least one thing plainly, and one more by a short step. I read "size" as the pixel dimensions of the written frames.

**The render region, when cropping is on.** The Format panel has both:

> "Render Region
>
> Renders just a portion of the view instead of the entire frame. See the Render Region documentation to see how to define the size of the render region.
>
> Crop to Render Region
>
> Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."

The API gives the two switches and the four numbers that define the region:

> "use_border
>
> Render a user-defined render region, within the frame size (default False)"
>
> "use_crop_to_border
>
> Crop the rendered frame to the defined render region size (default False)"

together with `border_min_x`, `border_min_y`, `border_max_x`, `border_max_y`, all floats. The region itself is set in the viewport and is part of the scene:

> "If you define one while in Camera View, it will apply not just to the viewport, but also to the final render. If you want to temporarily disable this region rather than clearing it entirely, you can do so in the Output tab of the Properties editor."

and:

> "Render regions only apply to the viewport when using Cycles, not when using EEVEE. However, they always affect the final render."

**Label: documented** that Crop to Render Region crops *the rendered image* to the region's size, and that the region reaches the final render; **inferred** for the step from "the rendered image is cropped" to "the file written to disk is that size" — every quoted sentence says "rendered image" or "rendered frame", never "written" or "saved". *(Citation check, 2026-08-03: previously labelled documented outright. The hand test below measures the written file and settles it.)*

**Stereo 3D output formats.** With multiple views, the format used to save them is a further variable. The API:

> "use_squeezed_frame
>
> Combine both views in a squeezed image (default False)"

with `display_mode` offering SIDEBYSIDE ("Render views for left and right eyes side-by-side") and TOPBOTTOM ("Render views for left and right eyes one above another"). The manual adds:

> "For the Stereo 3D formats the only lossless options are Top-Bottom and Side-by-Side without the Squeezed Frame option."

and:

> "The Top-Bottom compressed will discard half of your vertical resolution data."

**Label: inferred.** The step: the manual says the squeezed (compressed) top-bottom form throws away half the vertical resolution, and that the unsqueezed form is lossless. For an unsqueezed side-by-side or top-bottom pair to be lossless, the written frame must hold both full-size views, so it must be larger in one axis than a single view. The sources nowhere state the resulting pixel dimensions outright.

**What the sources establish about nearby settings.**

- For the main compositor output, the manual says: "The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size." The Scale node's "Render Size" option is described as "Image dimensions set in the Render panel." **Label: documented.** The crop case remains open below.
- For pixel aspect, the source says only: "Older televisions may have non-square pixels, so this can be used to control the shape of the pixels along the respective axis." It does not state what happens to written pixel dimensions.
- For metadata stamping, the source says only: "Burn into Image — Add metadata as text to the render." It does not state whether the canvas dimensions change.
- For EEVEE's Overscan, the source says only: "Percentage of the render size to add to the internal render buffer." It does not state that the written dimensions change.
- Codecs can impose limits: "Some codecs impose output size restrictions. For example, H.264 requires both width and height to be divisible by 2." The source does not say that Blender resizes to meet them.

**Label for the four limits above: not in the sources.** Each is limited to the cited page and quote rather than treated as proof that the setting cannot affect written dimensions.

**Not covered.** The biggest gap: the sources never say what pixel size a **File Output node** writes at. Its manual page describes directory, file name, media type, formats and paths, and says nothing about dimensions; its API page lists no width or height. **Label: inferred** that an input to a File Output node need not be the scene render size, because the compositor works in an "infinite compositing space" where each image carries its own domain. Nothing in the sources says what the node does about that. Likewise, the sources do not say what size the compositor's own output becomes when Crop to Render Region is on; the compositor page says "the scene render size" without qualification. **Label for both gaps: not in the sources.**

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/3dview/navigate/regions.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/transform/scale.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/film.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/files/media/video_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.Stereo3dFormat.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30

**Where I looked, to be able to say the rest is silent.** I searched the whole 5.2 manual for "output size", "output resolution", "final resolution", "image dimensions", "render size", "resolution", "squeezed", "crop", "render region", and "border", and read every hit in `render/`, `compositing/`, `files/media/` and `editors/` that was not obviously about something else (texture resolution, shadow resolution, remesh, proxy preview sizes, safe areas). In the API I listed every property of `RenderSettings` and of `ImageFormatSettings` by name; `ImageFormatSettings` has no width or height property at all. In Cycles' files I searched for "resolution" and "border" across the Python files; the only size arithmetic there is a warning in the interface code, quoted next.

**Supporting finding from Cycles' own files (observed).** Cycles' own interface code computes the effective render size the same way the question's premise describes:

> ```
>             render_scale = scene.render.resolution_percentage / 100.0
>             render_size_x = int(scene.render.resolution_x * render_scale)
>             render_size_y = int(scene.render.resolution_y * render_scale)
> ```

File: `Blender Sources/cycles_5.2/ui.py`, lines 399–401, inside the path-guiding panel's draw code. Read 2026-07-30. This is what the file computes for a tiling warning. That the written file has those dimensions is a step beyond the file and I do not take it.

**Outside the interface.** Every item above is scene or format data in the file. The render region is *defined* in the viewport ("View ‣ View Regions ‣ Render Region…", Ctrl-B), but the resulting numbers are stored as `border_min_x` and related properties and are settable from Python. The documented command-line arguments include no way to change resolution, percentage, or the render region — `-o`, `-F` and `-x` cover path, format and extension only. **Label: inferred** that a run from outside follows the file's sizing data unless a script changes it; the step is that the settings are stored in the file and no command-line override was found.

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. **The exception is the second half of hand test 18** — ticking Render Region and looking at Crop to Render Region again. Ticking a box needs the pointer, which could not be relied on on the Linux machine, so that half rests on the Mac alone. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. The second half of hand test 18 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)* One adjacent fact does differ by system: the command-line page lists formats that may be missing entirely on some systems — "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). That affects whether a format is available, not whether the sizing rules differ. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.**

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 6 — three things that might decide the written size.** A 400×200 Workbench scene with a cube and a camera, rendered three ways. First, with a Scale node set to Absolute 120×90 between the Render Layers node and a File Output node, while the unscaled render went to the scene's own output. Second, with the render region set to the middle half of the frame — 0.25 to 0.75 on both axes — and Crop to Render Region off and then on. Third, with stereoscopy on and both views written into one file, Side-by-Side and Top-Bottom, squeezed and unsqueezed:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> UnsavedBeauty.png is 122x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

**This answers the biggest gap in this answer.** The File Output node wrote at the size of the image plugged into it — 122×90 — while the scene's own output stayed at 400×200 in the same render. So the File Output node does **not** follow the same rule as the Viewer node. *(Citation check, 2026-08-03: this paragraph previously said the finding "answers it against the step taken above" and that "the sentence marked inferred above ('that the File Output node follows the same rule as the Viewer node') is wrong." No such sentence appears above in this document. What the "Not covered" paragraph above actually marks inferred is the opposite — "that an input to a File Output node need not be the scene render size" — which the test confirms rather than contradicts. The mis-description is struck. The sentence being contradicted is in this version's second-pass document, whose answer 6 does take that step and says it would be wrong if the node wrote at its input's size.)*

Note the exact number. The Scale node was set to 120×90 and the file came out 122 pixels wide. The same test in 4.5 and 5.0 gave exactly 120×90; 5.1 and 5.2 both gave 122×90. The two extra pixels were not asked for and the test does not explain them.

The stereo lines turn the step marked inferred above into a measurement: unsqueezed Side-by-Side doubles the width, unsqueezed Top-Bottom doubles the height, both squeezed forms keep the scene's own dimensions. The crop lines settle the second open item — what the compositor writes when Crop to Render Region is on — at exactly the region's own size, and show that a render region **without** the crop leaves the size untouched.

**Hand test 13 — what the output-path template reports under a crop.** The same 400×200 scene, render region cropped to the middle half, output path set to `{resolution_x}x{resolution_y}_`:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The template reports the scene's own Resolution values, not the size actually written.

**Hand test 9 — whether the sequencer route changes the written size.** A 400×200 scene rendered six ways across the Sequencer and Compositing switches, with and without a Scene strip. All six wrote 400×200; the lines are quoted under question 7.

**Hand test 14 — the documented 8k limit.** A 9000×400 flat-green scene with an Invert compositor, rendered with each device:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
```

`[0.7882, 0.4588, 0.7882, 1.0]` is the inverted green, so the compositor ran and the written size was the full 9000×400 on both devices. The limit this test was aimed at is on the compositor's Limitations page: "Resolution — Images with resolution larger than 8k are not supported and are cropped/scaled." (manual — https://docs.blender.org/manual/en/5.2/compositing/limitations.html, read 2026-07-30). On this machine that limit did not reach what got written. It is one graphics card of one make, and another maker's cannot be tested with the available equipment. *(Citation check, 2026-08-03: this paragraph referred to "the documented 8k limit" and "the documented 'cropped/scaled' limit" without quoting or addressing it anywhere in this document. The quote and address are now given. The passage exists only in 5.2's Limitations page; 4.5's has no Resolution section.)*

**Outside the interface. Now observed.** All of the above ran from a terminal with no window. The step marked inferred above is now observed for the render region, the crop, the stereo formats and the File Output node.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.2's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 16 and 18.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor's Output tab was opened at the Format panel, the two checkboxes looked at with Render Region unticked, then Render Region ticked and them looked at again. The Creator's report, which they gave for all four versions at once:

> All four versions: Render Region is clickable, off by default; while off, Crop to Render Region is greyed out. Ticking Render Region makes Crop to Render Region available, but it stays off by default.

So the two settings quoted above sit one under the other on that panel, both start off — matching the API defaults of False quoted above — and the second is unavailable until the first is on.

**Hand test 16 — what the Output panel holds.** Quoted in full under question 2. The one thing in it that bears on size is the Media Type control, which the Creator reported as a dropdown with three items — Image, Multi-Layer EXR, Video. Nothing in the panel offers a second way of deciding the written pixel size.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Test 16 was done again on the rented Linux machine and agreed; only the first half of test 18 could be done there. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.2.0 LTS. **Second results, not replacements.**

**Hand test 6 — what decides the pixel size of what gets written.** What came back, word for word:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> UnsavedBeauty.png is 122x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

Identical to the Mac's, figure for figure — including the two-pixel oddity, where a Scale node set to an absolute 120×90 produced a file 122 pixels wide. Both machines report 122, so it is not a property of one of them.

**Hand test 13 — the path template under a crop.** What came back, word for word:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The same as on the Mac: the template reports the scene's own numbers on a file that is half those in each direction.

**Hand test 14 — the compositor above the size the documentation names.** Written out in full under question 1: 5.2 writes the composited picture at 9000×400 on either device, on an NVIDIA card as on Apple's own graphics.

**Hand test 15 on the Linux list — the card against the processor.** Written out in full under question 1. All four compositor trees, rendered on both devices and compared value by value at 320×240, matched to the last figure.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 16 — the Output panel.** Written out in full under question 2: a ticked checkbox in the panel's own header, and a **Media Type** dropdown reading **Image**.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor was put on the Output tab, made to fill the window, and photographed. The Format panel holds two checkboxes one above the other, labelled exactly **Render Region** and **Crop to Render Region**. Both start unticked, and **Crop to Render Region is drawn greyed out** — its label and its box both dimmed — while Render Region is unticked. **Only the first half of the Mac's test was run here.** The Mac's version also ticked Render Region and looked again, and found Crop to Render Region became available; ticking a box needs a click, which does not reach Blender on that machine, so the second half stays as the Mac has it.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux, except the second half of test 18, which stays macOS only. With the Mac results above them, macOS and Linux now both have an answer for the rest and they agree. Windows stays untested.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Answer.** The written result can come from the Video Sequence Editor instead of the active camera while Compositing is off and Sequencer is on. The two settings are separate properties; **label: inferred** that they can be set independently. The sources do not establish whether "bypass" means that the compositor node tree does not execute. The manual describes the Sequencer switch:

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

and the API gives its meaning and its condition:

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

Note the two source wordings differ in emphasis: the manual says the sequencer output replaces the camera view, and the API says the render (and composited) result is *processed through* the sequencer pipeline. Both support that when strips exist and the switch is on, the sequencer's result is used. **Label: documented.**

**What that changes about what gets written, as documented:**

1. **The picture comes from the edit, not the camera.** Per the quote above — "instead of the view from the 3D scene's active camera."

2. **Only the combined image survives; individual passes do not.** For the scene strips that a sequencer edit is usually made of:

> "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

3. **The metadata written can be drawn from the strips.** The Metadata panel offers "Metadata Input — Where to grab metadata from" with "Sequencer Strips: Use metadata from the strips in the Sequencer", and the include list holds "Strip Name — Includes the name of the foreground sequence strip."

4. **If saving from the Output panel is off, the Video Sequencer is active and no alternative output is available, an error is raised.** That is the full condition stated by the source.

5. **In 5.2 the edit lives in its own scene, and Blender offers separate options to render that scene.** "The video sequencer uses a separate scene data-block that contains the edit. This is the Sequencer Scene." and "The render and output settings of the Sequencer Scene are the same as for regular scenes. They can be found in the Output tab in the Properties editor." and "When the active scene is different from the Sequencer Scene in the current workspace, the "Render" menu will include separate options to render the Sequencer Scene (and not the active Scene)."

**Label: documented** for all five points as worded above. **Label: not in the sources** for whether the compositor node tree still executes while its switch is off. The Post Processing manual and RenderSettings API pages cited below describe which result is used, but not execution; no wider execution search was recorded.

**Not covered.** The cited Post Processing, Scene strip and RenderSettings pages do not say what is written when the Sequencer switch is on but the scene has no strips, whether switching Compositing off stops the scene's compositor from running, what size or colour depth the sequencer's output is written at, or whose Compositing switch the Scene strip sentence means. The API's phrase "if sequencer strips exist" suggests a condition but does not name the other outcome. **Label: not in the sources.** The switch-owner question is covered by the search record in answer 4. No wider searches for the empty-sequencer, size, colour-depth or current-scene execution questions were recorded.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/video_editing/sequencer_scene.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30

**Outside the interface.** The switch is `scene.render.use_sequencer`, ordinary scene data, so the choice can be stored in the file. One thing is documented specifically for driving from outside: the render operator takes a flag for it —

> "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene (optional)"

(https://docs.blender.org/api/5.2/bpy.ops.render.html, read 2026-07-30). **Label: documented** for the operator flag. The command line's `-S, --scene <name>` sets the active scene for rendering. **Label: inferred** that a run from outside can name the Sequencer Scene directly; the step is that the Sequencer Scene is itself a scene. The cited operator and command-line pages do not settle whether the reported errors stop a background run. **Label for that limit: not in the sources.**

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it — and no operating-system search of the sources was recorded either. If the output is a video, the documented format-availability caveat quoted in answer 6 applies — FFMPEG is among the formats "not available on all systems" — but that does not establish whether the sequencer behaviour differs. **Label for Windows: not in the sources, and not tried — no Windows machine has run it.**

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS started from a terminal with no window and with `--factory-startup`.

**Hand test 9 — what the sequencer route writes.** A 400×200 scene was built whose camera view is blue, `[0.0745, 0.3333, 0.8196, 1.0]`, and whose compositor replaces the picture with flat red, `[0.8549, 0.2196, 0.1255, 1.0]`. A second scene rendering green, `[0.4275, 0.7686, 0.3137, 1.0]`, was added as a Scene strip for half the runs. Six renders across the Sequencer and Compositing switches:

```
RESULT no strips, sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer OFF, compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT strip,     sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer OFF, compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
```

This answers two things recorded above as not covered. With Sequencer on and no strips, a file is written and it holds the scene's ordinary picture — the camera view with Compositing off, the compositor's result with it on. And the written size stayed at 400×200 in all six, so the sequencer route does not change it.

Rows four and five also bear on what "bypass" means here. With a strip present, the strip's picture replaces the scene's own compositor result as well as the camera view — the compositor's flat red never reaches the file, whether the Compositing switch is on or off.

**Hand test 8 — whose Compositing switch a Scene strip follows.** Quoted in full under question 4. The four runs show the picture follows the strip's own scene's switch, not the rendering scene's, which is the ambiguity this answer inherits from question 4.

**Hand test 13 — what survives into a multi-layer EXR.** A scene with the Depth and Mist passes turned on, rendered to multi-layer OpenEXR twice: once plainly, once with a Scene strip supplying the picture and the Sequencer switch on. The channel names in the two files:

```
exr_seqFalse.exr: ViewLayer.Combined.A ViewLayer.Combined.B ViewLayer.Combined.G ViewLayer.Combined.R ViewLayer.CombinedpixelAspectRatio ViewLayer.Depth.Z ViewLayer.DepthpixelAspectRatio
exr_seqTrue.exr:  Composite.Combined.A Composite.Combined.B Composite.Combined.G Composite.Combined.R Composite.CombinedpixelAspectRatio
```

Without the sequencer the file holds a view layer with its Combined and Depth passes. With the sequencer supplying the picture it holds one layer called Composite with only the combined image; the Depth pass is gone. A multi-layer EXR was written either way.

**Outside the interface. Now observed.** Every run above was started from a terminal with no window. Both switches take effect that way, and the step marked inferred above is now observed. How a headless run selects a Sequencer Scene was not tested and stays open.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.2's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15 and 20.

**Hand test 15 — what the Post Processing panel holds.** Quoted in full under question 1. The Creator saw two checkboxes, Compositing and Sequencer, both on by default, and one slider called "Dither"; the Sequencer tooltip reads:

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

The step marked inferred above — that the two settings can be set independently — rested on their being separate properties. The Creator saw them as two separate checkboxes in one panel, both on by default, which is what the API defaults say. And the tooltip the running program shows is the API's sentence word for word, not the manual's "instead of the view from the 3D scene's active camera" — so where the two source wordings differ in emphasis, as noted above, the API's is the one put in front of a user. That records which wording the program shows; hand test 9 above is what bears on which describes the behaviour.

**Hand test 20 — what the Render menu offers.** Quoted in full under question 3. The Creator opened the Video Editing tab and then the Render menu in a fresh file, reported the menu identical in all four versions, and supplied one screenshot. It holds Render Image (F12), Render Animation (⌘ F12), Render Audio…, View Render (F11), View Animation (⌘ F11) and a Lock Interface checkbox. **No entry in it names a Sequencer Scene.**

Read that against point 5 above rather than instead of it. The manual makes those entries conditional — "When the active scene is different from the Sequencer Scene in the current workspace, the 'Render' menu will include separate options to render the Sequencer Scene" — and this look was taken in a fresh file where the two were not made different. So the entries are not always in the menu; whether they appear once the two scenes differ was not tested and stays open.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.2.0 LTS. **Second results, not replacements.**

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
RESULT multi-layer EXR, sequencer=False -> file exists: True size 2667
RESULT multi-layer EXR, sequencer=True -> file exists: True size 2110
--- channels in exr_seqFalse.exr
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
ViewLayer.CombinedpixelAspectRatio
ViewLayer.Depth.Z
ViewLayer.DepthpixelAspectRatio
--- channels in exr_seqTrue.exr
Composite.Combined.A
Composite.Combined.B
Composite.Combined.G
Composite.Combined.R
Composite.CombinedpixelAspectRatio
```

The same channel names as on the Mac: going through the sequencer collapses the file to one `Composite` layer and loses the depth pass.

**Hand test 4 — the Compositing switch on its own.** Written out in full under question 1.

**Hand test 8 — whose Compositing switch a Scene strip follows.** Written out in full under question 4.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 15 — what the Post Processing panel holds.** Written out in full under question 1: two checkboxes, Compositing and Sequencer, both ticked, and a Dither slider reading 1.00, and nothing else — and no checkbox in the panel's own header.

**Hand test 20 — what the Render menu offers.** Written out in full under question 3: Render Image, Render Animation, Render Audio…, View Render, View Animation and Lock Interface, and no entry naming a Sequencer Scene.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

**Every item on this list has been run, and nothing is left on it.** The results are written into the answers above with the machine's own output or the Creator's own words quoted, and the set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

Items 1, 2, 3, 4 and 6 below were run from a terminal. In short: a File Output node writes at its input's size, not the scene's; Crop to Render Region writes at exactly the region's size; a Render Layers node pointing at another scene brings back that scene's raw render and that scene's compositor does no work at all; a Scene strip follows the strip's own scene's Compositing switch; and unsqueezed stereo doubles one axis.

Item 5 was run by the Creator, looking at the window, and is settled under question 2: the checkbox is real, it sits immediately to the left of the word "Output" in the panel header, it carries no words of its own, and it starts ticked. Its full data path is `bpy.data.scenes["Scene"].render.save_output`. The Creator gave that path instead of the tooltip, so the tooltip's wording is still not recorded.

Items 7 to 12 were also run by the Creator, and are settled under the questions they answer — entries 19, 15, 17, 18, 20 and 21 of the hand-test list, in that order. Two small parts of them came back unanswered on the Mac, and **both have since been answered on a second machine**: the Post Processing panel's own header carries no checkbox, and neither Group Output node's header carries a control for making it the live one. One thing a fresh-file look cannot show is still open on both machines, and is written in beside its finding: whether the Render menu grows its Sequencer Scene entries once the Sequencer Scene differs from the active scene.

**A second machine, 2026-08-06.** Every test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are written in beside the Mac's, never merged with them. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`. Every test agreed with the Mac except one, which could not be settled there at all: what socket inspection shows (item 7, entry 19). One new test was added that only the second machine could carry, and it matters most in this version, because 5.2 is the one whose compositor device starts on the graphics card: whether a compositor tree gives the same picture on the card as on the processor. It does, at 320×240, for the four trees tried.

**Still cannot be run with the available equipment.** Two things stay untested. **Windows** — no Windows machine has run it, so every "operating system" line above covers macOS and Linux and leaves Windows open, and that is why the compositor's documented driver-reset limit on Windows, which the manual heads "TDR on Windows", stays untested. **A graphics card of another make** — the two machines have Apple's own graphics and one NVIDIA card between them, so nothing here speaks for an AMD or Intel card. Three smaller things need the rented machine to itself, because its display is shared and a click does not appear to reach Blender on it: what socket inspection shows, what the tooltip on the Output header checkbox says, and what Crop to Render Region looks like once Render Region is ticked.

The original list follows, with the settled items kept so a reader can match them to the results above.

1. **What a File Output node writes when its input is not the render size.** Open a new file. In Output Properties set Resolution to 1920×1080 at 100%. In the Compositor, add Render Layers, Group Output, a Scale node set to Absolute with X 640 and Y 360, and a File Output node. Connect Render Layers → Group Output, and branch Render Layers → Scale → File Output. Set the File Output node's directory to an empty folder. Render one frame (F12). Look at the pixel dimensions of the file that appears in that folder. Report the exact dimensions. (Settles the main gap in answer 6.)

2. **What size the compositor writes when Crop to Render Region is on.** Same file. In Camera View press Ctrl-B and drag a small box over roughly the middle quarter of the frame. In Output Properties ‣ Format, tick Render Region and tick Crop to Render Region. With a Render Layers → Group Output tree and Compositing enabled, render one frame and save it. Report the saved image's pixel dimensions, and then repeat with Compositing switched off and report whether the dimensions change. (Settles the second gap in answer 6.)

3. **Whether a Render Layers node aimed at a second scene causes that scene's compositor to run.** Make two scenes. In Scene B, build a compositor tree with Render Layers feeding both an Invert Color → Group Output path and a File Output node aimed at an empty folder. Turn Scene B's Compositing switch on. In Scene A, add a Render Layers node with its Scene set to B, wired to Scene A's Group Output. Clear the folder, render Scene A, and report two things: whether Scene A's result is inverted, and exactly which files appeared in Scene B's File Output folder. The first observation shows which result arrived; the second can distinguish a compositor that ran from one that was skipped. (Settles answer 4a.)

4. **Whose Compositing switch the Scene strip sentence means, and whether the source scene's compositor runs when its result is not used.** Keep Scene B's compositor and File Output node from test 3. In a sequencer scene, add a Scene strip referencing B. Render the sequencer scene four times, once for each combination of the sequencer scene's Compositing switch on/off and Scene B's Compositing switch on/off. Before each render, clear Scene B's File Output folder. For each combination, report whether the written frame shows B's compositor effect and exactly which files appeared in that folder. (Settles the ambiguity in answer 4b and records execution separately from the chosen result.)

5. **Where the Output panel's off switch is.** Open Output Properties and look at the header of the Output panel. Report whether a checkbox is there, its exact label, and whether it is ticked. The manual text says the switch exists, but its own picture does not show it. (Settles the picture gap in answer 2.)

6. **The written dimensions of an unsqueezed stereo pair.** In Output Properties enable Stereoscopy, set the Views Format for the saved image to Stereo 3D with Display Mode Side-by-Side and Squeezed Frame off, at a scene resolution of 1920×1080. Render and save one frame. Report the saved file's pixel dimensions, then repeat with Squeezed Frame on. (Turns the inference in answer 6 into a measurement.)

### Needed a Blender window — run by the Creator, and settled above

7. **What socket inspection shows, and whether it names where an image came from.** With a node tree containing both a Render Layers node and an Image node, hover over the link out of each and report every line the inspection display shows. This is new in 5.2 and settles whether anything beyond Resolution, Transform, Sampling and Precision is displayed. (Hand tests 19.)

8. **What the Post Processing panel actually contains**, control by control, and whether its header carries a checkbox. Everything the answers above say about that panel comes from a screenshot in the manual. (Hand tests 15.)

9. **What the Compositor editor's header offers**, control by control. (Hand tests 17.)

10. **Whether Crop to Render Region is greyed out until Render Region is ticked.** (Hand tests 18.)

11. **What the Render menu offers**, and in particular whether it names a Sequencer Scene. (Hand tests 20.)

12. **How a second Group Output node is marked as the unused one**, and what the "Unused Output" warning says word for word. (Hand tests 21.)

### Cannot be run with the available equipment

Nearly every "operating system" line above is now settled for macOS and for Linux — the exceptions are the three named at the end of this paragraph, which the shared display could not reach and which rest on the Mac alone. Neither machine is a Windows one, so Windows stays untested. *(Linux citation check, 2026-08-06: this sentence previously said "every", which the rest of its own paragraph contradicts. Narrowed.)* That is why the compositor's documented driver-reset limit on Windows stays untested. The two machines have Apple's own graphics and one NVIDIA card between them, so no card of any other make can be reached: the 9000-pixel result in question 6 speaks for those two cards and no others. Three smaller things need the rented Linux machine to itself, because its display is shared with other work and a click does not appear to reach Blender on it: what socket inspection shows, what the tooltip on the Output header checkbox says, and what Crop to Render Region looks like once Render Region is ticked.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 5.2

# The compositor — Blender 5.0

**Sources consulted, and when.** Blender 5.0 manual (plain-text snapshot plus two of its screenshots), Blender 5.0 Python API documentation, Blender 5.0 release notes, and the Cycles add-on files supplied in `Blender Sources/cycles_5.0`. All read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project. I used no helper agents, so this declaration covers all the reading behind these findings.

---

## 1. What decides whether a file's compositor runs at all?

**Answer.** Three things have to line up, and the sources name all three.

**(a) The Compositing switch in the scene's Post Processing properties must be on.**

> "To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline
> option must be enabled in the output's Post Processing properties."

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node."

**(b) A compositing node group must be assigned to the scene.** The switch itself says so:

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
>
> Type:
>
> boolean, default True"

The assignment slot is:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

**(c) The node group must contain an active Group Output node, or failing that an active Viewer node.**

> "The compositor searches for the active Group Output node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether.
> Consequently, note that adding a Viewer node will have no effect on the viewport render if there
> is a Group Output node, since the priority is given to Group Output nodes."

**What no longer decides it.** The old "Use Nodes" switch is gone and is not a condition any more:

> "use_nodes
>
> Enable the compositing node group.
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

and the release notes state it plainly:

> "\"Use Nodes\" in the node editor was removed (blender/blender@d88d4cc8ce)"

> "- `scene.use_nodes` is deprecated and will be removed in 6.0
>   Currently it always returns `True` and setting it has no effect.
>   (blender/blender!143578)."

**LABEL: documented** for (a), (b) and (c) taken separately, and for the removal of "Use Nodes"; **inferred** for the opening claim that all three "have to line up" and that the sources name all three — each condition is stated on its own page and no source presents them as one complete checklist. *(Citation check, 2026-08-03: the conjunction and the completeness claim were previously covered by the documented label. They are a step past the quoted words, as this topic's 5.1 first-pass findings already record for the same question.)*

**Supporting observed finding.** The Cycles code for the Post Processing panel draws `use_compositing` and `use_sequencer` under the "Pipeline" heading, followed by the Dither control:

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

LABEL for this piece: **observed** — file `Blender Sources/cycles_5.0/ui.py`, lines 1112–1128.

**Supporting documented finding from a picture.** The manual's Post Processing screenshot shows a panel headed "Post Processing" with two ticked checkboxes labelled "Compositing" and "Sequencer", and a "Dither" slider reading 1.000. Nothing else is in the panel. Picture: `_images/render_output_properties_post-processing_panel.png`, on the Post Processing page. LABEL for this piece: **documented**.

**Not covered.** The sources do not say what happens when a node group is assigned but is empty, when the Group Output node is muted, when the assigned node group is a missing or broken link, or in what order the three conditions are tested. They also do not say whether a Group Output whose first input is not a Color socket counts as "found" (the release notes only say its first input "needs to be a Color socket").

**Outside the interface.** The sources do not say directly. What they do say: all three conditions are properties stored in the file (`render.use_compositing`, `scene.compositing_node_group`, and the contents of the node group), and the documented command-line render options are only `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, `-F`, `-x` — none of which touches compositing. Where a render setting behaves differently outside the interface, this documentation says so explicitly; `use_single_layer` is the one that does: "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." No such note appears on `use_compositing`. Concluding from that silence that the three conditions apply identically to a command-line render is a step past the words, so: **inferred**.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and the conditions behave there as described. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. The sources still record one operating-system-specific compositor limit, which is about speed and driver resets rather than about whether the compositor runs: "TDR on Windows — Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." (Compositing ‣ Limitations.)

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, started with no window (`-b`) and with the Creator's own add-ons and preferences kept out of it (`--factory-startup`). Full set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

**Hand test 1 — what the switches are called, and what they start as.** Blender was started on its own empty startup file and asked to print each named property's value, or `MISSING` where it does not exist:

```
BLENDER_VERSION 5.0.1
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = MISSING
RenderSettings.compositor_device = 'CPU'
RenderSettings.compositor_precision = 'AUTO'
```

`Scene.node_tree` is gone and `compositing_node_group` has taken its place, starting empty — which is (b) above. `use_nodes` reads back True on a brand-new file, and Blender printed a deprecation warning when it was touched at all: `DeprecationWarning: 'Scene.use_nodes' is expected to be removed in Blender 6.0`.

**Hand test 3 — whether Use Nodes still decides anything.** A 64×64 scene rendering flat green, `[0.4275, 0.7686, 0.3137, 1.0]`, was given a compositor of Render Layers into Invert into Group Output, with Compositing on. `scene.use_nodes` was set each way, read back, and the frame rendered:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
```

`[0.7882, 0.4588, 0.7882, 1.0]` is the inverted green. Setting the switch off did not take, and the compositor ran either way. That turns the documented statement that the switch is inert into an observed one.

**Hand test 4 — what makes the compositor run in a render started with no window.** A 64×64 Workbench scene with a cube and a camera, whose raw render is black, was rendered five ways: a compositor of Render Layers into Invert into Group Output with the Compositing switch on and then off; the Group Output node deleted; a plain red colour fed to the Group Output with no Render Layers node at all; the Group Output node muted; and no compositing node group assigned:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Black `[0.0, 0.0, 0.0, 1.0]` is the raw render, `[0.7686, 0.7686, 0.7686, 1.0]` is black inverted, `[0.8549, 0.2196, 0.1255, 1.0]` is the red. This answers three of the things listed above as not covered. With no output node the raw render is written. With a node group assigned but no Render Layers node in it, the compositor still runs and its result is what lands. And **muting the Group Output node does not stop it counting as the active output** — the compositor's result was still written.

**Hand test 2 — a newly made node group starts empty.** Creating a compositing node tree with `bpy.data.node_groups.new` and assigning it to the scene gave:

```
TREE_TYPE CompositorNodeTree
TREE_STARTING_NODES []
```

So an assigned but empty group is a state that exists, and hand test 4 case 2 shows what a render does with one.

**Hand test 2 — the Composite node is gone.** Blender was asked whether the node type exists, and then asked to add one:

```
bpy.types.CompositorNodeComposite = MISSING
ADD CompositorNodeComposite -> ERROR Error: Node type CompositorNodeComposite undefined
```

**Hand test 5 — the compositor device, and the Enable Output node.**

```
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
Enable Output node inputs: ['Enable', 'Value'] outputs: ['Value']
RESULT Enable Output set False -> [0.0, 0.0, 0.0, 1.0]
```

The node type is `NodeEnableOutput`, not a compositor-specific name. With an Enable Output node between the Invert node and the Group Output and its Enable input turned off, the written frame was black rather than the scene's green camera view — so the compositor ran and delivered nothing, rather than being skipped.

**Hand test 14 — a very large frame.** A 9000×400 flat-green scene with an Invert compositor:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
```

**Outside the interface. Now observed.** Every case above was a render started from a terminal with no window. The step marked inferred above — concluding from the absence of an interface-only note that the conditions apply identically to a command-line render — is now observed for the Compositing switch, the assigned node group and the node group's contents.

### Observed — window tests, 2026-08-03

These three are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.0's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are written out in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15, 17 and 21. What follows is what the Creator reported, in their own words.

**Hand test 15 — what the Post Processing panel holds.** The Properties editor's Output tab was opened and the panel called Post Processing opened with it. The Creator gave their 4.5 answer in full and then said 5.0, 5.1 and 5.2 are:

> Identical to 4.5 except the Compositing tooltip reads: "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene."

Their 4.5 answer, which the rest of this therefore stands on, was:

> Two checkboxes — Compositing and Sequencer, both on by default. One slider called "Dither" set to 1.00 by default, a float, adjustable from 0 to 2. … Sequencer tooltip: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

So the panel holds what the manual's screenshot shows, the Compositing switch is on by default, and its tooltip is the same sentence as the API description of `use_compositing` quoted above, word for word — including the condition "if a compositing node group is assigned to the scene", which is (b) of this answer. The test also asked whether the panel's own header carries a checkbox beside the words "Post Processing". The Creator did not say, so that part is not answered.

**Hand test 17 — what the Compositor editor's header offers.** The Compositing tab along the top of the window was clicked, in a file where nothing had been set up. The Creator's report, which they gave for 5.0, 5.1 and 5.2 together as identical across all three:

> No checkbox. Same four menus, but now there's a Node Tree Sub-Type dropdown to their left with two options, "Scene" (default) and "Sequencer". Center-top is the ordinary data-block selector plus a "+New" button (the standard pattern used throughout Blender's node editors). Nothing exists until you click +New, which creates a real data-block called "Compositor Nodes" with the same default setup as 4.5, except the final node is called Group Output instead of Composite.

"Same four menus" refers to the View, Select, Add and Node menus the Creator listed for 4.5. So there is no "Use Nodes" checkbox in this header, which is what the release-note line quoted above says was removed, and the assignment slot described under (b) appears here as a data-block selector with a New button. Nothing is assigned in a fresh file until New is clicked.

**Hand test 21 — how a second output node is marked.** In the Compositing editor a Render Layers node was wired to a Group Output node, a second Group Output node was added, and the Render Layers node wired to that one too. The Creator's report, given for 5.0, 5.1 and 5.2 together as identical:

> The last-selected node is the active one. The inactive one shows a warning above its header: a triangle with an exclamation mark, followed by "Unused Output".

So which of two output nodes is the live one is shown by a warning on the other, reading "Unused Output". The test also asked whether either node's own header carries a control for making it the live one. The Creator did not name one, so that part is not answered.

**Outside the interface.** These three settle nothing about it. Each is a look at what a Blender window shows, so none of them speaks to a render driven from outside.

**Operating system.** All three were done on macOS. All three were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Observed — the same tests on a rented Linux machine, 2026-08-06

Every test above was run again, unchanged, on a second computer: a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own. The Blender used was 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, started with no window (`-b`) and with `--factory-startup`, except where the test says a window was opened. The scripts were the same files as on the Mac, copied across. The full set-up is in `Audits/Blender Research/The compositor/Hand tests — Linux.md`. **These are second results, not replacements for the ones above.**

One thing to know before reading the figures. Two of the reference colours come back one step of 255 lower in one channel on this machine than on the Mac: magenta reads `[0.7882, 0.4549, 0.7882, 1.0]` here against `[0.7882, 0.4588, 0.7882, 1.0]` there, and yellow reads `[0.7804, 0.7529, 0.4196, 1.0]` here against `[0.7804, 0.7529, 0.4235, 1.0]` there. Every other colour is identical to the last figure. Which colour arrived is the same on both machines in every test.

**Hand test 1 — what the switches are called, and what they start as.** What came back, word for word:

```
BLENDER_VERSION 5.0.1
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
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

**Hand test 14 — the compositor above the size 5.2's documentation names.** What came back, word for word:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
```

The same as the Mac: 5.0 writes the composited picture at 9000 pixels wide on either device.

**Hand test 15 on the Linux list — whether a compositor tree gives the same picture on the card as on the processor.** This test has no Mac counterpart. A 320×240 Workbench scene holding a sphere, a cube and a cone under the studio light was given four compositor trees in turn — an Invert; a Blur of size 12; a Scale set to absolute 160×120; and a chain of Invert into a Blur of size 6 into an Alpha Over blending the blurred copy back over the original at half strength. Each was rendered twice, once with the Compositor Device set to CPU and once to GPU, and the two written files compared value by value, all 307,200 of them. The first three trees were run by one command and the fourth by a second command; what came back is quoted below as one block, the two commands run one after the other. Word for word: *(Linux citation check, 2026-08-06: the block was previously headed "word for word" with no sign that it stitches the output of two separate commands. Said plainly.)*

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

**One error is recorded here because it happened before the run above.** The fourth tree was first built around a Mix node named `CompositorNodeMixRGB`, which worked in 4.5 and in 5.0 came back:

```
Traceback (most recent call last):
RuntimeError: Error: Node type CompositorNodeMixRGB undefined
```

It was rebuilt around an Alpha Over node, and that failed too, because the node's two picture inputs are named differently from 4.5's:

```
Traceback (most recent call last):
IndexError: list index out of range
ALPHAOVER inputs=['Background', 'Foreground', 'Factor', 'Type', 'Straight Alpha'] outputs=['Image']
```

The script was then changed to ask for whichever pair of inputs the version has, and it is that third version whose results are quoted above.

### Observed — the same window tests on the rented Linux machine, 2026-08-06

These are interface observations, like the three above. They were driven over SSH on the rented Linux machine, which has a display of its own, and photographed with Blender's own screenshot command. The pictures are on that machine under `/root/smoozy/The compositor/work/shots/`, and what each shows is written out in words here, so nothing depends on the picture surviving.

**Hand test 15 — what the Post Processing panel holds.** The panel holds a row labelled **Pipeline** with two checkboxes, **Compositing** and **Sequencer**, both ticked; and below it a slider labelled **Dither** reading **1.00**. Nothing else. That is what the Creator reported on the Mac.

**And the part the Mac left unanswered is now answered.** The same editor was scrolled to the bottom and photographed. The Post Processing panel's own header reads `> Post Processing` with only the fold arrow beside it — **no checkbox**. The Stereoscopy header higher up carries a checkbox in the same picture, so a checkbox in a header shows up in these pictures where one exists.

**Hand test 17 — what the Compositor editor's header offers.** Left to right the header holds: the editor-type dropdown; a second dropdown reading **Scene**; the menus **View**, **Select**, **Add** and **Node**; then a node-group chooser with a small browse button, a **+** and a button reading **New**; then a pin. **There is no Use Nodes checkbox.** On the right of the same strip: a greyed-out **Backdrop** button and three small toggles. That matches the Creator's report on the Mac.

**Hand test 21 — how a second output node is marked.** A Render Layers node was wired to two Group Output nodes. Both have the same grey header, and the second one carries a small strip directly above it holding a warning triangle and the words **Unused Output**. The first carries nothing.

**And the part the Mac left unanswered is now answered.** At twice magnification, **neither node's own header carries any control for making it the live one**. The header holds the fold arrow and the node's name, and nothing else.

**Outside the interface.** These three settle nothing about it, exactly as their Mac counterparts do not.

**Operating system.** These three were done on Linux and settle Linux. Together with the Mac results above them, macOS and Linux now both have an answer and the two agree. Windows stays untested.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/limitations.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30
- Cycles file `Blender Sources/cycles_5.0/ui.py`, lines 1112–1128 — read 2026-07-30

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Answer.** No such switch is described anywhere in these sources. The output location is a plain text field with no on/off beside it. What the sources *do* settle is a narrower thing: a single-frame render does not write to that location unless it is told to, while an animation render always does.

**LABEL: not in the sources** — for an on/off switch on the output location.
**LABEL: documented** — for the still-versus-animation difference below.

**What is documented about the field itself.**

> "File Path
>
> Choose the location to save rendered frames."

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers
>
> Type:
>
> string, default \"\", (never None, blend relative // prefix supported, Supports template expressions)"

There is no companion boolean. I read the whole property list of `RenderSettings`; the booleans near the output are `use_file_extension`, `use_overwrite`, `use_placeholder`, `use_render_cache`, `use_stamp*`, `use_border`, `use_crop_to_border`, `use_compositing`, `use_sequencer`, `use_multiview`, `use_single_layer` — none of them disables writing. The media choice has no "none" entry either:

> "media_type
>
> The type of media to save
>
> Type:
>
> enum in ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'], default 'IMAGE'"

**Animation renders always write.**

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

**Single-frame renders do not write unless told to.**

> "The Render button renders a single frame or image.
> Save your image using Save Image."

and the render operator carries the flag that controls it:

> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

So the closest thing to "switched off" that the sources describe is: render a still with writing not requested. For animation there is no documented off switch.

**One related documented behaviour.** A broken path stops the write rather than silently skipping it:

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

**Not covered.** The sources do not say what happens if the File Path is left empty, whether an empty path suppresses writing or falls back to somewhere, or whether a scene that is only rendered indirectly (as a Render Layers source or a Scene strip) writes anything to its own output path. They also say nothing about turning off writing for an animation.

**Where I looked, so that "not in the sources" is checkable.** I searched the whole manual, the whole API documentation and the whole release-notes set for: "output path", "render output", "disable output", "output disabled", "switch off", "turned off", "not saved", "without saving", "no file is written", "nothing is written", "is not written", "disable saving", "suppress", "dry run", "null output", "without writing", "do not write", "don't write", "skip writing", "media_type", "write_still". I read in full: the Output properties page, the Render Output introduction, the Rendering Animations page, the Post Processing page, the File Paths (path templates) page, the Render menu section of the topbar page, the `RenderSettings` API page, the `ImageFormatSettings` API page and the `bpy.ops.render` API page. I also looked at the manual's Output panel screenshot (`_images/render_output_properties_output_panel.png`): it shows a panel headed "Output" with a path field reading `/tmp/` and checkboxes for Overwrite, Placeholders, File Extensions and Cache Result, plus File Format, Color, Color Depth and Compression. There is no enable/disable control on the panel header or beside the path. (That screenshot also does not show the "Media Type" menu the page text describes, so the picture appears older than the text — I have not relied on it for anything but the absence of an on/off control, and even that is only what one screenshot shows.)

**Outside the interface.** The command-line render paths are documented to write. The release notes say: "Rendering through the command line always does an animation render, so no change of behavior is expected. Rendering using the `render.render()` operator when `animation=False` will see the change of behavior". Also `-o, --render-output <path>` sets the location from outside, and `-f, --render-frame <frame>` is described as "Render frame <frame> and save it." This is **documented** for command-line rendering. Outside-interface rendering as a whole does not have one answer: the `bpy.ops.render.render` operator also exposes the `write_still` choice quoted above when `animation=False`.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. The sources still say the *formats* available differ by build and system: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.0.1 started from a terminal with no window and with `--factory-startup`.

**Hand test 1 — is there an off switch at all.** Blender was asked for `save_output` on the render settings, the property Blender 5.2's manual describes as turning automatic saving off:

```
RenderSettings.save_output = MISSING
```

The property does not exist in 5.0. The same question was asked of all four versions and it appears only in 5.2, so the answer above is a fact about 5.0 rather than a search that stopped early.

**Hand test 10 — what happens with the File Path emptied.** A two-frame 64×64 file was saved with `render.filepath` set to the empty string, then rendered as an animation from a terminal with the working folder set to the folder holding the file. The command was `blender -b --factory-startup emptypath.blend -a`. What came back, word for word:

```
00:00.304  blend            | Read blend: ".../out_write/5.0/emptypath.blend"
00:00.309  render           | Rendering animation (frames 1..2)
00:00.309  render           | Rendering frame 1
00:00.309  render           | Start rendering: Scene, ViewLayer
00:00.309  render           | Engine: Workbench
00:00.418  image            | ERROR Couldn't create directory for file 0001.png: No such file or directory
00:00.418  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
00:00.418  render           | Time: 00:00.10
```

No file was written, and the run stopped after frame 1. So an empty File Path in 5.0 is a failure rather than a way of switching writing off.

**Hand test 10 — a muted File Output node.** Two otherwise identical two-frame files, one with a File Output node aimed at an empty folder and muted, one with the same node unmuted, each rendered as an animation from a terminal. The folder listings afterwards:

```
muted_fileoutput  -> [end]
unmuted_fileoutput-> file_nameBeauty0001.png file_nameBeauty0002.png [end]
```

Muting the node stops it writing. That settles the step this answer's other reading left open.

**Hand test 4 — what is written when the compositor has no output node.** With the Compositing switch on, the Group Output node deleted, and a Render Layers node feeding an Invert node that goes nowhere:

```
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

A file was written and it held the scene's raw render.

**Outside the interface. Now observed.** Every case above was driven from a terminal with no window, so the documented statement that a command-line render writes is observed as well, and the empty-path case shows what happens when the path gives it nothing to go on.

### Observed — window tests, 2026-08-03

This one is an **interface observation**, unlike the terminal tests above. The Creator ran it by hand in Blender 5.0's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-up is in `Audits/Blender Research/The compositor/Hand tests.md`, entry 16.

**Hand test 16 — what the Output panel holds, and whether its header has a checkbox.** The Properties editor's Output tab was opened and the panel called Output looked at, header first. The Creator answered in two parts: words for the Media Type control, and a screenshot for the rest, saved as `5.0 and 5.1.png` in a folder called `Test 16` on their own computer — one picture for both versions, because the panel looks the same in each.

Their words:

> 5.0, 5.1, 5.2: The Media Type dropdown has three items: Image; Multi-Layer EXR; Video.

Those are the same three values as the `media_type` enum quoted above, `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`. What the picture shows, top to bottom:

- The panel header reads **Output**, with the ordinary collapse arrow to its left and the drag handle on the right. **There is no checkbox beside the word "Output".**
- A file path field reading `/tmp/`, with a folder button at its right end.
- **Saving** — "File Extensions", ticked; "Cache Result", unticked.
- **Media Type** — "Image".
- **File Format** — "PNG (.png)".
- **Color** — three buttons, "BW", "RGB", "RGBA", with RGBA chosen.
- **Color Depth** — two buttons, "8" and "16", with 8 chosen.
- **Compression** — a slider reading "15%".
- **Image Sequence** — "Overwrite", ticked; "Placeholders", unticked.

Below the Output panel the picture also shows two further panels, "Color Management" and "Pixel Density".

Two things this settles. The Media Type control the 5.0 manual's text describes is really there, so the manual's own screenshot — which does not show it, as noted above — is the part that is out of date, not the text. And there is **no on/off checkbox in this panel's header in 5.0**, which is what this answer says from the sources. The same look in Blender 5.2 does show one; 4.5, 5.0 and 5.1 do not.

**Outside the interface.** This settles nothing about it. It is a look at what a Blender window shows.

**Operating system.** It was done on macOS. It was done again on the rented Linux machine, and the two agreed; the Linux result is in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`. **Second results, not replacements.**

**Hand test 1 — whether an output-off switch exists.** `RenderSettings.save_output = MISSING`, the same as on the Mac. There is no such property in 5.0.

**Hand test 10 — whether the place frames are written can be switched off.** Four files were built and rendered from the command line. What came back, word for word:

```
BLENDER_VERSION 5.0.1
SAVED emptypath.blend with render.filepath = ''
render.save_output MISSING in this version; file saved with the same tree and no such switch
SAVED saveoff.blend
SAVED mutedfo.blend with the File Output node muted
SAVED unmutedfo.blend
END_OF_BUILD
----- emptypath.blend -a
00:00.664  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
----- saveoff.blend -o cli_saveoff_ -F PNG -a
00:00.591  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/saveoff_fileoutput/file_nameBeauty0001.png'
00:00.592  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/cli_saveoff_0001.png'
00:00.596  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/saveoff_fileoutput/file_nameBeauty0002.png'
00:00.597  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/cli_saveoff_0002.png'
----- mutedfo.blend -a
00:00.688  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/muted_main_0001.png'
00:00.696  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/muted_main_0002.png'
----- unmutedfo.blend -a
00:00.782  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/unmuted_fileoutput/file_nameBeauty0001.png'
00:00.783  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/unmuted_main_0001.png'
00:00.804  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/unmuted_fileoutput/file_nameBeauty0002.png'
00:00.805  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.0/unmuted_main_0002.png'
----- listings
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
file_nameBeauty0001.png
file_nameBeauty0002.png
--- muted_fileoutput:
--- unmuted_fileoutput:
file_nameBeauty0001.png
file_nameBeauty0002.png
```

The same as on the Mac on every point: an emptied File Path does not switch the writing off, it makes the render fail with an error and write nothing. A muted File Output node writes nothing while the scene's own output still writes. And there is no switch that turns the writing off.

**Hand test 4, case 2 — removing the compositor's output point.** Written out in full under question 1: `RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]`. The raw render still reached the file.

### Observed — the same window test on the rented Linux machine, 2026-08-06

**Hand test 16 — whether the Output panel's header carries a checkbox.** The Properties editor was put on the Output tab, made to fill the window, scrolled so the panel and its header were both visible, and photographed with Blender's own screenshot command. The header reads `∨ Output` with **no checkbox** beside it. The panel holds, top to bottom: the file path field reading `/tmp/` with a folder button; **Saving** with **File Extensions** (ticked) and **Cache Result** (unticked); a **Media Type** dropdown reading **Image**; **File Format** reading **PNG (.png)**; **Color** as **BW / RGB / RGBA** with RGBA chosen; **Color Depth** as **8 / 16** with 8 chosen; **Compression** reading **15%**; **Image Sequence** with **Overwrite** (ticked) and **Placeholders** (unticked); and two closed sub-panels, **Color Management** and **Pixel Density**. That is what the Creator reported on the Mac, Media Type included.

**Outside the interface.** This settles nothing about it.

**Operating system.** It was done on Linux and settles Linux. With the Mac result above it, macOS and Linux now both have an answer and they agree. Windows stays untested.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.ops.render.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Answer.** The sources document that each scene chooses a compositor node tree, and that the same tree can be reused across scenes or blend files. Calling the tree itself file-level takes one extra step from where the release-note example creates it.

**The tree is a shareable data block.**

> "This adds a compositing node tree data block to the scene, which can then be reused
> across different scenes or linked/appended from other blend-files."

> "- The compositing node tree is now its own data block, making it reusable across blend files.
>   (blender/blender@bd61e69be5)"

**Each scene points at one.**

> "Node Tree
>
> Chooses which node group (compositor node tree) to display and edit.
> Each Scene can have its own compositor node tree.
> You can also create and switch to custom node groups for reuse."

The pointer is a scene property whose type is the shared data block type:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

and the release notes show the two being created and joined separately:

> "```python
> # New way of creating a node tree
> tree = bpy.data.node_groups.new(\"My new comp\", \"CompositorNodeTree\")
> scene.compositing_node_group = tree
> ```"

The example makes the tree in `bpy.data.node_groups` and then assigns it to the scene.

**LABEL: documented** — that the data block can be reused and that each scene points at one.

**LABEL: inferred** — for calling the node tree itself file-level. The step is that the release-note example creates it in `bpy.data.node_groups` before assigning it to a scene; the quoted words do not themselves call that collection the file's list.

**A compositor node tree is not only for scenes.** The video sequencer can hold one too, through a strip modifier:

> "node_group
>
> Node group that controls what this modifier does
>
> Type:
>
> NodeTree"

> "Compositor modifier is powerful tool to create arbitrary effects. To use it, you need to open compositor editor
> and change node subtree to sequencer. Then you can add new node tree and use it for any strip. These node trees
> can be marked as asset and reused across multiple .blend files."

**LABEL for the sequencer use: documented.**

**Not covered.** The sources do not say what happens when two scenes share one tree — in particular, whether a Render Layers node inside a shared tree follows the scene using the tree or the scene named on the node, nor whether the two scenes evaluate it independently. They also do not say whether a scene can hold more than one compositing node group at once (the property is a single slot, but the sources do not spell out the consequence).

**Outside the interface.** The ownership is a property of the file, so it is the same however the file is opened; the editor header described above is only the interface's way of picking the tree. That the ownership is unchanged when Blender is driven from outside is not stated in so many words: **inferred** from the fact that `compositing_node_group` is a saved scene property and `bpy.data.node_groups` is a file-level collection.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. Nothing in the passages above mentions an operating system.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.0.1 started from a terminal with no window and with `--factory-startup`.

**Hand test 5 — two scenes pointing at one node group.** Two scenes were made, A rendering blue `[0.0745, 0.3333, 0.8196, 1.0]` and B rendering green `[0.4275, 0.7686, 0.3137, 1.0]`. A was given a compositing node group holding a Render Layers node whose Scene was set explicitly to A, feeding an Invert node into the Group Output. B was then pointed at that same node group, and each scene rendered in turn with Compositing on:

```
RESULT sharing one tree between two scenes -> True
RESULT shared tree, rendering scene A -> [0.7804, 0.7529, 0.4235, 1.0]
RESULT shared tree, rendering scene B -> [0.7804, 0.7529, 0.4235, 1.0]
```

`[0.7804, 0.7529, 0.4235, 1.0]` is blue inverted. Both scenes wrote the same picture: A's render, put through the shared tree. So the sharing works, and the Render Layers node inside a shared tree follows the scene named on the node, not the scene being rendered. That answers the thing listed above as not covered.

**Hand test 12 — whether choosing a scene from outside picks that scene's compositor.** One file, `scenes.blend`, was saved holding two scenes: A rendering blue with a compositor that inverts it to yellow, and B rendering green with a compositor that inverts it to magenta, both with Compositing on. Two renders were run from a terminal, `blender -b --factory-startup scenes.blend -S A -o .../scene_A_ -F PNG -f 1` and the same with `-S B`, and the written frames read back:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [0.7804, 0.7529, 0.4235, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
```

Yellow for A, magenta for B: `-S` picks the scene and that scene's own compositor runs.

**Outside the interface. Now observed.** Both tests ran from a terminal with no window. The step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.0's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 17 and 20.

**Hand test 17 — what the Compositor editor's header offers.** Quoted in full under question 1. The Creator's report, given for 5.0, 5.1 and 5.2 together:

> No checkbox. Same four menus, but now there's a Node Tree Sub-Type dropdown to their left with two options, "Scene" (default) and "Sequencer". Center-top is the ordinary data-block selector plus a "+New" button (the standard pattern used throughout Blender's node editors). Nothing exists until you click +New, which creates a real data-block called "Compositor Nodes" with the same default setup as 4.5, except the final node is called Group Output instead of Composite.

Three things in that bear on this answer. The header's middle is a data-block selector with a New button, which is the ordinary way a shared data-block is picked in Blender — the arrangement the release-note example quoted above builds in two steps. Clicking New makes a data-block with a name of its own, "Compositor Nodes". And the dropdown to the left of the menus offers two sub-types, "Scene" and "Sequencer", which is the same two-place split this answer describes from the sources: a compositing node tree is used by scenes and also by the sequencer's own strip modifier.

**Hand test 20 — what the Render menu offers.** The Video Editing tab along the top of the window was clicked, then the Render menu in the very top bar was opened, in a file where nothing had been set up. The Creator reported the menu is identical in all four versions and answered with one screenshot, saved as `all versions.png` in a folder called `Test 20` on their own computer. What the picture shows, in order:

- Render Image — F12
- Render Animation — ⌘ F12
- Render Audio…
- View Render — F11
- View Animation — ⌘ F11
- Lock Interface — a checkbox, unticked

with dividing lines after "Render Animation", after "Render Audio…" and after "View Animation". **No entry in it names a Sequencer Scene.** Note what that does and does not show: the manual sentence quoted under question 7 makes those entries conditional — "If a Sequencer Scene exists that differs from the active scene, render that scene instead" — and this look was taken in a fresh file where no such scene had been made. So it records that the entries are not always in the menu; it does not show that they never are.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.0.1. **Second results, not replacements.**

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

### Observed — the same window tests on the rented Linux machine, 2026-08-06

**Hand test 17 — what the Compositor editor's header offers.** Written out in full under question 1: a scene dropdown, the four menus, a node-group chooser with a **New** button, and no Use Nodes checkbox.

**Hand test 20 — what the Render menu offers.** The top bar's Render menu was opened and photographed. It is headed **Render** and holds, in order: **Render Image** (F12), **Render Animation** (Ctrl F12), **Render Audio…**, a separator, **View Render** (F11), **View Animation** (Ctrl F11), a separator, and **Lock Interface** as an unticked checkbox. **No entry names a Sequencer Scene.** One thing to say plainly about the picture: a tooltip reading "Render active scene." had appeared over the item under the pointer and covers part of the Render Animation row; the shortcut column is still legible behind it. The menu is the same as the Creator reported on the Mac.

**Outside the interface.** These two settle nothing about it.

**Operating system.** Both were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/editors/compositor.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strip_modifiers.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.SequencerCompositorModifierData.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**Ambiguity flagged.** "Takes in another scene's rendered result" can mean two different things in a Blender file: a Render Layers node in scene A pointing at scene B, or a Scene strip in scene A's video sequencer pointing at scene B. Because this sheet is about the compositor, I answer the Render Layers reading first and give the Scene strip reading after. I have not rewritten the question.

### Reading one: a Render Layers node in scene A names scene B

**Answer.** What scene A receives is scene B's raw render, not scene B's composite. The manual states this as the reason for a workaround:

> "Hint
>
> To use the compositing output from another scene rather than its \"raw\" render output,
> first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
> then load those images into the Compositor of the current scene using the
> Image Node."

Scene B's render itself does happen — the node's whole job is to cause it:

> "Renders a View Layer and reads its
> Passes into the compositing node graph."

> "Scene
>
> The scene for which to render a view layer."

and the API describes the node as "Input render passes from a scene render". The API also confirms that a scene can be rendered as a side effect of another: `use_render_cache` reads "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)".

**LABEL: documented** — that what arrives is the raw render output and not the compositing output, and that scene B's view layer is rendered.

**LABEL: not in the sources** — for the literal question of whether scene B's compositor node tree is *executed* and then discarded. The manual only tells you what you get, not what runs. Nothing I found states that scene B's `use_compositing` setting, or its assigned node group, has any effect in this situation.

### Reading two: a Scene strip in scene A's sequencer names scene B

Here the sources describe the Scene strip case differently, but not far enough to identify whose compositor is meant:

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

**LABEL: documented** for the sentence itself. Note that the sentence does not say *whose* Compositing setting is meant — the strip's scene or the scene doing the rendering. I have not resolved that, and I am not going to guess.

Related documented limit on this path:

> "Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

**Not covered.** Whether scene B's node tree is evaluated-and-discarded in the Render Layers case. Whose "Compositing" setting the Post Processing sentence refers to in the Scene strip case. What happens when scene B has no node group assigned, or has compositing switched off, in either case. Whether a chain of three scenes behaves like a chain of two.

**Outside the interface. Now observed.** The tests below were run from a terminal with no window, and gave the results described.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.0.1 started from a terminal with no window and with `--factory-startup`.

**Hand test 7 — a Render Layers node pointing at another scene.** Two 64×64 scenes. B renders a flat green slab; its compositing node group sends its Render Layers node through an Invert node to its Group Output — so B's own composite is magenta — and also straight into a File Output node aimed at an empty folder, so that any work B's compositor does leaves a file behind. A renders blue on its own, and its whole compositing node group is one Render Layers node whose Scene is set to B, wired to A's Group Output. A was rendered twice, with B's Compositing switch on and then off, and B's File Output folder checked each time. Three controls were rendered as well:

```
RESULT B.use_compositing=True -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=True -> files in B's File Output folder: []
RESULT B.use_compositing=False -> A wrote 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=False -> files in B's File Output folder: []
CONTROL A alone, compositing off -> [0.0745, 0.3333, 0.8196, 1.0]
CONTROL B alone, compositing off -> [0.4275, 0.7686, 0.3137, 1.0]
CONTROL B alone, compositing on  -> [0.7882, 0.4588, 0.7882, 1.0]
```

The controls give the three colours to read the result by. What A wrote was B's raw render, both times. B's File Output folder was empty both times, though rendering B directly fills it. This is the positive-control test the audit asked for: it separates which result arrived from whether B's compositor did any work at all, and on this evidence it did none.

**Hand test 8 — a Scene strip pointing at another scene.** The same two scenes, except A takes B in through a Video Sequencer Scene strip with A's Sequencer switch on, and A's own compositor is a plain pass-through. Rendered four times, once for each combination of the two scenes' Compositing switches, with B's File Output folder cleared before each run:

```
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['file_nameBeauty.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['file_nameBeauty.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

This settles the ambiguity flagged above. "If Compositing is also enabled" means the strip's own scene's switch: the written frame follows B's switch and is unaffected by A's, and B's File Output node writes exactly when B's switch is on, which shows B's compositor genuinely runs.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Both tests were run again, unchanged, on the rented Linux machine, with Blender 5.0.1 started with no window and with `--factory-startup`. **Second results, not replacements.**

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
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0] ; files in B's File Output folder: ['file_nameBeauty.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4549, 0.7882, 1.0] ; files in B's File Output folder: ['file_nameBeauty.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

The frame follows B's switch and not A's, and B's File Output node writes exactly when B's switch is on. The same as on the Mac.

**Outside the interface.** Both were run from a terminal with no window, as on the Mac.

**Operating system.** Both were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeRLayers.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

Three parts, answered in turn.

### (a) What it arrives as

It arrives through a Render Layers node, as a set of output sockets: an image, an alpha, and one socket per enabled render pass.

> "Renders a View Layer and reads its
> Passes into the compositing node graph."

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

The Combined pass is described elsewhere as "The render output before any compositing is applied."

What kind of data those sockets carry is spelled out:

> "Compositing nodes operate on data that is either an image or a dimensionless single value. For
> instance, the Levels node outputs a single value, while the
> Render Layers node outputs an image."

> "Three types of data exist, all of which are stored in half precision formats:
>
> Float
> ...
> Vector
> ...
> Color"

An image is not only its pixels; it also has a position and size in the compositing space:

> "Consequently, images are not only represented by their size, but also by their transformation in
> that space, much like 3D objects have transformations. ... The rectangular area occupied by an image in that space as defined
> by its transformation and size is called the Domain of the image."

**LABEL: documented.**

### (b) Whether it can be told apart from anything else there

At the node level, yes. The render comes in through a node type that exists for nothing else, carrying the scene and view layer it came from:

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render
>
> layer
>
> Type:
>
> enum in ['PLACEHOLDER'], default 'PLACEHOLDER'
>
> scene
>
> Type:
>
> Scene"

Everything else that brings pixels into the tree is a different node type — the manual's Input Nodes list has Bokeh Image, Image, Image Info, Image Coordinates, Mask, Movie Clip, Group Input, and the Scene group that holds Render Layers.

**LABEL: documented** — that `CompositorNodeRLayers` is an input for render passes from a scene render and exposes scene and layer properties.

**LABEL: inferred** — for saying the origin can therefore be distinguished at node level. The step is that the Render Layers class and the separately named input-node types give the source a distinct node identity; the quoted words do not state the conclusion themselves.

At the data level, no — nothing marks a value as having come from the render. The only types are Float, Vector and Color, as quoted above, and none of them carries a source marking. Getting from "there are three types" to "so a render pixel is indistinguishable from any other pixel once it leaves the node" requires the reader to supply a step, so:

**LABEL: inferred.** The step: the sources enumerate the compositor's data types exhaustively ("Three types of data exist"), and none of the three has any field for provenance; therefore identification has to be done by looking at which node produced the value, not by looking at the value.

### (c) Whether it can be followed forward through pass-through and fan-out nodes

The link topology can be followed. A Reroute node has one input and allows more than one output connection:

> "A node used primarily for organization.
> Reroute looks and behaves much like a socket on other nodes in that it supports one input
> connection while allowing multiple output connections."

The API says the same in one line:

> "class bpy.types.NodeReroute(NodeInternal)
>
> A single-socket organization tool that supports one input and multiple outputs"

The API also exposes the links on each socket:

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

**LABEL: documented** — for the Reroute node's one input and multiple output connections, and for the socket link list.

**LABEL: inferred** — for saying that a Reroute passes the value along unchanged. The sources call it an organization tool that behaves like a socket, but they do not state in so many words that the value is unchanged.

A muted node exposes another mapping:

> "internal_links
>
> Internal input-to-output connections for muting"

That quote establishes that the mapping exists. It does not state how a trace should use it, or that a muted node passes the value through unchanged. A muted link is a different case: the manual says it is "ignored in evaluation", so it does not support the pass-through claim.

**Not covered.** The sources describe Reroute, muting and link lists, but they do not give a recipe for walking a tree, and they do not say how a trace behaves when it passes into or out of a node group (Group Input and Group Output nodes exist, but the traversal is not described). They also do not say whether pass names are stable — the release notes record that "Many render passes were renamed to avoid obscure abbreviations. For example 'DiffCol' to 'Diffuse Color', 'IndexMA' to 'Material Index' and 'Z' to 'Depth'." Nothing here settles what happens to a trace through the Switch node or the Menu Switch node at render time, only that Switch "Switch between two images using a checkbox."

**Outside the interface.** The node tree and its links are file data, and the API exposes both the node types and each socket's links, so the same tracing is available when Blender is driven from outside. That last step is **inferred** from the existence of the `NodeSocket.links` and `CompositorNodeRLayers` API entries; the sources do not state it. One genuine interface-only difference does exist nearby, and it is documented: the viewport compositor is limited — "The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty."

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The test below was run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. **The exception is hand test 19, what hovering a link shows.** That was attempted on the Linux machine and could not be settled there, so that reading rests on the Mac alone — see the two sections further down. Windows was not tried, because no Windows machine has run it. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. Hand test 19 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)*

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.0.1 started from a terminal with no window and with `--factory-startup`.

**Hand test 11 — telling a render apart, and following it forward, with no window open.** One 64×64 scene was given a compositing node group holding a Render Layers node named `TheRender`, an Image node named `TheImage`, a Reroute named `TheReroute`, a Blur node named `MutedBlur` that was muted, a Viewer named `TheViewer`, and the Group Output. The render's Image socket goes into the Reroute; the Reroute feeds the muted Blur, which feeds the Group Output; the Reroute also feeds the Viewer through a link that was itself muted. What came back, word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
RenderLayers output sockets = ['Image', 'Alpha', 'Depth', 'Normal', 'UV', 'Vector', 'Position', 'Deprecated', 'Deprecated', 'Shadow', 'Ambient Occlusion', 'Deprecated', 'Deprecated', 'Deprecated', 'Object Index', 'Material Index', 'Mist', 'Emission', 'Environment', 'Diffuse Direct', 'Diffuse Indirect', 'Diffuse Color', 'Glossy Direct', 'Glossy Indirect', 'Glossy Color', 'Transmission Direct', 'Transmission Indirect', 'Transmission Color', 'Subsurface Direct', 'Subsurface Indirect', 'Subsurface Color']
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

Three things this settles. The two node types carry different identifiers, so a program can tell the render's entry point from an image file's. The `layer` property reports `'ViewLayer'`, not the `'PLACEHOLDER'` the reference page prints. And the walk ran to completion, reaching both destinations through the Reroute, with the muted link and the muted node each flagged on the way. The pass socket names quoted here also record the renaming the release notes describe — `Diffuse Color` rather than `DiffCol`, `Material Index` rather than `IndexMA`, `Depth` rather than `Z`.

**Hand test 2 — what the Split node offers.** Blender was asked to add the node and print its sockets:

```
ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']
```

**Outside the interface. Now observed.** The whole of hand test 11 ran in a session with no window open, so the tracing described above is available that way. The step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal test above. The Creator ran them by hand in Blender 5.0's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 19 and 21.

**Hand test 19 — what hovering a link shows.** A small tree was built in the Compositing editor — a Render Layers node and an Image node with an image loaded, each feeding its own Viewer node — one frame was rendered and the render window closed, and then the link out of each node was hovered. The Creator's report for 5.0 and 5.1 together, against the "Image." with a trailing period they saw in 4.5:

> 5.0, 5.1: Same text, "Image", but no trailing period.

So something does appear in 5.0, and it is the socket's name. The Creator reported one wording and did not say it differed between the two links, so this record cannot say whether the Render Layers link and the Image link showed different things. Nothing in what was reported names where the image came from, which is what this answer says the sources describe no way of doing. Blender 5.2 shows a good deal more here; 5.0 shows the name alone.

**Hand test 21 — how a second output node is marked.** Quoted in full under question 1. With two Group Output nodes in one tree, the last-selected one is the active one, and the inactive one carries a warning above its header — a triangle with an exclamation mark, then the words "Unused Output". That is a second way of telling which output is the live one, beside the `is_active_output` reading in hand test 11 above, and this one is visible on the node itself.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Test 21 was done again on the rented Linux machine and agreed; test 19 was attempted there and could not be settled. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.0.1. **Second results, not replacements.**

**Hand test 2 — which compositor nodes exist, and what the Split node's sockets are.** The printed block is long; the lines that carry the answer came back word for word as:

```
BLENDER_VERSION 5.0.1
bpy.types.CompositorNodeComposite = MISSING
TREE_TYPE CompositorNodeTree
TREE_STARTING_NODES []
ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']
ADD CompositorNodeSwitch -> inputs=['Switch', 'Off', 'On'] outputs=['Image']
ADD CompositorNodeRLayers -> inputs=[] outputs=['Image', 'Alpha', 'Depth', 'Normal', 'UV', 'Vector', 'Position', 'Deprecated', 'Deprecated', 'Shadow', 'Ambient Occlusion', 'Deprecated', 'Deprecated', 'Deprecated', 'Object Index', 'Material Index', 'Mist', 'Emission', 'Environment', 'Diffuse Direct', 'Diffuse Indirect', 'Diffuse Color', 'Glossy Direct', 'Glossy Indirect', 'Glossy Color', 'Transmission Direct', 'Transmission Indirect', 'Transmission Color', 'Subsurface Direct', 'Subsurface Indirect', 'Subsurface Color']
ADD CompositorNodeOutputFile -> inputs=[''] outputs=[]
ADD CompositorNodeViewer -> inputs=['Image'] outputs=[]
ADD CompositorNodeComposite -> ERROR Error: Node type CompositorNodeComposite undefined
ADD NodeGroupOutput -> inputs=[''] outputs=[]
```

Identical to the Mac's, socket for socket, including the Split node having one output.

**Hand test 11 — telling a render apart and following it forward.** What came back, word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
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

**Hand test 19 — what hovering a link shows. This could not be settled here, and no answer is written in.** A Blender window was opened on the rented Linux machine with the same tree the Mac test used, one frame was rendered, and the pointer was put over the link out of the Render Layers node, over that node's own Image output socket, and over the link out of the Image node. It was tried four ways: with Blender moving its own pointer to coordinates it worked out from the node positions; with the pointer moved from outside in several small steps and held; with dwells of 2.5 seconds and of 8 seconds; and with the picture taken by Blender itself on a signal, so the pointer was checked to be in place first. No display appeared in any of them, and none appeared over a control that certainly has a tooltip either. **That is a fault in the run, not a finding about Blender.** The machine's display is shared with other topics being tested at the same moment: one check printed `AT 438,227 -> x:573 y:68`, meaning something else moved the pointer between putting it there and reading it back. So the socket-inspection reading for 5.0 still rests on the Mac alone. What to run when the machine is free is written out in `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

**Outside the interface.** These settle nothing about it.

**Operating system.** Test 21 now has an answer from both machines and they agree. Test 19 has an answer from the Mac only; Linux and Windows are both open for it.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/render/layers/passes.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/interface/controls/nodes/types/layout/reroute.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/interface/controls/nodes/node_editors.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/index.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeRLayers.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.NodeReroute.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.NodeSocket.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.Node.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/rendering/ — read 2026-07-30

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Answer.** The sources establish that Render Region together with Crop to Render Region can decide the final render size. They support an inference that an unsqueezed Stereo 3D layout can also decide one dimension. They also establish the size of the final composite. They do not establish the size written by a compositor File Output node or every multi-view case, so this is not an exhaustive list.

### (i) Render Region, together with Crop to Render Region — documented

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

The API pair:

> "use_border
>
> Render a user-defined render region, within the frame size"

> "use_crop_to_border
>
> Crop the rendered frame to the defined render region size"

The region itself is defined in the file, and it reaches the final render:

> "If you define one while in Camera View,
> it will apply not just to the viewport, but also to the final render."

> "Note
>
> Render regions only apply to the viewport when using Cycles, not when using EEVEE.
> However, they always affect the final render."

The region's extent is stored as four properties on the scene's render settings: `border_min_x`, `border_max_x`, `border_min_y`, `border_max_y`.

**LABEL: documented** that Crop to Render Region crops *the rendered image* to the region's size, and that the region reaches the final render; **inferred** for the step from "the rendered image is cropped" to "the file written to disk is that size" — the quoted words say "rendered image" and "rendered frame", never "written" or "saved". *(Citation check, 2026-08-03: previously labelled documented outright, and the four properties were called scene properties when they sit on `RenderSettings`. Both corrected. The hand test below measures the written file.)*

Supporting picture: the manual's Format panel screenshot (`_images/render_output_properties_format_panel.png`) shows Resolution X 1920 px, Y 1080 px, % 100%, Aspect X and Y 1.000, an unticked "Render Region" checkbox, an unticked and greyed-out "Crop to Render Region" checkbox below it, and Frame Rate 24 fps. That Crop to Render Region is greyed out while Render Region is off is visible in the picture. **LABEL for this piece: documented** (it is part of the manual).

### (ii) Stereo 3D output formats — inferred

The written file can hold two views combined into one image:

> "SIDEBYSIDE
> Side-by-Side – Render views for left and right eyes side-by-side.
>
> TOPBOTTOM
> Top-Bottom – Render views for left and right eyes one above another."

> "use_squeezed_frame
>
> Combine both views in a squeezed image"

and the manual talks about the resolution consequences:

> "The Top-Bottom compressed will discard half of your vertical resolution data."

> "For the Stereo 3D formats the only lossless options are
> Top-Bottom and Side-by-Side without the Squeezed Frame option."

**LABEL: inferred.** The step: the sources say the two views are placed side by side or one above the other in a single image, and that the squeezed variant is the one that loses half the resolution while the unsqueezed variant is lossless. For the unsqueezed variant to be lossless while holding two full views, the written image has to be twice the scene's width (side-by-side) or height (top-bottom). The sources never write down that doubled figure, so this is my step, not theirs.

### (iii) The final composite stays at the scene render size — documented

> "The operation domain of output nodes like the Viewer node is
> a domain with an identity transformation and the same size as the final compositor output. For
> viewport compositing, that size would be the viewport size, and for
> final render compositing, that size would be the scene render size."

and, said again from the stereo side:

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

This final-composite rule holds even though individual nodes can change an image's size inside the graph — the Crop node "crops an input image to a selected region by either making the cropped area transparent or by resizing the input image", and the Scale node has "Render Size — Image dimensions set in the Render panel". These passages do not settle the separate File Output node case below.

**LABEL: documented.**

### Things I checked that do not decide it

- Aspect X, Y: "Older televisions may have non-square pixels, so this can be used to control the shape of the pixels along the respective axis." **LABEL: documented** for controlling pixel shape rather than the listed pixel dimensions.
- Pixel Density: "The pixel-density is meta-data which doesn't impact the quality of the image." **LABEL: inferred** for not deciding pixel dimensions; the source calls it metadata but does not state that conclusion.
- Video codecs: "some codecs do not work with arbitrary dimensions". **LABEL: inferred** for treating this as a constraint on a chosen size rather than a setting that chooses one; the quote does not make that distinction itself.
- Video sequencer strips: "If you have a mismatch between the size of the input image and the render output size, the Video Sequencer will try to auto-scale the image to fit it entirely in the output." There is an operator that goes the other way: "Set Render Size — Sets the render resolution and aspect to match the strip's resolution", but that edits the scene's Resolution X/Y, which the question already excludes. **LABEL: inferred** for the conclusion that ordinary strip evaluation does not decide the output dimensions; the passages describe fitting and an explicit editing action, not that general conclusion.

**Not covered.** Whether the compositor's File Output node writes at the scene render size or at the size of whatever is plugged into it. The domain rule above names the Viewer node and says "output nodes like", and the File Output page itself says nothing about size; the 5.0 release notes only say File Output "now take[s] translation into consideration and relevant inputs are clipped if translated". Reading "clipped" as "clipped to the scene render size" would be my step and I have not taken it — I am recording this as unsettled, and it is in the hand-test list below. Also not covered: whether Render Region interacts with the compositor's domains, and whether multi-view (as opposed to Stereo 3D) affects a single file's dimensions.

**Outside the interface.** The Render Region case is documented to reach the final render, and all the settings involved are scene properties saved in the file. The sources do not state whether the result is the same when the render is driven from outside. One nearby documented interface-only case shows such differences can be called out: `use_single_layer` — "Only affects rendering from the interface, ignored for rendering from command line." No such note appears on `use_border`, `use_crop_to_border` or the stereo format settings. Concluding sameness from that silence is **inferred**.

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. **The exception is the second half of hand test 18** — ticking Render Region and looking at Crop to Render Region again. Ticking a box needs the pointer, which could not be relied on on the Linux machine, so that half rests on the Mac alone. Windows was not tried, because no Windows machine has run it. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. The second half of hand test 18 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)* The sources still say the available file formats differ by build and system: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.0.1 started from a terminal with no window and with `--factory-startup`.

**Hand test 6 — three things that might decide the written size.** A 400×200 Workbench scene with a cube and a camera, rendered three ways. First, with a Scale node set to Absolute 120×90 between the Render Layers node and a File Output node, while the unscaled render went to the scene's own output. Second, with the render region set to the middle half of the frame — 0.25 to 0.75 on both axes — and Crop to Render Region off and then on. Third, with stereoscopy on and both views written into one file, Side-by-Side and Top-Bottom, squeezed and unsqueezed:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> file_nameBeauty.png is 120x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

This settles the item recorded above as unsettled. The File Output node wrote at the size of the image plugged into it, 120×90, while the scene's own output stayed at 400×200 in the same render — so the domain rule that pins output nodes to the scene render size does not extend to it. It also turns the stereo step marked inferred above into a measurement: unsqueezed Side-by-Side doubles the width and unsqueezed Top-Bottom doubles the height, while the squeezed forms keep the scene's own dimensions. And it shows that a render region without Crop to Render Region leaves the written size untouched, which the answer above could only read as an implication.

One thing about the set-up, recorded because it changes how the test has to be built: in 5.0 a File Output node arrives with no input sockets at all and its format set to multi-layer EXR, so an item has to be added to it before anything can be plugged in. In 4.5 it arrives with one Image input already.

**Hand test 13 — what the output-path template reports under a crop.** The same 400×200 scene, render region cropped to the middle half, output path set to `{resolution_x}x{resolution_y}_`:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The template reports the scene's own Resolution values, not the size actually written.

**Hand test 9 — whether the sequencer route changes the written size.** A 400×200 scene rendered six ways across the Sequencer and Compositing switches, with and without a Scene strip. All six wrote 400×200; the lines are quoted under question 7.

**Hand test 14 — a very large frame.**

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
```

`[0.7882, 0.4588, 0.7882, 1.0]` is the inverted green, so the compositor ran and the size was not changed, on either device.

**Multi-view, which the audit sent back.** The stereo runs above used `use_multiview` with the Views Format set to Stereo 3D, which writes one file. The default Individual format writes one file per view instead and was not measured here.

**Outside the interface. Now observed.** All of the above ran from a terminal with no window. The step marked inferred above — concluding sameness from the absence of an interface-only note — is now observed for the render region, the crop, the stereo formats and the File Output node.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.0's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 16 and 18.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor's Output tab was opened at the Format panel, the two checkboxes looked at with Render Region unticked, then Render Region ticked and them looked at again. The Creator's report, which they gave for all four versions at once:

> All four versions: Render Region is clickable, off by default; while off, Crop to Render Region is greyed out. Ticking Render Region makes Crop to Render Region available, but it stays off by default.

That is what the manual's Format panel picture described above shows, seen now in the running program: both start off, and the second is unavailable until the first is on.

**Hand test 16 — what the Output panel holds.** Quoted in full under question 2. The one thing in it that bears on size is the Media Type control, which the Creator reported as a dropdown with three items — Image, Multi-Layer EXR, Video. Nothing in the panel offers a second way of deciding the written pixel size.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Test 16 was done again on the rented Linux machine and agreed; only the first half of test 18 could be done there. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.0.1. **Second results, not replacements.**

**Hand test 6 — what decides the pixel size of what gets written.** What came back, word for word:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> file_nameBeauty.png is 120x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

Identical to the Mac's, figure for figure — including the File Output node writing at its input's size, 120×90, and not the scene's.

**Hand test 13 — the path template under a crop.** What came back, word for word:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The same as on the Mac.

**Hand test 14 — the compositor above the size 5.2's documentation names.** Written out in full under question 1: 5.0 writes the composited picture at 9000×400 on either device.

**Hand test 15 on the Linux list — the card against the processor.** Written out in full under question 1. All four compositor trees, rendered on both devices and compared value by value at 320×240, matched to the last figure.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 16 — the Output panel.** Written out in full under question 2: a **Media Type** dropdown reading **Image** is there, and no checkbox in the panel's header.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor was put on the Output tab, made to fill the window, and photographed. The Format panel holds two checkboxes one above the other, labelled exactly **Render Region** and **Crop to Render Region**. Both start unticked, and **Crop to Render Region is drawn greyed out** — its label and its box both dimmed — while Render Region is unticked. **Only the first half of the Mac's test was run here.** The Mac's version also ticked Render Region and looked again, and found Crop to Render Region became available; ticking a box needs the pointer, which could not be relied on here, so the second half stays as the Mac has it.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux, except the second half of test 18, which stays macOS only. With the Mac results above them, macOS and Linux now both have an answer for the rest and they agree. Windows stays untested.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/editors/3dview/navigate/regions.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/stereoscopy/usage.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/transform/crop.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/transform/scale.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/image.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.Stereo3dFormat.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Answer.** Yes. The two pipeline stages are separate switches in the same panel, so turning Compositing off and Sequencer on gives exactly that.

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

The switch itself, and its condition:

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist
>
> Type:
>
> boolean, default True"

Both are on by default (`use_compositing` default True, `use_sequencer` default True), so bypassing the compositor means turning Compositing off deliberately.

**LABEL: documented.**

**Supporting observed finding.** The Cycles panel code draws the two switches under one heading, one after the other:

```
        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

LABEL for this piece: **observed** — file `Blender Sources/cycles_5.0/ui.py`, lines 1124–1126.

**Supporting documented finding from a picture.** The Post Processing screenshot (`_images/render_output_properties_post-processing_panel.png`) shows "Compositing" and "Sequencer" as two separate ticked checkboxes on one row. **LABEL: documented.**

### What the sources establish about what changes in what gets written

- **The picture itself.** What is written is the sequencer's result "instead of the view from the 3D scene's active camera" — documented, quoted above.
- **It only takes effect if there is something to sequence** — "if sequencer strips exist" (documented, from `use_sequencer`).
- **Scene strips inside it are rendered as part of the same job** — "If the sequence contains Scene strips, these will also be rendered as part of the pipeline" (documented).
- **Passes are lost on scene strips** — "Scene strips do not render individual Render Passes; only the Combined render pass will be used." (documented).
- **The size does not change.** Strips are fitted into the scene's Resolution rather than setting it: "the Video Sequencer will try to auto-scale the image to fit it entirely in the output", and the Image Transform operators are all described as fitting content to "the project's Resolution". (documented)
- **Output settings come from the same place.** "The render and output settings of the Sequencer Scene are the same as for regular scenes. They can be found in the Output tab in the Properties editor." (documented)

### One 5.0 wrinkle worth recording, because it changes which scene is written

In 5.0 the sequencer works on a separate scene reference, and there are separate render commands for it:

> "The sequencer no longer uses the active scene in the window for its context. Instead it now uses the _Sequencer Scene_. This is a reference to a scene stored per workspace."

> "Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that scene instead.
>
> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene."

and from outside the interface the same choice exists as an operator flag:

> "use_sequencer_scene (boolean, (optional)) – Use Sequencer Scene, Render the sequencer scene instead of the active scene"

**LABEL: documented.**

**Not covered.** The sources do not say what a sequencer-pipeline render writes when the output format is Multi-Layer EXR, nor whether the Compositing switch being off changes anything about metadata, colour management or audio. They do not resolve whose Compositing setting the Post Processing sentence about Scene strips refers to (see question 4). They also do not say what is written when Sequencer is on but no strips exist and Compositing is off.

**Outside the interface.** Documented, in part: `bpy.ops.render.render` takes `use_sequencer_scene` — "Render the sequencer scene instead of the active scene" — so the sequencer-scene choice is available from outside. The switches themselves are scene properties saved in the file. The documented command-line render options do not include a flag for either pipeline switch. That the pipeline switches behave identically outside the interface is **inferred** from their being saved scene properties with no interface-only note, unlike `use_single_layer` which carries one.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. Video output does still depend on the build: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." Also relevant if the sequencer's compositor strip modifier is in play: the release notes say "Note that currently VSE Compositor modifier is always using the CPU compositor."

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.0.1 started from a terminal with no window and with `--factory-startup`.

**Hand test 9 — what the sequencer route writes.** A 400×200 scene was built whose camera view is blue, `[0.0745, 0.3333, 0.8196, 1.0]`, and whose compositor replaces the picture with flat red, `[0.8549, 0.2196, 0.1255, 1.0]`. A second scene rendering green, `[0.4275, 0.7686, 0.3137, 1.0]`, was added as a Scene strip for half the runs. Six renders across the Sequencer and Compositing switches:

```
RESULT no strips, sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer OFF, compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT strip,     sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer OFF, compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
```

This answers what the sources left open. With Sequencer on and no strips, a file is written and it holds the scene's ordinary picture — the camera view with Compositing off, the compositor's result with it on. With a strip present, the strip's picture is written whether the scene's own Compositing switch is on or off. And the written size stayed at 400×200 in all six.

**Hand test 8 — whose Compositing switch a Scene strip follows.** Quoted in full under question 4. The four runs show the picture follows the strip's own scene's switch, not the rendering scene's — which answers the ambiguity this answer inherits from question 4.

**Hand test 13 — what survives into a multi-layer EXR.** A scene with the Depth and Mist passes turned on, rendered to multi-layer OpenEXR twice: once plainly, once with a Scene strip supplying the picture and the Sequencer switch on. The channel names in the two files:

```
exr_seqFalse.exr: ViewLayer.Combined.A ViewLayer.Combined.B ViewLayer.Combined.G ViewLayer.Combined.R ViewLayer.CombinedpixelAspectRatio ViewLayer.Depth.Z ViewLayer.DepthpixelAspectRatio
exr_seqTrue.exr:  Composite.Combined.A Composite.Combined.B Composite.Combined.G Composite.Combined.R Composite.CombinedpixelAspectRatio
```

Without the sequencer the file holds a view layer with its Combined and Depth passes. With the sequencer supplying the picture it holds one layer called Composite with only the combined image; the Depth pass is gone. That answers the Multi-Layer EXR gap the audit sent back.

**Outside the interface. Now observed.** Every run above was started from a terminal with no window. Both switches take effect that way, and the step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.0's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15 and 20.

**Hand test 15 — what the Post Processing panel holds.** Quoted in full under question 1. The Creator saw two checkboxes, Compositing and Sequencer, both on by default, and one slider called "Dither"; and the Sequencer tooltip reads:

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

That is the same sentence as the API description of `use_sequencer` quoted above, word for word, and it confirms both switches sit in one panel as two separate boxes, either of which can be off while the other is on.

**Hand test 20 — what the Render menu offers.** Quoted in full under question 3. The Creator opened the Video Editing tab and then the Render menu in a fresh file, reported the menu identical in all four versions, and supplied one screenshot. It holds Render Image (F12), Render Animation (⌘ F12), Render Audio…, View Render (F11), View Animation (⌘ F11) and a Lock Interface checkbox. **No entry in it names a Sequencer Scene.**

Read this against the wrinkle recorded above rather than instead of it. The manual makes those extra entries conditional — "If a Sequencer Scene exists that differs from the active scene, render that scene instead" — and this look was taken in a fresh file where no such scene had been made. So the entries are not always in the menu; whether they appear once a Sequencer Scene differs from the active one was not tested and stays open.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.0.1. **Second results, not replacements.**

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
RESULT multi-layer EXR, sequencer=False -> file exists: True size 2640
RESULT multi-layer EXR, sequencer=True -> file exists: True size 1806
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

**Hand test 8 — whose Compositing switch a Scene strip follows.** Written out in full under question 4: the strip carries the strip's own scene's composite when that scene's Compositing switch is on and its raw camera view when it is off, and the rendering scene's own switch makes no difference.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 15 — what the Post Processing panel holds.** Written out in full under question 1: two checkboxes, Compositing and Sequencer, both ticked, and a Dither slider reading 1.00, and nothing else — and no checkbox in the panel's own header.

**Hand test 20 — what the Render menu offers.** Written out in full under question 3: Render Image, Render Animation, Render Audio…, View Render, View Animation and Lock Interface, and no entry naming a Sequencer Scene.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

Sources and addresses for this answer:
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/sequencer_scene.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/image.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.0/bpy.ops.render.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.0/sequencer/ — read 2026-07-30
- Cycles file `Blender Sources/cycles_5.0/ui.py`, lines 1124–1126 — read 2026-07-30

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

Every test that was on this list has been run, and the results are written into the answers above with the machine's own output or the Creator's own words quoted. The set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`. Nothing is left on it.

The seven that needed somebody looking at a Blender window were run by the Creator and are recorded under the questions they answer: what the Post Processing panel holds (entry 15, questions 1 and 7), what the Output panel holds (entry 16, questions 2 and 6), what the Compositor editor's header offers (entry 17, questions 1 and 3), whether Crop to Render Region is greyed out until Render Region is on (entry 18, question 6), what hovering a link shows (entry 19, question 5), what the Render menu offers (entry 20, questions 3 and 7), and how a second Group Output node is marked as the unused one (entry 21, questions 1 and 5).

Three things those seven left open on the Mac. **Two have since been answered on a second machine**: the Post Processing panel's own header carries no checkbox, and neither Group Output node's header carries a control for making it the live one. The third is still open on both: whether the Render menu grows its Sequencer Scene entries once a Sequencer Scene differs from the active scene, which a fresh-file look cannot show. All three are written in beside their findings.

**A second machine, 2026-08-06.** Every test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are written in beside the Mac's, never merged with them. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`. Every test agreed with the Mac except one, which could not be settled there at all: what hovering a link shows (entry 19). One new test was added that only the second machine could carry: whether a compositor tree gives the same picture on the graphics card as on the processor. It does, at 320×240, for the four trees tried.

**Still cannot be run with the available equipment.** Two things stay untested. **Windows** — no Windows machine has run it, so every "operating system" line above covers macOS and Linux and leaves Windows open, and that is why the compositor's documented driver-reset limit on Windows stays untested. **A graphics card of another make** — the two machines have Apple's own graphics and one NVIDIA card between them, so nothing here speaks for an AMD or Intel card. Three smaller things need the rented machine to itself, because its display is shared: what hovering a link shows, what the tooltip on 5.2's Output header checkbox says, and what Crop to Render Region looks like once Render Region is ticked.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 5.0

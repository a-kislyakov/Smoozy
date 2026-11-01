# The compositor — Blender 5.1

**Sources consulted, and when.** Blender 5.1 manual (plain-text snapshot plus two of its screenshots in `_images`), Blender 5.1 Python API documentation, Blender 5.1 release notes, and Cycles' own files as shipped with 5.1. All read on **2026-07-30** (the snapshot date given to me).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

(One qualification, so the line above is exact: I ran a single directory-existence check on the folder I was told to save this file in, `Preparation/Blender Research/Findings/5.1`, before writing. I did not list or open anything in it, or anywhere else in `Preparation/`.)

A note on how to read the answers below. Blender 5.1's compositor is documented as a **node group** owned by a scene, whose **Group Output** node carries the final image. Several pages call that node group "the compositing node tree" and several call it "the compositing node group"; I have quoted whichever wording the page used.

---

## 1. What decides whether a file's compositor runs at all?

**Label: documented** for each of the three conditions below, taken separately. **Inferred** for the claim that they form one complete checklist — see Not covered.

The sources document three separate conditions that affect whether a scene's compositor runs. Treating them as the whole list requires an inference.

**(a) A compositing node group must be assigned to the scene.**

> "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

— Python API documentation, `RenderSettings.use_compositing`, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

**(b) The Compositing option in Post Processing must be on.**

> "To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline
> option must be enabled in the output's Post Processing properties."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

The panel itself:

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

The screenshot on that same page (`_images/render_output_properties_post-processing_panel.png`, part of the manual) shows a panel titled "Post Processing" containing two ticked checkboxes labelled **Compositing** and **Sequencer**, and a **Dither** slider. **Documented** — what is visible in the picture is the two checkboxes and the slider, nothing more. Read 2026-07-30.

**(c) The node tree must contain an active output node, or the compositor is skipped.**

> "The compositor searches for the active Group Output node, if none was found, it searches for the
> active Viewer node. If none was found, the compositor doesn't run altogether."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30.

The same page adds that only one output can be active at a time:

> "The compositor only supports a single active output target, that is, only one of the
> Group Output nodes or Viewer nodes in the node tree will be considered active and the rest will be
> ignored."

Two further things that bear on this:

**One old switch no longer decides anything.**

> "use_nodes
>
> Enable the compositing node group. (default False)
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

— Python API documentation, `Scene.use_nodes`, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

**The compositor's 3D viewport preview has a separate property.**

> "use_compositor
>
> When to preview the compositor output inside the viewport (default 'DISABLED')
>
> DISABLED
> Disabled – The compositor is disabled.
>
> CAMERA
> Camera – The compositor is enabled only in camera view.
>
> ALWAYS
> Always – The compositor is always enabled regardless of the view."

— Python API documentation, `View3DShading.use_compositor`, https://docs.blender.org/api/5.1/bpy.types.View3DShading.html — read 2026-07-30.

**Observed, from Cycles' own files.** In the quoted part of Cycles' Post Processing panel, the two pipeline properties drawn are `use_compositing` and `use_sequencer`:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"

    def draw(self, context):
        ...
        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")

        layout.prop(rd, "dither_intensity", text="Dither", slider=True)
```

— `Blender Sources/cycles_5.1/ui.py`, class `CYCLES_PT_post_processing`, lines 1112–1128. Read 2026-07-30. The file assigns `bl_context = "output"`; I did not run Blender to confirm the panel's placement.

**Not covered.** Whether these three are the complete set — the sources nowhere give a checklist. Nothing says what happens when the assigned node group is empty of nodes but present. Nothing says whether the compositor runs once per frame, once per view layer, or once per view. Nothing states whether `use_compositing` is consulted for the sequencer's Compositor strip modifier, which is a separate compositing node group attached to a strip rather than to a scene.

**Outside the interface. Label: inferred.** The two scene controls are exposed as Python properties (`scene.render.use_compositing`, `scene.compositing_node_group`). From that, I infer that they can be set when Blender is driven from outside. The third condition is described in terms of the node tree's contents, but the quoted page also discusses viewport compositing and does not state that "the compositor doesn't run altogether" applies identically to a background render. `View3DShading.use_compositor` is documented only as a viewport preview control.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS and the conditions behave there as described. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. The one operating-system remark in the sources anywhere near this subject concerns the GPU compositor's speed, not whether it runs: "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit" — Manual, https://docs.blender.org/manual/en/5.1/compositing/limitations.html, read 2026-07-30. Note also that `compositor_device` is documented as "Set how compositing is executed (default 'CPU')" (Python API documentation, `RenderSettings.compositor_device`, same address as above, read 2026-07-30).

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, started with no window (`-b`) and with the Creator's own add-ons and preferences kept out of it (`--factory-startup`). Full set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

**Hand test 1 — what the switches are called, and what they start as.** Blender was started on its own empty startup file and asked to print each named property's value, or `MISSING` where it does not exist:

```
BLENDER_VERSION 5.1.2
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = MISSING
RenderSettings.compositor_device = 'CPU'
RenderSettings.compositor_precision = 'AUTO'
```

The documented default of `compositor_device` is confirmed as `'CPU'`, and `compositing_node_group` starts empty, which is condition (a) above.

**Hand test 3 — whether Use Nodes still decides anything.** A 64×64 scene rendering flat green, `[0.4275, 0.7686, 0.3137, 1.0]`, was given a compositor of Render Layers into Invert into Group Output, with Compositing on. `scene.use_nodes` was set each way, read back, and the frame rendered:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back True -> written pixel [0.7882, 0.4588, 0.7882, 1.0]
```

`[0.7882, 0.4588, 0.7882, 1.0]` is the inverted green. Setting the switch off did not take and the compositor ran either way, which turns the documented statement that it is inert into an observed one.

**On gate (d) above, the render engine's post-processing flag.** The finding for that gate came from the second pass and is worth pinning down, because it is named only there and not in the other three versions' findings. Both `bl_use_postprocess` and `bl_use_image_save` are present on the render-engine type in **all four** researched versions, including 4.5 — the two `in RNA` lines above are the 5.1 half of that check. So this is not something new in 5.1; the other versions' researchers simply did not look. It remains a flag an add-on render engine declares, not a setting in the file.

**Hand test 4 — what makes the compositor run in a render started with no window.** A 64×64 Workbench scene with a cube and a camera, whose raw render is black, rendered five ways: a compositor of Render Layers into Invert into Group Output with Compositing on and then off; the Group Output node deleted; a plain red colour fed to the Group Output with no Render Layers node at all; the Group Output node muted; and no compositing node group assigned:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [0.7686, 0.7686, 0.7686, 1.0]
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Black `[0.0, 0.0, 0.0, 1.0]` is the raw render, `[0.7686, 0.7686, 0.7686, 1.0]` is black inverted, `[0.8549, 0.2196, 0.1255, 1.0]` is the red. Three things listed above as not covered are answered: with no active output node the raw render is written; a node group with no Render Layers node in it still runs and its result is what lands; and muting the Group Output node does **not** stop it counting as the active output.

**Hand test 5 — the compositor device, and the Enable Output node.**

```
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
Enable Output node inputs: ['Enable', 'Value'] outputs: ['Value']
RESULT Enable Output set False -> [0.0, 0.0, 0.0, 1.0]
```

The node type is `NodeEnableOutput`, shared with the other node editors rather than compositor-specific. With one between the Invert node and the Group Output and its Enable input turned off, the written frame was black rather than the scene's green camera view — the compositor ran and delivered nothing, rather than being skipped. That answers the question this answer's "Not covered" raises about the active Group Output's image socket being disabled.

**Hand test 14 — a very large frame, and a difference this version alone shows.** A 9000×400 flat-green scene with an Invert compositor was rendered with the Compositor Device set each way. The run was repeated three times and gave the same result each time:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.0, 0.0, 0.0, 1.0]
```

On the CPU the compositor produced its inverted green as usual. On the GPU, at that width, the written frame was black. The same test in 4.5, 5.0 and 5.2 gave the inverted green on both devices. This is on Apple's own graphics; what another maker's card would do cannot be tested with the available equipment.

**Outside the interface. Now observed.** Every case above was a render started from a terminal with no window. The step marked inferred above is now observed for the Compositing switch, the assigned node group and the node group's contents. The audit's concern was well placed — Blender does skip some settings outside the window and documents it when it does — and on this evidence the compositing conditions are not among them.

### Observed — window tests, 2026-08-03

These three are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.1's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are written out in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15, 17 and 21. What follows is what the Creator reported, in their own words.

**Hand test 15 — what the Post Processing panel holds.** The Properties editor's Output tab was opened and the panel called Post Processing opened with it. The Creator gave their 4.5 answer in full and then said 5.0, 5.1 and 5.2 are:

> Identical to 4.5 except the Compositing tooltip reads: "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene."

Their 4.5 answer, which the rest of this therefore stands on, was:

> Two checkboxes — Compositing and Sequencer, both on by default. One slider called "Dither" set to 1.00 by default, a float, adjustable from 0 to 2. … Sequencer tooltip: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

So the panel really does hold what the manual's screenshot and Cycles' own file say it holds — the two switches and the Dither slider, nothing else — and the Compositing switch is on by default. Its tooltip names the same condition as gate (b) above: a compositing node group assigned to the scene. The test also asked whether the panel's own header carries a checkbox beside the words "Post Processing". The Creator did not say, so that part is not answered.

**Hand test 17 — what the Compositor editor's header offers.** The Compositing tab along the top of the window was clicked, in a file where nothing had been set up. The Creator's report, given for 5.0, 5.1 and 5.2 together as identical across all three:

> No checkbox. Same four menus, but now there's a Node Tree Sub-Type dropdown to their left with two options, "Scene" (default) and "Sequencer". Center-top is the ordinary data-block selector plus a "+New" button (the standard pattern used throughout Blender's node editors). Nothing exists until you click +New, which creates a real data-block called "Compositor Nodes" with the same default setup as 4.5, except the final node is called Group Output instead of Composite.

"Same four menus" refers to the View, Select, Add and Node menus the Creator listed for 4.5. So there is no "Use Nodes" checkbox in this header, which is what the release-note line quoted above under "What no longer decides it" says was removed. Gate (b)'s assignment slot appears here as a data-block selector with a New button, and nothing is assigned in a fresh file until New is clicked.

**Hand test 21 — how a second output node is marked.** In the Compositing editor a Render Layers node was wired to a Group Output node, a second Group Output node was added, and the Render Layers node wired to that one too. The Creator's report, given for 5.0, 5.1 and 5.2 together as identical:

> The last-selected node is the active one. The inactive one shows a warning above its header: a triangle with an exclamation mark, followed by "Unused Output".

So gate (c)'s "active" output node is picked by last selection, and the one that is not active carries a warning reading "Unused Output". The test also asked whether either node's own header carries a control for making it the live one. The Creator did not name one, so that part is not answered.

**Outside the interface.** These three settle nothing about it. Each is a look at what a Blender window shows, so none of them speaks to a render driven from outside.

**Operating system.** All three were done on macOS. All three were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Observed — the same tests on a rented Linux machine, 2026-08-06

Every test above was run again, unchanged, on a second computer: a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own. The Blender used was 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, started with no window (`-b`) and with `--factory-startup`, except where the test says a window was opened. The scripts were the same files as on the Mac, copied across. The full set-up is in `Audits/Blender Research/The compositor/Hand tests — Linux.md`. **These are second results, not replacements for the ones above.**

One thing to know before reading the figures. Two of the reference colours come back one step of 255 lower in one channel on this machine than on the Mac: magenta reads `[0.7882, 0.4549, 0.7882, 1.0]` here against `[0.7882, 0.4588, 0.7882, 1.0]` there, and yellow reads `[0.7804, 0.7529, 0.4196, 1.0]` here against `[0.7804, 0.7529, 0.4235, 1.0]` there. Every other colour is identical to the last figure. Which colour arrived is the same on both machines in every test.

**Hand test 1 — what the switches are called, and what they start as.** What came back, word for word:

```
BLENDER_VERSION 5.1.2
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = MISSING
RenderSettings.compositor_device = 'CPU'
RenderSettings.compositor_precision = 'AUTO'
```

Identical to the Mac's, line for line — including `compositor_device` starting at `CPU`, which is what the API documentation quoted above says.

**Hand test 3 — whether Use Nodes still decides anything.** What came back, word for word:

```
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.7882, 0.4549, 0.7882, 1.0]
RESULT set scene.use_nodes=False -> reads back True -> written pixel [0.7882, 0.4549, 0.7882, 1.0]
```

The same as the Mac: the switch refuses to go off, and the compositor runs either way. Both render-engine flags are present.

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

**Hand test 14 — the compositor above the size 5.2's documentation names. This is the one place the two machines have something to say about the card.** The script that runs both devices in one Blender **stopped part-way through, twice.** Both times it ended like this:

```
00:01.658  render           | Saved: '/root/smoozy/The compositor/work/out_8k/5.1/big_CPU.png'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
Writing: /tmp/blender.crash.txt
```

So Blender 5.1.2 stopped after writing the processor version and before finishing the card version. The crash file begins:

```
# Blender 5.1.2, Commit date: 2026-05-18 13:43, Hash ec6e62d40fa9

# backtrace
/opt/blender/blender-5.1.2-linux-x64/blender() [0x10b31ed]
/opt/blender/blender-5.1.2-linux-x64/blender() [0x784f67]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520) [0x7dfa79642520]
```

The same build was then split so that each device gets its own Blender, and each was run twice. What came back, word for word:

```
===== 5.1, device CPU, run 1 =====
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
===== 5.1, device CPU, run 2 =====
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4549, 0.7882, 1.0]
===== 5.1, device GPU, run 1 =====
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.0, 0.0, 0.0, 1.0]
===== 5.1, device GPU, run 2 =====
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.0, 0.0, 0.0, 1.0]
```

At 9000 pixels wide, 5.1 writes a black frame on the card and the composited picture on the processor. The Mac found the same thing on Apple's own graphics; this machine finds it on an NVIDIA card. Both runs of both devices agree. The crash of the two-device script is recorded here rather than tidied away, because it is a result too.

**Hand test 15 on the Linux list — whether a compositor tree gives the same picture on the card as on the processor.** This test has no Mac counterpart. A 320×240 Workbench scene holding a sphere, a cube and a cone under the studio light was given four compositor trees in turn — an Invert; a Blur of size 12; a Scale set to absolute 160×120; and a chain of Invert into a Blur of size 6 into an Alpha Over blending the blurred copy back over the original at half strength. Each was rendered twice, once on each device, and the two written files compared value by value, all 307,200 of them. The first three trees were run by one command and the fourth by a second command; what came back is quoted below as one block, the two commands run one after the other. Word for word: *(Linux citation check, 2026-08-06: the block was previously headed "word for word" with no sign that it stitches the output of two separate commands. Said plainly.)*

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

All four trees came back identical to the last figure on the two devices, at 320×240. **That is not the same claim as the test above it:** at 9000 pixels wide the card and the processor disagree completely in 5.1. This settles Linux and one NVIDIA card at the small size, and says nothing about macOS, Apple's own graphics, or a card of another make.

**One error is recorded here because it happened before that run.** The fourth tree was first built around a Mix node named `CompositorNodeMixRGB`, and came back:

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

These are interface observations, like the three above. They were driven over SSH on the rented Linux machine, which has a display of its own, and photographed with Blender's own screenshot command. **No click or keystroke was used** — everything was done by asking Blender to change an area, pop a panel or open a menu from inside itself. The pictures are on that machine under `/root/smoozy/The compositor/work/shots/`, and what each shows is written out in words here, so nothing depends on the picture surviving.

**Hand test 15 — what the Post Processing panel holds.** The panel holds a row labelled **Pipeline** with two checkboxes, **Compositing** and **Sequencer**, both ticked; and below it a slider labelled **Dither** reading **1.00**. Nothing else. That is what the Creator reported on the Mac.

**And the part the Mac left unanswered is now answered.** The same editor was scrolled to the bottom and photographed. The Post Processing panel's own header reads `> Post Processing` with only the fold arrow beside it — **no checkbox**. The Stereoscopy header higher up carries a checkbox in the same picture, so a checkbox in a header shows up in these pictures where one exists.

**Hand test 17 — what the Compositor editor's header offers.** Left to right the header holds: the editor-type dropdown; a second dropdown reading **Scene**; the menus **View**, **Select**, **Add** and **Node**; then a node-group chooser with a small browse button, a **+** and a button reading **New**; then a pin. **There is no Use Nodes checkbox.** That matches the Creator's report on the Mac.

**Hand test 21 — how a second output node is marked.** A Render Layers node was wired to two Group Output nodes. Both have the same grey header, and the second one carries a small strip directly above it holding a warning triangle and the words **Unused Output**. The first carries nothing.

**And the part the Mac left unanswered is now answered.** At twice magnification, **neither node's own header carries any control for making it the live one**. The header holds the fold arrow and the node's name, and nothing else.

**Outside the interface.** These three settle nothing about it, exactly as their Mac counterparts do not.

**Operating system.** These three were done on Linux and settle Linux. Together with the Mac results above them, macOS and Linux now both have an answer and the two agree. Windows stays untested.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Label: not in the sources.**

**Ambiguity, flagged.** I read "the place where a scene's rendered frames are written" as the scene's render output destination on disk — Properties ‣ Output ‣ Output ‣ File Path, i.e. `scene.render.filepath`. The other reading available is "the compositor's output target" (the Group Output node that carries the final image); I answer that one briefly at the end.

**On the reading I took.** I found no switch, checkbox, enum value or property in the 5.1 manual, API documentation or release notes that turns the render output location off. What the sources do say:

The location is a plain text field with no companion toggle:

> "File Path
>
> Choose the location to save rendered frames."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)"

— Python API documentation, `RenderSettings.filepath`, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

And for animations, writing is described as automatic:

> "animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The nearest things to an "off" that the sources do document, none of which is the same as switching the location off:

- **A single-frame render operator has a control for writing the image.** "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)" — Python API documentation, `bpy.ops.render.render`, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. The same parameter, with the same wording, exists on `bpy.ops.render.opengl`. The quoted words establish that the control exists; they do not state the result of omitting it.
- **Overwrite can be turned off**, so that frames already on disk are not rewritten: "Overwrite existing files while rendering (default True)" — Python API documentation, `RenderSettings.use_overwrite`, address as above, read 2026-07-30. The manual's "Primitive Render Farm" hint uses exactly this: "Disable Overwrite, enable Placeholders in the Render Output panel." (Manual, output.html, read 2026-07-30.)
- **A broken path template stops the render rather than switching it off**: "if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors." — Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.
- **A separate, independent destination exists in the compositor.** "The File Output node writes one or more images to disk during rendering or compositing." — Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30. The manual recommends it "to save raw passes or intermediate steps without overwriting the main render result", but does not say the main render result can be turned off.

**Where I looked, to justify "not in the sources".** Whole-tree text searches of `manual_5.1`, `api_5.1` and `release_notes_5.1` for: `output path`, `Output Path`, `render_output`, `use_output`, `write output`, `save output`, `Save Result`, `output enabled`, `output is disabled`, `Disable Output`, `disable the output`, `turn off output`, `no render output`, `nothing is written`, `not be written`, `no file will be`, `not be saved`, `does not save`, `do not save`, `without writing`, `skip writing`, `discard the render`, `empty path`, `blank path`, `None format`. I also read in full: the Output panel page, the Post Processing page, the Format page, the Rendering Animations page, the File Paths (path templates) page, the command-line arguments and command-line render pages, and the File Output node page. And I listed every property on `bpy.types.RenderSettings` (there are 100 or so) and read the description of each boolean: the only ones touching output writing are `use_file_extension`, `use_overwrite`, `use_placeholder`, `use_render_cache`, `use_compositing` and `use_sequencer`. None of them switches the destination off.

**On the other reading** (the compositor's output target): that *can* be absent, and the documented consequence is that the compositor is skipped, not that writing stops — see the quote in answer 1(c). Separately, the compositor has an **Enable Output** node: "The Enable Output node toggles the visibility and value of a socket for a Node Group outputs… When Enable is False, the output is disabled and not available to the group output." (Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/enable_output.html — read 2026-07-30.) The sources do not say what that does to the scene's render output when it is applied to the compositor's own group outputs, so I make no claim about it.

**Not covered.** What Blender actually does when `filepath` is the empty string (the API's stated default) — the sources do not say. Whether a File Output node still writes when the scene's own output would be skipped. Whether any add-on or command-line option can suppress writing. Whether the render result is still held in memory when nothing is written.

**Outside the interface. Label: documented** for the controls quoted here; **not in the sources** for one general off behaviour. The `write_still` control is an operator parameter, so the source documents a writing control available through the Python API. The command-line documentation shows `-o, --render-output <path>` as a path setting with no quoted "none" value, and it states: "-f, --render-frame <frame> — Render frame <frame> and save it." (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.) These quotes do not establish one general writing behaviour for every way of driving Blender from outside.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. The only related remark in the sources is about which file formats exist at all: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (Manual, command-line arguments page, address as above, read 2026-07-30.)

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.1.2 started from a terminal with no window and with `--factory-startup`.

**Hand test 1 — is there an off switch at all.** Blender was asked for `save_output` on the render settings, the property Blender 5.2's manual describes as turning automatic saving off:

```
RenderSettings.save_output = MISSING
```

The property does not exist in 5.1. The same question was asked of all four versions and it appears only in 5.2, so the answer above is a fact about 5.1 rather than a search that stopped early.

**Hand test 10 — what happens with the File Path emptied.** A two-frame 64×64 file was saved with `render.filepath` set to the empty string, then rendered as an animation from a terminal with the working folder set to the folder holding the file. The command was `blender -b --factory-startup emptypath.blend -a`. What came back, word for word:

```
00:00.362  blend            | Read blend: ".../out_write/5.1/emptypath.blend"
00:00.366  render           | Rendering animation (frames 1..2)
00:00.366  render           | Rendering frame 1
00:00.366  render           | Start rendering: Scene, ViewLayer
00:00.366  render           | Engine: Workbench
00:00.492  image            | ERROR Couldn't create directory for file 0001.png: No such file or directory
00:00.492  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
00:00.492  render           | Time: 00:00.12
```

No file was written and the run stopped after frame 1. That answers the thing recorded above as not covered: an empty File Path in 5.1 is a failure rather than a way of switching writing off.

**Hand test 10 — a muted File Output node.** Two otherwise identical two-frame files, one with a File Output node aimed at an empty folder and muted, one with the same node unmuted, each rendered as an animation from a terminal. The folder listings afterwards:

```
muted_fileoutput  -> [end]
unmuted_fileoutput-> file_nameBeauty0001.png file_nameBeauty0002.png [end]
```

Muting the node stops it writing. That settles the step this answer flagged rather than asserted.

**Hand test 4 — what is written when the compositor has no output node.** With the Compositing switch on, the Group Output node deleted, and a Render Layers node feeding an Invert node that goes nowhere:

```
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

A file was written and it held the scene's raw render, so the compositor being skipped does not stop the write.

**Hand test 5 — the Enable Output node.** With an Enable Output node between the compositor's last node and the Group Output, its Enable input turned off, the written frame was black rather than the scene's green camera view: `RESULT Enable Output set False -> [0.0, 0.0, 0.0, 1.0]`. So it does not stop writing; it empties what is written.

**Outside the interface. Now observed.** Every case above was driven from a terminal with no window, so the documented command-line writing behaviour is observed as well.

### Observed — window tests, 2026-08-03

This one is an **interface observation**, unlike the terminal tests above. The Creator ran it by hand in Blender 5.1's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-up is in `Audits/Blender Research/The compositor/Hand tests.md`, entry 16.

**Hand test 16 — what the Output panel holds, and whether its header has a checkbox.** The Properties editor's Output tab was opened and the panel called Output looked at, header first. The Creator answered in two parts: words for the Media Type control, and a screenshot for the rest, saved as `5.0 and 5.1.png` in a folder called `Test 16` on their own computer — one picture for both versions, because the panel looks the same in each.

Their words:

> 5.0, 5.1, 5.2: The Media Type dropdown has three items: Image; Multi-Layer EXR; Video.

What the picture shows, top to bottom:

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

Two things this settles. The Media Type control the 5.1 manual's text describes is really there, with the three items the text names, so where the manual's own screenshot of that page disagrees with its words the screenshot is the part that is out of date. And there is **no on/off checkbox in this panel's header in 5.1** — which is what this answer says from the sources, and it is now seen rather than argued from silence. The same look in Blender 5.2 does show one; 4.5, 5.0 and 5.1 do not.

**Outside the interface.** This settles nothing about it. It is a look at what a Blender window shows.

**Operating system.** It was done on macOS. It was done again on the rented Linux machine, and the two agreed; the Linux result is in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`. **Second results, not replacements.**

**Hand test 1 — whether an output-off switch exists.** `RenderSettings.save_output = MISSING`, the same as on the Mac. There is no such property in 5.1.

**Hand test 3 — whether the render-engine save flag is a 5.1-only thing.** `bl_use_image_save in RNA: True` and `bl_use_postprocess in RNA: True`, the same as on the Mac, and the same in every other version.

**Hand test 10 — whether the place frames are written can be switched off.** Four files were built and rendered from the command line. What came back, word for word:

```
BLENDER_VERSION 5.1.2
SAVED emptypath.blend with render.filepath = ''
render.save_output MISSING in this version; file saved with the same tree and no such switch
SAVED saveoff.blend
SAVED mutedfo.blend with the File Output node muted
SAVED unmutedfo.blend
END_OF_BUILD
----- emptypath.blend -a
00:00.754  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
----- saveoff.blend -o cli_saveoff_ -F PNG -a
00:00.718  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/saveoff_fileoutput/file_nameBeauty0001.png'
00:00.719  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/cli_saveoff_0001.png'
00:00.724  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/saveoff_fileoutput/file_nameBeauty0002.png'
00:00.725  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/cli_saveoff_0002.png'
----- mutedfo.blend -a
00:00.701  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/muted_main_0001.png'
00:00.722  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/muted_main_0002.png'
----- unmutedfo.blend -a
00:00.724  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/unmuted_fileoutput/file_nameBeauty0001.png'
00:00.725  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/unmuted_main_0001.png'
00:00.729  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/unmuted_fileoutput/file_nameBeauty0002.png'
00:00.730  render           | Saved: '/root/smoozy/The compositor/work/out_write/5.1/unmuted_main_0002.png'
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

**Hand test 4, case 2 — removing the compositor's output point.** Written out in full under question 1: the raw render still reached the file.

### Observed — the same window test on the rented Linux machine, 2026-08-06

**Hand test 16 — whether the Output panel's header carries a checkbox.** The Properties editor was put on the Output tab, made to fill the window, scrolled so the panel and its header were both visible, and photographed with Blender's own screenshot command. The header reads `∨ Output` with **no checkbox** beside it. The panel holds, top to bottom: the file path field reading `/tmp/` with a folder button; **Saving** with **File Extensions** (ticked) and **Cache Result** (unticked); a **Media Type** dropdown reading **Image**; **File Format** reading **PNG (.png)**; **Color** as **BW / RGB / RGBA** with RGBA chosen; **Color Depth** as **8 / 16** with 8 chosen; **Compression** reading **15%**; **Image Sequence** with **Overwrite** (ticked) and **Placeholders** (unticked); and two closed sub-panels, **Color Management** and **Pixel Density**. That is what the Creator reported on the Mac, Media Type included.

**Outside the interface.** This settles nothing about it.

**Operating system.** It was done on Linux and settles Linux. With the Mac result above it, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Label: documented.**

The sources do not support choosing only one side of the question. A scene has a compositing node group assigned to it, while the node group is a reusable data-block that can be used by more than one scene or linked or appended from another file.

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

— Python API documentation, `Scene.compositing_node_group`, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

> "This adds a compositing node tree data block to the scene, which can then be reused
> across different scenes or linked/appended from other blend-files."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

That the tree is a data-block in its own right, and where it lives in the file:

> "class bpy.types.CompositorNodeTree(NodeTree)
>
> Node tree consisting of linked nodes used for compositing"

with base classes "bpy_struct, ID, NodeTree" — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeTree.html — read 2026-07-30. Node trees are listed in the file's data under `node_groups`: "Node group data-blocks (default None, readonly)" — Python API documentation, `BlendData.node_groups`, https://docs.blender.org/api/5.1/bpy.types.BlendData.html — read 2026-07-30.

The operators confirm the "assigned to a scene" framing:

> "bpy.ops.node.new_compositing_node_group(\*, name='')
>
> Create a new compositing node group and initialize it with default nodes"

> "bpy.ops.node.duplicate_compositing_node_group()
>
> Duplicate the currently assigned compositing node group."

— Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.node.html — read 2026-07-30.

One thing worth stating because it complicates the simple picture: a compositing node group is not only used by scenes. The video sequencer has a Compositor strip modifier that takes one:

> "Compositor modifier is powerful tool to create arbitrary effects. To use it, you need to open compositor editor
> and change node subtree to sequencer. Then you can add new node tree and use it for any strip. These node trees
> can be marked as asset and reused across multiple .blend files."

— Manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strip_modifiers.html — read 2026-07-30. The API lists `SequencerCompositorModifierData.node_group` alongside `Scene.compositing_node_group` as the places a `NodeTree` is referenced from (Python API documentation, https://docs.blender.org/api/5.1/bpy.types.NodeTree.html — read 2026-07-30).

**Not covered.** Whether a scene can hold more than one compositing node group (the property is a single pointer, which suggests not, but the sources do not say it). What happens to a scene that points at a node group also pointed at by another scene when one of them is rendered. Whether the node editor's "Pinned" option changes anything about ownership — it is documented as a display convenience only: "the editor always displays the currently selected node tree, regardless of changes in the active object or scene" (Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html — read 2026-07-30).

**Outside the interface. Label: inferred.** The assignment is exposed as a Python property on the scene. From that, I infer that it can be read and changed from outside the interface. Nothing quoted here establishes whether any related behaviour differs without a window or editor.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. The sources say nothing that makes this depend on the operating system.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.1.2 started from a terminal with no window and with `--factory-startup`.

**Hand test 5 — two scenes pointing at one node group.** Two scenes were made, A rendering blue `[0.0745, 0.3333, 0.8196, 1.0]` and B rendering green `[0.4275, 0.7686, 0.3137, 1.0]`. A was given a compositing node group holding a Render Layers node whose Scene was set explicitly to A, feeding an Invert node into the Group Output. B was then pointed at that same node group, and each scene rendered in turn with Compositing on:

```
RESULT sharing one tree between two scenes -> True
RESULT shared tree, rendering scene A -> [0.7804, 0.7529, 0.4235, 1.0]
RESULT shared tree, rendering scene B -> [0.7804, 0.7529, 0.4235, 1.0]
```

`[0.7804, 0.7529, 0.4235, 1.0]` is blue inverted. Both scenes wrote the same picture: A's render, put through the shared tree. So the sharing works, and **a Render Layers node inside a shared tree follows the scene named on the node, not the scene being rendered.** That is exactly the item this answer records as one it could not settle.

**Hand test 12 — whether choosing a scene from outside picks that scene's compositor.** One file, `scenes.blend`, was saved holding two scenes: A rendering blue with a compositor that inverts it to yellow, and B rendering green with a compositor that inverts it to magenta, both with Compositing on. Two renders were run from a terminal, `blender -b --factory-startup scenes.blend -S A -o .../scene_A_ -F PNG -f 1` and the same with `-S B`, and the written frames read back:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [0.7804, 0.7529, 0.4235, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0]
```

Yellow for A, magenta for B: `-S` picks the scene and that scene's own compositor runs.

**Outside the interface. Now observed.** Both tests ran from a terminal with no window, so the assignment is readable and in force there. The step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.1's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 17 and 20.

**Hand test 17 — what the Compositor editor's header offers.** Quoted in full under question 1. The Creator's report, given for 5.0, 5.1 and 5.2 together:

> No checkbox. Same four menus, but now there's a Node Tree Sub-Type dropdown to their left with two options, "Scene" (default) and "Sequencer". Center-top is the ordinary data-block selector plus a "+New" button (the standard pattern used throughout Blender's node editors). Nothing exists until you click +New, which creates a real data-block called "Compositor Nodes" with the same default setup as 4.5, except the final node is called Group Output instead of Composite.

Three things in that bear on this answer. The header's middle is a data-block selector with a New button — the ordinary way a shared data-block is picked in Blender, which is what this answer says a compositing node tree is. Clicking New makes one with a name of its own, "Compositor Nodes", matching the operator `bpy.ops.node.new_compositing_node_group` quoted above. And the dropdown to the left offers two sub-types, "Scene" and "Sequencer" — the same two places this answer says such a tree gets used, a scene's own assignment and the sequencer's Compositor strip modifier.

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

Run again, unchanged, on the rented Linux machine, with Blender 5.1.2. **Second results, not replacements.**

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

**Outside the interface.** These settle nothing about it; the terminal tests were both run with no window.

**Operating system.** All were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**Ambiguity, flagged.** "Takes in another scene's rendered result" can mean two different things in 5.1, and they have different answers. I take the compositor reading as the one meant, because this is the compositor sheet, and answer the other briefly after it.

### Reading A — a Render Layers node in scene 1 pointing at scene 2 (the one I take as meant)

**Label: inferred** for what reaches scene 1. **Not in the sources** for whether scene 2's compositor runs. The quoted words indicate that scene 1 gets scene 2's raw render rather than its composited output. They do not establish whether scene 2's compositor runs and its result is then unused.

The node's own page says what it does and then, in a hint, distinguishes the two things by name:

> "Renders a View Layer and reads its
> Passes into the compositing node graph."

> "Hint
>
> To use the compositing output from another scene rather than its "raw" render output,
> first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
> then load those images into the Compositor of the current scene using the
> Image Node."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

**The step I took.** The source calls what the node supplies scene 2's *raw* render output and says that getting scene 2's *compositing* output instead requires a detour through files on disk. From that I infer that the compositor's result does not reach scene 1 through this node. It does not follow that scene 2's compositor does not execute at all.

That scene 2 is genuinely **rendered** (its view layer, at least) is supported directly:

> "if Freestyle is used for two or more view layers
> (possibly in different scenes through the Compositor),
> then the cached view map for one view layer is replaced by a new view map
> for another view layer"

— Manual, https://docs.blender.org/manual/en/5.1/render/freestyle/view_layer/freestyle.html — read 2026-07-30. **Documented** that the page contemplates view layers in different scenes through the Compositor; **inferred** that this is the Render Layers node doing it.

The API's one-line description of the node agrees on what arrives: "Input render passes from a scene render" — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

### Reading B — a Scene strip in scene 1's video sequencer referencing scene 2

**Label: documented** for the result used. When Compositing is also enabled, the Scene strip uses the output of the Compositor. The source does not say whose Compositing setting governs, and it does not state whether that compositor is newly run at this point.

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

Note that the strip has its own choice of what to take from the other scene:

> "Input
>
> Input type to use for the Scene strip.
>
> Camera:
>
> Use the Scene's 3D camera as input.
>
> Sequencer:
>
> Use the scene's Sequencer timeline as input, allowing one scene to reuse
> another scene's edit (instead of taking the render output from the scene)."

and

> "Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

— Manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

**Not covered.** For reading A: nothing about what happens if scene 2's Render Layers node in turn points at scene 3. Nothing about whether scene 2's *Post Processing ‣ Compositing* setting is even consulted in this case. Nothing about whether scene 2's render is written to scene 2's output path as a side effect (I found no statement either way). For reading B: which of the two scenes' Post Processing settings governs, when they disagree — the quote says "If Compositing is also enabled" without saying whose. Nothing in either reading covers ordering, caching or how many times scene 2 is rendered if two nodes reference it.

**Outside the interface. Now observed.** The tests below were run from a terminal with no window, and gave the results described. The quoted sources still do not establish what executes in a background render; the tests do.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.1.2 started from a terminal with no window and with `--factory-startup`.

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

The controls give the three colours to read the result by. What A wrote was B's raw render, both times. B's File Output folder was empty both times, though rendering B directly fills it. This is the side-effect test the audit asked for: it separates which result arrived from whether B's compositor did any work, and on this evidence it did none, whichever way B's own Compositing switch was set. That also answers the item recorded above as nothing about whether scene 2's Post Processing setting is consulted: it makes no difference to either signal.

**Hand test 8 — a Scene strip pointing at another scene.** The same two scenes, except A takes B in through a Video Sequencer Scene strip with A's Sequencer switch on, and A's own compositor is a plain pass-through. Rendered four times, once for each combination of the two scenes' Compositing switches, with B's File Output folder cleared before each run:

```
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['file_nameBeauty.png']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.7882, 0.4588, 0.7882, 1.0] ; files in B's File Output folder: ['file_nameBeauty.png']
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

This settles which reading of "If Compositing is also enabled" is meant. It is the strip's own scene's switch: the written frame follows B's and is unaffected by A's, and B's File Output node writes exactly when B's switch is on, which shows B's compositor genuinely runs in that case.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Both tests were run again, unchanged, on the rented Linux machine, with Blender 5.1.2 started with no window and with `--factory-startup`. **Second results, not replacements.**

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

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

Three parts, with different labels.

### (a) What it arrives as — **documented**

It arrives through the Render Layers node, as a set of output sockets: an image, an alpha, and one socket per enabled pass.

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

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

What "an image" means to the compositor is spelled out. It is data with a type and a place in space:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats:" — Float, Vector, Color.

> "images are not only represented by their size, but also by their transformation in
> that space, much like 3D objects have transformations. An identity transformation represents an
> image that is centered in space. The rectangular area occupied by an image in that space as defined
> by its transformation and size is called the Domain of the image."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30.

And one image at a time:

> "The Compositor processes one image at a time, no matter what kind of input you provide."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

### (b) Can it be told apart from anything else there — **not in the sources**

The sources describe no marker, tag, name or identity that travels with an image once it leaves a node. What they describe is that you can tell the *node* apart. The Render Layers node is documented as bringing a scene render in ("Input render passes from a scene render" — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html, read 2026-07-30), and it carries two properties naming what it is reading:

> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render."

— Manual, render_layers.html, address as above, read 2026-07-30.

Other imagery can come from the separately named Image node: "The Image node injects any image format that is supported by Blender" (Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/image.html, read 2026-07-30).

What a socket shows about the data it carries is its **type**, not its origin: "Each socket is color-coded depending on what type of data it handles" (Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/parts.html — read 2026-07-30). The Image Info node reports geometry rather than identity:

> "The Image Info node outputs spatial and transformation information about an image in the compositor."
> — outputs: "Dimensions", "Resolution", "Location", "Rotation", "Scale".

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/image_info.html — read 2026-07-30.

**Where I looked, to justify "not in the sources".** Whole-tree searches of `manual_5.1` and `api_5.1` for the wording such a feature would use, and reading in full the pages the searches pointed to: the Compositor System page (data types, domains, implicit conversion), Node Parts (sockets, socket colours, socket shapes, structures), Node Editors (overlays, wire colours, context path, timings), the Render Layers, Image, Image Info, Sequencer Strip Info, Viewer, Group Output and File Output node pages, and the compositing Sidebar page. Nothing describes an identity or label attached to image data as it flows.

### (c) Can it be followed forward — **documented** for the two link shapes, **not in the sources** for whether the render's origin can be followed through them

The sources document a pass-through node and links that split. They do not say that an image's origin remains identifiable or that an arbitrary path can be followed across those links.

**Pass-through shape — inferred.** The Reroute node is described as organizational and supports one input with several outputs. From that, I infer that it passes the connection through. The quoted words do not state that the values are unchanged:

> "A node used primarily for organization.
> Reroute looks and behaves much like a socket on other nodes in that it supports one input
> connection while allowing multiple output connections."

— Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/types/layout/reroute.html — read 2026-07-30.

**Split into more than one.** A single output socket can feed several places at once:

> "While multiple links can route out of an output socket, typically a single link can be attached to an input socket,
> that is unless the input is a multi-socket input with looks like a pill shaped socket."

— Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html — read 2026-07-30.

**Following it with your eyes** is documented as a per-node preview and as the Viewer node:

> "Previews are an overlay that shows a small image above the node displaying the node result.
> Not all nodes support previews, but the ones that do can be toggled using
> the / icons in the top right-hand corner of the node next to the title."

— Manual, parts.html, address as above, read 2026-07-30.

> "The Viewer node is used to preview image data or value maps anywhere within a compositor node graph.
> It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/viewer.html — read 2026-07-30.

**Following it programmatically** is not addressed by any page I found; the API pages I read describe node and socket types, not link traversal helpers.

**Not covered.** Whether anything downstream can report "this came from the render". Whether a muted or collapsed node counts as passing data through unchanged. *(Citation check, 2026-08-03: this line previously ended "— the sources describe muting nowhere I found." That is wrong. Muting is described in the 5.1 manual at https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html: "Muting a node removes its contribution to the node tree, and makes all links pass through it without change." and "A muted link acts as though it's no longer there". The second-pass document for this version quotes both, and the note at the end of this answer already says so. The false claim of silence is struck.)* What happens to identity across a node group boundary. Whether passes stay associated with the image they came from once separated onto different sockets. Whether the compositor's own precision setting (half by default per the Compositor System page, "all of which are stored in half precision formats", against `compositor_precision` "Auto – Full precision for final renders, half precision otherwise") changes what arrives — the two statements sit oddly together and I did not resolve them.

**Outside the interface. Label: inferred** for the stored node links. Parts (a) and (c-links) are described as contents of the node tree, so I infer that those links remain present however Blender is driven. The previews and Viewer node are described as visual inspection aids. The quoted sources do not establish what data either exposes in a background render. The manual says only that "adding a Viewer node will have no effect on the viewport render if there is a Group Output node, since the priority is given to Group Output nodes" (Manual, compositor_system.html — read 2026-07-30).

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The test below was run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. **The exception is hand test 19, what hovering a link shows.** That was attempted on the Linux machine and could not be settled there, so that reading rests on the Mac alone — see the two sections further down. Windows was not tried, because no Windows machine has run it. The sources say nothing that makes any of this depend on the operating system. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. Hand test 19 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)*

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.1.2 started from a terminal with no window and with `--factory-startup`.

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

Four things this settles. The two node types carry different identifiers, so a program can tell the render's entry point from an image file's. The `layer` property reports `'ViewLayer'`. The walk ran to completion, reaching both destinations through the Reroute, with the muted link and the muted node each flagged on the way — so the pass-through and fan-out shapes this answer describes do behave as a trace needs. And `is_active_output` on the Group Output node reads `True`, so which output node is the live one is a property of the node in the file, not something derived from an open editor — which is the gap this answer's "outside the interface" line says it could not close.

**On muting, which this answer says it found nowhere.** Muting is real and readable: the muted node above reports `mute = True` and its `internal_links` reports `[('Image', 'Image')]`, and a muted link reports `is_muted=True`. The second pass found the manual pages for both; the observed behaviour is here.

**Hand test 2 — what the Split node offers.**

```
ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']
```

**Outside the interface. Now observed.** The whole of hand test 11 ran in a session with no window open. The step marked inferred above is now observed, and the open question about whether "active" is stored in the file is answered by the `is_active_output` line.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal test above. The Creator ran them by hand in Blender 5.1's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 19 and 21.

**Hand test 19 — what hovering a link shows.** A small tree was built in the Compositing editor — a Render Layers node and an Image node with an image loaded, each feeding its own Viewer node — one frame was rendered and the render window closed, and then the link out of each node was hovered. The Creator's report for 5.0 and 5.1 together, against the "Image." with a trailing period they saw in 4.5:

> 5.0, 5.1: Same text, "Image", but no trailing period.

So something does appear in 5.1, and it is the socket's name. The Creator reported one wording and did not say it differed between the two links, so this record cannot say whether the Render Layers link and the Image link showed different things. Nothing in what was reported names where the image came from, which is what part (b) above says the sources describe no way of doing. Blender 5.2 shows a good deal more here; 5.1 shows the name alone.

**Hand test 21 — how a second output node is marked.** Quoted in full under question 1. With two Group Output nodes in one tree, the last-selected one is the active one, and the inactive one carries a warning above its header — a triangle with an exclamation mark, then the words "Unused Output". That is a second way of telling which output is the live one, beside the `is_active_output` reading in hand test 11 above, and this one is visible on the node itself.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Test 21 was done again on the rented Linux machine and agreed; test 19 was attempted there and could not be settled. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.1.2. **Second results, not replacements.**

**Hand test 2 — which compositor nodes exist, and what the Split node's sockets are.** The printed block is long; the lines that carry the answer came back word for word as:

```
BLENDER_VERSION 5.1.2
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

Identical to the Mac's, socket for socket, including the Render Layers node offering only Image and Alpha in 5.1 and the Split node having one output.

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

**Hand test 19 — what hovering a link shows. This could not be settled here, and no answer is written in.** A Blender window was opened on the rented Linux machine with the same tree the Mac test used, one frame was rendered, and the pointer was put over the link out of the Render Layers node, over that node's own Image output socket, and over the link out of the Image node. It was tried four ways: with Blender moving its own pointer to coordinates it worked out from the node positions; with the pointer moved from outside in several small steps and held; with dwells of 2.5 seconds and of 8 seconds; and with the picture taken by Blender itself on a signal, so the pointer was checked to be in place first. No display appeared in any of them, and none appeared over a control that certainly has a tooltip either. **That is a fault in the run, not a finding about Blender.** The machine's display is shared with other topics being tested at the same moment: one check printed `AT 438,227 -> x:573 y:68`, meaning something else moved the pointer between putting it there and reading it back. So the socket-inspection reading for 5.1 still rests on the Mac alone. What to run when the machine is free is written out in `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

**Outside the interface.** These settle nothing about it.

**Operating system.** Test 21 now has an answer from both machines and they agree. Test 19 has an answer from the Mac only; Linux and Windows are both open for it.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Label: documented (for the render region), plus one inferred item.** Yes — at least one other thing, and possibly two.

**The render region, when cropped.** Two settings on the same panel as resolution:

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

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30.

The region itself is defined in the 3D viewport and does reach the final render:

> "If you define one while in Camera View,
> it will apply not just to the viewport, but also to the final render."

> "Render regions only apply to the viewport when using Cycles, not when using EEVEE.
> However, they always affect the final render."

— Manual, https://docs.blender.org/manual/en/5.1/editors/3dview/navigate/regions.html — read 2026-07-30.

In the API these are `use_border` ("Render a user-defined render region, within the frame size (default False)"), `use_crop_to_border` ("Crop the rendered frame to the defined render region size (default False)") and the four `border_min_x` / `border_max_x` / `border_min_y` / `border_max_y` floats — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

*(Citation check, 2026-08-03: the label at the head of this answer covered the render region as documented outright. One step remains and is now marked: every quoted sentence says "rendered image" or "rendered frame", never "written" or "saved", so going from "the rendered frame is cropped to the region size" to "the file on disk is that size" is **inferred**. The second-pass document for this version flags the same step. The hand test below measures the written file and settles it.)*

**Multi-view and stereo output — documented that it changes how many files, inferred that it changes dimensions.**

> "INDIVIDUAL:
>
> Individual.
>
> Individual files for each view with the prefix as defined by the scene views.
>
> STEREO_3D:
>
> Stereo 3D.
>
> Single file with an encoded stereo pair.
>
> MULTIVIEW:
>
> Multi-View.
>
> Single file with all the views."

— Python API documentation, `views_format` enum items, https://docs.blender.org/api/5.1/bpy_types_enum_items/views_format_multiview_items.html — read 2026-07-30.

> "For the Stereo 3D formats the only lossless options are
> Top-Bottom and Side-by-Side without the Squeezed Frame option."

> "The Top-Bottom compressed will discard half of your vertical resolution data."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/stereoscopy/usage.html — read 2026-07-30. And `Stereo3dFormat.use_squeezed_frame`: "Combine both views in a squeezed image (default False)" — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Stereo3dFormat.html — read 2026-07-30.

**The step I took.** The source says compressed Top-Bottom discards half the vertical resolution data and names an unsqueezed Top-Bottom option as lossless. From that I infer that an unsqueezed Top-Bottom file holds more vertical pixel data than the scene resolution. The quoted words do not establish the dimensions of Side-by-Side output. The sources never state a pixel count, so this is **inferred**, not documented.

**Which scene is rendered can itself be decided elsewhere in the file.** 5.1 has a Sequencer Scene, a second scene the Render menu can target instead of the active one, and it brings its own output settings: "The render and output settings of the Sequencer Scene are the same as for regular scenes." (Manual, https://docs.blender.org/manual/en/5.1/video_editing/sequencer_scene.html — read 2026-07-30), reached by "Render Sequencer Image Alt-F12 — If a Sequencer Scene exists that differs from the active scene, render that scene instead." (Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30). That is still "a scene's pixel dimensions" — just a different scene's.

**The final compositor output is documented at the scene render size.** This does not establish the size written by every output node.

> "The operation domain of output nodes like the Viewer node is
> a domain with an identity transformation and the same size as the final compositor output. For
> viewport compositing, that size would be the viewport size, and for
> final render compositing, that size would be the scene render size."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30.

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

— Manual, stereoscopy usage page, address as above, read 2026-07-30.

Compositor nodes do resize images inside the graph — the Scale node offers "Render Size: Image dimensions set in the Render panel" and "Scene Size: Sizes an image to the size of the final render resolution for the scene" (Manual, https://docs.blender.org/manual/en/5.1/compositing/types/transform/scale.html — read 2026-07-30). The quote above establishes the domain of the final compositor output and gives the Viewer as an example; it does not settle every output node.

Pixel aspect is documented as a shape, not a count: "Older televisions may have non-square pixels, so this can be used to control the shape of the pixels along the respective axis." (Manual, format.html — read 2026-07-30.)

**Not covered.** What size the **File Output node** writes. It is an output node, and output nodes are documented as taking the final compositor output size — but that sentence names the Viewer node as its example and the File Output page says nothing about size at all. I could not settle it and have put it on the hand-check list. Also not covered: whether a render region combined with the compositor produces a cropped or a full-size composite; what happens when Crop to Render Region is on and the region is a single pixel; whether video containers impose their own sizes (the manual warns only that "some codecs do not work with arbitrary dimensions", Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30, without saying Blender changes the size).

**Outside the interface. Label: inferred.** The deciding settings are exposed as scene or node properties, including the four render-region bounds. From that, I infer that their stored values remain in force when Blender is driven from outside. The command-line argument list I read documents format (`-F`) and path (`-o`) overrides but no resolution or region override. The render-region page says the region "always" affects the final render, in contrast to its engine-dependent viewport behaviour; it does not name background rendering separately.

**Operating system. Most of this answer is settled on macOS and Linux both; one part is settled on macOS alone; Windows is not settled at all.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. **The exception is the second half of hand test 18** — ticking Render Region and looking at Crop to Render Region again. Ticking a box needs the pointer, which could not be relied on on the Linux machine, so that half rests on the Mac alone. Windows was not tried, because no Windows machine has run it. *(Linux citation check, 2026-08-06: this line previously said flatly that macOS and Linux were both settled by test. The second half of hand test 18 is part of this answer and is not settled on Linux, as this document says further down. Narrowed to name the exception.)* The nearest remark in the sources is again format availability, not size: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.)

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.1.2 started from a terminal with no window and with `--factory-startup`.

**Hand test 6 — three things that might decide the written size.** A 400×200 Workbench scene with a cube and a camera, rendered three ways. First, with a Scale node set to Absolute 120×90 between the Render Layers node and a File Output node, while the unscaled render went to the scene's own output. Second, with the render region set to the middle half of the frame — 0.25 to 0.75 on both axes — and Crop to Render Region off and then on. Third, with stereoscopy on and both views written into one file, Side-by-Side and Top-Bottom, squeezed and unsqueezed:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> file_nameBeauty.png is 122x90
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200)
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200)
```

This settles the item this answer put on its own hand-check list. **The File Output node writes at the size of the image plugged into it, not at the scene render size** — 122×90 against the scene's 400×200, in the same render in which the scene's own output was 400×200. So the domain rule that pins output nodes to the scene render size does not extend to it, and this answer's caution about how far that sentence reaches was right.

Note the exact number. The Scale node was set to 120×90 and the file came out 122 pixels wide. The same test in 4.5 and 5.0 gave exactly 120×90; 5.1 and 5.2 both gave 122×90. The two extra pixels were not asked for and the test does not explain them.

The stereo lines turn the step marked inferred above into a measurement, and they cover Side-by-Side as well as Top-Bottom: unsqueezed Side-by-Side doubles the width, unsqueezed Top-Bottom doubles the height, and both squeezed forms keep the scene's own dimensions. The crop lines also settle that a render region **without** Crop to Render Region leaves the written size untouched, which the answer above could only read as an implication, and they turn the crop's own written size from inferred into measured.

**Hand test 13 — what the output-path template reports under a crop.** The same 400×200 scene, render region cropped to the middle half, output path set to `{resolution_x}x{resolution_y}_`:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
```

The template reports the scene's own Resolution values, not the size actually written. That answers the item listed above about whether the size changes show up in those variables: they do not.

**Hand test 9 — whether the sequencer route changes the written size.** A 400×200 scene rendered six ways across the Sequencer and Compositing switches, with and without a Scene strip. All six wrote 400×200; the lines are quoted under question 7.

**Hand test 14 — a very large frame, and a difference this version alone shows.** A 9000×400 flat-green scene with an Invert compositor, rendered with each device. The run was repeated three times with the same result:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.7882, 0.4588, 0.7882, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.0, 0.0, 0.0, 1.0]
```

The written file was 9000×400 either way, so the size was not changed. But on the GPU the picture came out black instead of the inverted green. The same test in 4.5, 5.0 and 5.2 gave the inverted green on both devices. This is on Apple's own graphics; another maker's card cannot be tested with the available equipment.

**Outside the interface. Now observed.** All of the above ran from a terminal with no window. The step marked inferred above is now observed for the render region, the crop, the stereo formats and the File Output node — which matters, because this answer rightly flagged that Blender does skip at least one render setting outside the window and documents it when it does. None of these is among them.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.1's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 16 and 18.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor's Output tab was opened at the Format panel, the two checkboxes looked at with Render Region unticked, then Render Region ticked and them looked at again. The Creator's report, which they gave for all four versions at once:

> All four versions: Render Region is clickable, off by default; while off, Crop to Render Region is greyed out. Ticking Render Region makes Crop to Render Region available, but it stays off by default.

So the two settings quoted above sit one under the other on that panel, both start off, and the second is unavailable until the first is on. The API defaults quoted above — `use_border` default False, `use_crop_to_border` default False — are what the panel shows.

**Hand test 16 — what the Output panel holds.** Quoted in full under question 2. The one thing in it that bears on size is the Media Type control, which the Creator reported as a dropdown with three items — Image, Multi-Layer EXR, Video. Nothing in the panel offers a second way of deciding the written pixel size.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Test 16 was done again on the rented Linux machine and agreed; only the first half of test 18 could be done there. Both are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.1.2. **Second results, not replacements.**

**Hand test 6 — what decides the pixel size of what gets written.** What came back, word for word:

```
RESULT case1 scene output -> 400x200 (scene is 400x200)
RESULT case1 File Output after Scale Absolute 120x90 -> file_nameBeauty.png is 122x90
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

The same as on the Mac.

**Hand test 14 — the compositor above the size 5.2's documentation names.** Written out in full under question 1, and it is the one place in this topic where the card and the processor disagree: at 9000 pixels wide, 5.1 writes a black frame on the card and the composited picture on the processor, on both machines and on both makes of card.

**Hand test 15 on the Linux list — the card against the processor at an ordinary size.** Written out in full under question 1. All four compositor trees, rendered on both devices and compared value by value at 320×240, matched to the last figure.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 16 — the Output panel.** Written out in full under question 2: a **Media Type** dropdown reading **Image** is there, and no checkbox in the panel's header.

**Hand test 18 — whether Crop to Render Region is greyed out until Render Region is on.** The Properties editor was put on the Output tab, made to fill the window, and photographed. The Format panel holds two checkboxes one above the other, labelled exactly **Render Region** and **Crop to Render Region**. Both start unticked, and **Crop to Render Region is drawn greyed out** — its label and its box both dimmed — while Render Region is unticked. **Only the first half of the Mac's test was run here.** The Mac's version also ticked Render Region and looked again, and found Crop to Render Region became available; ticking a box needs a click, which does not reach Blender on that machine, so the second half stays as the Mac has it.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux, except the second half of test 18, which stays macOS only. With the Mac results above them, macOS and Linux now both have an answer for the rest and they agree. Windows stays untested.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Label: documented** for what the two switches mean. **Inferred** for the claim that either combination can be selected, with one wording conflict I flag rather than resolve.

**Yes.** The sources show two separate properties on the same panel. From that, I infer that one can be off while the other is on.

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node.
>
> Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

**Observed** — the two switches are drawn together under a heading "Pipeline", and there is nothing else in the panel but Dither:

```
        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

— `Blender Sources/cycles_5.1/ui.py`, class `CYCLES_PT_post_processing`, lines 1124–1126. Read 2026-07-30.

**The wording conflict.** The API describes the sequencer as running *after* the compositor rather than instead of it:

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The manual says "instead of the view from the 3D scene's active camera"; the API says "the render (and composited) result … through the video sequence editor pipeline". I quote both and do not choose between them. Note the API's condition, which the manual does not state: the sequencer pipeline applies "if sequencer strips exist".

**What it changes about what gets written** — two things the sources state:

1. **Where the picture comes from.** "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera." (Manual, post_processing.html — read 2026-07-30.)

2. **A Scene strip uses only the Combined render pass.** "Scene strips do not render individual Render Passes; only the Combined render pass will be used." (Manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30.)

One related input limit is documented: **Multi-layer EXR is not a usable input to the Video Sequencer.** "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor." (Manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.)

Two more settings sit here and are worth naming, though neither is a bypass: `use_sequencer_override_scene_strip` — "Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip (default False)"; and `sequencer_gl_preview` — "Display method used in the sequencer view (default 'SOLID')" (Python API documentation, RenderSettings, address as above, read 2026-07-30). I found no manual page for either.

**Not covered.** What is written when Sequencer is on but no strips exist (the API's "if sequencer strips exist" implies a fall-back but does not say to what). What is written when *both* Compositing and Sequencer are off. Whether the file path, format, resolution and metadata settings are taken from the same scene in the sequencer case — the sources never say, though nothing suggests otherwise. Whether the scene's own compositor is skipped entirely, or run and then discarded, when Compositing is off and Sequencer is on. Whether alpha survives (the Scene strip's "Transparent — Creates a transparent background" option exists, but the sources do not connect it to what is written).

**Outside the interface. Label: inferred** for the two stored switch values, and **documented** for the outside controls quoted here. Both switches are scene properties (`scene.render.use_compositing`, `scene.render.use_sequencer`), so I infer that their stored values remain available when Blender is driven from outside. Which scene the Render menu targets is interface-bound: "Render Sequencer Image Alt-F12 — If a Sequencer Scene exists that differs from the active scene, render that scene instead" and "Note that these settings follow the active scene in the window by default (unless there is a Scene pinned to the properties editor)" (Manual, topbar.html and sequencer_scene.html — read 2026-07-30). From outside, the scene can be named explicitly: `-S, --scene <name>` — "Set the active scene <name> for rendering" (Manual, command-line arguments page — read 2026-07-30), and `bpy.ops.render.render` takes both `scene` ("Scene to render, current scene if not specified") and `use_sequencer_scene` ("Render the sequencer scene instead of the active scene") — Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

**Operating system. macOS and Linux are both settled by test; Windows is not.** The tests below were run on macOS. The same tests were run again on a rented Linux machine and the two runs agreed; the Linux results are in their own section further down. Windows was not tried, because no Windows machine has run it. Video encoding availability is the only nearby operating-system-sensitive thing in the sources, and they frame it as which formats are compiled in, not as behaviour.

### Observed — hand tests, 2026-08-03

Run on the Creator's own computer, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.1.2 started from a terminal with no window and with `--factory-startup`.

**Hand test 9 — what the sequencer route writes.** A 400×200 scene was built whose camera view is blue, `[0.0745, 0.3333, 0.8196, 1.0]`, and whose compositor replaces the picture with flat red, `[0.8549, 0.2196, 0.1255, 1.0]`. A second scene rendering green, `[0.4275, 0.7686, 0.3137, 1.0]`, was added as a Scene strip for half the runs. Six renders across the Sequencer and Compositing switches:

```
RESULT no strips, sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer OFF, compositing OFF -> 400x200 first pixel RGBA [0.0745, 0.3333, 0.8196, 1.0]
RESULT no strips, sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
RESULT strip,     sequencer ON,  compositing OFF -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer ON,  compositing ON  -> 400x200 first pixel RGBA [0.4275, 0.7686, 0.3137, 1.0]
RESULT strip,     sequencer OFF, compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1255, 1.0]
```

This answers three things this answer lists as not covered. With Sequencer on and no strips, a file is written and it holds the scene's ordinary picture — the camera view with Compositing off, the compositor's result with it on. With both switches off, the camera view is written. And the written size stayed at 400×200 in every one of the six.

It also bears on the wording conflict flagged above. Rows four and five show that with a strip present the strip's picture replaces the scene's own compositor result as well as the camera view — the compositor's flat red never appears — so on this evidence the manual's "instead of the view from the 3D scene's active camera" describes what happens better than the API's "the render (and composited) result … through the video sequence editor pipeline" does. That is what the test showed; it is not a claim about which sentence is right in general.

**Hand test 8 — whose Compositing switch a Scene strip follows.** Quoted in full under question 4. The four runs show the picture follows the strip's own scene's switch, not the rendering scene's.

**Hand test 13 — what survives into a multi-layer EXR.** A scene with the Depth and Mist passes turned on, rendered to multi-layer OpenEXR twice: once plainly, once with a Scene strip supplying the picture and the Sequencer switch on. The channel names in the two files:

```
exr_seqFalse.exr: ViewLayer.Combined.A ViewLayer.Combined.B ViewLayer.Combined.G ViewLayer.Combined.R ViewLayer.CombinedpixelAspectRatio ViewLayer.Depth.Z ViewLayer.DepthpixelAspectRatio
exr_seqTrue.exr:  Composite.Combined.A Composite.Combined.B Composite.Combined.G Composite.Combined.R Composite.CombinedpixelAspectRatio
```

Without the sequencer the file holds a view layer with its Combined and Depth passes. With the sequencer supplying the picture it holds one layer called Composite with only the combined image; the Depth pass is gone. Note that the multi-layer EXR was written in both cases — the documented limit is about multi-layer EXR going *into* the sequencer, and this answer was right not to extend it to what comes out.

**Outside the interface. Now observed.** Every run above was started from a terminal with no window. Both switches take effect that way, and the step marked inferred above is now observed.

### Observed — window tests, 2026-08-03

These two are **interface observations**, unlike the terminal tests above. The Creator ran them by hand in Blender 5.1's own window, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. The set-ups are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 15 and 20.

**Hand test 15 — what the Post Processing panel holds.** Quoted in full under question 1. The Creator saw two checkboxes, Compositing and Sequencer, both on by default, and one slider called "Dither"; the Compositing tooltip reads "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene", and the Sequencer tooltip:

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist."

Two things follow for this answer. The step marked inferred above — that one switch can be off while the other is on — rested on the two being separate properties on one panel; the Creator saw two separate checkboxes there, both on by default, which is what the API defaults say. And on the wording conflict flagged above: the tooltip the running program shows for the Sequencer switch is the API's sentence, word for word, not the manual's "instead of the view from the 3D scene's active camera". That records which of the two wordings the program itself puts in front of a user. It does not settle which describes the behaviour better — hand test 9 above is what bears on that.

**Hand test 20 — what the Render menu offers.** Quoted in full under question 3. The Creator opened the Video Editing tab and then the Render menu in a fresh file, reported the menu identical in all four versions, and supplied one screenshot. It holds Render Image (F12), Render Animation (⌘ F12), Render Audio…, View Render (F11), View Animation (⌘ F11) and a Lock Interface checkbox. **No entry in it names a Sequencer Scene.**

Read that against the manual sentence quoted above rather than instead of it: those entries are conditional — "If a Sequencer Scene exists that differs from the active scene, render that scene instead" — and this look was taken in a fresh file where no such scene had been made. So the entries are not always in the menu; whether they appear once a Sequencer Scene differs from the active one was not tested and stays open.

**Outside the interface.** These two settle nothing about it. Both are looks at what a Blender window shows.

**Operating system.** Both were done on macOS. Both were done again on the rented Linux machine, and the two agreed; the Linux results are in the section below. Windows was not tried — no Windows machine has run it.

### Observed — the same tests on the rented Linux machine, 2026-08-06

Run again, unchanged, on the rented Linux machine, with Blender 5.1.2. **Second results, not replacements.**

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

**Hand test 8 — whose Compositing switch a Scene strip follows.** Written out in full under question 4.

### The window tests on the rented Linux machine, 2026-08-06

**Hand test 15 — what the Post Processing panel holds.** Written out in full under question 1: two checkboxes, Compositing and Sequencer, both ticked, and a Dither slider reading 1.00, and nothing else — and no checkbox in the panel's own header.

**Hand test 20 — what the Render menu offers.** Written out in full under question 3: Render Image, Render Animation, Render Audio…, View Render, View Animation and Lock Interface, and no entry naming a Sequencer Scene.

**Outside the interface.** The window tests settle nothing about it; the terminal tests were all run with no window.

**Operating system.** All of these were done on Linux and settle Linux. With the Mac results above them, macOS and Linux now both have an answer and they agree. Windows stays untested.

---

## Things only a running Blender 5.1 can settle

Every test that was on this list has been run except one, and the results are written into the answers above with the machine's own output or the Creator's own words quoted. The set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

The seven that needed somebody looking at a Blender window were run by the Creator and are recorded under the questions they answer: what the Post Processing panel holds (entry 15, questions 1 and 7), what the Output panel holds and whether its header carries a checkbox (entry 16, questions 2 and 6 — the Media Type dropdown is really there, with the three items the manual's text names, and there is no header checkbox), what the Compositor editor's header offers (entry 17, questions 1 and 3), whether Crop to Render Region is greyed out until Render Region is on (entry 18, question 6), what hovering a link shows (entry 19, question 5), what the Render menu offers (entry 20, questions 3 and 7), and how a second Group Output node is marked as the unused one, with the warning reading "Unused Output" (entry 21, questions 1 and 5).

**One is still open, and it needs somebody looking at a Blender window.**

1. **How many inputs the scene's render output actually uses (questions 2 and 5).** The Group Output page says "In the Compositor, only the first image input is used as the final render result. Any additional sockets will be ignored", while the Output panel page says of Media Type Image, "Saves each input as a separate image file. Each input can use its own image format settings." Build a compositor tree whose Group Output has two image inputs, both connected to visibly different images, and render one frame. Report how many files were written and what each contains. This was never put on the numbered hand-test list.

Three smaller things the seven left open on the Mac. **Two have since been answered on a second machine**: the Post Processing panel's own header carries no checkbox, and neither Group Output node's header carries a control for making it the live one. The third is still open on both: whether the Render menu grows its Sequencer Scene entries once a Sequencer Scene differs from the active scene, which a fresh-file look cannot show. All three are written in beside their findings.

**A second machine, 2026-08-06.** Every test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are written in beside the Mac's, never merged with them. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`. Every test agreed with the Mac except one, which could not be settled there at all: what hovering a link shows (entry 19). One new test was added that only the second machine could carry: whether a compositor tree gives the same picture on the graphics card as on the processor. At 320×240 it does, for the four trees tried — and at 9000 pixels wide it does not, which is 5.1's own black-frame result, now seen on two different makes of card.

**Still cannot be run with the available equipment.** Two things stay untested. **Windows** — no Windows machine has run it, so every "operating system" line above covers macOS and Linux and leaves Windows open. Two things in this document turn on that and stay untested with it: the compositor's documented driver-reset limit on Windows, and the note that headless rendering is not supported on headless Windows systems. **A graphics card of another make** — the two machines have Apple's own graphics and one NVIDIA card between them, so nothing here speaks for an AMD or Intel card. Three smaller things need the rented machine to itself, because its display is shared and a click does not reach Blender on it: what hovering a link shows, what the tooltip on 5.2's Output header checkbox says, and what Crop to Render Region looks like once Render Region is ticked.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 5.1

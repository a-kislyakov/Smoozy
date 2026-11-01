# The compositor — added questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender's manual for 5.0, Blender's Python API documentation for 5.0, the 5.0 release notes, and Cycles' own files for 5.0 — all read from the snapshot taken **2026-07-30**, which is the date read for every citation below. Four pictures from the manual's `_images` folder were opened and are named where they are used. Blender's requirements page was not needed for this question and was not used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 8 only. No other question on the sheet was answered.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

### The question is ambiguous, and I answer both readings

"Followed forward" can mean two different things, and the sources treat them very differently:

- **Reading A — followed through the wiring.** Can you tell, from the file, that this picture goes into that node, and out of it into the next one, and so on? This is the reading I take as the one meant, because it is the one the sources actually address.
- **Reading B — followed in the pixels.** Does a picture carry some mark or name of its own that survives a node, so that you could recognise it again on the far side without knowing the wiring?

I answer A first, then B, then the "which of the two" part.

---

### A. Following through the wiring

**LABEL: documented.**

Every colour-changing node documents one picture in and one picture out, so the chain does not break at one.

> "Inputs
>
> Image/Color
>
> Standard color input.
> […]
> Outputs
>
> Image/Color
>
> Standard color output."

— Blender manual, Hue/Saturation/Value Node, https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/hue_saturation.html — read 2026-07-30

> "Inputs
>
> Image
>
> Standard color input.
>
> Factor
>
> Controls the amount of influence the node exerts on the output image."

and

> "Outputs
>
> Image
>
> Standard output image."

— Blender manual, Color Balance Node, https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/color_balance.html — read 2026-07-30

The connections themselves are readable, one by one, as data in the file. A link names both ends and both sockets:

> "class bpy.types.NodeLink(bpy_struct)
>
> Link between nodes in a node tree
>
> from_node
>
> Type:
>
> Node, (readonly)
>
> from_socket
>
> Type:
>
> NodeSocket, (readonly)"

and

> "to_node
>
> Type:
>
> Node, (readonly)
>
> to_socket
>
> Type:
>
> NodeSocket, (readonly)"

— Blender Python API, NodeLink, https://docs.blender.org/api/5.0/bpy.types.NodeLink.html — read 2026-07-30

Sockets can be told apart even when two of them carry the same visible name:

> "identifier
>
> Unique identifier for mapping sockets
>
> Type:
>
> string, default "", (readonly, never None)"

and

> "is_linked
>
> True if the socket is connected"

— Blender Python API, NodeSocket, https://docs.blender.org/api/5.0/bpy.types.NodeSocket.html — read 2026-07-30

That matters here, because the node that takes two pictures in sometimes labels both inputs with the same word. The manual's own picture of the Split node shows two input sockets both labelled `Image` (manual, Split Node page, picture `_images/node-types_CompositorNodeSplit.webp`, https://docs.blender.org/manual/en/5.0/compositing/types/utilities/split.html — read 2026-07-30). Visible in that picture: a node headed "Split", one output socket labelled `Image`, an X/Y pair of buttons, a `Factor` field reading 0.500, and two input sockets each labelled `Image`. **Documented** as far as what the picture shows. Note that this picture disagrees with the text on the same page, which describes `Position` and `Rotation` inputs rather than X/Y and Factor — see the caution about stale pictures below.

**Watching the picture as it goes.** The manual documents two ways to look at the picture partway along the chain.

> "Shows/Hides a preview region on the node that displays the frame after that node's operation has been applied. This can also be toggled by clicking the material ball icon in the node header."

followed immediately by:

> "Note
>
> This operator are only available in the Compositor."

— Blender manual, Node Preview, https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html — read 2026-07-30 (the odd grammar is the manual's own)

> "The Viewer node is used to preview image data or value maps anywhere within a compositor node graph. It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."

— Blender manual, Viewer Node, https://docs.blender.org/manual/en/5.0/compositing/types/output/viewer.html — read 2026-07-30

The per-node previews are also a switch in the file:

> "show_previews
>
> Display each node's preview if node is toggled
>
> Type:
>
> boolean, default False"

— Blender Python API, SpaceNodeOverlay, https://docs.blender.org/api/5.0/bpy.types.SpaceNodeOverlay.html — read 2026-07-30

So: yes to reading A, for both kinds of node. The wiring is fully readable, and there is a documented way to look at the picture after any node's work has been done.

---

### B. Following in the pixels

**LABEL: not in the sources** (that a picture carries any identity of its own), with the record of where I looked at the end.

What travels between compositor nodes is described exhaustively, and it is only pixel values plus a frame. There is no field in it for a name, a tag, or an origin:

> "Three types of data exist, all of which are stored in half precision formats:
>
> Float
> […]
> Vector
> […]
> Color
>
> A 4D vector storing the Red, Green, Blue, and Alpha of the color."

and

> "Consequently, images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations. […] The rectangular area occupied by an image in that space as defined by its transformation and size is called the Domain of the image."

— Blender manual, Compositor System, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30

**Inferred**, and here is the step: the manual lists the kinds of data that exist and describes an image as its pixels plus a domain. Nothing else is listed. So a picture entering a colour node and leaving it is not carrying a label that a later node could read back. A reader has to supply the step "the list is complete", so this is inferred, not documented.

The one thing in the sources that comes close is a *deliberate* mark the user puts into the pixels themselves. The manual describes encoding a faint watermark with a Mix node and then recognising it later:

> "Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

and, for reading it back:

> "In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference. You can see how the original mark clearly stands out."

— Blender manual, Mix Color Node, https://docs.blender.org/manual/en/5.0/compositing/types/color/mix_color.html — read 2026-07-30

That is a user-built mark in the picture's own pixels, not something the compositor attaches or maintains. It is the only mechanism in the sources by which a picture can be recognised later by looking at it.

---

### C. Which of the two came out the far side

The answer depends on which two-in-one-out node it is. The sources describe three different kinds, and they behave differently.

#### C1. Nodes that pass exactly one input through, and say so outright

**LABEL: documented.**

> "Switch
>
> When it is unchecked, the first input labeled "Off" is passed to the output.
>
> When checked, the second input labeled "On" is passed to the output.
>
> Off
>
> First image input.
>
> On
>
> Second image input."

— Blender manual, Switch Node, https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch.html — read 2026-07-30

The manual's picture of that node confirms the layout (picture `_images/node-types_CompositorNodeSwitch.webp`, on the same page, read 2026-07-30). Visible in it: a node headed "Switch", one output socket labelled `Image`, a checkbox labelled `Switch`, and two input sockets labelled `Off` and `On`. **Documented** for what is visible.

The Menu Switch node goes further and publishes the answer as data:

> "Menu
>
> Determines which input socket is passed through to the output."

> "Outputs the value from the input corresponding to the selected menu item, unchanged.
>
> For each menu entry, an additional Boolean output is created. The output corresponding to the selected item is true, and all others are false."

— Blender manual, Menu Switch Node, https://docs.blender.org/manual/en/5.0/compositing/types/utilities/menu_switch.html — read 2026-07-30

The release notes record that this node is available in the compositor in this version:

> "**Menu** sockets and **Switch Menu** node are now supported in the compositor."

— Blender 5.0 release notes, Compositor, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30

So for these nodes: yes, and the "which" is a setting you can read, and in the Menu Switch's case an output socket that says so.

#### C2. Nodes where both come out, each in a known place

**LABEL: documented.**

> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line."

> "Image
>
> The first image input, displayed on one side of the split.
>
> Image
>
> The second image input, displayed on the opposite side of the split."

— Blender manual, Split Node, https://docs.blender.org/manual/en/5.0/compositing/types/utilities/split.html — read 2026-07-30

Depth Combine decides per pixel, by a stated rule:

> "Result
>
> If both depth values are equal, it will use the foreground image.
>
> Whichever depth value is less decides which image pixel is used."

— Blender manual, Depth Combine Node, https://docs.blender.org/manual/en/5.0/compositing/types/color/depth_combine.html — read 2026-07-30

So here the answer is not "one of the two" but "this part of one and that part of the other", and the rule that decides it is stated.

#### C3. Nodes that blend the two

**LABEL: documented** for what the node does; **not in the sources** for any mark on the output naming its origin.

> "Factor
>
> Controls the amount of mixing between the A and B inputs.
>
> A/B
>
> The two inputs that are mixed together."

and

> "Result
>
> Output the result of the mix using the data type selected."

— Blender manual, Mix Color Node, https://docs.blender.org/manual/en/5.0/compositing/types/color/mix_color.html — read 2026-07-30

> "Factor
>
> Controls the opacity of the foreground image, from 0 (fully transparent) to 1 (fully opaque)."

and

> "Outputs
>
> Image
>
> The combined image after alpha compositing."

— Blender manual, Alpha Over Node, https://docs.blender.org/manual/en/5.0/compositing/types/color/alpha_over.html — read 2026-07-30

The output is described as a result, or a combination, of both. Nothing in the sources says the result records which input it came from.

There is a limit case the sources let you reason to. **Inferred**, and the step is this: Alpha Over's Factor is documented as running "from 0 (fully transparent) to 1 (fully opaque)" for the foreground, so at 0 the foreground contributes nothing and what comes out is the background alone. The words "fully transparent" are the source's; "so what comes out is the background alone" is my step, which is why this is inferred and not documented. The same shape of reasoning applies to Mix's Factor, but the manual does not spell out which end of the Factor range corresponds to which of A and B, so I do not claim it.

#### C4. One thing about "which of the two" that *is* stated outright — the frame, not the content

**LABEL: documented.**

The compositor names one of a node's inputs as the one that decides the size and placement of what comes out:

> "One of the inputs of the nodes is designated as the Domain Input of the node, and the operation domain of the node is identical to the domain of that designated input."

> "Each input in the node has the so called Domain Priority property, the operation domain of the node is the domain of the non single value input with the highest domain priority."

And it names it for the two nodes in question:

> "For instance, if the Foreground input is bigger than the Background input in the Alpha Over node, the output will be clipped to the Background input because it is the domain input"

> "The Alpha Over node currently does not support changing the domain priority for its inputs, so as a workaround, one can use a Mix node to achieved the desired behavior, noting that the first Image input in the Mix node has the highest domain priority"

— Blender manual, Compositor System, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30 (the grammar is the manual's own)

So the *frame* of what comes out of a two-input node is inherited from one named input: the Background for Alpha Over, the first Image input for Mix. That is a real, documented sense in which one of the two governs the far side — but it is about the picture's size and position, not about its pixels.

I opened the manual's picture of this (`_images/compositing_realtime-compositor_compositing-space_operation-domain_considerations_clipping.png`, on the Compositor System page, read 2026-07-30). Visible in it: a full Blender window, a compositor tree of two Image nodes each into a Scale node, both into an Alpha Over node, then into a Composite node, and a backdrop showing the logo clipped to the square area of the smaller background image. **Documented** for what is visible. **Caution:** that screenshot is stale relative to the 5.0 text. It reads "3.5.0 Alpha" in its corner, it shows a `Composite` node and a `Use Nodes` checkbox — both of which the 5.0 release notes record as removed — and it labels the Alpha Over inputs `Fac`, `Image`, `Image` rather than the names the 5.0 text uses. The same is true of the Alpha Over node's own picture (`_images/node-types_CompositorNodeAlphaOver.webp`, read 2026-07-30), which shows `Fac`, two sockets labelled `Image`, and `Straight Alpha`. The 5.0 release notes state the change:

> "The **Alpha Over** node input names were renamed to Background and Foreground."

and

> "The **Composite** node was removed and replaced by a Group Output node whose first input needs to be a Color socket, and other inputs are ignored."

and

> ""Use Nodes" in the node editor was removed"

— Blender 5.0 release notes, Compositor, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30

I therefore take socket names from the text and the release notes, not from those pictures.

#### C5. Two further signals in the file, with a caution on each

**LABEL: documented** that these exist; the sources do not say how they are filled in for compositor nodes.

A node states, as data, which of its inputs hands over to which output when it is switched off:

> "internal_links
>
> Internal input-to-output connections for muting
>
> Type:
>
> bpy_prop_collection of NodeLink, (readonly)"

— Blender Python API, Node, https://docs.blender.org/api/5.0/bpy.types.Node.html — read 2026-07-30

and the manual says what muting does:

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

— Blender manual, Mute, https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html — read 2026-07-30

So for a *muted* two-input node, the file itself names which input reaches the output. The sources do not state which input each particular compositor node picks; they state only that the answer is recorded there to be read.

Second, a socket can say that it does not reach the output at all:

> "is_inactive
>
> Socket is grayed out because it has been detected to not have any effect on the output
>
> Type:
>
> boolean, default False, (readonly)"

— Blender Python API, NodeSocket, https://docs.blender.org/api/5.0/bpy.types.NodeSocket.html — read 2026-07-30

This is on the general socket type, not on a compositor-specific one. The sources do not say whether compositor nodes set it, nor under what conditions. I do not claim they do.

---

### What a running Blender showed

**LABEL: observed.** Blender 5.0.1, run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, with Blender pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics while this ran. The whole procedure, every script in full and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only.** The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Every pixel comparison below rests on a control (entry 1) that came back `MAXDIFF render vs source 0.0` and `MAXDIFF render vs same render again 0.0`, so a difference of zero means the pictures match.

**A muted two-picture node says which input it hands over, and what it says is what the render does (entries 5 and 7).**

> `=== AlphaOver (CompositorNodeAlphaOver) ===`
> `  unmuted internal_links: [('Background', 'Image')]`
> `  muted   internal_links: [('Background', 'Image')]`
> `=== Split (CompositorNodeSplit) ===`
> `  unmuted internal_links: [('Image', 'Image')]`
> `=== Z/Depth Combine (CompositorNodeZcombine) ===`
> `  unmuted internal_links: [('A', 'Result'), ('Depth B', 'Depth')]`

and, rendered with two different pictures on the two inputs:

> `MUTED AlphaOver            internal_links=[('Background', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Mix                  internal_links=[('A_Color', 'Result_Color')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Switch               internal_links=[('Off', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Split                internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Z/DepthCombine       internal_links=[('A', 'Result'), ('Depth B', 'Depth')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`

**`is_inactive` is never set on these compositor sockets in this version (entry 5).** With both picture inputs linked, muted and unmuted alike:

> `  unmuted is_inactive: [('Background', 'Background', False), ('Foreground', 'Foreground', False), ('Factor', 'Fac', False), ('Type', 'Type', False), ('Straight Alpha', 'Straight Alpha', False)]`
> `  muted   is_inactive: [('Background', 'Background', False), ('Foreground', 'Foreground', False), ('Factor', 'Fac', False), ('Type', 'Type', False), ('Straight Alpha', 'Straight Alpha', False)]`

**Which end of Mix's Factor gives A and which gives B (entry 3).**

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

with picA on the A input and picB on the B input. Alpha Over behaves the same way at its own ends:

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

**The frame does come from the named input (entry 9).** A 32 by 32 picture and a 128 by 128 picture, rendered at 128 by 128, reading the area whose alpha is above zero:

> `AlphaOver(bg,fg)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `AlphaOver(bg,fg)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`
> `Mix(A,B)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `Mix(A,B)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`

*[Note added 2026-08-07 by entry 20, below. This entry's conclusion for 5.0 is right, and entry 20 confirms it by reading the frame straight off the socket instead of inferring it from the alpha. Two cautions belong with it even so. The Factor was never written down here, and it matters: a Mix node starts at Factor 0.5 in this version and at 1.0 in 5.2, which is what made 5.2 look unlike the rest. And the area whose alpha is above zero is a reading of the alpha, not of the frame; the two happen to agree in this version at this Factor, and they do not always. Entry 20 is what to read.]*

**The Viewer produces nothing with no window (entry 10).** In this version no image called `Viewer Node` exists at all:

> `VIEWER IMAGE before the render: there is no image called 'Viewer Node'`
> `VIEWER IMAGE after the render: there is no image called 'Viewer Node'`

**The two Split inputs do have different identifiers despite sharing a name (entry 2).**

> `PRESENT CompositorNodeSplit label: Split`
> `   IN  [('Position', 'Position', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

and the first of them lands on the left of the frame (entry 8):

> `   SPLIT left column x=5 y=32 -> [0.07937, 0.50794, 0.25, 1.0]   (picA there [0.07937, 0.50794, 0.25, 1.0], picB there [0.1, 0.92063, 0.8, 1.0])`
> `   SPLIT right column x=58 y=32 -> [0.1, 0.07937, 0.8, 1.0]   (picA there [0.92063, 0.50794, 0.25, 1.0], picB there [0.1, 0.07937, 0.8, 1.0])`

**Three further things this document leans on, settled the same day.** The whole graph reads back out of a saved file with no window, every link with both its nodes and both its sockets (entry 11). No property on a socket or on a node names a priority or a domain — `ANY NAMING PRIORITY OR DOMAIN: []` (entry 5). And the Menu Switch this document describes is not a compositor node type at all: what can be added to a compositor tree is `GeometryNodeMenuSwitch` (entry 2).

**The processor against the card, run on 2026-08-07 once the card was repaired (entry 12).** On 2026-08-06 the machine's graphics card could not be reached: its kernel driver and its driver library had drifted apart, `nvidia-smi` would not start, and Blender printed EGL errors on every run. The card has since been repaired. Everything in this paragraph was got on **2026-08-07**, on that same rented Linux machine — **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and still no display server at all** — with Blender 5.0.1, `-b --factory-startup`, and a settings folder of its own made beforehand under `/root/compositor/`. The card now answers:

> `NVIDIA-SMI 580.173.02             Driver Version: 580.173.02     CUDA Version: 13.0`
> `NVIDIA GeForce RTX 4080 SUPER, 580.173.02, 16376 MiB`

and no run printed an EGL line of any kind.

A colour node and an Alpha Over at factor 0.5, rendered once with the compositing device set to `CPU` and once to `GPU`, the two pictures loaded back and compared pixel by pixel:

> `DEVICE CPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `DEVICE GPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `MAXDIFF between the processor's picture and the card's picture: 6.258487701416016e-07`

**The run marked `GPU` really does use the card.** A heavier tree — a 1024 by 1024 picture through sixty Blur nodes, rendered at 2048 by 2048 — was run twice, one device to a Blender, while the card was asked over and over which processes were on it. Only this Blender's own process was counted, because the machine is shared with other topics' tests. Set to the processor, then set to the card:

> `HEAVY PROCESSOR CORE-SECONDS CPU 62.190`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 44`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`
> `HEAVY PROCESSOR CORE-SECONDS GPU 0.940`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 20`
> `TIMES THIS PROCESS WAS ON THE CARD: 5`
> `MOST CARD MEMORY THIS PROCESS HELD: 214 MiB`

and the two heavy pictures, loaded back and compared pixel by pixel:

> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`

So the setting really moves the work onto the card, and the picture the card makes matches the processor's to about four parts in ten million. **This bears on identifying a picture by its exact values**: two pictures made from the same tree on the two devices are not bit-for-bit equal, so an identity test finer than a millionth would fail across them, and any test coarser than that still holds.

**Added 2026-08-07 — the three tests the citation check sent back (entries 20, 21 and 22 of the hand-test list).**

**LABEL: observed.** Everything in this sub-section was got by running Blender 5.0.1 on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB with its kernel driver and its driver library both at 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all**, reached at `ssh -i ~/.ssh/id_ed25519 -p 39348 root@85.218.235.6` — on **2026-08-07**. Every run used `-b --factory-startup` and was pointed at a settings folder of its own, made beforehand under `/root/again-compositor/`, which is a third working folder outside both earlier sittings and outside every other topic's work. The Blender named itself with the same build hash as on 2026-08-06, so this is the same build. The machine was shared with other topics while this ran, so nothing here reports how long anything took or how much of the card was in use, and nothing here needed either. **Pictures were compared as pictures, never as files.** The full procedure, every script, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 20, 21 and 22. This settles Linux alone; none of the three has been run on the Creator's Mac.

The citation check of question 8 sent three tests here. One of them carried a disagreement it had recorded and could not resolve. All three are answered below, and the disagreement is settled.

**Which input decides the frame of a Mix node, with the Factor stated and swept (entry 20). This is the test that settles the disagreement recorded against entry 9 above.** `picSmall` (32 by 32) and `picLarge` (128 by 128), both fully opaque — each reported its own lowest alpha as `1.0` when it was made — into the two colour inputs of a `ShaderNodeMix` set to `RGBA` and `MIX`, once small-first and once large-first, rendered at 128 by 128, at Factor 0.0, 0.5 and 1.0, with the Factor read back off the node and printed on every line. The frame was read **two ways**: by the area whose alpha is above zero, which is what entry 9 read; and by feeding the Mix node's output into an Image Info node and rendering its `Dimensions` output, which does not go through alpha at all.

**What a negative result would have looked like.** The frame could have followed the B input, or the larger input, or the render size, rather than A. It could have moved with the Factor, which is what the citation check's sweep suggested. It could have differed between the versions, which is what entry 9 concluded. The Image Info reading could have disagreed with its own control, in which case it reports something other than the frame and would have had to be thrown out. And a freshly added Mix node could have held the same Factor in all four versions, which would have left the disagreement unexplained. None of those was ruled out by the setup.

Way one, the area whose alpha is above zero — the measure entry 9 used — word for word:

> `Mix(A,B)  FACTOR SET 0.0  FACTOR HELD ON THE NODE 0.0  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.0, 0.0, 0.0, 0.0]    pixel (64,64), inside both: [0.9, 0.2, 0.2, 1.0]`
> `Mix(A,B)  FACTOR SET 0.0  FACTOR HELD ON THE NODE 0.0  A=picLarge(128) B=picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.2, 0.2, 0.9, 1.0]    pixel (64,64), inside both: [0.2, 0.2, 0.9, 1.0]`
> `Mix(A,B)  FACTOR SET 0.5  FACTOR HELD ON THE NODE 0.5  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.0, 0.0, 0.0, 0.0]    pixel (64,64), inside both: [0.55, 0.2, 0.55, 1.0]`
> `Mix(A,B)  FACTOR SET 0.5  FACTOR HELD ON THE NODE 0.5  A=picLarge(128) B=picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.1, 0.1, 0.45, 1.0]    pixel (64,64), inside both: [0.55, 0.2, 0.55, 1.0]`
> `Mix(A,B)  FACTOR SET 1.0  FACTOR HELD ON THE NODE 1.0  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.0, 0.0, 0.0, 0.0]    pixel (64,64), inside both: [0.2, 0.2, 0.9, 1.0]`
> `Mix(A,B)  FACTOR SET 1.0  FACTOR HELD ON THE NODE 1.0  A=picLarge(128) B=picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.0, 0.0, 0.0, 1.0]    pixel (64,64), inside both: [0.9, 0.2, 0.2, 1.0]`

The same twelve lines came back from 4.5. 5.1 and 5.2 gave the same sizes except at Factor 1.0 with the large picture on A, where they gave 32 by 32; and the pixel outside the small picture came back with a different alpha in them, which is the next point below.

Way two, the frame read straight off the socket. **This is the answer.** Word for word:

> `IMAGE INFO NODE USED CompositorNodeImageInfo outputs [('Dimensions', 'Dimensions', 'VECTOR'), ('Resolution', 'Resolution', 'VECTOR'), ('Location', 'Location', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Scale', 'Scale', 'VECTOR')]`
> `Mix(A,B)  FACTOR 0.0  A=picSmall(32) B=picLarge(128) -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]   at pixel (64,64) [32.0, 32.0, 0.0, 1.0]`
> `Mix(A,B)  FACTOR 0.0  A=picLarge(128) B=picSmall(32) -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]   at pixel (64,64) [128.0, 128.0, 0.0, 1.0]`
> `Mix(A,B)  FACTOR 0.5  A=picSmall(32) B=picLarge(128) -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]   at pixel (64,64) [32.0, 32.0, 0.0, 1.0]`
> `Mix(A,B)  FACTOR 0.5  A=picLarge(128) B=picSmall(32) -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]   at pixel (64,64) [128.0, 128.0, 0.0, 1.0]`
> `Mix(A,B)  FACTOR 1.0  A=picSmall(32) B=picLarge(128) -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]   at pixel (64,64) [32.0, 32.0, 0.0, 1.0]`
> `Mix(A,B)  FACTOR 1.0  A=picLarge(128) B=picSmall(32) -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]   at pixel (64,64) [128.0, 128.0, 0.0, 1.0]`
> `CONTROL  picSmall alone -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]`
> `CONTROL  picLarge alone -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]`

The control at the foot of that list is what makes the rest of it readable: with no Mix node in the tree at all, the Image Info node reports 32 for the 32 and 128 for the 128. **All six Mix readings say the same thing, and the other three versions printed the same six: the frame follows the first colour input, A. It does not follow B, it does not follow the larger input, it does not follow the render size, and it does not move with the Factor.**

**What a freshly added Mix node holds before anything is set**, in this version:

> `FRESH ShaderNodeMix, nothing set: Factor_Float default_value = 0.5`
> `AFTER data_type='RGBA' and blend_type='MIX': Factor_Float default_value = 0.5`

It is `0.5` in 4.5, 5.0 and 5.1 and `1.0` in 5.2. With the Factor then left alone — which is entry 9's own condition, reproduced:

> `Mix(A,B)  FACTOR LEFT AT THE DEFAULT 0.5  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `Mix(A,B)  FACTOR LEFT AT THE DEFAULT 0.5  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`

Those two lines are entry 9's two Mix lines for this version, back to the letter.

**So the disagreement is settled, and both runs that stood are explained rather than overruled.**

- **The answer.** A Mix node's output takes its frame from its first colour input, in all four versions, at every Factor tried.
- **Why entry 9 saw a difference between the versions.** It never set the Factor, so each version ran at its own starting Factor — 0.5 in three of them and 1.0 in 5.2. Its numbers were right; the sentence drawn from them was not, because what differed was the setting and not the behaviour.
- **Why the citation check's repeat suggested the Factor decides the frame.** It set the Factor and swept it, both correctly, and 5.1 and 5.2 then matched each other, correctly. But it read the frame by the area whose alpha is above zero, and that is a reading of the alpha. At Factor 1.0 the output's alpha is the second input's alpha, so from 5.1 outward the covered area shrinks to the second input while the frame stays on the first. The frame never moved; the alpha did.
- **A real difference between the versions, newly shown here.** From 5.1 the Mix node mixes the alpha as well as the colour. At Factor 0.5 with the large picture on A and the small one on B, a pixel outside the small picture comes out `[0.1, 0.1, 0.45, 1.0]` in 4.5 and 5.0 and `[0.1, 0.1, 0.45, 0.5]` in 5.1 and 5.2 — same colours, different alpha.
- **A second real difference.** The Mix node's starting Factor is 1.0 in 5.2 and 0.5 in the other three, which is what a file opened with nothing changed will use.

**What counts as a switch-like node type in a compositor tree, with the rule written down (entry 21).** Every name in `dir(bpy.types)` offered to `nodes.new()` on a compositor tree; the accepted ones kept if the type name contains the exact text `Switch`, and the older wider rule — `Switch` or `Index` or `Menu` — printed beside it, because entry 2 above never wrote its rule down and the citation check could not check one list against the other without it.

**What a negative result would have looked like.** The two rules could have given the same list in 5.2, which would leave the difference between the two earlier lists unexplained. `GeometryNodeSwitch` could have come back addable in 5.1 too, in which case it is not a 5.2 arrival. The addable counts could have come back unlike entry 2's, which would mean the two sweeps are not looking at the same thing.

Word for word, in this version:

> `ADDABLE NODE TYPE COUNT 115`
> `SWITCH IN THE TYPE NAME ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeMenuSwitch']`
> `THE OLDER, WIDER RULE — 'Switch' or 'Index' or 'Menu' in the type name: ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeMenuSwitch']`
> `NAMES THE WIDER RULE ADDS THAT HAVE NO 'Switch' IN THEM: []`

The two rules agree in this version, and the addable count is entry 2's count to the number. `GeometryNodeMenuSwitch` arrives here; it is not addable in 4.5. `GeometryNodeIndexSwitch` arrives in 5.1, and `GeometryNodeSwitch` and `FunctionNodeInputMenu` in 5.2.

**`is_inactive` on a Mix node with both picture inputs linked (entry 22).** A `ShaderNodeMix` set to `RGBA` and `MIX`, with a different picture linked to each of its two colour inputs and the tree's output linked up, printing `(name, identifier, is_inactive)` for every input socket, unmuted and then muted, **before any render and again after one**. Entry 5 above records this line for 5.2 only, and the citation check had nothing to set beside it.

**What a negative result would have looked like.** The 5.2 reading could have held at `True` across every Factor, which would make it a real difference between the versions and leave entry 5's line standing as written. It could have failed to reproduce at all. The reading could have changed after a render, which would mean both earlier runs read it at the wrong moment. And the Factor sweep could have shown no movement, which would leave the 5.2 line unexplained.

The Factor the node held, never set by the script, in this version:

> `A_Color is_linked: True   B_Color is_linked: True   Factor_Float held: 0.5`

and the line itself, word for word — identical before a render and after one, so the pair is printed once:

> `  unmuted is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', False), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`
> `  muted   is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', False), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`

This is one of the three `False` readings entry 5's 5.2 line was set against, and it stands. Muting changes nothing on this node in this version, which matches what entry 5 found for the other nodes here. 4.5 is the one version where muting does move the line.

**The Factor sweep, which is the control on all of it.** Word for word, and **identical in all four versions**:

> `FACTOR 0.0  A_Color is_inactive=False  B_Color is_inactive=True  (both linked: True, True)`
> `FACTOR 0.25  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 0.5  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 0.75  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 1.0  A_Color is_inactive=True  B_Color is_inactive=False  (both linked: True, True)`

So `is_inactive` on this node's two colour inputs behaves the same way in all four versions: A greys out at Factor 1.0, B at Factor 0.0, neither in between. Entry 5's 5.2-only line is the starting Factor of 1.0 showing through, the same fault as in entry 9 in a different place. And **what `is_inactive` reports is a Factor's arithmetic, not a wiring fact** — a socket with a picture linked to it still reads inactive when the Factor makes its contribution nil. So it cannot on its own say which picture reached the far side. `internal_links` and the renders of entries 5 and 7 are what do that, and they are unaffected.

### What a running Blender showed on the Creator's Mac

**LABEL: observed.** Everything in this section was got by running Blender 5.0.1 on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**. That Blender says of itself:

> `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)`

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-5.0`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure is in the hand-test document named above, entries 15 to 18.

**The graph reads the same with a window open as with none (entry 15).** One Blender built and saved the tree with no window; a second opened that file with no window and printed it; a third opened the same file with the window open and printed it. The run with a window, word for word:

> `TREE bpy.data.node_groups['Comp']`
> `NODES [('Chooser', 'CompositorNodeSwitch'), ('Colour', 'CompositorNodeHueSat'), ('Group Output', 'NodeGroupOutput'), ('Image', 'CompositorNodeImage'), ('Image.001', 'CompositorNodeImage'), ('TwoIn', 'CompositorNodeAlphaOver')]`
> `LINK Image.Image -> Colour.Image   muted=False valid=True`
> `LINK Colour.Image -> TwoIn.Background   muted=False valid=True`
> `LINK Image.001.Image -> TwoIn.Foreground   muted=False valid=True`
> `LINK Image.Image -> Chooser.Off   muted=False valid=True`
> `LINK Image.001.Image -> Chooser.On   muted=False valid=True`
> `LINK TwoIn.Image -> Group Output.Socket_0   muted=False valid=True`
> `CHOOSER internal_links: [('Off', 'Image')]`
> `TWOIN inputs: [('Background', 'Background', True, False), ('Foreground', 'Foreground', True, False), ('Factor', 'Fac', False, False), ('Type', 'Type', False, False), ('Straight Alpha', 'Straight Alpha', False, False)]`
> `TWOIN internal_links: [('Background', 'Image')]`

The Mac's run with no window printed those same lines, character for character. **This test also ran on Linux** — entry 11 above — and the Linux run printed the same lines too, apart from the file's own path. The two machines agree.

**A Viewer image appears in this version only when there is a window (entry 16).** The same script, one render of 64 by 64, run twice. With no window:

> `VIEWER IMAGE before the render: there is no image called 'Viewer Node'`
> `VIEWER IMAGE after the render: there is no image called 'Viewer Node'`

With a window:

> `VIEWER IMAGE before the render: there is no image called 'Viewer Node'`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; there is no Linux counterpart to the windowed case.

**Every property name on a socket is the same with a window as without (entry 17).** Both runs printed:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree. `is_inactive` and `is_multi_input` are both on that list, which are two of the things this document's *Not covered* section asks about; how `is_inactive` behaves is entry 5 above.

Both runs also printed the same four screen areas — `PROPERTIES`, `OUTLINER`, `DOPESHEET_EDITOR`, `VIEW_3D` — and the run with no window printed `SCREEN PRESENT: True` as well. So on macOS a Blender started with `-b` still has a screen with areas on it, and that printout does not tell a windowed run from a windowless one. What tells them apart is entry 16, where the Viewer image appeared only in the windowed run.

**A picture rendered with a window is the same picture as one rendered with none (entry 18).** The four renders of entry 4 were made twice, once with no window and once with a window, then loaded back and compared pixel by pixel — pictures against pictures, never files against files:

> `PAIR ao_0.0     vs win_ao_0.0      sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`
> `PAIR ao_1.0     vs win_ao_1.0      sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`
> `PAIR sw_False   vs win_sw_False    sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`
> `PAIR sw_True    vs win_sw_True     sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`

Both runs also printed the same four lines about what each render held, and they are the lines the Linux machine printed with no window (entry 4):

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

The two machines agree.

**One thing about ending a Blender that has a window.** The scripts were written to close Blender with `bpy.ops.wm.quit_blender()`. On this Mac that line on its own did not end it: the script's printing all appeared, and the process was still running when a limit of my own ended it. The same script with `sys.exit(0)` added after that line ended in about a second and printed the same thing. Both are recorded in entry 19 of the hand-test list.

### Not covered

- This does not settle whether the *content* of a blended output can be attributed to one input or the other. Nothing in the sources supports that, and I did not test it.
- ~~It does not settle what Mix does at either end of its Factor range, because the manual does not say which end favours A and which favours B.~~ **Settled by entry 3 above:** at Factor 0 the output is the A input, at Factor 1 the B input. The manual still does not say it; the machine does.
- ~~It does not settle whether `is_inactive` is ever set by compositor nodes, or whether `internal_links` on a given compositor node points at the first input, the second, or something else. Both would have to be read from a running Blender.~~ **Settled by entry 5 above:** in 5.0 `is_inactive` stays false throughout, muted or not, and `internal_links` names the first picture input on every node tested.
- It does not settle anything about multi-input sockets beyond the existence of `is_multi_input` and `multi_input_sort_id` in the API.
- It says nothing about performance, or about how much of the tree is actually evaluated, beyond the Menu Switch's own claim that "Unused inputs are not computed".

*[Changed by the citation check, 2026-08-06: two bullets struck through because tests printed earlier in this same document answered them, and a final bullet reading "Nothing here was verified by running Blender. Every claim is from reading." removed, because this document now carries a running-Blender section on two machines. Repeats of the two tests on 2026-08-07 came back the same.]*

### Outside the interface

**LABEL: observed for both halves, on Linux and on macOS.** On the rented Linux machine named above, on 2026-08-06, Blender 5.0.1 with `-b` opened a saved file and printed the whole graph — every link with both its nodes and both its sockets, the Switch node's stored choice, and each node's internal links. So the wiring, which this section calls file data, does read with no window; the inference below is replaced by a result for that half. On the Creator's Mac on 2026-08-07 the same file was read both with no window and with the window open, and the two printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round: the wiring reads the same whether Blender has its window open or not.

**The looking-at-it half was tested both ways too, and it turns on the window.** With no window, no image called `Viewer Node` exists in this version at all, on either machine. With a window open on the Mac, that image appeared after the render at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16). So the Viewer route is reachable when there is a window and not when there is none, which is what this section's reading of the sources pointed at.

**And what a script cannot reach either way.** Every property name on a socket was the same with a window as without, on both machines, and none of them names an inspection, a resolution, a transform, a precision, a sampling, a priority or a domain (entry 17).

**Choosing the graphics card, and having the choice acted on, both work with no window.** On 2026-08-07, on the same machine with its card repaired and its driver at 580.173.02, a Blender started with `-b` and its compositing device set to the card really did the compositing there — the card named that Blender's own process and it held 214 MiB while it worked, where the same run set to the processor never appeared on the card at all (entry 12). Neither the setting nor its effect needs a window.

The reading from the sources, unchanged, follows.

Split answer.

- **The wiring, and the settings that decide which input passes through, are file data.** `NodeLink.from_node` / `to_node`, `NodeSocket.identifier`, `Node.internal_links`, the Switch's checkbox and the Menu Switch's menu are all described as properties of the node tree, not of a window. **Inferred** that they can therefore be read with no window open: the sources describe them as data on the node tree and never tie them to an editor, but they do not say in so many words that they work headless. That step is mine.
- **The two ways of looking at the picture partway along are interface features by their own descriptions.** Node Preview is documented under the node editor's Show/Hide menu with a keyboard shortcut, and its own note says it "are only available in the Compositor" (that is, the compositor editor). The Viewer node's result is documented as appearing "in the backdrop or Image Editor", and `SpaceNodeEditor.show_backdrop` is described as "Use active Viewer Node output as backdrop for compositing nodes" (Blender Python API, SpaceNodeEditor, https://docs.blender.org/api/5.0/bpy.types.SpaceNodeEditor.html — read 2026-07-30). Both need an editor to show anything. **Documented** that they are editor features; **not in the sources** whether either produces anything when Blender is driven from outside its own window.
- The compositor system page does distinguish the two situations in which the compositor runs, which is the closest the sources come: "For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size" (Compositor System, address above, read 2026-07-30). It says nothing about whether domain priority, switch pass-through, or muting differ between them.
- The sources do not say that any of the rules quoted above — domain priority, the Switch's pass-through, Depth Combine's depth rule — change when Blender is driven from outside. They simply do not address it.

**This is the weakest part of the answer**, and it is worth saying plainly: the parts of "following a picture forward" that a person would actually use — the node preview and the Viewer backdrop — are the parts the sources tie to an editor window.


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card result added on 2026-08-07 was got on that same Linux machine once its graphics card had been repaired, and settles Linux alone in the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window; the four Alpha Over and Switch renders; no image called `Viewer Node` existing at all with no window; and every property name on a socket. Nothing came out differently on the two operating systems. The rest of the Linux tests are still owed on the Mac and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding.

The reading from the sources, unchanged, follows.

**Not in the sources.** Nothing in what I read makes any of the above depend on the operating system. I searched for such a statement and found only one operating-system-specific remark anywhere in the compositing chapter, and it is about something else:

> "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

— Blender manual, Limitations, https://docs.blender.org/manual/en/5.0/compositing/limitations.html — read 2026-07-30 (the page heads that passage "TDR on Windows")

That concerns how long a node may take on a graphics card under Windows. It has no bearing on which input reaches an output, or on whether a picture can be traced through the tree. I found no other operating-system-specific statement touching this question.

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
### Where I looked

**Files and folders searched.** The whole of `manual_5.0` (all subject areas, not only compositing), the whole of `api_5.0`, the whole of `release_notes_5.0`, and `cycles_5.0`.

**Pages opened in full.**

- Manual: `compositing/introduction.txt`, `compositing/compositor_system.txt`, `compositing/limitations.txt`, `compositing/types/color/mix_color.txt`, `compositing/types/color/alpha_over.txt`, `compositing/types/color/depth_combine.txt`, `compositing/types/color/adjust/hue_saturation.txt`, `compositing/types/color/adjust/color_balance.txt`, `compositing/types/utilities/switch.txt`, `compositing/types/utilities/menu_switch.txt`, `compositing/types/utilities/split.txt`, `compositing/types/input/image_info.txt`, `compositing/types/output/viewer.txt`, `interface/controls/nodes/editing.txt` (the Mute and Node Preview sections).
- API: `bpy.types.NodeLink`, `bpy.types.NodeSocket`, `bpy.types.Node`, `bpy.types.SpaceNodeOverlay`, `bpy.types.SpaceNodeEditor`, `bpy.types.ShaderNodeMix`, `bpy.types.CompositorNodeAlphaOver`, `bpy.types.CompositorNodeSwitch`, `bpy.types.CompositorNodeHueSat`.
- Release notes: `compositor.md` in full, `python_api.md` (the Nodes section and the Inline Shader Nodes section).

**Pictures opened.** `node-types_CompositorNodeSwitch.webp`, `node-types_CompositorNodeSplit.webp`, `node-types_CompositorNodeAlphaOver.webp`, `compositing_realtime-compositor_compositing-space_operation-domain_considerations_clipping.png`.

**Terms searched, and what came back.**

- Across the compositing chapter: *trace / traced / tracing*, *provenance*, *identity*, *identifie*, *which input* — the only hits were the mathematical sense of "identity transformation", the Menu Switch's "Determines which input socket is passed through to the output", and "ray-traced"/"ray tracing" in the Bilateral Blur and Denoise pages. Nothing about tracking a picture.
- Across the compositing chapter: *upstream*, *downstream*, *data flow*, *dataflow*, *provenance*, *attribution*, *originated*, *lineage* — zero hits for every one. *came from* — one hit, in the historical aside about paper watermarks. *source image* — three hits, all in Glare and Bilateral Blur and all meaning "the picture you fed in", not an origin record.
- Across the whole API documentation: *upstream*, *downstream*, *provenance*, *lineage* — zero hits each. *dependency graph* — nine files, all about the scene dependency graph rather than node trees, ruled out.
- Across the compositing chapter: *metadata* — one hit, in the Cryptomatte page, saying "Cryptomatte sidecars (metadata files) are not supported". Nothing about metadata travelling with a picture between nodes.
- Across the compositing chapter: *watermark*, *fingerprint*, *tag*, *marker*, *label* — hits were the Mix node's watermark tutorial, the Mask node's data-block label, and tracking markers in Keying Screen and Track Position. Ruled out as unrelated except the watermark tutorial, which is reported above.
- Across the compositing chapter: *passed to / passed through / pass through / unchanged / passthrough* — hits were the Switch, the Menu Switch, the Enable Output node, and the Box/Ellipse Mask pages ("passed through unchanged" there means unmasked areas of one picture, not a whole input). All reported or ruled out above.
- Across the whole manual: *muting* — four files; only `interface/controls/nodes/editing.txt` was relevant. *node preview* — two hits, both in the same file, quoted above.
- Across the whole manual: *domain priority* — one file, `compositing/compositor_system.txt`, quoted above. Across the whole API documentation: *domain priority* — zero hits, so the priority order is not published as a readable property.
- Across `cycles_5.0`: *composit* — nine files. `ui.py` draws the compositor performance panels and the `use_compositing` checkbox; `properties.py` mentions compositing only in the description of the transparent-glass option; the rest are unrelated words inside kernel source. **Ruled out:** Cycles' files carry no compositor node data. Nothing there bears on this question.
- Release notes, `python_api.md`: the "Inline Shader Nodes" section describes a new API that "inlines node groups and eliminates reroute and muted nodes". **Ruled out** — it says *shader* nodes throughout and names `InlineShaderNodes`. I found no compositor counterpart, and I did not follow the web address printed in that note.

**One dead end worth recording.** There is no `bpy.types.CompositorNodeMix` page in the 5.0 API documentation. The compositor's Mix node is the shared `bpy.types.ShaderNodeMix`, which the 5.0 release notes confirm as the pattern for such moves: "Many compositor nodes like the Gamma Node `CompositorNodeGamma` were replaced by their Shader Node counterpart like `ShaderNodeGamma`" (Blender 5.0 release notes, Python API, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30). Anyone looking up the Mix node by a compositor name will find nothing.

---

### Things a person with Blender open could settle in a few minutes

**All six of these have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above. The two things the Linux machine could not reach were run on the Creator's Mac on 2026-08-07, and are in *What a running Blender showed on the Creator's Mac*: the graph read the same with Blender's window open as with none, and a Viewer image did appear when a window was open, where none exists at all without one. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **Most of the tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Does a muted two-input node say which input it hands over?**
   Open the compositor. Add an Alpha Over node and connect two different pictures to Background and Foreground. Select it and press M to mute it. In the Python console, run `n = C.scene.compositing_node_group.nodes.active` then `[(l.from_socket.identifier, l.to_socket.identifier) for l in n.internal_links]`.
   *Negative result:* the list comes back empty, or names a socket other than one of the two picture inputs. Either would mean the file does not record which input passes through when muted.
   Repeat for Mix and for Split.

2. **Does the compositor mark a socket that has no effect on the output?**
   Same Alpha Over node, unmuted, Factor set to 0. Run `[(s.name, s.is_inactive) for s in n.inputs]`.
   *Negative result:* every socket reports `False`, meaning the compositor does not populate `is_inactive` and it cannot be used to tell which input reached the far side.

3. **Which end of Mix's Factor gives you A, and which gives you B?**
   Feed a pure red picture into A and a pure green one into B, with blend mode Mix. Set Factor to 0, attach a Viewer, and sample the colour in the Image Editor. Then set Factor to 1 and sample again.
   *Negative result:* both readings come back the same, or come back as a blend at both extremes — which would mean the Factor does not resolve to a single input at its ends.

4. **Does the domain really come from the named input?**
   Make a large background picture and a small foreground one. Wire them into Alpha Over the documented way and note the size of the result via an Image Info node. Then swap the two.
   *Negative result:* the result's Resolution is the same both ways round, which would contradict the manual's clipping claim.

5. **Do node previews and the Viewer produce anything with no window open?**
   Run Blender from a command line with `--background` on a file whose compositor has a Viewer node, and render one frame with a script that afterwards inspects `bpy.data.images` for a Viewer Node entry.
   *Negative result:* no such image exists, or it is blank — meaning the Viewer is of no use when Blender is driven from outside, and only file-side link walking is.

6. **Do the two Split inputs really have different identifiers despite sharing a name?**
   Add a Split node and run `[(s.name, s.identifier) for s in n.inputs]`.
   *Negative result:* the two identifiers come back equal, which would mean the two picture inputs cannot be told apart from the file at all.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 5.0

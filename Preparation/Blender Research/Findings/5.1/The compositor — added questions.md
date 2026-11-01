# The compositor — added questions (Blender 5.1)

**Sources consulted, and when.** Blender's manual (`manual_5.1`, including its pictures in `_images`), Blender's Python API documentation (`api_5.1`), Blender's release notes (`release_notes_5.1`), and Cycles' own files (`cycles_5.1`) — all read on **2026-07-30** (the snapshot date). Blender's requirements page was not used: it does not name 5.1 in so many words, so it cannot speak for this version, and nothing in question 8 turns on it.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Only question 8 was in scope. Every other question on the sheet was left alone.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

### The question is ambiguous in two places, and I say so before answering

**First ambiguity — what "followed forward" means.** It can mean either:

- (a) following the *connection*: reading the node tree and seeing which node the output of one node is wired into, and onward; or
- (b) following the *picture itself*: recognising, in the data that comes out the far side, the picture that went in.

These two come apart badly at the second node in the question, so I answer both and label them separately. If only one reading was meant, reading (a) is the one I would bet on, because the question asks whether the picture can be "followed" *through* named node types rather than whether it can be recognised.

**Second ambiguity — what "a node that takes two pictures in and gives one out" is.** In this version that description fits two different families of node, and the answer to "which of the two came out" is completely different for each:

- nodes that **choose** one of the two and pass it on — Switch, Index Switch, Menu Switch, and (per pixel) Depth Combine;
- nodes that **blend** the two into something new — Mix, Alpha Over, Split.

I cover both families. I have not rewritten the question.

---

### 8a. Following the connection through a colour-changing node

**LABEL: documented.**

The colour-changing nodes named in the manual as a group take one image in and put one image out, so the wire can be picked up again on the far side.

The group itself:

> "These nodes adjust the image's colors, for example increasing the contrast, making it warmer, overlaying another image, etc."

— Blender manual, Color Nodes, https://docs.blender.org/manual/en/5.1/compositing/types/color/index.html — read 2026-07-30

Four of them, each showing one image socket in and one image socket out:

> "The RGB Curves Node performs level adjustments on each color channel."
> … "Inputs / Image/Color / Standard color input."
> … "Outputs / Image/Color / Standard color output."

— Blender manual, RGB Curves Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/rgb_curves.html — read 2026-07-30

> "The Hue/Saturation/Value Node applies a color transformation in the HSV Color Model."
> … "Inputs / Image/Color / Standard color input."
> … "Outputs / Image/Color / Standard color output."

— Blender manual, Hue/Saturation/Value Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/hue_saturation.html — read 2026-07-30

> "The Color Balance node adjusts the color and values of an image."
> … "Inputs / Image / Standard color input."
> … "Outputs / Image / Standard output image."

— Blender manual, Color Balance Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/color_balance.html — read 2026-07-30

> "Inputs / Image / Standard color input." … "Outputs / Image / Standard color output."

— Blender manual, Brightness/Contrast Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/bright_contrast.html — read 2026-07-30

The Python API says the same of RGB Curves:

> "Perform level adjustments on each color channel of an image"

— Blender Python API, `CompositorNodeCurveRGB`, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeCurveRGB.html — read 2026-07-30

**From a picture.** The manual's screenshot of the RGB Curves node (`_images/node-types_CompositorNodeCurveRGB.webp`, shown on the RGB Curves page above) shows one socket named `Image` on the output side and, on the input side, `Fac`, `Image`, `Black Level`, `White Level` — one picture in, one picture out. Label for that: **documented** (it is part of the manual). What is visible is the socket names and count; nothing more should be read into it.

That an output can be traced onward at all is stated generally:

> "The outputs are located on the top right side of the node, and can be connected to the input of nodes further down the node tree."

— Blender manual, Node Parts, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/parts.html — read 2026-07-30

And the connection itself is a readable, directional thing with both ends named:

> "Link between nodes in a node tree"
> … "from_node … (readonly) … Type: Node"
> … "from_socket … (readonly) … Type: NodeSocket"
> … "to_node … (readonly) … Type: Node"
> … "to_socket … (readonly) … Type: NodeSocket"
> … "is_muted / Link is muted and can be ignored (default False)"

— Blender Python API, `NodeLink`, https://docs.blender.org/api/5.1/bpy.types.NodeLink.html — read 2026-07-30

Nodes are individually named:

> "Name / A unique identifier for the node within its node tree. This name must be unique and is used internally for references."

— Blender manual, Node Editors, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html — read 2026-07-30

So, on reading (a), this much is carried by the words above and is **documented**: a colour-changing node has one image input and one image output; every link states which node and which socket it leaves and which node and which socket it enters; and every node has a unique name.

Going from there to *the chain can be walked forward step by step* takes one step the quotes do not make. Nothing quoted here says how the links of a node tree are reached in the first place — only what a link holds once you already have one. That step is **inferred**, and it is mine, not the sources'.

---

### 8b. Following the picture itself through a colour-changing node

**LABEL: not in the sources.**

Nothing in the sources describes a name, tag, identifier, or piece of metadata that travels with an image from one compositor node to the next. What the sources do say is that the data flowing between compositor nodes is nothing but numbers and a rectangle:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats:"
> "Float / A signed floating-point number. Integer data is also stored as floats because no integer type exist."
> "Vector / A 4D vector. …"
> "Color / A 4D vector storing the Red, Green, Blue, and Alpha of the color. The color is free form and does not conform to a specific color space or alpha storage model …"

> "The compositor is designed in such a way as to allow compositing in an infinite compositing space. Consequently, images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations. … The rectangular area occupied by an image in that space as defined by its transformation and size is called the Domain of the image."

— Blender manual, Compositor System, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30

An image between nodes is therefore pixel values plus a domain. The sources name no other property of it.

The one thing in the sources that lets metadata into the compositor at all is a release note, and it is confined to the two nodes that touch files:

> "The **Image** and **File Output** nodes now read and write custom meta-data."

— Blender release notes, 5.1 Compositor, https://developer.blender.org/docs/release_notes/5.1/compositor/ — read 2026-07-30

That note is about reading metadata off a file and writing metadata into a file. The sources say nothing about metadata being carried along a link from one node to the next, and the manual's compositing pages do not mention metadata at all (the only hit anywhere under `compositing/` is a Cryptomatte note that sidecar metadata files are not supported).

There is one documented *technique* for recognising a picture inside a result, but it is something a person builds by hand, not a property the compositor maintains:

> "Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

> "When you see an image that you think might be yours, use the node tree below to compare it to your stock image (non-watermarked original). In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference. You can see how the original mark clearly stands out."

— Blender manual, Mix Color Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/mix_color.html — read 2026-07-30

That is **documented**, and it is worth knowing, but it detects a mark that was deliberately put in beforehand. It is not the compositor keeping track of anything.

So: on reading (b), through a colour-changing node — not in the sources. The node changes the colours, and the sources describe nothing that survives the change to say what the picture was.

The two standing lines for this part follow from that. The sources name nothing that travels with the picture, so there is nothing here to hold or fail to hold when Blender is driven from outside its window, and nothing to hold or fail to hold on one operating system rather than another. Both lines for 8b are therefore **not in the sources**, on the record of looking set out at the end of this document.

---

### 8c. Following the connection through a node that takes two pictures in and gives one out

**LABEL: documented.**

Every such node in the sources has a single image output, and the same link mechanism (8a) applies to it. Both families:

Blending family —

> "The Alpha Over node composites two images by layering the Foreground image over the Background, using standard alpha blending operations."
> … "Inputs / Background / The background image." … "Foreground / The foreground image to be placed over the background."
> … "Outputs / Image / The combined image after alpha compositing."

— Blender manual, Alpha Over Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/alpha_over.html — read 2026-07-30

> "The Mix Node mixes values, colors and vectors inputs using a factor to control the amount of interpolation."
> … "A/B / The two inputs that are mixed together."
> … "Outputs / Result / Output the result of the mix using the data type selected."

— Blender manual, Mix Color Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/mix_color.html — read 2026-07-30

> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line."
> … "Outputs / Image / The combined image showing both inputs divided by the split line."

— Blender manual, Split Node, https://docs.blender.org/manual/en/5.1/compositing/types/utilities/split.html — read 2026-07-30

Choosing family —

> "Switch between two images using a checkbox."
> … "Inputs / Switch / When it is unchecked, the first input labeled "Off" is passed to the output. / When checked, the second input labeled "On" is passed to the output."
> … "Off / First image input." … "On / Second image input."
> … "Outputs / Image / Standard color output."

— Blender manual, Switch Node, https://docs.blender.org/manual/en/5.1/compositing/types/utilities/switch.html — read 2026-07-30

> "The Depth Combine node combines two images based on their depth maps. It overlays the images using the provided depth values to detect which parts of one image are in front of the other."
> … "Outputs / Result / If both depth values are equal, it will use the foreground image. Whichever depth value is less decides which image pixel is used."

— Blender manual, Depth Combine Node, https://docs.blender.org/manual/en/5.1/compositing/types/color/depth_combine.html — read 2026-07-30

**From pictures.** The manual's screenshot of the Alpha Over node (`_images/node-types_CompositorNodeAlphaOver.webp`, on the Alpha Over page) shows one socket named `Image` on the output side and, on the input side, `Fac`, `Image`, `Image`, `Straight Alpha` — two picture sockets in, one out. Note what is visible and what is not: in this picture **both input sockets carry the same name, `Image`**, while the page's text names them `Background` and `Foreground`. I report only what is visible; I do not draw a conclusion about which is which from the picture. The screenshot of the Switch node (`_images/node-types_CompositorNodeSwitch.webp`, on the Switch page) shows one output socket named `Image` and inputs `Switch`, `Off`, `On`. Label for both: **documented**.

So the connection can be followed forward through these nodes as well: one output, traceable by the same links.

---

### 8d. Can it be told which of the two came out the far side?

This is where the two families separate.

**Choosing family — LABEL: documented, yes, and the sources say what decides it.**

For Switch, the output *is* one of the two inputs, and the setting says which:

> "When it is unchecked, the first input labeled "Off" is passed to the output. / When checked, the second input labeled "On" is passed to the output."

— Blender manual, Switch Node, address above — read 2026-07-30

With the caveat, from the same page, that the choice is not fixed:

> "Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

And the manual's own screenshot of that node shows `Switch` drawn as an input socket. What is visible in the picture is the socket and nothing else; that it can therefore be fed from elsewhere in the tree rather than only ticked by hand is a step past what the picture shows, and that step is **inferred**. So "which of the two came out" is answerable, and the keyframe words above already put the answer *per frame* rather than once for the whole file.

The two related nodes say it even more plainly:

> "Determines which of the following input sockets is passed through to the output. The index is zero-based (i.e., the first input corresponds to index 0)."
> … "Outputs / Output / Outputs the value from the input corresponding to the current Index value, without any modification."

— Blender manual, Index Switch Node, https://docs.blender.org/manual/en/5.1/compositing/types/utilities/index_switch.html — read 2026-07-30

> "Outputs the value from the input corresponding to the selected menu item, unchanged."

— Blender manual, Menu Switch Node, https://docs.blender.org/manual/en/5.1/compositing/types/utilities/menu_switch.html — read 2026-07-30

Index Switch is new to this version:

> "The **Index Switch** node is now supported. (blender/blender@33d8d2b6f2)"

— Blender release notes, 5.1 Compositor, https://developer.blender.org/docs/release_notes/5.1/compositor/ — read 2026-07-30

Depth Combine also chooses, but it chooses **per pixel**, and what decides it is the two depth inputs, not a setting:

> "Whichever depth value is less decides which image pixel is used."

— Blender manual, Depth Combine Node, address above — read 2026-07-30

So for Depth Combine the honest answer is that both pictures came out, in different places, and which one at any given pixel is decided by data, not by anything readable off the node.

**Blending family — two labels, because two different things are being said.**

That the output is described as a *combination* rather than as one of the two inputs is **documented**: "the combined image after alpha compositing", "the result of the mix", "the combined image showing both inputs divided by the split line" — all quoted above, for Alpha Over, Mix and Split. Neither of the two "came out" — a third thing did.

That nothing on the output says which input contributed what is **not in the sources**. I searched for a marker, channel or property of that kind (terms listed below) and found none.

The Split node is the partial exception worth naming: the quoted words say the output shows both inputs divided by a split line, so both inputs appear in the output, each in its own region of the frame. What sets that boundary is not among the words I have quoted here, so I say nothing about it. Either way it is still not "which of the two came out" — the answer is "both, in known places".

**One further case, in both families: a muted node. LABEL: inferred — the step is written out at the end of this passage.**

> "Mute / Temporarily disables the node without removing it. When possible, the node is bypassed and its inputs are passed directly to its outputs."

— Blender manual, Node Editors, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html — read 2026-07-30

> "internal_links / Internal input-to-output connections for muting (default None, readonly) / Type: bpy_prop_collection[NodeLink]"
> … "mute / (default False) / Type: bool"

— Blender Python API, `Node`, https://docs.blender.org/api/5.1/bpy.types.Node.html — read 2026-07-30

Read together: when a two-input node is muted, one of its inputs is passed straight to its output, and `internal_links` is described as holding exactly those input-to-output connections. **Inferred**, and here is the step a reader has to supply: the manual says muting passes inputs directly to outputs, and the API says `internal_links` holds the input-to-output connections used for muting — neither source states outright that reading `internal_links` on a muted compositor node will tell you which of two image inputs emerges. That is my joining of the two, not something either says.

Also worth recording, because it bears on whether a link is live at all: a link itself can be muted — `is_muted`, "Link is muted and can be ignored" (`NodeLink`, address above).

---

### What a running Blender showed

**LABEL: observed.** Everything in this section was got by running Blender 5.1.2 on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, and each Blender was pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics' tests while this ran, so nothing here reports how long anything took. The full procedure, every script in full, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only. The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**A control first (entry 1).** A picture put through a bare tree and written out came back identical to the picture that went in, and the same render twice gave the same picture:

> `MAXDIFF render vs source 0.0`
> `MAXDIFF render vs same render again 0.0`

So every comparison below is a comparison of pictures, not of files, and a difference of zero means the pixels match.

**Which nodes take two pictures in, and what their sockets are called (entry 2).**

> `ADDABLE NODE TYPE COUNT 122`
> `TWO-PICTURE-IN, ONE-PICTURE-OUT COUNT 25`
> `SWITCH-LIKE NAMES AMONG ADDABLE ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch']`

So the Index Switch and Menu Switch this answer describes are not compositor node types at all — they are the geometry-node ones, added to a compositor tree.

> `PRESENT CompositorNodeAlphaOver label: Alpha Over`
> `   IN  [('Background', 'Background', 'RGBA'), ('Foreground', 'Foreground', 'RGBA'), ('Factor', 'Fac', 'VALUE'), ('Type', 'Type', 'MENU'), ('Straight Alpha', 'Straight Alpha', 'BOOLEAN')]`
> `   OUT [('Image', 'Image', 'RGBA')]`
> `PRESENT CompositorNodeSwitch label: Switch`
> `   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`
> `PRESENT CompositorNodeSplit label: Split`
> `   IN  [('Position', 'Position', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`
> `PRESENT CompositorNodeZcombine label: Depth Combine`
> `   IN  [('A', 'A', 'RGBA'), ('Depth A', 'Depth A', 'VALUE'), ('B', 'B', 'RGBA'), ('Depth B', 'Depth B', 'VALUE'), ('Use Alpha', 'Use Alpha', 'BOOLEAN'), ('Anti-Alias', 'Anti-Alias', 'BOOLEAN')]`
> `   OUT [('Result', 'Result', 'RGBA'), ('Depth', 'Depth', 'VALUE')]`

So the two Alpha Over picture inputs are named Background and Foreground here, not both `Image` as the manual's picture shows; Depth Combine's picture inputs are named A and B; and Depth Combine has a second output, `Depth`, which this answer did not establish.

**What comes out of a Mix at the ends of its factor (entry 3).**

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

**Alpha Over at the ends of its factor, and the Switch node (entry 4).**

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

The foreground picture used was opaque throughout, which is why factor 1 gives it back whole. The Switch node hands the chosen picture through with no change at all.

**What the file says about which input reaches the output (entry 5).**

> `=== AlphaOver (CompositorNodeAlphaOver) ===`
> `  unmuted internal_links: [('Background', 'Image')]`
> `  unmuted is_inactive: [('Background', 'Background', False), ('Foreground', 'Foreground', False), ('Factor', 'Fac', False), ('Type', 'Type', False), ('Straight Alpha', 'Straight Alpha', False)]`
> `  muted   internal_links: [('Background', 'Image')]`
> `  muted   is_inactive: [('Background', 'Background', False), ('Foreground', 'Foreground', False), ('Factor', 'Fac', False), ('Type', 'Type', False), ('Straight Alpha', 'Straight Alpha', False)]`
> `=== Switch (CompositorNodeSwitch) ===`
> `  unmuted internal_links: [('Off', 'Image')]`
> `=== Split (CompositorNodeSplit) ===`
> `  unmuted internal_links: [('Image', 'Image')]`
> `=== Z/Depth Combine (CompositorNodeZcombine) ===`
> `  unmuted internal_links: [('A', 'Result'), ('Depth B', 'Depth')]`
> `=== Hue/Saturation/Value (CompositorNodeHueSat) ===`
> `  unmuted internal_links: [('Image', 'Image')]`

So `internal_links` is filled in for compositor nodes and names one picture input on every one of them, muted or not. `is_inactive` stayed false throughout.

Sweeping every property name on a socket and on the Alpha Over node for anything naming a priority or a domain:

> `  ANY NAMING PRIORITY OR DOMAIN: []`

**The Switch node's choice fed by a link (entry 6).**

> `VALUE NODE USED ShaderNodeValue`
> `LINK FROM VALUE NODE ACCEPTED, is_valid: True`
> `LINK FROM A PICTURE ACCEPTED, is_valid: True`
> `WITH A PICTURE ON THE SWITCH INPUT: maxdiff vs picA=0.0  maxdiff vs picB=1.0`

The picture fed in ran from 0 on the left of the frame to 1 on the right. The whole frame still came out as the Off input.

**What a muted node passes, rendered (entry 7).**

> `MUTED AlphaOver            internal_links=[('Background', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Mix                  internal_links=[('A_Color', 'Result_Color')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Switch               internal_links=[('Off', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Split                internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Z/DepthCombine       internal_links=[('A', 'Result'), ('Depth B', 'Depth')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `COLOUR NODE HueSat     muted=False maxdiff vs the picture that went in = 1.0`
> `COLOUR NODE HueSat     muted=True  maxdiff vs the picture that went in = 0.0`
> `COLOUR NODE RGBCurves  muted=False maxdiff vs the picture that went in = 0.5`
> `COLOUR NODE RGBCurves  muted=True  maxdiff vs the picture that went in = 0.0`

**The Split node's two sides, and Depth Combine pixel by pixel (entry 8).**

> `   SPLIT left column x=5 y=32 -> [0.07937, 0.50794, 0.25, 1.0]   (picA there [0.07937, 0.50794, 0.25, 1.0], picB there [0.1, 0.92063, 0.8, 1.0])`
> `   SPLIT right column x=58 y=32 -> [0.1, 0.07937, 0.8, 1.0]   (picA there [0.92063, 0.50794, 0.25, 1.0], picB there [0.1, 0.07937, 0.8, 1.0])`

so the first input lands on the left of the frame and the second on the right.

> `DEPTH COMBINE first=picA(depth 0.2) second=picB(depth 0.8): maxdiff vs picA=0.0 maxdiff vs picB=1.0`
> `DEPTH COMBINE first=picA(depth 0.8) second=picB(depth 0.2): maxdiff vs picA=1.0 maxdiff vs picB=0.0`
> `DEPTH COMBINE with a depth running 0 on the left to 1 on the right for picA, 0.5 flat for picB:`
> `   DEPTHGRAD left column x=5 y=32 -> [0.07937, 0.50794, 0.25, 1.0]   (picA there [0.07937, 0.50794, 0.25, 1.0], picB there [0.1, 0.92063, 0.8, 1.0])`
> `   DEPTHGRAD right column x=58 y=32 -> [0.1, 0.07937, 0.8, 1.0]   (picA there [0.92063, 0.50794, 0.25, 1.0], picB there [0.1, 0.07937, 0.8, 1.0])`
> `   maxdiff vs picA=1.0  maxdiff vs picB=1.0`

**Which input decides the frame (entry 9).** A 32 by 32 picture and a 128 by 128 picture, rendered at 128 by 128, reading the area whose alpha is above zero:

> `AlphaOver(bg,fg)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `AlphaOver(bg,fg)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`
> `Mix(A,B)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `Mix(A,B)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`

This is the thing the answer above says it did not establish: a colour node's output area, and a two-picture node's, does follow one named input.

*[Note added 2026-08-07 by entry 20, below. This entry's conclusion for 5.1 is right, and entry 20 confirms it by reading the frame straight off the socket instead of inferring it from the alpha. Two cautions belong with it even so. The Factor was never written down here, and it matters: a Mix node starts at Factor 0.5 in this version and at 1.0 in 5.2, which is what made 5.2 look unlike the rest. And the area whose alpha is above zero is a reading of the alpha, not of the frame — in this version, at Factor 1.0, the two disagree, and only the direct reading is a reading of the frame. Entry 20 is what to read.]*

**A Viewer node, a node group boundary, and a File Output name (entry 10).** With no window, and the render 64 by 64:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `   image 'Viewer Node' source=VIEWER size=(256, 256) pixel count=262144`

A picture into a node group holding a colour-changing node:

> `WALK FROM THE PICTURE NODE, in the outer tree:`
> `   link -> node 'Group' (CompositorNodeGroup), socket 'Socket_0'`
> `   the group node's node_tree is 'Inner'; walking inside it:`
> `      inner link: NodeGroupInput.Socket_0 -> CompositorNodeHueSat.Image`
> `      inner link: CompositorNodeHueSat.Image -> NodeGroupOutput.Socket_1`
> `   any link in the OUTER tree that reaches a node inside the group: []`

A File Output node whose input item was named `MyBranchName`:

> `file_output_items after: ['MyBranchName']`
> `file_name is 'file_name'`
> `FILE OUTPUT input sockets: [('MyBranchName', 'Item_1'), ('', '__extend__')]`
> `WHAT LANDED IN THE FILE OUTPUT FOLDER: ['file_name.exr']`

**Reading the whole graph out of a saved file with no window (entry 11).** The tree was built and saved by one Blender and read back by a second, separate one, with no window at either end:

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

**The processor against the card, and whether a socket reports its picture (entry 12).** Every run on this machine printed these lines first:

> `libEGL warning: egl: failed to create dri2 screen`
> `EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).`

then:

> `compositor_device present: True value: CPU`
> `compositor_precision present: True value: AUTO`
> `MAXDIFF between the processor's picture and the card's picture: 6.854534149169922e-07`

This machine has no display server and `nvidia-smi` on it would not start (`Failed to initialize NVML: Driver/library version mismatch`), so whether the run marked `GPU` really used the card is not settled. What is settled is that changing that setting changed the picture, by about seven parts in ten million.

**The card was repaired, and the card half was run again on 2026-08-07 (entry 12).** The same rented Linux machine — **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and still no display server at all** — now has a working graphics card, its kernel driver and its driver library agreeing at last. The card answers:

> `NVIDIA-SMI 580.173.02             Driver Version: 580.173.02     CUDA Version: 13.0`
> `NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.173.02  Release Build`
> `NVIDIA GeForce RTX 4080 SUPER, 580.173.02, 16376 MiB`

and no run printed an EGL line of any kind. **The lines just above are superseded by what follows**: with the card unreachable on 2026-08-06, that run marked `GPU` cannot have been the card's work, so it was never a comparison of the processor against the card.

The same tree — a colour node and an Alpha Over at factor 0.5 — rendered once with the compositing device set to `CPU` and once to `GPU`, and the two pictures loaded back and compared pixel by pixel:

> `compositor_device present: True value: CPU`
> `compositor_precision present: True value: AUTO`
> `DEVICE CPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `DEVICE GPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `MAXDIFF between the processor's picture and the card's picture: 6.258487701416016e-07`

**The run marked `GPU` really does use the card.** A second, heavier tree — a 1024 by 1024 picture through sixty Blur nodes, rendered at 2048 by 2048 — was run twice, one device to a Blender, while the card was asked over and over which processes were on it. Only this Blender's own process was counted, because the machine is shared with other topics' tests. Set to the processor:

> `ASKED FOR DEVICE CPU`
> `compositor_device now reads: CPU`
> `HEAVY RENDER SECONDS CPU 4.705`
> `HEAVY PROCESSOR CORE-SECONDS CPU 38.420`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 116`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`

Set to the card:

> `ASKED FOR DEVICE GPU`
> `compositor_device now reads: GPU`
> `HEAVY RENDER SECONDS GPU 0.977`
> `HEAVY PROCESSOR CORE-SECONDS GPU 1.210`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 40`
> `TIMES THIS PROCESS WAS ON THE CARD: 13`
> `MOST CARD MEMORY THIS PROCESS HELD: 150 MiB`

Only the setting differed between those two runs, and three things move together. Set to the processor, this Blender never appeared on the card at all. Set to the card, it appeared on it holding 150 MiB, and its own share of processor work fell from thirty-eight core-seconds to just over one.

**And the card's picture matches the processor's picture.** The two heavy renders, loaded back and compared pixel by pixel:

> `HEAVY CPU SIZE (2048, 2048) HEAVY GPU SIZE (2048, 2048)`
> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`
> `HEAVY MIDDLE PIXEL, processor: [0.500416, 0.500416, 0.25, 1.0]`
> `HEAVY MIDDLE PIXEL, card     : [0.500416, 0.500416, 0.25, 1.0]`

So the two devices give the same picture to about four parts in ten million on the heavy tree and about six parts in ten million on the small one. Not identical, and not far off: a picture cannot be told apart by its exact values across the two devices, but any test coarser than a millionth still holds. The seconds above are rough, because the machine was shared while this ran; the count of times this process was on the card is not, because it names one process by its own number.

Every property name on a colour node's output socket, after a render:

> `ANY NAMING INSPECTION, RESOLUTION, TRANSFORM, PRECISION, SAMPLING, VALUE OR IMAGE: ['default_value', 'hide_value']`
> `default_value on the output socket: <bpy_float[4], NodeSocketColor.default_value>`

`default_value` on a colour socket holds four numbers, which is a socket's own fixed colour rather than the picture that travelled through it. Nothing on a socket reports a name, a source, or an identifier for the picture.

**Does a faint mark survive a colour change (entry 13).**

> `BEFORE any colour change: biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `AFTER the colour change  : biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `MARKED-AND-GRADED against the UNMARKED ORIGINAL: inside the mark = 0.607381, outside it = 1.000000`

**Is there any metadata on the compositor's own nodes (entry 14).** This is the release note about the Image and File Output nodes reading and writing custom metadata, put to the machine:

> `CompositorNodeImage: properties naming metadata -> none`
> `   its input socket names: []`
> `   its output socket names: [('Image', 'Image', 'RGBA'), ('Alpha', 'Alpha', 'VALUE')]`
> `CompositorNodeOutputFile: properties naming metadata -> none`
> `CompositorNodeImageInfo: properties naming metadata -> none`
> `Image data-block properties naming metadata -> none`

**Added 2026-08-07 — the three tests the citation check sent back (entries 20, 21 and 22 of the hand-test list).**

**LABEL: observed.** Everything in this sub-section was got by running Blender 5.1.2 on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB with its kernel driver and its driver library both at 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all**, reached at `ssh -i ~/.ssh/id_ed25519 -p 39348 root@85.218.235.6` — on **2026-08-07**. Every run used `-b --factory-startup` and was pointed at a settings folder of its own, made beforehand under `/root/again-compositor/`, which is a third working folder outside both earlier sittings and outside every other topic's work. The Blender named itself with the same build hash as on 2026-08-06, so this is the same build. The machine was shared with other topics while this ran, so nothing here reports how long anything took or how much of the card was in use, and nothing here needed either. **Pictures were compared as pictures, never as files.** The full procedure, every script, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 20, 21 and 22. This settles Linux alone; none of the three has been run on the Creator's Mac.

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
> `      pixel (10,10), outside the small picture: [0.1, 0.1, 0.45, 0.5]    pixel (64,64), inside both: [0.55, 0.2, 0.55, 1.0]`
> `Mix(A,B)  FACTOR SET 1.0  FACTOR HELD ON THE NODE 1.0  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.0, 0.0, 0.0, 0.0]    pixel (64,64), inside both: [0.2, 0.2, 0.9, 1.0]`
> `Mix(A,B)  FACTOR SET 1.0  FACTOR HELD ON THE NODE 1.0  A=picLarge(128) B=picSmall(32) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]`
> `      pixel (10,10), outside the small picture: [0.0, 0.0, 0.0, 0.0]    pixel (64,64), inside both: [0.9, 0.2, 0.2, 1.0]`

5.2 gave the same twelve lines. 4.5 and 5.0 differ in two places, both of them an alpha: the pixel outside the small picture reads alpha 1.0 in them where it reads 0.5 here at Factor 0.5, and alpha 1.0 where it reads 0.0 here at Factor 1.0 — which is why their Factor 1.0 line reports 128 by 128 and this one reports 32 by 32.

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

> `ADDABLE NODE TYPE COUNT 122`
> `SWITCH IN THE TYPE NAME ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch']`
> `THE OLDER, WIDER RULE — 'Switch' or 'Index' or 'Menu' in the type name: ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch']`
> `NAMES THE WIDER RULE ADDS THAT HAVE NO 'Switch' IN THEM: []`

The two rules agree in this version, and the addable count is entry 2's count to the number. `GeometryNodeIndexSwitch` arrives here; it is not addable in 4.5 or 5.0. `GeometryNodeSwitch` is **not** addable here — it arrives in 5.2, which is the thing the citation check asked to have dated.

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

**LABEL: observed.** Everything in this section was got by running Blender 5.1.2 on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**. That Blender says of itself:

> `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)`

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-5.1`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 15 to 18.

**The graph reads the same with a window open as with none (entry 15).** One Blender built and saved the tree with no window; a second opened that file with no window and printed it; a third opened the same file with the window open and printed it. The run with a window, word for word:

> `VERSION 5.1.2`
> `FILE /Users/artist/Documents/Smoozy hand tests/The compositor/graph_5_1.blend`
> `TREE bpy.data.node_groups['Comp']`
> `NODES [('Chooser', 'CompositorNodeSwitch'), ('Colour', 'CompositorNodeHueSat'), ('Group Output', 'NodeGroupOutput'), ('Image', 'CompositorNodeImage'), ('Image.001', 'CompositorNodeImage'), ('TwoIn', 'CompositorNodeAlphaOver')]`
> `LINK Image.Image -> Colour.Image   muted=False valid=True`
> `LINK Colour.Image -> TwoIn.Background   muted=False valid=True`
> `LINK Image.001.Image -> TwoIn.Foreground   muted=False valid=True`
> `LINK Image.Image -> Chooser.Off   muted=False valid=True`
> `LINK Image.001.Image -> Chooser.On   muted=False valid=True`
> `LINK TwoIn.Image -> Group Output.Socket_0   muted=False valid=True`
> `CHOOSER stored choice: [('Switch', 'Switch', False), ('Off', 'Off', bpy.data.node_groups['Comp'].nodes["Chooser"].inputs[1].default_value), ('On', 'On', bpy.data.node_groups['Comp'].nodes["Chooser"].inputs[2].default_value)]`
> `CHOOSER internal_links: [('Off', 'Image')]`
> `TWOIN inputs: [('Background', 'Background', True, False), ('Foreground', 'Foreground', True, False), ('Factor', 'Fac', False, False), ('Type', 'Type', False, False), ('Straight Alpha', 'Straight Alpha', False, False)]`
> `TWOIN internal_links: [('Background', 'Image')]`

The Mac's run with no window printed those same lines, character for character. **This test was also run on Linux** — entry 11 above — and the Linux run printed the same lines too, apart from the file's own path. The two machines agree.

**The Viewer node's image fills when a window is open, and stays empty when there is none (entry 16).** The same script, one render of 64 by 64, run twice. With no window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`

With a window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; the windowed case has no Linux counterpart, because that machine has no display server.

**Every property name on a socket is the same with a window as without (entry 17).** The same list came back from both runs:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree, and none of the names is an inspection, a resolution, a transform, a precision or a sampling. This bears on the *Watching rather than reading* section below, which records an unsupported note about "Socket Inspection": whatever that feature is, nothing behind it is reachable from a script in 5.1, with a window or without one. What the panel draws when a socket is hovered is still untested, because a script cannot hover a socket.

Both runs also printed the same four screen areas:

> `AREA PROPERTIES`
> `AREA OUTLINER`
> `AREA DOPESHEET_EDITOR`
> `AREA VIEW_3D`

and the run with no window printed, in addition:

> `SCREEN PRESENT: True`

So on macOS a Blender started with `-b` still has a screen with areas on it. That printout therefore does not tell a windowed run from a windowless one; what tells them apart is entry 16, where the Viewer image filled only in the windowed run.

**A picture rendered with a window is the same picture as one rendered with none (entry 18).** The four renders of entry 4 were made twice, once with no window and once with a window, then loaded back and compared pixel by pixel — pictures against pictures, never files against files:

> `PAIR ao_0.0     vs win_ao_0.0      sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`
> `PAIR ao_1.0     vs win_ao_1.0      sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`
> `PAIR sw_False   vs win_sw_False    sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`
> `PAIR sw_True    vs win_sw_True     sizes (64, 64) (64, 64)  MAXDIFF no-window against window = 0.0`

Both runs also printed the same four lines about what each render held:

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

Those are the same four lines the Linux machine printed with no window (entry 4). The two machines agree.

**One thing about ending a Blender that has a window.** The scripts were written to close Blender with `bpy.ops.wm.quit_blender()`. On this Mac that line on its own did not end it: the script's printing all appeared, and the process was still running when a limit of my own ended it. The same script with `sys.exit(0)` added after that line ended in about a second and printed the same thing. Both are recorded in entry 19 of the hand-test list.

### Not covered

- Nothing here settles what happens *inside* a node group. Group Input and Group Output nodes exist in the compositor's node list, and node groups can be nested, so a wire followed forward can leave the tree you are looking at. I did not research how a link crosses that boundary.
- Nothing here settles the Reroute node, or any other node not named above. Question 5 on the sheet covers pass-along and splitting nodes and I was told not to answer it.
- ~~I did not establish what a colour-changing node does to a picture's *domain*.~~ **Partly settled by entry 9 above**, which shows a two-picture node's output area following one named input; the section itself says so. What is still open is the colour-node case on its own.
- For the Switch family, I established what decides the choice but not how to read that decision out of a file — whether the value is a fixed socket value, a keyframe, a driver, or a link from elsewhere changes the difficulty enormously, and the sources do not walk through it. (Entry 6 above settles that a link is accepted and that the choice still does not vary across the frame.)
- ~~I did not establish whether `internal_links` is populated for compositor nodes specifically.~~ **Settled by entry 5 above:** it is, and it names one picture input on every compositor node tested, muted or not.

*[Two bullets struck through and one qualified by the citation check, 2026-08-06: all three were answered by tests printed earlier in this same document and were left standing. Repeats on 2026-08-07 came back the same.]*
- Whether any of this could be determined by *watching* rather than reading — node previews, the Viewer node — is only partly covered, below.
- A 5.1 change I read, in these words — "The **Mix** color mode of the **Mix** node now mixes the alpha as well" — carries its address now: Blender 5.1 release notes, Compositor, https://developer.blender.org/docs/release_notes/5.1/compositor/ — read 2026-07-30. I did not chase what it means for telling inputs apart, because the sources do not connect the two. *[Changed by the citation check, 2026-08-06: this bullet asked for the address to be found. The words are in the 5.1 compositor release notes and the address is now given.]*

### Watching rather than reading — partial

The sources describe two ways a person can look at what a picture has become at a given point in the tree:

> "The Viewer node is used to preview image data or value maps anywhere within a compositor node graph. It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."

— Blender manual, Viewer Node, https://docs.blender.org/manual/en/5.1/compositing/types/output/viewer.html — read 2026-07-30

> "Previews are an overlay that shows a small image above the node displaying the node result. Not all nodes support previews, but the ones that do can be toggled using the / icons in the top right-hand corner of the node next to the title."

— Blender manual, Node Parts, address above — read 2026-07-30

Both show what a picture *looks like* at a point. Neither is described as saying where that picture came from.

*[Rewritten by the citation check, 2026-08-06. The paragraph here recorded, without a quote or an address, that "Socket Inspection" appears in the sources only under Geometry Nodes and not for the compositor, and asked to be checked. Checked: there is no socket-inspection passage anywhere in the 5.1 manual's compositing pages. The feature is named in the compositor from 5.2 — "Socket inspection displays information about the image contained in a socket after the node tree has been evaluated." (Blender 5.2 manual, Compositor System, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30) — which is a different version's page and settles nothing for 5.1. Nothing turns on the original remark, and it should not be leant on for this version.]*

---

### Outside the interface

Does this hold when Blender is driven from outside its own window?

**LABEL: observed for both halves, on Linux and on macOS.**

**The wiring holds with no window, and reads the same with one.** On the rented Linux machine named above, on 2026-08-06, Blender 5.1.2 with `-b` opened a saved file and printed the whole chain — picture node, colour node, two-picture node, output node — with every link's node and socket at both ends, the Switch node's stored choice, and each node's mute state and internal links. That is entry 11 above, and it replaces the inference below for this half. On the Creator's Mac on 2026-08-07 the same file was read both with no window and with the window open, and the two printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round: the wiring reads the same whether Blender has its window open or not.

**The Viewer does not hold with no window, and does hold with one.** The image called `Viewer Node` exists as soon as a Viewer node is added, but stayed at 256 by 256 with every value zero across a 64 by 64 render with no window — on Linux (entry 10) and again on the Mac. With a window open on the Mac, that same image came back at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16). So the watching route is reachable when there is a window and not when there is none, which is what this section's reading of the sources pointed at.

**What a script cannot reach either way.** Every property name on a socket was the same with a window as without, on both machines, and none of them names an inspection, a resolution, a transform, a precision or a sampling (entry 17). So nothing behind what the *Watching rather than reading* section calls Socket Inspection is readable from a script, window or no window.

**The compositor can be put on the graphics card with no window at all.** On the same machine with its card repaired — driver 580.173.02 — on 2026-08-07, a Blender started with `-b` and its compositing device set to the card really did the work there: the card named that Blender's own process and it held 150 MiB while it worked, where the same run set to the processor never appeared on the card. So choosing the device is not an interface feature, and neither is acting on the choice (entry 12).

The reading from the sources, unchanged, follows.

**Split answer.**

- The node tree, its links, node names, mute states and switch settings — the whole of 8a, 8c and the readable part of 8d — are file data described by the Python API, and neither the `NodeLink` page nor the `Node` page mentions any interface requirement. The manual documents running Blender with no window and running a script: "-b, --background / Run in background (often used for UI-less rendering)." and "-P, --python <filepath> / Run the given Python script file." (Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30). **Inferred**, with the step written out: the sources do not say in so many words that a compositor node tree can be read this way; I am joining "the API describes links and nodes as ordinary readable data" to "Blender can run a script with no window", and a reader has to accept that join. It is not documented.
- The watching methods are the other way round. The Viewer node is described as feeding "the backdrop or Image Editor" (Blender manual, Viewer Node, address above — read 2026-07-30; these words are not among those quoted in full earlier, so the citation check should confirm them), and node previews as an overlay in the node editor — both parts of the interface. Whether either produces anything when Blender is driven with no window is **not in the sources**: none of the terms I searched for, listed below, was aimed at that, and the Command Line Arguments page names no node editor and no viewer.
- Whether the Switch family's per-frame choice behaves identically in a driven render is **not in the sources**. The pages describe the setting and its animation, not the render path.


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

Does this hold on every operating system Blender runs on?

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card half of entry 12 was run again on that same Linux machine on 2026-08-07, once its graphics card had been repaired and its driver stood at 580.173.02, and it settles Linux alone in exactly the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window (entry 11 on Linux, entry 15 on the Mac); the four Alpha Over and Switch renders (entry 4 on Linux, entry 18's no-window half on the Mac); the Viewer node's image staying empty with no window (entry 10 on Linux, entry 16's no-window half on the Mac); and every property name on a socket (entry 12 on Linux, entry 17 on the Mac). Nothing came out differently on the two operating systems.

**Ten of the fourteen Linux tests are still owed on the Mac**, and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding.

The reading from the sources, unchanged, follows.

**Not in the sources.** None of the pages quoted above names an operating system, and none states an exception. Here is the limit of that record, so nobody reads it as wider than it is: I read the pages listed at the end of this document and looked through the compositing section, but the names of the operating systems themselves are not among the terms I searched for across the whole of the manual, the API documentation and the release notes. The only operating-system-specific statement I found anywhere in the compositing section is unrelated to this question and concerns the GPU compositor's speed, not what nodes do:

> "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

— Blender manual, Limitations, https://docs.blender.org/manual/en/5.1/compositing/limitations.html — read 2026-07-30 (the page's heading for that passage is "TDR on Windows")

I will not turn silence into "yes, on all of them".

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
### Where I looked, and what I ruled out

**Searched across the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1`** for these terms, plain and industry both: `provenance`, `lineage`, `trace`, `traceab`, `fingerprint`, `watermark`, `tag the image`, `image id`, `identity of`, `identifi`, `distinguish`, `tell apart`, `which input`, `which image`, `came from`, `source image`, `upstream`, `downstream`, `unchanged`, `passed to the output`, `pass-through`, `passthrough`, `metadata`, `meta-data`.

What that turned up and what it ruled out:

- `provenance`, `lineage`, `traceab`, `fingerprint`, `tag the image` — **no hits anywhere**. This is the core of the "not in the sources" answer in 8b: the compositor is not documented as carrying any identity with an image.
- `metadata` / `meta-data` — one hit in the whole of `compositing/`, a Cryptomatte note that sidecar files are not supported. The 5.1 release note about the Image and File Output nodes reading and writing custom metadata is quoted above; I checked the API pages `bpy.types.CompositorNodeImage`, `bpy.types.CompositorNodeOutputFile` and `bpy.types.CompositorNodeImageInfo` for any metadata property and found none. Ruled out as a way of following a picture *between* nodes.
- `watermark` — hits on the Mix pages of four different node systems. The compositing one is quoted above. Ruled in as a hand-built technique, ruled out as an automatic property.
- `which input`, `which image`, `passed to the output`, `unchanged` — these led to the Switch, Index Switch, Menu Switch and Depth Combine pages quoted above. That is the whole of the "yes" side of 8d.
- `distinguish`, `identifi` — nothing about images; hits were about node colours, node names, and bundle socket names.

**Pages opened in full:** the compositor's Compositor System, Introduction and Limitations pages; the Color Nodes index and Adjust index; RGB Curves, Hue/Saturation/Value, Color Balance, Brightness/Contrast, Mix Color, Alpha Over, Depth Combine; Switch, Index Switch, Menu Switch, Split; Viewer; Node Parts and Node Editors under the interface's node reference; Command Line Arguments. API pages `NodeLink`, `Node`, `CompositorNodeCurveRGB`, `CompositorNodeAlphaOver`, `CompositorNodeSwitch`. The full 5.1 compositor release note.

**Pictures opened:** `_images/node-types_CompositorNodeCurveRGB.webp`, `_images/node-types_CompositorNodeAlphaOver.webp`, `_images/node-types_CompositorNodeSwitch.webp`.

**Ruled out without a full read, and why:**

- **Cycles' own files (`cycles_5.1`).** Grepped the whole folder for `composit`. Five hits: two in `ui.py` and `properties.py` that are panel-registration lines and a glass-compositing description, and three in kernel headers unrelated to node trees. Cycles is the render engine, not the compositor; it holds nothing about following a picture through nodes.
- **The requirements page.** It names 4.5 LTS and "Blender 5.0 and later", not 5.1 in so many words, so by the rules of this pass it cannot speak for this version. It is about hardware and operating systems in any case.
- **The keying, mask, filter, transform, texture and tracking node families.** None of them is "a node that changes its colours" or "a node that takes two pictures in and gives one out" as the question describes, and the file list shows no candidate I would have expected to change the answer. I did not read them page by page.
- **The other node systems' Mix and Switch pages** (shader nodes, geometry nodes, texture nodes). They appeared in searches. They describe different node trees and were not used.

---

## Things only a running Blender can settle

**All seven of these have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above. Two more were run on the Creator's Mac on 2026-08-07 and are in *What a running Blender showed on the Creator's Mac*: the graph read the same with Blender's window open as with none, and the Viewer node's image filled only when there was a window. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **What the interface *draws* about a socket after a colour change** — the hovering half of test 5. A script cannot hover a socket. What is settled is that nothing behind it is readable from a script, with a window or without.
- **Ten of the fourteen tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run, and every answer here leaves Windows open.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Does a muted Alpha Over pass a specific one of its two inputs through?**
   Open the Compositor on a scene with a compositing node tree. Add two Image nodes loading two visibly different pictures. Wire the first into the Alpha Over node's upper image socket and the second into the lower. Add a Viewer node on the Alpha Over output. Select the Alpha Over node and mute it, using whatever the node editor itself offers for muting — I quote no shortcut key from the sources, so do not take one on trust from me. Look at the backdrop or the Image Editor.
   *Negative result:* the backdrop shows a blend of the two, or black, or is unchanged from the unmuted state — i.e. muting does not pass one input through whole. A positive result is one of the two pictures showing intact, and you note which socket it came from.

2. **Does `internal_links` report that choice, and is it populated for compositor nodes?**
   With the muted Alpha Over from test 1 selected, open the Python Console and print, for that node, each entry in `internal_links` with the names of its from-socket and to-socket.
   *Negative result:* the collection is empty, or it exists only while the node is unmuted, or the socket it names is not the one test 1 showed coming through. Either of those kills the inference in 8d.

3. **Do the same two tests survive with no window.**
   Save that file. Run Blender from a terminal with `-b`, the file, and `-P` pointing at a short script that prints, for every link in the scene's compositing node tree, the from-node name, from-socket name, to-node name, to-socket name, and each node's `mute` and `internal_links`.
   *Negative result:* the script errors, the node tree is absent or empty, or the printed links differ from what the interface showed. That would mean the reading in 8a does not hold outside the interface — which is the answer that matters most here.

4. **Does a Switch node pass one input through byte-for-byte, or does it alter it?**
   Two parts. First: wire one picture into a Switch node's `Off` input and a different one into `On`, put a Viewer on the output, toggle the Switch and watch. Second, to test "unchanged" rather than merely "looks the same": feed one single picture into the Switch, take the Switch's output into one side of a Mix node set to Difference, take the same picture straight into the other side, and look at the Difference result. **Read the actual pixel values of that result rather than judging it by eye** — a small difference looks black, so an eye check would pass a Switch that had altered the picture slightly. If the only reading available is by eye, say so in the result rather than reporting a clean pass.
   *Negative result:* the Difference result is not all zero — meaning the Switch did not pass the picture through untouched. This is the test that separates "chose it" from "chose it and left it alone".

5. **Does anything at all identify a picture after a colour change?**
   Load one picture, run it through an RGB Curves node with a strong curve, and put a Viewer after it. Then check every place the interface could report on that socket: the node's preview overlay, the Viewer in the Image Editor, the sidebar with the RGB Curves node active, and hovering the output socket.
   *Negative result — and this is the one I expect:* none of those shows any name, source, or identifier for the picture, only pixel values. A positive result would be any readout naming where the picture came from, which would contradict 8b and would be a genuinely important find.

6. **Does Depth Combine's per-pixel choice show up anywhere readable?**
   First write down every output socket the Depth Combine node actually has — the only one I quote from the sources is `Result`, so do not assume any other exists. Then feed two pictures with two different flat depth values into the node, put a Viewer on `Result`, and swap which depth is smaller. If the node turns out to have a second output as well, view that too and say what it showed.
   *Negative result:* the `Result` does not change when the depths are swapped. Also worth recording as its own answer: the node has no output beyond `Result`, in which case the choice is not readable anywhere but in the picture itself.

7. **Does the Viewer node produce anything when Blender runs with no window?**
   Take the file from test 1, with a Viewer node on the compositor output. Run Blender from a terminal with `-b`, that file, and `-P` pointing at a short script that renders the scene and then prints, for every image Blender is holding, its name and its size in pixels. Record the whole of what the terminal printed, word for word.
   *Negative result:* no image corresponding to the Viewer appears, or it appears with no pixels, or the script errors. A positive result is an image from the Viewer with real pixel dimensions. This settles the second bullet of "Outside the interface" above, which the sources leave open.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 5.1

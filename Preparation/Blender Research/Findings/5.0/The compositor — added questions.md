# The compositor — added questions (Blender 5.0)

**Sources consulted, and when.** Blender's manual for 5.0, Blender's Python API documentation for 5.0, and Blender's 5.0 release notes — all read on 2026-07-30. Three pictures from the manual's `_images` folder, also read on 2026-07-30 (they are named at the end of this document). Blender's requirements page as taken on 2026-08-01 (looked at only to see whether it names 5.0; it does, and it says nothing bearing on this question). Cycles' own files were not needed for this question and were not opened.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 8 was asked for. Only question 8 is answered here.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

### The question is ambiguous, and here is how I read it

Two words carry more than one meaning here.

**"Followed forward"** can mean either (a) *tracing the wiring* — knowing where a picture goes next and what it reaches, or (b) *recognising the picture at the far end* — being able to say that what came out is the thing that went in. I answer both, because the sources speak to both and they come out differently.

**"A node that takes two pictures in and gives one out"** can mean either (i) a node that *combines* two pictures into one — Alpha Over, Mix Color, Depth Combine, Split — or (ii) a node that *chooses* one of two and hands it on — the Switch node. I take (i) as the reading meant, because that is the ordinary shape of a compositing node with two picture inputs, and I answer it first and in full. Reading (ii) is answered afterwards, briefly, because the sources answer it outright and the difference between the two readings is large.

### Short answer

**Following the wiring: yes, and it is documented.** Every node named here has one combined-picture output — Depth Combine also gives a second, Depth output (see 8.2) — and the links between nodes are readable one by one, each carrying which node and which socket it comes from and goes to. So the wiring onward from a picture, through a colour node and on through a two-input node, can be walked. That is the path the picture takes, which is not the same as saying the picture itself comes out the far side; the next paragraph is about that.

**Recognising the picture at the far end: partly.** A colour node has one picture in and one picture out, so what comes out is that picture with its colours changed. A combining node is documented as producing a *combined* picture, not one of its two inputs; the sources describe no mark, label, tag or other identifier that travels with a picture and would say where it came from. But there is one thing that *is* documented and does survive: the output's **domain** — its size and where it sits — is taken from one designated input, and the sources say which input that is for Alpha Over and for Mix. In that sense, and only that sense — the frame, not the colours — which of the two came out the far side is documented. The manual states the rule for "all other nodes" "unless stated otherwise in their respective documentation pages", so it holds generally only as far as the node pages actually read (see *Not covered*).

**Labels:** documented for the wiring, for the socket shapes, and for the domain rule. Inferred for what follows from those. Not in the sources for any identifier travelling with the pixels.

---

### 8.1 Through a node that changes its colours

**Label: documented for the socket shapes; inferred for the frame.**

Colour-changing nodes take one picture and give back one picture.

> "Inputs
>
> Image/Color
>
> Standard color input.
> …
> Outputs
>
> Image/Color
>
> Standard color output."

Source: Blender manual for 5.0, Hue/Saturation/Value Node — https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/hue_saturation.html — read 2026-07-30.

The same shape is documented for the other colour nodes I checked. RGB Curves: "Image/Color — Standard color input." (https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/rgb_curves.html). Color Balance: "The Color Balance node adjusts the color and values of an image." and "Image — Standard color input." (https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/color_balance.html). Both read 2026-07-30.

The picture's **frame** is documented to come through a one-picture node unchanged in which input decides it:

> "Unless stated otherwise in their respective documentation pages, all other nodes use the following mechanism. One of the inputs of the nodes is designated as the Domain Input of the node, and the operation domain of the node is identical to the domain of that designated input. For many nodes, the domain input can be intuitively identified as the main input of the node, for instance, the domain input for the Filter node is the Image input."

Source: Blender manual for 5.0, Compositor System — https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30.

**Inferred, with the step written out:** for a colour node with one picture input and one picture output, that rule makes the picture input the domain input, so the output sits in the same place and at the same size as the input. The step a reader has to supply is that the colour node is one of the "all other nodes" the paragraph covers and that its picture input is its "main input"; the manual states the rule generally and gives the Filter node as its example, not the colour nodes.

### 8.2 Through a node that takes two pictures in and gives one out

**Label: documented for the socket shapes and the domain rule; inferred for what Alpha Over gives at Factor 0; not in the sources for any identifier carried by the pixels.**

The combining nodes each have two picture inputs and one picture output.

Alpha Over:

> "The Alpha Over node composites two images by layering the Foreground image over the Background, using standard alpha blending operations."

and

> "Outputs
>
> Image
>
> The combined image after alpha compositing."

Source: Blender manual for 5.0, Alpha Over Node — https://docs.blender.org/manual/en/5.0/compositing/types/color/alpha_over.html — read 2026-07-30.

The 5.0 release notes state the input names:

> "The **Alpha Over** node input names were renamed to Background and Foreground."

Source: Blender 5.0 release notes, Compositor — https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30.

Mix Color:

> "A/B
>
> The two inputs that are mixed together."

and

> "Outputs
>
> Result
>
> Output the result of the mix using the data type selected."

Source: Blender manual for 5.0, Mix Color Node — https://docs.blender.org/manual/en/5.0/compositing/types/color/mix_color.html — read 2026-07-30.

Split:

> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line."

Source: Blender manual for 5.0, Split Node — https://docs.blender.org/manual/en/5.0/compositing/types/utilities/split.html — read 2026-07-30.

So on the wiring, the answer is plain: one output socket, and the link out of it can be followed (see 8.3). On the *content*, the manual's own words for what emerges are "The combined image after alpha compositing", "the result of the mix", and, for Split, "a single output image" made by combining two — a new picture made from two, not one of the two.

**Which of the two came out — what the sources do settle.**

*Documented, for the frame rather than the colours.* The output's domain — its size and where it sits — comes from one designated input, and the manual names which:

> "Each input in the node has the so called Domain Priority property, the operation domain of the node is the domain of the non single value input with the highest domain priority."

> "The output of nodes will be intuitively clipped to the operation domain, or rather, the domain of the domain input. For instance, if the Foreground input is bigger than the Background input in the Alpha Over node, the output will be clipped to the Background input because it is the domain input"

> "The Alpha Over node currently does not support changing the domain priority for its inputs, so as a workaround, one can use a Mix node to achieved the desired behavior, noting that the first Image input in the Mix node has the highest domain priority"

Source: Blender manual for 5.0, Compositor System — https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read 2026-07-30. (The spelling "achieved" is the manual's.)

That page also says what happens to the *other* input:

> "The nodes only consider the area of the input images that overlap the operation domain and ignores the rest of the images. If an input image doesn't completely overlap the operation domain, the rest of the operation domain for that input will be assumed to be a zero value, a zero vector, or a transparent zero color depending on the type."

Same page, same date.

**Documented — a picture in the manual showing this.** The page carries a screenshot captioned "The Foreground input is bigger than the Background input in the Alpha Over node, so the output is intuitively clipped to the Background input because it is the domain input." I opened it: `_images/compositing_realtime-compositor_compositing-space_operation-domain_considerations_clipping.png`, on the Compositor System page above, read 2026-07-30. It shows two Image nodes, each through a Scale node, into an Alpha Over node, into a Composite node, with the result in the editor backdrop. **A caution about that picture:** its status bar reads "3.5.0 Alpha", it shows a "Composite" node and a "Use Nodes" checkbox, and the 5.0 release notes say the Composite node was removed and "Use Nodes" in the node editor was removed. I therefore take from it only what the caption claims, and I do not take its socket names or node set as showing 5.0. The same caution applies to the Alpha Over node picture, `_images/node-types_CompositorNodeAlphaOver.webp` on the Alpha Over page: I opened it and it shows sockets labelled "Fac", "Image", "Image", "Straight Alpha" — the older names, not the "Background" and "Foreground" the 5.0 text and release note give.

*Documented, per pixel, for one specific node.* Depth Combine states outright which input's pixel wins:

> "Result
>
> If both depth values are equal, it will use the foreground image. Whichever depth value is less decides which image pixel is used."

Source: Blender manual for 5.0, Depth Combine Node — https://docs.blender.org/manual/en/5.0/compositing/types/color/depth_combine.html — read 2026-07-30. (The 5.0 release notes record the rename: "The Z Combine node was renamed to Depth Combine".) That node also gives a second output, "Depth — The combined depth, which allows to thread multiple Depth Combine nodes together", same page.

*Documented, by position, for Split.* Which side each input lands on is stated in relative terms only:

> "Image
>
> The first image input, displayed on one side of the split.
>
> Image
>
> The second image input, displayed on the opposite side of the split."

Source: Split Node page above, read 2026-07-30. The manual does not say which physical side the first input takes. *[Changed by the citation check, 2026-08-06: the second quoted line was cut short at "opposite side"; the page reads "opposite side of the split". Restored.]*

*Inferred, for Alpha Over at the extreme of its Factor.* The manual says:

> "Factor
>
> Controls the opacity of the foreground image, from 0 (fully transparent) to 1 (fully opaque)."

Source: Alpha Over Node page above, read 2026-07-30. **The step:** if at Factor 0 the foreground is fully transparent, then nothing of the foreground shows and what comes out is the background. The manual does not say "the output equals the background", so this is a step the reader supplies, and it is inferred, not documented. I found no equivalent statement for the Mix Color node — its page says only "Controls the amount of mixing between the A and B inputs", and never says what a Factor of 0 or 1 produces.

**Which of the two came out — what the sources do not settle. Label: not in the sources.**

Nothing I found describes any name, label, tag, identifier or piece of metadata that travels along with a picture through the compositor and would say which input it came from. The manual's own account of what compositor data *is* lists only pixel kinds and a domain:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats: Float … Vector … Color"

Source: Compositor System page above, read 2026-07-30. There is no fourth thing in that list that could carry an origin.

The one node that reports *about* a picture reports geometry only:

> "The Image Info node outputs spatial and transformation information about an image in the compositor."

with outputs "Dimensions", "Resolution", "Location", "Rotation", "Scale". Source: Blender manual for 5.0, Image Info Node — https://docs.blender.org/manual/en/5.0/compositing/types/input/image_info.html — read 2026-07-30. Nothing there names a source.

### 8.3 Following the wiring forward — the mechanism

**Label: documented for the link and socket properties; not in the sources for which input a muted two-input node passes through, and not in the sources for any domain priority readable from a script.**

Links are individually readable and each says where it comes from and where it goes:

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

and further down the same page:

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

Source: Blender Python API documentation for 5.0, NodeLink — https://docs.blender.org/api/5.0/bpy.types.NodeLink.html — read 2026-07-30.

The whole set of links belongs to the tree — "links / Type: NodeLinks bpy_prop_collection of NodeLink, (readonly)" (https://docs.blender.org/api/5.0/bpy.types.NodeTree.html) — and each socket can list its own — "links / List of node links from or to this socket." and "is_linked / True if the socket is connected" (https://docs.blender.org/api/5.0/bpy.types.NodeSocket.html). Both read 2026-07-30.

Two details on the same pages that bear on following a picture. A link can be switched off — "is_muted / Link is muted and can be ignored" (NodeLink page). And a whole node can be switched off, which turns it into a pass-through:

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Source: Blender manual for 5.0, Editing (Nodes) — https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html — read 2026-07-30. Which of a two-input node's inputs passes through when it is muted is described in the API only as a bare fact of the node — "internal_links / Internal input-to-output connections for muting" (https://docs.blender.org/api/5.0/bpy.types.Node.html, read 2026-07-30) — and the sources never say, for Alpha Over or Mix, which input that is. **Label for that last point: not in the sources.**

I searched the whole of the 5.0 Python API documentation for "domain priority" and "domain_priority" and found nothing. So the priority the manual describes, which is the one documented thing that says which input came out, does not appear as a readable property in the API documentation. **Label: not in the sources.**

### 8.4 Following the picture forward by looking at it

**Label: documented for the preview, the Viewer node and the image data-block; inferred for using them to attribute an output to an input.**

Two documented ways to see the picture at a point in the graph.

> "Shows/Hides a preview region on the node that displays the frame after that node's operation has been applied."
> …
> "This operator are only available in the Compositor."

Source: Editing (Nodes) page above, read 2026-07-30. (The grammar is the manual's.)

> "The Viewer node is used to preview image data or value maps anywhere within a compositor node graph. It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."

Source: Blender manual for 5.0, Viewer Node — https://docs.blender.org/manual/en/5.0/compositing/types/output/viewer.html — read 2026-07-30. The same page says where that result appears: "Only the active Viewer node … will be displayed in the backdrop or Image Editor", and "Click and holding the mouse in the Image displayed allows you to sample the values."

The viewer's result also exists as an image data-block. The API's list of where an image comes from includes:

> "VIEWER
> Viewer – Compositing node viewer."

and an image has

> "pixels
>
> Image buffer pixels in floating-point values"

Source: Blender Python API documentation for 5.0, Image — https://docs.blender.org/api/5.0/bpy.types.Image.html — read 2026-07-30.

**Inferred, with the step written out:** put a Viewer node after the combining node, read the numbers, and compare them with the numbers from Viewer nodes on each input; that would tell you what the output is made of. The sources document each piece — the Viewer previews any point in the graph, its result is an image, an image's pixels are readable — but nowhere do they say this comparison is a supported way to attribute an output to an input. The reader supplies the joining-up.

The manual does document one worked case of telling a picture apart after it has been through a combining node, though for a mark deliberately put into it rather than for the picture itself:

> "Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

and, for the decoding:

> "use the node tree below to compare it to your stock image (non-watermarked original). In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference. You can see how the original mark clearly stands out."

Source: Mix Color Node page above, read 2026-07-30. Note what this shows and does not show: it tells apart by comparing pixels against a kept copy of the original, not by anything the compositor carries along with the picture.

### 8.5 The other reading — a node that chooses one of two

**Label: documented.**

If "a node that takes two pictures in and gives one out" means the Switch node, the sources answer plainly and the answer is yes:

> "Switch between two images using a checkbox."
>
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

Source: Blender manual for 5.0, Switch Node — https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch.html — read 2026-07-30. I also opened that page's picture, `_images/node-types_CompositorNodeSwitch.webp`, on 2026-07-30: it shows a node headed "Switch" with a single output labelled "Image" and inputs "Switch" (a checkbox), "Off" and "On". Label for the picture: documented, and it shows only that — one picture output and those three inputs.

The Menu Switch node goes further and exposes which input was taken, as a value inside the tree:

> "Outputs the value from the input corresponding to the selected menu item, unchanged."

> "For each menu entry, an additional Boolean output is created. The output corresponding to the selected item is true, and all others are false."

Source: Blender manual for 5.0, Menu Switch Node — https://docs.blender.org/manual/en/5.0/compositing/types/utilities/menu_switch.html — read 2026-07-30. The 5.0 release notes record it arriving here: "**Menu** sockets and **Switch Menu** node are now supported in the compositor." (https://developer.blender.org/docs/release_notes/5.0/compositor/, read 2026-07-30).

So under this reading: the picture is "passed to the output", and which of the two came out is stated by the node's own setting — for Menu Switch, readable as an output socket. Note the exact words: the Switch page says the chosen input is passed to the output, and does not say it is passed *unchanged*. "Unchanged" is the Menu Switch page's word, about that node. Whether the Switch node alters what it passes is not stated on its page, and is one of the things a person at a computer could settle below.

---

### What a running Blender showed

**LABEL: observed.** Everything in this section was got by running Blender 5.0.1 on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, and each Blender was pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics' tests while this ran, so nothing here reports how long anything took. The full procedure, every script in full, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only. The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**A control first (entry 1).** A picture put through a bare tree and written out came back identical to the picture that went in, and the same render twice gave the same picture:

> `MAXDIFF render vs source 0.0`
> `MAXDIFF render vs same render again 0.0`

So every comparison below is a comparison of pictures, not of files, and a difference of zero means the pixels match.

**Which nodes take two pictures in, and what their sockets are called (entry 2).**

> `ADDABLE NODE TYPE COUNT 115`
> `TWO-PICTURE-IN, ONE-PICTURE-OUT COUNT 24`
> `SWITCH-LIKE NAMES AMONG ADDABLE ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeMenuSwitch']`

So the Menu Switch this answer describes is not a compositor node type at all — it is the geometry-node one, added to a compositor tree. There is no Index Switch in this version.

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
> `ABSENT CompositorNodeMixRGB -> RuntimeError Error: Node type CompositorNodeMixRGB undefined`
> `ABSENT CompositorNodeComposite -> RuntimeError Error: Node type CompositorNodeComposite undefined`

The Split node's two identically named picture inputs carry the identifiers `Image` and `Image_001`. Depth Combine's picture inputs are named A and B, not Foreground and Background.

**What comes out of a Mix at the ends of its factor (entry 3).**

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

**Alpha Over at the ends of its factor, and the Switch node (entry 4).**

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

The foreground picture used was opaque throughout, which is why factor 1 gives it back whole.

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

So `internal_links` is filled in and names one picture input on every one of these nodes, and `is_inactive` stayed false on all of them, muted or not.

Sweeping every property name on a socket and on the Alpha Over node for anything naming a priority or a domain:

> `  ANY NAMING PRIORITY OR DOMAIN: []`

**The Switch node's choice fed by a link (entry 6).**

> `  value node CompositorNodeValue not available: Error: Node type CompositorNodeValue undefined`
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

**Which physical side the Split node's first input lands on, and Depth Combine pixel by pixel (entry 8).**

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

*[Note added 2026-08-07 by entry 20, below. This entry's conclusion for 5.0 is right, and entry 20 confirms it by reading the frame straight off the socket instead of inferring it from the alpha. Two cautions belong with it even so. The Factor was never written down here, and it matters: a Mix node starts at Factor 0.5 in this version and at 1.0 in 5.2, which is what made 5.2 look unlike the rest. And the area whose alpha is above zero is a reading of the alpha, not of the frame; the two happen to agree in this version at this Factor, and they do not always. Entry 20 is what to read.]*

**A Viewer node, a node group boundary, and a File Output name (entry 10).** With no window, and the render 64 by 64:

> `VIEWER IMAGE before the render: there is no image called 'Viewer Node'`
> `VIEWER IMAGE after the render: there is no image called 'Viewer Node'`
> `   image 'picA.exr' source=FILE size=(64, 64) pixel count=16384`
> `   image 'Render Result' source=VIEWER size=(0, 0) pixel count=0`

In this version no image called `Viewer Node` exists at all, before or after a render with no window.

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

The socket's name did not reach the written file's name. The node's file-name setting was left at whatever the version starts with; Blender's own description of it says only "The base name of the file. Other information might be included in the final file name depending on the node options", and what that other information is was not settled.

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
> `HEAVY RENDER SECONDS CPU 4.071`
> `HEAVY PROCESSOR CORE-SECONDS CPU 62.190`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 44`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`

Set to the card:

> `ASKED FOR DEVICE GPU`
> `compositor_device now reads: GPU`
> `HEAVY RENDER SECONDS GPU 0.679`
> `HEAVY PROCESSOR CORE-SECONDS GPU 0.940`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 20`
> `TIMES THIS PROCESS WAS ON THE CARD: 5`
> `MOST CARD MEMORY THIS PROCESS HELD: 214 MiB`

Only the setting differed between those two runs, and three things move together. Set to the processor, this Blender never appeared on the card at all. Set to the card, it appeared on it holding 214 MiB, and its own share of processor work fell from sixty-two core-seconds to under one.

**And the card's picture matches the processor's picture.** The two heavy renders, loaded back and compared pixel by pixel:

> `HEAVY CPU SIZE (2048, 2048) HEAVY GPU SIZE (2048, 2048)`
> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`
> `HEAVY MIDDLE PIXEL, processor: [0.500416, 0.500416, 0.25, 1.0]`
> `HEAVY MIDDLE PIXEL, card     : [0.500416, 0.500416, 0.25, 1.0]`

So the two devices give the same picture to about four parts in ten million on the heavy tree and about six parts in ten million on the small one. Not identical, and not far off: a picture cannot be told apart by its exact values across the two devices, but any test coarser than a millionth still holds. The seconds above are rough, because the machine was shared while this ran; the count of times this process was on the card is not, because it names one process by its own number.

Every property name on a colour node's output socket, after a render:

> `ANY NAMING INSPECTION, RESOLUTION, TRANSFORM, PRECISION, SAMPLING, VALUE OR IMAGE: ['default_value', 'hide_value']`
> `default_value on the output socket: <bpy_float[4], NodeSocketColor.default_value>`

`default_value` on a colour socket holds four numbers, which is a socket's own fixed colour rather than the picture that travelled through it.

**Does a faint mark survive a colour change (entry 13).**

> `BEFORE any colour change: biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `AFTER the colour change  : biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `MARKED-AND-GRADED against the UNMARKED ORIGINAL: inside the mark = 0.607381, outside it = 1.000000`

**Is there any metadata on the compositor's own nodes (entry 14).**

> `CompositorNodeImage: properties naming metadata -> none`
> `   its input socket names: []`
> `   its output socket names: [('Image', 'Image', 'RGBA'), ('Alpha', 'Alpha', 'VALUE')]`
> `CompositorNodeOutputFile: properties naming metadata -> none`
> `CompositorNodeImageInfo: properties naming metadata -> none`
> `Image data-block properties naming metadata -> none`

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

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-5.0`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 15 to 18.

**The graph reads the same with a window open as with none (entry 15).** One Blender built and saved the tree with no window; a second opened that file with no window and printed it; a third opened the same file with the window open and printed it. The run with a window, word for word:

> `VERSION 5.0.1`
> `FILE /Users/artist/Documents/Smoozy hand tests/The compositor/graph_5_0.blend`
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

**A Viewer image appears in this version only when there is a window (entry 16).** The same script, one render of 64 by 64, run twice. With no window, before and after the render:

> `VIEWER IMAGE before the render: there is no image called 'Viewer Node'`
> `VIEWER IMAGE after the render: there is no image called 'Viewer Node'`

With a window:

> `VIEWER IMAGE before the render: there is no image called 'Viewer Node'`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

So in 5.0 the image does not exist at all until a render happens with a window open, and then it appears at the size of the render with values above zero. The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; the windowed case has no Linux counterpart, because that machine has no display server.

**Every property name on a socket is the same with a window as without (entry 17).** The same list came back from both runs:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree, and none of the names is an inspection, a resolution, a transform, a precision or a sampling — and none names a priority or a domain either.

Both runs also printed the same four screen areas:

> `AREA PROPERTIES`
> `AREA OUTLINER`
> `AREA DOPESHEET_EDITOR`
> `AREA VIEW_3D`

and the run with no window printed, in addition:

> `SCREEN PRESENT: True`

So on macOS a Blender started with `-b` still has a screen with areas on it. That printout therefore does not tell a windowed run from a windowless one; what tells them apart is entry 16, where the Viewer image appeared only in the windowed run.

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

- Whether any identifier survives a combining node **in fact**. My answer is that the sources describe none. That is not the same as the software carrying none.
- What a Mix Color node produces at Factor 0 or Factor 1. The compositor's Mix Color page never says. I did not go to the shader-node Mix page for this, because that describes a different editor's node, and the compositor page's own picture is even filed under a shader-node name (`render_shader-nodes_shader_mix_node.jpg`), which makes borrowing from that page unsafe.
- ~~Which physical side the Split node's first input appears on.~~ **Settled by entry 8 above:** in 5.0 the first input lands on the left of the frame and the second on the right.
- ~~Which input a muted Alpha Over or Mix node passes through.~~ **Settled by entries 5 and 7 above:** the Alpha Over passes its Background, the Mix its A colour input, and the render agrees with what `internal_links` says.

*[Two bullets struck through by the citation check, 2026-08-06: both were answered by tests printed earlier in this same document and were left standing. A repeat of both tests on 2026-08-07 came back the same.]*
- Whether the domain rule holds for every two-input node. The manual states it for "all other nodes … unless stated otherwise in their respective documentation pages", and names Alpha Over and Mix; I did not check every node page for an "otherwise".
- Ordering when several links feed one socket. The API mentions "multi_input_sort_id — Used to sort multiple links coming into the same input. The highest ID is at the top." (NodeLink page, read 2026-07-30). I did not pursue what that means for following a picture, because the question asks about two pictures on two sockets.
- Anything about how this behaves during an actual render, as opposed to what the pages describe. No page I read describes the running of a composite step by step.

### Outside the interface

**LABEL: observed for the wiring half and for the Viewer, on Linux and on macOS.**

**The wiring holds with no window, and reads the same with one.** On the rented Linux machine named above, on 2026-08-06, Blender 5.0.1 with `-b` opened a saved file and printed the whole chain — picture node, colour node, two-picture node, output node — with every link's node and socket at both ends, the Switch node's stored choice, and each node's internal links. That is entry 11 above, and it replaces the inference below for this half. On the Creator's Mac on 2026-08-07 the same file was read both with no window and with the window open, and the two printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round: the wiring reads the same whether Blender has its window open or not.

**The Viewer does not hold with no window, and does hold with one.** With no window, no image called `Viewer Node` exists in this version at all, before or after a render — on Linux (entry 10) and again on the Mac. With a window open on the Mac, that image appeared after the render at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16). So the looking-at-it route is reachable when there is a window and not when there is none.

**What a script cannot reach either way.** Every property name on a socket was the same with a window as without, on both machines, and none of them names an inspection, a resolution, a transform, a precision, a sampling, a priority or a domain (entry 17). So neither a socket's own picture nor a domain priority is readable from a script, window or no window.

**The compositor can be put on the graphics card with no window at all.** On the same machine with its card repaired — driver 580.173.02 — on 2026-08-07, a Blender started with `-b` and its compositing device set to the card really did the work there: the card named that Blender's own process and it held 214 MiB while it worked, where the same run set to the processor never appeared on the card. So choosing the device is not an interface feature, and neither is acting on the choice (entry 12).

The reading from the sources, unchanged, follows.

Mixed, and partly unanswered.

- **The wiring** (`NodeLink`, `NodeTree.links`, `NodeSocket.links`) is Python API. The API documentation states that Blender can be run without its interface and scripts executed that way: "Use Blender without it's User Interface … blender --background --python myscript.py" (https://docs.blender.org/api/5.0/info_tips_and_tricks.html, read 2026-07-30; the apostrophe is theirs). The command-line page confirms "-b, --background — Run in background (often used for UI-less rendering)." (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30). **Inferred, step written out:** since these are ordinary data properties of the node tree and scripts run in background mode, reading them should not need the interface. The sources nowhere say so about these particular properties.
- **The domain rule** is a statement about how nodes compute, not about the interface, and the page describing it makes no mention of the interface. The sources do not say whether it differs either way. **Label: not in the sources.**
- **Node Preview** is documented as an interface operator under "Node ‣ Show/Hide ‣ Node Preview" with a keyboard shortcut, and **the Viewer node's** result is documented as appearing "in the backdrop or Image Editor". Both of those are parts of Blender's window. The sources do not say what either does when there is no window. Label: not in the sources.
- The Compositor System page does distinguish "viewport compositing" from "final render compositing" when giving output sizes, which shows the compositor runs in more than one setting, but it says nothing about running without an interface.


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card half of entry 12 was run again on that same Linux machine on 2026-08-07, once its graphics card had been repaired and its driver stood at 580.173.02, and it settles Linux alone in exactly the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window (entry 11 on Linux, entry 15 on the Mac); the four Alpha Over and Switch renders (entry 4 on Linux, entry 18's no-window half on the Mac); no image called `Viewer Node` existing at all with no window (entry 10 on Linux, entry 16's no-window half on the Mac); and every property name on a socket (entry 12 on Linux, entry 17 on the Mac). Nothing came out differently on the two operating systems.

**Ten of the fourteen Linux tests are still owed on the Mac**, and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding.

The reading from the sources, unchanged, follows.

The sources do not say that any of this differs by operating system. None of the pages quoted above names one.

The only operating-system-specific statement I found anywhere in the compositing section is about the graphics driver, not about following pictures: "TDR on Windows — Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." (https://docs.blender.org/manual/en/5.0/compositing/limitations.html, read 2026-07-30). That concerns a composite being interrupted, not what a node does with its inputs.

The requirements page names this version once — "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (https://www.blender.org/download/requirements/, read 2026-08-01) — which is about where 5.0 runs at all, not about this question.

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
### Where I looked

**Searched, across the whole of the 5.0 manual unless said otherwise:**

- In `compositing/`: `metadata` (one hit only, on the Cryptomatte page — about object identity within a render, not about telling two pictures apart, so ruled out); `provenance` (nothing); `identity` (hits are the compositing-space "identity transformation" and "identity values", not picture identity); `trace` (one hit, "ray-traced", irrelevant); `tag` (hits are "footage", "montage" and similar, no relevant sense); `which input` (one hit, Menu Switch); `unchanged` (Menu Switch, Box Mask, Ellipse Mask); `pass through` / `passthrough` (nothing in `compositing/`); `passed to the output` (Switch node only); `watermark` (Mix Color page).
- Across the whole manual: `metadata` — the substantial page is `render/output/properties/metadata`, which is about stamping information onto written frames, not about node-to-node flow; ruled out as outside the question. `Color Blend Modes` — the glossary entry says only "Methods for blending two colors together" and points at documentation outside these sources, so it settles nothing about which input dominates.
- Across the whole manual: `background mode`, `--background`, `headless` — hits only in `advanced/command_line/arguments.txt`, none about the compositor or the viewer.
- Across `compositing/`: `windows`, `macos`, `linux` — one hit, the TDR note quoted above.
- Across the whole 5.0 Python API documentation: `domain priority`, `domain_priority` — no hits at all; `operation domain` — no hits.

**Pages opened in full:**

- Manual: Compositing Introduction; Compositor System; Compositing Limitations; Mix Color Node; Alpha Over Node; Depth Combine Node; Set Alpha Node (opened to check whether it carried an origin marker — it does not, it only sets alpha); Hue/Saturation/Value Node; Color Balance Node; RGB Curves Node; Switch Node; Split Node; Menu Switch Node; Viewer Node; Image Info Node; Editing (Nodes) under the interface reference; Command Line Arguments.
- Python API: `bpy.types.NodeLink`; `bpy.types.NodeSocket`; `bpy.types.NodeTree` (links and nodes); `bpy.types.Node` (internal_links); `bpy.types.Image`; `bpy.types.CompositorNodeSwitch`, `bpy.types.CompositorNodeAlphaOver`, `bpy.types.CompositorNodeHueSat` (all three carry only the class description and the generic socket-template methods, no socket list, so they added nothing beyond the manual); `info_tips_and_tricks`.
- Release notes: the whole 5.0 Compositor page. I also listed the other 5.0 release-note subjects and judged only the compositor one relevant to this question.
- Pictures: `compositing_realtime-compositor_compositing-space_operation-domain_considerations_clipping.png` (on the Compositor System page); `node-types_CompositorNodeSwitch.webp` (on the Switch Node page); `node-types_CompositorNodeAlphaOver.webp` (on the Alpha Over page).

**Ruled out and why:** Cryptomatte and ID Mask identify objects and materials *inside* one render; they say nothing about telling two pictures apart in the graph, which is what the question asks. The render output metadata page concerns what is stamped onto a written file. Cycles' own files were not opened because this question is about the compositor's node graph, which is not part of Cycles.

---

### Things a person at a computer could settle in a few minutes

**All eight of these have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above. Three things the Linux machine could not reach were run on the Creator's Mac on 2026-08-07, and are in *What a running Blender showed on the Creator's Mac*: the graph read the same with Blender's window open as with none; a Viewer image did appear when a window was open, where none exists at all without one; and a picture rendered with a window matched one rendered with none exactly. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **Ten of the fourteen tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run, and every answer here leaves Windows open.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Does a Mix Color node at Factor 0 give back exactly the A input?** Open the compositor. Add two Image nodes holding two visibly different pictures of the same size — **not** a flat colour from an RGB node, because the manual's quoted rule treats a "non single value input" differently from a single value, and a flat colour would change what is being measured. Feed the first picture into the Mix Color node's A input and the second into B. Set Factor to 0. Put a Viewer node on the Mix output and sample pixel values, then move the Viewer to the A input and sample the same pixel. Repeat with Factor 1 and the B input. *Negative result:* the sampled values differ between the Mix output and the input at the extreme setting, which would mean the extreme does not hand one input through untouched. That is a real possible outcome, so the test carries information.
2. **Which input decides the frame at Alpha Over, and does it match the manual's domain rule?** Build two Image nodes with pictures of clearly different sizes. Put the larger one into Foreground and the smaller into Background of an Alpha Over node, with a Viewer on the output. Note the size of the result. Swap the two inputs and note it again. *Negative result:* the result keeps the size of the larger picture in both arrangements, or changes size when the inputs are swapped in the direction opposite to what the manual's "clipped to the Background input" says. Either would contradict the documented rule.
3. **What passes through a muted Alpha Over node, and through a muted Mix Color node?** Wire two visibly different pictures into each. Select the node and press M to mute it. Look at the Viewer on its output. *Negative result:* the output shows neither input plainly, or shows the input the eye did not expect. The sources do not say which one it should be, so any of the outcomes is informative.
4. **Does the Switch node really hand its input through untouched?** Wire two different pictures into Off and On. Sample a pixel on the Switch output with the checkbox off, then sample the same pixel on the Off input directly. Repeat with the checkbox on and the On input. *Negative result:* the sampled values differ, meaning the node alters what it passes.
5. **Which physical side does the Split node's first input land on?** Wire two Image nodes holding two pictures that are told apart at a glance — again two pictures, not flat colours out of an RGB node, for the reason in test 1 — into the Split node's two Image inputs, leave Position at (0.5, 0.5) and Rotation at 0, and look at the Viewer output. *Negative result:* neither picture occupies a clean half, which would mean the two inputs are not simply divided by the line.
6. **Do the link properties read correctly with no interface?** Save a file containing a Render Layers node into a colour node into an Alpha Over node into the output. Run Blender with `--background` and a short script that walks the tree's links from the Render Layers output, printing each link's from-node, from-socket, to-node and to-socket, and prints where the walk ends. *Negative result:* the script errors, or the link list comes back empty, meaning the wiring is not readable without the interface.
7. **Is there a Viewer image to read with no interface?** With the same file, add a Viewer node, run Blender with `--background` and a script that renders a frame and then looks for an image whose source is `VIEWER` and reports how many pixel values it holds. *Negative result:* no such image exists, or it holds no pixels — which would mean the looking-at-it route in 8.4 is an interface-only route.
8. **Is there any per-socket domain priority visible from a script?** With a file open in the background, print every attribute name on an Alpha Over node's input sockets, and on a Mix Color node's, and look for anything naming a priority or a domain. *The outcome that would come out the other way:* an attribute naming a priority or a domain **does** appear on a socket — which would mean the setting is reachable from a script after all, and the *not in the sources* answer in 8.3 would have to be revisited. The opposite outcome, no such attribute, is the one the API documentation's silence points at. Both are possible answers from the machine, so the test carries information; it must not be written up as a confirmation of the expected one.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 5.0.

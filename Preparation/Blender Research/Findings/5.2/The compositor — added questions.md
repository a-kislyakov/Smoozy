# The compositor — added questions (Blender 5.2)

Sources consulted: Blender's 5.2 manual, Blender's 5.2 Python API documentation, and the Blender 5.2 release notes, all read 2026-07-30. Cycles' own 5.2 files, read 2026-07-30. Blender's requirements page, read 2026-08-01. No other source of any kind was used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing I did not choose, stated for the record: the project's `CLAUDE.md` was placed in front of me automatically before I began, not opened by me. I did not follow where it points, I did not open `Preparation/Groundwork/Start Here.md` or anything else it names, and its text contains no claim about how Blender behaves. I also checked that the folder I was told to save into exists, without listing or opening anything in it. I started no helper agents; every finding below is my own reading. Beyond that, the line above stands exactly as written.

Only question 8 was asked of me, and only question 8 is answered here.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

**The question was ambiguous, and I say so plainly.** "Followed forward" can mean two different things, and the sources answer them differently:

- **Reading A (the one I answer):** can the path of a picture be traced forward through the node graph — that is, can you establish, by looking at the file, that the picture arriving at a node is what leaves it and goes on to the next node.
- **Reading B:** can a picture be *recognised* on the far side — that is, does the picture itself carry any mark, name or record of where it came from, so that a picture found later can be matched back to it.

I answer Reading A first, because it is the reading that the phrase "followed forward … through" most naturally carries, and because the sources answer it. I then answer Reading B, because the second half of the question ("can it be told which of the two came out") only makes sense under it. Both are below. If only Reading B was meant, the answer is the short one in part (d).

---

### (a) Through a node that changes its colours — the manual lists one image input and one image output

**LABEL: documented.**

The RGB Curves node is a node that changes an image's colours. The manual gives it a single image input and a single image output.

> "The RGB Curves Node performs level adjustments on each color channel."

> "Inputs
>
> Image/Color
>
> Standard color input."

> "Outputs
>
> Image/Color
>
> Standard color output."

Source: Blender manual, RGB Curves Node — https://docs.blender.org/manual/en/5.2/compositing/types/color/adjust/rgb_curves.html — read 2026-07-30.

The manual's own picture of the node shows the same thing: one output socket labelled `Image` at the top right, and input sockets `Fac`, `Image`, `Black Level`, `White Level` down the left.

Picture: `[image: _images/node-types_CompositorNodeCurveRGB.webp - RGB Curves Node.]`, on the page above. **LABEL for the picture: documented** (it is part of the manual).

So the quoted words give this node one image output, not two: nothing in them splits the picture into more than one.

Two limits on that. The Inputs quote above shows only the `Image` entry, while the page's own picture shows four input sockets. What kind of thing `Fac`, `Black Level` and `White Level` carry, the quoted words do not say. Reading the node as taking exactly one picture in therefore rests on those three not being pictures, and that is a step past the words quoted here. **LABEL for that step: inferred.**

### (b) The connections are exposed as data, and each link names the node at both ends

**LABEL: documented.**

Blender's Python API exposes the connections as objects that point both ways, so a link found at a node's output tells you which node it goes to.

> "Link between nodes in a node tree"

and, on the same page, the four properties `from_node` (type `Node | None`), `from_socket` (type `NodeSocket | None`), `to_node` (type `Node | None`), `to_socket` (type `NodeSocket | None`), all marked `(readonly)`.

Source: Blender Python API, `NodeLink` — https://docs.blender.org/api/5.2/bpy.types.NodeLink.html — read 2026-07-30.

Every socket knows its own links:

> "List of node links from or to this socket."

(the `links` property of `NodeSocket`), and each socket carries

> "Unique identifier for mapping sockets (default “”, readonly, never None)"

(the `identifier` property of `NodeSocket`).

Source: Blender Python API, `NodeSocket` — https://docs.blender.org/api/5.2/bpy.types.NodeSocket.html — read 2026-07-30.

The whole tree's links are also listed in one place: `NodeTree.links`, type `NodeLinks[NodeLink]`.

Source: Blender Python API, `NodeTree` — https://docs.blender.org/api/5.2/bpy.types.NodeTree.html — read 2026-07-30.

The `identifier` property matters here for a practical reason. The manual's picture of the Alpha Over node shows **both** of its image inputs labelled simply `Image` (the text of the page calls them Background and Foreground). Picture: `[image: _images/node-types_CompositorNodeAlphaOver.webp - Alpha Over Node]`, on https://docs.blender.org/manual/en/5.2/compositing/types/color/alpha_over.html, read 2026-07-30. The same is true of the Split node's picture (`[image: _images/node-types_CompositorNodeSplit.webp - Split Node]`, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/split.html) — two inputs, both shown as `Image`. Telling those two apart by displayed name alone would not work; the socket identifier is documented as unique. The quoted words do not say within what it is unique, so that two inputs on one node carry different identifiers is not settled by them. Hand test 6 below is aimed at that.

**Nothing further is claimed here.** The API pages say these properties exist and what they hold. They do not say anything about what the *pixels* do.

### (c) Through a node that takes two pictures in and gives one out — it depends on which node, and what the sources say differs from node to node

**LABEL: documented.**

Blender 5.2's compositor has several nodes that take two images and give one out, and the manual states, for each of them, what the output is made of. They fall into two groups.

**Group one — the manual describes the output as a combination of both.**

Mix:

> "The Mix Node mixes values, colors and vectors inputs
> using a factor to control the amount of interpolation."

> "A/B
>
> The two inputs that are mixed together."

> "Result
>
> Output the result of the mix using the data type selected."

Source: Blender manual, Mix Color Node — https://docs.blender.org/manual/en/5.2/compositing/types/color/mix_color.html — read 2026-07-30.

Alpha Over:

> "The Alpha Over node composites two images by layering the Foreground image over the Background,
> using standard alpha blending operations."

> "Image
>
> The combined image after alpha compositing."

Source: Blender manual, Alpha Over Node — https://docs.blender.org/manual/en/5.2/compositing/types/color/alpha_over.html — read 2026-07-30.

For these two, the manual describes the far side as a combination of both inputs, and names no single input as the one that came out. What either node puts out at the extreme factor settings is not settled by these words — see *Not covered* below.

**Group two — the output is not described as a combination, and for two of the three the sources give the rule.**

Switch:

> "The Switch node outputs one of two inputs depending on a condition."

> "Switch
>
> Determines which of the two inputs below will be passed through.
>
> False
>
> Is passed through when the switch is set to false.
>
> True
>
> Is passed through when the switch is set to true."

> "Outputs
>
> Output
>
> One of the two inputs without any modifications."

Source: Blender manual, Switch Node — https://docs.blender.org/manual/en/5.2/compositing/types/utilities/switch.html — read 2026-07-30.

The API describes the compositor's own Switch node the same way:

> "Switch between two images using a checkbox"

Source: Blender Python API, `CompositorNodeSwitch` — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeSwitch.html — read 2026-07-30.

And the 5.2 release notes state that a Switch node's condition can be known before the tree runs:

> "The compositor now supports lazy evaluation of Switch nodes whose condition is known statically, that is, not connected or connected directly to a Group Input socket. Lazy Evaluation means nodes that are not needed are not executed to save execution time."

Source: Blender 5.2 release notes, Compositor — https://developer.blender.org/docs/release_notes/5.2/compositor/ — read 2026-07-30.

Depth Combine — here it is decided per pixel, and the rule is written down:

> "Outputs
>
> Result
>
> If both depth values are equal, it will use the foreground image.
> Whichever depth value is less decides which image pixel is used."

Source: Blender manual, Depth Combine Node — https://docs.blender.org/manual/en/5.2/compositing/types/color/depth_combine.html — read 2026-07-30.

Split — here it is decided per region:

> "The Split node combines two input images into a single output image,
> displaying them side by side or along a rotated split line."

> "Image
>
> The combined image showing both inputs divided by the split line."

Source: Blender manual, Split Node — https://docs.blender.org/manual/en/5.2/compositing/types/utilities/split.html — read 2026-07-30.

The quoted words say both inputs appear in the output, divided by the split line. They do not say which input takes which side. So for this node the words quoted here do not answer *which of the two* is at any given place, and I have not settled it.

**One more case, which applies to any node at all.** A node can be switched off, and when it is, the sources say the inputs pass straight through it:

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change.
> Links will appear red as an indicator of passing through the muted node."

Source: Blender manual, Editing Nodes — https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html — read 2026-07-30.

And which input passes to which output while muted is readable in a script:

> "internal_links
>
> Internal input-to-output connections for muting (default None, readonly)
>
> Type:
>
> bpy_prop_collection[NodeLink]"

Source: Blender Python API, `Node` — https://docs.blender.org/api/5.2/bpy.types.Node.html — read 2026-07-30.

The quoted words say this property holds the input-to-output connections used for muting, and that its default is None. Taking that to mean a muted two-in-one-out node's `internal_links` names which of the two reached the far side is a step past those words: it assumes the property is filled in, and that what it names matches what actually passes through. **LABEL for that step: inferred.** Hand test 4 below is aimed at it.

### (d) Reading B — does the picture itself carry any record of where it came from?

**LABEL: not in the sources.**

I found nothing in the 5.2 manual, the 5.2 API documentation or the 5.2 release notes that says an image travelling through the compositor carries any name, tag, or record of the node or input it came from. The two places the sources describe *what is known about an image at a point in the graph* both list only its shape and sampling, never its origin.

Socket inspection:

> "Socket inspection displays information about the image contained in a socket
> after the node tree has been evaluated."

> "Rather than displaying the image itself, the inspection panel summarizes
> properties of the image, making it easier to verify intermediate results and
> debug node trees."

The listed items are Resolution, Transform, Sampling, Precision, and nothing else. The page also warns:

> "Socket values are only available after the node tree has been evaluated.
> Nodes that do not contribute to the final output are generally not evaluated
> and therefore have no inspection data available."

and

> "Pixel sockets are currently not supported."

Source: Blender manual, Compositor System — https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30.

The manual's picture of this feature confirms the list: a tooltip on a socket reading `Resolution: 1920x1080`, `Transformation: Identity`, `Sampling: • Interpolation: Bilinear • Extension: Clip`, `Precision: Half`, `Type: Image`. Picture: `[image: _images/compositing_socket-inspection.png]`, on the page above. **LABEL for the picture: documented.** Nothing in it names where the image came from.

The Image Info node, the compositor's own node for asking about an image, is the same:

> "The Image Info node outputs spatial and transformation information about an image in the compositor."

Its outputs are Dimensions, Resolution, Location, Rotation and Scale — no origin, no identity.

Source: Blender manual, Image Info Node — https://docs.blender.org/manual/en/5.2/compositing/types/input/image_info.html — read 2026-07-30.

Two related statements point the same way. On sockets generally:

> "Once a socket conversion is made, data may be lost and cannot be retrieved later down the node tree."

Source: Blender manual, Node Parts — https://docs.blender.org/manual/en/5.2/interface/controls/nodes/parts.html — read 2026-07-30.

And the manual describes what an image *is* inside the compositor purely as data and a rectangle in space — a type (Float, Vector or Color), a size, and a transformation — with no mention of an identity:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "The rectangular area occupied by an image in that space as defined
> by its transformation and size is called the Domain of the image."

Source: Blender manual, Compositor System — https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30.

**The one thing the sources do offer under Reading B is a deliberate marking technique**, described on the Mix node's own page — you put a mark into the image yourself, and later test for it:

> "Blender provides a complete set of tools for you to both encode your watermark
> and to tell if an image has your watermark."

and, for the test:

> "When you see an image that you think might be yours,
> use the node tree below to compare it to your stock image (non-watermarked original).
> In this tree, the Mix node is set to Difference,
> and the Map Value node amplifies any difference."

Source: Blender manual, Mix Color Node — https://docs.blender.org/manual/en/5.2/compositing/types/color/mix_color.html — read 2026-07-30.

That is a mark somebody adds on purpose, not something Blender carries for you. **LABEL for this technique: documented.** It sits inside a section otherwise labelled *not in the sources*, so it carries its own label.

### One further documented point that bears on "which came out"

For Alpha Over specifically, the manual says the output's *extent* is taken from the Background input, not the Foreground:

> "The output of nodes will be intuitively clipped to the operation domain, or rather, the domain of
> the domain input. For instance, if the Foreground input is bigger than the Background input in
> the Alpha Over node, the output will be clipped to the Background input because it is the domain input"

> "The Alpha Over node currently does not support changing
> the domain priority for its inputs, so as a workaround,
> one can use a Mix node to achieve the desired behavior,
> noting that the first Image input in the Mix node has the highest domain priority"

Source: Blender manual, Compositor System — https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html — read 2026-07-30.

So even where the colours are a blend of both, the *size and placement* of the result follow one designated input, and the manual names which one.

### What a running Blender showed

**LABEL: observed.** Everything in this section was got by running Blender 5.2.0 LTS on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, and each Blender was pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics' tests while this ran, so nothing here reports how long anything took. The full procedure, every script in full, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only. The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**A control first (entry 1).** A picture put through a bare tree and written out came back identical to the picture that went in, and the same render twice gave the same picture:

> `MAXDIFF render vs source 0.0`
> `MAXDIFF render vs same render again 0.0`

So every comparison below is a comparison of pictures, not of files, and a difference of zero means the pixels match.

**Which nodes take two pictures in, and what their sockets are called (entry 2).**

> `ADDABLE NODE TYPE COUNT 168`
> `TWO-PICTURE-IN, ONE-PICTURE-OUT COUNT 25`
> `SWITCH-LIKE NAMES AMONG ADDABLE ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'FunctionNodeInputMenu', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch']`

So the Index Switch and Menu Switch nodes mentioned in the record above are not compositor node types at all — they are the geometry-node ones, added to a compositor tree.

*[Note added by the citation check, 2026-08-06. The rule that picked those five names is not written down here, and a repeat on 2026-08-07 that kept every addable type with "Switch" in its name came back `['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch', 'GeometryNodeSwitch']` — one name more and one name fewer. `GeometryNodeSwitch` is addable to a 5.2 compositor tree and is not addable to a 5.1 one, on both runs, so it is a 5.2 arrival this line misses. The conclusion above — that the Index Switch and Menu Switch are geometry-node types — holds on both runs. The original line is left as it was; what is added is the rule, which the next reader will need.]*

*[**Answered 2026-08-07** — see entry 21 below, which ran the sweep in all four versions with both rules printed side by side. The line above was made with the wider rule — `Switch` or `Index` or `Menu` in the type name — which is why it carries `FunctionNodeInputMenu`. But it is missing `GeometryNodeSwitch`, and both rules keep that one, so the line is short by a name under its own rule. `GeometryNodeSwitch` is addable to a 5.2 compositor tree and to no earlier one. The conclusion the line was drawn for is untouched on either rule.]*

> `PRESENT CompositorNodeSwitch label: Switch`
> `   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

The 5.2 manual page quoted above names this node's sockets `Switch`, `False`, `True` and `Output`. The running 5.2 offers `Switch`, `Off`, `On` and `Image`. The two disagree.

> `PRESENT CompositorNodeAlphaOver label: Alpha Over`
> `   IN  [('Background', 'Background', 'RGBA'), ('Foreground', 'Foreground', 'RGBA'), ('Factor', 'Fac', 'VALUE'), ('Type', 'Type', 'MENU'), ('Straight Alpha', 'Straight Alpha', 'BOOLEAN')]`
> `   OUT [('Image', 'Image', 'RGBA')]`
> `PRESENT CompositorNodeSplit label: Split`
> `   IN  [('Position', 'Position', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`
> `PRESENT CompositorNodeZcombine label: Depth Combine`
> `   IN  [('A', 'A', 'RGBA'), ('Depth A', 'Depth A', 'VALUE'), ('B', 'B', 'RGBA'), ('Depth B', 'Depth B', 'VALUE'), ('Use Alpha', 'Use Alpha', 'BOOLEAN'), ('Anti-Alias', 'Anti-Alias', 'BOOLEAN')]`
> `   OUT [('Result', 'Result', 'RGBA'), ('Depth', 'Depth', 'VALUE')]`
> `PRESENT CompositorNodeCurveRGB label: RGB Curves`
> `   IN  [('Image', 'Image', 'RGBA'), ('Factor', 'Fac', 'VALUE'), ('Black Level', 'Black Level', 'RGBA'), ('White Level', 'White Level', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

This last one settles the step marked inferred in part (a): the RGB Curves node has one picture input and one picture output, and its other three inputs are a single value and two colour swatches.

**Hand test 6 of this document, settled: the two same-named picture inputs do carry different identifiers.** The Split node's are `Image` and `Image_001`, above; the Alpha Over node's are `Background` and `Foreground`, so in this version they do not share a name at all.

**What comes out of a Mix at the ends of its factor (entry 3).**

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=5.960464477539063e-08`

with the first picture on the A input and the second on B. At factor 0 the output matches A to the last figure; at factor 1 it matches B to within one step of a floating-point number.

**Alpha Over at the ends of its factor, and the Switch node (entry 4).**

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

The foreground picture used was opaque throughout, which is why factor 1 gives it back whole. That last pair is hand test 7 of this document, settled: the Switch node's output matches the corresponding input exactly.

**What the file says about which input reaches the output (entry 5).** This is hand test 4 of this document, settled:

> `=== AlphaOver (CompositorNodeAlphaOver) ===`
> `  unmuted internal_links: [('Background', 'Image')]`
> `  muted   internal_links: [('Background', 'Image')]`
> `=== Switch (CompositorNodeSwitch) ===`
> `  unmuted internal_links: [('Off', 'Image')]`
> `=== Split (CompositorNodeSplit) ===`
> `  unmuted internal_links: [('Image', 'Image')]`
> `=== Z/Depth Combine (CompositorNodeZcombine) ===`
> `  unmuted internal_links: [('A', 'Result'), ('Depth B', 'Depth')]`

So `internal_links` is filled in — it is not left at the `None` its description gives as a default — and names one picture input on every one of these nodes, muted or not.

One thing 5.2 printed that 4.5, 5.0 and 5.1 did not, on a Mix node with both picture inputs linked and the node unmuted:

> `  unmuted is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', True), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`

where the other three print `('A', 'A_Color', False)` in that place. On the Alpha Over, Switch, Split and Depth Combine nodes `is_inactive` stayed false throughout, muted or not.

*[**Answered 2026-08-07** — see entry 22 below, which ran this line in all four versions side by side and then swept the Factor. The line above is reproduced exactly. But it is **not** a difference between the versions: all four grey out the A input at Factor 1.0 and the B input at Factor 0.0, and neither in between. What differs is where the Factor starts — 1.0 in 5.2 and 0.5 in the other three — and this entry never set it. So `is_inactive` here is reporting a Factor's arithmetic, not a wiring fact, and it cannot on its own say which picture reached the far side. The `internal_links` readings and the renders of entries 5 and 7 are what do that, and they are unaffected.]*

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

so the first input lands on the left of the frame and the second on the right — the thing the answer above says the manual does not state.

> `DEPTH COMBINE first=picA(depth 0.2) second=picB(depth 0.8): maxdiff vs picA=5.960464477539063e-08 maxdiff vs picB=1.0`
> `DEPTH COMBINE first=picA(depth 0.8) second=picB(depth 0.2): maxdiff vs picA=1.0 maxdiff vs picB=5.960464477539063e-08`
> `DEPTH COMBINE with a depth running 0 on the left to 1 on the right for picA, 0.5 flat for picB:`
> `   DEPTHGRAD left column x=5 y=32 -> [0.07937, 0.50794, 0.25, 1.0]   (picA there [0.07937, 0.50794, 0.25, 1.0], picB there [0.1, 0.92063, 0.8, 1.0])`
> `   DEPTHGRAD right column x=58 y=32 -> [0.1, 0.07937, 0.8, 1.0]   (picA there [0.92063, 0.50794, 0.25, 1.0], picB there [0.1, 0.07937, 0.8, 1.0])`
> `   maxdiff vs picA=1.0  maxdiff vs picB=1.0`

**Which input decides the frame (entry 9).** A 32 by 32 picture and a 128 by 128 picture, rendered at 128 by 128, reading the area whose alpha is above zero:

> `AlphaOver(bg,fg)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `AlphaOver(bg,fg)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`
> `Mix(A,B)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `Mix(A,B)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 48..79, y 48..79  (32 by 32)`

Alpha Over follows its Background input. The Mix node came back at the smaller of the two sizes both ways round in this version, where 4.5, 5.0 and 5.1 gave the larger size when the larger picture was in the A input.

*[The second of those two sentences is **superseded on 2026-08-07 by entry 20 below**, which is where to read. The four lines above are right and were reproduced exactly; the sentence drawn from them is not. There is no difference between the versions here. The Mix node's frame follows its first colour input in all four, at every Factor. This entry never wrote its Factor down, and a freshly added Mix node starts at Factor 1.0 in 5.2 and at 0.5 in the other three — so this entry read 5.2 at one setting and the other three at another. Leaving the Factor alone, exactly as this entry did, gives these same two Mix lines back to the letter in 5.2 and the other pattern in the other three. The Alpha Over sentence stands untouched.]*

*[Disagreement recorded by the citation check, 2026-08-06. The Mix half of this entry was run again on the same rented Linux machine on 2026-08-07 and came back differently: with the large picture on the first input and the small one on the second, 5.2 gave `alpha covers x 0..127, y 0..127  (128 by 128)`, not 32 by 32. Run again on its own, with the Factor swept, 5.1 and 5.2 behaved identically to each other, and which size came out depended on where the Factor sat: at 0.0 and 0.5 the large-first arrangement gave 128 by 128 on both versions, and only at Factor 1.0 did it give 32 by 32 on both. The two runs are written out in full in `Audits/Blender Research/The compositor/Citation check/Added questions — Opus 5.md`. Neither result is being overwritten and neither is being picked. The Alpha Over half of this entry stands on both runs. The Mix half goes back to the hand tests to be run again, with the Factor stated in the record.]*

*[**Settled 2026-08-07** — see entry 20 below. It was run again, with the Factor stated and swept and the frame read straight off the socket. Neither of the two runs above was wrong about its numbers; both drew a conclusion the numbers did not carry, because both read the area whose alpha is above zero and that is a reading of the alpha rather than of the frame. The frame follows the first colour input in all four versions at every Factor. The version difference this entry reported was a starting Factor of 1.0 in 5.2 against 0.5 in the other three.]*

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
> `file_name is '{blend_name}'`
> `FILE OUTPUT input sockets: [('MyBranchName', 'Item_1'), ('', '__extend__')]`
> `WHAT LANDED IN THE FILE OUTPUT FOLDER: ['Unsaved.exr']`

The socket's name did not reach the written file's name. The node's file-name setting was left at its factory value, `{blend_name}`, which resolved to the unsaved file's name. Blender's own description of that setting says only "The base name of the file. Other information might be included in the final file name depending on the node options", and what that other information is, or whether a token standing for the socket's name exists, was not settled.

**Reading the whole graph out of a saved file with no window (entry 11).** This is hand test 5 of this document, settled. The tree was built and saved by one Blender and read back by a second, separate one, with no window at either end:

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

> `compositor_device present: True value: GPU`
> `compositor_precision present: True value: AUTO`
> `MAXDIFF between the processor's picture and the card's picture: 6.854534149169922e-07`

So in this version the compositing device starts at `GPU`, where 4.5, 5.0 and 5.1 start at `CPU`. This machine has no display server and `nvidia-smi` on it would not start (`Failed to initialize NVML: Driver/library version mismatch`), so whether the run marked `GPU` really used the card is not settled. What is settled is that changing that setting changed the picture, by about seven parts in ten million.

**The card was repaired, and the card half was run again on 2026-08-07 (entry 12).** The same rented Linux machine — **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and still no display server at all** — now has a working graphics card, its kernel driver and its driver library agreeing at last. The card answers:

> `NVIDIA-SMI 580.173.02             Driver Version: 580.173.02     CUDA Version: 13.0`
> `NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.173.02  Release Build`
> `NVIDIA GeForce RTX 4080 SUPER, 580.173.02, 16376 MiB`

and no run printed an EGL line of any kind. **The lines just above are superseded by what follows**: with the card unreachable on 2026-08-06, that run marked `GPU` cannot have been the card's work, so it was never a comparison of the processor against the card. That this version starts at `GPU` is unchanged, and it still reads `GPU` on a fresh start:

> `compositor_device present: True value: GPU`
> `compositor_precision present: True value: AUTO`

**This version starting at `GPU` now means something it did not mean before.** On a machine with a working card, a 5.2 file opened with nothing changed will composite on the card, where 4.5, 5.0 and 5.1 will composite on the processor.

The same tree — a colour node and an Alpha Over at factor 0.5 — rendered once with the compositing device set to `CPU` and once to `GPU`, and the two pictures loaded back and compared pixel by pixel:

> `DEVICE CPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `DEVICE GPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `MAXDIFF between the processor's picture and the card's picture: 6.258487701416016e-07`

**The run marked `GPU` really does use the card.** A second, heavier tree — a 1024 by 1024 picture through sixty Blur nodes, rendered at 2048 by 2048 — was run twice, one device to a Blender, while the card was asked over and over which processes were on it. Only this Blender's own process was counted, because the machine is shared with other topics' tests. Set to the processor:

> `ASKED FOR DEVICE CPU`
> `compositor_device now reads: CPU`
> `HEAVY RENDER SECONDS CPU 5.197`
> `HEAVY PROCESSOR CORE-SECONDS CPU 38.300`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 115`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`

Set to the card:

> `ASKED FOR DEVICE GPU`
> `compositor_device now reads: GPU`
> `HEAVY RENDER SECONDS GPU 2.370`
> `HEAVY PROCESSOR CORE-SECONDS GPU 3.370`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 47`
> `TIMES THIS PROCESS WAS ON THE CARD: 12`
> `MOST CARD MEMORY THIS PROCESS HELD: 150 MiB`

Only the setting differed between those two runs, and three things move together. Set to the processor, this Blender never appeared on the card at all. Set to the card, it appeared on it holding 150 MiB, and its own share of processor work fell from thirty-eight core-seconds to a little over three.

**And the card's picture matches the processor's picture.** The two heavy renders, loaded back and compared pixel by pixel:

> `HEAVY CPU SIZE (2048, 2048) HEAVY GPU SIZE (2048, 2048)`
> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`
> `HEAVY MIDDLE PIXEL, processor: [0.500416, 0.500416, 0.25, 1.0]`
> `HEAVY MIDDLE PIXEL, card     : [0.500416, 0.500416, 0.25, 1.0]`

So the two devices give the same picture to about four parts in ten million on the heavy tree and about six parts in ten million on the small one. Not identical, and not far off: a picture cannot be told apart by its exact values across the two devices, but any test coarser than a millionth still holds. The seconds above are rough, because the machine was shared while this ran; the count of times this process was on the card is not, because it names one process by its own number.

Every property name on a colour node's output socket, after a render — this is hand test 1 of this document, as far as a script can reach it:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`
> `ANY NAMING INSPECTION, RESOLUTION, TRANSFORM, PRECISION, SAMPLING, VALUE OR IMAGE: ['default_value', 'hide_value']`
> `default_value on the output socket: <bpy_float[4], NodeSocketColor.default_value>`

Nothing on a socket names an inspection, a resolution, a transform, a precision or a sampling. `default_value` on a colour socket holds four numbers, which is a socket's own fixed colour rather than the picture that travelled through it. What the inspection panel draws when a socket is hovered is not something a script can reach, and stays untested.

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
> `   its output socket names: [('Dimensions', 'Dimensions', 'INT_VECTOR'), ('Resolution', 'Resolution', 'INT_VECTOR'), ('Location', 'Location', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Scale', 'Scale', 'VECTOR')]`

**Added 2026-08-07 — the three tests the citation check sent back (entries 20, 21 and 22 of the hand-test list).**

**LABEL: observed.** Everything in this sub-section was got by running Blender 5.2.0 LTS on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB with its kernel driver and its driver library both at 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all**, reached at `ssh -i ~/.ssh/id_ed25519 -p 39348 root@85.218.235.6` — on **2026-08-07**. Every run used `-b --factory-startup` and was pointed at a settings folder of its own, made beforehand under `/root/again-compositor/`, which is a third working folder outside both earlier sittings and outside every other topic's work. The Blender named itself with the same build hash as on 2026-08-06, so this is the same build. The machine was shared with other topics while this ran, so nothing here reports how long anything took or how much of the card was in use, and nothing here needed either. **Pictures were compared as pictures, never as files.** The full procedure, every script, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 20, 21 and 22. This settles Linux alone; none of the three has been run on the Creator's Mac.

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

5.1 gave the same twelve lines. 4.5 and 5.0 differ in two places, both of them an alpha: the pixel outside the small picture reads alpha 1.0 in them where it reads 0.5 here at Factor 0.5, and alpha 1.0 where it reads 0.0 here at Factor 1.0 — which is why their Factor 1.0 line reports 128 by 128 and this one reports 32 by 32.

Way two, the frame read straight off the socket. **This is the answer.** Word for word:

> `IMAGE INFO NODE USED CompositorNodeImageInfo outputs [('Dimensions', 'Dimensions', 'INT_VECTOR'), ('Resolution', 'Resolution', 'INT_VECTOR'), ('Location', 'Location', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Scale', 'Scale', 'VECTOR')]`
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

> `FRESH ShaderNodeMix, nothing set: Factor_Float default_value = 1.0`
> `AFTER data_type='RGBA' and blend_type='MIX': Factor_Float default_value = 1.0`

It is `0.5` in 4.5, 5.0 and 5.1 and `1.0` in 5.2. With the Factor then left alone — which is entry 9's own condition, reproduced:

> `Mix(A,B)  FACTOR LEFT AT THE DEFAULT 1.0  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `Mix(A,B)  FACTOR LEFT AT THE DEFAULT 1.0  first input = picLarge(128), second = picSmall(32) -> alpha covers x 48..79, y 48..79  (32 by 32)`

Those two lines are entry 9's two Mix lines for this version, back to the letter — including the one that made 5.2 look unlike the other three.

**So the disagreement is settled, and both runs that stood are explained rather than overruled.**

- **The answer.** A Mix node's output takes its frame from its first colour input, in all four versions, at every Factor tried.
- **Why entry 9 saw a difference between the versions.** It never set the Factor, so each version ran at its own starting Factor — 0.5 in three of them and 1.0 in 5.2. Its numbers were right; the sentence drawn from them was not, because what differed was the setting and not the behaviour.
- **Why the citation check's repeat suggested the Factor decides the frame.** It set the Factor and swept it, both correctly, and 5.1 and 5.2 then matched each other, correctly. But it read the frame by the area whose alpha is above zero, and that is a reading of the alpha. At Factor 1.0 the output's alpha is the second input's alpha, so from 5.1 outward the covered area shrinks to the second input while the frame stays on the first. The frame never moved; the alpha did.
- **A real difference between the versions, newly shown here.** From 5.1 the Mix node mixes the alpha as well as the colour. At Factor 0.5 with the large picture on A and the small one on B, a pixel outside the small picture comes out `[0.1, 0.1, 0.45, 1.0]` in 4.5 and 5.0 and `[0.1, 0.1, 0.45, 0.5]` in 5.1 and 5.2 — same colours, different alpha.
- **A second real difference.** The Mix node's starting Factor is 1.0 in 5.2 and 0.5 in the other three, which is what a file opened with nothing changed will use.

**What counts as a switch-like node type in a compositor tree, with the rule written down (entry 21).** Every name in `dir(bpy.types)` offered to `nodes.new()` on a compositor tree; the accepted ones kept if the type name contains the exact text `Switch`, and the older wider rule — `Switch` or `Index` or `Menu` — printed beside it, because entry 2 above never wrote its rule down and the citation check could not check one list against the other without it.

**What a negative result would have looked like.** The two rules could have given the same list in 5.2, which would leave the difference between the two earlier lists unexplained. `GeometryNodeSwitch` could have come back addable in 5.1 too, in which case it is not a 5.2 arrival. The addable counts could have come back unlike entry 2's, which would mean the two sweeps are not looking at the same thing.

Word for word, in this version:

> `ADDABLE NODE TYPE COUNT 168`
> `SWITCH IN THE TYPE NAME ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch', 'GeometryNodeSwitch']`
> `THE OLDER, WIDER RULE — 'Switch' or 'Index' or 'Menu' in the type name: ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'FunctionNodeInputMenu', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch', 'GeometryNodeSwitch']`
> `NAMES THE WIDER RULE ADDS THAT HAVE NO 'Switch' IN THEM: ['FunctionNodeInputMenu']`

This is the version the citation check queried, and both rules are now on the record. Entry 2's recorded list for 5.2 carries `FunctionNodeInputMenu`, which only the wider rule keeps — so entry 2 was using the wider rule. But entry 2's list is missing `GeometryNodeSwitch`, and **both** rules keep that one, so entry 2's line is short by a name under its own rule. `GeometryNodeSwitch` is addable to a 5.2 compositor tree and is not addable to a 4.5, 5.0 or 5.1 one, so it is a 5.2 arrival. The addable count, 168, is entry 2's count to the number, so the two sweeps are looking at the same thing. The point entry 2 drew from its list is untouched on either rule: every switch-like name beyond the two `CompositorNode` ones is a geometry-node or function-node type added to a compositor tree, not a compositor node.

**`is_inactive` on a Mix node with both picture inputs linked (entry 22).** A `ShaderNodeMix` set to `RGBA` and `MIX`, with a different picture linked to each of its two colour inputs and the tree's output linked up, printing `(name, identifier, is_inactive)` for every input socket, unmuted and then muted, **before any render and again after one**. Entry 5 above records this line for 5.2 only, and the citation check had nothing to set beside it.

**What a negative result would have looked like.** The 5.2 reading could have held at `True` across every Factor, which would make it a real difference between the versions and leave entry 5's line standing as written. It could have failed to reproduce at all. The reading could have changed after a render, which would mean both earlier runs read it at the wrong moment. And the Factor sweep could have shown no movement, which would leave the 5.2 line unexplained.

The Factor the node held, never set by the script, in this version:

> `A_Color is_linked: True   B_Color is_linked: True   Factor_Float held: 1.0`

and the line itself, word for word — identical before a render and after one, so the pair is printed once:

> `  unmuted is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', True), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`
> `  muted   is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', True), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`

Entry 5's 5.2-only `('A', 'A_Color', True)` is reproduced exactly, muted and unmuted alike, before a render and after one. Nobody misread it. But it is not a difference between the versions, as the sweep below shows.

**The Factor sweep, which is the control on all of it.** Word for word, and **identical in all four versions**:

> `FACTOR 0.0  A_Color is_inactive=False  B_Color is_inactive=True  (both linked: True, True)`
> `FACTOR 0.25  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 0.5  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 0.75  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 1.0  A_Color is_inactive=True  B_Color is_inactive=False  (both linked: True, True)`

So `is_inactive` on this node's two colour inputs behaves the same way in all four versions: A greys out at Factor 1.0, B at Factor 0.0, neither in between. Entry 5's 5.2-only line is the starting Factor of 1.0 showing through, the same fault as in entry 9 in a different place. And **what `is_inactive` reports is a Factor's arithmetic, not a wiring fact** — a socket with a picture linked to it still reads inactive when the Factor makes its contribution nil. So it cannot on its own say which picture reached the far side. `internal_links` and the renders of entries 5 and 7 are what do that, and they are unaffected.

### What a running Blender showed on the Creator's Mac

**LABEL: observed.** Everything in this section was got by running Blender 5.2.0 LTS on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**. That Blender says of itself:

> `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)`

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-5.2`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 15 to 18.

**The graph reads the same with a window open as with none (entry 15).** One Blender built and saved the tree with no window; a second opened that file with no window and printed it; a third opened the same file with the window open and printed it. The run with a window, word for word:

> `VERSION 5.2.0 LTS`
> `FILE /Users/artist/Documents/Smoozy hand tests/The compositor/graph_5_2.blend`
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

**Socket inspection stays out of reach of a script when a window is open (entry 17).** This is the version whose manual names the feature. The same list of property names came back from both runs, the one with a window and the one without:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree, and none of the names is an inspection, a resolution, a transform, a precision or a sampling. So nothing behind the inspection panel becomes reachable from a script when there is a window. What the panel *draws* when a socket is hovered is still untested, because a script cannot hover a socket.

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

Those are the same four lines the Linux machine printed with no window (entry 4). The two machines agree. Note that this Mac has Apple's own graphics and no second card, and the compositing device was left wherever 5.2 starts it, which is the card.

**One thing about ending a Blender that has a window.** The scripts were written to close Blender with `bpy.ops.wm.quit_blender()`. On this Mac that line on its own did not end it: the script's printing all appeared, and the process was still running when a limit of my own ended it. The same script with `sys.exit(0)` added after that line ended in about a second and printed the same thing. Both are recorded in entry 19 of the hand-test list.

### Not covered

- The answer settles what the sources **say**. It does not settle what the compositor **does** to the pixels beyond the quoted sentences. In particular I did not, and could not from these sources, confirm what a Mix node outputs at a Factor of exactly 0 or exactly 1, nor what Alpha Over outputs at a Factor of 0. The manual says Alpha Over's Factor "Controls the opacity of the foreground image, from 0 (fully transparent) to 1 (fully opaque)" — reading a result of "the background image unchanged" out of that would be my step, not the manual's, and I have not taken it.
- ~~I did not verify that reading `NodeLink` and `NodeSocket.links` actually reproduces the graph in a running Blender. I read the API pages; I ran nothing.~~ **Done in entry 11 above**, which builds a tree in one Blender, reads it back in a second with no window, and prints every link with both its nodes and both its sockets. *[Changed by the citation check, 2026-08-06: this bullet was left standing after the test that answered it.]*
- The Split node's page and the Split node's picture disagree about its inputs: the text describes `Position` (a 2D vector) and `Rotation` (degrees), while the picture shows `X`/`Y` buttons and a `Factor` of 0.500. Both are in the 5.2 manual. This does not change the answer (two image inputs, one image output, and the split line decides which region is which), but I flag it rather than pick one.
- Whether an image is *the same image* in any sense stronger than "it is what the previous node produced" is not something the sources define. There is no notion of image identity in them that I could find.
- Compositor node groups can also be used from the Video Sequencer ("compositor strips and compositor modifiers", per https://docs.blender.org/manual/en/5.2/compositing/usage.html, read 2026-07-30). I did not follow that path; everything above is about a compositor node tree.
- Nothing here covers Geometry Nodes or Shader Nodes, even though several of the quoted API pages (`NodeLink`, `NodeSocket`, `Node`) are shared across all node systems.

### Outside the interface

Does this hold when Blender is driven from outside its own window?

**LABEL: observed for the reading half, for what a node puts out, for socket inspection and for the Viewer, on Linux and on macOS.**

**Reading the connections holds with no window, and reads the same with one.** On the rented Linux machine named above, on 2026-08-06, Blender 5.2.0 LTS with `-b` opened a saved file and printed the whole chain — picture node, colour node, two-picture node, output node — with every link's node and socket at both ends, the Switch node's stored choice, and each node's internal links. That is entry 11 above, and it replaces the inference below for this half. On the Creator's Mac on 2026-08-07 the same file was read both with no window and with the window open, and the two printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round.

**What a node puts out does not change with a window.** Every one of the pixel results above was got with no window: Mix and Alpha Over at the ends of their factor, the Switch node's pass-through, a muted node's pass-through, the Split node's two sides, Depth Combine's per-pixel choice, and which input decides the frame. On the Mac, four of those renders were made twice — once with no window and once with a window — and every pair matched to the last figure (entry 18). So the compositor's result is the same whether Blender is driven from outside its own window or not.

**Socket inspection is not reachable from a script, window or no window.** No property on a socket names an inspection, a resolution, a transform, a precision or a sampling, with no window (entry 12) and with one (entry 17), on either machine. What the panel draws when a socket is hovered is still untested, because a script cannot hover a socket.

**The Viewer node produces nothing readable with no window, and does with one.** Its image exists but stayed at 256 by 256 with every value zero across a 64 by 64 render with no window, on both machines (entry 10, and again on the Mac). With a window open on the Mac it came back at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16).

**The compositor can be put on the graphics card with no window at all.** On the same machine with its card repaired — driver 580.173.02 — on 2026-08-07, a Blender started with `-b` and its compositing device set to the card really did the work there: the card named that Blender's own process and it held 150 MiB while it worked, where the same run set to the processor never appeared on the card. So choosing the device is not an interface feature, and neither is acting on the choice (entry 12). This matters more in 5.2 than in the other three, because 5.2 starts at the card and the other three start at the processor.

The reading from the sources, unchanged, follows.

- **Reading the connections — part (b), and the muting point in part (c).** `NodeLink`, `NodeSocket`, `Node.internal_links` and `NodeTree.links` are Python API properties of the file's own data. The API documentation for them makes no mention of the user interface, an editor, or a window. **LABEL: inferred.** The step I took: the pages describe these as data on the node tree and never condition them on an editor being open, so I take them to be readable wherever `bpy` is. The sources do not state this, and I did not test it.
- **What a node puts out — parts (a) and (c).** The reasoning above does not reach these; it is about reading the tree, not about what comes out of a node. Nothing quoted above ties what a node outputs to a window being open, and the whole-folder searches recorded below for `background mode`, `--background`, `headless` and `command line` turned up nothing on it. **LABEL: not in the sources.**
- **Part (d)'s socket inspection is a different matter.** The manual describes it as an inspection panel that *displays* information, and its picture is a tooltip in the Node Editor. **LABEL: inferred** that it is therefore of no use when there is no window: the step is that a thing described only as a display in an editor needs that editor. The manual does not say this outright, and I flag it as my inference, not theirs. Its own stated condition — "Socket values are only available after the node tree has been evaluated" — is documented.
- The Viewer node, the other way of looking at an intermediate result, is described entirely in terms of the compositor backdrop and the Image Editor, and is called "a diagnostic tool that allows users to inspect intermediate results without affecting the final output" (https://docs.blender.org/manual/en/5.2/compositing/types/output/viewer.html, read 2026-07-30). The sources do not say what it does with no window open. **LABEL: not in the sources.**
- The manual does say the compositor's active output is chosen without reference to any editor: "the compositor searches for the active Group Output node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether." (Compositor System, address above, read 2026-07-30.) **LABEL: documented.**


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

Does this hold on every operating system Blender runs on?

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card half of entry 12 was run again on that same Linux machine on 2026-08-07, once its graphics card had been repaired and its driver stood at 580.173.02, and it settles Linux alone in exactly the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window (entry 11 on Linux, entry 15 on the Mac); the four Alpha Over and Switch renders (entry 4 on Linux, entry 18's no-window half on the Mac); the Viewer node's image staying empty with no window (entry 10 on Linux, entry 16's no-window half on the Mac); and every property name on a socket (entry 12 on Linux, entry 17 on the Mac). Nothing came out differently on the two operating systems.

**Ten of the fourteen Linux tests are still owed on the Mac**, and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card — which matters more in 5.2 than in the other three, since 5.2 starts at the card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding.

The reading from the sources, unchanged, follows.

- **The sources say nothing about any operating system difference in how nodes, links or sockets are read, or in what any of the nodes above output.** I searched the compositing section and the node API pages and found no such statement. **LABEL: not in the sources.** The record below names the areas looked at but no operating-system search terms, so this line rests on a thinner record of looking than the rest.
- The one operating-system-specific statement I found anywhere in the compositing section is unrelated to following an image forward, but I report it because it is the only one: "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." — under a heading reading "TDR on Windows". Source: Blender manual, Limitations — https://docs.blender.org/manual/en/5.2/compositing/limitations.html — read 2026-07-30. **LABEL: documented.** That is about the compositor being interrupted, not about which image comes out of a node.
- Blender's requirements page does not name version 5.2 anywhere in the file I was given, so it speaks for some other release and I have not used it here. (`requirements_2026-08-01.txt`, https://www.blender.org/download/requirements/, read 2026-08-01.)

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
## Where I looked

Searches were run across whole folders, not by opening likely files.

**Searched, whole-folder, in `manual_5.2`, `api_5.2` and `release_notes_5.2`** — terms: `metadata`, `provenance`, `keep track`, `trace`, `traced`, `tracing`, `lineage`, `audit`, `identify`, `identified`, `which image`, `which of the two`, `source image`, `originating`, `origin of`, `passthrough`, `pass-through`, `pass through`, `passed through`, `without any modifications`, `unchanged`, `carried over`, `internal_links`, `from_socket`, `mute`, `muting`, `inspection`, `background mode`, `--background`, `headless`, `command line`.

**Pages opened and read in full:**
- Manual: Compositor System; Introduction; Using The Compositor; Limitations; Mix Color Node; Alpha Over Node; RGB Curves Node; Depth Combine Node; Split Node; Switch Node; Index Switch Node (partly); Image Info Node; Viewer Node; Node Parts; Editing Nodes (the Mute section).
- API: `NodeLink`; `NodeSocket` (the `links`, `identifier`, `is_linked`, `is_multi_input` entries); `Node` (`inputs`, `outputs`, `internal_links`); `NodeTree` (`links`); `CompositorNodeSwitch`; `CompositorNodeAlphaOver`; `CompositorNodeCurveRGB`; `ShaderNodeMix`.
- Release notes: the whole Compositor page for 5.2.

**Pictures opened:**
- `_images/compositing_socket-inspection.png` (from Compositor System)
- `_images/node-types_CompositorNodeAlphaOver.webp` (from Alpha Over Node)
- `_images/node-types_CompositorNodeCurveRGB.webp` (from RGB Curves Node)
- `_images/node-types_CompositorNodeSplit.webp` (from Split Node)

**Ruled out, and why:**
- **Cycles' own 5.2 files.** I searched the whole `cycles_5.2` folder for `compositor`. The only file that mentions it is `ui.py`, and only to place two panels in the render properties (`CYCLES_RENDER_PT_performance_compositor` and `CYCLES_RENDER_PT_performance_compositor_denoise_settings`, at lines 842 and 847, imported at line 14 from `bl_ui.properties_render`). Cycles defines no compositor nodes, sockets or links, so it has nothing to say about following an image through a node tree.
- **The requirements page.** It does not name 5.2, so by its own terms it cannot speak for it.
- **Cryptomatte** (`manual_5.2/compositing/types/mask/cryptomatte.txt`, found by the `metadata` search). It identifies objects and materials within a rendered image, not which node input an image came from. Not an answer to this question.
- **The keying section** (found by the `identify` search). It concerns identifying content to be removed from an image, not tracing an image through nodes.
- **Menu Switch and Index Switch.** They select among an arbitrary number of inputs rather than exactly two, so they fall outside "a node that takes two pictures in". I read them to be sure they said nothing extra about the two-input case, and they did not.

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

**Six of these seven have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above. Test 1 was carried as far as a script can carry it, on both machines and with a window and without: no property on a socket reports any of what the inspection panel is said to show. Three more were run on the Creator's Mac on 2026-08-07 and are in *What a running Blender showed on the Creator's Mac*: the graph read the same with Blender's window open as with none, the Viewer node's image filled only when there was a window, and a picture rendered with a window matched one rendered with none exactly. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **What the socket inspection panel draws when a socket is hovered.** A script cannot hover a socket, so this stays untested. What is settled is that the data behind it does not become reachable once a window exists.
- **Ten of the fourteen tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run, and every answer here leaves Windows open.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Does socket inspection name where an image came from?**
   Open the Compositor, enable Use Nodes, connect Render Layers → RGB Curves → Composite, render once so the tree is evaluated, then hover the output socket of the RGB Curves node.
   *Negative result:* the tooltip shows any field naming the source node, the source scene, the render layer, or a file name — i.e. more than Resolution, Transform, Sampling, Precision and Type.
   *Positive result (matching this finding):* it shows only those five.

2. **Does a Mix node at Factor 0 give back exactly one of its two inputs?**
   Build two visibly different images (any two image sources set to different, uniform colours) into a Mix node, set Factor to 0, connect to a Viewer, and sample the pixel values in the Image Editor. Repeat at Factor 1.
   *Negative result:* at Factor 0 the sampled values match neither input exactly, or match the input the opposite way round from expectation.
   *Positive result:* at 0 the values equal one input exactly, at 1 the other.

3. **Does Alpha Over at Factor 0 give back the Background untouched?**
   Feed two different images into Alpha Over, set Factor to 0, view and sample.
   *Negative result:* the sampled pixels differ from the Background image anywhere.

4. **Does muting a two-input node make a specific one of the two pass through, and does `internal_links` say which?**
   Put two visibly different images into an Alpha Over node, select it, press M to mute it, and look at the Viewer. Then in the Python console read `node.internal_links` for that node and print each link's `from_socket.identifier` and `to_socket.identifier`.
   *Negative result:* the viewed image is a blend of both, or `internal_links` is empty, or the socket it names is not the one visibly coming through.

5. **Can the whole path be walked in a script with no window open?**
   Save a file with Render Layers → RGB Curves → Alpha Over → Composite. Run Blender with `--background` and a script that walks forward from the Render Layers node's Image socket using `socket.links` and `link.to_node`, printing each node it reaches.
   *Negative result:* the script errors, or reports no links, or stops before reaching the Composite node.

6. **Do the two `Image` input sockets of Alpha Over have distinct identifiers?**
   With an Alpha Over node selected, print `[(s.name, s.identifier) for s in node.inputs]` in the Python console.
   *Negative result:* the two image inputs share an identifier, or the identifier is an empty string, which would mean they cannot be told apart in a script.

7. **Does the Switch node's output match one input's pixel values exactly?**
   Feed two different images into a compositor Switch node, view the output with the switch off, then on, sampling pixel values each time against the two inputs.
   *Negative result:* either output differs from the corresponding input at any pixel, which would contradict "without any modifications".

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 5.2

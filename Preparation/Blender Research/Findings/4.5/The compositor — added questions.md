# The compositor — added questions (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, the Blender 4.5 release notes, and Cycles' own files — all as supplied in `Blender Sources/`, all read on **2026-07-30**. Two pictures from the manual's `_images` folder, same date. Blender's requirements page as supplied, read **2026-08-01**. No web access was used, and none was needed.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

### The question is ambiguous, and I say which reading I answered

"Followed forward" can mean two different things, and the answer differs between them:

- **Reading A (the one I answer):** following the picture along the wiring — can you tell, from the node tree itself, that the picture arriving at a node continues out of that node, and out of the far side of a node that takes two pictures in.
- **Reading B (I answer it too, more briefly):** following the picture by its pixels — can you recognise the picture again after it has been through those nodes, by looking at what comes out.

I answer Reading A as the main answer, because the question's second half — "can it be told which of the two came out the far side" — is a question about the node, not about pixel-matching. Reading B is covered under "If the question means recognising the pixels" below.

### LABEL

Mixed, and I split it rather than round it off:

- **documented** — that the three colour-changing nodes quoted below (Brightness/Contrast, Hue/Saturation/Value, RGB Curves) each take one picture in and give one picture out; that a node tree's links and sockets are readable as data, node by node and socket by socket; that the **Switch** node passes one named input straight to its output and which one is set by a value that can be read; that the **Z Combine** node chooses between its two inputs pixel by pixel on depth; that a node's output takes its **domain** — where a picture sits and how big it is — from a designated one of its inputs, by a stated rule.
- **not in the sources** — that a picture keeps any identity, name or tag of its own as it travels; and, for the **Mix Color** and **Alpha Over** nodes, which of the two inputs "came out" — the sources describe both as blending both inputs and never state that either input emerges intact at any setting.
- **inferred**, with the step written out where each one is made below: that the same one-in-one-out shape holds for the rest of the Color ‣ Adjust group; that the links and sockets quoted add up to a chain that can be followed from one node to the next; that the two identically-labelled Alpha Over inputs can be told apart by their identifiers; that a picture carries no name or origin marker at all; that a differing domain tells you which input the output took its framing from; and that the reading-the-wiring half holds with no window open.

### Part 1 — through a node that changes its colours

**Yes, by the wiring, and the sources are explicit that these nodes are one picture in, one picture out.**

The colour-changing nodes are gathered in the manual under Color ‣ Adjust: Brightness/Contrast, Color Balance, Color Correction, Exposure, Gamma, Hue Correct, Hue/Saturation/Value, RGB Curves, Tone Map.

> "Inputs
>
> Image
>
> Standard color input.
> […]
> Outputs
>
> Image
>
> Standard color output."

— Blender 4.5 manual, Brightness/Contrast Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/adjust/bright_contrast.html — read 2026-07-30

The same shape appears on the Hue/Saturation/Value node, which also has a Factor:

> "Factor
>
> The amount of influence the node exerts on the image."

and

> "Image/Color
>
> Standard color output."

— Blender 4.5 manual, Hue/Saturation/Value Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/adjust/hue_saturation.html — read 2026-07-30

RGB Curves is worded the same way:

> "Factor
>
> Controls the amount of influence the node exerts on the image."

— Blender 4.5 manual, RGB Curves Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/adjust/rgb_curves.html — read 2026-07-30

So for each of the three nodes quoted above there is exactly one picture output, and there is no branch to choose between. Following the picture forward is following the single link out of that socket.

**Inferred, and here is the step.** Three of the nine nodes in that group are quoted. That the other six have the same one-in-one-out shape is not quoted anywhere here — the reader has to supply "the group is uniform". That is inference, not reading.

**The wiring itself is readable from outside the interface.** The API documents links as objects with both ends named:

> "Link between nodes in a node tree"

with the properties `from_node`, `from_socket`, `to_node`, `to_socket`, each

> "Type:
>
> Node, (readonly)"

/ "NodeSocket, (readonly)"

— Blender 4.5 Python API, NodeLink, https://docs.blender.org/api/4.5/bpy.types.NodeLink.html — read 2026-07-30

And a socket knows its own links and has a stable name of its own:

> "links
>
> List of node links from or to this socket."

> "identifier
>
> Unique identifier for mapping sockets"

> "is_linked
>
> True if the socket is connected"

— Blender 4.5 Python API, NodeSocket, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30

A node's sockets are listed in `inputs` and `outputs` (Blender 4.5 Python API, Node, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30).

**Inferred, and here is the step.** Each of those three pages is quoted on its own above. That they join up — that from an output socket you can reach the links leaving it, and from each link the node and socket it arrives at, and so on down the tree — is the reader putting the three together. No quoted sentence says it.

**One thing the sources do not say:** that setting a colour node's Factor to 0 leaves the picture untouched. The word used is "influence" and "amount of influence"; no page I found states what value means "none". The only statement in that neighbourhood is about *unconnected* single-value inputs, and is about defaults, not about the Factor extremes:

> "The default values are those that are considered identity and thus have no effect on the output"

— Blender 4.5 manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30

**There is a documented way to make a colour node pass the picture through untouched**, which is muting it:

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

— Blender 4.5 manual, Editing (Nodes), https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30

and the mapping a muted node uses is itself exposed:

> "internal_links
>
> Internal input-to-output connections for muting"

— Blender 4.5 Python API, Node, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30

That is worth noting for the two-input case below: a muted two-input node's `internal_links` state, in the file's own data, which input is being carried to which output.

### Part 2 — through a node that takes two pictures in and gives one out

There is no single answer, because the compositor has several such nodes and they behave differently. Taking them one at a time:

**Switch node — yes, and which input came out is stated outright.**

> "Switch between two images using a checkbox."

> "Switch
>
> When it is unchecked, the first input labeled “Off” is passed to the output.
>
> When checked, the second input labeled “On” is passed to the output."

> "Tip
>
> Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

— Blender 4.5 manual, Switch Node, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch.html — read 2026-07-30

The manual's picture of this node shows a node headed "Switch", one output socket labelled "Image", a checkbox labelled "Switch", and two input sockets labelled "Off" and "On" — matching the text. (Picture `_images/node-types_CompositorNodeSwitch.webp`, from that same manual page; label **documented**; read 2026-07-30.)

The setting is readable from outside as well:

> "check
>
> Off: first socket, On: second socket. (Deprecated: Use Switch input instead.)"

— Blender 4.5 Python API, CompositorNodeSwitch, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeSwitch.html — read 2026-07-30

The "use Switch input instead" note lines up with a change recorded for this version:

> "Most nodes now expose their options as single value inputs."

— Blender 4.5 release notes, Compositor, https://developer.blender.org/docs/release_notes/4.5/compositor/ — read 2026-07-30

**Caveat on Switch, and it matters.** The release note says the option is now an input socket. Whether that socket can itself be fed by a link is not stated in anything quoted here, and neither is what would happen if it were — whether the choice could then vary, or vary from pixel to pixel. Where the choice is a stored value, that value tells you which input came out. Beyond that the sources do not settle it. This is a genuine gap, not a rounding-off.

**Mix Color node — both inputs contribute; the sources never say either one comes out.**

> "The Mix Node mixes values, colors and vectors inputs using a factor to control the amount of interpolation."

> "Factor
>
> Controls the amount of mixing between the A and B inputs.
>
> A/B
>
> The two inputs that are mixed together."

> "Result
>
> Output the result of the mix using the data type selected."

— Blender 4.5 manual, Mix Color Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/mix_color.html — read 2026-07-30

I looked for a statement that a factor of 0 gives back A untouched, or that 1 gives back B untouched, and did not find one — not on that page, not on the API page for the node ("Mix values by a factor", Blender 4.5 Python API, ShaderNodeMix, https://docs.blender.org/api/4.5/bpy.types.ShaderNodeMix.html — read 2026-07-30), not in the enumeration of blend modes, which lists "MIX: Mix." and nothing more (Blender 4.5 Python API, Ramp Blend Items, https://docs.blender.org/api/4.5/bpy_types_enum_items/ramp_blend_items.html — read 2026-07-30). The manual points elsewhere for the blend maths and leaves Blender's own documentation:

> "See Color Blend Modes for details on each blending mode."

— same Mix Color page — and the glossary entry it leads to says only "Methods for blending two colors together" and then refers the reader to another program's documentation (Blender 4.5 manual, Glossary, https://docs.blender.org/manual/en/4.5/glossary/index.html — read 2026-07-30). I did not follow that off-site reference; it is outside the sources I was given.

So for Mix Color: **which of the two came out the far side is not in the sources.** Both went in, one blended thing came out.

**Alpha Over node — same: a blend, and the sources say so.**

> "The Alpha Over node is used to layer an image on top of another with alpha blending."

> "Image
>
> The background image.
>
> Image
>
> The foreground image."

> "Image
>
> The blended result."

— Blender 4.5 manual, Alpha Over Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/alpha_over.html — read 2026-07-30

The manual's picture of this node shows, reading down: one output "Image", then a "Fac" slider showing 1.000, then the two picture inputs both labelled just "Image", then a "Straight Alpha" checkbox. Which of the two identically-labelled sockets is the background is not shown on the node face; it is the text above that says the first is background and the second foreground. (Picture `_images/node-types_CompositorNodeAlphaOver.webp`, from that same manual page; label **documented**; read 2026-07-30.) *[Changed by the citation check, 2026-08-06: this sentence said the two Image inputs sat "above" the Fac slider. They sit below it. Corrected against the picture.]* **Inferred, and here is the step.** The API says a socket has a "Unique identifier for mapping sockets" (NodeSocket, cited above). That the two identically-labelled Image sockets on this node therefore carry different identifiers, and can be told apart that way, is a step the reader supplies. No quoted sentence says it of this node.

For this version the release notes also record a change to this node:

> "The Premultiplied option in the Alpha Over node was removed. The same functionality can be achieved using Convert Alpha and a Mix Color nodes."

— Blender 4.5 release notes, Compositor, https://developer.blender.org/docs/release_notes/4.5/compositor/ — read 2026-07-30

**Z Combine node — which one came out is decided pixel by pixel, and the rule is documented.**

> "The Z Combine node combines two images based on their Z-depth maps. It overlays the images using the provided Z values to detect which parts of one image are in front of the other."

> "Image
>
> If both Z values are equal, it will use the foreground image. Whichever Z value is less decides which image pixel is used."

— Blender 4.5 manual, Z Combine Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/z_combine.html — read 2026-07-30

So here one of the two inputs really does come out at each pixel — but which one varies across the picture and depends on the depth pictures fed in, so it cannot be settled by looking at the node alone.

**Two nodes that look like candidates and are not.** The Split node takes two pictures but gives nothing out — "This node has no output sockets" (Blender 4.5 manual, Split Node, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/split.html). The Switch View node takes a left and a right picture and gives "Stereo 3D image output" — both go out, combined, not one chosen (Blender 4.5 manual, Switch View Node, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch_stereo_view.html). Both read 2026-07-30.

**How this list was arrived at, which bears on how far it goes.** The nodes above are the ones I found by name and opened. I did not work through a full list of the compositor's nodes checking each one's inputs, and nothing in my record of looking below is such a sweep. So read this as the two-input nodes I reached, not as all of them.

### Part 3 — one thing that does survive a two-input node, and is documented

A picture in the 4.5 compositor is not only its pixels. It also has a *domain* — where it sits and how big it is:

> "The compositor is designed in such a way as to allow compositing in an infinite compositing space. Consequently, images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations."

And which input's domain comes out the far side of a node is settled by a stated rule:

> "One of the inputs of the nodes is designated as the Domain Input of the node, and the operation domain of the node is identical to the domain of that designated input."

> "Each input in the node has the so called Domain Priority property, the operation domain of the node is the domain of the non single value input with the highest domain priority."

The manual then names it for the two nodes in question:

> "For instance, if the Foreground input is bigger than the Background input in the Alpha Over node, the output will be clipped to the Background input because it is the domain input"

> "one can use a Mix node to achieved the desired behavior, noting that the first Image input in the Mix node has the highest domain priority"

— all from Blender 4.5 manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30

And that property of a picture can be read back inside the node tree:

> "The Image Info node outputs spatial and transformation information about an image in the compositor."

with outputs "Dimensions", "Resolution", "Location", "Rotation", "Scale".

— Blender 4.5 manual, Image Info Node, https://docs.blender.org/manual/en/4.5/compositing/types/input/image_info.html — read 2026-07-30

This node is new in this version: "A new **Image Info** node was added to provide information like dimensions, resolution, and image transformations." (Blender 4.5 release notes, Compositor, https://developer.blender.org/docs/release_notes/4.5/compositor/ — read 2026-07-30).

**Inferred, and here is the step.** The sources say the output's domain is the domain input's domain, and they say Image Info reports a picture's dimensions, location, rotation and scale. From those two together: if the two pictures entering a Mix Color or an Alpha Over node have *different* domains, then reading the output's domain tells you which of the two the output inherited its framing from. The reader has to supply the step "different domains therefore distinguishable", so this is inference, not something the sources state. It also settles only the framing, not the pixels — the colours coming out are still a blend of both.

### If the question means recognising the pixels (Reading B)

**Not in the sources, with one adjacent documented technique.**

The compositor's data model is documented, and it holds no name, tag or origin — only numbers:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats:" — Float, Vector, Color, where Color is "A 4D vector storing the Red, Green, Blue, and Alpha of the color."

— Blender 4.5 manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30

> "The Compositor processes one image at a time, no matter what kind of input you provide."

— Blender 4.5 manual, Introduction (Compositing), https://docs.blender.org/manual/en/4.5/compositing/introduction.html — read 2026-07-30

**Inferred, step written out:** since the documented data travelling between nodes is pixel values (plus a domain), and nothing in the documented data is a name or origin marker, a picture carries nothing that would let a later node say where it came from. The reader has to supply "the documented list is complete", so this is inference. I searched for a contrary mechanism and found none — see the record below.

The one documented technique for telling a picture apart later is a manual worked example, and it is put there deliberately by the user, not carried by Blender:

> "Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

> "When you see an image that you think might be yours, use the node tree below to compare it to your stock image (pre-watermarked original). In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference."

— Blender 4.5 manual, Mix Color Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/mix_color.html — read 2026-07-30

Two things about that. It is a comparison against a kept original, not a mark the picture carries. And the node it names has been deprecated in this version: "The Map Value node was deprecated and will be removed in v5.0. The same functionality can be achieved using the Map Range and Clamp nodes." (Blender 4.5 release notes, Compositor, https://developer.blender.org/docs/release_notes/4.5/compositor/ — read 2026-07-30). The sources do not say whether a watermark survives an arbitrary colour change.

I also checked Cryptomatte, since it is the compositor's identity feature. It identifies things *inside* a render, not the render itself: "The Cryptomatte node uses a Cryptomatte image to create a mask for one or more objects or materials." (Blender 4.5 manual, Cryptomatte Node, https://docs.blender.org/manual/en/4.5/compositing/types/mask/cryptomatte.html — read 2026-07-30). It does not answer this question.

### Looking at the picture as it goes — documented, but through the interface

Two documented ways to see the picture at any point in the tree:

> "The Viewer node is used to preview image data or value maps anywhere within a compositor node graph. It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."

> "Only the active Viewer node (with the icon in the header) will be displayed in the backdrop or Image Editor."

— Blender 4.5 manual, Viewer Node, https://docs.blender.org/manual/en/4.5/compositing/types/output/viewer.html — read 2026-07-30

> "Toggle Node Preview Shift-H
>
> Shows/Hides a preview region on the node that displays the frame after that node’s operation has been applied."

— Blender 4.5 manual, Editing (Nodes), https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30

Both are ways of following a picture forward by eye. Both are described in terms of the interface — a backdrop, an Image Editor, a preview region on the node.

### What a running Blender showed

**LABEL: observed.** Everything in this section was got by running Blender 4.5.12 LTS on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, and each Blender was pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics' tests while this ran, so nothing here reports how long anything took. The full procedure, every script in full, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only. The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**A control first (entry 1).** A picture put through a bare tree and written out came back identical to the picture that went in, and the same render twice gave the same picture. Blender's own words:

> `MAXDIFF render vs source 0.0`
> `MAXDIFF render vs same render again 0.0`

So every comparison below is a comparison of pictures, not of files, and a difference of zero means the pixels match.

**Which nodes take two pictures in, and what their sockets are called (entry 2).** Sweeping every node type this Blender will add to a compositor tree and keeping those with two or more picture inputs and at least one picture output:

> `ADDABLE NODE TYPE COUNT 133`
> `TWO-PICTURE-IN, ONE-PICTURE-OUT COUNT 23`

The twenty-three are Alpha Over, Bilateral Blur, Bokeh Blur, Chroma Key, Color Balance, Color Key, Color Spill, Cryptomatte (Legacy), RGB Curves, Denoise, Difference Key, Distance Key, Glare, Keying, Mix (`CompositorNodeMixRGB`), Split, Switch, Switch View, Z Combine, Mix (`ShaderNodeMix`), Mix (Legacy), Brick Texture and Checker Texture. So the list in this answer — Switch, Mix Color, Alpha Over, Z Combine — is four of twenty-three.

The sockets of the nodes named above:

> `PRESENT CompositorNodeAlphaOver label: Alpha Over`
> `   IN  [('Fac', 'Fac', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA'), ('Straight Alpha', 'Straight Alpha', 'BOOLEAN')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

so the two identically named picture inputs do carry different identifiers, which the answer above had marked inferred.

> `PRESENT CompositorNodeSwitch label: Switch`
> `   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

so the Switch node's choice is an input socket in 4.5, not a property on the node.

> `PRESENT CompositorNodeSplit label: Split`
> `   IN  [('Factor', 'Factor', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

The Split node has one picture output. The 4.5 manual page quoted above says "This node has no output sockets", and the running 4.5 disagrees with it.

Two nodes the answer never mentions came back absent, so nothing in this version can lean on them:

> `ABSENT CompositorNodeIndexSwitch -> RuntimeError Error: Node type CompositorNodeIndexSwitch undefined`
> `ABSENT CompositorNodeMenuSwitch -> RuntimeError Error: Node type CompositorNodeMenuSwitch undefined`

**What comes out of a Mix at the ends of its factor (entry 3).** Both of 4.5's Mix nodes, with one picture on the first input and a different one on the second:

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `CompositorNodeMixRGB factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `CompositorNodeMixRGB factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

**Alpha Over at the ends of its factor, and the Switch node (entry 4).**

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

The foreground picture used was opaque throughout, which is why factor 1 gives it back whole.

**What the file says about which input reaches the output (entry 5).** Every two-picture node's `internal_links` names one picture input, muted or not:

> `=== AlphaOver (CompositorNodeAlphaOver) ===`
> `  unmuted internal_links: [('Image', 'Image')]`
> `  muted   internal_links: [('Image', 'Image')]`
> `  muted   is_inactive: [('Fac', 'Fac', True), ('Image', 'Image', False), ('Image', 'Image_001', True), ('Straight Alpha', 'Straight Alpha', True)]`
> `=== Switch (CompositorNodeSwitch) ===`
> `  unmuted internal_links: [('Off', 'Image')]`
> `=== Z/Depth Combine (CompositorNodeZcombine) ===`
> `  unmuted internal_links: [('Image', 'Image'), ('Z_001', 'Z')]`

So in 4.5 `is_inactive` does change when a node is muted, and marks the second picture input as having no effect.

Sweeping every property name on a socket and on the Alpha Over node for anything naming a priority or a domain:

> `  ANY NAMING PRIORITY OR DOMAIN: []`

**The Switch node's choice fed by a link (entry 6).**

> `LINK FROM VALUE NODE ACCEPTED, is_valid: True`
> `LINK FROM A PICTURE ACCEPTED, is_valid: True`
> `links now on the Switch input: [('CompositorNodeImage', 'Image', True)]`
> `WITH A PICTURE ON THE SWITCH INPUT: maxdiff vs picA=0.0  maxdiff vs picB=1.0`

The picture fed in ran from 0 on the left of the frame to 1 on the right. The whole frame still came out as the Off input.

**What a muted node passes, rendered (entry 7).**

> `MUTED AlphaOver            internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Mix                  internal_links=[('A_Color', 'Result_Color')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Switch               internal_links=[('Off', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Split                internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Z/DepthCombine       internal_links=[('Image', 'Image'), ('Z_001', 'Z')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `COLOUR NODE HueSat     muted=False maxdiff vs the picture that went in = 1.0`
> `COLOUR NODE HueSat     muted=True  maxdiff vs the picture that went in = 0.0`
> `COLOUR NODE RGBCurves  muted=False maxdiff vs the picture that went in = 0.5`
> `COLOUR NODE RGBCurves  muted=True  maxdiff vs the picture that went in = 0.0`

**The Split node's two sides, and Z Combine pixel by pixel (entry 8).** With the first picture on the first input:

> `   SPLIT left column x=5 y=32 -> [0.1, 0.92063, 0.8, 1.0]   (picA there [0.07937, 0.50794, 0.25, 1.0], picB there [0.1, 0.92063, 0.8, 1.0])`
> `   SPLIT right column x=58 y=32 -> [0.92063, 0.50794, 0.25, 1.0]   (picA there [0.92063, 0.50794, 0.25, 1.0], picB there [0.1, 0.07937, 0.8, 1.0])`

so in 4.5 the first input lands on the right of the frame and the second on the left.

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

*[Note added 2026-08-07 by entry 20, below. This entry's conclusion for 4.5 is right, and entry 20 confirms it by reading the frame straight off the socket instead of inferring it from the alpha. Two cautions belong with it even so. The Factor was never written down here, and it matters: a Mix node starts at Factor 0.5 in this version and at 1.0 in 5.2, which is what made 5.2 look unlike the rest. And the area whose alpha is above zero is a reading of the alpha, not of the frame; the two happen to agree in this version at this Factor, and they do not always. Entry 20 is what to read.]*

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

A File Output node whose input socket was named `MyBranchName`:

> `FILE OUTPUT input sockets: [('Image', 'Image'), ('MyBranchName', 'MyBranchName')]`
> `WHAT LANDED IN THE FILE OUTPUT FOLDER: ['MyBranchName0001.png']`

**Reading the whole graph out of a saved file with no window (entry 11).** The tree was built and saved by one Blender and read back by a second, separate one, with no window at either end:

> `TREE bpy.data.scenes['Scene'].node_tree`
> `NODES [('Chooser', 'CompositorNodeSwitch'), ('Colour', 'CompositorNodeHueSat'), ('Composite', 'CompositorNodeComposite'), ('Image', 'CompositorNodeImage'), ('Image.001', 'CompositorNodeImage'), ('TwoIn', 'CompositorNodeAlphaOver')]`
> `LINK Image.Image -> Colour.Image   muted=False valid=True`
> `LINK Colour.Image -> TwoIn.Image   muted=False valid=True`
> `LINK Image.001.Image -> TwoIn.Image_001   muted=False valid=True`
> `LINK Image.Image -> Chooser.Off   muted=False valid=True`
> `LINK Image.001.Image -> Chooser.On   muted=False valid=True`
> `LINK TwoIn.Image -> Composite.Image   muted=False valid=True`
> `CHOOSER internal_links: [('Off', 'Image')]`
> `TWOIN inputs: [('Fac', 'Fac', False, False), ('Image', 'Image', True, False), ('Image', 'Image_001', True, False), ('Straight Alpha', 'Straight Alpha', False, False)]`

**The processor against the card, and whether a socket reports its picture (entry 12).** Every run on this machine printed these lines first, and they bear on how the device half is read:

> `libEGL warning: egl: failed to create dri2 screen`
> `EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).`

then:

> `compositor_device present: True value: CPU`
> `compositor_precision present: True value: AUTO`
> `MAXDIFF between the processor's picture and the card's picture: 6.556510925292969e-07`

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
> `HEAVY RENDER SECONDS CPU 3.679`
> `HEAVY PROCESSOR CORE-SECONDS CPU 61.060`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 67`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`

Set to the card:

> `ASKED FOR DEVICE GPU`
> `compositor_device now reads: GPU`
> `HEAVY RENDER SECONDS GPU 0.707`
> `HEAVY PROCESSOR CORE-SECONDS GPU 1.760`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 31`
> `TIMES THIS PROCESS WAS ON THE CARD: 9`
> `MOST CARD MEMORY THIS PROCESS HELD: 214 MiB`

Only the setting differed between those two runs, and three things move together. Set to the processor, this Blender never appeared on the card at all. Set to the card, it appeared on it holding 214 MiB, and its own share of processor work fell from sixty-one core-seconds to under two.

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

**Does a faint mark survive a colour change (entry 13).** A copy of one picture with a 16 by 16 block at its centre raised by 0.02, put through a Hue/Saturation/Value node with the hue moved to 0.8 and the saturation to 1.7:

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

**LABEL: observed.** Everything in this sub-section was got by running Blender 4.5.12 LTS on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB with its kernel driver and its driver library both at 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all**, reached at `ssh -i ~/.ssh/id_ed25519 -p 39348 root@85.218.235.6` — on **2026-08-07**. Every run used `-b --factory-startup` and was pointed at a settings folder of its own, made beforehand under `/root/again-compositor/`, which is a third working folder outside both earlier sittings and outside every other topic's work. The Blender named itself with the same build hash as on 2026-08-06, so this is the same build. The machine was shared with other topics while this ran, so nothing here reports how long anything took or how much of the card was in use, and nothing here needed either. **Pictures were compared as pictures, never as files.** The full procedure, every script, and the whole of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 20, 21 and 22. This settles Linux alone; none of the three has been run on the Creator's Mac.

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

The same twelve lines came back from 5.0. 5.1 and 5.2 gave the same sizes except at Factor 1.0 with the large picture on A, where they gave 32 by 32; and the pixel outside the small picture came back with a different alpha in them, which is the next point below.

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

> `ADDABLE NODE TYPE COUNT 133`
> `SWITCH IN THE TYPE NAME ['CompositorNodeSwitch', 'CompositorNodeSwitchView']`
> `THE OLDER, WIDER RULE — 'Switch' or 'Index' or 'Menu' in the type name: ['CompositorNodeSwitch', 'CompositorNodeSwitchView']`
> `NAMES THE WIDER RULE ADDS THAT HAVE NO 'Switch' IN THEM: []`

The two rules agree in this version, and the addable count is entry 2's count to the number. Set against the other three: `GeometryNodeMenuSwitch` arrives in 5.0, `GeometryNodeIndexSwitch` in 5.1, and `GeometryNodeSwitch` and `FunctionNodeInputMenu` in 5.2. None of them is addable here.

**`is_inactive` on a Mix node with both picture inputs linked (entry 22).** A `ShaderNodeMix` set to `RGBA` and `MIX`, with a different picture linked to each of its two colour inputs and the tree's output linked up, printing `(name, identifier, is_inactive)` for every input socket, unmuted and then muted, **before any render and again after one**. Entry 5 above records this line for 5.2 only, and the citation check had nothing to set beside it.

**What a negative result would have looked like.** The 5.2 reading could have held at `True` across every Factor, which would make it a real difference between the versions and leave entry 5's line standing as written. It could have failed to reproduce at all. The reading could have changed after a render, which would mean both earlier runs read it at the wrong moment. And the Factor sweep could have shown no movement, which would leave the 5.2 line unexplained.

The Factor the node held, never set by the script, in this version:

> `A_Color is_linked: True   B_Color is_linked: True   Factor_Float held: 0.5`

and the line itself, word for word — identical before a render and after one, so the pair is printed once:

> `  unmuted is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', False), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`
> `  muted   is_inactive: [('Factor', 'Factor_Float', True), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', False), ('B', 'B_Color', True), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]`

This is the `False` that entry 5's 5.2 line was set against, and it stands. Muting changes the line in this version — the Factor and the B input turn true and the A input stays false, the surviving input being the one that passes, which agrees with `internal_links` and with the renders of entries 5 and 7. 5.0, 5.1 and 5.2 do not move on muting at all.

**The Factor sweep, which is the control on all of it.** Word for word, and **identical in all four versions**:

> `FACTOR 0.0  A_Color is_inactive=False  B_Color is_inactive=True  (both linked: True, True)`
> `FACTOR 0.25  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 0.5  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 0.75  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)`
> `FACTOR 1.0  A_Color is_inactive=True  B_Color is_inactive=False  (both linked: True, True)`

So `is_inactive` on this node's two colour inputs behaves the same way in all four versions: A greys out at Factor 1.0, B at Factor 0.0, neither in between. Entry 5's 5.2-only line is the starting Factor of 1.0 showing through, the same fault as in entry 9 in a different place. And **what `is_inactive` reports is a Factor's arithmetic, not a wiring fact** — a socket with a picture linked to it still reads inactive when the Factor makes its contribution nil. So it cannot on its own say which picture reached the far side. `internal_links` and the renders of entries 5 and 7 are what do that, and they are unaffected.

### What a running Blender showed on the Creator's Mac

**LABEL: observed.** Everything in this section was got by running Blender 4.5.12 LTS on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**. That Blender says of itself:

> `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)`

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-4.5`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`, entries 15 to 18.

**The graph reads the same with a window open as with none (entry 15).** One Blender built and saved the tree with no window; a second opened that file with no window and printed it; a third opened the same file with its window open and printed it. The run with a window, word for word:

> `VERSION 4.5.12 LTS`
> `FILE /Users/artist/Documents/Smoozy hand tests/The compositor/graph_4_5.blend`
> `TREE bpy.data.scenes['Scene'].node_tree`
> `NODES [('Chooser', 'CompositorNodeSwitch'), ('Colour', 'CompositorNodeHueSat'), ('Composite', 'CompositorNodeComposite'), ('Image', 'CompositorNodeImage'), ('Image.001', 'CompositorNodeImage'), ('TwoIn', 'CompositorNodeAlphaOver')]`
> `LINK Image.Image -> Colour.Image   muted=False valid=True`
> `LINK Colour.Image -> TwoIn.Image   muted=False valid=True`
> `LINK Image.001.Image -> TwoIn.Image_001   muted=False valid=True`
> `LINK Image.Image -> Chooser.Off   muted=False valid=True`
> `LINK Image.001.Image -> Chooser.On   muted=False valid=True`
> `LINK TwoIn.Image -> Composite.Image   muted=False valid=True`
> `CHOOSER stored choice: [('Switch', 'Switch', False), ('Off', 'Off', bpy.data.scenes['Scene'].node_tree.nodes["Chooser"].inputs[1].default_value), ('On', 'On', bpy.data.scenes['Scene'].node_tree.nodes["Chooser"].inputs[2].default_value)]`
> `CHOOSER internal_links: [('Off', 'Image')]`
> `TWOIN inputs: [('Fac', 'Fac', False, False), ('Image', 'Image', True, False), ('Image', 'Image_001', True, False), ('Straight Alpha', 'Straight Alpha', False, False)]`
> `TWOIN internal_links: [('Image', 'Image')]`

The Mac's run with no window printed those same lines, character for character. So on the Mac the two agree. **This test was also run on Linux** — entry 11 above — and the Linux run with no window printed the same lines too, apart from the file's own path. The two machines agree.

**The Viewer node's image fills when a window is open, and stays empty when there is none (entry 16).** The same script, one render of 64 by 64, run twice. With no window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`

With a window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; the windowed case has no Linux counterpart, because that machine has no display server.

**Every property name on a socket is the same with a window as without (entry 17).** The same list came back from both runs:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree, and none of the names is an inspection, a resolution, a transform, a precision or a sampling.

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

**One thing about ending a Blender that has a window.** The scripts were written to close Blender with `bpy.ops.wm.quit_blender()`. On this Mac that line on its own did not end it: the script's printing all appeared, and the process was still running when a limit of my own ended it after three minutes. The same script with `sys.exit(0)` added after that line ended in about a second and printed the same thing. Both are recorded in entry 19 of the hand-test list.

### Not covered

What this answer does not settle:

- What Mix Color and Alpha Over actually compute at the extremes of their Factor. The sources describe a blend and stop. Whether Factor 0 hands back one input bit-for-bit is unanswered here.
- ~~What the sources say about the Alpha Over node's own Factor.~~ **The sources do describe it.** The 4.5 Alpha Over page reads "Factor / The alpha of the foreground image, going from 0 (fully transparent) to 1 (fully opaque)." (Blender 4.5 manual, Alpha Over Node, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/alpha_over.html — read 2026-07-30). *[Changed by the citation check, 2026-08-06: this bullet said no description of the Factor was quotable from anywhere. It was on the node's own page.]*
- Which nodes in the compositor take two pictures in and give one out. I covered the ones I reached by name; I did not check every node.
- ~~What happens when the Switch node's Switch is fed by a link rather than left as a stored value.~~ **Settled by entry 6 above:** the socket accepts a link, from a value node and from a whole picture alike, and the frame still comes out entirely as the Off input. *[Changed by the citation check, 2026-08-06: this bullet was left standing after the test that answered it; a repeat of that test on 2026-08-07 came back the same.]*
- Whether any picture identity survives a *render*, i.e. once written to a file and read back. Only the in-tree case is answered.
- Whether Cryptomatte or any pass data survives a colour-changing node in usable form.
- Whether a watermark of the kind the manual describes survives an arbitrary colour change.
- Node groups. The compositor documentation mentions groups, and a picture entering a group crosses a boundary; I did not test how far the link-walking above carries across it.
- Anything about numerical precision. The stated storage is "half precision", which bears on pixel-comparison methods, but the sources say nothing about comparing pictures for equality.

### Outside the interface

**LABEL: observed for the wiring half, on Linux. Partly documented, partly inferred for the rest.**

**The wiring half was tested and holds with no window, and reads the same with one.** On the rented Linux machine described above, on 2026-08-06, Blender 4.5.12 LTS with `-b` opened a saved file and printed the whole chain — picture node, colour node, two-picture node, output node — with every link's node and socket at both ends, the Switch node's stored choice, and each node's internal links. That is entry 11 above, and it replaces the inference below for this half. On the Creator's Mac on 2026-08-07 the same file was read three ways — built with no window, read back with no window, and read back with the window open — and all three printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round: the wiring reads the same whether Blender has its window open or not.

**The looking-at-it half does not hold with no window, and does hold with one.** The image called `Viewer Node` exists as soon as a Viewer node is added, but stayed at 256 by 256 with every value zero across a 64 by 64 render with no window (entry 10 on Linux, and the same again on the Mac). With a window open on the Mac, that same image came back at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16). So the Viewer node's result is reachable when there is a window and not when there is none.

**What a script cannot reach either way.** Every property name on a socket was the same with a window as without, on both machines, and none of them names an inspection, a resolution, a transform, a precision or a sampling (entry 17). So nothing about a socket's own picture is readable from a script whether Blender has a window or not.

**The compositor can be put on the graphics card with no window at all.** On the same machine with its card repaired — driver 580.173.02 — on 2026-08-07, a Blender started with `-b` and its compositing device set to the card really did the work there: the card named that Blender's own process and it held 214 MiB while it worked, where the same run set to the processor never appeared on the card. So choosing the device is not an interface feature, and neither is acting on the choice (entry 12).

The reading from the sources, unchanged, follows.

The wiring half holds outside the interface. Links, sockets and node inputs and outputs are documented as ordinary readable data on the node tree, with no note anywhere on those pages that they exist only while a window is open (Blender 4.5 Python API, NodeLink / NodeSocket / Node, addresses cited above — read 2026-07-30). The Switch node's stored value is likewise a property of the node (Blender 4.5 Python API, CompositorNodeSwitch — read 2026-07-30). **Inferred**, step: these are documented as data-level properties of a node tree rather than as editor state, so reading them does not require the editor. The sources do not say this in so many words.

The looking-at-it half does not carry over. The Viewer node's result is shown "in the backdrop or Image Editor", and the node preview is a "preview region on the node" toggled by a shortcut in the node editor — all parts of Blender's own window (both cited above). **Inferred**, step: a feature described only as a display inside an editor is not available when there is no editor. The sources do not state this.

The sources do not say anything at all about the compositor's node behaviour differing when Blender is run in background mode. The manual's command-line page documents the switch itself —

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

— Blender 4.5 manual, Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

— and says nothing about compositor nodes there. Treat any claim beyond that as unanswered.

One documented condition worth carrying, because following a picture by looking at it needs the compositor to run at all:

> "The compositor searches for the active Composite node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn’t run altogether."

— Blender 4.5 manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

**LABEL: observed for Linux; still open for macOS and Windows.**

Every result in *What a running Blender showed* above was got on Linux — the rented machine named there — and settles Linux alone. The card half of entry 12 was run again on that same Linux machine on 2026-08-07, once its graphics card had been repaired and its driver stood at 580.173.02, and it settles Linux alone in exactly the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window (entry 11 on Linux, entry 15 on the Mac); the four Alpha Over and Switch renders (entry 4 on Linux, entry 18's no-window half on the Mac); the Viewer node's image staying empty with no window (entry 10 on Linux, entry 16's no-window half on the Mac); and every property name on a socket (entry 12 on Linux, entry 17 on the Mac). Nothing came out differently on the two operating systems.

**Ten of the fourteen Linux tests are still owed on the Mac**, and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding. The reading from the sources, unchanged, follows.

**Nothing in the sources ties any of the above to an operating system.** Every node page, and the Compositor System page, describe the behaviour without qualification. The API pages likewise carry no per-system notes on the properties quoted.

The one operating-system-specific statement I found anywhere in the compositing documentation is about speed, not about what any node does:

> "TDR on Windows
>
> Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit. This is especially common with older GPUs when using expensive nodes like Bokeh Blur node. To prevent unexpected resets, it is recommended to increase the TDR timeout."

— Blender 4.5 manual, Limitations (Compositing), https://docs.blender.org/manual/en/4.5/compositing/limitations.html — read 2026-07-30

For completeness on which systems are in scope, the requirements page names this version under all three system headings — "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer." under Windows and again under Linux, and "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2 (AMD, Apple Silicon, or Intel Skylake and newer)" under macOS (https://www.blender.org/download/requirements/ — read 2026-08-01). That page speaks for 4.5 only where it names it, as it does in those three lines.

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
## Where I looked

**Searched across the whole of `manual_4.5/` and `api_4.5/` (case-insensitive), for:** metadata, provenance, watermark, trace / "trace the image", track / "track the image", traverse, traversal, "node graph", downstream, upstream, "which input", "came from", "identify the image", "origin of the image", "no effect", unchanged, identity, "Blend Modes", "Color Blend Modes", mute, background mode, `--background`.

**Manual pages opened in full:** Compositing ‣ Introduction; Compositor System; Limitations; Switch Node; Split Node; Switch View Node; Mix Color Node; Alpha Over Node; Z Combine Node; Mix (index); Brightness/Contrast Node; Hue/Saturation/Value Node; RGB Curves Node (top); Color Balance Node (top); Adjust (index); Image Info Node; Viewer Node; Cryptomatte Node (top); Editing (Nodes), the Delete/Mute/Show-Hide section; Glossary, the Blend Modes entry; Command Line Arguments (searched, not read whole).

**Manual pictures opened:** `_images/node-types_CompositorNodeSwitch.webp` and `_images/node-types_CompositorNodeAlphaOver.webp`.

**API pages opened:** NodeLink; NodeSocket (the identifier, is_linked, is_multi_input, is_output and links entries); Node (the inputs, outputs, internal_links and mute entries); CompositorNodeSwitch; CompositorNodeMixRGB; CompositorNodeAlphaOver; CompositorNodeZcombine; ShaderNodeMix; Ramp Blend Items; Image (the pixels entry).

**Release notes:** the whole of `compositor.md` for 4.5; searched every 4.5 release-note file for switch, mix, "alpha over", "z combine".

**Cycles' own files:** searched all of `cycles_4.5/` for "compositor" (case-insensitive). The only hits are in one file, and they are performance panels, nothing about node behaviour or picture identity. Exactly:

> `from bl_ui.properties_render import draw_curves_settings, CompositorPerformanceButtonsPanel, CompositorDenoisePerformanceButtonsPanel`

— `Blender Sources/cycles_4.5/ui.py`, line 14

> `class CYCLES_RENDER_PT_performance_compositor(CyclesButtonsPanel, CompositorPerformanceButtonsPanel, Panel):`

— `Blender Sources/cycles_4.5/ui.py`, line 818

Label **observed**; read 2026-07-30. Ruled out as a source for this question: Cycles is a render engine and its files carry no compositor node definitions.

**Ruled out, and why:**

- Cryptomatte and ID Mask — they identify objects and materials within a picture, not the picture itself.
- The Split node — no output socket, so it is not a two-in one-out node.
- The Compositing ‣ Limitations page — hardware and speed only.
- The requirements page — used only for which operating systems 4.5 is named under; it says nothing about nodes.
- The off-site "Color Blend Modes" reference in the glossary — outside the sources I was given; not followed.

**Nothing was found for:** any per-picture name, tag, marker or origin field travelling between compositor nodes; any documented statement that a Mix Color or Alpha Over output equals one of its inputs at any factor setting; any documented account of how the compositor behaves differently when Blender is run without its window.

---

## Things a person at a computer could settle in a few minutes

**This list has been carried out.** Tests 1 to 8 were run on Linux on 2026-08-06 and their results are in *What a running Blender showed* above. Test 9, both halves, was run on the Creator's Mac on 2026-08-07 and its results are in *What a running Blender showed on the Creator's Mac*: the graph read the same with the window open as with none, and a picture rendered with a window matched one rendered with none exactly. Whether the Viewer node's image fills when a window is open was settled at the same time — it does. Two things are still open, both written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **Ten of the fourteen Linux tests, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** Test 10 below asks for it. No Windows machine has run it, so none of it has been run, and every answer here leaves Windows open.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Does Mix Color at Factor 0 hand back the first input untouched?**
   Open Blender 4.5, switch to the Compositing workspace and tick Use Nodes. Add two Image nodes loading two visibly different pictures. Add a Mix Color node (blend mode Mix), feed the first picture to A and the second to B, set Factor to 0. Add a Viewer node on the Mix output, and a second Viewer directly on the first picture. Flip between them in the Image Editor and sample the same pixel in each by clicking and holding on the image.
   *Positive:* the sampled values match exactly. *Negative:* the values differ, meaning the output is not the first picture untouched even at Factor 0.

2. **Same question for Factor 1 and the B input.** Identical setup, Factor 1, compare against the second picture. *Negative:* sampled values differ.

3. **Same question for Alpha Over at Factor 0.** Feed a picture to the first (background) Image input and a different one to the second (foreground), set Fac to 0, and compare the output against the background picture by sampling. *Positive:* they match. *Negative:* they differ, meaning the background does not pass through untouched.

4. **What happens when the Switch node's Switch input is linked?** In 4.5 the node's options are exposed as inputs. Add a Switch node, feed two different pictures to Off and On, and try to connect something to the Switch input — for example a Value or Boolean-producing node, then a full-picture input such as a black-and-white gradient. Watch the output in the Viewer.
   *Positive result of interest:* the choice varies across the picture, i.e. parts of both inputs appear at once. *Negative:* the socket refuses the link, or the output shows only one of the two pictures whole — in which case the choice stays a single decision for the whole frame.

5. **Which input's framing comes out of Alpha Over.** Load a large picture and a small one. Feed the small one to the first (background) Image input and the large one to the second (foreground). Put an Image Info node on the Alpha Over output and read its Resolution, using a Viewer on that value or the node's own preview.
   *Positive:* the resolution reported matches the small (background) picture, as the domain-priority rule predicts. *Negative:* it matches the large picture, or something else entirely.

6. **Same for Mix Color.** Same two pictures into A and B, Image Info on the Result, read Resolution. *Positive:* it matches whichever picture is in the A input. *Negative:* it matches B, or neither.

7. **Does a muted colour node pass the picture through bit-for-bit?** Put an RGB Curves node with a strong curve between a picture and a Viewer. Sample a pixel. Select the node and press M to mute it. Sample the same pixel again, and also sample the picture directly.
   *Positive:* muted output equals the picture directly. *Negative:* it differs, meaning muting is not an exact pass-through.

8. **Does the manual's watermark trick survive a colour change?** Build the manual's encode setup (mark, RGB to BW, Map Range to a tenth intensity, Mix set to Add). Then pass the result through a Hue/Saturation/Value node with a large Hue shift, and finally through the decode setup (Mix set to Difference against the kept original, amplified). Look at the result in the Viewer.
   *Positive:* the mark is still legible. *Negative:* the difference is a wash of noise with no readable mark, meaning a colour change destroys it.

9. **Do these answers hold with no window open?** Two parts, because one comparison on its own would not tell you which thing went wrong.
   *Part one, reading the wiring.* Save a file holding the setup from test 1, plus the Switch node from test 4. Run Blender from a terminal with `--background`, that file, and a short line of Python that prints, for the Mix Color node's output socket, every link leaving it with the node and socket each link arrives at, and prints the Switch node's stored value. Run the same line with the window open.
   *Positive:* the two printouts are the same. *Negative:* the background run prints nothing, stops with an error, or prints something different — meaning the wiring cannot be read the same way, or at all, without a window.
   *Part two, the pixels.* With the same file, put a Composite node on the Mix output and set an output path. Render one frame with `--background`. Then render the same frame with the window open, to the same file format and the same colour management settings. Compare the two written pictures pixel by pixel.
   *Positive:* the pixels match. *Negative:* they differ, meaning the compositor's result is not the same when driven from outside.
   Do not compare a written frame against a value read in the Image Editor. Those two are not read on the same terms, so a difference would not tell you whether the compositor behaved differently or whether writing the file changed the numbers.

10. **Do they hold on more than one operating system?** Carry out tests 1, 5 and 9 on a Windows machine and on a macOS machine with the same file and the same Blender 4.5 build lineage.
    *Positive:* identical sampled values and identical reported resolutions. *Negative:* any difference between the two, which would mean at least one of these behaviours is system-dependent.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 4.5

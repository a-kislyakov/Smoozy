# The compositor — added questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender's manual for 5.1, Blender's Python API documentation for 5.1, the 5.1 release notes, and Cycles' own files for 5.1 — all read on **2026-07-30** (the date the copies were taken). The pictures named below sit in the manual's `_images` folder and were read the same day. Blender's requirements page was not needed for this question and was not used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: only question 8 of the sheet is answered here. The rest of the sheet was left alone.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

### The reading I took, and the other one

The question is ambiguous, so I say plainly which reading I answered.

**The reading I answered:** can a picture be *traced* forward — that is, can you tell, from what the file holds, that a picture arriving at such a node is what the node's output carries onward, and for a two-in-one-out node, can you tell which of the two inputs the output came from.

**The other reading:** can a picture be *recognised* afterwards — can you look at the pixels that came out and say which picture they were made from. I say what the sources hold on this reading too, at the end, because one part of the manual speaks to it directly.

---

### Part A — through a node that changes its colours

**Label: documented.**

Every colour-changing node in the compositor is described as taking one picture in and giving one picture out. The tracing is therefore unbroken: there is one place a picture can enter and one place a result can leave.

Exact words, from the Hue/Saturation/Value Node page:

> "Inputs
>
> Image/Color
>
> Standard color input."

and

> "Outputs
>
> Image/Color
>
> Standard color output."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/hue_saturation.html`. Date read: 2026-07-30.

The same one-in-one-out shape is stated on the other adjust nodes. From the Brightness/Contrast Node page:

> "Inputs
>
> Image
>
> Standard color input."
>
> "Outputs
>
> Image
>
> Standard color output."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/bright_contrast.html`. Date read: 2026-07-30.

From the Color Balance Node page:

> "The Color Balance node adjusts the color and values of an image."

and its inputs begin:

> "Image
>
> Standard color input."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/color_balance.html`. Date read: 2026-07-30.

From the RGB Curves Node page:

> "The RGB Curves Node performs level adjustments on each color channel."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/rgb_curves.html`. Date read: 2026-07-30.

From the Exposure Node page:

> "The Exposure Node adjusts the brightness of an image using a camera exposure parameter."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/exposure.html`. Date read: 2026-07-30.

**What makes the following-forward possible at all.** Every connection between two nodes is recorded as a thing with both of its ends named, and both ends can be read. From the API documentation for a link between nodes:

> "Link between nodes in a node tree"

and its parts include:

> "from_node
>
> (readonly)
>
> Type:
>
> Node"

> "from_socket"
> "to_node"
> "to_socket"

Source: Blender Python API 5.1, `https://docs.blender.org/api/5.1/bpy.types.NodeLink.html`. Date read: 2026-07-30.

And from the socket's own page, the connections touching a single socket can be read directly:

> "links
>
> List of node links from or to this socket."

> "is_linked
>
> True if the socket is connected (default False, readonly)"

Source: Blender Python API 5.1, `https://docs.blender.org/api/5.1/bpy.types.NodeSocket.html`. Date read: 2026-07-30.

**Not covered (Part A).**
- The sources nowhere say that a colour-changing node leaves the picture recognisable, or leaves any part of it untouched. The opposite is its purpose.
- Several of these nodes have a Factor input described as "The amount of influence the node exerts on the image" (Hue/Saturation/Value Node page, address above, read 2026-07-30). The sources do **not** say that a Factor of zero passes the picture through unaltered. If you want that claim it is **inferred**, and the step you must supply is "no influence means no change", which the pages do not write down.
- One thing does make a colour node pass its input straight out, and it is documented, but it is a switch on the node rather than a value: *"Muting a node removes its contribution to the node tree, and makes all links pass through it without change."* (Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html`, read 2026-07-30.) The API documentation records both the muted state and the internal routing that muting uses: *"internal_links — Internal input-to-output connections for muting (default None, readonly)"* and *"mute — (default False)"* (Blender Python API 5.1, `https://docs.blender.org/api/5.1/bpy.types.Node.html`, read 2026-07-30). So a colour node that is muted is not changing the picture at all, and whether it is muted is readable.

---

### Part B — through a node that takes two pictures in and gives one out

The compositor has more than one kind of such node, and the answer to "which of the two came out" is different for each kind. This is not a single answer.

#### B1 — Blending nodes: both go in, both contribute, neither one "came out"

**Label: documented.**

Mix Color Node:

> "The Mix Node mixes values, colors and vectors inputs using a factor to control the amount of interpolation."

> "Factor
>
> Controls the amount of mixing between the A and B inputs.
>
> A/B
>
> The two inputs that are mixed together."

> "Outputs
>
> Result
>
> Output the result of the mix using the data type selected."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/mix_color.html`. Date read: 2026-07-30.

Alpha Over Node:

> "The Alpha Over node composites two images by layering the Foreground image over the Background, using standard alpha blending operations."

> "Background
>
> The background image.
>
> Foreground
>
> The foreground image to be placed over the background.
>
> Factor
>
> Controls the opacity of the foreground image, from 0 (fully transparent) to 1 (fully opaque)."

> "Outputs
>
> Image
>
> The combined image after alpha compositing."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/alpha_over.html`. Date read: 2026-07-30.

So for these two nodes the sources describe a **combination**, not a selection. The words used are "mixes", "the result of the mix", "combined image". Nothing in either page says the output is one of the two inputs. On this reading the question "which of the two came out the far side" has no answer for a blending node, because the documented answer is "both did".

**A partial exception the sources do state, and it is worth being exact about.** The manual does name, for a two-input node, which input decides the *shape and place* of the result — its size and position, not its colours:

> "Unless stated otherwise in their respective documentation pages, all other nodes use the following mechanism. One of the inputs of the nodes is designated as the Domain Input of the node, and the operation domain of the node is identical to the domain of that designated input."

> "The output of nodes will be intuitively clipped to the operation domain, or rather, the domain of the domain input. For instance, if the Foreground input is bigger than the Background input in the Alpha Over node, the output will be clipped to the Background input because it is the domain input"

> "Working around the clipping behavior of the Alpha Over node using a Mix node, noting that the first Image input in the Mix node has the highest domain priority"

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html`. Date read: 2026-07-30.

This settles *which input governs the frame the result is cut to* — the Background for Alpha Over, the first image input for Mix. It does **not** settle which picture's content came out. Do not read it as the latter.

#### B2 — Selecting nodes: exactly one goes in and out, and the sources say which

**Label: documented.**

Switch Node:

> "Switch between two images using a checkbox."

> "Switch
>
> When it is unchecked, the first input labeled "Off" is passed to the output.
>
> When checked, the second input labeled "On" is passed to the output."

> "Off
>
> First image input.
>
> On
>
> Second image input."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/switch.html`. Date read: 2026-07-30.

This is the clearest "yes" on the sheet's question: with a Switch node, the sources say outright which of the two came out the far side, and it is decided by one readable value.

Index Switch Node:

> "The Index Switch node outputs one of its inputs based on an integer Index value. Only the selected input is evaluated"

> "Determines which of the following input sockets is passed through to the output. The index is zero-based (i.e., the first input corresponds to index 0)."

> "Outputs the value from the input corresponding to the current Index value, without any modification."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/index_switch.html`. Date read: 2026-07-30.

The Index Switch node is new to the compositor in this version. From the release notes:

> "The **Index Switch** node is now supported."

Source: Blender 5.1 release notes, `https://developer.blender.org/docs/release_notes/5.1/compositor/`. Date read: 2026-07-30.

Menu Switch Node:

> "The Menu Switch node outputs one of its inputs based on a selected menu item. Only the active input is evaluated"

> "Menu
>
> Determines which input socket is passed through to the output."

> "Outputs the value from the input corresponding to the selected menu item, unchanged."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/menu_switch.html`. Date read: 2026-07-30.

All three of these nodes appear in the compositor's own list of utility nodes: "Switch Node", "Index Switch Node", "Menu Switch Node" (Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/index.html`, read 2026-07-30).

**But the deciding value is itself an input, not a fixed setting on the node.** This matters if you plan to read the answer out of a stored file rather than watch it happen.

- The manual lists "Switch" under *Inputs* on the Switch Node page, alongside "Off" and "On" (address above).
- The picture on that page shows it as an input socket: the node has one output "Image" at the top, then a row "Switch" with a checkbox, then "Off" and "On", each with a connector dot on the left-hand side. Picture: `_images/node-types_CompositorNodeSwitch.webp`, shown on the Switch Node page. Label for what the picture shows: **documented** (it is part of the manual). Date read: 2026-07-30.
- The API page for the Switch node lists no property of its own — every property named on it is inherited from the general node type. Source: Blender Python API 5.1, `https://docs.blender.org/api/5.1/bpy.types.CompositorNodeSwitch.html`, read 2026-07-30.
- The manual also says the state moves over time: *"Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."* (Switch Node page, address above, read 2026-07-30.)
- For Index Switch, the deciding Index is likewise an input, and the manual describes connecting something to it: *"When the Index input is connected to a Menu Switch node set to Integer Type, the corresponding menu labels will automatically be shown next to the index value."* (Index Switch Node page, address above, read 2026-07-30.)
- For Menu Switch, the menu can be handed in from outside the group: *"Connecting the menu input with a Group Input node will expose the menu as a group input."* (Menu Switch Node page, address above, read 2026-07-30.)

So: which of the two came out is fixed and readable only when the deciding input is left at a plain value and not keyframed or connected. Where it is keyframed or connected, the sources say the choice varies, and they do not offer any way to read off a single answer for the whole render.

#### B3 — A node that chooses per pixel

**Label: documented.**

Depth Combine Node:

> "The Depth Combine node combines two images based on their depth maps. It overlays the images using the provided depth values to detect which parts of one image are in front of the other."

> "Result
>
> If both depth values are equal, it will use the foreground image. Whichever depth value is less decides which image pixel is used."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/depth_combine.html`. Date read: 2026-07-30.

Here the answer to "which of the two came out" is settled, but only one pixel at a time, and by data rather than by a setting. There is no single whole-picture answer.

#### B4 — A node where both come out, side by side

**Label: documented.**

Split Node:

> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line."

> "Image
>
> The first image input, displayed on one side of the split.
>
> Image
>
> The second image input, displayed on the opposite side of the split."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/split.html`. Date read: 2026-07-30.

Both came out, each occupying its own region, and which is which is decided by the Position and Rotation inputs.

#### B5 — One naming trap, seen in the manual's own picture

**Label: documented (from a picture in the manual).**

The Alpha Over Node page names its two picture inputs "Background" and "Foreground" in its text. The picture on that same page shows the node with its two picture inputs both labelled **"Image"** — reading down the node: an output "Image", then "Fac" with the value 1.000, then "Image", then "Image", then "Straight Alpha". Picture: `_images/node-types_CompositorNodeAlphaOver.webp`, shown on `https://docs.blender.org/manual/en/5.1/compositing/types/color/alpha_over.html`. Date read: 2026-07-30.

By contrast the Mix node's picture shows its two picture inputs with distinct names: in the Color version of the node, "Factor", then "A", then "B", with the output named "Result". Picture: `_images/render_shader-nodes_shader_mix_node.jpg`, shown on `https://docs.blender.org/manual/en/5.1/compositing/types/color/mix_color.html`. Date read: 2026-07-30.

What I take from this is only what is visible: the two Alpha Over picture inputs carry the same displayed name, and the two Mix picture inputs do not. ~~**Not covered:** the sources do not say what name each of those sockets is given when the file is read from outside.~~ **Settled by entry 2 below:** read from outside, the two Alpha Over picture inputs come back as `Background` and `Foreground`, so they are distinguishable by name and not only by order. *[Changed by the citation check, 2026-08-06: this gap was answered by a test printed later in the same document and was left standing here. A repeat on 2026-08-07 came back the same.]*

---

### Part C — is there any tag or marker carried along with a picture?

**Label: not in the sources.**

I looked for anything that would let a picture be told apart from another picture as it moves through the compositor — a name, a tag, a marker, carried metadata, anything. There is nothing of the sort in the 5.1 sources.

What I found instead, and ruled out:

- **The Image Info node** carries only spatial facts, no identity: *"The Image Info node outputs spatial and transformation information about an image in the compositor."* Its outputs are Dimensions, Resolution, Location, Rotation and Scale. Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/input/image_info.html`, read 2026-07-30.
- **Cryptomatte** identifies objects and materials *within* a picture, not which upstream picture a result came from: *"The Cryptomatte node uses a Cryptomatte image to create a mask for one or more objects or materials."* Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/mask/cryptomatte.html`, read 2026-07-30.
- **Bundles** — a general node feature that groups several named values into one connection — exist in 5.1 (*"A bundle is a container that groups multiple values into one socket."*, Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/types/utilities/bundles/index.html`, read 2026-07-30) but no bundle node appears in the compositor's own list of utility nodes (`https://docs.blender.org/manual/en/5.1/compositing/types/utilities/index.html`, read 2026-07-30), which lists Math, Vector, Levels, Normalize, Split, Switch, Index Switch, Menu Switch, Switch View, Relative To Pixel and Retime. *[Changed by the citation check, 2026-08-06: the Normalize Node was missing from this listing. The conclusion — no bundle node in that index — is unaffected.]*
- **Custom metadata** is read and written at the two ends of the graph only. From the release notes: *"The **Image** and **File Output** nodes now read and write custom meta-data."* Source: Blender 5.1 release notes, `https://developer.blender.org/docs/release_notes/5.1/compositor/`, read 2026-07-30. Nothing in the sources says this metadata travels along a connection from one node to the next, and neither the Image node page nor the File Output node page in the 5.1 manual mentions it at all.

**On the second reading of the question — recognising a picture afterwards.** The manual documents one deliberate technique for exactly this, and it works by putting a mark into the pixels beforehand:

> "Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

and, for the detection half:

> "When you see an image that you think might be yours, use the node tree below to compare it to your stock image (non-watermarked original). In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference. You can see how the original mark clearly stands out."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/color/mix_color.html`. Date read: 2026-07-30.

That is a technique you build by hand out of ordinary nodes, not something the compositor does for you, and the manual describes it as surviving mixing rather than colour grading. It settles nothing about a picture that has been through a colour-changing node.

---

### What a running Blender showed

**LABEL: observed.** Blender 5.1.2, run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, with Blender pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics while this ran. The whole procedure, every script in full and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only.** The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Every pixel comparison below rests on a control (entry 1) that came back `MAXDIFF render vs source 0.0` and `MAXDIFF render vs same render again 0.0`, so a difference of zero means the pictures match.

**The two Alpha Over picture inputs have different names when the file is read from outside (entry 2).** This is the naming trap in B5, settled:

> `PRESENT CompositorNodeAlphaOver label: Alpha Over`
> `   IN  [('Background', 'Background', 'RGBA'), ('Foreground', 'Foreground', 'RGBA'), ('Factor', 'Fac', 'VALUE'), ('Type', 'Type', 'MENU'), ('Straight Alpha', 'Straight Alpha', 'BOOLEAN')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

So the manual's picture, which draws both as `Image`, does not match what a script reads back.

**The Switch node's state is an input socket, not a property on the node (entry 2).**

> `PRESENT CompositorNodeSwitch label: Switch`
> `   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

**And it can be driven by a connection (entry 6).**

> `VALUE NODE USED ShaderNodeValue`
> `LINK FROM VALUE NODE ACCEPTED, is_valid: True`
> `LINK FROM A PICTURE ACCEPTED, is_valid: True`
> `WITH A PICTURE ON THE SWITCH INPUT: maxdiff vs picA=0.0  maxdiff vs picB=1.0`

The picture fed in ran from 0 on the left of the frame to 1 on the right, and the whole frame still came out as the Off input — so the choice was accepted from a link but did not vary across the frame.

**Mix at Factor 0 gives back the A input exactly, and Alpha Over at Factor 0 gives back the Background exactly (entries 3 and 4).**

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=0.0`
> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

The foreground picture used was opaque throughout, which is why factor 1 gives it back whole.

**A muted Alpha Over passes the Background (entries 5 and 7).** The file says so, and the render agrees:

> `=== AlphaOver (CompositorNodeAlphaOver) ===`
> `  unmuted internal_links: [('Background', 'Image')]`
> `  muted   internal_links: [('Background', 'Image')]`
> `MUTED AlphaOver            internal_links=[('Background', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Mix                  internal_links=[('A_Color', 'Result_Color')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Switch               internal_links=[('Off', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Split                internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Z/DepthCombine       internal_links=[('A', 'Result'), ('Depth B', 'Depth')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`

and a muted colour-changing node hands the picture back untouched, where an unmuted one does not:

> `COLOUR NODE HueSat     muted=False maxdiff vs the picture that went in = 1.0`
> `COLOUR NODE HueSat     muted=True  maxdiff vs the picture that went in = 0.0`

**Custom metadata (entry 14).** The release note quoted above says the Image and File Output nodes now read and write custom metadata. Asked of the running 5.1:

> `CompositorNodeImage: properties naming metadata -> none`
> `   its input socket names: []`
> `   its output socket names: [('Image', 'Image', 'RGBA'), ('Alpha', 'Alpha', 'VALUE')]`
> `CompositorNodeOutputFile: properties naming metadata -> none`
> `CompositorNodeImageInfo: properties naming metadata -> none`
> `Image data-block properties naming metadata -> none`

The Image node has no input socket at all, so nothing can be fed into it, and no property on any of the three names metadata. What was not reached is whether metadata already inside a file on disk comes out anywhere — there is no place in the node's own properties to look.

**One further thing this document leans on, settled the same day.** A faint mark put into a picture does survive a colour-changing node, if it is looked for against the same picture put through the same colour change (entry 13):

> `BEFORE any colour change: biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `AFTER the colour change  : biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `MARKED-AND-GRADED against the UNMARKED ORIGINAL: inside the mark = 0.607381, outside it = 1.000000`

Compared against the unmarked original, as the documented technique describes, the difference outside the mark is larger than the difference inside it.

**The processor against the card, run on 2026-08-07 once the card was repaired (entry 12).** On 2026-08-06 the machine's graphics card could not be reached: its kernel driver and its driver library had drifted apart, `nvidia-smi` would not start, and Blender printed EGL errors on every run. The card has since been repaired. Everything in this paragraph was got on **2026-08-07**, on that same rented Linux machine — **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and still no display server at all** — with Blender 5.1.2, `-b --factory-startup`, and a settings folder of its own made beforehand under `/root/compositor/`. The card now answers:

> `NVIDIA-SMI 580.173.02             Driver Version: 580.173.02     CUDA Version: 13.0`
> `NVIDIA GeForce RTX 4080 SUPER, 580.173.02, 16376 MiB`

and no run printed an EGL line of any kind.

A colour node and an Alpha Over at factor 0.5, rendered once with the compositing device set to `CPU` and once to `GPU`, the two pictures loaded back and compared pixel by pixel:

> `DEVICE CPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `DEVICE GPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `MAXDIFF between the processor's picture and the card's picture: 6.258487701416016e-07`

**The run marked `GPU` really does use the card.** A heavier tree — a 1024 by 1024 picture through sixty Blur nodes, rendered at 2048 by 2048 — was run twice, one device to a Blender, while the card was asked over and over which processes were on it. Only this Blender's own process was counted, because the machine is shared with other topics' tests. Set to the processor, then set to the card:

> `HEAVY PROCESSOR CORE-SECONDS CPU 38.420`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 116`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`
> `HEAVY PROCESSOR CORE-SECONDS GPU 1.210`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 40`
> `TIMES THIS PROCESS WAS ON THE CARD: 13`
> `MOST CARD MEMORY THIS PROCESS HELD: 150 MiB`

and the two heavy pictures, loaded back and compared pixel by pixel:

> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`

So the setting really moves the work onto the card, and the picture the card makes matches the processor's to about four parts in ten million. **This bears on identifying a picture by its exact values**: two pictures made from the same tree on the two devices are not bit-for-bit equal, so an identity test finer than a millionth would fail across them, and any test coarser than that still holds.

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

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-5.1`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure is in the hand-test document named above, entries 15 to 18.

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

The Mac's run with no window printed those same lines, character for character. **This test also ran on Linux** — entry 11 above — and the Linux run printed the same lines too, apart from the file's own path. The two machines agree. This also bears on the naming trap in B5: the socket names a file reports are the same whether it is read with a window or without one, on both machines.

**The Viewer node's image fills when a window is open, and stays empty when there is none (entry 16).** The same script, one render of 64 by 64, run twice. With no window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`

With a window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; there is no Linux counterpart to the windowed case.

**Every property name on a socket is the same with a window as without (entry 17).** Both runs printed:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree, and none of the names is an inspection, a resolution, a transform, a precision or a sampling.

Both runs also printed the same four screen areas — `PROPERTIES`, `OUTLINER`, `DOPESHEET_EDITOR`, `VIEW_3D` — and the run with no window printed `SCREEN PRESENT: True` as well. So on macOS a Blender started with `-b` still has a screen with areas on it, and that printout does not tell a windowed run from a windowless one. What tells them apart is entry 16, where the Viewer image filled only in the windowed run.

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

### Where I looked

**Files searched.** The whole of the 5.1 manual's `compositing` folder (every page, listed by directory walk); the manual's `interface/controls/nodes` folder; the manual's glossary; the whole of the 5.1 Python API documentation folder (searched by file name and by content); all of the 5.1 release notes; all of the Cycles 5.1 files.

**Terms searched for**, across the compositing manual and the node-interface manual: *identif*, *provenance*, *trace*, *track*, *origin*, *metadata*, *tag*, *label*, *unchanged*, *pass through*, *passed to the output*, *which input*, *source image*, *name*, *came from*, *originated*, *which image*, *source of the image*, *image identity*, *bundle*, *mute*, *Color Blend Modes*. Across the API documentation: *domain priority*, *is_linked*, *links*, *internal_links*, *mute*, and the node type names for Alpha Over, Mix, Switch, Index Switch, Menu Switch, Hue/Saturation, Colour Balance and RGB Curves.

**Pages opened in full:** the compositor's Introduction; Compositor System; Limitations; Mix Color; Alpha Over; Depth Combine; Hue/Saturation/Value; Colour Balance; RGB Curves; Exposure; Brightness/Contrast; the Adjust index; Switch; Index Switch; Menu Switch; Split; the Utilities index; Viewer; File Output; Image; Image Info; Cryptomatte; Groups; the node editing page under the interface section; the glossary entry for Color Blend Modes; the shader-node Mix page. API pages: the link between nodes, the socket, the general node, the Alpha Over node, the Switch node, the boolean socket. Release notes: the compositor page for 5.1.

**Pictures opened:** `_images/node-types_CompositorNodeSwitch.webp`, `_images/node-types_CompositorNodeAlphaOver.webp`, `_images/render_shader-nodes_shader_mix_node.jpg`.

**Ruled out and why.**
- *Cycles' own files* — searched for "compositor", "AlphaOver" and "alpha_over". The only hits are a panel for compositor performance settings in the interface file, and a line of arithmetic inside the film-reading code. Neither says anything about following a picture through compositor nodes. Ruled out.
- *The requirements page* — it describes what hardware and systems a release runs on. It has no bearing on this question and was not used.
- *Keying, mask and tracking nodes* — searched; they identify content within a picture (a colour range, a drawn shape, a tracked plane), not which upstream picture emerged. Ruled out.
- *The glossary's Color Blend Modes entry* — opened in the hope it defined what a Factor of zero or one does. It says only *"Methods for blending two colors together."* and points at a page outside these sources, which I did not follow. So the sources here do not define the endpoints of the mixing factor.

---

### Outside the interface

**LABEL: observed for the tracing half, on Linux and on macOS.** On the rented Linux machine named above, on 2026-08-06, Blender 5.1.2 with `-b` opened a saved file and printed the whole graph — every link with both its nodes and both its sockets, the Switch node's stored choice, and each node's internal links. So the tracing described in Part A and the reading of the deciding input in Part B2 do not depend on the interface, and that is now a result rather than an appearance. On the Creator's Mac on 2026-08-07 the same file was read both with no window and with the window open, and the two printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round: the wiring reads the same whether Blender has its window open or not, and the naming trap noted at the end of this section is answered for the file's own report — the socket names came back the same either way.

**The Viewer node is the other way round, and that is now tested too.** With no window its image exists but stayed at 256 by 256 with every value zero across a 64 by 64 render, on both machines. With a window open on the Mac it came back at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16). So the Viewer's preview really is reachable only when there is a window, as this section's reading of the sources said.

**And what a script cannot reach either way.** Every property name on a socket was the same with a window as without, on both machines, and none of them names an inspection, a resolution, a transform, a precision or a sampling (entry 17).

**Choosing the graphics card, and having the choice acted on, both work with no window.** On 2026-08-07, on the same machine with its card repaired and its driver at 580.173.02, a Blender started with `-b` and its compositing device set to the card really did the compositing there — the card named that Blender's own process and it held 150 MiB while it worked, where the same run set to the processor never appeared on the card at all (entry 12). Neither the setting nor its effect needs a window.

The reading from the sources, unchanged, follows.

**Label: not in the sources, with one documented part.**

The parts of this answer that come from the API documentation are statements about data the file holds — the connections between nodes, both of their ends, whether a socket is connected, whether a node is muted. Nothing on those pages says they hold only while a person is looking at a window, and the API documentation is the very thing used to drive Blender from outside. On that basis the tracing described in Part A and the reading of the deciding input in Part B2 do not appear to depend on the interface. But the sources do not say so in so many words, so I will not label it documented.

What is plainly of the interface only, and so does not survive being driven from outside a window:

- The Viewer node's preview. *"It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."* and *"Only the active Viewer node (with the icon in the header) will be displayed in the backdrop or Image Editor."* Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/types/output/viewer.html`, read 2026-07-30.
- The Split node's on-screen handle: *"The Split node provides an interactive gizmo in the Node Editor."* Same source folder, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/split.html`, read 2026-07-30.
- Muting a node, and muting a link, are described in the manual purely as menu items and key presses. The API documentation exposes the muted state itself, but the manual's description of it is an interface one. `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html`, read 2026-07-30.

~~**Not covered:** the sources do not state whether the picture the manual shows of a node matches what a file read from outside reports.~~ **Settled by entry 2 above:** it does not. The manual's picture draws both Alpha Over picture inputs as `Image`; a file read from outside reports them as `Background` and `Foreground`, and reports the Switch state as an input socket. *[Changed by the citation check, 2026-08-06: this gap was answered by a test printed earlier in the same document, and the section above it already said so, so the two paragraphs contradicted each other.]*


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card result added on 2026-08-07 was got on that same Linux machine once its graphics card had been repaired, and settles Linux alone in the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window; the four Alpha Over and Switch renders; the Viewer node's image staying empty with no window; and every property name on a socket. Nothing came out differently on the two operating systems. The rest of the Linux tests are still owed on the Mac and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding.

The reading from the sources, unchanged, follows.

**Label: not in the sources.**

Nothing in the pages above ties any of this to one operating system. The only operating-system-specific statement I found anywhere in the compositing section is about the graphics card timing out during compositing, and it is about speed, not about following a picture through nodes:

> "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Source: Blender manual 5.1, `https://docs.blender.org/manual/en/5.1/compositing/limitations.html`. Date read: 2026-07-30. The page heads that section "TDR on Windows".

So: nothing suggests the answer differs by operating system, and nothing states that it does not. Searched the compositing folder for operating-system names; that limitations page is the only hit relevant here.

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
## Things a person with Blender open could settle in a few minutes

**All seven of these have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above. The two things the Linux machine could not reach were run on the Creator's Mac on 2026-08-07, and are in *What a running Blender showed on the Creator's Mac*: the graph read the same with Blender's window open as with none, and the Viewer node's image filled only when there was a window. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **Most of the tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Do the two Alpha Over picture inputs have different names when the file is read from outside?**
   Open Blender 5.1. In a scene, open the Compositor and turn on Use Nodes. Add an Alpha Over node. Open a Python Console and print the name of every input socket on that node, in order.
   *Positive result:* the two picture sockets come back with different names (for example "Background" and "Foreground"), so they can be told apart by name.
   *Negative result:* both come back with the same name (for example both "Image"), so only their order tells them apart. Either outcome is possible, so the test carries information.

2. **Is the Switch node's state a socket or a setting on the node?**
   Same setup. Add a Switch node. Print the names of its input sockets, and separately try to read a state property directly on the node.
   *Positive result:* an input socket named "Switch" exists and its value can be read there.
   *Negative result:* there is no such socket and the state is only a property on the node itself. The manual and its picture point to the first, but this settles it.

3. **Can the Switch state be driven by a connection rather than a fixed value?**
   Add a Switch node and try to connect the output of another node into its Switch input.
   *Positive result:* the connection is accepted, which means the choice of which picture comes out cannot always be read as a single fixed value.
   *Negative result:* the connection is refused, which means the choice is always a stored value or a keyframed one.

4. **Does a Mix node at Factor 0 give back exactly the A input?**
   Build: two different Image or colour inputs into a Mix node set to Color, blend mode Mix, Factor at 0. Feed the result and the A input separately into a second Mix node set to Difference, and view that with a Viewer node; sample a few pixels.
   *Positive result:* the difference is zero everywhere, so Factor 0 passes A through untouched.
   *Negative result:* the difference is not zero, so it does not. The manual does not state either way.

5. **The same test for Alpha Over at Factor 0.**
   Two pictures into an Alpha Over node with Factor 0. Difference the result against the Background input.
   *Positive result:* zero everywhere, so a Factor of 0 leaves only the Background.
   *Negative result:* not zero.

6. **Does a muted Alpha Over node pass the Background or the Foreground?**
   Connect two clearly different pictures into an Alpha Over node, mute the node, and view the output.
   *Positive result:* one of the two appears, identifying which input the mute routes through — and it can also be read directly from the node's internal routing list.
   *Negative result:* the output is black or empty, meaning muting does not pass either through for this node.

7. **Does custom metadata written by the File Output node survive a colour-changing node in between?**
   Load an image carrying custom metadata with an Image node, pass it through a Hue/Saturation/Value node, then into a File Output node, and render one frame. Inspect the written file's metadata.
   *Positive result:* the custom metadata is present in the written file, so it travels along the graph.
   *Negative result:* it is absent, so metadata does not follow a picture through nodes. The sources say only that the two end nodes read and write it, not that it travels.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 5.1

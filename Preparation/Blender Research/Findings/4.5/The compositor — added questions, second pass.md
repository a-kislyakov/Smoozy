# The compositor — added questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual (including its pictures in `_images`), Blender's 4.5 Python API documentation, and the 4.5 release notes — all read on 2026-07-30 (the snapshot date). Cycles' own 4.5 files were not needed and were not opened; nothing in this question touches the renderer. Blender's requirements page was not needed and was not opened.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 8 only. Questions 1–7 were left alone.

One note on method, for honesty: to read the small node screenshots properly I copied two of them out of `Blender Sources/manual_4.5/_images` into a temporary folder outside the project and enlarged them. The pictures are the manual's own; nothing was added to or taken from the project by doing so.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

**The question is ambiguous, and I say so rather than choosing silently.** "Followed forward" can mean either (a) tracing the picture's onward path through the connections of the node tree — reading the file and seeing where the picture goes next — or (b) recognising the picture itself in the finished result, by looking at pixels. I answer reading (a), because "followed forward **through a node**" describes a path through the tree. Reading (b) is addressed briefly at the end of this answer, and it comes out very differently.

### 8a. Through a node that changes its colours

**LABEL: documented.**

A colour-changing compositor node has exactly one image input and exactly one image output, so there is exactly one place the picture can leave it, and that place is recorded in the file.

Hue/Saturation/Value node:

> "The Hue/Saturation/Value Node applies a color transformation in the HSV Color Model."

> "Inputs … Image/Color — Standard color input."

> "Outputs … Image/Color — Standard color output."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/adjust/hue_saturation.html — read 2026-07-30.

The manual's own picture of that node agrees: one input socket named `Image`, one output socket named `Image`, with the rest of the rows (`Hue`, `Saturation`, `Value`, `Fac`) being value inputs.
Picture: `_images/node-types_CompositorNodeHueSat.webp`, on the page above. **documented** — what is visible in the picture is one image in and one image out.

RGB Curves is the same shape:

> "The RGB Curves Node performs level adjustments on each color channel."

> "Inputs … Image/Color — Standard color input."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/adjust/rgb_curves.html — read 2026-07-30.

The onward path is readable because links are stored objects with both ends named:

> "NodeLink(bpy_struct) … Link between nodes in a node tree"

> "from_node — Type: Node, (readonly)"

> "from_socket — Type: NodeSocket, (readonly)"

> "to_node — Type: Node, (readonly)"

> "to_socket — Type: NodeSocket, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeLink.html — read 2026-07-30.

And every link in the tree, and every socket of a node, can be listed:

> "links — Type: NodeLinks bpy_prop_collection of NodeLink, (readonly)"

> "nodes — Type: Nodes bpy_prop_collection of Node, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeTree.html — read 2026-07-30.

> "inputs — Type: NodeInputs bpy_prop_collection of NodeSocket, (readonly)"

> "outputs — Type: NodeOutputs bpy_prop_collection of NodeSocket, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30.

A socket also knows whether anything is attached to it, and which links those are:

> "is_linked — True if the socket is connected"

> "links — List of node links from or to this socket."

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30.

So: yes. A colour-changing node has a single image output; whatever went in comes out there and nowhere else, and the links leaving it can be listed.

One caution the sources make explicit: what is being followed is the *connection*, not the pixels. The node "applies a color transformation" (quoted above), so the picture that leaves is not the picture that arrived.

### 8b. Through a node that takes two pictures in and gives one out

**LABEL: documented** for the following; **not in the sources** for the part noted at the end.

The compositor has several such nodes, and all of them have a single image output, so the onward path is followed exactly as in 8a.

Alpha Over:

> "The Alpha Over node is used to layer an image on top of another with alpha blending."

> "Inputs … Image — The background image. Image — The foreground image."

> "Outputs … Image — The blended result."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/alpha_over.html — read 2026-07-30.

Z Combine:

> "The Z Combine node combines two images based on their Z-depth maps."

> "Inputs … Image — The background image. Z — Z depth of the background image. Image — The foreground image. Z — Z depth of the foreground image."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/z_combine.html — read 2026-07-30.

Mix Color:

> "The Mix Node mixes values, colors and vectors inputs using a factor to control the amount of interpolation."

> "A/B — The two inputs that are mixed together."

> "Outputs … Result — Output the result of the mix using the data type selected."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/mix_color.html — read 2026-07-30.

Switch:

> "Switch between two images using a checkbox."

> "Inputs … Image — First image input. Image — Second image input."

> "Outputs … Image — Standard color output."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch.html — read 2026-07-30.

**Telling the two inputs apart, going in.** For Alpha Over and Z Combine the two image inputs carry *the same name*. The manual's pictures show this plainly: Alpha Over is drawn with `Fac`, then `Image`, then `Image`, then `Straight Alpha`, and one output `Image`; Z Combine is drawn with `Image`, `Z`, `Image`, `Z`, `Use Alpha`, `Anti-Alias`, and two outputs `Image` and `Z`.
Pictures: `_images/node-types_CompositorNodeAlphaOver.webp` and `_images/node-types_CompositorNodeZcombine.webp`, on the two pages above. **documented** — the duplicate names are visible in the pictures and in the text quoted above.

Name alone therefore cannot separate them. Position and identifier can. `Node.inputs` is an ordered collection (quoted above), and every socket carries:

> "identifier — Unique identifier for mapping sockets … Type: string, default "", (readonly, never None)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30.

The Switch node is the exception: its two image inputs are drawn with distinct labels. Its picture shows one output `Image`, then a `Switch` checkbox drawn as a node property with no socket dot of its own, then two image inputs with sockets, labelled `Off` and `On`.
Picture: `_images/node-types_CompositorNodeSwitch.webp`, on the Switch page above. **documented** — those labels and that arrangement are visible in the picture.

### 8c. Which of the two came out the far side

There is no single answer. It depends entirely on which node it is, and the sources are explicit for some and explicit that it is a mixture for others.

**Switch — it can be told. LABEL: documented.**

> "Properties … Switch — When it is unchecked, the first input labeled "Off" is passed to the output. When checked, the second input labeled "On" is passed to the output."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch.html — read 2026-07-30.

The same choice is exposed as a stored value:

> "check — Off: first socket, On: second socket. (Deprecated: Use Switch input instead.) Type: boolean, default False"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeSwitch.html — read 2026-07-30.

Two things qualify this, both from the sources. First, that deprecation note says the choice is now to be made through a "Switch input" — an input socket rather than a property — which fits the release note:

> "Most nodes now expose their options as single value inputs. (blender/blender#137223)"

> "Most options in nodes are now deprecated, replaced with corresponding inputs, and will be removed in v5.0. (blender/blender#137223)"

Source: Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/compositor/ — read 2026-07-30.

The manual page still lists Switch under *Properties*, and the manual's picture draws it as a checkbox without a socket. The sources therefore disagree with each other on this point in 4.5, and I do not resolve it. If the choice is an input socket and something is plugged into it, nothing in the sources says how to read which way it will fall without running the compositor.

Second, the choice need not be constant:

> "Tip — Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

Source: Blender manual, Switch page above — read 2026-07-30.

So "which came out" can differ from frame to frame.

**Z Combine — it is decided per pixel, not per picture. LABEL: documented.**

> "Outputs … Image — If both Z values are equal, it will use the foreground image. Whichever Z value is less decides which image pixel is used."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/z_combine.html — read 2026-07-30.

Different pixels of the one output come from different inputs, so the question has no single answer for the picture as a whole. The page also notes alpha can make the result a mixture rather than a choice:

> "Use Alpha — The chosen Image pixel alpha channel is also carried over. If a pixel is partially or totally transparent, the result of the Z Combine will also be partially transparent; in which case the background image will show through the foreground (chosen) pixel."

Source: same page, read 2026-07-30.

**Alpha Over and Mix Color — neither one comes out; both do. LABEL: documented.**

Alpha Over's output is "The blended result" of "alpha blending" (quoted in 8b). Mix Color's output is "the result of the mix" using "a factor to control the amount of interpolation" (quoted in 8b). The glossary defines what the blend modes are:

> "Blend Modes — Color Blend Modes — Methods for blending two colors together."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/glossary/index.html — read 2026-07-30.

So for these two nodes, asking which of the two came out the far side has no documented answer, because the documented answer is "a combination of both".

**A muted node — it can be told, and it is readable from the file. LABEL: documented.**

> "Mute … Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30.

The mapping that muting uses is stored on the node and is made of ordinary links, so both ends can be read:

> "internal_links — Internal input-to-output connections for muting — Type: bpy_prop_collection of NodeLink, (readonly)"

> "mute — Type: boolean, default False"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30.

So for a *muted* two-in-one-out node, which input reaches the output is stated by `internal_links`, whose members carry `from_socket` and `to_socket`.

**A muted link — also readable. LABEL: documented.**

> "A muted link acts as though it's no longer there; this also means the input fields for specifying fixed values become visible again."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30.

> "is_muted — Link is muted and can be ignored — Type: boolean, default False"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeLink.html — read 2026-07-30.

If the link feeding one of the two inputs is muted, that is a stored flag on the link and can be read.

**A socket that has no effect — a flag exists, but the sources do not say when it applies. LABEL: documented, with the gap named.**

> "is_inactive — Socket is grayed out because it has been detected to not have any effect on the output — Type: boolean, default False, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30.

This is described in terms of how the socket is *drawn* ("grayed out"), and nothing in the sources says which editors set it, for which nodes, or when it is recalculated. I record it because it is the closest thing in the sources to a direct statement that one of two inputs does not reach the output, but I would not lean on it.

**A picture carrying its own identity through a node — LABEL: not in the sources.**

Nothing in the 4.5 manual, the 4.5 Python API documentation or the 4.5 release notes describes any name, tag, label or metadata attached to a picture inside the compositor that survives passing through a node and can be recovered on the far side. The compositor's own account of what a picture *is* while it is in there names only its numbers and its position in space, with no identity:

> "Three types of data exist, all of which are stored in half precision formats: Float … Vector … Color — A 4D vector storing the Red, Green, Blue, and Alpha of the color."

> "Consequently, images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30.

The one thing a node *is* documented to know about which of two inputs matters is the shape of the output area, not which picture came out:

> "Each input in the node has the so called Domain Priority property, the operation domain of the node is the domain of the non single value input with the highest domain priority."

> "The Alpha Over node currently does not support changing the domain priority for its inputs, so as a workaround, one can use a Mix node to achieved the desired behavior, noting that the first Image input in the Mix node has the highest domain priority"

Source: same page, read 2026-07-30.

### 8d. The other reading — recognising the picture in the result

Under reading (b) — recognising the picture itself once it has come out — the sources describe no built-in facility, but they do describe a technique the user builds by hand, on the Mix Color page:

> "Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

> "When you see an image that you think might be yours, use the node tree below to compare it to your stock image (pre-watermarked original). In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference. You can see how the original mark clearly stands out."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/mix_color.html — read 2026-07-30.
Pictures: `_images/compositing_types_color_mix_watermark-encode.png` and `_images/compositing_types_color_mix_watermark-decode.png`, on that page.

**LABEL: documented** that the technique exists and how it is built. It is a node setup a person makes, not a property of the compositor, and the page also notes the Map Value node used in it is on its way out:

> "The Map Value node was deprecated and will be removed in v5.0."

Source: Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/compositor/ — read 2026-07-30.

### What a running Blender showed

**LABEL: observed.** Blender 4.5.12 LTS, run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, with Blender pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics while this ran. The whole procedure, every script in full and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only.** The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Every pixel comparison below rests on a control (entry 1) that came back `MAXDIFF render vs source 0.0` and `MAXDIFF render vs same render again 0.0`, so a difference of zero means the pictures match.

**The Switch node's choice is an input socket, and the two Alpha Over picture inputs carry different identifiers (entry 2).**

> `PRESENT CompositorNodeSwitch label: Switch`
> `   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`
> `PRESENT CompositorNodeAlphaOver label: Alpha Over`
> `   IN  [('Fac', 'Fac', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA'), ('Straight Alpha', 'Straight Alpha', 'BOOLEAN')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

**A muted node says which input reaches the output, and what it says is what the render does (entries 5 and 7).**

> `=== AlphaOver (CompositorNodeAlphaOver) ===`
> `  unmuted internal_links: [('Image', 'Image')]`
> `  muted   internal_links: [('Image', 'Image')]`
> `  muted   is_inactive: [('Fac', 'Fac', True), ('Image', 'Image', False), ('Image', 'Image_001', True), ('Straight Alpha', 'Straight Alpha', True)]`

and, rendered with two different pictures on the two inputs:

> `MUTED AlphaOver            internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Mix                  internal_links=[('A_Color', 'Result_Color')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Switch               internal_links=[('Off', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Split                internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `MUTED Z/DepthCombine       internal_links=[('Image', 'Image'), ('Z_001', 'Z')]  maxdiff vs picA=0.0  maxdiff vs picB=1.0`

**`is_inactive` is set on compositor sockets in this version** — the muted line above shows it turning true for the second picture input, the factor and the checkbox. Unmuted, every one of those reads false.

**Alpha Over at Factor 0 gives back the background exactly (entry 4).**

> `AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

**Z Combine really does choose pixel by pixel (entry 8).** With the first picture's depth taken from a picture running 0 on the left of the frame to 1 on the right, against a flat 0.5 for the second:

> `   DEPTHGRAD left column x=5 y=32 -> [0.07937, 0.50794, 0.25, 1.0]   (picA there [0.07937, 0.50794, 0.25, 1.0], picB there [0.1, 0.92063, 0.8, 1.0])`
> `   DEPTHGRAD right column x=58 y=32 -> [0.1, 0.07937, 0.8, 1.0]   (picA there [0.92063, 0.50794, 0.25, 1.0], picB there [0.1, 0.07937, 0.8, 1.0])`
> `   maxdiff vs picA=1.0  maxdiff vs picB=1.0`

**The whole graph reads back out of a saved file with no window (entry 11).** One Blender built and saved the tree; a second, separate one opened it with `-b` and printed:

> `TREE bpy.data.scenes['Scene'].node_tree`
> `LINK Image.Image -> Colour.Image   muted=False valid=True`
> `LINK Colour.Image -> TwoIn.Image   muted=False valid=True`
> `LINK Image.001.Image -> TwoIn.Image_001   muted=False valid=True`
> `LINK Image.Image -> Chooser.Off   muted=False valid=True`
> `LINK Image.001.Image -> Chooser.On   muted=False valid=True`
> `LINK TwoIn.Image -> Composite.Image   muted=False valid=True`
> `CHOOSER internal_links: [('Off', 'Image')]`

**Three further things this document leans on, settled the same day.** The Switch input accepts a link, including from a whole picture, and the choice still does not vary across the frame (entry 6). No property on a socket or on a node names a priority or a domain (entry 5). And a link cannot be walked across a node group boundary — nothing in the outer tree names a node inside the group (entry 10).

**The processor against the card, run on 2026-08-07 once the card was repaired (entry 12).** On 2026-08-06 the machine's graphics card could not be reached: its kernel driver and its driver library had drifted apart, `nvidia-smi` would not start, and Blender printed EGL errors on every run. The card has since been repaired. Everything in this paragraph was got on **2026-08-07**, on that same rented Linux machine — **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and still no display server at all** — with Blender 4.5.12 LTS, `-b --factory-startup`, and a settings folder of its own made beforehand under `/root/compositor/`. The card now answers:

> `NVIDIA-SMI 580.173.02             Driver Version: 580.173.02     CUDA Version: 13.0`
> `NVIDIA GeForce RTX 4080 SUPER, 580.173.02, 16376 MiB`

and no run printed an EGL line of any kind.

A colour node and an Alpha Over at factor 0.5, rendered once with the compositing device set to `CPU` and once to `GPU`, the two pictures loaded back and compared pixel by pixel:

> `DEVICE CPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `DEVICE GPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `MAXDIFF between the processor's picture and the card's picture: 6.258487701416016e-07`

**The run marked `GPU` really does use the card.** A heavier tree — a 1024 by 1024 picture through sixty Blur nodes, rendered at 2048 by 2048 — was run twice, one device to a Blender, while the card was asked over and over which processes were on it. Only this Blender's own process was counted, because the machine is shared with other topics' tests. Set to the processor, then set to the card:

> `HEAVY PROCESSOR CORE-SECONDS CPU 61.060`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 67`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`
> `HEAVY PROCESSOR CORE-SECONDS GPU 1.760`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 31`
> `TIMES THIS PROCESS WAS ON THE CARD: 9`
> `MOST CARD MEMORY THIS PROCESS HELD: 214 MiB`

and the two heavy pictures, loaded back and compared pixel by pixel:

> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`

So the setting really moves the work onto the card, and the picture the card makes matches the processor's to about four parts in ten million. **This bears on identifying a picture by its exact values**: two pictures made from the same tree on the two devices are not bit-for-bit equal, so an identity test finer than a millionth would fail across them, and any test coarser than that still holds.

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

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-4.5`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure is in the hand-test document named above, entries 15 to 18.

**The graph reads the same with a window open as with none (entry 15).** One Blender built and saved the tree with no window; a second opened that file with no window and printed it; a third opened the same file with the window open and printed it. The run with a window, word for word:

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

The Mac's run with no window printed those same lines, character for character. **This test also ran on Linux** — entry 11 above — and the Linux run printed the same lines too, apart from the file's own path. The two machines agree.

**The Viewer node's image fills when a window is open, and stays empty when there is none (entry 16).** The same script, one render of 64 by 64, run twice. With no window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`

With a window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; there is no Linux counterpart to the windowed case.

**Every property name on a socket is the same with a window as without (entry 17).** Both runs printed:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree. `is_inactive` is on that list, which is the flag this document's *Not covered* section asked about; how it behaves is entry 5 above.

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

**One thing about ending a Blender that has a window.** The scripts were written to close Blender with `bpy.ops.wm.quit_blender()`. On this Mac that line on its own did not end it: the script's printing all appeared, and the process was still running when a limit of my own ended it after three minutes. The same script with `sys.exit(0)` added after that line ended in about a second and printed the same thing. Both are recorded in entry 19 of the hand-test list.

### Not covered

**A note on this list, added by the citation check, 2026-08-06.** It was written before the hand tests above were run, and several of its entries were answered by them. Each such entry is struck through and answered below. Nothing in this list should be read as still open unless it says so.

- ~~What a render actually *produces*. Everything above is what the sources say a node is for and what the file stores. None of it is a statement about the pixels a real render puts out.~~ **Answered by the tests above**, which are renders throughout.
- Whether a Mix Color factor of exactly 0 or 1, or an Alpha Over factor of exactly 0, makes the output identical to one input. The Alpha Over page says the factor is "The alpha of the foreground image, going from 0 (fully transparent) to 1 (fully opaque)", which points that way, but getting from those words to "at 0 the background alone comes out" is a step the reader has to supply, and the sources do not take it. I do not claim it.
- How to read which way a Switch will fall when its choice comes from an input socket that is linked to something, or is driven by an animated value. The sources do not say.
- ~~Whether `NodeSocket.is_inactive` is set for compositor nodes at all, and when.~~ **Settled by entry 5 above:** in 4.5 it is set, and muting a node is what sets it — on a muted Alpha Over the second picture input, the factor and the checkbox all turn true, and unmuted every one reads false. *[Changed by the citation check, 2026-08-06: this bullet was left standing after the test that answered it; a repeat on 2026-08-07 came back the same.]*
- Node groups and reroute nodes. The question names a colour-changing node and a two-in-one-out node; I did not follow the path through groups.
- Every two-in-one-out node in the compositor. I looked at Alpha Over, Z Combine, Mix Color, Switch and Switch View. I did not go through nodes that take one picture plus a control input. (Entry 2 above counts twenty-three such nodes in 4.5 and names them.)
- Anything about versions other than 4.5.

### Outside the interface

**LABEL: observed for the reading half, on Linux and on macOS.** On the rented Linux machine named above, on 2026-08-06, Blender 4.5.12 LTS with `-b` opened a saved file and printed the whole graph — every link with both its nodes and both its sockets, the Switch node's stored choice, each node's mute state and internal links. So the reading this section calls stored data does read with no window. On the Creator's Mac on 2026-08-07 the same file was read both with no window and with the window open, and the two printouts held the same nodes, the same links, the same socket identifiers and the same stored choice (entry 15). So this half is settled both ways round: the wiring reads the same whether Blender has its window open or not.

**The looking-at-it half does not hold with no window, and does hold with one.** The image called `Viewer Node` exists as soon as a Viewer node is added, but stayed at 256 by 256 with every value zero across a 64 by 64 render with no window, on both machines. With a window open on the Mac it came back at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16).

**And what a script cannot reach either way.** Every property name on a socket was the same with a window as without, on both machines, and none of them names an inspection, a resolution, a transform, a precision or a sampling (entry 17).

**Choosing the graphics card, and having the choice acted on, both work with no window.** On 2026-08-07, on the same machine with its card repaired and its driver at 580.173.02, a Blender started with `-b` and its compositing device set to the card really did the compositing there — the card named that Blender's own process and it held 214 MiB while it worked, where the same run set to the processor never appeared on the card at all (entry 12). Neither the setting nor its effect needs a window.

The reading from the sources, unchanged, follows.

Everything I have relied on for "yes, it can be followed" is stored data rather than screen state: the list of nodes, the list of links, each link's `from_node`/`from_socket`/`to_node`/`to_socket`, each socket's `identifier` and `is_linked`, each node's `mute` and `internal_links`, and the Switch node's stored choice. The Python API documents all of these as properties of the node tree, readable and (where not read-only) writable.

But the sources do not say, in so many words, that these read the same when Blender is started and driven by something other than a person at the window. I did not find any statement in the 4.5 manual's compositing pages about running the compositor from outside the interface; searching that folder for "background mode", "command line", "--background" and "headless" returned nothing.

Some of what I quoted *is* interface-only, and I separate it out plainly:

- The `Off` and `On` labels on the Switch node, and the layouts in all the node pictures, are how the node is drawn.
- `is_inactive` is defined by how the socket is drawn ("grayed out").
- "Links will appear red as an indicator of passing through the muted node" is a drawing cue; the `mute` boolean behind it is data.
- The keyboard and menu routes for muting nodes and links (M, Ctrl-Alt-RMB) are interface only. The stored flags they set are not.
- Node previews and the backdrop are interface features, listed under the node editor's overlays: "Previews — Display each node's Preview if the node's preview is also toggled."
  Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/introduction.html — read 2026-07-30.

So: the *reading* of the graph is described as data, which is the part that matters; whether it differs when Blender runs without its window is not stated either way in the sources.


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card result added on 2026-08-07 was got on that same Linux machine once its graphics card had been repaired, and settles Linux alone in the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window; the four Alpha Over and Switch renders; the Viewer node's image staying empty with no window; and every property name on a socket. Nothing came out differently on the two operating systems. The rest of the Linux tests are still owed on the Mac and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding.

The reading from the sources, unchanged, follows.

The sources say nothing that ties any of this to a particular operating system. The node tree, its links and its sockets are described without any such qualification anywhere I looked.

The only operating-system-specific statement I found anywhere in the compositing part of the manual is about GPU driver timeouts, and it has no bearing on this question:

> "TDR on Windows — Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/limitations.html — read 2026-07-30.


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
### Where I looked

Files opened, in full or in part, all under `Blender Sources/`:

- manual 4.5, compositing: `introduction.txt`, `compositor_system.txt`, `limitations.txt`, `index.txt` listings of `types/`
- manual 4.5, node pages: `types/color/mix/alpha_over.txt`, `types/color/mix/mix_color.txt`, `types/color/mix/z_combine.txt`, `types/utilities/switch.txt`, `types/utilities/split.txt`, `types/utilities/switch_stereo_view.txt`, `types/color/adjust/hue_saturation.txt`, `types/color/adjust/rgb_curves.txt`, `types/input/image_info.txt`
- manual 4.5, general node reference: `interface/controls/nodes/parts.txt`, `interface/controls/nodes/introduction.txt`, `interface/controls/nodes/editing.txt`
- manual 4.5, other: `glossary/index.txt`, `render/shader_nodes/color/mix.txt`
- manual 4.5 pictures: `_images/node-types_CompositorNodeSwitch.webp`, `_images/node-types_CompositorNodeAlphaOver.webp`, `_images/node-types_CompositorNodeZcombine.webp`, `_images/node-types_CompositorNodeHueSat.webp`
- API 4.5: `bpy.types.NodeLink.txt`, `bpy.types.NodeSocket.txt`, `bpy.types.Node.txt`, `bpy.types.NodeTree.txt`, `bpy.types.CompositorNode.txt`, `bpy.types.CompositorNodeSwitch.txt`, `bpy.types.CompositorNodeAlphaOver.txt`, `bpy.types.CompositorNodeMixRGB.txt`
- Release notes 4.5: `compositor.md`, `python_api.md`

Searches run across the whole of the 4.5 manual and the whole of the 4.5 API documentation, and what they turned up:

- `provenance`, `lineage`, `traceab*`, `which input`, `came from`, `origin of the image` — over the whole manual. Only hits were the Mix Color watermark passage ("where it came from", about paper-making) and unrelated import/export and video-sequencer pages. Nothing about tracking a picture through nodes.
- `identify`, `identity`, `tag the image`, `traceable` — over the compositing folder. Hits were the mathematical sense of "identity transformation" on the Compositor System page, the Viewer node's numbered shortcut, the keying pages' use of "identify content", and the watermark passage. None is an identity carried by a picture.
- `metadata`, `stamp` — over the compositing folder. One hit: "Cryptomatte sidecars (metadata files) are not supported." Ruled out: that is about files on disk, not about a picture inside the graph.
- `carries`, `carried over`, `preserve`, `retain` — over the compositing folder. Hits were all about image detail, alpha and aspect ratio being preserved by particular filters. Nothing about identity.
- `which of the two`, `which input`, `passed to the output`, `passes through`, `pass through` — over the compositing folder and the general node reference. Exactly three relevant hits, all quoted above: the Switch node's two sentences, and the mute sentence in `editing.txt`.
- `Domain Priority` — over the whole manual. One hit, on the Compositor System page, quoted above.
- `Color Blend Modes` and `Color Dodge` — over the whole manual, looking for a page defining what a blend actually does to two inputs. The manual's blend-mode pages all point at a "Color Blend Modes" reference which resolves, in this snapshot, only to the glossary entry quoted above; there is no page in the snapshot giving the per-mode formulas. Ruled out as a source for "which input dominates".
- `is_inactive` — over the whole API documentation. Defined once on `NodeSocket` and only listed as inherited elsewhere. No page says which node trees set it.
- `Viewer Node` — over the whole API documentation. Hits were the spreadsheet editor, the node editor's backdrop option, and the link-to-viewer operator. Nothing that documents reading a viewer's pixels to identify a source picture.
- `background mode`, `command line`, `--background`, `headless` — over the compositing folder. No hits at all. This is why the "outside the interface" answer above is a gap rather than a claim.
- Cycles' 4.5 files were deliberately not searched. Cycles is the renderer; the compositor's node graph is not part of it, and nothing in the question touches rendering.

---

## Things a person with Blender open could settle in a few minutes

**All seven of these have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above. The half of test 5 the Linux machine could not reach — whether the graph reads the same with Blender's window open — was run on the Creator's Mac on 2026-08-07, and so was the Viewer node question; both are in *What a running Blender showed on the Creator's Mac*. The graph read the same either way, and the Viewer node's image filled only when there was a window. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **Most of the tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

1. **Does the Switch node's choice exist as an input socket in 4.5, or only as a property?**
   Open Blender 4.5, switch to the Compositing workspace, tick *Use Nodes*, add a Switch node (Add ▸ Utilities ▸ Switch). Look at whether the `Switch` row has a socket dot on its left edge that a link can be dragged to. Then, in the Python console, run `bpy.context.scene.node_tree.nodes["Switch"].inputs.keys()` and read the list back.
   *Negative result:* the `Switch` row has no socket dot and the printed list contains only the two image inputs — meaning the API's "Use Switch input instead" note does not describe 4.5, and the manual's *Properties* listing is right.

2. **Can the two same-named image inputs of Alpha Over actually be told apart from the file?**
   Add an Alpha Over node. In the Python console, run `[(s.name, s.identifier) for s in bpy.context.scene.node_tree.nodes["Alpha Over"].inputs]`.
   *Negative result:* the two image inputs come back with identical `identifier` strings as well as identical names — meaning they cannot be separated by identifier and only their order in the list distinguishes them.

3. **Does muting a two-in-one-out node say which input reaches the output?**
   Add an Alpha Over node (or a Mix Color node), select it, press M to mute it. In the Python console, run `[(l.from_socket.identifier, l.to_socket.identifier) for l in bpy.context.scene.node_tree.nodes["Alpha Over"].internal_links]`.
   *Negative result:* `internal_links` comes back empty, or lists more than one connection into the output — meaning muting does not give a single readable answer to which input passes.

4. **Is `is_inactive` ever set on a compositor socket?**
   Build a Switch node with both image inputs fed by different Image or Render Layers nodes, leave the Switch choice off, and run `[(s.name, s.is_inactive) for s in bpy.context.scene.node_tree.nodes["Switch"].inputs]`. Then tick the switch and run it again.
   *Negative result:* every socket reports `False` both times — meaning the flag is not used in the compositor and cannot be used to tell which input has no effect.

5. **Does the whole graph read the same when Blender is run without its window?**
   Save a file containing a Render Layers node → Hue/Saturation/Value node → Alpha Over node → Composite node. Then run Blender from a terminal with `--background` and a short Python script that prints, for every link in `bpy.data.scenes["Scene"].node_tree.links`, the four names `from_node.name`, `from_socket.identifier`, `to_node.name`, `to_socket.identifier`. Compare that printout with the same script run from the Python console inside the open window.
   *Negative result:* the two printouts differ — some links missing, renamed sockets, or an error — meaning the graph cannot be read the same way from outside the interface.

6. **At factor 0, does Alpha Over's output equal the background exactly?**
   Set up two clearly different Image nodes into an Alpha Over node, set Factor to 0, connect the result to a Viewer node, and compare the viewer pixels against the background image alone (swap the Alpha Over out for a direct link and look at the same pixels, or use a Mix Color node set to Difference between the two and check the result is black).
   *Negative result:* the Difference is not black — meaning a factor of 0 does not reduce the node to passing the background through, and "which came out" has no clean answer even at the extreme.

7. **Does Z Combine really mix per pixel rather than choosing one picture?**
   Feed two flat, differently coloured images into a Z Combine node with two Value/Vector inputs on the Z sockets arranged so that one Z is smaller over part of the frame and larger over the rest (for example by driving one Z from an Image Coordinates node through a Math node). Look at the result in a Viewer.
   *Negative result:* the whole frame shows one colour only — meaning the choice is made once for the picture, not per pixel, contrary to the manual's wording.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 4.5

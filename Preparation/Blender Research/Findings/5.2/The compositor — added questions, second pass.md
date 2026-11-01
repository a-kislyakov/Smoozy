# The compositor — added questions, second pass — Blender 5.2

**Sources consulted, and when.** Blender's manual for 5.2, Blender's Python API documentation for 5.2, the 5.2 release notes, and Cycles' own files for 5.2 — all read on 2026-07-30 (the date the copies were taken). Pictures from the 5.2 manual's `_images` folder, same date. I did not need Blender's requirements page for this question and did not use it. I did not use the web.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

Scope of this pass: question 8 only. Every other question on the sheet was left alone.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

### The reading I answered, and the other one

The question is ambiguous about what "followed forward" means, so I say plainly which reading I answered.

**The reading I answered:** can a particular picture still be identified downstream — after a node has altered its colours, and after a node has folded it together with a second picture — and, at the far side of that second node, can it be established which of the two inputs the result came from.

**The other reading:** can the *connection* be traced — i.e. can you walk the node graph and see that the wire out of node X reaches node Y. That reading is answered too, in the "Following the wire" part below, because it is cheap to answer and the two are easily confused.

I have split the answer into four parts. Each carries its own label; the two mandatory lines are at the end and cover the whole answer.

---

### 8a. Through a node that changes its colours

**Label: documented** (that such a node takes one picture in and gives one picture out), **and inferred** (that the picture's size and placement survive it unchanged).

The manual groups these nodes together and says what they do:

> "These nodes adjust the image's colors, for example increasing the contrast, making it warmer, overlaying another image, etc."
> — Blender manual, *Color Nodes*, https://docs.blender.org/manual/en/5.2/compositing/types/color/index.html (read 2026-07-30)

The category page lists the ones that only adjust colour: Brightness/Contrast, Color Balance, Color Correction, Exposure, Gamma, Hue Correct, Hue/Saturation/Value, RGB Curves, Tone Map (Blender manual, *Adjust*, https://docs.blender.org/manual/en/5.2/compositing/types/color/adjust/index.html, read 2026-07-30).

Each of these has a single picture in and a single picture out. Taking Hue/Saturation/Value as the example:

> "Inputs
> Image/Color
> Standard color input."

> "Outputs
> Image/Color
> Standard color output."
> — Blender manual, *Hue/Saturation/Value Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/adjust/hue_saturation.html (read 2026-07-30)

So the picture continues forward as one picture. What survives it, and what does not:

**What survives — size and placement.** The manual's rule for what area a node works over is:

> "Unless stated otherwise in their respective documentation pages, all other nodes use the following mechanism. One of the inputs of the nodes is designated as the Domain Input of the node, and the operation domain of the node is identical to the domain of that designated input. For many nodes, the domain input can be intuitively identified as the main input of the node, for instance, the domain input for the Filter node is the Image input."
> — Blender manual, *Compositor System*, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html (read 2026-07-30)

*The step I took:* none of the colour-adjust pages state otherwise, and each has an Image input as its main input, so the general rule applies and the output occupies the same domain — same pixel size, same transform — as the picture that went in. That is an inference, not a quoted statement.

**What does not survive — the pixel values.** That is what these nodes are for. The manual is explicit that a colour node's output can even leave the ordinary numeric range:

> "It is possible that this node will put out a value set that has values beyond the normal range, i.e. values greater than one and less than zero."
> — Blender manual, *Brightness/Contrast Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/adjust/bright_contrast.html (read 2026-07-30)

**Can you see it is still the same picture?** Blender 5.2 added a way to look at what is on a wire. The release notes record the addition:

> "Support for socket value inspection was added. (blender/blender@ffd88022c9)"
> — Blender 5.2 release notes, *Compositor*, https://developer.blender.org/docs/release_notes/5.2/compositor/ (read 2026-07-30)

The manual describes what it shows:

> "Socket inspection displays information about the image contained in a socket after the node tree has been evaluated.
> Rather than displaying the image itself, the inspection panel summarizes properties of the image, making it easier to verify intermediate results and debug node trees."

and lists the fields: Resolution, Transform, Sampling, Precision — with two conditions attached:

> "Socket values are only available after the node tree has been evaluated. Nodes that do not contribute to the final output are generally not evaluated and therefore have no inspection data available."

> "Pixel sockets are currently not supported."
> — Blender manual, *Compositor System*, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html (read 2026-07-30)

I opened the picture the manual places at that spot, `_images/compositing_socket-inspection.png` (Blender manual, same page, read 2026-07-30). **Documented, from the picture:** it shows a Render Layers node feeding a Pixelate node feeding a Viewer node, with an inspection panel hanging off the wire between Pixelate and Viewer. The panel is headed "Color" and reads: `Resolution: 1920x1080`, `Transformation: Identity`, `Sampling: • Interpolation: Bilinear • Extension: Clip`, `Precision: Half`, `Type: Image`. Note the picture shows one field the surrounding text does not list — `Type`.

So what you can follow forward past a colour change is the picture's *shape* — its resolution, its transform, its sampling and precision. Not its contents, and not any name or identity.

Two other nodes read properties off a picture from inside the graph:

> "The Image Info node outputs spatial and transformation information about an image in the compositor." — outputs Dimensions, Resolution, Location, Rotation, Scale.
> — Blender manual, *Image Info Node*, https://docs.blender.org/manual/en/5.2/compositing/types/input/image_info.html (read 2026-07-30)

> "The Levels Node reads the input color channels and outputs analytical values." — outputs Mean, Standard Deviation, Minimum, Maximum.
> — Blender manual, *Levels Node*, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/levels.html (read 2026-07-30)

Levels is the only thing I found in the sources that turns a picture's *contents* into numbers you can carry on with inside the graph.

**Label for the identity part: not in the sources.** Nothing in the manual or the API documentation says a picture in the compositor carries a name, label, tag, identifier or any other mark of where it came from. The record of that search is in "Where I looked".

**One documented way round it, and it is a technique rather than a feature.** The Mix Color page carries a worked method for marking a picture so it can be recognised later:

> "Nowadays, people watermark their images to identify them as personal intellectual property, for subliminal advertising of the author or hosting service, or simply to track their image's proliferation throughout the web.
> Blender provides a complete set of tools for you to both encode your watermark and to tell if an image has your watermark."

and, for reading the mark back:

> "When you see an image that you think might be yours, use the node tree below to compare it to your stock image (non-watermarked original). In this tree, the Mix node is set to Difference, and the Map Value node amplifies any difference. You can see how the original mark clearly stands out."
> — Blender manual, *Mix Color Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/mix_color.html (read 2026-07-30)

I opened the picture for that, `_images/compositing_types_color_mix_watermark-decode.png` (same page, read 2026-07-30). **Documented, from the picture:** two Image nodes labelled "Original" and "Marked" both feed a Mix node set to Difference; its Image output goes through a Map Value node (Size 25.000) into a Composite node, whose preview shows the word "Mark" in white on black. So the picture shows a mark being recovered by differencing two versions of the same image.

*What this does and does not establish:* it is a documented way to put a recognisable mark into a picture and get it back out. The manual does **not** say the mark survives a colour-changing node, and I have not claimed it does. The encoding example makes the mark "ever-so-slightly brighter", and a colour node that rescales or reshapes brightness would act on the mark as well as the picture. That is untested here.

---

### 8b. Through a node that takes two pictures in and gives one out

**Label: documented.**

Blender 5.2 has several such nodes, and each documents what it does with the two:

**Mix Color.**
> "Factor — Controls the amount of mixing between the A and B inputs."
> "A/B — The two inputs that are mixed together."
> "Result — Output the result of the mix using the data type selected."
> — Blender manual, *Mix Color Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/mix_color.html (read 2026-07-30)

**Alpha Over.**
> "The Alpha Over node composites two images by layering the Foreground image over the Background, using standard alpha blending operations."
> — Blender manual, *Alpha Over Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/alpha_over.html (read 2026-07-30)

**Depth Combine.**
> "The Depth Combine node combines two images based on their depth maps. It overlays the images using the provided depth values to detect which parts of one image are in front of the other."
> — Blender manual, *Depth Combine Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/depth_combine.html (read 2026-07-30)

**Switch.**
> "The Switch node outputs one of two inputs depending on a condition."
> — Blender manual, *Switch Node*, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/switch.html (read 2026-07-30)

**Split.**
> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line."
> — Blender manual, *Split Node*, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/split.html (read 2026-07-30)

In every one of them, two pictures go in and one comes out. What can be followed forward past such a node is, again, the *shape* rather than the identity — and here the manual says outright which of the two inputs decides that shape:

> "Each input in the node has the so called Domain Priority property, the operation domain of the node is the domain of the non single value input with the highest domain priority."

> "if the Foreground input is bigger than the Background input in the Alpha Over node, the output will be clipped to the Background input because it is the domain input"

> "The Alpha Over node currently does not support changing the domain priority for its inputs, so as a workaround, one can use a Mix node to achieve the desired behavior, noting that the first Image input in the Mix node has the highest domain priority"
> — Blender manual, *Compositor System*, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html (read 2026-07-30)

So after an Alpha Over, the output's size and placement are the Background's; after a Mix, they are the first Image input's. That is a documented, one-directional inheritance: one named input's shape comes out the far side, the other's does not.

The manual also states what happens to the part of the other picture that does not reach:

> "The nodes only consider the area of the input images that overlap the operation domain and ignores the rest of the images. If an input image doesn't completely overlap the operation domain, the rest of the operation domain for that input will be assumed to be a zero value, a zero vector, or a transparent zero color depending on the type."
> — same page, read 2026-07-30

---

### 8c. Can it be told which of the two came out the far side?

**Label: documented for the specific nodes below; not in the sources as a general facility.**

There is no general mechanism — no output, no flag, no marker anywhere in the compositor that reports "this result came from input A". What there is instead is a documented rule per node, from which you can work out the answer:

**Switch — yes, one of the two comes out whole and untouched, and the condition says which.**
> "Switch — Determines which of the two inputs below will be passed through."
> "False — Is passed through when the switch is set to false."
> "True — Is passed through when the switch is set to true."
> "Output — One of the two inputs without any modifications."
> — Blender manual, *Switch Node*, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/switch.html (read 2026-07-30)

The API says the same in one line:
> "Switch between two images using a checkbox"
> — Blender Python API, *CompositorNodeSwitch*, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeSwitch.html (read 2026-07-30)

The 5.2 release notes add that when the condition is fixed, the losing branch is not even computed:
> "The compositor now supports lazy evaluation of Switch nodes whose condition is known statically, that is, not connected or connected directly to a Group Input socket. Lazy Evaluation means nodes that are not needed are not executed to save execution time."
> — Blender 5.2 release notes, *Compositor*, https://developer.blender.org/docs/release_notes/5.2/compositor/ (read 2026-07-30)

**Mix — yes at the two extremes of the Factor, and the manual states which socket is which.** The Mix Color page itself does not say which end of the Factor gives which input, but the Luminance Key page does, in an aside:
> "(recall the Mix node uses the top socket where the factor is 0.0, and the bottom socket where the factor is 1.0)"
> — Blender manual, *Luminance Key Node*, https://docs.blender.org/manual/en/5.2/compositing/types/keying/luminance_key.html (read 2026-07-30)

Between those extremes the result is a blend, and the sources give no way to say which of the two "came out" — because both did, mixed.

**Depth Combine — yes, per pixel, by a stated rule.**
> "If both depth values are equal, it will use the foreground image. Whichever depth value is less decides which image pixel is used."
> — Blender manual, *Depth Combine Node*, https://docs.blender.org/manual/en/5.2/compositing/types/color/depth_combine.html (read 2026-07-30)

That node also emits the depth it settled on:
> "Depth — The combined depth, which allows you to thread multiple Depth Combine nodes together."
> — same page, read 2026-07-30

**Inferred:** comparing that combined Depth output against the two input depth maps tells you, pixel by pixel, which input won — wherever the two input depths differ. *The step I took:* the manual says the smaller depth decides the pixel and the node outputs the combined depth; so the output depth equals whichever input depth was smaller, and matching it back against Depth A and Depth B identifies the winner. Where the two depths are equal the comparison cannot separate them, though the quoted rule says the foreground is used there. The manual does not describe this comparison; I put it together from the two quoted lines.

**Alpha Over — no, not in general.** The page describes layering and blending by alpha, and gives a Factor for the foreground's opacity ("from 0 (fully transparent) to 1 (fully opaque)"). Nothing on the page says which input a given output pixel came from, and there is no second output that reports it. The one thing that *is* documented is the shape inheritance quoted in 8b: the output is clipped to the Background, because Background is the domain input.

**Split — yes, by where it sits.**
> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line. This is useful for comparing two versions of an image, such as before and after a color correction, compositing effect, or render adjustment."
> "Position — A 2D vector that defines the location of the split line in normalized coordinates."
> "Image — The first image input, displayed on one side of the split."
> "Image — The second image input, displayed on the opposite side of the split."
> — Blender manual, *Split Node*, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/split.html (read 2026-07-30)

> "Combine two images for side-by-side display. Typically used in combination with a Viewer node"
> — Blender Python API, *CompositorNodeSplit*, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeSplit.html (read 2026-07-30)

Both come out, each on its own side of a line you place, so you can tell them apart by position. The manual does not say which input takes which side.

---

### 8d. Following the wire (the other reading)

**Label: documented.**

The graph itself can be walked from outside Blender's window. Every connection records both ends:

> "NodeLink — Link between nodes in a node tree"
> "from_node (readonly) — Type: Node | None"
> "from_socket (readonly) — Type: NodeSocket | None"
> "to_node (readonly) — Type: Node | None"
> "to_socket (readonly) — Type: NodeSocket | None"
> "is_muted — Link is muted and can be ignored (default False)"
> "is_valid — Link is valid (default False)"
> — Blender Python API, *NodeLink*, https://docs.blender.org/api/5.2/bpy.types.NodeLink.html (read 2026-07-30)

So the route a picture takes through the tree — into the colour node, out of it, into the A or B socket of the mixing node — is fully readable as data. What is *not* readable that way is the picture itself: nothing in the API documentation exposes the evaluated image sitting on a socket.

The documented way to get an intermediate picture out where you can look at it is the File Output node:

> "The File Output node writes one or more images to disk during rendering or compositing."
> "This node can be used to: ... Save intermediate results from anywhere in the node tree."
> "Use the File Output node to save raw passes or intermediate steps without overwriting the main render result."
> — Blender manual, *File Output Node*, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html (read 2026-07-30)

Its input socket names become part of the written file's name:
> "For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."
> — same page, read 2026-07-30

**Inferred:** that gives you a label you choose yourself, attached to a *wire* rather than to a picture, that ends up in the written file's name. *The step I took:* the manual states the file name is built from the socket name; naming the socket after the branch you are watching therefore identifies which branch wrote the file. The manual does not present this as an identification technique.

One caution about comparing pictures exactly. Two statements in the sources bear on it:

> "Three types of data exist, all of which are stored in half precision formats"
> — Blender manual, *Compositor System*, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html (read 2026-07-30)

> "compositor_precision — The precision of compositor intermediate result (default 'AUTO')
> AUTO — Auto – Full precision for final renders, half precision otherwise.
> FULL — Full – Full precision."
> — Blender Python API, *RenderSettings*, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (read 2026-07-30)

I am not reconciling those two; I report both. Anyone planning to identify a picture by exact pixel values downstream should know the stored precision is a setting, and differs between a final render and everything else.

---

### What a running Blender showed

**LABEL: observed.** Blender 5.2.0 LTS, run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-06**, except for the entry 12 card paragraph below, which was got on the same machine on **2026-08-07** once its graphics card had been repaired, and which names its own driver version. Every run used `-b --factory-startup`, with Blender pointed at a settings folder of its own made beforehand under `/root/compositor/`, never at the machine's real one. The machine was shared with other topics while this ran. The whole procedure, every script in full and all of what the machine printed are in `Audits/Blender Research/The compositor/Hand tests — added questions.md`; the entry numbers below are that document's. **Everything down to entry 14 settles Linux only.** The four tests that need a screen — entries 15 to 18 — were run on the Creator's Mac and are written up at the end of this section. No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Every pixel comparison below rests on a control (entry 1) that came back `MAXDIFF render vs source 0.0` and `MAXDIFF render vs same render again 0.0`, so a difference of zero means the pictures match. **Pictures were compared as pictures, never as files.**

**Which input decides the output size of Alpha Over and of Mix (entry 9).** A 32 by 32 picture and a 128 by 128 picture, rendered at 128 by 128, reading the area of the written picture whose alpha is above zero:

> `AlphaOver(bg,fg)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `AlphaOver(bg,fg)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)`
> `Mix(A,B)  first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)`
> `Mix(A,B)  first input = picLarge(128), second = picSmall(32) -> alpha covers x 48..79, y 48..79  (32 by 32)`

Alpha Over follows its Background input, as the domain-priority passage quoted above says. The Mix node came back at the smaller of the two sizes both ways round in this version, where 4.5, 5.0 and 5.1 gave the larger size when the larger picture was in the A input.

*[The second of those two sentences is **superseded on 2026-08-07 by entry 20 below**, which is where to read. The four lines above are right and were reproduced exactly; the sentence drawn from them is not. There is no difference between the versions here. The Mix node's frame follows its first colour input in all four, at every Factor. This entry never wrote its Factor down, and a freshly added Mix node starts at Factor 1.0 in 5.2 and at 0.5 in the other three — so this entry read 5.2 at one setting and the other three at another. Leaving the Factor alone, exactly as this entry did, gives these same two Mix lines back to the letter in 5.2 and the other pattern in the other three. The Alpha Over sentence stands untouched, and the domain-priority passage it rests on now covers the Mix node too.]*

*[Disagreement recorded by the citation check, 2026-08-06. The Mix half of this entry was run again on the same rented Linux machine on 2026-08-07 and came back differently: with the large picture on the first input, 5.2 gave `alpha covers x 0..127, y 0..127  (128 by 128)`. Run again with the Factor swept, 5.1 and 5.2 behaved identically, and the size that came out depended on the Factor rather than on the version. Both runs are written out in full in `Audits/Blender Research/The compositor/Citation check/Added questions — Opus 5.md`. Neither result is overwritten and neither is picked. The Alpha Over half stands on both runs; the Mix half goes back to the hand tests to be run again, with the Factor stated.]*

*[**Settled 2026-08-07** — see entry 20 below. It was run again, with the Factor stated and swept and the frame read straight off the socket. Neither of the two runs above was wrong about its numbers; both drew a conclusion the numbers did not carry, because both read the area whose alpha is above zero and that is a reading of the alpha rather than of the frame. The frame follows the first colour input in all four versions at every Factor. The version difference this entry reported was a starting Factor of 1.0 in 5.2 against 0.5 in the other three.]*

**Mix at the ends of its factor (entry 3).** This is the Luminance Key page's aside, put to the machine, with the first picture on A and the second on B:

> `ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `ShaderNodeMix factor=1.0  maxdiff vs picA=1.0  maxdiff vs picB=5.960464477539063e-08`

At factor 0 the output matches A to the last figure; at factor 1 it matches B to within one step of a floating-point number.

**The Switch node leaves the chosen picture alone (entry 4).**

> `Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.0`
> `Switch=True  Off=picA On=picB  maxdiff vs picA=1.0  maxdiff vs picB=0.0`

**But its sockets are not the ones the manual names (entry 2).**

> `PRESENT CompositorNodeSwitch label: Switch`
> `   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]`
> `   OUT [('Image', 'Image', 'RGBA')]`

The manual page quoted above names them `Switch`, `False`, `True` and `Output`. The running 5.2 offers `Switch`, `Off`, `On` and `Image`.

**Does a faint mark survive a colour grade (entry 13).** A copy of one picture with a 16 by 16 block at its centre raised by 0.02, put through a Hue/Saturation/Value node with the hue moved to 0.8 and the saturation to 1.7:

> `BEFORE any colour change: biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `AFTER the colour change  : biggest difference inside the mark = 0.020000, outside it = 0.000000`
> `MARKED-AND-GRADED against the UNMARKED ORIGINAL: inside the mark = 0.607381, outside it = 1.000000`

So compared against the same picture put through the same grade, the mark is still there at its full strength and nothing else has moved. Compared against the unmarked original, which is the comparison the documented technique describes, the difference outside the mark is larger than the difference inside it.

**Socket inspection from a script (entry 12).** After a render, every property name on a colour node's output socket:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`
> `ANY NAMING INSPECTION, RESOLUTION, TRANSFORM, PRECISION, SAMPLING, VALUE OR IMAGE: ['default_value', 'hide_value']`
> `default_value on the output socket: <bpy_float[4], NodeSocketColor.default_value>`

Nothing there names an inspection, a resolution, a transform, a precision or a sampling. `default_value` on a colour socket holds four numbers, which is a socket's own fixed colour rather than the picture that travelled through it. This is with no window; whether anything appears once a window exists is still open.

**The processor against the card (entry 12).** Every run on this machine printed these lines first:

> `libEGL warning: egl: failed to create dri2 screen`
> `EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).`

then:

> `compositor_device present: True value: GPU`
> `compositor_precision present: True value: AUTO`
> `MAXDIFF between the processor's picture and the card's picture: 6.854534149169922e-07`

So the setting starts at `GPU` in this version. This machine has no display server and `nvidia-smi` on it would not start (`Failed to initialize NVML: Driver/library version mismatch`), so whether the run marked `GPU` really used the card is not settled. What is settled is that changing that setting changed the picture, by about seven parts in ten million — which bears directly on identifying a picture by its exact values.

**The card was repaired, and the card half was run again on 2026-08-07 (entry 12).** On 2026-08-06 the machine's graphics card could not be reached: its kernel driver and its driver library had drifted apart. It has since been repaired. Everything in this paragraph was got on **2026-08-07**, on that same rented Linux machine — **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and still no display server at all** — with Blender 5.2.0 LTS, `-b --factory-startup`, and a settings folder of its own made beforehand under `/root/compositor/`. The card now answers:

> `NVIDIA-SMI 580.173.02             Driver Version: 580.173.02     CUDA Version: 13.0`
> `NVIDIA GeForce RTX 4080 SUPER, 580.173.02, 16376 MiB`

and no run printed an EGL line of any kind. **The paragraph just above is superseded**: with the card unreachable, that run marked `GPU` cannot have been the card's work. The setting still starts at `GPU` in this version, and on a working card that now means a 5.2 file opened with nothing changed will composite on the card.

The same tree — a colour node and an Alpha Over at factor 0.5 — rendered once on each device, the two pictures loaded back and compared pixel by pixel:

> `DEVICE CPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `DEVICE GPU: rendered, corner sample [0.175, 0.5, 0.628175, 1.0]`
> `MAXDIFF between the processor's picture and the card's picture: 6.258487701416016e-07`

**The run marked `GPU` really does use the card.** A heavier tree — a 1024 by 1024 picture through sixty Blur nodes, rendered at 2048 by 2048 — was run twice, one device to a Blender, while the card was asked over and over which processes were on it. Only this Blender's own process was counted, because the machine is shared with other topics' tests. Set to the processor, then set to the card:

> `HEAVY PROCESSOR CORE-SECONDS CPU 38.300`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 115`
> `TIMES THIS PROCESS WAS ON THE CARD: 0`
> `HEAVY PROCESSOR CORE-SECONDS GPU 3.370`
> `TIMES THE CARD WAS ASKED WHO IS ON IT: 47`
> `TIMES THIS PROCESS WAS ON THE CARD: 12`
> `MOST CARD MEMORY THIS PROCESS HELD: 150 MiB`

and the two heavy pictures, loaded back and compared pixel by pixel:

> `HEAVY MAXDIFF processor against card: 3.5762786865234375e-07`

So the setting really moves the work onto the card, and the picture the card makes matches the processor's to about four parts in ten million. **This is the sharpest form of the point above about identifying a picture by its exact values**, and it now has a real cause behind it: two pictures made from the same tree on the two devices are not bit-for-bit equal, so an identity test finer than a millionth would fail across them, and any test coarser than that still holds. In this version it bites hardest, because the device starts at the card and a machine without one will fall the other way.

**Does the written file's name follow the File Output socket's name (entry 10).**

> `file_output_items after: ['MyBranchName']`
> `file_name is '{blend_name}'`
> `FILE OUTPUT input sockets: [('MyBranchName', 'Item_1'), ('', '__extend__')]`
> `WHAT LANDED IN THE FILE OUTPUT FOLDER: ['Unsaved.exr']`

The socket's name did not appear in the written file's name. The node's file-name setting was left at its factory value, `{blend_name}`, which resolved to the unsaved file's name. Blender's own description of that setting says only "The base name of the file. Other information might be included in the final file name depending on the node options", so whether a token standing for the socket's name exists was not settled — and the inferred step in 8d, that naming the socket identifies which branch wrote the file, does not hold as the node comes.

**Two further things this document leans on, settled the same day.** The whole graph reads back out of a saved file with no window, every link with both its nodes and both its sockets (entry 11). And a link cannot be walked across a node group boundary — nothing in the outer tree names a node inside the group (entry 10).

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

which is the same build as the Linux one, put together on a different machine. Every run was pointed at a settings folder of its own, made before it was used, at `/Users/artist/Documents/Smoozy hand tests/The compositor/res-5.2`, never at the Mac's real one. The Mac was being used by other topics' Blender runs at the same time, so nothing here says how long anything took or how much memory anything used. These four tests need a screen, so the rented Linux machine, which has no display server, could not run them at all. The whole procedure is in the hand-test document named above, entries 15 to 18.

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

**The Viewer node's image fills when a window is open, and stays empty when there is none (entry 16).** The same script, one render of 64 by 64, run twice. With no window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`

With a window:

> `VIEWER IMAGE before the render: size=(256, 256) pixels=262144  min=0.0 max=0.0`
> `VIEWER IMAGE after the render: size=(64, 64) pixels=16384  min=0.0 max=1.0`

The Mac's no-window run matches the Linux no-window run (entry 10) line for line. The two machines agree on the no-window case; there is no Linux counterpart to the windowed case.

**Socket inspection stays out of reach of a script when a window is open (entry 17).** This answers one of the items in this document's *Not covered* list. The same list of property names came back from both runs, the one with a window and the one without:

> `OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']`

That is the same list the Linux machine gave with no window (entry 12). Both machines agree, and none of the names is an inspection, a resolution, a transform, a precision or a sampling. So the absence in the API documentation is matched by an absence in the running program, with a window and without. What the panel *draws* when a socket is hovered is still untested, because a script cannot hover a socket.

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

The two machines agree. This Mac has Apple's own graphics and no second card, and the compositing device was left wherever 5.2 starts it, which is the card.

**One thing about ending a Blender that has a window.** The scripts were written to close Blender with `bpy.ops.wm.quit_blender()`. On this Mac that line on its own did not end it: the script's printing all appeared, and the process was still running when a limit of my own ended it. The same script with `sys.exit(0)` added after that line ended in about a second and printed the same thing. Both are recorded in entry 19 of the hand-test list.

### Not covered

- Whether any mark encoded into a picture (the watermark technique) survives a colour-changing node. The sources describe encoding and decoding a mark; they say nothing about it surviving a grade, a curve, or a tone map. **Partly answered by entry 13 above:** a faint mark of 0.02 came through a Hue/Saturation/Value grade at its full strength when the comparison was made against the same picture put through the same grade. What is still open is the comparison the manual actually describes — against the unmarked original — where the difference outside the mark came out larger than the difference inside it. *[Changed by the citation check, 2026-08-06: the bullet said "untested here" while a test of it sits above.]*
- ~~Whether socket inspection can be read from a script.~~ **Settled by a running Blender.** I found no API for it — see "Where I looked" — and an absence in the API documentation is not proof it is absent from the program, so it was put to the machine. No property on a socket names an inspection, a resolution, a transform, a precision or a sampling, on either machine, with a window and without (entries 12 and 17). What the panel draws when a socket is hovered is still untested, because a script cannot hover a socket.
- What "Pixel sockets are currently not supported" excludes from socket inspection. The manual states it without explaining which sockets those are.
- Whether the Split node's first Image input takes the left or the right side. Not stated.
- Anything about numerical exactness — whether a colour node with a neutral setting returns pixels bit-for-bit unchanged. The sources describe what each node does, not what it computes to the last digit.
- Nodes that take two pictures in and give one out that I did not read: the keying and matte nodes (Difference Key and similar), and node groups with two image inputs. I read the five named above.
- Question 8 asks about two pictures. Nodes taking three or more (Index Switch, Menu Switch) were not examined beyond noting they exist.

### Outside the interface

**LABEL: observed, on Linux and on macOS.** On the rented Linux machine named above, on 2026-08-06, Blender 5.2.0 LTS with `-b` opened a saved file and printed the whole graph — every link with both its nodes and both its sockets, the Switch node's stored choice, and each node's internal links. Every pixel result in *What a running Blender showed* was got the same way, with no window: the node rules do hold there, and that is now a result rather than a reading. Socket inspection was tested and is not reachable from a script with no window — no property on a socket names an inspection, a resolution, a transform, a precision or a sampling. The Viewer node's image exists but stayed at 256 by 256 with every value zero across a 64 by 64 render.

**All three were then put to a Blender with its window open**, on the Creator's Mac on 2026-08-07, since the rented Linux machine has no display server. Two of the three came back the same either way and one did not. The graph read the same with the window open as with none — the same nodes, links, socket identifiers and stored choice (entry 15). Socket inspection stayed out of reach: the same list of property names came back with a window as without (entry 17). The Viewer node was the one that changed: with a window, its image came back at 64 by 64 — the size of the render — with values running up to 1.0 (entry 16). And four renders made twice, once with a window and once without, matched to the last figure (entry 18). So the reading half and the node rules hold whether Blender is driven from outside its own window or not; only the Viewer's preview turns on there being a window.

**Choosing the graphics card, and having the choice acted on, both work with no window.** On 2026-08-07, on the same machine with its card repaired and its driver at 580.173.02, a Blender started with `-b` and its compositing device set to the card really did the compositing there — the card named that Blender's own process and it held 150 MiB while it worked, where the same run set to the processor never appeared on the card at all (entry 12). Neither the setting nor its effect needs a window. That matters more here than in the other three versions, because 5.2 starts at the card.

The reading from the sources, unchanged, follows.

Mixed, and the distinction matters.

- **The node rules hold regardless.** Which input decides the domain, what the Switch passes through, what Depth Combine picks per pixel — these are properties of the nodes, not of the window. Nothing in the sources ties them to a person clicking.
- **Socket inspection is an interface feature and does not hold outside it.** The manual describes it as a panel shown in the node editor, the release note shows it as a UI addition, and I found no Python API for it (the only "socket inspection" mentions in the 5.2 API documentation are `inspection_index` on two Geometry Nodes classes, which is a different thing). Its stated condition — "Socket values are only available after the node tree has been evaluated. Nodes that do not contribute to the final output are generally not evaluated" — is a further limit even inside the interface.
- **The Viewer node is also an interface path.** The manual describes it as feeding the backdrop and the Image Editor: "It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output" (Blender manual, *Viewer Node*, https://docs.blender.org/manual/en/5.2/compositing/types/output/viewer.html, read 2026-07-30).
- **The parts that do work outside the interface** are the File Output node — "writes one or more images to disk during rendering or compositing" — and reading the graph through `NodeLink`. Both are quoted above.

The sources do not discuss driving the compositor from a command line at all, so beyond the above I cannot say more without guessing.


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three ran with no window at all, on the rented Linux machine named in that sub-section, and all three reached what they were after. **A node's output frame can be read from outside the interface.** Feeding a Mix node's output into an Image Info node and rendering its `Dimensions` output gives the frame as a pair of numbers, with no window and no hovering — which is worth setting beside the socket-inspection panel, which a script cannot reach at all. **Which node types a compositor tree will accept can be swept with no window**, and the counts match those got the same way on 2026-08-06. **And `is_inactive` reads the same with no window before a render and after one**, so nothing in that reading waits on the tree having been drawn anywhere. None of the three needed a screen, so none of them waited for the Mac; and none of the three has yet been run on the Mac, so on that side they are owed rather than answered.
### Operating system

**LABEL: observed for Linux, and now partly for macOS; still open for Windows.** Every result in *What a running Blender showed* above was got on the rented Linux machine named there, and settles Linux alone. The card result added on 2026-08-07 was got on that same Linux machine once its graphics card had been repaired, and settles Linux alone in the same way.

**Four of those results are now settled on macOS as well**, from *What a running Blender showed on the Creator's Mac* above, got on 2026-08-07 on a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Where the same test ran on both machines the two agreed, every time and to the last figure: reading the whole graph out of a saved file with no window; the four Alpha Over and Switch renders; the Viewer node's image staying empty with no window; and every property name on a socket. Nothing came out differently on the two operating systems. The rest of the Linux tests are still owed on the Mac and until they are run they settle Linux alone. On the Mac the card question is a different question again, because that machine has Apple's own graphics and no second card — which matters more here than in the other three versions, since 5.2 starts at the card. **Windows is untried: no Windows machine has run it**, so that silence is a limit of what was run rather than a finding. One thing the runs do settle about the question raised at the end of this section: on a working card, the processor's picture and the card's picture differ by about six parts in ten million on a small tree and about four on a heavy one, so exact pixel values are not the same across device settings even within one machine.

The reading from the sources, unchanged, follows.

The sources do not state any operating-system difference in what these nodes do or in how a picture can be identified. The only operating-system-specific statements I found anywhere near this are about the GPU compositor's performance and stability, not its results:

> "TDR on Windows
> Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

> "Images with resolution larger than 8k are not supported and are cropped/scaled."
> — Blender manual, *Limitations*, https://docs.blender.org/manual/en/5.2/compositing/limitations.html (read 2026-07-30)

Compositing can also be run on either device:
> "compositor_device — Set how compositing is executed (default 'GPU') — Type: Literal['CPU', 'GPU']"
> — Blender Python API, *RenderSettings*, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (read 2026-07-30)

Whether CPU and GPU compositing produce identical pixels is not stated in the sources, and that bears directly on identifying a picture by its exact values. ~~I am not guessing either way.~~ **Answered by entry 12 above**, not by the sources: they are not identical, and the gap is about six parts in ten million on a small tree and four on a heavy one. *[Changed by the citation check, 2026-08-06: this closing line was left standing after the test that answered it, and the paragraph above it in this same section already reported the answer.]*

---


**Brought up to date 2026-08-07, by the three tests the citation check sent back (entries 20, 21 and 22 above).** All three were run on the rented Linux machine named in that sub-section, and **all three settle Linux alone**. They are owed on the Creator's Mac in exactly the way the other Linux-only entries are, and they are not blocked by any missing equipment — none of them needs a screen. **Windows stays untried because no Windows machine has run it**, so that silence is a limit of what was run and not a finding. One thing about these three is worth saying plainly against this line: two of them overturned a difference that had been recorded as a difference between Blender versions, and in both cases the real cause was a control left at its starting value rather than anything about the version. That is a caution about reading version differences out of any test on this list that does not write its settings down; it is not a caution about operating systems, and nothing here suggests the operating system moved any of it.
## Where I looked

**Files searched in bulk** (whole-tree text search, not file-by-file opening): `Blender Sources/manual_5.2` (whole tree, and the `compositing/` and `render/` subtrees specifically), `Blender Sources/api_5.2` (whole tree), `Blender Sources/release_notes_5.2`, `Blender Sources/cycles_5.2` (whole tree).

**Terms searched, and what came back:**

| Term | Where | Result |
| --- | --- | --- |
| `provenance`, `identity`, `identif*`, `which input`, `came from`, `originat*`, `traceab*` | manual `compositing/` | Only unrelated hits — "identity transformation", "identifier" for text-node socket names, the Viewer node's shortcut number, and the watermark passage on the Mix Color page. No image-identity concept. |
| `watermark` | manual `compositing/` | Only the Mix Color page. Quoted above. |
| `fingerprint`, `checksum`, `hash` | manual `compositing/` | **Zero hits.** |
| `unique` | manual `compositing/` | Only text-node socket names and Radial Tiling's per-segment ID. Nothing about images. |
| `tag`, `label` | manual `compositing/` | Only node labels, mask data-block names, menu item labels. Nothing attached to an image. |
| `image name`, `name of the image` | manual `compositing/` | Only the File Output node's file-naming. |
| `(image\|picture)` near `(id\|identifier\|tag\|marker)` | manual `compositing/` | **Zero hits.** |
| `metadata`, `stamp` | manual `compositing/` | Two hits only: the String to Image node's description, and "Cryptomatte sidecars (metadata files) are not supported." Nothing about metadata riding along a wire. |
| `cryptomatte` | manual (whole tree), API (whole tree) | Found and read the Cryptomatte node page. **Ruled out:** it isolates objects, materials and assets *within a render* using a dedicated render pass — "The Cryptomatte node uses a Cryptomatte image to create a mask for one or more objects or materials." It is not a way of telling two pictures apart in the node graph, and the page lists as a limitation that "The Cryptomatte node cannot be used in node groups." |
| `socket inspect*`, `inspection` | API 5.2 (whole tree) | Only `inspection_index` on `GeometryNodeRepeatOutput` and `GeometryNodeForeachGeometryElementOutput`. **Nothing for the compositor.** This is the basis for saying socket inspection is not documented as scriptable. |
| `viewer node` | API 5.2 (whole tree) | Only operators (`node.link_viewer`, toggling viewers), the Split node's description, the backdrop setting, and the spreadsheet's viewer-node source. No API for reading a viewer's pixels was found on this search. |
| `compositor_device`, `compositor_precision`, `compositor_denoise*` | API 5.2 | Found on `RenderSettings`. Quoted above. |
| `compositing device`, `compositor device`, `GPU compositor` | manual `render/`, `compositing/` | Found the Cycles and EEVEE performance pages and the compositing Limitations page. |
| `background mode`, `--background`, `command line` | manual `compositing/`, `render/output/` | One hit only, about the animation player. **The compositing documentation says nothing about running headless.** |
| `factor is 0`, `factor of 0`, `fully (the )?(first\|second)` | manual `compositing/`, `render/shader_nodes/` | Turned up the decisive line on the Luminance Key page about the Mix node's top and bottom sockets. Quoted above. |
| `composit*` | Cycles 5.2 files (whole tree) | **Ruled out.** Only panel plumbing in `ui.py` (the compositor performance panels are imported from Blender's own render properties, not defined by Cycles), one property description in `properties.py` about rendering glass for compositing, and kernel comments about compositing shadow catchers. Cycles' files contain nothing about the compositor node graph, so they cannot answer this question. |

**Pages opened and read in full:** the compositing section's `Compositor System`, `Using The Compositor`, `Limitations`; the node pages for Mix Color, Alpha Over, Depth Combine, Hue/Saturation/Value, Brightness/Contrast, Switch, Split, Levels, Image Info, String to Image, Viewer, File Output, Enable Output, Cryptomatte; the `Color Nodes` and `Adjust` category pages; the Luminance Key page (reached by search). In the API: `NodeLink`, `RenderSettings`, `CompositorNodeAlphaOver`, `CompositorNodeSwitch`, `CompositorNodeZcombine`, `CompositorNodeSplit`. Release notes: the 5.2 compositor page in full.

**Pictures opened:** `_images/compositing_socket-inspection.png` and `_images/compositing_types_color_mix_watermark-decode.png`, both from the 5.2 manual. What each shows is written into the answer above.

**Also noted while searching, and left alone as outside this question:** the compositing folder in 5.2 has no `composite.txt` page — its output nodes are Enable Output, File Output, Group Output, Viewer and Warning — and the Compositor System page speaks of Group Output and Viewer nodes as the output targets. That bears on other questions on the sheet, not this one, and I did not pursue it.

---

## Things a person with Blender open could settle in a few minutes

**All eight of these have been carried out on Linux**, on 2026-08-06, and their results are in *What a running Blender showed* above — test 1 and test 6 as far as a script can carry them, since a script cannot hover a socket. Three more were run on the Creator's Mac on 2026-08-07 and are in *What a running Blender showed on the Creator's Mac*: the graph read the same with Blender's window open as with none, the Viewer node's image filled only when there was a window, and a picture rendered with a window matched one rendered with none exactly. What is left is written out in full in `Audits/Blender Research/The compositor/Hand tests — added questions.md`:

- **What the socket inspection panel draws when a socket is hovered.** A script cannot hover a socket, so this stays untested. What is settled is that the data behind it does not become reachable once a window exists.
- **Most of the tests already run, run again on macOS.** Four have been; the rest settle Linux only.
- **Windows.** No Windows machine has run it, so none of it has been run.

The original list follows, kept as the researcher wrote it so a reader can see what was asked for.

**1. Does socket inspection show anything for a colour-changing node's output?**
Open a compositor node tree with a Render Layers node → a Hue/Saturation/Value node → the output node. Hover the wire leaving the Hue/Saturation/Value node and read the inspection panel.
*Negative result:* no panel appears, or it appears empty, or the Resolution/Transform it reports differ from the same reading taken on the wire going *into* the Hue/Saturation/Value node. Any of those contradicts the inference in 8a that a colour node passes size and placement through unchanged.

**2. Which input decides the output size of Alpha Over and of Mix?**
Feed two pictures of clearly different pixel sizes (say 1920×1080 and 640×360) into an Alpha Over node, once with the large one as Background and once as Foreground. Read the Resolution reported by socket inspection on the output wire each time. Repeat with a Mix node, swapping which picture goes into the first Image input.
*Negative result:* the reported output resolution follows the Foreground rather than the Background on Alpha Over, or follows the second rather than the first Image input on Mix. Either would contradict the domain-priority statements quoted in 8b.

**3. Does the Mix node's Factor at its extremes give back exactly one input?**
Set a Mix node to blend mode Mix with Clamp Factor on, feed two visibly different pictures into A and B, and set Factor to 0.0, then to 1.0. Write each result out with a File Output node and compare against the two source pictures.
*Negative result:* at Factor 0.0 the written file does not match the top input (or at 1.0 does not match the bottom), or matches only approximately. That would contradict the Luminance Key page's aside.

**4. Does a Switch really leave the chosen picture untouched?**
Put two pictures into a Switch node's False and True inputs. Write the output to a file with a File Output node with the switch off, then on. Compare each written file, pixel for pixel, against the corresponding source picture.
*Negative result:* the written file differs from its source picture in any pixel. That would contradict "One of the two inputs without any modifications."

**5. Does a faint watermark survive a colour grade?**
Build the encode setup from the Mix Color page (mark → RGB to BW → Map Range to about a tenth intensity → Mix set to Add). Then put a colour node after it — try Brightness/Contrast with contrast raised, and separately Tone Map. Then run the decode setup (Difference against the un-marked original, amplified) on the result.
*Negative result:* the mark does not stand out in the decoded picture after the colour node. This is the test the sources cannot answer, and it decides whether marking a picture is a usable way to follow it forward past a grade.

**6. Does socket inspection exist outside the interface?**
From a script, look for any property or method on a compositor node's sockets that returns the inspected resolution, transform, sampling or precision. Also check whether it returns anything when Blender is started without its window.
*Negative result:* such a property is found and returns data. That would contradict the conclusion in "Outside the interface" that socket inspection is an interface-only feature, which rests on an absence in the API documentation rather than on a positive statement.

**7. Do CPU and GPU compositing give identical pixels?**
Take one node tree containing a colour node and a two-in-one-out node. Render it once with the compositing device set to CPU and once set to GPU, writing both through a File Output node at full precision. Compare the two files.
*Negative result:* the two files differ. That would mean a picture cannot be identified by exact pixel values across machines with different device settings — which matters directly to any scheme for following a picture forward by comparing it.

**8. Does the written file's name follow the File Output socket's name?**
Add a File Output node, rename its input socket to something distinctive, connect a branch to it, and render one frame. Look at the file that lands on disk.
*Negative result:* the socket name does not appear in the file name. That would contradict the documented pattern `{Directory}/{File Name}{Socket Name}{frame number}.{extension}` and remove the only self-chosen label found in the sources.

---

2026-08-06 — Opus 5 (Claude Code) — The compositor, Blender 5.2

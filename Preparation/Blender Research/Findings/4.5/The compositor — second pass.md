# The compositor — Blender 4.5 — second pass

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, text pages and the `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All were taken on 2026-07-30, and that is the date read for everything quoted below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One disclosure, so the line above stands without an asterisk: the project's root `CLAUDE.md` was placed in front of me automatically by the tool I run in, before I did anything. I did not open it and I did not follow it. Its text is a signpost only — it names `Preparation/Groundwork/Start Here.md` and tells Blender researchers to stop, which is what I did. It contains no statement about how Blender behaves, so nothing in it could have coloured an answer below. I opened nothing it points to.

Only question 5 was assigned, and only question 5 is answered.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

### A note on the reading I took

This question has three parts and one of them is ambiguous, so I say up front how I read it.

"Can it be told apart from anything else there" I read as: **in a compositor node graph that may also contain still images, movie clips, masks, textures and so on, can the render be distinguished from those other sources?** That is the reading I answer at length.

The other possible reading is: **once the render is travelling along a link, does it carry any marker on itself that says it is a render?** I answer that too, separately, because it is answerable and the answers differ sharply.

"Nodes that only pass it along unchanged" I read as nodes whose documented job is to relay data without altering it — the Reroute node, and any node that has been muted. "Split it into more than one" I read as one source feeding more than one destination. The other possible reading of "split" is a node that separates an image into its components (Separate Color, Separate XYZ) or the compositor's own **Split** node; I checked both and say what I found, but they are not what I take the question to mean.

---

### 5a. What does it arrive as

**LABEL: documented**

A render enters the compositor through the **Render Layers node**, and it arrives as node output sockets — an Image socket, an Alpha socket, and one further socket per enabled render pass.

Exact words, Render Layers Node page:

> "Renders a View Layer and reads its Passes into the compositing node graph."

> "Inputs
>
> This node has no input sockets."

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

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html — read 2026-07-30.

The picture on that same page confirms the shape of the node. I opened `Blender Sources/manual_4.5/_images/node-types_CompositorNodeRLayers.webp`, which the page references as `[image: _images/node-types_CompositorNodeRLayers.webp - Render Layers Node.]`. It shows a node titled "Render Layers", with two output sockets on the right labelled **Image** (a yellow socket) and **Alpha** (a grey socket), and below them two controls: a data-block field holding a scene named "Scene", and a dropdown holding "ViewLayer". Label for what the picture shows: **documented** (it is part of the manual).

The passes page says the same thing from the other direction:

> "A Pass is a type of intermediate rendering information that's extracted as a separate image."

> "These images can be accessed using the Render Layers Node in the Compositor and combined in a custom way that replaces the standard one."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/render/layers/passes.html — read 2026-07-30.

What the data on those sockets *is*, in the compositor's own terms, is set out on the Compositor System page:

> "Compositing nodes operate on data that is either an image or a dimensionless single value. For instance, the Levels node outputs a single value, while the Render Layers node outputs an image."

> "Three types of data exist, all of which are stored in half precision formats:"
>
> "Float
>
> A signed floating-point number. Integer data is also stored as floats because no integer type exist."
>
> "Vector
>
> A 4D vector. …"
>
> "Color
>
> A 4D vector storing the Red, Green, Blue, and Alpha of the color. The color is free form and does not conform to a specific color space or alpha storage model …"

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30.

And an image in the compositor is not only pixels but a position, because:

> "The compositor is designed in such a way as to allow compositing in an infinite compositing space. Consequently, images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations. An identity transformation represents an image that is centered in space. The rectangular area occupied by an image in that space as defined by its transformation and size is called the Domain of the image."

Same page and address as above, read 2026-07-30.

One frame at a time:

> "The term Image may refer to a single picture, a picture in a numbered sequence of images, or a frame of a movie clip. The Compositor processes one image at a time, no matter what kind of input you provide."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/introduction.html — read 2026-07-30.

In Python, the node type that carries the render in is described as:

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

**Not covered.** These words say the render arrives as sockets on a node and that the data is Float, Vector or Color at half precision. They do not say what happens in memory, whether the render is copied or referenced, when in the render the sockets become populated, or what the sockets hold if the render fails part way. The manual's Render Layers page does not list which passes produce which socket names; the passes page lists the passes but not their socket names.

**Outside the interface.** The Render Layers page carries one note that is explicitly about the interface only: "The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty." (same page and address, read 2026-07-30). The viewport compositor is a thing that exists inside Blender's window. The sources do not say anything that limits the Render Layers node's Image, Alpha and pass sockets when Blender is driven from outside its own window, and they do not state a difference. I searched the manual and API for "background mode" and found only six hits, none about the compositor (`manual_4.5/advanced/scripting/security.txt`, `manual_4.5/advanced/scripting/addon_tutorial.txt`, `manual_4.5/advanced/command_line/arguments.txt`, `api_4.5/bpy.utils.txt`, `api_4.5/bpy.app.handlers.txt`, `api_4.5/info_tips_and_tricks.txt`, `api_4.5/bpy.types.RenderEngine.txt`). So: **the sources do not say**, beyond the viewport-compositor note.

**Operating system.** The sources do not say. Nothing on any page I read ties the Render Layers node's outputs to an operating system. The only operating-system-specific statement I found anywhere near the compositor is on the compositor's Limitations page and concerns GPU driver timeouts, not what the render arrives as: "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." (https://docs.blender.org/manual/en/4.5/compositing/limitations.html, read 2026-07-30).

---

### 5b. Can it be told apart from anything else there — first reading (distinguishing the render's entry point from other sources)

**LABEL: documented**

Yes. The render enters through a node type of its own, which no other source in the compositor shares, and that node carries a scene and a view layer on its face.

The Render Layers node's own properties name the scene and the view layer:

> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html — read 2026-07-30.

In Python those two are exposed directly, so a script can read them:

> "layer
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

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

(The `'PLACEHOLDER'` is what the reference page prints for `layer`. The page does not explain why. I do not draw any conclusion from it.)

Every other input node in the compositor is a different type with different properties. The other image-bearing input is the Image node:

> "The Image node injects any image format that is supported by Blender."

> "Properties
>
> Image
>
> Selection of different types of media."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/image.html — read 2026-07-30.

And in Python:

> "class bpy.types.CompositorNodeImage(CompositorNode)
>
> Input image or movie file"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeImage.html — read 2026-07-30.

The manual lists the full set of input nodes, so the field of "anything else there" that produces an image is bounded:

> "Input nodes produce information from a data source. For instance, an input can be:
>
> Taken directly from the active camera in a selected scene.
>
> A static image.
>
> A movie clip (such as an image sequence or video).
>
> A color or value.
>
> These nodes generate the information that is passed to other nodes. As such, they have no input sockets; only outputs."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/index.html — read 2026-07-30.

Each node in a tree also has a type identifier and a unique name, both readable:

> "bl_static_type
>
> Legacy unique node type identifier, redundant with bl_idname property
>
> Type:
>
> string, default "", (readonly, never None)"

> "name
>
> Unique node identifier
>
> Type:
>
> string, default "", (never None)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30. (`Node.type` on the same page carries the identical description to `bl_static_type`: "Legacy unique node type identifier, redundant with bl_idname property".)

The API also warns that the identifier used to *create* a node is `bl_idname`, not `type`:

> "type (string, (never None)) – Type, Type of node to add (Warning: should be same as node.bl_idname, not node.type!)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Nodes.html — read 2026-07-30.

And the whole compositor tree is reachable from Python without touching the interface:

> "node_tree
>
> Compositing node tree
>
> Type:
>
> NodeTree, (readonly)"

> "use_nodes
>
> Enable the compositing node tree
>
> Type:
>
> boolean, default False"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

> "nodes
>
> Type:
>
> Nodes bpy_prop_collection of Node, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeTree.html — read 2026-07-30.

Putting those together — that a script can walk `scene.node_tree.nodes`, check each node's type identifier for the Render Layers type, and read `scene` and `layer` off the ones that match — is a step the sources do not spell out. **That combination is inferred**, and the step is: each quoted piece is documented on its own, and joining them requires the reader to supply the idea of iterating the collection.

**Not covered.** The sources do not say whether two Render Layers nodes pointing at the same scene and view layer are distinguishable from each other other than by node name. They do not say what the Render Layers node does if its Scene field is left empty. They do not say whether the `layer` enum reports a usable value in the situations where the reference page shows `'PLACEHOLDER'`.

**Outside the interface.** The properties quoted are RNA properties on the node, reachable through `bpy` whether or not a window is open; nothing in the sources ties them to the interface. But that is my reading, not a statement in the sources — the sources do not say. The one interface-only element here is the small button beside the View Layer dropdown that "re-renders it immediately", which is a button and therefore has no meaning outside the window; the sources do not say what, if anything, corresponds to it from outside.

**Operating system.** The sources do not say. Nothing on the pages read ties node types, names or properties to an operating system.

---

### 5b (other reading). Does the render carry a marker on itself once it is flowing?

**LABEL: not in the sources**

I could find nothing in the 4.5 manual, the 4.5 API documentation, the 4.5 release notes or the Cycles files saying that data travelling along a compositor link carries any tag, label, metadata or provenance identifying it as having come from a render rather than from an image file or a texture.

What the sources do say is what travels: data of one of three types.

> "Three types of data exist, all of which are stored in half precision formats: Float … Vector … Color"

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30.

And that sockets are typed, with implicit conversion between types:

> "type
>
> Data type
>
> Type:
>
> enum in Node Socket Type Items, default 'VALUE'"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30.

> "In case a node input is given data of type other than its own type, the following implicit conversions are performed:"

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30.

Nothing there is an identity, only a data type. On the strength of that, the honest answer is that the sources describe no marker.

**Where I looked (this sub-answer):**
- Searched the whole `manual_4.5/compositing` folder, case-insensitively, for `metadata`, `provenance`, `trace`, `identify`, `identifier`, `tag`. Thirty-odd hits, all irrelevant: they are about footage, watermarking, Viewer node shortcut numbers, Cryptomatte sidecar files ("Cryptomatte sidecars (metadata files) are not supported."), and mask/keying prose. None describes anything attached to data on a link.
- Read the Compositor System page in full (data types, implicit conversion, image domain, operation domain, output selection). It defines what data is, and never gives it an identity.
- Read `bpy.types.NodeSocket` and `bpy.types.NodeLink` in full. `NodeSocket` has `identifier` ("Unique identifier for mapping sockets"), `name`, `label`, `type`, `is_linked`, `is_output`, `link_limit`, `is_multi_input`, `is_inactive`, `is_unavailable`. All describe the *socket*, not the data on it. `NodeLink` has `from_node`, `from_socket`, `to_node`, `to_socket`, `is_hidden`, `is_muted`, `is_valid`, `multi_input_sort_id`. All describe the *link*, not the data.
- Searched `cycles_4.5` for `composit`, `rlayers`, `node_tree`. Hits are a Cycles render-property description, `CyclesRenderLayerSettings`, panel classes for compositor performance, `rd.use_compositing` in the UI file, and unrelated kernel comments. Nothing about compositor data identity.
- Read the 4.5 compositor release notes in full. Nothing about tagging or identifying data.

**Ruled out and why.** I did not chase the Cryptomatte pages further: Cryptomatte identifies *objects and materials within* an image, not the image's own origin, and the manual's Cryptomatte page describes it that way. I did not chase the Image Info node further either: its outputs are "Dimensions", "Resolution", "Location", "Rotation", "Scale" (https://docs.blender.org/manual/en/4.5/compositing/types/input/image_info.html, read 2026-07-30) — measurements of an image, not its provenance.

**Not covered.** Absence from documentation is not absence from the program. I only checked the 4.5 manual, 4.5 API documentation, 4.5 release notes and the Cycles files supplied. Blender's own source code beyond Cycles was not available to me and would be the place to settle this properly.

**Outside the interface.** The finding is that the sources are silent; that silence is the same whether Blender is driven by a person or from outside. Nothing changes.

**Operating system.** The sources do not say, and the finding is a silence, which does not vary by operating system.

---

### 5c. Can it be followed forward through pass-along nodes and through a split into more than one?

**LABEL: documented** for the parts below that are quoted; **inferred** for the one step I mark as such.

**One output can feed several destinations.** This is stated plainly:

> "While multiple links can route out of an output socket, typically a single link can be attached to an input socket, that is unless the input is a multi-socket input with looks like a pill shaped socket."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30.

**The Reroute node is a pass-along node that fans out.** Two independent statements:

> "A node used primarily for organization. Reroute looks and behaves much like a socket on other nodes in that it supports one input connection while allowing multiple output connections."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/reroute.html — read 2026-07-30.

> "class bpy.types.NodeReroute(NodeInternal)
>
> A single-socket organization tool that supports one input and multiple outputs"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeReroute.html — read 2026-07-30.

**The manual illustrates exactly this case with a render.** The Reroute Node page carries `[image: _images/interface_controls_nodes_reroute_node.png]`. I opened that file and, to read it clearly, cropped it and enlarged it (see the hand-check note about the method at the end). It shows a **Render Layers node** on the left — titled "Render Layers", with a scene data-block field reading "Scene" and a dropdown reading "RenderLayer" — whose output sockets are **Image**, **Alpha** and **Depth**. Two Reroute nodes sit in the middle of the graph as small round dots. The Image output runs to a yellow Reroute dot, and **two** links leave that same dot: one up to the Image socket of a **Viewer** node, one down to the Image socket of a **Composite** node. The Depth output runs to a grey Reroute dot, and again two links leave it, one to the Viewer node's Z socket and one heading down toward the Composite node. So the manual's own picture shows a render's Image output passing through a single Reroute node and arriving at two different destinations. Label: **documented** (this is a manual page's own figure).

One caveat on that figure, and I flag it rather than lean on it: the Viewer and Composite nodes in the picture show a "Use Alpha" checkbox, and the 4.5 release notes say "The Use Alpha option in the Composite and Viewer nodes was removed. The same functionality can be achieved using a Set Alpha node." (https://developer.blender.org/docs/release_notes/4.5/compositor/, read 2026-07-30). So the screenshot predates that change even though it sits in the 4.5 manual. I rely on it only for what it shows about Reroute fan-out, not for what the Viewer and Composite nodes offer.

**A muted node also passes data along unchanged.**

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30.

And the routing a muted node uses is exposed in Python:

> "internal_links
>
> Internal input-to-output connections for muting
>
> Type:
>
> bpy_prop_collection of NodeLink, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30. (`Node.mute` on the same page is "boolean, default False", with no description text.)

**Individual links can be muted, which breaks the trail — and this is detectable.**

> "A muted link acts as though it's no longer there; this also means the input fields for specifying fixed values become visible again."

> "When muting links on the input side of a reroute node, the links on its output side will be muted too."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30.

> "is_muted
>
> Link is muted and can be ignored
>
> Type:
>
> boolean, default False"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeLink.html — read 2026-07-30.

**The graph is fully readable, in both directions, from Python.**

> "class bpy.types.NodeLink(bpy_struct)
>
> Link between nodes in a node tree"

> "from_node
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

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeLink.html — read 2026-07-30.

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
> Takes O(len(nodetree.links)) time.
>
> (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30. Note that `NodeSocket.links` is plural on purpose: it returns a list, which is consistent with the manual's statement that multiple links can leave one output socket.

> "links
>
> Type:
>
> NodeLinks bpy_prop_collection of NodeLink, (readonly)"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeTree.html — read 2026-07-30.

**The inferred step.** That a program can therefore *follow the render forward* — start at the Render Layers node's Image socket, read `socket.links`, take each link's `to_node`, and recurse, treating Reroute nodes and muted nodes as pass-through and multiple links as a fan-out — is not stated anywhere in the sources. **LABEL for that claim: inferred.** The step a reader must supply: the sources give the pieces (a socket lists its links, a link names its destination node and socket, a Reroute relays one input to many outputs, a muted node passes links through unchanged) but never say that walking those pieces reconstructs the path of an image. Nothing I read says the walk is complete or that no other mechanism moves data between nodes.

**Two things that complicate a naive follow, both documented.**

First, node groups. A compositor node tree can contain groups, and data crosses the group boundary through separate nodes:

> "A Group Node combines a set of nodes into a single one, and selectively exposes inputs and outputs of those nodes."

> "Group Input
>
> Exposes the inputs of the node group."

> "Group Output
>
> Receives the outputs of the node group."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/groups.html — read 2026-07-30. In Python, `bpy.types.CompositorNodeGroup` carries a `node_tree` of type `NodeTree` (https://docs.blender.org/api/4.5/bpy.types.CompositorNodeGroup.html, read 2026-07-30), so the inner tree is a separate collection of nodes and links from the outer one.

Second, "unchanged" is not guaranteed through arbitrary nodes, because the compositor silently converts between data types:

> "Implicit Conversion
>
> In case a node input is given data of type other than its own type, the following implicit conversions are performed:"

with the table giving, among others, "Color / Float / (r, g, b, a) => Luminance(r, g, b)".

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30. The manual's node-parts page puts the consequence bluntly: "Once a socket conversion is made, data may be lost and cannot be retrieved later down the node tree." (https://docs.blender.org/manual/en/4.5/interface/controls/nodes/parts.html, read 2026-07-30). Only the Reroute node and muted nodes are documented as passing data along without change.

**The other reading of "split".** If "split it into more than one" was meant as a node that separates an image into components, the compositor has such nodes (Separate Color, Separate XYZ, listed at https://docs.blender.org/manual/en/4.5/compositing/types/color/mix/separate_color.html and https://docs.blender.org/manual/en/4.5/compositing/types/vector/separate_xyz.html, read 2026-07-30) and they are followed the same way — through their output sockets' links. If it was meant as the compositor's node actually named **Split**, that node is not a splitter of signal at all:

> "The Split node combines two images for side-by-side display. Typically used in combination with the Viewer Node."

> "Outputs
>
> This node has no output sockets."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/split.html — read 2026-07-30. Because it has no outputs, nothing can be followed forward past it.

**Not covered.** The sources say the links exist and are readable; they do not say the compositor evaluates the tree strictly along those links, nor that following the links reproduces what actually reaches the output. They say nothing about how a follow should treat a Switch node (whose Switch property picks which of two inputs reaches the output, and which the manual says "may be animated by adding a keyframe" — https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch.html, read 2026-07-30), so which branch is live can change per frame. They say nothing about cycles in a tree, about links to disabled or hidden sockets, or about what `NodeLink.is_valid` ("Link is valid", default False) means in practice.

**Outside the interface.** The Reroute node, the links and the muting flags are all data in the node tree, and the Python properties quoted are RNA properties on that data, not window operations. The sources do not state that any of this differs when Blender is driven from outside its own window. What is plainly interface-only in this area is the *making* and *muting* of links by hand (the shortcuts Ctrl-Alt-RMB, M and so on quoted above are keyboard actions), the red colouring of links through a muted node, and the backdrop. Also interface-only, and relevant if a render is being followed to a Viewer node: "The backdrop is the output of a Viewer node in the background of the Compositor" (https://docs.blender.org/manual/en/4.5/compositing/sidebar.html, read 2026-07-30), and "The compositor searches for the active Composite node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether." (https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html, read 2026-07-30). Beyond those, the sources do not say.

**Operating system.** The sources do not say. Nothing on any page read here ties reroutes, links, muting or the readability of the graph to an operating system.

---

### Record of where I looked (question 5 as a whole)

**Files searched, whole-folder, case-insensitively.**
- `manual_4.5` for: `render layers`, `reroute`, `Render Result`, `metadata`, `provenance`, `trace`, `identify`, `identifier`, `tag`, `background mode`, `viewport compositor`, `pass along`, `pass through`, `without change`, `unchanged`.
- `api_4.5` for: `Render Result`, `background mode`; and by filename for `RLayers`, `Reroute`, `NodeLink`, `NodeSocket`, `NodeTree`, `CompositorNode(Image|Composite|OutputFile|Viewer|Switch|Split|Group)`, `NodeGroupInput`, `NodeGroupOutput`, `Nodes`.
- `release_notes_4.5` — read `compositor.md` in full; searched `python_api.md` for `node`, `socket`, `link` (three hits: a bump/displacement note, and "New `NodeSocket.is_inactive` and `NodeSocket.is_icon_visible` accessors").
- `cycles_4.5` for: `composit`, `rlayers`, `node_tree`, `nodetree`.

**Manual pages opened in full.** `compositing/types/input/scene/render_layers.txt`, `compositing/types/input/scene/index.txt`, `compositing/types/input/index.txt`, `compositing/types/input/image.txt`, `compositing/types/input/image_info.txt`, `compositing/introduction.txt`, `compositing/compositor_system.txt`, `compositing/limitations.txt`, `compositing/sidebar.txt`, `compositing/types/groups.txt`, `compositing/types/utilities/split.txt`, `compositing/types/utilities/switch.txt`, `compositing/types/layout/index.txt`, `interface/controls/nodes/reroute.txt`, `interface/controls/nodes/introduction.txt`, `interface/controls/nodes/parts.txt`, `interface/controls/nodes/editing.txt` (the Connecting Sockets, Mute Links and Mute sections), `render/layers/passes.txt` (opening section).

**API pages opened.** `bpy.types.CompositorNodeRLayers`, `bpy.types.CompositorNodeImage`, `bpy.types.CompositorNodeGroup`, `bpy.types.CompositorNodeTree`, `bpy.types.NodeReroute`, `bpy.types.Node`, `bpy.types.NodeLink`, `bpy.types.NodeSocket`, `bpy.types.NodeTree`, `bpy.types.Nodes`, and the `node_tree` / `use_nodes` entries in `bpy.types.Scene`.

**Pictures opened.** `_images/node-types_CompositorNodeRLayers.webp` (from the Render Layers Node page) and `_images/interface_controls_nodes_reroute_node.png` (from the Reroute Node page). The second I also cropped and enlarged to read the link routing; the method is in the hand-check list below so it can be repeated.

**Ruled out, and why.**
- `manual_4.5/compositing/types/layout/index.txt` — it is four lines ("These are nodes which help you control the layout and connectivity of nodes within the Compositor.") and lists nothing. The Reroute and Frame nodes are documented under `interface/controls/nodes/` instead, which is where I went.
- `manual_4.5/addons/node/node_wrangler.txt` — a hit on `reroute`, but it documents an add-on's convenience operators, not what a Reroute node does.
- `manual_4.5/modeling/geometry_nodes/utilities/menu_switch.txt` and `manual_4.5/render/cycles/optimizations/nodes.txt` — hits on `reroute`, but about Geometry Nodes and Cycles shader optimisation, not the compositor.
- All eleven `Render Result` hits in the manual — they are about the Image Editor, the Video Sequencer preview, colour management and output saving. None describes the render arriving in the compositor. There are no `Render Result` hits in the API documentation at all.
- The Cryptomatte pages and the Image Info node — considered and rejected as answers to "told apart", for the reasons given in 5b (other reading).
- Cycles' own files — searched, and nothing in them touches the compositor node graph. The only near-hits are `cycles_4.5/ui.py` line 1115 (`col.prop(rd, "use_compositing")`) and the compositor performance panel classes around lines 818-825 of the same file, which concern whether compositing runs and how fast, not what a render arrives as.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Items 1 to 7 have been run and are **settled in the first-pass document**, `Preparation/Blender Research/Findings/4.5/The compositor.md`, where the observed findings and the machine's raw output are recorded under question 5. In short: the type identifiers do tell a Render Layers node from an Image node; `layer` reports `'ViewLayer'` rather than the `'PLACEHOLDER'` the reference page prints; a Reroute does relay to two destinations; the forward walk does run to completion, with muted links and muted nodes each flagged; a muted node's `internal_links` reports `[('Image', 'Image')]`; and all of it works in a session with no window open. The set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`, entry 11.

**Two more, settled since, by the Creator looking at a Blender window.** Both are recorded in the same first-pass document under question 5. Hovering the link out of a node in Blender 4.5 does show something — the socket's name, reported by the Creator as "Image." with a trailing period — and nothing in it names where the image came from (`Audits/Blender Research/The compositor/Hand tests.md`, entry 19). And with two Composite nodes in one tree, the active one carries a dark red header while the inactive one is grey, and the last node clicked is the active one (entry 21).

Two things are still open, and both need somebody looking at a Blender window rather than a terminal:

1. **What a Reroute looks like when it is made by hand**, and whether inserting one by the shortcut described above produces the same thing a script produces. Only the script route has been tried.

2. **How to reproduce my reading of the Reroute figure.** The file is `Blender Sources/manual_4.5/_images/interface_controls_nodes_reroute_node.png`, 750 by 361 pixels. Crop it to a 290-wide by 240-tall region with its top-left corner at x=270, y=50, then enlarge it about three times. On macOS: `sips -c 240 290 --cropOffset 50 270 <file> --out crop.png` followed by `sips -Z 900 crop.png --out big.png`. In the enlarged crop the Render Layers node's Image output is the yellow dot at upper left, the yellow Reroute dot sits in the middle, and two links leave it — one to the Viewer node's Image socket, one to the Composite node's Image socket. Anyone can open the original in an image viewer and zoom to check the same thing.

**One thing this pass got wrong, corrected by a test.** Section 5c says the compositor's Split node "has no output sockets", quoting the 4.5 manual's Split page. Adding that node in Blender 4.5 and printing its sockets gives `inputs=['Factor', 'Image', 'Image'] outputs=['Image']` — it has one. The observed finding is recorded in the first-pass document under question 5, and the same test on a second machine gave the same sockets.

**A second machine, 2026-08-06.** Every test named above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are recorded beside the Mac's in the first-pass document, never merged with them. All of them agreed with the Mac. **One exception:** hovering a link could not be settled on that machine at all, because its display is shared with other work and the pointer would not stay put, so the "Image." reading still rests on the Mac alone. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`. The two items still open above both need somebody looking at a Blender window and were not attempted on either machine.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 4.5

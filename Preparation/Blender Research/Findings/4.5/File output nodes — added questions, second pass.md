# File output nodes — added questions, second pass (Blender 4.5)

Sources consulted, and when:

- Blender 4.5 manual (plain-text copy, including its pictures), read 2026-07-30
- Blender 4.5 Python API documentation (plain-text copy), read 2026-07-30
- Blender 4.5 release notes (plain files), read 2026-07-30
- Cycles' own files as shipped with Blender 4.5, read 2026-07-30
- Blender's requirements page, taken 2026-08-01, read 2026-08-01

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 14 only. No other question on the sheet was answered or touched.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

### How I read the question

The question is ambiguous in two places, and I say so rather than choose quietly.

- **"whatever governs the compositor it sits in."** I read this as the two switches that turn the scene's compositor on or off for a render: the compositor's own **Use Nodes**, and **Compositing** in the Output properties' Post Processing panel. The other reading is the mute of a node group the File Output node happens to sit inside; I say what the sources hold on that too, at the end of this answer, but I have not treated it as the main reading.
- **"inside a single render."** I read this as one animation render covering a range of frames — one press of Animation, or one command-line run that renders a range. The other reading is one single-frame render, where "from one frame to the next" would not arise.

I have split the answer into the two things the question names, and then given the general machinery the sources do describe, because that machinery is where a reader will otherwise assume an answer that the sources do not give.

---

### 14a. The node's own silencing (Mute)

**LABEL: documented** — for what Mute is and how it is set.
**LABEL: not in the sources** — for whether Mute can carry a keyframe or a driver, and for whether a change of Mute between frames takes effect inside one render.

What the sources do say:

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change.
> Links will appear red as an indicator of passing through the muted node."

— Blender manual, *Editing* (nodes), https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30

The same page gives how it is reached:

> "Node ‣ Toggle Node Mute"

and

> "Shortcut:
>
> M"

— same page, same date.

Mute is a plain boolean on every node in the Python API:

> "mute
>
> Type:
>
> boolean, default False"

— Blender Python API, `bpy.types.Node`, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30

The API page for the File Output node itself lists no property of its own that switches writing on or off; it inherits `mute` from `Node` and adds `active_input_index`, `base_path`, `file_slots`, `format`, `layer_slots` and `save_as_render`.

> "class bpy.types.CompositorNodeOutputFile(CompositorNode)
>
> Write image file to disk"

— Blender Python API, `bpy.types.CompositorNodeOutputFile`, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30

What the sources never say: that `mute` is animatable, that it can hold a keyframe or a driver, or that muting or unmuting between frames changes what the File Output node writes during a render already under way. I searched for this hard; the record is below.

---

### 14b. Whatever governs the compositor (Use Nodes, and Compositing in Post Processing)

**LABEL: documented** — for what the two switches are and what they do.
**LABEL: not in the sources** — for whether either can be made to change per frame, and for whether such a change is honoured inside one render.

The two switches, in the sources' own words:

> "Use Nodes
>
> Tells the render engine to use the node tree when computing the material color or rendering the final image,
> or not. If not, the tree is ignored."

— Blender manual, *Introduction* (nodes), https://docs.blender.org/manual/en/4.5/interface/controls/nodes/introduction.html — read 2026-07-30

> "After clicking Use Nodes the Compositor is enabled, however,
> it can also be disabled in the Post Processing."

— Blender manual, *Introduction* (compositing), https://docs.blender.org/manual/en/4.5/compositing/introduction.html — read 2026-07-30

> "Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Composite Output node."

— Blender manual, *Post Processing*, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30

In the Python API these are two separate properties, on two different things:

> "use_compositing
>
> Process the render result through the compositing pipeline, if compositing nodes are enabled
>
> Type:
>
> boolean, default True"

— Blender Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30

> "use_nodes
>
> Enable the compositing node tree
>
> Type:
>
> boolean, default False"

— Blender Python API, `bpy.types.Scene`, https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30

Nothing in the manual, the API documentation or the release notes says whether either of these can be keyframed or driven, or what happens if either changes while a render is running.

**One observed thing that bears on it, and only weakly.** In Cycles' own files, the panel that draws the Compositing switch turns off the property decorators for that panel:

```
1102:class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
1103:    bl_label = "Post Processing"
1104:    bl_options = {'DEFAULT_CLOSED'}
1105:    bl_context = "output"
1106:
1107:    def draw(self, context):
1108:        layout = self.layout
1109:        layout.use_property_split = True
1110:        layout.use_property_decorate = False
1111:
1112:        rd = context.scene.render
1113:
1114:        col = layout.column(align=True, heading="Pipeline")
1115:        col.prop(rd, "use_compositing")
1116:        col.prop(rd, "use_sequencer")
```

— Cycles' own files, `cycles_4.5/ui.py`, lines 1102–1116 — read 2026-07-30. **LABEL: observed.**

The manual's own picture of that panel shows the Compositing and Sequencer checkboxes with no decorator beside them, while Dither below them does have the round decorator dot. **The picture and this code do not agree.** The code switches decorators off for the whole panel, which would take the dot off the Dither row too; the picture has one. So whatever panel the picture shows, it is not the one this file draws, and I cannot use the file to explain the picture. *(Corrected by the citation check, 2026-08-06. The sentence here previously read that the picture "agrees with what that produces", which neither the picture nor the code carries. The 5.0 and 5.2 documents on this same point already say it the way it now reads.)*

— Picture `_images/render_output_properties_post-processing_panel.png`, on https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30. **LABEL: documented** (it is part of the manual).

What a decorator means is documented:

> "Decorators may appear next to number fields, menus,
> and checkboxes to indicate the property can be animated."

and

> "Clicking on the decorator dot icon will add a Keyframe to that property."

— Blender manual, *Decorators*, https://docs.blender.org/manual/en/4.5/interface/controls/buttons/decorators.html — read 2026-07-30

**LABEL: inferred**, and I write the step out: because that panel is drawn with decorators switched off, there is no one-click keyframe dot beside the Compositing switch in the Output properties. The step a reader must supply is that `use_property_decorate = False` suppresses the decorators the manual describes; the API documentation gives that property no description at all, so I am leaning on its name. **This does not mean the property cannot be animated.** The manual says a decorator indicates a property *can* be animated; it does not say the absence of one means it cannot, and the same page and the drivers pages both describe adding keyframes and drivers through a property's right-click menu rather than through the decorator. So this settles only that the quickest route is absent, not that the property is closed.

---

### 14c. The general machinery the sources do describe

None of this answers question 14. It is the nearest documented ground, and I set it out so that the gap is visible rather than filled by assumption.

**Almost any property can be animated — but "almost".**

> "Blender lets you animate almost any property, going from the X coordinate of an object to the
> transparency of a material."

— Blender manual, *Introduction* (F-Curves), https://docs.blender.org/manual/en/4.5/editors/graph_editor/fcurves/introduction.html — read 2026-07-30

The word "almost" is why this does not settle Mute or the Compositing switch. The sources give no list of what falls outside it.

**Keyframes and drivers are added from a property's context menu.**

> "Hovering over a property and pressing I or with the context menu by RMB
> a property and choose Insert Keyframe from the menu."

— Blender manual, *Editing Keyframes*, https://docs.blender.org/manual/en/4.5/animation/keyframes/editing.html — read 2026-07-30

> "The usual way to add a driver to a property is to RMB click a property,
> then choose Add Driver in the context menu."

— Blender manual, *Usage* (drivers), https://docs.blender.org/manual/en/4.5/animation/drivers/usage.html — read 2026-07-30

**A compositor node's boolean is documented as animatable in one case — the Switch node.**

> "Switch state may be animated by adding a keyframe.
> This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

— Blender manual, *Switch Node*, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch.html — read 2026-07-30

This is the only place in the whole 4.5 manual where a compositor node's own on/off state is said to be animatable. It is about the Switch node's Switch property, not about Mute, and the Switch node changes which image is passed on, not whether a File Output node writes.

**Node trees can carry animation data.**

> "animation_data
>
> Animation data for this data-block
>
> Type:
>
> AnimData, (readonly)"

— Blender Python API, `bpy.types.NodeTree`, https://docs.blender.org/api/4.5/bpy.types.NodeTree.html — read 2026-07-30

`bpy.types.CompositorNodeTree` lists `NodeTree.animation_data` among its inherited properties. So a compositing node tree is a thing that can hold animation; the sources do not say which of its nodes' properties that animation may address.

**Something is documented as running per frame during a render — Python handlers.**

> "Called after frame change for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

— Blender Python API, `bpy.app.handlers.frame_change_pre`, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30

> "Called after frame change for playback and rendering, after the data has been evaluated for the new frame."

— same page, `frame_change_post`, same date.

> "on writing a render frame (directly after the frame is written)"

— same page, `render_write`, same date.

The same page warns about doing exactly this:

> "Altering data from handlers should be done carefully. While rendering the
> frame_change_pre and frame_change_post handlers are called from one
> thread and the viewport updates from a different thread. If the handler changes
> data that is accessed by the viewport, this can cause a crash of Blender. In
> such cases, lock the interface (Render → Lock Interface or
> bpy.types.RenderSettings.use_lock_interface) before starting a render."

— same page, same date.

So the sources document a hook that fires once per frame during a render and is explicitly meant for changing data for the new frame. **LABEL: inferred**, step written out: a handler on `frame_change_pre` could set `node.mute` or `scene.render.use_compositing` while an animation render is running. The step a reader must supply is that the compositor reads those values *after* the handler has run, for that same frame. **The sources do not say that.** The handler documentation names "data and relations" and gives a mesh as its example; it says nothing about node mute or about the render pipeline's own switches, and nothing about when during a frame the compositor reads them. I will not carry it further than that.

**Drivers do run during a background animation render.**

> "To render an animation in background mode, allowing drivers and other scripts to run:
>
> blender --background --enable-autoexec my_movie.blend --render-anim"

— Blender manual, *Scripting & Security*, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html — read 2026-07-30

That is documented for drivers as a class. It does not say which properties may be driven.

---

### 14d. The other reading — a node group's mute

If "whatever governs the compositor it sits in" means a node group the File Output node has been placed inside, then the sources give the same shape of answer: the group node's Mute is the same `Node.mute` boolean quoted above, described by the same manual sentence, and the sources say nothing about animating it or about changing it mid-render. The 4.5 release notes record one change touching muted group-like behaviour, and it is about Geometry Nodes, not the compositor:

> "* Muted *Switch* nodes have better internal links (blender/blender@61e99ca6676d7ee281db981b17e8b6bfbc9f2d99)."

— Blender 4.5 release notes, *Geometry Nodes*, https://developer.blender.org/docs/release_notes/4.5/geometry_nodes/ — read 2026-07-30

---

### What running Blender 4.5 showed — observed, 2026-08-06

Run by Opus 5 (Claude Code) on **a rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all, reached over SSH**. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, started with no window, with `--factory-startup`, and pointed at a settings folder of its own made beforehand. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **These results are the Linux half.** Every one of these tests was run again on the Creator's Mac, and that half is the section immediately below. Windows was not tried at all: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Which of these switches Blender will animate at all** (entry 1). Asked for each property's own animatable flag, then asked to insert a keyframe and to add a driver on each. What came back:

```
RESULT [4.5.12 LTS] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> is_animatable=False  is_readonly=True  type=POINTER
    Scene.compositing_node_group -> no such property in this version
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [4.5.12 LTS] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [4.5.12 LTS] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
```

The word `mute` was typed at a link first and came back "not found"; a second run listed every property a link has and tried the name that turned up:

```
RESULT [4.5.12 LTS] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. Every one of the four switches stopped the node writing when set:

```
RESULT [4.5.12 LTS] (i) nothing switched off
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
RESULT [4.5.12 LTS] (ii) the node muted for the whole render
RESULT [4.5.12 LTS] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as False
RESULT [4.5.12 LTS] (iv) scene.use_nodes switched off for the whole render
RESULT [4.5.12 LTS] (v) the link feeding the node muted for the whole render
```

(The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.)

*(Note added by the citation check, 2026-08-06, in both this block and its copy in the Mac section below: the line "after setting scene.use_nodes = False it reads back as ..." appears under heading (iii), the Compositing switch, but it is about `scene.use_nodes` and belongs to (iv). It is where the running script printed it, not evidence about the Compositing switch. The file counts under each heading are right and were reproduced by this check.)*

**A keyframed `scene.use_nodes` across one animation render** (entry 3). Keyed on at frame 1 and off at frame 3, keys flattened to Constant, frames 1 to 4 rendered as one animation:

```
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1, frame 3 = 0
RESULT [4.5.12 LTS] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
RESULT [4.5.12 LTS] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = False
    frame 4 -> scene.use_nodes = False
```

The same test with Cycles on the processor instead of Workbench gave the same two files.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch. What was set, and what was written:

```
    handler ran: frame 3, node.mute set to True
RESULT [4.5.12 LTS] (A) frame_change_pre sets node.mute on from frame 3
    out/img0001.png, out/img0002.png, out/img0003.png
    handler ran: frame 3, render.use_compositing set to False
RESULT [4.5.12 LTS] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/img0001.png, out/img0002.png, out/img0003.png, out/img0004.png
    handler ran: frame 3, scene.use_nodes set to False, reads back False
RESULT [4.5.12 LTS] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/img0001.png, out/img0002.png
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [4.5.12 LTS] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/img0001.png, out/img0002.png, out/img0004.png, out/img0005.png
RESULT [4.5.12 LTS] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/img0001.png, out/img0003.png, out/img0004.png, out/img0006.png
RESULT [4.5.12 LTS] (J) handler mutes the link on frames 2 and 5 only
    out/img0001.png, out/img0002.png, out/img0004.png, out/img0005.png
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved, and rendered twice from a terminal:

```
RESULT [4.5.12 LTS] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = False
--- command: /opt/blender/4.5/blender -b  /root/file-output-nodes/work/driven-4.5.12.blend -a
--- files written by the File Output node:
    img0001.png
    img0002.png
--- command: /opt/blender/4.5/blender -b -y /root/file-output-nodes/work/driven-4.5.12.blend -a
--- files written by the File Output node:
    img0001.png
    img0002.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [4.5.12 LTS] (i) the group not muted
    out/img0001.png … out/img0006.png  (six files)
RESULT [4.5.12 LTS] (ii) the group node muted for the whole render
RESULT [4.5.12 LTS] (iii) handler mutes the group node on frames 2 and 5 only
    out/img0001.png, out/img0002.png, out/img0004.png, out/img0005.png
RESULT [4.5.12 LTS] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which tree the scene composites with** (entry 7):

```
RESULT [4.5.12 LTS] can the scene's compositor tree be swapped at all
    Scene.node_tree -> read-only=True  is_animatable=False
    scene.node_tree = other -> raised AttributeError: bpy_struct: attribute "node_tree" from "Scene" is read-only
```

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [4.5.12 LTS] the tree's output node is a CompositorNodeComposite
RESULT [4.5.12 LTS] (ii) the output node muted for the whole render
    out/img0001.png … out/img0006.png  (six files)
RESULT [4.5.12 LTS] (iv) the output node removed from the tree altogether
    out/img0001.png … out/img0006.png  (six files)
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /opt/blender/4.5/blender -b /root/file-output-nodes/work/keyed-4.5.12.blend -a
--- files written by the File Output node:
    img0001.png
    img0002.png
--- command: /opt/blender/4.5/blender -b /root/file-output-nodes/work/plain-4.5.12.blend --python /root/file-output-nodes/work/flip.py -a
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
--- files written by the File Output node:
    img0001.png
    img0002.png
    img0004.png
    img0005.png
```

### What running Blender 4.5 on the Creator's Mac showed — observed, 2026-08-07

Run by Opus 5 (Claude Code) on **the Creator's own Mac: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, pointed at a settings folder of its own made beforehand. Entries 1 to 9 were run with no window and with `--factory-startup`, exactly as on Linux. Entries 10 and 11 were run with Blender's window open, which is something the Linux machine cannot do at all, having no display server. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **Nothing here was timed and nothing here measured memory**, because other topics were running Blender on the same Mac at the same time; every result is a count of files or a line Blender printed.

**Every test that had been run on Linux was run again here, and the two machines agreed on every one of them.** Same scripts, same commands, same four versions. The file counts and the lines Blender printed matched the Linux run word for word. The only difference anywhere is the size in bytes of the pictures, which is the two machines writing the same picture rather than a different one, so no file was compared byte for byte.

**Which of these switches Blender will animate at all** (entry 1). What came back on the Mac:

```
RESULT [4.5.12 LTS] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> is_animatable=False  is_readonly=True  type=POINTER
    Scene.compositing_node_group -> no such property in this version
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [4.5.12 LTS] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [4.5.12 LTS] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
RESULT [4.5.12 LTS] where the curves ended up
    fcurves on the compositor node tree: 0
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1
```

And the link's own silencing, asked by its real name:

```
RESULT [4.5.12 LTS] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.

```
RESULT [4.5.12 LTS] (i) nothing switched off
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
RESULT [4.5.12 LTS] (ii) the node muted for the whole render
RESULT [4.5.12 LTS] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as False
RESULT [4.5.12 LTS] (iv) scene.use_nodes switched off for the whole render
RESULT [4.5.12 LTS] (v) the link feeding the node muted for the whole render
```

**A keyframed `scene.use_nodes` across one animation render** (entry 3). Keyed on at frame 1 and off at frame 3, keys flattened to Constant, frames 1 to 4 rendered as one animation:

```
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1, frame 3 = 0
RESULT [4.5.12 LTS] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
RESULT [4.5.12 LTS] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = False
    frame 4 -> scene.use_nodes = False
```

The same test with Cycles on the processor instead of Workbench left the same two files, at 832 bytes each, so the engine makes no difference on this machine either.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [4.5.12 LTS] (A) frame_change_pre sets node.mute on from frame 3
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    handler ran: frame 3, render.use_compositing set to False
RESULT [4.5.12 LTS] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
    handler ran: frame 3, scene.use_nodes set to False, reads back False
RESULT [4.5.12 LTS] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    handler ran: frame 3, link.is_muted set to True
RESULT [4.5.12 LTS] (D) frame_change_pre sets the link's mute on from frame 3
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [4.5.12 LTS] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0004.png  710 bytes
    out/img0005.png  710 bytes
RESULT [4.5.12 LTS] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/img0001.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
    out/img0006.png  710 bytes
RESULT [4.5.12 LTS] (J) handler mutes the link on frames 2 and 5 only
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0004.png  710 bytes
    out/img0005.png  710 bytes
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved on the Mac, and rendered twice from the Mac's terminal:

```
RESULT [4.5.12 LTS] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = False
--- command: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender -b <the saved file> -a
--- files written by the File Output node:
    img0001.png
    img0002.png
--- command: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender -b -y <the saved file> -a
--- files written by the File Output node:
    img0001.png
    img0002.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [4.5.12 LTS] (i) the group not muted
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
    out/img0005.png  710 bytes
    out/img0006.png  710 bytes
RESULT [4.5.12 LTS] (ii) the group node muted for the whole render
RESULT [4.5.12 LTS] (iii) handler mutes the group node on frames 2 and 5 only
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0004.png  710 bytes
    out/img0005.png  710 bytes
RESULT [4.5.12 LTS] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which tree the scene composites with** (entry 7):

```
RESULT [4.5.12 LTS] can the scene's compositor tree be swapped at all
    Scene.node_tree -> read-only=True  is_animatable=False
    scene.node_tree = other -> raised AttributeError: bpy_struct: attribute "node_tree" from "Scene" is read-only
```

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [4.5.12 LTS] the tree's output node is a CompositorNodeComposite
RESULT [4.5.12 LTS] (ii) the output node muted for the whole render
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
    out/img0005.png  710 bytes
    out/img0006.png  710 bytes
RESULT [4.5.12 LTS] (iv) the output node removed from the tree altogether
    out/img0001.png  710 bytes
    out/img0002.png  710 bytes
    out/img0003.png  710 bytes
    out/img0004.png  710 bytes
    out/img0005.png  710 bytes
    out/img0006.png  710 bytes
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender -b <keyed-4.5.12.blend> -a
--- files written by the File Output node:
    img0001.png
    img0002.png
--- command: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender -b <plain-4.5.12.blend> --python <flip.py> -a
    handler registered
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
--- files written by the File Output node:
    img0001.png
    img0002.png
    img0004.png
    img0005.png
```

**Whether Blender's own interface offers a keyframe on either switch** (entry 10). This one needs a screen and could not be asked on Linux at all. Blender was started on the Mac with a window and a script that switched the Properties editor to its Output tab, scrolled to the bottom of it, took a picture of the whole window, then switched a large area to the Compositor with a File Output node selected and the sidebar open on its Node tab, took a second picture, and quit. It was done once with Cycles as the engine and once with EEVEE, because Cycles supplies its own copy of the Post Processing panel. The panel comes up closed, so the script took the panel class out and put it back with its "start closed" option removed; what it printed:

```
CYCLES_PT_post_processing starts with bl_options = {'DEFAULT_CLOSED'}
CYCLES_PT_post_processing put back with bl_options = set()
RENDER_PT_post_processing starts with bl_options = {'DEFAULT_CLOSED'}
RENDER_PT_post_processing put back with bl_options = set()
the sidebar's tab starts as Node
the sidebar's tab is now Node
```

What the pictures show, and these are pictures rather than words, so this is a description of what is visible in them:

- With the engine set to Cycles, the Post Processing panel shows a **Pipeline** heading with **Compositing** and **Sequencer** checkboxes under it and a **Dither** slider below, and **no decorator beside any of the three**.
- With the engine set to EEVEE (`BLENDER_EEVEE_NEXT`), the same panel shows the same three controls, and **a small round decorator sits at the right-hand edge of the Dither row while Compositing and Sequencer have none**.
- In the compositor's sidebar, with a File Output node selected, the **Node** panel lists Name, Label and Color, and no Mute box at all. No decorator sits beside any row in that panel.

The pictures are kept at `/Users/artist/Documents/Smoozy hand tests/File output nodes/shots/`.

**Whether a render started with Blender's interface open agrees with one started with no window** (entry 11). The two saved files from entry 9 were rendered again on the Mac, this time with a window on screen instead of `-b`, and with `Render ‣ Lock Interface` switched on:

```
--- command: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender <keyed-4.5.12.blend> --python <windowed.py>
    out/img0001.png  781 bytes
    out/img0002.png  781 bytes
--- command: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender <plain-4.5.12.blend> --python <flip.py> --python <windowed.py>
    handler registered
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
    out/img0001.png  781 bytes
    out/img0002.png  781 bytes
    out/img0004.png  781 bytes
    out/img0005.png  781 bytes
```

The main render output was written for every frame in both runs and is cut from the quote. **These are the same frames the same two files wrote with no window in entry 9.**

---

### Not covered

*Several of the items below were settled afterwards by running Blender; the observed section above is where those answers are. What stands here is what the sources did not carry.*

- Whether `Node.mute` accepts a keyframe or a driver at all. The sources neither grant nor deny it.
- Whether `Scene.use_nodes` or `RenderSettings.use_compositing` accept a keyframe or a driver.
- If any of them can be animated, whether the animation is read at each frame of a render already running, or read once when the render starts.
- Where in a frame's work the compositor reads those values, relative to when `frame_change_pre` fires and relative to when the File Output node writes.
- Whether a File Output node that is muted writes nothing (that is question 8's ground, and I have not gone there).
- Whether the answer differs between the CPU compositor and the GPU compositor.
- Whether the answer differs between render engines. I checked Cycles' own files; the panel drawing the Compositing switch is Cycles' but the switch itself belongs to the scene's render settings, not to Cycles.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

The sources do not say whether any of this holds when Blender is driven from outside its own window, because they do not say it holds inside the window either. What they do document about driving Blender from outside, and which bears on the question:

- Command-line arguments run in order, so a script can be run before the render begins:

  > "Arguments are executed in the order they are given!"

  — Blender manual, *Command Line Rendering*, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30

  > "-P, --python <filepath>
  >
  > Run the given Python script file."

  — Blender manual, *Arguments*, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

  and the same page's Argument Order section gives the reason it matters:

  > "Arguments are executed in the order they are given. eg:"
  >
  > "blender --background test.blend --render-frame 1 --render-output "/tmp""
  >
  > "…will not render to /tmp because --render-frame 1 renders before the output path is set."

  — same page, same date.

- Drivers and scripts run in background renders when auto-execution is enabled, per the `--enable-autoexec` example quoted above.

- The handler warning about the viewport thread is written for a running interface. In a background render there is no viewport, but the sources do not say the warning ceases to apply, and I will not claim it does.

**LABEL for this section: documented** for the argument ordering and the autoexec example; **not in the sources** for the question itself.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources say nothing that makes any of this differ by operating system. Blender 4.5 is named on the requirements page as running on all three:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)."

> "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer."

— Blender requirements page, https://www.blender.org/download/requirements/ — read 2026-08-01. The second line appears under both the Windows and the Linux headings; the macOS line above is under the macOS heading.

The only operating-system-specific statement anywhere near the compositor in these sources is about the GPU compositor being reset by Windows' graphics driver watchdog:

> "Timeout Detection Recovery
> (TDR)
> resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

— Blender manual, *Limitations* (compositing), https://docs.blender.org/manual/en/4.5/compositing/limitations.html — read 2026-07-30

That concerns how long a compositing task may run, not whether anything decides that it runs at all. **LABEL: documented**, and it does not bear on question 14.

---

## Where I looked

All searches were run across the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` unless a narrower place is named.

**Searched for, across the whole manual:** `mute`, `muted`, `muting`, `bypass`, `silence`/`silenc`, `animatable`, `can be animated`, `not animatable`, `cannot be animated`, `keyframe`, `driver`, `Insert Keyframe`, `Add Driver`, `Animate Property`, `Use Nodes`, `Post Processing`, `use_compositing`, `use_nodes`, `each frame`, `every frame`, `per frame`, `frame by frame`, `during render`, `during rendering`, `Lock Interface`, `background mode`, `most properties`, `any property`, `every property`, `not all properties`.

**Searched for, across the whole API documentation:** `mute` (then filtered out the sequencer, NLA, F-curve, constraint and speaker senses, which are unrelated), `use_compositing`, `use_nodes`, `animation_data`, `keyframe_insert`, `driver_add`, `use_property_decorate`, `frame_change`, `render_pre`, `render_post`, `render_write`, `render_init`, `render_complete`, `persistent`, `use_lock_interface`.

**Searched for, across the release notes:** `mute`, `file output`, `use_compositing`, `post processing`, `animat`.

**Searched for, across Cycles' own files:** `use_compositing`, `compositor`, `file output`, `OutputFile`.

**Pages opened in full:**

- `compositing/types/output/file_output.txt` — the File Output node page. Says the node writes per frame during an animation render ("The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."), but says nothing about mute, nor about anything that switches it off.
- `interface/controls/nodes/editing.txt` — the Mute section, quoted above. This is the only place Mute for nodes is described.
- `interface/controls/nodes/index.txt` — confirms Mute has no second page; the index entry points at the section above.
- `interface/controls/nodes/introduction.txt` — the Use Nodes header item, quoted above.
- `interface/controls/nodes/groups.txt` — checked its "Animation" section in case node-group animation covered node state. It covers "animation data for node group properties, including active Actions and their assigned Slot", i.e. the group's interface values, not node mute. Ruled out.
- `compositing/introduction.txt`, `compositing/index.txt`, `compositing/compositor_system.txt`, `compositing/limitations.txt` — the whole non-node-type part of the compositing chapter. `compositor_system.txt` describes how the compositor picks its active output and how domains work; it says nothing about frames, animation, or muted nodes. `limitations.txt` covers only GPU hardware limits.
- `compositing/types/utilities/switch.txt` — quoted above.
- `render/output/properties/post_processing.txt` and its picture — quoted above. Note that the Pipeline part of this page is unwritten in 4.5: it reads "Pipeline / Todo / Add this information."
- `render/output/animation.txt` — the whole Rendering Animations page, in case it described what may change between frames of one render. It describes workflow choices (direct to movie vs. frame sequence), Overwrite, and Frame Step. Nothing about node state or the compositing switch changing mid-render. Ruled out.
- `animation/keyframes/introduction.txt`, `animation/keyframes/editing.txt`, `animation/drivers/introduction.txt`, `animation/drivers/usage.txt`, `animation/drivers/troubleshooting.txt` — read for any statement of which properties are open to keyframes or drivers. None gives one. The troubleshooting page covers only script-security and radians. Ruled out.
- `editors/graph_editor/fcurves/introduction.txt` — the "almost any property" sentence, quoted above.
- `interface/controls/buttons/decorators.txt` and its picture — quoted above.
- `advanced/scripting/security.txt` — the background-render autoexec example, quoted above.
- `advanced/command_line/arguments.txt` (the Python arguments and the Argument Order section) and `advanced/command_line/render.txt` — quoted above.
- API: `bpy.types.Node`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeTree`, `bpy.types.CompositorNodeTree`, `bpy.types.Scene`, `bpy.types.RenderSettings`, `bpy.types.UILayout`, `bpy.types.bpy_struct` (the `keyframe_insert` entry), `bpy.app.handlers` (read in full).
- Release notes: `compositor.md` and `core.md` read in full; `bugfixes.md`, `python_api.md`, `geometry_nodes.md` and `pipeline_assets_io.md` searched and their matching lines read.
- Cycles: `ui.py` around the Post Processing panel, quoted above.

**What I ruled out and why:**

- The Video Sequencer, NLA, Graph Editor channel, constraint and speaker senses of "mute" — different features entirely; none touches compositor nodes.
- `bpy.types.UILayout.use_property_decorate` — it exists in the API but carries no description text at all, so it cannot be quoted as evidence of what it does. I said so where I used it.
- The `bpy.types.Node` API page shows no marking of any kind on `mute` (no "readonly", no note); Blender's API pages in this snapshot carry no "animatable" marking on any property, so their silence here proves nothing either way.
- Cycles' files describe the render engine, not the compositor. The only compositor-adjacent thing in them is the panel drawing quoted above and some compositor performance panel class names; there is no compositing evaluation code among them.
- I did not use the web, and did not need to.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 4.5

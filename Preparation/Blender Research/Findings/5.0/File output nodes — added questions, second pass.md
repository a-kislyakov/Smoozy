# File output nodes — added questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender's manual for 5.0, Blender's Python API documentation for 5.0, and Blender's 5.0 release notes — all read on 2026-07-30 (the date the copies in `Blender Sources/` were taken). Cycles' own files for 5.0, read on 2026-07-30. Blender's requirements page, read on 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 14 only. No other question on the sheet was answered.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**Label: not in the sources** — for the question as asked. Nothing I could find states, for either switch, that it can be animated, driven, or otherwise made to differ from frame to frame within one render. Several pieces around the question are documented, and they are set out below, each with its own label, so the gap is visible rather than papered over.

**The reading I answered.** "Inside a single render" I read as one render job that covers a range of frames — an animation render — not as a series of separate single-frame renders. "The node's own silencing" I read as Mute, the node-level switch the manual describes under Node ‣ Show/Hide ‣ Mute. "Whatever governs the compositor it sits in" I read as the scene-level switch that turns compositing on for the render — Properties ‣ Output ‣ Post Processing ‣ Compositing — together with whether a compositing node group is assigned to the scene at all. The other reading of that phrase would be a node group the File Output node is nested inside; I did not answer that reading, and the sheet asks about node groups separately.

### What is documented about the two switches themselves

**Mute exists and is a node-level property.**

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change."
> "Links will appear red as an indicator of passing through the muted node."

Source: manual, Editing (nodes). `https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html`. Read 2026-07-30. Label: **documented**.

The File Output node inherits that property. The API page for the node lists, under Inherited Properties:

> "Node.mute"

and `Node` itself gives:

> "mute
> Type:
> boolean, default False"

Sources: `https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html` and `https://docs.blender.org/api/5.0/bpy.types.Node.html`. Read 2026-07-30. Label: **documented**.

There is also an operator for it:

> "bpy.ops.node.mute_toggle()
> Toggle muting of selected nodes"

Source: `https://docs.blender.org/api/5.0/bpy.ops.node.html`. Read 2026-07-30. Label: **documented**.

**The compositor's own governor.**

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

Source: manual, Compositing Introduction. `https://docs.blender.org/manual/en/5.0/compositing/introduction.html`. Read 2026-07-30. Label: **documented**.

> "Pipeline – Compositing
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

Source: manual, Post Processing. `https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html`. Read 2026-07-30. Label: **documented**.

The property behind it:

> "use_compositing
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
> Type:
> boolean, default True"

Source: `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`. Read 2026-07-30. Label: **documented**.

The older switch is gone in 5.0, so it is not a second lever:

> "`scene.use_nodes` is deprecated and will be removed in 6.0
> Currently it always returns `True` and setting it has no effect."

Source: release notes, Python API. `https://developer.blender.org/docs/release_notes/5.0/python_api/`. Read 2026-07-30. The API page says the same and points to the replacement:

> "Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

Source: `https://docs.blender.org/api/5.0/bpy.types.Scene.html`. Read 2026-07-30. Label: **documented**.

So in 5.0 there are two things at the scene level: `use_compositing`, and whether a node group is assigned —

> "compositing_node_group
> Compositor Nodes
> Type:
> NodeTree"

Source: `https://docs.blender.org/api/5.0/bpy.types.Scene.html`. Read 2026-07-30. Label: **documented**.

### What is documented about making either of them vary

**Both switches sit on data-blocks that can hold animation.** The node tree is one:

> "animation_data
> Animation data for this data-block"

on `NodeTree`, and the same line on `Scene`. Sources: `https://docs.blender.org/api/5.0/bpy.types.NodeTree.html` and `https://docs.blender.org/api/5.0/bpy.types.Scene.html`. Read 2026-07-30. Label: **documented**.

The manual says the same in words, on both sides:

> "Animation
> Controls animation data for node group properties, including active Actions and their assigned Slot."

Source: manual, Node Groups. `https://docs.blender.org/manual/en/5.0/interface/controls/nodes/groups.html`. Read 2026-07-30.

> "Animation
> Controls animation data for scene-level properties, including active Actions and their assigned Slot."

Source: manual, Scene Properties. `https://docs.blender.org/manual/en/5.0/scene_layout/scene/properties.html`. Read 2026-07-30. Label: **documented**.

Neither passage names Mute or Compositing. "Node group properties" and "scene-level properties" are general phrases; reading either of our two switches into them is a step the source does not take.

**The general means of keying or driving a property are documented and are inherited by the File Output node.** Its API page lists, under Inherited Functions:

> "bpy_struct.driver_add"
> "bpy_struct.keyframe_insert"

Source: `https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html`. Read 2026-07-30. And:

> "keyframe_insert(data_path, /, *, index=-1, frame=bpy.context.scene.frame_current, group='', options=set(), keytype='KEYFRAME')
> Insert a keyframe on the property given, adding fcurves and animation data when necessary."

Source: `https://docs.blender.org/api/5.0/bpy.types.bpy_struct.html`. Read 2026-07-30. Label: **documented**.

This says the call exists on the node. It does not say it succeeds for `mute`, and I found no page that lists which properties accept it.

**The nearest thing the manual says to a direct answer is about a different node.** On the Switch node:

> "Switch state may be animated by adding a keyframe.
> This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

Source: manual, Switch Node. `https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch.html`. Read 2026-07-30. Label: **documented**, for the Switch node only.

That is the one place the manual says outright that a compositor node's own state can be keyed and made to differ across part of a sequence. It is about the Switch node's Switch input, not about Mute, and not about the File Output node. I am not extending it: doing so would be a guess about a different property on a different node.

**The interface's own signal is mixed, and I could not settle it.** The manual explains what a decorator means:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated."

Source: manual, Decorators. `https://docs.blender.org/manual/en/5.0/interface/controls/buttons/decorators.html`. Read 2026-07-30. Label: **documented**.

I opened the picture of the panel: `_images/render_output_properties_post-processing_panel.png`, shown on the Post Processing page (`https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html`). Read 2026-07-30. What is visible: a Post Processing panel with two checkboxes side by side, Compositing and Sequencer, both ticked, and below them a Dither slider reading 1.000. A small round decorator dot sits to the right of the Dither slider. No decorator dot is visible beside either checkbox. Label for what is visible: **documented** (it is part of the manual).

Two things stop that from settling the question, and both matter. First, the manual only says decorators "may appear", so their absence is not stated to mean anything. Second, Cycles' own panel for the same properties switches decorators off for the whole panel:

> `        layout.use_property_decorate = False`
> `        rd = context.scene.render`
> `        col = layout.column(align=True, heading="Pipeline")`
> `        col.prop(rd, "use_compositing")`

File: `cycles_5.0/ui.py`, in `class CYCLES_PT_post_processing`. Read 2026-07-30. Label: **observed**.

Because that panel turns decorators off for everything it draws, the absence of a dot in a Cycles panel would carry no information at all. The picture in the manual does show a dot on Dither, so the picture is not of Cycles' panel; which engine's panel it is, I cannot tell from the sources, and I do not have the file that draws it. Any conclusion from the missing dot is therefore **inferred** and weak, and I am recording it as unsettled rather than as a finding.

### One documented way data can be made to change between frames of a render

This is not about either switch by name, but it is the only per-frame mechanism the sources describe, and it is what a negative answer above would have to be weighed against:

> "bpy.app.handlers.frame_change_pre
> Called after frame change for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

Source: `https://docs.blender.org/api/5.0/bpy.app.handlers.html`. Read 2026-07-30. Label: **documented**, for the mechanism.

The same page warns about doing this:

> "Altering data from handlers should be done carefully. While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render."

Same source and date. Label: **documented**.

Taking that from "change data … for the new frame" to "change this node's Mute, or the scene's Compositing switch, for the new frame" is a step the source does not take — its only example is swapping a mesh. Label for that step: **inferred**. The step is: the page says a handler running per frame during a render may change data; Mute and `use_compositing` are data on the scene and on a node tree; therefore a handler could set them per frame. What the page does not say, and what I therefore cannot claim, is whether the compositor and the File Output node read the new value for that same frame, or one frame late, or not at all.

There is also a handler that fires per written frame, which is worth knowing exists:

> "bpy.app.handlers.render_write
> on writing a render frame (directly after the frame is written)"

Same source and date. Label: **documented**.

### What running Blender 5.0 showed — observed, 2026-08-06

Run by Opus 5 (Claude Code) on **a rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all, reached over SSH**. Blender 5.0.1 at `/opt/blender/5.0/blender`, started with no window, with `--factory-startup`, and pointed at a settings folder of its own made beforehand. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **These results are the Linux half.** Every one of these tests was run again on the Creator's Mac, and that half is the section immediately below. Windows was not tried at all: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Which of these switches Blender will animate at all** (entry 1). Asked for each property's own animatable flag, then asked to insert a keyframe and to add a driver on each. What came back:

```
RESULT [5.0.1] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [5.0.1] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [5.0.1] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
```

The word `mute` was typed at a link first and came back "not found"; a second run listed every property a link has and tried the name that turned up:

```
RESULT [5.0.1] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. Three of the four switches stopped the node writing; `scene.use_nodes` did not, and would not even take the value:

```
RESULT [5.0.1] (i) nothing switched off
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.0.1] (ii) the node muted for the whole render
RESULT [5.0.1] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as True
RESULT [5.0.1] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.0.1] (v) the link feeding the node muted for the whole render
```

(The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.)

*(Note added by the citation check, 2026-08-06, in both this block and its copy in the Mac section below: the line "after setting scene.use_nodes = False it reads back as ..." appears under heading (iii), the Compositing switch, but it is about `scene.use_nodes` and belongs to (iv). It is where the running script printed it, not evidence about the Compositing switch. The file counts under each heading are right and were reproduced by this check.)*

**A keyframed `scene.use_nodes` across one animation render** (entry 3). Keyed on at frame 1 and off at frame 3, keys flattened to Constant, frames 1 to 4 rendered as one animation. The curve recorded the value 1 at both keys, because the second setting never took:

```
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1, frame 3 = 1
RESULT [5.0.1] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.0.1] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = True
    frame 4 -> scene.use_nodes = True
```

The same test with Cycles on the processor instead of Workbench gave the same four files.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [5.0.1] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
    handler ran: frame 3, render.use_compositing set to False
RESULT [5.0.1] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
    handler ran: frame 3, scene.use_nodes set to False, reads back True
RESULT [5.0.1] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [5.0.1] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.0.1] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.0.1] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved, and rendered twice from a terminal:

```
RESULT [5.0.1] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = True
--- command: /opt/blender/5.0/blender -b  /root/file-output-nodes/work/driven-5.0.1.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /opt/blender/5.0/blender -b -y /root/file-output-nodes/work/driven-5.0.1.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [5.0.1] (i) the group not muted
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.0.1] (ii) the group node muted for the whole render
RESULT [5.0.1] (iii) handler mutes the group node on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.0.1] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which node group the scene composites with** (entry 7). A second group with no File Output node in it was built, and the scene pointed at each in turn, then swapped per frame by a handler. The printing here was flushed line by line, so the order shown is the true one:

```
RESULT [5.0.1] (i) the scene set to the group holding the File Output node
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.0.1] (ii) the scene set to a group with no File Output node, for the whole render
RESULT [5.0.1] (iii) the scene set to no compositing group at all
    handler ran: frame 1, scene.compositing_node_group set to Comp
00:00.905  render           | Saved: '/root/file-output-nodes/work/fo/out/file_nameimg0001.png'
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
00:00.919  render           | Saved: '/root/file-output-nodes/work/fo/out/file_nameimg0002.png'
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
    handler ran: frame 1, scene.compositing_node_group set to Comp
RESULT [5.0.1] (iv) handler swaps to the writer-less group on frames 2 and 5 only
    out/file_nameimg0001.png
    out/file_nameimg0002.png
RESULT [5.0.1] can that pointer be keyframed
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
```

The handler printed for frames 1 and 2 and not again, while `render_write` printed for all six frames. No error and no traceback appeared anywhere in that run — but that on its own settles nothing, because a Blender that honoured the swap and a Blender that ignored it would both be quiet; what carries the point is the file count beside it, and that does tell the two apart. *(Sentence qualified by the citation check, 2026-08-06, which reran this test and saw the same interleaving, the same silence and the same two files.)* (The main render output's own `Saved:` lines sat between these and are cut from the quote; it was written on all six frames.)

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [5.0.1] the tree's output node is a NodeGroupOutput
RESULT [5.0.1] (ii) the output node muted for the whole render
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.0.1] (iv) the output node removed from the tree altogether
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /opt/blender/5.0/blender -b /root/file-output-nodes/work/keyed-5.0.1.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /opt/blender/5.0/blender -b /root/file-output-nodes/work/plain-5.0.1.blend --python /root/file-output-nodes/work/flip.py -a
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0004.png
    file_nameimg0005.png
```

### What running Blender 5.0 on the Creator's Mac showed — observed, 2026-08-07

Run by Opus 5 (Claude Code) on **the Creator's own Mac: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, pointed at a settings folder of its own made beforehand. Entries 1 to 9 were run with no window and with `--factory-startup`, exactly as on Linux. Entries 10 and 11 were run with Blender's window open, which is something the Linux machine cannot do at all, having no display server. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **Nothing here was timed and nothing here measured memory**, because other topics were running Blender on the same Mac at the same time; every result is a count of files or a line Blender printed.

**Every test that had been run on Linux was run again here, and the two machines agreed on every one of them.** Same scripts, same commands, same four versions. The file counts and the lines Blender printed matched the Linux run word for word. The only difference anywhere is the size in bytes of the pictures, which is the two machines writing the same picture rather than a different one, so no file was compared byte for byte.

**Which of these switches Blender will animate at all** (entry 1). What came back on the Mac:

```
RESULT [5.0.1] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [5.0.1] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [5.0.1] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
RESULT [5.0.1] where the curves ended up
    fcurves on the compositor node tree: 0
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1
```

And the link's own silencing, asked by its real name:

```
RESULT [5.0.1] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.

```
RESULT [5.0.1] (i) nothing switched off
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
RESULT [5.0.1] (ii) the node muted for the whole render
RESULT [5.0.1] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as True
RESULT [5.0.1] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
RESULT [5.0.1] (v) the link feeding the node muted for the whole render
```

This version also printed a warning of its own each time that switch was touched, which 4.5 does not print:

```
run.py:111: DeprecationWarning: 'Scene.use_nodes' is expected to be removed in Blender 6.0
  sc.use_nodes = False
```

**A keyframed `scene.use_nodes` across one animation render** (entry 3). Keyed on at frame 1 and off at frame 3, keys flattened to Constant, frames 1 to 4 rendered as one animation:

```
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1, frame 3 = 1
RESULT [5.0.1] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
RESULT [5.0.1] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = True
    frame 4 -> scene.use_nodes = True
```

The same test with Cycles on the processor instead of Workbench left the same four files, at 1882 bytes each, so the engine makes no difference on this machine either.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [5.0.1] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    handler ran: frame 3, render.use_compositing set to False
RESULT [5.0.1] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    handler ran: frame 3, scene.use_nodes set to False, reads back True
RESULT [5.0.1] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    handler ran: frame 3, link.is_muted set to True
RESULT [5.0.1] (D) frame_change_pre sets the link's mute on from frame 3
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [5.0.1] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
RESULT [5.0.1] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
    out/file_nameimg0006.png  856 bytes
RESULT [5.0.1] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved on the Mac, and rendered twice from the Mac's terminal:

```
RESULT [5.0.1] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = True
--- command: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender -b <the saved file> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender -b -y <the saved file> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [5.0.1] (i) the group not muted
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
    out/file_nameimg0006.png  856 bytes
RESULT [5.0.1] (ii) the group node muted for the whole render
RESULT [5.0.1] (iii) handler mutes the group node on frames 2 and 5 only
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
RESULT [5.0.1] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which tree the scene composites with** (entry 7):

```
RESULT [5.0.1] (i) the scene set to the group holding the File Output node
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
    out/file_nameimg0006.png  856 bytes
RESULT [5.0.1] (ii) the scene set to a group with no File Output node, for the whole render
RESULT [5.0.1] (iii) the scene set to no compositing group at all
    handler ran: frame 1, scene.compositing_node_group set to Comp
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
RESULT [5.0.1] (iv) handler swaps to the writer-less group on frames 2 and 5 only
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
RESULT [5.0.1] can that pointer be keyframed
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
```

The handler printed for frames 1 and 2 and not again, exactly as on Linux; `render_write` printed for all six frames. In run (iv) the main render output for frames 3 to 6 came out at 442 bytes against 700 bytes for frames 1 and 2. No error and no traceback appeared anywhere.

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [5.0.1] the tree's output node is a NodeGroupOutput
RESULT [5.0.1] (ii) the output node muted for the whole render
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
    out/file_nameimg0006.png  856 bytes
RESULT [5.0.1] (iv) the output node removed from the tree altogether
    out/file_nameimg0001.png  856 bytes
    out/file_nameimg0002.png  856 bytes
    out/file_nameimg0003.png  856 bytes
    out/file_nameimg0004.png  856 bytes
    out/file_nameimg0005.png  856 bytes
    out/file_nameimg0006.png  856 bytes
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender -b <keyed-5.0.1.blend> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender -b <plain-5.0.1.blend> --python <flip.py> -a
    handler registered
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0004.png
    file_nameimg0005.png
```

**Whether Blender's own interface offers a keyframe on either switch** (entry 10). This one needs a screen and could not be asked on Linux at all. Blender was started on the Mac with a window and a script that switched the Properties editor to its Output tab, scrolled to the bottom of it, took a picture of the whole window, then switched a large area to the Compositor with a File Output node selected and the sidebar open on its Node tab, took a second picture, and quit. It was done once with Cycles as the engine and once with EEVEE, because Cycles supplies its own copy of the Post Processing panel. The panel comes up closed, so the script took the panel class out and put it back with its "start closed" option removed; what it printed:

```
CYCLES_PT_post_processing starts with bl_options = {'DEFAULT_CLOSED'}
CYCLES_PT_post_processing put back with bl_options = set()
RENDER_PT_post_processing starts with bl_options = {'DEFAULT_CLOSED'}
RENDER_PT_post_processing put back with bl_options = set()
the sidebar's tab starts as Group
the sidebar's tab is now Node
```

What the pictures show, and these are pictures rather than words, so this is a description of what is visible in them:

- With the engine set to Cycles, the Post Processing panel shows a **Pipeline** heading with **Compositing** and **Sequencer** checkboxes under it and a **Dither** slider below, and **no decorator beside any of the three**.
- With the engine set to EEVEE (`BLENDER_EEVEE`), the same panel shows the same three controls, and **a small round decorator sits at the right-hand edge of the Dither row while Compositing and Sequencer have none**.
- In the compositor's sidebar, with a File Output node selected, the **Node** panel lists Name, Label and Color, and no Mute box at all. No decorator sits beside any row in that panel.

The pictures are kept at `/Users/artist/Documents/Smoozy hand tests/File output nodes/shots/`.

**Whether a render started with Blender's interface open agrees with one started with no window** (entry 11). The two saved files from entry 9 were rendered again on the Mac, this time with a window on screen instead of `-b`, and with `Render ‣ Lock Interface` switched on:

```
--- command: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender <keyed-5.0.1.blend> --python <windowed.py>
    out/file_nameimg0001.png  926 bytes
    out/file_nameimg0002.png  926 bytes
    out/file_nameimg0003.png  926 bytes
    out/file_nameimg0004.png  926 bytes
--- command: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender <plain-5.0.1.blend> --python <flip.py> --python <windowed.py>
    handler registered
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
    out/file_nameimg0001.png  926 bytes
    out/file_nameimg0002.png  926 bytes
    out/file_nameimg0004.png  926 bytes
    out/file_nameimg0005.png  926 bytes
```

The main render output was written for every frame in both runs and is cut from the quote. **These are the same frames the same two files wrote with no window in entry 9.**

---

### Not covered

*Several of the items below were settled afterwards by running Blender; the observed section above is where those answers are. What stands here is what the sources did not carry.*

- Whether Mute actually stops a File Output node writing at all. The manual says muting "removes its contribution to the node tree", and the File Output node has no output sockets ("This node has no output sockets", manual, File Output Node). What that means for a node whose whole job is writing to disk is not stated anywhere I found.
- Whether `mute` accepts a keyframe or a driver in practice. The API lists the call; no page lists which properties it works on.
- Whether `render.use_compositing` accepts a keyframe or a driver.
- If either can be animated, whether the value is re-read for each frame of a running render, or read once when the render job starts. This is the heart of the question and no source addresses it.
- Whether the answer differs between the CPU and GPU compositor. The 5.0 release notes mention a "CPU Compositor" in passing (release notes, Rendering, `https://developer.blender.org/docs/release_notes/5.0/rendering/`, read 2026-07-30) but say nothing about animated node state.
- Whether swapping `scene.compositing_node_group` for a different node group between frames is possible. It is a pointer, not a checkbox, and nothing in the sources discusses changing it during a render.
- The other reading of "the compositor it sits in" — a node group the File Output node is nested inside, and what muting that group node would do.

### Where I looked, and what I ruled out

Searched, across the whole of the 5.0 manual and the whole of the 5.0 API documentation (plain-text search over every file, pictures excluded from the text searches):

- `mute`, `muted`, `muting`, `unmute` — 60-odd hits in the manual, all listed and read down. The compositor-relevant ones are the two node pages quoted above (`interface/controls/nodes/editing.txt`, `interface/controls/nodes/node_editors.txt`). Everything else is the Video Sequencer, the NLA editor, the Graph Editor's channel mute, shape keys, bones, sound, and the Evaluate Closure / Join Geometry / Geometry to Instance pages in Geometry and Shader nodes. None connects muting to animation or to rendering. In the API, `mute` outside our two pages belongs to `Strip`, `Constraint`, `ActionGroup`, `NlaStrip`, the audio module and the sequencer operators.
- `animatable`, `not animatable`, `cannot be animated`, `can not be animated`, `non-animatable` — the last four return nothing at all in either the manual or the API documentation, so there is no page that rules any property out.
- `animation_data`, `keyframe_insert`, `driver_add` — followed to `bpy_struct`, `NodeTree`, `CompositorNodeTree` and `Scene`; quoted above. Generic; they name no specific property.
- `decorator` — one page, quoted above.
- `render settings` — 25 hits across the manual and API, all about quality and engine settings; none about animating them.
- `use_compositing`, `use_nodes`, `compositing_node_group` — API pages and release notes quoted above. Also searched Cycles' files: `use_compositing` appears once in `cycles_5.0/ui.py` (quoted) and twice in `cycles_5.0/source/kernel/device/gpu/kernel.h`, where it is a kernel argument unrelated to the scene switch.
- `per frame`, `per-frame`, `bypass`, `conditional`, `toggle` — within `compositing/`, `render/output/` and `interface/controls/nodes/`. The only relevant return is the Switch node tip, quoted above. `conditional` also returns the Enable Output node, which governs a node group's output sockets rather than whether a File Output node writes, so I ruled it out as outside this question.
- `Use Nodes` — in the 5.0 manual this phrase survives only for line styles, lights, texture nodes and the Shader Editor. There is no compositor "Use Nodes" toggle documented in 5.0, which matches the release note that `scene.use_nodes` now does nothing.
- Pages opened in full and ruled out as silent on this: File Output Node (`compositing/types/output/file_output.txt`), Compositing Introduction, Post Processing, Rendering Animations (`render/output/animation.txt`), Rendering From The Command Line, Animation Introduction, Keyframes Introduction, Editing Keyframes, Drivers Introduction, Drivers Usage, Node Groups, Scene Properties, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeCompositorFileOutputItem` (checked in case an individual output had its own on/off switch — it has none), `bpy.types.Node`, `bpy.types.Scene`, `bpy.types.RenderSettings`, `bpy.app.handlers`, `bpy.ops.render` and `bpy.ops.node`.
- Release notes for 5.0: searched every file for `mute`, `File Output`, `use_nodes`, `use_compositing`, `compositor`. The File Output entries concern the node's redesign, its split into Directory and File Name, translation handling, and frame numbering; none concerns muting or per-frame gating.
- Pictures: `_images/render_output_properties_post-processing_panel.png`, opened and described above.

Nothing in any of that says whether either switch can be made to vary within a render. I did not use the web and did not need to.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

The sources say nothing about either switch behaving differently when Blender is driven from outside its own window, so on the question itself: the sources do not say.

What is documented is that the same render can be started without the interface, and that a script can be run first:

> "blender -b file.blend -a"
> "-b
> Render in the background (without UI)."
> "-a
> Render the whole animation using all the settings saved in the blend-file."

and

> "Arguments are executed in the order they are given!"

Source: manual, Rendering From The Command Line. `https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html`. Read 2026-07-30.

> "-P, --python <filepath>
> Run the given Python script file."

Source: manual, Command Line Arguments. `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`. Read 2026-07-30. Label: **documented**. *(Corrected by the citation check, 2026-08-06: the option was written here as `--python <filename>`. The page's own entry reads `-P, --python <filepath>`; the description under it was quoted right.)*

That a script placed before `-a` could register the per-frame handler quoted earlier, and that the handler would then be in place for the render, is **inferred**: the ordering rule and the handler page are separate statements, and no page joins them.

One documented difference in background mode is worth recording, because it touches evaluation rather than the interface:

> "--disable-depsgraph-on-file-load
> Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)."
> "NOTE: this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."

Same source and date. This is about loading a file, not about frames within a render, and it is an option rather than the default today. I record it because it is the only documented statement that evaluation is arranged differently outside the interface. Label: **documented**; any bearing it has on this question is not stated.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources do not say. Nothing in the manual, the API documentation, the release notes or Cycles' files ties muting, the Compositing switch, animation evaluation, or handlers to any particular operating system.

The requirements page speaks for 5.0 in one line only, and it is about which machines run it, not about this behaviour:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

Source: `https://www.blender.org/download/requirements/`. Read 2026-08-01.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 5.0

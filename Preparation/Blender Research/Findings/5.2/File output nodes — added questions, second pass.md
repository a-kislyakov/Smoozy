# File output nodes — added questions, second pass (Blender 5.2)

Sources consulted: Blender's manual for 5.2, Blender's Python API documentation for 5.2, the 5.2 release notes, and Cycles' own files for 5.2 — all read on 2026-07-30 (the date the copy was taken). Blender's requirements page, read on 2026-08-01. Nothing else.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Scope: question 14 only. No other question on the sheet was answered.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**The question is ambiguous, and I say which reading I answered.** I read it as: inside one render run that covers a range of frames (Render Animation, or `blender -b file.blend -a`), can the Mute switch on a File Output node, and the scene switch that turns compositing on, hold one value on one frame and a different value on the next? The other reading would be about change *during* the rendering of a single frame — I did not answer that one.

**LABEL: not in the sources** (for the question as asked). The pieces below are labelled one by one.

### What the two switches are — documented

The node's own silencing is Mute, a property every node has.

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html — read 2026-07-30.

> "Mute
> Temporarily disables the node without removing it. When possible, the node is bypassed and its inputs are passed directly to its outputs."

Manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html — read 2026-07-30.

In the Python API it is a plain switch on the base node type, with no note attached to it:

> "mute
> (default False)
> Type:
> bool"

API documentation, https://docs.blender.org/api/5.2/bpy.types.Node.html — read 2026-07-30.

The File Output node inherits it. Its own page lists `active_item_index`, `directory`, `file_name`, `file_output_items`, `format`, `save_as_render`, `use_file_extension`, and names its base classes:

> "base classes — bpy_struct, Node, NodeInternal, CompositorNode"

API documentation, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

What governs the compositor is the Compositing switch in Output ‣ Post Processing.

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

Manual, https://docs.blender.org/manual/en/5.2/compositing/introduction.html — read 2026-07-30.

> "Pipeline – Compositing
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30.

Its name in the API, and what it says:

> "use_compositing
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

**One trap is documented outright.** The older scene-level switch no longer does anything, so animating it would change nothing:

> "use_nodes
> Enable the compositing node group. (default False)
> Deprecated since version 5.0: removal planned in version 6.0
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

The third thing that governs the compositor is which node tree the scene points at. The sources say only that the pointer exists:

> "compositing_node_group
> Compositor Nodes
> Type:
> NodeTree | None"

API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

### Whether either switch can be made to vary per frame — not in the sources

Nothing in the manual, the API documentation, the release notes or Cycles' files says that Mute can be keyframed or driven, or that the Compositing switch can be. Nothing says they cannot, either. The API documentation does not mark properties as animatable in its text at all; the only trace of the idea is a runtime flag a person can read for themselves:

> "is_animatable
> Property is animatable through RNA (default False, readonly)"

API documentation, https://docs.blender.org/api/5.2/bpy.types.Property.html — read 2026-07-30. (That entry describes the flag. It does not publish the flag's value for `mute` or for `use_compositing`. Reading it is the first hand-check listed at the end.)

**Two weak signals point the same way for the Compositing switch, and I label them carefully.**

*From a picture (documented).* The manual's picture of the Post Processing panel, `_images/render_output_properties_post-processing_panel.png`, shown on https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30. What is visible: a "Compositing" checkbox and a "Sequencer" checkbox, both ticked, with nothing to the right of them; below them a "Dither" slider reading 1.000 with a small dot to its right. The manual says what such a dot is:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated."

Manual, https://docs.blender.org/manual/en/5.2/interface/controls/buttons/decorators.html — read 2026-07-30.

**Inferred**, and here is the step: the picture shows a decorator beside Dither and none beside Compositing, and the decorators page says a decorator indicates a property can be animated; so in that panel Compositing is not shown as animatable. The reader has to supply the assumption that the absence of a decorator means something — the manual only says what a present decorator means. That is why this is inferred and not documented.

*From Cycles' own files (observed).* Cycles draws that panel, and it switches decorators off for the whole panel before drawing the Compositing switch:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False

        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

File: `cycles_5.2/ui.py`, lines 1169–1183 (class `CYCLES_PT_post_processing`) — read 2026-07-30. **Observed**: those are the lines in the file. I do not take the further step of saying what `use_property_decorate = False` produces on screen, because the sources do not describe it — the API entry for it gives only a default and a type, with no description (https://docs.blender.org/api/5.2/bpy.types.UILayout.html — read 2026-07-30). It matches the picture, and that is all I claim.

*The equivalent picture for Mute settles nothing.* The manual's picture of the node Sidebar, `_images/interface_controls_nodes_sidebar_item.png` on https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html — read 2026-07-30 — shows only Name, Label and a collapsed Color row for a Render Layers node. The Mute checkbox the text describes is not in the picture, so it cannot show whether a decorator sits beside it.

### What the sources do say about changing things per frame during a render

These are documented, and none of them is about Mute or about the Compositing switch. They are the mechanisms a per-frame change would have to use.

*Drivers work on compositor node properties, and can depend on the frame.* The manual tells a reader to put a driver on a compositor node's Offset and gives a frame-dependent expression:

> "You can put a Driver on the Offset to work around this. Simply type the following into the field, replacing StartFrame, VideoFrameRate and SceneFrameRate by their respective numbers:
> #(frame - StartFrame) * (VideoFrameRate - SceneFrameRate) / SceneFrameRate"

Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/image.html — read 2026-07-30.

`frame` is a listed global of the fast subset of driver expressions:

> "Globals
> frame"

and

> "Simple expressions are evaluated even when Python script execution is disabled."

Manual, https://docs.blender.org/manual/en/5.2/animation/drivers/drivers_panel.html — read 2026-07-30.

*A compositing node tree can hold animation of its own.* Documented only as far as the data existing:

> "animation_data
> Animation data for this data-block (readonly)"

API documentation, https://docs.blender.org/api/5.2/bpy.types.NodeTree.html — read 2026-07-30 (and the same entry on https://docs.blender.org/api/5.2/bpy.types.Scene.html). The sources do not say which of a node's properties that animation may cover.

*A keyframed on/off property steps rather than blends.*

> "F-Curves for properties that only accept discrete values will always have a stair step pattern, no matter which option you chose."

Manual, https://docs.blender.org/manual/en/5.2/editors/graph_editor/fcurves/properties.html — read 2026-07-30. This is general. It does not say Mute is among the properties that can be keyed.

*A script can change data per frame while a render is running.* This is the plainest statement I found that anything at all can be made to differ from frame to frame inside one render:

> "frame_change_pre
> Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

and the warning that goes with it:

> "Altering data from handlers should be done carefully. While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render."

There are also per-frame hooks around the render itself:

> "render_pre
> on render (before). Accepts one argument: the scene data-block being rendered"

> "render_write
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30.

**What the sources never say**, and this is the gap that leaves the question unanswered: that any of these reaches `mute` on a node or `use_compositing` on the render settings, and that the compositor re-reads either of them for every frame of a render. Writing that it does would be me speaking, not the sources.

### One release note that sits next to this, clearly labelled

For 5.2, under a change to the *interactive* compositor's execution trigger:

> "Frame changes will no longer execute the compositor if it does not depend on time."

Release notes, https://developer.blender.org/docs/release_notes/5.2/compositor/ — read 2026-07-30. The bullet it belongs to begins "The interactive compositor now relies on a more reliable mechanism for triggering execution", so it describes the compositor running in the editor, not the compositing done as part of a render. I include it because it is the only statement in the sources tying frame changes to whether the compositor executes at all; I make no claim that it applies to rendering.

### What running Blender 5.2 showed — observed, 2026-08-06

Run by Opus 5 (Claude Code) on **a rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all, reached over SSH**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, started with no window, with `--factory-startup`, and pointed at a settings folder of its own made beforehand. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **These results are the Linux half.** Every one of these tests was run again on the Creator's Mac, and that half is the section immediately below. Windows was not tried at all: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Which of these switches Blender will animate at all** (entry 1). Asked for each property's own animatable flag, then asked to insert a keyframe and to add a driver on each. What came back:

```
RESULT [5.2.0 LTS] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [5.2.0 LTS] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [5.2.0 LTS] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
```

The word `mute` was typed at a link first and came back "not found"; a second run listed every property a link has and tried the name that turned up:

```
RESULT [5.2.0 LTS] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. Three of the four switches stopped the node writing; `scene.use_nodes` did not, and would not even take the value:

```
RESULT [5.2.0 LTS] (i) nothing switched off
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.2.0 LTS] (ii) the node muted for the whole render
RESULT [5.2.0 LTS] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as True
RESULT [5.2.0 LTS] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.2.0 LTS] (v) the link feeding the node muted for the whole render
```

(The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.)

*(Note added by the citation check, 2026-08-06, in both this block and its copy in the Mac section below: the line "after setting scene.use_nodes = False it reads back as ..." appears under heading (iii), the Compositing switch, but it is about `scene.use_nodes` and belongs to (iv). It is where the running script printed it, not evidence about the Compositing switch. The file counts under each heading are right and were reproduced by this check.)*

**A keyframed `scene.use_nodes` across one animation render** (entry 3). Keyed on at frame 1 and off at frame 3, keys flattened to Constant, frames 1 to 4 rendered as one animation. The curve recorded the value 1 at both keys, because the second setting never took:

```
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1, frame 3 = 1
RESULT [5.2.0 LTS] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.2.0 LTS] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = True
    frame 4 -> scene.use_nodes = True
```

The same test with Cycles on the processor instead of Workbench gave the same four files.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [5.2.0 LTS] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
    handler ran: frame 3, render.use_compositing set to False
RESULT [5.2.0 LTS] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
    handler ran: frame 3, scene.use_nodes set to False, reads back True
RESULT [5.2.0 LTS] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [5.2.0 LTS] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.2.0 LTS] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.2.0 LTS] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved, and rendered twice from a terminal:

```
RESULT [5.2.0 LTS] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = True
--- command: /opt/blender/5.2/blender -b  /root/file-output-nodes/work/driven-5.2.0.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /opt/blender/5.2/blender -b -y /root/file-output-nodes/work/driven-5.2.0.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [5.2.0 LTS] (i) the group not muted
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.2.0 LTS] (ii) the group node muted for the whole render
RESULT [5.2.0 LTS] (iii) handler mutes the group node on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.2.0 LTS] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which node group the scene composites with** (entry 7). A second group with no File Output node in it was built, and the scene pointed at each in turn, then swapped per frame by a handler. The printing here was flushed line by line, so the order shown is the true one:

```
RESULT [5.2.0 LTS] (i) the scene set to the group holding the File Output node
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.2.0 LTS] (ii) the scene set to a group with no File Output node, for the whole render
RESULT [5.2.0 LTS] (iii) the scene set to no compositing group at all
    handler ran: frame 1, scene.compositing_node_group set to Comp
00:01.330  render           | Saved: '/root/file-output-nodes/work/fo/out/file_nameimg0001.png'
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
00:01.369  render           | Saved: '/root/file-output-nodes/work/fo/out/file_nameimg0002.png'
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
    handler ran: frame 1, scene.compositing_node_group set to Comp
RESULT [5.2.0 LTS] (iv) handler swaps to the writer-less group on frames 2 and 5 only
    out/file_nameimg0001.png
    out/file_nameimg0002.png
RESULT [5.2.0 LTS] can that pointer be keyframed
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
```

The handler printed for frames 1 and 2 and not again, while `render_write` printed for all six frames. No error and no traceback appeared anywhere in that run — but that on its own settles nothing, because a Blender that honoured the swap and a Blender that ignored it would both be quiet; what carries the point is the file count beside it, and that does tell the two apart. *(Sentence qualified by the citation check, 2026-08-06, which reran this test and saw the same interleaving, the same silence and the same two files.)* (The main render output's own `Saved:` lines sat between these and are cut from the quote; it was written on all six frames.)

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [5.2.0 LTS] the tree's output node is a NodeGroupOutput
RESULT [5.2.0 LTS] (ii) the output node muted for the whole render
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.2.0 LTS] (iv) the output node removed from the tree altogether
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /opt/blender/5.2/blender -b /root/file-output-nodes/work/keyed-5.2.0.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /opt/blender/5.2/blender -b /root/file-output-nodes/work/plain-5.2.0.blend --python /root/file-output-nodes/work/flip.py -a
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0004.png
    file_nameimg0005.png
```

### What running Blender 5.2 on the Creator's Mac showed — observed, 2026-08-07

Run by Opus 5 (Claude Code) on **the Creator's own Mac: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`, pointed at a settings folder of its own made beforehand. Entries 1 to 9 were run with no window and with `--factory-startup`, exactly as on Linux. Entries 10 and 11 were run with Blender's window open, which is something the Linux machine cannot do at all, having no display server. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **Nothing here was timed and nothing here measured memory**, because other topics were running Blender on the same Mac at the same time; every result is a count of files or a line Blender printed.

**Every test that had been run on Linux was run again here, and the two machines agreed on every one of them.** Same scripts, same commands, same four versions. The file counts and the lines Blender printed matched the Linux run word for word. The only difference anywhere is the size in bytes of the pictures, which is the two machines writing the same picture rather than a different one, so no file was compared byte for byte.

**Which of these switches Blender will animate at all** (entry 1). What came back on the Mac:

```
RESULT [5.2.0 LTS] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [5.2.0 LTS] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [5.2.0 LTS] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
RESULT [5.2.0 LTS] where the curves ended up
    fcurves on the compositor node tree: 0
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1
```

And the link's own silencing, asked by its real name:

```
RESULT [5.2.0 LTS] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.

```
RESULT [5.2.0 LTS] (i) nothing switched off
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
RESULT [5.2.0 LTS] (ii) the node muted for the whole render
RESULT [5.2.0 LTS] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as True
RESULT [5.2.0 LTS] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
RESULT [5.2.0 LTS] (v) the link feeding the node muted for the whole render
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
RESULT [5.2.0 LTS] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
RESULT [5.2.0 LTS] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = True
    frame 4 -> scene.use_nodes = True
```

The same test with Cycles on the processor instead of Workbench left the same four files, at 1949 bytes each, so the engine makes no difference on this machine either.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [5.2.0 LTS] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    handler ran: frame 3, render.use_compositing set to False
RESULT [5.2.0 LTS] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    handler ran: frame 3, scene.use_nodes set to False, reads back True
RESULT [5.2.0 LTS] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    handler ran: frame 3, link.is_muted set to True
RESULT [5.2.0 LTS] (D) frame_change_pre sets the link's mute on from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [5.2.0 LTS] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
RESULT [5.2.0 LTS] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.2.0 LTS] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved on the Mac, and rendered twice from the Mac's terminal:

```
RESULT [5.2.0 LTS] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = True
--- command: /Applications/Blender.app/Contents/MacOS/Blender -b <the saved file> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /Applications/Blender.app/Contents/MacOS/Blender -b -y <the saved file> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [5.2.0 LTS] (i) the group not muted
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.2.0 LTS] (ii) the group node muted for the whole render
RESULT [5.2.0 LTS] (iii) handler mutes the group node on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
RESULT [5.2.0 LTS] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which tree the scene composites with** (entry 7):

```
RESULT [5.2.0 LTS] (i) the scene set to the group holding the File Output node
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.2.0 LTS] (ii) the scene set to a group with no File Output node, for the whole render
RESULT [5.2.0 LTS] (iii) the scene set to no compositing group at all
    handler ran: frame 1, scene.compositing_node_group set to Comp
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
RESULT [5.2.0 LTS] (iv) handler swaps to the writer-less group on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
RESULT [5.2.0 LTS] can that pointer be keyframed
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
```

The handler printed for frames 1 and 2 and not again, exactly as on Linux; `render_write` printed for all six frames. In run (iv) the main render output for frames 3 to 6 came out at 515 bytes against 773 bytes for frames 1 and 2. No error and no traceback appeared anywhere.

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [5.2.0 LTS] the tree's output node is a NodeGroupOutput
RESULT [5.2.0 LTS] (ii) the output node muted for the whole render
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.2.0 LTS] (iv) the output node removed from the tree altogether
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /Applications/Blender.app/Contents/MacOS/Blender -b <keyed-5.2.0.blend> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /Applications/Blender.app/Contents/MacOS/Blender -b <plain-5.2.0.blend> --python <flip.py> -a
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
- In the compositor's sidebar, with a File Output node selected, the **Node** panel lists Name, Label, Color, Show Options, Mute and Propagate. No decorator sits beside any row in that panel.

The pictures are kept at `/Users/artist/Documents/Smoozy hand tests/File output nodes/shots/`.

**Whether a render started with Blender's interface open agrees with one started with no window** (entry 11). The two saved files from entry 9 were rendered again on the Mac, this time with a window on screen instead of `-b`, and with `Render ‣ Lock Interface` switched on:

```
--- command: /Applications/Blender.app/Contents/MacOS/Blender <keyed-5.2.0.blend> --python <windowed.py>
    out/file_nameimg0001.png  999 bytes
    out/file_nameimg0002.png  999 bytes
    out/file_nameimg0003.png  999 bytes
    out/file_nameimg0004.png  999 bytes
--- command: /Applications/Blender.app/Contents/MacOS/Blender <plain-5.2.0.blend> --python <flip.py> --python <windowed.py>
    handler registered
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
    out/file_nameimg0001.png  999 bytes
    out/file_nameimg0002.png  999 bytes
    out/file_nameimg0004.png  999 bytes
    out/file_nameimg0005.png  999 bytes
```

The main render output was written for every frame in both runs and is cut from the quote. **These are the same frames the same two files wrote with no window in entry 9.**

---

### Not covered

*Several of the items below were settled afterwards by running Blender; the observed section above is where those answers are. What stands here is what the sources did not carry.*

- Whether Mute and the Compositing switch accept keyframes or drivers at all. The sources are silent; my two weak signals concern only how the Compositing switch is drawn, and only in the panel Cycles draws.
- What happens on a frame where such a switch is off — whether that frame's file is simply skipped, or something else. Nothing in the sources addresses it.
- The other reading of the question: change *within* the rendering of one frame.
- Other things that might decide per frame whether this node writes — a Switch node feeding it, a muted enclosing group, per-output settings. The question named the node's own silencing and what governs the compositor, and I stayed on those two.
- The viewport (interactive) compositor, except for the one release note quoted above.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

A render run that covers many frames exists outside the interface, and this is documented:

> "-b
> Render in the background (without UI)."

> "-a
> Render the whole animation using all the settings saved in the blend-file."

Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

The sources do not answer the main question inside the interface, so they cannot answer it outside either. But one difference that would bear on it *is* documented, and it matters:

> "-y, --enable-autoexec
> Enable automatic Python script execution.
> -Y, --disable-autoexec
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

So from the command line, automatic Python script execution is off unless asked for. Against that stands the line quoted earlier: "Simple expressions are evaluated even when Python script execution is disabled." **Inferred**, with the step written out: a driver whose expression stays inside the Simple Expressions subset (which includes the global `frame`) would still be evaluated in a background render, while one outside that subset would need `-y`. The step is that "Python script execution is disabled" in the drivers page and `--disable-autoexec` in the arguments page describe the same state; the sources do not say so in one place.

Handlers must be registered by a script, which from the command line means `--python <filepath>`, `--python-text <name>` or `--python-expr <expression>` (same arguments page, read 2026-07-30). The handler documentation says `frame_change_pre` is called "for playback and rendering" and does not restrict it to the interface. The crash warning attached to it is about the viewport updating on another thread; whether it applies when there is no interface at all is not stated.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources say nothing operating-system-specific about Mute, about the Compositing switch, about keyframes, drivers or handlers. I searched the compositing and render sections for platform names and found one platform-specific compositor statement, and it is about something else:

> "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Manual, https://docs.blender.org/manual/en/5.2/compositing/limitations.html — read 2026-07-30, under the heading "TDR on Windows". That page describes the GPU compositor used while the interface is open, not what decides whether a File Output node writes. Blender's requirements page (https://www.blender.org/download/requirements/, read 2026-08-01) carries nothing on this topic at all.

### Where I looked

Searched across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` unless noted.

Terms searched: `mute`, `muting`, `mute_toggle`, `bypass`, `silence`, `disable`, `temporarily disables`, `use_compositing`, `use_nodes`, `compositing_node_group`, `Post Processing`, `compositing pipeline`, `animat` (catching animate/animated/animatable/animation), `animatable`, `cannot be animated`, `not animatable`, `keyframe`, `keying`, `driver`, `drivers`, `is_animatable`, `keyframe_insert`, `driver_add`, `animation_data`, `decorator`, `use_property_decorate`, `per frame`, `per-frame`, `each frame`, `during rendering`, `frame_change`, `handlers`, `render_pre`, `render_write`, `lock interface`, `use_lock_interface`, `autoexec`, `Simple Expressions`, `discrete`, `boolean`, `checkbox`.

Pages opened in full or in part:
- Manual: `interface/controls/nodes/editing`, `interface/controls/nodes/node_editors`, `interface/controls/buttons/decorators`, `compositing/introduction`, `compositing/limitations`, `compositing/compositing_workspace`, `compositing/types/output/file_output`, `compositing/types/input/image`, `render/output/properties/post_processing`, `render/output/animation`, `animation/keyframes/introduction`, `animation/keyframes/editing`, `animation/actions`, `animation/drivers/introduction`, `animation/drivers/drivers_panel`, `animation/drivers/troubleshooting`, `editors/graph_editor/fcurves/properties`, `advanced/command_line/render`, `advanced/command_line/arguments`.
- Pictures: `render_output_properties_post-processing_panel.png`, `interface_controls_nodes_sidebar_item.png`.
- API: `bpy.types.Node`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeCompositorFileOutputItem`, `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.NodeTree`, `bpy.types.CompositorNodeTree`, `bpy.types.Property`, `bpy.types.bpy_struct`, `bpy.types.UILayout`, `bpy.app.handlers`, `bpy.ops.node`, `bpy.props`.
- Release notes 5.2: `compositor.md`, `python_api.md`, `bugfixes.md` (searched), `index.md` folder listing.
- Cycles 5.2: `ui.py`, `properties.py`, `operators.py`, and a search of `source/` — the only hits for compositing outside `ui.py` are kernel comments and a GPU kernel argument about combining passes, none of them about the compositor node tree or about muting.
- Requirements page, `requirements_2026-08-01.txt`.

Ruled out and why:
- The Video Sequencer and NLA pages carry most of the manual's uses of the word "mute"; they are about strips and tracks, not nodes, so they do not speak for this.
- `Scene.use_nodes` is ruled out as a per-frame lever by its own documentation, quoted above: setting it has no effect.
- `NodeCompositorFileOutputItem` (a single output on the node) has no switch of its own — its properties are `color`, `format`, `name`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions`. So there is no per-output silencing to consider alongside the node's Mute (API documentation, https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html — read 2026-07-30).
- The 5.2 release notes mention Mute only in the bug-fix list, for editing links and grouping, not for animation.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 5.2

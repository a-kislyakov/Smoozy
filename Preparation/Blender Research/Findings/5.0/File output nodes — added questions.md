# File output nodes — added questions (Blender 5.0)

Sources consulted: the Blender 5.0 manual (`Blender Sources/manual_5.0`, including its images), the Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), the Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`) — all read on 2026-07-30 (the snapshot date). Blender's requirements page was not needed for this question and was not used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure, so the line above is exact. Before starting, I listed the file *names* in the folder I was told to save into (`Preparation/Blender Research/Findings/5.0/`) to confirm it existed and to be sure of the name I was to write. I opened none of those files and read no word of their contents; all I saw were filenames such as "Frames.md" and "Add-ons.md". No claim below draws on that.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**The question was ambiguous; here is the reading I answered.** I read "the node's own silencing" as muting the node (the Mute command in the node editor, and the `mute` property on a node). I read "whatever governs the compositor it sits in" as the scene-level switch that turns compositing on or off for the render, plus which node group the scene is set to composite with. The other reading of the second half would be the node *group* the File Output node is placed inside, since a group node can itself be muted; I have not answered that reading, and it is Question 7's ground.

**LABEL: not in the sources** — for the question actually asked, which is whether these things can be *made to change from one frame to the next inside a single render*. One narrow part of it is **documented** and is marked as such below: the old "Use Nodes" switch has no effect when set, so changing that one from frame to frame would do nothing.

The sources establish that these switches exist, and what each one does. They never say whether either one can be animated, driven, or otherwise changed part-way through a render, and they never say what the compositor does if one of them changes between frames of a single render. I searched for that and found nothing. What follows is what the sources do establish, then the record of where I looked.

### What the sources do establish

**The node's own silencing exists, and it is a plain on/off property.**

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Blender manual, `https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html` (Node ‣ Show/Hide ‣ Mute, shortcut M). Read 2026-07-30.

> "mute
>
> Type:
>
> boolean, default False"

Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Node.html`. Read 2026-07-30. The API index lists `mute` on `bpy.types.Node` and nowhere else in the node types, and `bpy.types.CompositorNodeOutputFile` inherits it (its page lists `Node.mute` among inherited members). There is also an operator `bpy.ops.node.mute_toggle()` — Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.node.html`, read 2026-07-30.

**The scene-level compositing switch exists, and it is a plain on/off property.**

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
>
> Type:
>
> boolean, default True"

Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`. Read 2026-07-30.

Its place in the interface, and the manual's own emphasis that it is what makes compositing happen at all:

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

Blender manual, `https://docs.blender.org/manual/en/5.0/compositing/introduction.html`. Read 2026-07-30.

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

Blender manual, `https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html` (Properties ‣ Output ‣ Post Processing). Read 2026-07-30.

**One switch that used to govern the compositor no longer does.** Two documented removals, both bearing on "whatever governs the compositor":

> "\"Use Nodes\" in the node editor was removed (blender/blender@d88d4cc8ce)"

Blender 5.0 release notes, `https://developer.blender.org/docs/release_notes/5.0/compositor/`. Read 2026-07-30.

> "use_nodes
>
> Enable the compositing node group.
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Scene.html`. Read 2026-07-30. So changing that one per frame would do nothing, whatever else is true — that part is documented.

**Which node group the scene composites with is also a property.**

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Scene.html`. Read 2026-07-30. And the tree is now a data-block in its own right:

> "The compositing node tree is now its own data block, making it reusable across blend files."

Blender 5.0 release notes, `https://developer.blender.org/docs/release_notes/5.0/compositor/`. Read 2026-07-30.

### What the sources do NOT establish, and what sits closest to it

**Nothing in the sources says whether `mute` or `use_compositing` can be keyframed or driven.** The API documentation does not mark individual properties as animatable. It does show that animatability is a per-property fact Blender itself tracks:

> "is_animatable
>
> Property is animatable through RNA
>
> Type:
>
> boolean, default False, (readonly)"

Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Property.html`. Read 2026-07-30. The documentation never gives that flag's value for either property, so it settles nothing here — but it is exactly the thing a person could read off in a minute (see the hand-check list).

Both properties sit on data-blocks that can hold animation. `bpy.types.NodeTree` has "animation_data — Animation data for this data-block" and `bpy.types.Scene` has the same (`https://docs.blender.org/api/5.0/bpy.types.NodeTree.html` and `https://docs.blender.org/api/5.0/bpy.types.Scene.html`, read 2026-07-30), and `bpy_struct.keyframe_insert` exists to "Insert a keyframe on the property given" (`https://docs.blender.org/api/5.0/bpy.types.bpy_struct.html`, read 2026-07-30). That a data-block can hold animation does not tell you that a particular property on it accepts it, so I am not calling this an inference — it is the shape of the gap, not a way round it.

**The manual does document one compositor on/off thing being animated, but it is not muting.** The Switch node:

> "Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

Blender manual, `https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch.html`. Read 2026-07-30. This is the only place in the whole compositing manual that says any compositor switch can be animated. It is about the Switch node's own state, not about muting a node and not about the scene's compositing switch, and the Switch node routes images between two inputs rather than deciding whether a File Output node writes.

**There is a documented hook that runs per frame during a render and can change data.**

> "frame_change_pre
>
> Called after frame change for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

Blender Python API documentation, `https://docs.blender.org/api/5.0/bpy.app.handlers.html`. Read 2026-07-30. The same page warns:

> "Altering data from handlers should be done carefully. While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render."

So the sources document a per-frame point inside a render at which data can be changed. They do not say that changing `mute` or `use_compositing` there takes effect for that frame's compositing. Going from "you may change data for the new frame" to "you may switch a File Output node off for one frame of a render" requires supplying the claim that the compositor re-reads those two switches per frame, which no source states. I am not making that step. There are also `render_pre`, `render_post` and `render_write` handlers listed on the same page ("on render (before)", "on render (after)", "on writing a render frame (directly after the frame is written)"), with no statement about what may be changed from them.

**A note on one thing that looks like evidence and is not.** The manual's screenshot of the Post Processing panel (`[image: _images/render_output_properties_post-processing_panel.png]`, on `https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html`, viewed 2026-07-30) shows "Compositing" and "Sequencer" as ticked checkboxes with no animation decorator dot beside either, and "Dither" with a small dot to its right. The manual says decorators indicate animatability:

> "Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated."

Blender manual, `https://docs.blender.org/manual/en/5.0/interface/controls/buttons/decorators.html`. Read 2026-07-30. It would be tempting to read the missing dot as "Compositing cannot be animated". That reading is unsafe, and Cycles' own files give the reason to doubt it. **LABEL for this sub-point: the quoted code is observed; the conclusion drawn from it is inferred, and the step is written out under the quote.**

> ```
> class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
>     bl_label = "Post Processing"
>     bl_options = {'DEFAULT_CLOSED'}
>     bl_context = "output"
>
>     def draw(self, context):
>         layout = self.layout
>         layout.use_property_split = True
>         layout.use_property_decorate = False
>
>         rd = context.scene.render
>
>         col = layout.column(align=True, heading="Pipeline")
>         col.prop(rd, "use_compositing")
>         col.prop(rd, "use_sequencer")
>
>         layout.prop(rd, "dither_intensity", text="Dither", slider=True)
> ```

Cycles' own files, `Blender Sources/cycles_5.0/ui.py`, lines 1112–1128 (class `CYCLES_PT_post_processing`). Read 2026-07-30. *(Line range corrected from 1111–1128 by the citation check, 2026-08-06: the block begins at 1112; line 1111 is blank. The quoted code itself was exact.)*

**Observed** — what the file says: this panel sets `use_property_decorate` to `False` on its own layout before it draws any property.

**Inferred** — the step past those words: reading that line as meaning no decorator is drawn beside any property in this panel while it is on screen. The file says what the panel sets, not what Blender then does with the setting, and no source I have states what it does. The step is mine, so it is labelled.

Either way the screenshot settles nothing. With the step, the missing dot beside Compositing carries no information at all. Without it, the missing dot still proves nothing on its own, because the manual says decorators "may appear" beside animatable properties, not that one appears beside every animatable property.

(Unresolved: the screenshot shows a dot beside Dither, and the quoted panel code sets the same flag for Dither as for the other two. Those two things do not fit together and I have no source that would settle why. It does not change the conclusion.)

### Where I looked

Searches run across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` unless narrowed below.

- `use_compositing` — whole of manual, API, release notes. Hits only in `api_5.0/bpy.types.RenderSettings.txt`, `api_5.0/bpy.types.Scene.txt` (the deprecation note on `use_nodes`), the API indexes, and `cycles_5.0/ui.py`. No hit anywhere describing it changing during a render.
- `mute`, `muted`, `Mute`, `mute_toggle` — whole of manual and API. In the compositing manual: **no hits at all** for `mute` (the compositing section never mentions muting). Hits in `interface/controls/nodes/editing.txt` and `interface/controls/nodes/node_editors.txt` (the Mute command and Mute Links), in the video sequencer, NLA, graph editor and clip editor pages (unrelated senses), and `render/output/audio/speaker.txt` (audio, unrelated). In the API: `bpy.types.Node.mute`, plus the same property on `NodeGroup` and `CompositorNodeGroup` as an inherited member, plus unrelated `mute` on ActionGroup, Constraint, FCurve, FModifier, tracking markers, NLA strips/tracks, sequencer channels, shape keys, strips and strip modifiers (checked via `genindex-M`). Nothing about animating it.
- `animatable`, `not animatable`, `cannot be animated`, `can be animated` — whole of manual and API. In the manual, hits at `interface/controls/buttons/decorators.txt`, the dope sheet and timeline pages, `animation/constraints/interface/header.txt`, the Switch node page, and the Hue/Saturation node pages ("The different values can be animated using a Time Curve node or by setting keyframes"). None of them concern node muting or the scene compositing switch. In the API, the only hits are `Property.is_animatable` and its index entries.
- `animat` across the whole compositing manual — 21 hits, all read. The only one about switching something on and off is the Switch node tip quoted above.
- `every frame`, `each frame`, `per frame`, `for each frame` across `manual_5.0/render` and `manual_5.0/compositing` — nothing about settings being re-read per frame.
- Read in full and ruled out: `manual_5.0/compositing/types/output/file_output.txt` (the File Output node page — it documents Directory, File Name, Media Type, Node Format, Images/Layers, Override Node Format, Item Format, Output Paths, and says the node has no output sockets; it says nothing about muting, enabling, or conditional writing), `manual_5.0/compositing/types/output/enable_output.txt` (the Enable Output node — it "toggles the visibility and value of a socket for a Node Group outputs", i.e. it governs a group's output sockets, not whether a File Output node writes), `manual_5.0/render/output/properties/post_processing.txt`, `manual_5.0/render/output/animation.txt` (Rendering Animations — nothing about settings varying across the frames of a render), `manual_5.0/animation/keyframes/introduction.txt`, `manual_5.0/animation/keyframes/editing.txt` and `manual_5.0/animation/drivers/introduction.txt` (none of them state which properties accept keyframes or drivers), `api_5.0/bpy.app.handlers.txt` (in full), `release_notes_5.0/compositor.md` (in full).
- `conditionally`, `only write`, `skip`, `toggle`, `bypass`, `silenc`, `disable the node`, `muted node` across the compositing manual, the render output manual and the File Output API page — nothing describing a File Output node writing on some frames and not others.
- Cycles' own files: searched for `use_compositing`, `node.mute`, `file_output`, `OutputFile`. The only relevant hit is the Post Processing panel quoted above; Cycles' files say nothing else about the compositor or about File Output nodes.
- The release notes were searched for `mute` and `compositing` across all 23 subject files. The compositor and python_api notes are quoted where relevant. `release_notes_5.0/python_api.md` mentions muted nodes only in the context of a new inlining API for *shader* nodes ("this inlines node groups and eliminates reroute and muted nodes, which simplifies export of shader nodes"), which is not the compositor and not about rendering behaviour.

I did not need the web and did not use it.

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

- Whether either switch is animatable at all. The sources do not say, and I did not run Blender.
- What the compositor actually does if one of them changes between frames of a single render — including whether a change is picked up for that frame, the next frame, or not until the render restarts.
- The other reading of the second half of the question: the node group a File Output node is placed inside, and whether muting *that* group node per frame would stop the File Output node writing.
- Whether a File Output node placed in a scene that is not the one being rendered (for instance reached through a Scene strip in the sequencer) behaves differently.
- Whether anything else at all decides whether a File Output node writes — the question named two things and I answered those two. Question 13 on the sheet covers what causes writing, and I have not trespassed on it.
- The Enable Output node's effect on a File Output node's inputs. It governs a node group's output sockets; whether it could be used to withhold an image from a File Output input per frame is not something the sources address, and I have not inferred it.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

The sources do not say. Nothing I found distinguishes what happens when Blender is driven from outside its own window from what happens when a person is clicking. Three things bear on it and none of them settle it:

- The Mute command itself is described as an interface command with a menu path and the shortcut M, and what it changes is an ordinary property (`Node.mute`), with an operator beside it (`bpy.ops.node.mute_toggle()`). That the property and the operator exist is documented. That either of them can be reached with no interface open is a step past those words, and no source I found states it, so I am not making that step.
- `frame_change_pre` is documented as "Called after frame change for playback and rendering" with no mention of the interface being open, and the same page's warning about the viewport thread is framed as something to guard against with Lock Interface, not as something that limits handlers to interface use.
- One release note is explicit that a command-line render is always an animation render: "The **File Output** node now only appends the frame number to the image name when doing an animation render. Rendering through the command line always does an animation render, so no change of behavior is expected." (Blender 5.0 release notes, `https://developer.blender.org/docs/release_notes/5.0/compositor/`, read 2026-07-30.) That establishes that a command-line render is treated as an animation render. It does not say that an animation render is always more than one frame, and it says nothing about switches changing during it.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources do not say. Nothing in the manual, API documentation, release notes or Cycles' files ties node muting, the Post Processing compositing switch, animation of properties, or frame-change handlers to any particular operating system, and I found no page drawing such a distinction.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 5.0

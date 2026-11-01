# File output nodes — added questions (Blender 5.1)

Sources consulted, and when: Blender's manual for 5.1 (`manual_5.1`, including its `_images` pictures), Blender's Python API documentation for 5.1 (`api_5.1`), Blender's release notes for 5.1 (`release_notes_5.1`), and Cycles' own files for 5.1 (`cycles_5.1`) — all read 2026-07-30. Blender's requirements page, read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 14 only. No other question on the sheet was answered.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**LABEL: not in the sources** (for the question actually asked — whether these things can be made to change from frame to frame inside one render). Two supporting pieces underneath it are documented, and are set out separately below with their own labels.

### The question was ambiguous; here is the reading I answered

I read "the node's own silencing" as the node's **Mute** state, and "whatever governs the compositor it sits in" as the **Compositing** option in the Output properties' Post Processing panel — the switch that decides whether the compositor runs at all. Those are the two things in 5.1 that the sources describe as turning a node, or the compositor, off.

The other reading, which I did not answer, is that "whatever governs the compositor it sits in" means the muting of an enclosing node group, or the assignment of a compositing node group to the scene. I mention below what the sources say about the assignment, because the Compositing option's own wording depends on it, but I did not chase either of them as things that might be made to change from frame to frame.

That second reading is unanswered here and is not covered elsewhere on this sheet. Question 7 asks what happens to a File Output node inside a group of nodes; it does not ask whether an enclosing group's muting, or the scene's assignment of a compositing node group, can be made to vary during a single render. So that reading has been sent back to be researched from the sources afresh.

### Part one — what the two things are (documented)

**The node's own silencing.** The one property I found for this is `mute`. The manual says:

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change."
> "Links will appear red as an indicator of passing through the muted node."

Manual, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html`, read 2026-07-30.

And, in that page's **Sidebar ‣ Node** section:

> "Mute
> Temporarily disables the node without removing it.
> When possible, the node is bypassed and its inputs are passed directly to its outputs."

Manual, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html`, read 2026-07-30. *(Placement corrected by the citation check, 2026-08-06: this was attributed to "the node context menu description". The words are exactly where the address says, but they sit in the page's Sidebar ‣ Node section, whose Reference block reads "Sidebar ‣ Node".)*

In the Python API this is the `mute` property on the base node type, and the entry says nothing beyond its kind and default:

> "mute
> (default False)
> Type:
> bool"

API documentation, `https://docs.blender.org/api/5.1/bpy.types.Node.html`, read 2026-07-30. I read the full property list on that page; `mute` is the only property there that switches a node off. `hide`, `show_options`, `show_preview`, `show_texture`, `select`, `use_custom_color` and `warning_propagation` are the other booleans and enums, and none of them is described as disabling the node. *(`hide` added by the citation check, 2026-08-06: the page carries it as a boolean on `Node` and this list had left it out. It does not change the point — `hide` is not described as disabling anything either.)*

The File Output node type itself carries no on/off property. Its full property list is `active_item_index`, `directory`, `file_name`, `file_output_items`, `format`, `save_as_render` — API documentation, `https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html`, read 2026-07-30.

**What governs the compositor.** The manual says:

> "Pipeline – Compositing
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node."

Manual, `https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html`, read 2026-07-30. The panel is at Properties ‣ Output ‣ Post Processing.

And:

> "To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline
> option must be enabled in the output's Post Processing properties."

Manual, `https://docs.blender.org/manual/en/5.1/compositing/introduction.html`, read 2026-07-30.

In the Python API this is `use_compositing` on the render settings:

> "use_compositing
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

API documentation, `https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html`, read 2026-07-30.

The older switch of the same apparent name no longer does anything in 5.1:

> "use_nodes
> Enable the compositing node group. (default False)
> Deprecated since version 5.0: removal planned in version 6.0
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

API documentation, `https://docs.blender.org/api/5.1/bpy.types.Scene.html`, read 2026-07-30. This matters here: anything aimed at the old switch, per frame or otherwise, has no effect in 5.1.

The release notes record that this switch was not always honoured, and that this was fixed:

> "5.0 - Not always respecting Compositor bypass using Post Processing->Compositing option [...] - Backported to 5.0.1"

Release notes, `https://developer.blender.org/docs/release_notes/5.1/bugfixes/`, read 2026-07-30. (Under the heading "VFX & Video" in that page's list.)

The Cycles files draw this same panel, and show the two switches side by side:

> ```
>         col = layout.column(align=True, heading="Pipeline")
>         col.prop(rd, "use_compositing")
>         col.prop(rd, "use_sequencer")
> ```

`Blender Sources/cycles_5.1/ui.py`, lines 1124–1126, inside `class CYCLES_PT_post_processing` (which begins at line 1112). Label: **observed**. Read 2026-07-30.

### Part two — can either be made to change from frame to frame inside one render? Not in the sources

Nothing in the 5.1 manual, the 5.1 API documentation, the 5.1 release notes or the Cycles files says whether node Mute can be keyframed or driven, whether the Compositing option can be keyframed or driven, or what happens if either changes partway through a render. I searched for this specifically and found no statement either way. What I searched is recorded below.

Three things I did find that sit next to the question, and what each does and does not settle:

**(a) The Switch node is documented as animatable — but it is a different thing.** The manual says:

> "Switch state may be animated by adding a keyframe.
> This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

Manual, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/switch.html`, read 2026-07-30. Label: **documented**, for the Switch node's own state. This is the only place in the 5.1 compositing documentation where a per-node on/off control is said outright to be animatable. It says nothing about node Mute, and a Switch changes which image travels down a link, not whether a File Output node writes a file.

**(b) Nodes can carry keyframes at all.** The manual, describing a Dope Sheet filter, says:

> "If this option is enabled, the Dope Sheet may not show all material
> keyframes of the selected objects. Instead, it only shows the keyframes belonging to the selected nodes
> in the Shader Editor."

Manual, `https://docs.blender.org/manual/en/5.1/editors/dope_sheet/navigating.html`, read 2026-07-30. And node trees are data-blocks with their own animation data:

> "animation_data
> Animation data for this data-block (readonly)"

API documentation, `https://docs.blender.org/api/5.1/bpy.types.NodeTree.html`, read 2026-07-30 (`CompositorNodeTree` inherits it — `https://docs.blender.org/api/5.1/bpy.types.CompositorNodeTree.html`). The release notes also refer to animation on compositor nodes as an existing thing, in a fixed bug:

> "Versioning for animation on compositor nodes older than 4.5 broken"

Release notes, `https://developer.blender.org/docs/release_notes/5.1/bugfixes/`, read 2026-07-30 (under "Nodes & Physics"). Label: **documented** that a node tree is a data-block with its own animation data, and that the release notes speak of animation on compositor nodes as an existing thing.

The Dope Sheet quote is worth being exact about: it is about material keyframes and the Shader Editor, so it carries the general point that nodes can hold keyframes and says nothing about the compositor. And that a node tree can hold keyframes does not tell you that the `mute` property in particular accepts one. I am not making that step.

**(c) There is a documented hook that fires per frame while rendering.** The API documentation says:

> "bpy.app.handlers.frame_change_pre
> Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

API documentation, `https://docs.blender.org/api/5.1/bpy.app.handlers.html`, read 2026-07-30. Label: **documented** that this hook exists and is called for each frame change during rendering, and that it is a place where data may be changed for the new frame.

What it does **not** say — and what I will not claim — is that setting `mute` on a File Output node, or `use_compositing` on the scene, from inside such a hook changes whether that frame's file is written. To get from the quoted words to that claim a reader has to supply an assumption about when the compositor and the File Output node read those two values relative to the hook. The sources say nothing about that. Any answer of that kind would be **inferred**, and I do not have enough to infer it responsibly, so I am leaving it unanswered.

The same page carries a warning about changing data from these hooks during a render:

> "Altering data from handlers should be done carefully. While rendering the
> frame_change_pre and frame_change_post handlers are called from one
> thread and the viewport updates from a different thread. If the handler changes
> data that is accessed by the viewport, this can cause a crash of Blender. In
> such cases, lock the interface (Render → Lock Interface or
> bpy.types.RenderSettings.use_lock_interface) before starting a render."

Same page, read 2026-07-30. Label: **documented**.

### One thing I checked and am deliberately not reading anything into

The manual's picture of the Post Processing panel (`_images/render_output_properties_post-processing_panel.png`, shown on `https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html`, read 2026-07-30) shows a "Compositing" checkbox and a "Sequencer" checkbox side by side, and a "Dither" slider with a small dot to its right. Label for what is visible: **documented**. The manual explains that dot:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property.
> Decorators may appear next to number fields, menus,
> and checkboxes to indicate the property can be animated."

Manual, `https://docs.blender.org/manual/en/5.1/interface/controls/buttons/decorators.html`, read 2026-07-30.

It is tempting to read the missing dot beside Compositing as meaning it cannot be animated. That reading does not hold, because whether decorators are drawn at all is a choice made by whoever wrote the panel. Cycles' own file, which draws a Post Processing panel holding these same two switches, turns decorators off for the whole panel:

> ```
>         layout.use_property_split = True
>         layout.use_property_decorate = False
> ```

`Blender Sources/cycles_5.1/ui.py`, lines 1119–1120, inside `class CYCLES_PT_post_processing`. Label: **observed**. Read 2026-07-30.

Nothing on the manual page says which render engine's panel the picture was taken from, so it cannot even be said which file drew the panel in the picture. Either way the picture settles nothing about animatability in either direction, and I am not using it as evidence.

### What running Blender 5.1 showed — observed, 2026-08-06

Run by Opus 5 (Claude Code) on **a rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all, reached over SSH**. Blender 5.1.2 at `/opt/blender/5.1/blender`, started with no window, with `--factory-startup`, and pointed at a settings folder of its own made beforehand. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **These results are the Linux half.** Every one of these tests was run again on the Creator's Mac, and that half is the section immediately below. Windows was not tried at all: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Which of these switches Blender will animate at all** (entry 1). Asked for each property's own animatable flag, then asked to insert a keyframe and to add a driver on each. What came back:

```
RESULT [5.1.2] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [5.1.2] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [5.1.2] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
```

The word `mute` was typed at a link first and came back "not found"; a second run listed every property a link has and tried the name that turned up:

```
RESULT [5.1.2] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. Three of the four switches stopped the node writing; `scene.use_nodes` did not, and would not even take the value:

```
RESULT [5.1.2] (i) nothing switched off
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.1.2] (ii) the node muted for the whole render
RESULT [5.1.2] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as True
RESULT [5.1.2] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.1.2] (v) the link feeding the node muted for the whole render
```

(The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.)

*(Note added by the citation check, 2026-08-06, in both this block and its copy in the Mac section below: the line "after setting scene.use_nodes = False it reads back as ..." appears under heading (iii), the Compositing switch, but it is about `scene.use_nodes` and belongs to (iv). It is where the running script printed it, not evidence about the Compositing switch. The file counts under each heading are right and were reproduced by this check.)*

**A keyframed `scene.use_nodes` across one animation render** (entry 3). Keyed on at frame 1 and off at frame 3, keys flattened to Constant, frames 1 to 4 rendered as one animation. The curve recorded the value 1 at both keys, because the second setting never took:

```
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1, frame 3 = 1
RESULT [5.1.2] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png
    out/file_nameimg0002.png
    out/file_nameimg0003.png
    out/file_nameimg0004.png
RESULT [5.1.2] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = True
    frame 4 -> scene.use_nodes = True
```

The same test with Cycles on the processor instead of Workbench gave the same four files.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [5.1.2] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
    handler ran: frame 3, render.use_compositing set to False
RESULT [5.1.2] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
    handler ran: frame 3, scene.use_nodes set to False, reads back True
RESULT [5.1.2] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [5.1.2] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.1.2] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.1.2] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved, and rendered twice from a terminal:

```
RESULT [5.1.2] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = True
--- command: /opt/blender/5.1/blender -b  /root/file-output-nodes/work/driven-5.1.2.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /opt/blender/5.1/blender -b -y /root/file-output-nodes/work/driven-5.1.2.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [5.1.2] (i) the group not muted
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.1.2] (ii) the group node muted for the whole render
RESULT [5.1.2] (iii) handler mutes the group node on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.1.2] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which node group the scene composites with** (entry 7). A second group with no File Output node in it was built, and the scene pointed at each in turn, then swapped per frame by a handler. The printing here was flushed line by line, so the order shown is the true one:

```
RESULT [5.1.2] (i) the scene set to the group holding the File Output node
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.1.2] (ii) the scene set to a group with no File Output node, for the whole render
RESULT [5.1.2] (iii) the scene set to no compositing group at all
    handler ran: frame 1, scene.compositing_node_group set to Comp
00:01.209  render           | Saved: '/root/file-output-nodes/work/fo/out/file_nameimg0001.png'
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
00:01.229  render           | Saved: '/root/file-output-nodes/work/fo/out/file_nameimg0002.png'
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
    handler ran: frame 1, scene.compositing_node_group set to Comp
RESULT [5.1.2] (iv) handler swaps to the writer-less group on frames 2 and 5 only
    out/file_nameimg0001.png
    out/file_nameimg0002.png
RESULT [5.1.2] can that pointer be keyframed
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
```

The handler printed for frames 1 and 2 and not again, while `render_write` printed for all six frames. No error and no traceback appeared anywhere in that run — but that on its own settles nothing, because a Blender that honoured the swap and a Blender that ignored it would both be quiet; what carries the point is the file count beside it, and that does tell the two apart. *(Sentence qualified by the citation check, 2026-08-06, which reran this test and saw the same interleaving, the same silence and the same two files.)* (The main render output's own `Saved:` lines sat between these and are cut from the quote; it was written on all six frames.)

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [5.1.2] the tree's output node is a NodeGroupOutput
RESULT [5.1.2] (ii) the output node muted for the whole render
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
RESULT [5.1.2] (iv) the output node removed from the tree altogether
    out/file_nameimg0001.png … out/file_nameimg0006.png  (six files)
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /opt/blender/5.1/blender -b /root/file-output-nodes/work/keyed-5.1.2.blend -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /opt/blender/5.1/blender -b /root/file-output-nodes/work/plain-5.1.2.blend --python /root/file-output-nodes/work/flip.py -a
    handler ran: frame 2, node.mute set to True
    handler ran: frame 5, node.mute set to True
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0004.png
    file_nameimg0005.png
```

### What running Blender 5.1 on the Creator's Mac showed — observed, 2026-08-07

Run by Opus 5 (Claude Code) on **the Creator's own Mac: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, pointed at a settings folder of its own made beforehand. Entries 1 to 9 were run with no window and with `--factory-startup`, exactly as on Linux. Entries 10 and 11 were run with Blender's window open, which is something the Linux machine cannot do at all, having no display server. Every set-up and every line typed is written out in full in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`; the entry numbers below are that list's. **Nothing here was timed and nothing here measured memory**, because other topics were running Blender on the same Mac at the same time; every result is a count of files or a line Blender printed.

**Every test that had been run on Linux was run again here, and the two machines agreed on every one of them.** Same scripts, same commands, same four versions. The file counts and the lines Blender printed matched the Linux run word for word. The only difference anywhere is the size in bytes of the pictures, which is the two machines writing the same picture rather than a different one, so no file was compared byte for byte.

**Which of these switches Blender will animate at all** (entry 1). What came back on the Mac:

```
RESULT [5.1.2] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
RESULT [5.1.2] what keyframe_insert returns
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
RESULT [5.1.2] what driver_add returns
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
RESULT [5.1.2] where the curves ended up
    fcurves on the compositor node tree: 0
    fcurves on the scene: 1
        use_nodes [0]  frame 1 = 1
```

And the link's own silencing, asked by its real name:

```
RESULT [5.1.2] keyframe and driver on the link's silencing
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
    link.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not found
    link.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not found
```

**Each switch simply set for a whole render** (entry 2). Frames 1 to 4 rendered five times, one thing changed each time, the folder emptied in between. The main render output was written on all four frames in all five runs and is cut from the quote; a heading with nothing under it means the folder held no File Output file.

```
RESULT [5.1.2] (i) nothing switched off
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
RESULT [5.1.2] (ii) the node muted for the whole render
RESULT [5.1.2] (iii) Compositing switched off for the whole render
    after setting scene.use_nodes = False it reads back as True
RESULT [5.1.2] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
RESULT [5.1.2] (v) the link feeding the node muted for the whole render
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
RESULT [5.1.2] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
RESULT [5.1.2] what scene.use_nodes read at each frame
    frame 1 -> scene.use_nodes = True
    frame 2 -> scene.use_nodes = True
    frame 3 -> scene.use_nodes = True
    frame 4 -> scene.use_nodes = True
```

The same test with Cycles on the processor instead of Workbench left the same four files, at 1955 bytes each, so the engine makes no difference on this machine either.

**A per-frame script handler** (entry 4). A `frame_change_pre` handler was registered, `Render ‣ Lock Interface` switched on, and frames 1 to 4 rendered as one animation, once per switch:

```
    handler ran: frame 3, node.mute set to True
RESULT [5.1.2] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    handler ran: frame 3, render.use_compositing set to False
RESULT [5.1.2] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    handler ran: frame 3, scene.use_nodes set to False, reads back True
RESULT [5.1.2] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    handler ran: frame 3, link.is_muted set to True
RESULT [5.1.2] (D) frame_change_pre sets the link's mute on from frame 3
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
```

The same test with a `render_pre` handler instead gave the same counts — three files for the node's silencing, four for the Compositing switch.

**Which frame the handler's change lands on** (entry 4, second script). The range was stretched to 1 to 6 and each thing was switched off on frames 2 and 5 only:

```
RESULT [5.1.2] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
RESULT [5.1.2] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.1.2] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
```

**A frame-dependent driver, from a saved file, with and without automatic script running** (entry 5). A driver with the expression `frame < 3` was added to `scene.use_nodes`, the file saved on the Mac, and rendered twice from the Mac's terminal:

```
RESULT [5.1.2] the driver as built
    expression = 'frame < 3'  type = SCRIPTED
    frame 3 -> scene.use_nodes = True
--- command: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender -b <the saved file> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender -b -y <the saved file> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
```

**The node inside a group of nodes** (entry 6). The File Output node was put inside a group, the group fed from the Render Layers node, frames 1 to 6 rendered three times:

```
RESULT [5.1.2] (i) the group not muted
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.1.2] (ii) the group node muted for the whole render
RESULT [5.1.2] (iii) handler mutes the group node on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
RESULT [5.1.2] is the group node's mute animatable
    CompositorNodeGroup.mute -> is_animatable=False
    group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
```

**Swapping which tree the scene composites with** (entry 7):

```
RESULT [5.1.2] (i) the scene set to the group holding the File Output node
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.1.2] (ii) the scene set to a group with no File Output node, for the whole render
RESULT [5.1.2] (iii) the scene set to no compositing group at all
    handler ran: frame 1, scene.compositing_node_group set to Comp
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
RESULT [5.1.2] (iv) handler swaps to the writer-less group on frames 2 and 5 only
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
RESULT [5.1.2] can that pointer be keyframed
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
```

The handler printed for frames 1 and 2 and not again, exactly as on Linux; `render_write` printed for all six frames. In run (iv) the main render output for frames 3 to 6 came out at 515 bytes against 773 bytes for frames 1 and 2. No error and no traceback appeared anywhere.

**The tree's own output node** (entry 8). Frames 1 to 6 rendered four times — output node present, muted throughout, muted by a handler on frames 2 and 5, and taken out of the tree altogether. All four runs wrote six files:

```
RESULT [5.1.2] the tree's output node is a NodeGroupOutput
RESULT [5.1.2] (ii) the output node muted for the whole render
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
RESULT [5.1.2] (iv) the output node removed from the tree altogether
    out/file_nameimg0001.png  929 bytes
    out/file_nameimg0002.png  929 bytes
    out/file_nameimg0003.png  929 bytes
    out/file_nameimg0004.png  929 bytes
    out/file_nameimg0005.png  929 bytes
    out/file_nameimg0006.png  929 bytes
```

**A saved file rendered from a real command line** (entry 9). The keyed file, and a file with no keyframes rendered with a handler script named before `-a`:

```
--- command: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender -b <keyed-5.1.2.blend> -a
--- files written by the File Output node:
    file_nameimg0001.png
    file_nameimg0002.png
    file_nameimg0003.png
    file_nameimg0004.png
--- command: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender -b <plain-5.1.2.blend> --python <flip.py> -a
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
- In the compositor's sidebar, with a File Output node selected, the **Node** panel lists Name, Label, Color, Show Options and Mute. No decorator sits beside any row in that panel.

The pictures are kept at `/Users/artist/Documents/Smoozy hand tests/File output nodes/shots/`.

**Whether a render started with Blender's interface open agrees with one started with no window** (entry 11). The two saved files from entry 9 were rendered again on the Mac, this time with a window on screen instead of `-b`, and with `Render ‣ Lock Interface` switched on:

```
--- command: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender <keyed-5.1.2.blend> --python <windowed.py>
    out/file_nameimg0001.png  999 bytes
    out/file_nameimg0002.png  999 bytes
    out/file_nameimg0003.png  999 bytes
    out/file_nameimg0004.png  999 bytes
--- command: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender <plain-5.1.2.blend> --python <flip.py> --python <windowed.py>
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

- Whether `mute` on a node accepts a keyframe or a driver at all. The sources do not say, and I found no way to settle it by reading.
- Whether `use_compositing` accepts a keyframe or a driver at all.
- If either can be changed per frame, whether the change takes effect for the frame it lands on, the next one, or not at all — that is, when during a frame the compositor and the File Output node read these values. The sources describe none of this ordering.
- Whether a mid-render change to the Compositing option is picked up at all once a render job has started, as opposed to being read once when the job begins.
- Whether muting a File Output node stops it writing. That is question 8's ground and I did not answer it here; this answer is only about whether the silencing can be made to vary per frame.
- Group muting, and swapping the scene's assigned compositing node group per frame, as alternative ways to reach the same end.
- Anything about what happens when a frame is skipped or re-rendered.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

Both things named here are ordinary data properties reachable without the interface: `Node.mute` and `RenderSettings.use_compositing` in the Python API, cited above. The command line offers the pieces needed to drive a render without a person clicking:

> "-b, --background
> Run in background (often used for UI-less rendering)."
> "-a, --render-anim
> Render frames from start to end (inclusive)."
> "-P, --python <filepath>
> Run the given Python script file."

Manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, read 2026-07-30. Label: **documented**.

One documented difference bears on the driver route in particular. Automatic Python execution — the kind Blender starts on its own, named here as drivers and startup scripts — is off by default:

> "-Y, --disable-autoexec
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Same page, read 2026-07-30. And the manual repeats it for drivers:

> "By default Blender will restrict execution of Python scripts."
> "If using a Scripted Expression Driver Type that doesn't follow
> the Simple Expressions
> subset, you will have to open the file as Trusted Source,
> or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files."

Manual, `https://docs.blender.org/manual/en/5.1/animation/drivers/troubleshooting.html`, read 2026-07-30. Label: **documented**. So a driver written as a scripted expression beyond the simple subset will not run unless auto-run is enabled or the file is opened as a trusted source — which is a real difference for an unattended render started from a command line.

That is about scripts Blender starts by itself. It is not a statement about a script named on the command line with `-P`, which the same option list carries as its own separate entry, quoted above.

Beyond that: the sources do not say whether the main answer differs between a render driven from the interface and one driven from outside it, because the sources do not give the main answer in the first place. The handler warning quoted above concerns a viewport updating on another thread, and a background render has no viewport — but the sources do not say that, so I am not claiming it.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources say nothing that makes any of the above depend on the operating system. Nothing in the Mute description, the Post Processing description, the `use_compositing` entry, the handlers page or the command-line page is marked as applying to one system and not another. The only operating-system marking I found anywhere near this material was on an unrelated command-line option ("--no-window-frame … Disable all window decorations (Linux only)", same command-line page). *(Option name corrected by the citation check, 2026-08-06: it was written here as `--no-window-decorations`. The page's entry is `--no-window-frame`; the description under it was quoted right.)*

For completeness, the systems in play: the requirements page names Windows, macOS and Linux, and says of the version researched here, in so many words, "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." — `https://www.blender.org/download/requirements/`, read 2026-08-01. That is a statement about which machines Blender runs on, not about this question.

### Where I looked

Searched across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` unless noted.

Terms searched: `mute`, `muting`, `muted`, `mute_toggle`, `bypass`, `animat` (catching animate, animated, animatable, animation), `animatable`, `keyframe`, `keyframing`, `driver`, `drivers`, `use_compositing`, `use_nodes`, `compositing_node_group`, `Post Processing`, `Compositing Pipeline`, `Pipeline – Compositing`, `can be animated`, `cannot be animated`, `not animatable`, `non-animatable`, `mute can be animated`, `animate the mute`, `keyframe the mute`, `animated mute`, `muting can be`, `re-evaluated`, `evaluated per frame`, `evaluated for each frame`, `render animation`, `lock interface`, `auto run`, `autoexec`, `.mute`.

Pages opened in full or in the relevant part:

- Manual: `interface/controls/nodes/editing.html`, `interface/controls/nodes/node_editors.html`, `interface/controls/nodes/groups.html` (animation section), `interface/controls/buttons/decorators.html`, `compositing/introduction.html`, `compositing/types/output/file_output.html`, `compositing/types/output/enable_output.html`, `compositing/types/output/index.html`, `compositing/types/utilities/switch.html`, `render/output/properties/post_processing.html`, `render/output/animation.html`, `animation/introduction.html`, `animation/keyframes/introduction.html`, `animation/keyframes/editing.html`, `animation/drivers/introduction.html`, `animation/drivers/usage.html`, `animation/drivers/troubleshooting.html`, `editors/dope_sheet/navigating.html`, `advanced/command_line/arguments.html`.
- Manual pictures: `_images/render_output_properties_post-processing_panel.png` (the Post Processing panel), `_images/interface_controls_nodes_sidebar_item.png` (the node sidebar, opened to see whether Mute appears there as a property with a decorator — it does not; the sidebar shows Name, Label and Color only).
- API documentation: `bpy.types.Node.html`, `bpy.types.CompositorNodeOutputFile.html`, `bpy.types.NodeTree.html`, `bpy.types.CompositorNodeTree.html`, `bpy.types.Scene.html`, `bpy.types.RenderSettings.html`, `bpy.types.Property.html`, `bpy.types.bpy_struct.html` (the `keyframe_insert` entry), `bpy.app.handlers.html`, `bpy.ops.node.html` (the `mute_toggle` entry).
- Release notes for 5.1: `compositor/`, `pipeline_io/`, `python_api/`, `bugfixes/`.
- Cycles files: `ui.py` (searched for `use_compositing` and `mute`; the Post Processing panel class is the only relevant place. The two other hits for `use_compositing` are in `source/kernel/device/gpu/kernel.h` and are a kernel argument of the same name, unrelated to the scene switch).

Ruled out, and why:

- **The API documentation as a way to settle animatability.** The 5.1 API pages carry no per-property "animatable" marking. The only occurrences of the word in the whole of `api_5.1` are the `ANIMATABLE` option name in the `bpy.props` function signatures, and the entry `Property.is_animatable` — "Property is animatable through RNA (default False, readonly)" (`https://docs.blender.org/api/5.1/bpy.types.Property.html`, read 2026-07-30), which is a value a running Blender can be asked for, not a statement the documentation makes about `mute` or `use_compositing`. So no amount of reading the API pages can answer this; it has to be asked of a running Blender.
- **`bpy_struct.keyframe_insert`.** Reads "Insert a keyframe on the property given, adding fcurves and animation data when necessary." — general, with no list of which properties qualify (`https://docs.blender.org/api/5.1/bpy.types.bpy_struct.html`, read 2026-07-30).
- **The general animation pages.** `animation/keyframes/introduction.html` and `animation/keyframes/editing.html` explain what a keyframe is and how to insert one, and name no set of animatable properties. The drivers pages likewise describe how drivers are built, not what may be driven.
- **`render/output/animation.html`.** Describes the two workflows for rendering a sequence and the Frame Step and Overwrite options. Says nothing about settings changing during a render.
- **The Enable Output node.** Read in full; it toggles a node group's output socket, not a File Output node's writing, so it is not the "silencing" the question means.
- **Cycles files generally.** Cycles is a render engine; it draws the Post Processing panel but has no part in muting compositor nodes. The only relevant lines are the two blocks quoted above.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 5.1

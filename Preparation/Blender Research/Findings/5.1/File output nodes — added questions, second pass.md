# File output nodes — added questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender's 5.1 manual, Blender's 5.1 Python API documentation, Blender's 5.1 release notes and Cycles' own 5.1 files, all read on 2026-07-30 (the date the copies were taken). Blender's requirements page, read on 2026-08-01; it does not name Blender 5.1 anywhere, so I have not used it to say anything about 5.1.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 14 only. The rest of the sheet was left alone.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**How I read the question.** It was ambiguous in one place, so I say plainly which reading I answered. I took "the node's own silencing" to mean muting the node — the Mute switch on a node, `Node.mute` in the Python API. I took "whatever governs the compositor it sits in" to mean the switch that turns the whole compositing stage of a render on or off — the Compositing option in Post Processing, `RenderSettings.use_compositing` in the Python API. The other reading of "silencing" would be pulling the node's inputs off, or the separate Enable Output node; I have not answered that reading, and I say below what the sources hold about the Enable Output node so nobody has to guess.

**LABEL: not in the sources** for the question actually asked — whether either switch can be made to change from frame to frame inside one render. The sources describe both switches plainly. They never say either can be keyframed, driven or otherwise varied over time, and they never say it cannot. Parts of the surrounding ground are documented, and I set those out separately below so the gap is visible rather than papered over.

### What the two switches are — documented

The node's own silencing. The manual describes muting twice.

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html`, read 2026-07-30.

> "Temporarily disables the node without removing it.
> When possible, the node is bypassed and its inputs are passed directly to its outputs."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html`, read 2026-07-30.

In the Python API the switch is a plain on/off value on every node, and the File Output node inherits it. The API page for the File Output node lists `Node.mute` among its inherited properties.

> "mute
>
> (default False)
>
> Type:
>
> bool"

Blender 5.1 Python API, `https://docs.blender.org/api/5.1/bpy.types.Node.html`, read 2026-07-30. The inherited listing is on `https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html`, same date.

The 5.1 release notes record that the switch also became reachable from the node sidebar in this version.

> "- \"Node\" panel now has toggles to \"Show Options\", \"Mute\", placeholders in text inputs, and color settings are now aligned in a row."

Blender 5.1 release notes, `https://developer.blender.org/docs/release_notes/5.1/user_interface/`, read 2026-07-30.

What governs the compositor. The manual is explicit that a single option in Post Processing decides whether the compositing node tree is used at all.

> "To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline
> option must be enabled in the output's Post Processing properties."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/compositing/introduction.html`, read 2026-07-30.

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html`, read 2026-07-30.

In the Python API it is one on/off value on the render settings.

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

Blender 5.1 Python API, `https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html`, read 2026-07-30.

One thing worth naming, because an older switch still exists under a familiar name and now does nothing:

> "use_nodes
>
> Enable the compositing node group. (default False)
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

Blender 5.1 Python API, `https://docs.blender.org/api/5.1/bpy.types.Scene.html`, read 2026-07-30.

There is a third thing that governs whether the compositor runs, which the question did not name but which sits in the same place:

> "The compositor searches for the active Group Output node, if none was found, it searches for the
> active Viewer node. If none was found, the compositor doesn't run altogether."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html`, read 2026-07-30.

### Why the answer to the question asked is "not in the sources"

Nothing in the 5.1 manual, the 5.1 API documentation, the 5.1 release notes or Cycles' 5.1 files says that a node's Mute switch can carry a keyframe or a driver, and nothing says the Compositing option in Post Processing can. Nor does anything say they cannot. Neither page above mentions animation, keyframes, drivers or frames at all.

### What the sources do say nearby, kept separate from the answer

These are real quotes about neighbouring things. None of them settles the question; I list them so a reader can see exactly how far the sources reach.

**Some compositor node settings are documented as animatable — but the one named is a socket on a node, not the Mute switch.** The Switch node's page says, in a Tip:

> "Switch state may be animated by adding a keyframe.
> This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/compositing/types/utilities/switch.html`, read 2026-07-30. This is the only place in the 5.1 compositing documentation I found that says a compositor setting can be made to change over the course of a sequence, and it is about an input on the Switch node.

The compositing workspace page says the same thing in general terms:

> "It allows scrubbing through frames, controlling playback,
> and animating values that affect the compositing process."

and

> "This is especially useful when compositing image sequences or video,
> or when animating node parameters."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/compositing/compositing_workspace.html`, read 2026-07-30. "Node parameters" is not defined there, and the Mute switch is documented elsewhere as a show/hide control rather than a parameter.

**A compositing node tree can hold animation.** The compositing node tree is a data-block with animation data attached:

> "animation_data
>
> Animation data for this data-block (readonly)"

Blender 5.1 Python API, `https://docs.blender.org/api/5.1/bpy.types.NodeTree.html`, read 2026-07-30 (`CompositorNodeTree` inherits it — `https://docs.blender.org/api/5.1/bpy.types.CompositorNodeTree.html`). The scene carries the same thing (`https://docs.blender.org/api/5.1/bpy.types.Scene.html`), and the node group interface has a panel for it:

> "Animation
>
> Controls animation data for node group properties, including active Actions
> and their assigned Slot."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/groups.html`, read 2026-07-30.

The 5.1 release notes list fixes that only make sense if compositor nodes can hold animation, for example:

> " * Versioning for animation on compositor nodes older than 4.5 broken"

Blender 5.1 release notes, `https://developer.blender.org/docs/release_notes/5.1/bugfixes/`, read 2026-07-30.

None of this says which properties may be animated. That is a per-property matter, and the API documentation says so without publishing the value for any particular property:

> "is_animatable
>
> Property is animatable through RNA (default False, readonly)"

Blender 5.1 Python API, `https://docs.blender.org/api/5.1/bpy.types.Property.html`, read 2026-07-30. So whether `Node.mute` and `RenderSettings.use_compositing` are animatable is a fact Blender itself holds and can be asked for, but the documentation does not print it. See the hand-check list at the end.

**Cycles' own files show that the Post Processing panel sets decorators off for every row it draws, including the Compositing switch.** LABEL for this paragraph: **observed**, for what the file sets. What is actually *drawn* is a step past the file, and for that the evidence is the screenshot in entry 10 below, not this. *(Corrected by the citation check, 2026-08-06: this heading read "deliberately draws no keyframe button beside the Compositing switch", which the file does not carry — the paragraph three lines down already said so, and the heading contradicted it.)*

> `        layout.use_property_decorate = False`
>
> …
>
> `        col = layout.column(align=True, heading="Pipeline")`
> `        col.prop(rd, "use_compositing")`
> `        col.prop(rd, "use_sequencer")`

File `Blender Sources/cycles_5.1/ui.py`, line 1120 and lines 1124–1126, inside `class CYCLES_PT_post_processing` (which begins at line 1112), read 2026-07-30.

The manual explains what those small buttons are:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property.
> Decorators may appear next to number fields, menus,
> and checkboxes to indicate the property can be animated."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/interface/controls/buttons/decorators.html`, read 2026-07-30.

Careful about how far this goes. What the file says is that this panel turns decorators off for the rows it draws. That is observed. Reading that as "the Compositing switch cannot be animated" would be a step past the file and would be wrong to label observed — a panel can hide the button for a property that is animatable, and this panel hides it for every row it draws, including Dither. All I can say is: with Cycles as the render engine, there is no one-click keyframe button beside Compositing in that panel.

**A picture in the manual, for completeness.** `_images/render_output_properties_post-processing_panel.png`, shown on the Post Processing page, shows the panel with Compositing and Sequencer as two checkboxes on one row and a Dither slider beneath. LABEL: **documented** for what is visible — those are the controls in the panel. I do not read anything about animation out of it.

**There is a documented way to change data on every frame of a render, but the sources do not say what happens if you change these two switches that way.** The API documentation describes handlers that fire per frame during rendering:

> "Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

(that is `bpy.app.handlers.frame_change_pre`), and

> "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

(that is `bpy.app.handlers.render_write`). Blender 5.1 Python API, `https://docs.blender.org/api/5.1/bpy.app.handlers.html`, read 2026-07-30. The same page warns that altering data from handlers during a render needs care:

> "Altering data from handlers should be done carefully. While rendering the
> frame_change_pre and frame_change_post handlers are called from one
> thread and the viewport updates from a different thread."

What the sources never say is whether setting `mute` or `use_compositing` from such a handler is picked up for that same frame's compositing, or whether the compositing stage reads those values once when the render job starts. That is the heart of the question and it is simply absent.

**The other reading of "silencing", for the record.** The File Output node's individual outputs, as described in the API, carry a colour, a format, a name, an override flag, a save-as-render flag, a socket type and the socket's dimensions — there is no per-output on/off flag among them (`https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html`, read 2026-07-30). *(List completed by the citation check, 2026-08-06: it named four of the page's seven entries, leaving out `color`, `save_as_render` and `vector_socket_dimensions`. None of the three is an on/off flag for writing, so the point stands.)* The separate Enable Output node exists but is documented as acting on a node group's outputs, not on a File Output node:

> "The Enable Output node toggles the visibility and value
> of a socket for a Node Group outputs."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/compositing/types/output/enable_output.html`, read 2026-07-30.

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

- Whether `Node.mute` or `RenderSettings.use_compositing` will accept a keyframe or a driver at all. The documentation does not say; the flag that would answer it exists but is not printed.
- If they do accept one, whether the render pipeline re-reads them for each frame of an animation render, or reads them once when the job starts. Entirely absent from the sources.
- Whether a change made from a per-frame handler lands in time to affect that frame's compositing.
- Whether the File Output node consults the mute state at the moment it writes, or earlier.
- Whether the active-output rule ("the compositor doesn't run altogether") can be made to vary per frame. I did not chase this, as the question named only the two switches.
- The other reading of "silencing" — disconnecting inputs, or the Enable Output node — is not answered here.
- Any Blender version other than 5.1.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

Does this hold when Blender is driven from outside its own window? The sources do not say anything specific about muting or about the Compositing switch under command-line rendering. The command-line pages describe rendering a whole animation without a window —

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html`, read 2026-07-30 — but say nothing about either switch.

One documented difference does bear on any answer that would rely on drivers, and it is easy to miss:

> "-Y, --disable-autoexec
>
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, read 2026-07-30. So Python-expression drivers do not run by default when Blender is started from a command line; `-y, --enable-autoexec` turns them on. Not every driver needs it, though:

> "When a driver uses a Scripted Expression, Blender can evaluate it without using
> the fully featured Python interpreter if it is simple enough."

and, from the drivers troubleshooting page:

> "If using a Scripted Expression Driver Type that doesn't follow
> the Simple Expressions
> subset, you will have to open the file as Trusted Source,
> or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/animation/drivers/introduction.html` and `https://docs.blender.org/manual/en/5.1/animation/drivers/troubleshooting.html`, read 2026-07-30.

The sources list `-P, --python <filepath>` — "Run the given Python script file" — as a separate argument from the auto-execution setting, but do not state whether a script given that way runs when automatic execution is disabled. I am not going to guess.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources do not say. Nothing in the 5.1 manual, API documentation, release notes or Cycles' files ties muting or the Compositing switch to any particular operating system. The only operating-system-specific statement I found anywhere near the compositor is about the graphics driver resetting, which is a different matter entirely:

> "Timeout Detection Recovery
> (TDR)
> resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Blender 5.1 manual, `https://docs.blender.org/manual/en/5.1/compositing/limitations.html`, read 2026-07-30.

Blender's requirements page (`https://www.blender.org/download/requirements/`, read 2026-08-01) does not name Blender 5.1 anywhere, so by its own terms it cannot speak for this version, and I have not used it.

### Where I looked

Searched across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` unless noted.

Terms searched: `mute`, `muted`, `mute_toggle`, `bypass`, `silenc`, `suppress`, `animat`, `animatable`, `non-animatable`, `cannot be animated`, `can be animated`, `keyframe`, `keyframed`, `driver`, `drivers`, `animation_data`, `use_compositing`, `use_nodes`, `compositing_node_group`, `Compositing Pipeline`, `post processing`, `post-processing`, `persistent data`, `use_persistent_data`, `lock interface`, `use_lock_interface`, `--python`, `frame-jump`.

Pages opened and read in full or in the relevant part:

- File Output node — `https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html`. No mention of muting, animation or per-frame behaviour.
- Enable Output node — `.../compositing/types/output/enable_output.html`. About node group sockets, not File Output.
- Compositing introduction — `.../compositing/introduction.html`. Names the governing option.
- Compositor System — `.../compositing/compositor_system.html`. Names the active-output rule.
- Compositing limitations — `.../compositing/limitations.html`. Hardware only.
- Compositing sidebar — `.../compositing/sidebar.html`. Backdrop and performance only; no mute, no animation.
- Compositing workspace — `.../compositing/compositing_workspace.html`.
- Switch node — `.../compositing/types/utilities/switch.html`.
- Node editing — `.../interface/controls/nodes/editing.html`. The Mute entry.
- Node editors — `.../interface/controls/nodes/node_editors.html`. The sidebar Mute entry.
- Node groups — `.../interface/controls/nodes/groups.html`. The Animation panel entry.
- Decorators — `.../interface/controls/buttons/decorators.html`.
- Post Processing — `.../render/output/properties/post_processing.html`, and its picture `_images/render_output_properties_post-processing_panel.png`.
- Rendering Animations — `.../render/output/animation.html`. Covers workflow and the Overwrite, Placeholder and Frame Step options for the main render output; says nothing about the compositor being switched on or off per frame.
- Keyframes introduction and editing — `.../animation/keyframes/introduction.html`, `.../animation/keyframes/editing.html`. Describe how keys are inserted; say nothing about which properties accept them.
- Drivers introduction, usage, troubleshooting — `.../animation/drivers/introduction.html`, `.../animation/drivers/usage.html`, `.../animation/drivers/troubleshooting.html`. Same: no list of drivable properties.
- Command line rendering and arguments — `.../advanced/command_line/render.html`, `.../advanced/command_line/arguments.html`.
- Cycles performance settings — `.../render/cycles/render_settings/performance.html`. Read to rule out Persistent Data as something that would freeze compositor settings across frames; it speaks only about render data kept in memory and says nothing about the compositor.
- API: `bpy.types.CompositorNodeOutputFile`, `bpy.types.Node`, `bpy.types.NodeTree`, `bpy.types.CompositorNodeTree`, `bpy.types.Scene`, `bpy.types.RenderSettings`, `bpy.types.Property`, `bpy.types.NodeCompositorFileOutputItem`, `bpy.types.bpy_struct` (the `keyframe_insert` entry), `bpy.app.handlers`, `bpy.ops.node` (the `mute_toggle` entry), `bpy.props`.
- Release notes 5.1: the compositor, user interface, Python API and bug fixes pages.
- Cycles 5.1: `ui.py` (the Post Processing panel), and a search of the whole folder for `use_compositing`, `compositing` and `mute`. Nothing there concerns node muting; Cycles ships the Post Processing panel and nothing else relevant.

Ruled out and why: the release notes for 5.1 contain no entry about muting nodes, about animating node flags, or about the Compositing switch. The Cycles files contain no compositor code — the compositor is not part of Cycles — so the only thing they could contribute was the panel drawing, which they did. The requirements page never names 5.1 and so was set aside.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 5.1

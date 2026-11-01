# File output nodes — added questions (Blender 4.5)

Sources consulted, and when: Blender's 4.5 manual, Blender's 4.5 Python API documentation, the 4.5 release notes, and Cycles' own files, all read on 2026-07-30 from the copies in `Blender Sources/`. Blender's requirements page as taken on 2026-08-01. Nothing else.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: this document answers question 14 only. Every other question on the sheet was left alone.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

### How I read the question

The question is ambiguous in two places, and I say so rather than choosing quietly.

"The node's own silencing" is not a term Blender uses. I read it as **Mute**, the per-node switch described in the manual under Node ‣ Toggle Node Mute and exposed in the API as `Node.mute`. That is the only per-node off switch I found in the sources.

"Whatever governs the compositor it sits in" I read as the switches that decide whether the compositor runs at all for a render. The sources gave me three: the **Compositing** checkbox in Properties ‣ Output ‣ Post Processing (`RenderSettings.use_compositing`), the **Use Nodes** checkbox in the compositor's header (`Scene.use_nodes`), and — separately — the presence of an active output node in the tree. The other reading would be that only the Compositing checkbox was meant; the answer for that switch alone is the same as below.

"Inside a single render" I read as one animation render, started once and left to run over a frame range. The other reading — a person or a script rendering one frame at a time and changing things between calls — is a different thing, and I treat it separately under **Outside the interface**, because there the answer changes.

### Answer

**A. Whether Mute can be made to change from frame to frame during a render — not in the sources.**

The sources describe what Mute does, and they show it as a plain writable setting, but they nowhere say whether it can be keyframed or driven, and nowhere say what happens if it changes while a render is running.

What they do say:

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30. Label: **documented** (this is what Mute is, not whether it can vary per frame).

In the API documentation `mute` is listed on `Node` with no restriction:

> "mute
>
> Type:
>
> boolean, default False"

Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30. Label: **documented** that the page lists this setting with that type and that default. It is not marked `(readonly)`, unlike neighbouring entries such as `dimensions` and `internal_links` which are — but the quoted words do not say the setting can be written to. Reading it as writable is a step: these pages mark read-only entries as read-only, and this one is not so marked. Label for that step: **inferred**. Whether it can be animated is **not in the sources**. Only one term was searched for an animatability marker across the API documentation, so that silence is thin and nothing here leans on it — see *Where I looked*.

`CompositorNodeOutputFile` inherits `Node.mute`, and also inherits `bpy_struct.keyframe_insert` and `bpy_struct.driver_add`. I am deliberately not treating that as an answer: the page carries them as inherited from `bpy_struct` rather than as anything belonging to this property, so their presence says nothing about this property in particular. That reading is mine and not the page's — label: **inferred**. (https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.)

**B. Whether the switches governing the compositor can be made to change from frame to frame during a render — not in the sources.**

The three governing things I found, and what the sources say about each:

The Compositing checkbox:

> "Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Composite Output node."

Blender manual, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30.

> "use_compositing
>
> Process the render result through the compositing pipeline, if compositing nodes are enabled
>
> Type:
>
> boolean, default True"

Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

Use Nodes:

> "Access the Compositor and activate nodes for compositing by clicking the Use Nodes checkbox in the header (see Introduction). … After clicking Use Nodes the Compositor is enabled, however, it can also be disabled in the Post Processing."

Blender manual, https://docs.blender.org/manual/en/4.5/compositing/introduction.html — read 2026-07-30.

> "use_nodes
>
> Enable the compositing node tree
>
> Type:
>
> boolean, default False"

Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

And a third condition, which is not a checkbox but does decide whether the compositor runs:

> "The compositor searches for the active Composite node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether."

Blender manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30.

All of that is **documented**. None of it says whether any of the three can be made to differ between frame 5 and frame 6 of one running render. That part is **not in the sources**.

**C. There is a documented way to change data between frames while a render is running — but not a documented statement that these particular settings respond to it.**

The Python API documents a hook that fires per frame during a render and is explicitly for changing data:

> "bpy.app.handlers.frame_change_pre
>
> Called after frame change for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame. Note that this handler is not to be used as 'before the frame changes' event. The dependency graph is not available in this handler, as data and relations may have been altered and the dependency graph has not yet been updated for that. Accepts two arguments: The scene datablock and the dependency graph being updated"

> "bpy.app.handlers.frame_change_post
>
> Called after frame change for playback and rendering, after the data has been evaluated for the new frame. Accepts two arguments: The scene datablock and the dependency graph being updated"

Blender Python API, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30.

Label: **documented** that a per-frame hook exists during rendering and that data may be changed in it. That something changed there — specifically `mute`, `use_compositing` or `use_nodes` — is then honoured for that frame's File Output write is a step the sources do not take, so I do not take it either. That would be **inferred**, and I am flagging it as unverified rather than asserting it.

The same page carries a warning that bears directly on doing this during a render:

> "Altering data from handlers should be done carefully. While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render."

Same page and date. Label: **documented**.

**D. One near-miss that people will reach for, and why it does not answer this.**

The manual says one compositor node's state can be keyframed, and says why that is useful:

> "Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence."

Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch.html — read 2026-07-30. Label: **documented**.

The quoted words establish that the manual says one compositor node's state may be animated with a keyframe, and calls that useful for bypassing nodes during part of a sequence. That it is therefore read afresh on every frame of a render is a step the words do not take — label for that step: **inferred**, and unverified. The words say nothing about `mute`, nothing about the File Output node, and nothing about the Compositing or Use Nodes switches.

**E. A picture that looks like an answer and is not.**

The manual's Decorators page gives the interface's own way of telling whether a setting can be animated:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated."

Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/buttons/decorators.html — read 2026-07-30. I opened the picture on that page, `_images/interface_controls_buttons_decorators_ui.png`, which shows the three states beside three fields: a small dot, a filled diamond, and a driver arrow. Label: **documented**.

I then opened the picture on the Post Processing page, `_images/render_output_properties_post-processing_panel.png` (https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30). What is visible in it: a Compositing checkbox and a Sequencer checkbox side by side with no decorator beside either, and a Dither slider with a dot decorator to its right. Label for what is visible: **documented**.

It is tempting to conclude from this that the Compositing switch cannot be animated. It does not support that. Cycles' own files hold a panel of this name, and its code sets `use_property_decorate` to `False`:

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

File `Blender Sources/cycles_4.5/ui.py`, class `CYCLES_PT_post_processing`, lines 1102–1118 — read 2026-07-30. *(Line range corrected from 1101–1118 by the citation check, 2026-08-06: the block begins at 1102; line 1101 is blank. The quoted code itself was exact.)* Label: **observed** for the words in the file. Going from those words to what the panel actually draws on screen is a step beyond reading the file, so the conclusion — that whether a decorator appears beside these two checkboxes is a choice made in this panel rather than a statement about the setting — is labelled **inferred**. Also worth noting: the panel in the manual's picture does not match this code — the picture shows the two checkboxes side by side with no "Pipeline" heading, while Cycles' code stacks them under one. Why they differ is not established here. Either way, the picture settles nothing about whether the setting can be animated.

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

- Whether `Node.mute` can be keyframed or driven at all. The sources do not say.
- Whether `RenderSettings.use_compositing` or `Scene.use_nodes` can be keyframed or driven at all. The sources do not say.
- Whether either kind of switch, if changed, is re-read once per frame during a running animation render, or read once when the render starts and then held. This is the heart of the question and the sources are silent on it for all three switches.
- Whether a muted File Output node writes nothing, or writes something. The manual's Mute wording is about a node's contribution and about links passing through, and a File Output node has no output sockets ("This node has no output sockets." — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30), so the wording does not reach it. That is question 8's ground and I have not settled it here; I note it only because question 14 assumes an answer to it.
- Whether the per-slot entries of a File Output node have any off switch of their own. I did not find one and did not go looking beyond the node's own API page, since the question asks about the node.
- Whether changing these things from a frame-change handler is safe or produces a usable result. The documented crash warning above is about the viewport, not about output correctness.

### Outside the interface

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

The direct question is unanswered by the sources whether Blender is driven by a person or not, so nothing here differs in what is known. Three documented things do bear on the case where Blender is driven from outside, and they cut in different directions.

First, the per-frame hook is not an interface feature. `frame_change_pre` and `frame_change_post` are documented as being "called after frame change for playback and rendering" (https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30), and scripts can be supplied at launch:

> "-P, --python <filepath>
>
> Run the given Python script file."

Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. Label: **documented**.

Second, the argument list marks one switch as the default:

> "-Y, --disable-autoexec
>
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Same page and date. Label: **documented** for the quoted words. Going from them to *an approach relying on a Python-expression driver would need the opposite switch passed explicitly* is a step — label for that step: **inferred**. The opposite switch is not quoted anywhere here, so its exact name is **not in the sources** as this document stands. Keyframes are not named in the quoted words, so I do not extend it to them.

Third, there is a route that sidesteps the question entirely, but it is a different thing from one render. Instead of `--render-anim`, a script can set the frame and render frames one at a time:

> "frame_set(frame, *, subframe=0.0)
>
> Set scene frame updating all objects and view layers immediately"

https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30, together with

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')"

https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30. Both **documented**. Between two such calls the script is plainly free to set `mute`, `use_compositing` or `use_nodes` to whatever it likes, because nothing is running in between. What the sources do not establish is that this is equivalent to changing them inside a single `--render-anim` pass; I am not claiming it is.

I searched the full command-line argument list for any switch that turns compositing on or off, per frame or otherwise. There is none.

### Operating system

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

Whether this behaviour differs by operating system is **not in the sources**; the record of where I looked is below. The requirements page names Blender 4.5 for Windows, macOS and Linux (https://www.blender.org/download/requirements/ — read 2026-08-01). Label: **documented**. *(Quotes supplied by the citation check, 2026-08-06, which checked this line against the page and found it right. Under the Windows heading and again under the Linux heading: "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer." Under the macOS heading: "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2 (AMD, Apple Silicon, or Intel Skylake and newer)".)*

The one operating-system-specific compositor note I found does not bear on this question, but I record it because I looked at it:

> "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Blender manual, https://docs.blender.org/manual/en/4.5/compositing/limitations.html — read 2026-07-30, under the heading "TDR on Windows". Label: **documented**. It concerns how long a GPU compositing task may run on Windows, not whether a switch can change between frames.

### Where I looked

Searches run over the whole of `Blender Sources/manual_4.5` and `Blender Sources/api_4.5` unless noted, plus `Blender Sources/release_notes_4.5`, `Blender Sources/cycles_4.5` and `Blender Sources/requirements`:

- `file output`, `file_output` — four manual pages and the two API pages; read the File Output node page in full.
- `mute`, `muting`, `Toggle Node Mute` — read the node editing page and the node sidebar page in full. The sidebar's Node panel lists Name, Label, Warning Propagation and Color, and no Mute field, so there is no property row in the sidebar that would carry a decorator for it.
- `animatable`, `be animated`, `not animatable`, `cannot be animated` — 35 hits across the manual, every one checked. Only three sit anywhere near this question: the Decorators page, the Switch node tip, and the Hue/Saturation node's "can be animated using a Time Curve node or by setting keyframes". None concerns Mute or the compositing switches.
- `keyframe`, `driver` limited to `compositing/`, `interface/controls/nodes/` and `render/output/` — only the Switch node tip and an unrelated video "Keyframe Interval" setting.
- `animat` limited to `compositing/`, `interface/controls/nodes/` and `editors/node_editor/` — 20 hits, all checked, all about node values or examples rather than about Mute or the compositing switches.
- `Post Processing`, `Use Nodes`, `use_compositing`, `use_nodes` — read every hit. Nothing about animation.
- `per frame`, `every frame`, `each frame`, `for each frame` limited to `render/`, `compositing/` and `animation/` — 18 hits, all checked. Nothing about when the compositing switches are read.
- `cannot be animated`, `not animated`, `animated per frame`, `evaluated per frame`, `per-frame` across the API documentation — two hits, both unrelated (an Alembic import option, and a clip editor curve display).
- `is_property_animatable` across the API documentation — no hits. That is one term only. The plainer words searched across the manual were not searched across the API documentation, so this does not establish that the API pages carry no animatability marker.
- Read in full: `bpy.app.handlers`, and the API pages for `Node`, `CompositorNodeOutputFile`, and the relevant parts of `Scene`, `RenderSettings`, `bpy_struct` (`keyframe_insert`, `driver_add`) and `bpy.ops.render`.
- Read in full: the manual's Rendering Animations page, Rendering From The Command Line page, Command Line Arguments render and Python sections, the Compositor System page, the compositing Introduction, Sidebar and Limitations pages, the compositing output node index, the Drivers introduction and usage pages, and the Keyframes introduction and editing pages.
- Release notes 4.5: searched `mute`, `file output`, `animat`, `compositing` across `compositor.md`, `python_api.md`, `core.md`, `pipeline_assets_io.md` and `rendering.md`, and read `compositor.md` and `core.md` in full. The compositor notes for 4.5 list new nodes, options exposed as inputs, and removals; nothing about muting, about animating node state, or about when the compositor runs. `core.md` mentions the File Output node only in connection with path templates. Those five files were picked as the likely ones; the rest of the release-notes set for 4.5 was not searched.
- Cycles files: searched for `use_compositing`, `node.mute` and `frame_change`. Two hits. One is the Post Processing panel in `ui.py`, quoted above. The other is a `use_compositing` argument in `source/kernel/device/gpu/kernel.h` around line 1116, which I read and ruled out as sitting nowhere near the compositor node tree. The words there are not quoted, so that rule-out rests on my reading of the file rather than on anything reproduced here.
- Requirements page: checked which releases it names; it names Blender 4.5 LTS for Windows, macOS and Linux. It says nothing about this question.

Pictures opened: `_images/interface_controls_buttons_decorators_ui.png`, `_images/render_output_properties_post-processing_panel.png`, `_images/render_output_properties_output_panel.png`. The third was opened to see whether checkboxes in the Output properties are drawn with decorators at all; that panel shows no decorators anywhere, including next to number fields, which is a further reason not to read anything into their absence beside a checkbox.

Ruled out as places to look, and why: the Video Sequencer pages (a different pipeline stage — the Post Processing panel's Sequencer switch, not the compositor), the shader and geometry node pages (Mute is documented once, in the shared node editing page, which I read), and the Grease Pencil, physics and constraint pages that matched `mute`/`animatable` for unrelated reasons.

I did not need the web and did not use it.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 4.5

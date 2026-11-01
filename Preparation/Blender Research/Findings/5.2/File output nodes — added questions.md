# File output nodes — added questions (Blender 5.2)

Sources consulted, and when: Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2
release notes, and Cycles' own files, all read on 2026-07-30 from the snapshot in `Blender Sources/`.
Blender's requirements page (snapshot dated 2026-08-01) was opened once and searched; it does not
name Blender 5.2 anywhere, so it speaks for some other release and I have not leant on it.
The manual pictures listed below were also read on 2026-07-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 14 was asked for, and only question 14 is answered.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**LABEL: not in the sources.**

The sources describe both switches plainly, and they describe a general machinery for making
properties change over time. They never join the two. Nothing I found says whether the node's mute
switch, or the scene's Compositing pipeline switch, can be keyed, driven, or otherwise made to hold a
different value on frame 12 than on frame 11 within one render — and nothing says what the compositor
would do with such a change if it were made.

**A note on the reading.** "The node's own silencing" I have read as the node's **Mute** switch, the
one thing in the sources that silences an individual node. There is a second thing the sources call
muting, which is muting an individual **link** (`Node ‣ Mute Links`); a File Output node could also be
silenced by muting the link feeding it. I have covered both below, since the sources treat them as
separate features. "Whatever governs the compositor it sits in" I have read as the
`Properties ‣ Output ‣ Post Processing ‣ Pipeline ‣ Compositing` switch, which the manual names as the
thing that makes the render engine use the compositing tree at all. The other candidate reading — the
node group assigned to the scene, which could in principle be swapped — is noted at the end of this
answer but is not what I have answered.

### What the sources do say

**The node's own silencing is an ordinary on-off property, and the File Output node has it.**

> mute
>
> (default False)
>
> Type:
>
> bool

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Node.html`, read 2026-07-30.
That is the entire entry: no description, and no statement about animation either way.

The File Output node's page lists `Node.mute` among its inherited properties:

> Node.mute

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html`,
under the inherited-properties list, read 2026-07-30. **LABEL for this sub-point: documented.**

What muting does:

> Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change.

— manual, `https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html`, read 2026-07-30.

> Mute
>
> Temporarily disables the node without removing it.
> When possible, the node is bypassed and its inputs are passed directly to its outputs.

— manual, `https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html`, in the
`Sidebar ‣ Node` section, read 2026-07-30.

Neither passage says what muting does to a node that has no outputs and whose only job is to write
files. The File Output node's own page never mentions muting at all
(`https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html`, read in full,
2026-07-30; its "Outputs" section reads "This node has no output sockets.").

There is an operator for it, again with no timing or animation statement:

> bpy.ops.node.mute_toggle()
>
> Toggle muting of selected nodes

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.node.html`, read 2026-07-30.

**Link muting is a separate, documented feature.**

> A muted link acts as though it’s no longer there; this also means the input fields
> for specifying fixed values become visible again.

— manual, `https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html`, read 2026-07-30.

> Muted links are kept in place but ignored in evaluation.

— manual, `https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html`, read 2026-07-30.

Again nothing about changing that state over time.

**What governs the compositor is also an ordinary on-off property.**

> use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)
>
> Type:
>
> bool

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`, read 2026-07-30.

> Important
>
> To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline
> option must be enabled in the output’s Post Processing properties.

— manual, `https://docs.blender.org/manual/en/5.2/compositing/introduction.html`, read 2026-07-30.

> Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node.

— manual, `https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html`,
read 2026-07-30.

The older scene-level switch is documented as gone in substance:

> use_nodes
>
> Enable the compositing node group. (default False)
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing.

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Scene.html`, read 2026-07-30.
**LABEL for this sub-point: documented.** It matters here only because it rules out one candidate
switch: setting it does nothing at all, on any frame.

**There is a general machinery for making a property change over time, and nothing that names either
of these two properties as eligible.**

> keyframe_insert(data_path, *, index=-1, frame=None, group='', options=set(), keytype='KEYFRAME')
>
> Insert a keyframe on the property given, adding fcurves and animation data when necessary.

> driver_add(path, index=-1, /)
>
> Adds driver(s) to the given property

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.bpy_struct.html`, read 2026-07-30.
Both are offered on every property-bearing thing in Blender, so their existence says nothing about
these two properties in particular.

The API is explicit that eligibility is a per-property matter and that the default is "no":

> is_animatable
>
> Property is animatable through RNA (default False, readonly)
>
> Type:
>
> bool

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Property.html`, read 2026-07-30.
The documentation never states the value of that flag for `Node.mute` or for
`RenderSettings.use_compositing`. This is exactly the gap. It is also the reason a hand check is
cheap: the flag can be read out (see the list at the end).

A compositing node tree is a data-block that can carry animation:

> animation_data
>
> Animation data for this data-block (readonly)

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.NodeTree.html`, read 2026-07-30;
the same entry is listed as inherited on
`https://docs.blender.org/api/5.2/bpy.types.CompositorNodeTree.html`. That establishes that *some*
animation can live on a compositing tree. It does not establish that muting is part of it.

The manual states in passing that compositor node values can be keyed:

> The different values can be animated using a Time Curve node or by setting keyframes.

— manual, `https://docs.blender.org/manual/en/5.2/compositing/types/color/adjust/hue_saturation.html`,
read 2026-07-30. That is about a node's input values, not about its mute switch.

The 5.2 release notes add that keying now reaches compositor gizmos:

> Node gizmos now support auto keying.

— release notes, `https://developer.blender.org/docs/release_notes/5.2/compositor/`, read 2026-07-30.
That line says nothing about the mute switch. What a gizmo can be attached to is not stated on that
page, and I am not filling it in, so nothing here reaches the question either way.

### The picture, and why it settles nothing

The manual's Post Processing page carries `[image: _images/render_output_properties_post-processing_panel.png]`.
I opened it (manual, `https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html`,
read 2026-07-30). It shows a "Pipeline" row with **Compositing** and **Sequencer** checkboxes, both
ticked, and below them a **Dither** slider reading 1.000. There is a small round dot at the right-hand
edge of the Dither row. There is no such dot on the Compositing row. **LABEL for what is visible:
documented.**

The manual explains what those dots are:

> Decorators are small buttons that appear to the right of other buttons and show the state of the property.
> Decorators may appear next to number fields, menus,
> and checkboxes to indicate the property can be animated.

— manual, `https://docs.blender.org/manual/en/5.2/interface/controls/buttons/decorators.html`, read 2026-07-30.

It is tempting to conclude that Compositing therefore cannot be animated. **It does not follow, and I
am not concluding it.** The decorators page says a dot **may** appear to show that a property can be
animated. It does not say that every property which can be animated gets one, and it does not say what
a missing dot means. Reading an absence as a "no" is a step the sources do not supply.

One Cycles file bears on the same panel. It is set down here because it was read, not because it
settles anything:

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

        layout.prop(rd, "dither_intensity", text="Dither", slider=True)
```

— Cycles' own files, `Blender Sources/cycles_5.2/ui.py`, class `CYCLES_PT_post_processing`, at lines
1169–1185, read 2026-07-30. **LABEL: observed** (this is what the file holds).

What the file holds is a line setting that panel's decorate flag to off. What happens when the panel is
actually drawn is a step past reading the file, and I am not taking it. It also cannot explain the
picture in any case: the picture's Dither row *does* carry a dot, so whatever panel the picture shows
is one that draws dots, and it is therefore not the panel this file describes. The sources I was given
hold no other copy of this panel, so I cannot compare further. The picture and the file together leave
the question open rather than answering it.

I also opened `[image: _images/interface_controls_nodes_sidebar_item.png]` from
`https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html` (read 2026-07-30),
hoping to see the Mute checkbox and whether it carries a decorator. It shows the Sidebar's Node tab
with a Render Layers node selected: Name, Label and a collapsed Color row, then a Properties panel with
a Scene field and a View Layer field. **No Mute checkbox is visible in that picture**, so it settles
nothing about muting either.

### The one route the sources do document for per-frame change

There is a documented way for a script to alter data between frames of a render:

> bpy.app.handlers.frame_change_pre
>
> Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame.

— Python API documentation, `https://docs.blender.org/api/5.2/bpy.app.handlers.html`, read 2026-07-30.

Also documented, per-frame during a render:

> bpy.app.handlers.render_pre
>
> on render (before). Accepts one argument: the scene data-block being rendered

> bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered

— same page, same date.

And a caution attached to changing data from those handlers:

> Altering data from handlers should be done carefully. While rendering the
> frame_change_pre and frame_change_post handlers are called from one
> thread and the viewport updates from a different thread. If the handler changes
> data that is accessed by the viewport, this can cause a crash of Blender. In
> such cases, lock the interface (Render → Lock Interface or
> bpy.types.RenderSettings.use_lock_interface) before starting a render.

— same page, same date.

This says a script can change data for the new frame during a render. It does **not** say that changing
`mute` or `use_compositing` from such a handler is honoured by the compositor for that frame, and it
does not say when during the frame either value is read. Going from "data can be changed" to "this
particular switch will take effect on this particular frame's file writing" needs a step the sources do
not supply, so I am not making it. That would be an inference, and a weak one.

### Where I looked

Everything below was searched across the whole of `manual_5.2` and `api_5.2` unless stated.

- Terms searched, case-insensitive, across the whole manual: `mute`, `animatable`, `can be animated`,
  `cannot be animated`, `can not be animated`, `silenc`, `bypass`, `lock interface`, `during a render`,
  `while rendering`, `during rendering`, `render is running`, `post processing`, `compositing node group`.
- Terms searched across the whole API documentation: `mute`, `use_compositing`, `use_nodes`,
  `compositing_node_group`, `keyframe_insert`, `driver_add`, `is_animatable`, `frame_change_pre`,
  `render_pre`, `render_write`, `composite_pre`, `composite_post`, `silenc`.
- Terms searched across the whole 5.2 release notes: `mute`, `use_compositing`, `use_nodes`,
  `compositing`. Only three hits for `mute` in the whole of 5.2's notes: a motion-tracking shortcut
  change, and two bug-fix lines about link mute state being lost when adding a reroute and when
  grouping/ungrouping (`https://developer.blender.org/docs/release_notes/5.2/bugfixes/`, read
  2026-07-30). Nothing about animating mute.
- Terms searched across Cycles' own files: `use_compositing`, `mute`. The only user-facing hit is the
  `CYCLES_PT_post_processing` panel quoted above; the other hits are unrelated maths in the kernel
  (`_mm256_permute*`) and the same word appearing in
  `Blender Sources/cycles_5.2/source/kernel/device/gpu/kernel.h`, which I did not quote and have read
  nothing into — so nothing at all is claimed here about what that one is.
- Pages opened in full: the File Output node page; the node editing page (`Mute`, `Mute Links`); the
  node editors page (Sidebar `Node` panel); the Post Processing page; the compositing introduction;
  the compositing limitations page; the compositing workspace page; the compositor sidebar page; the
  keyframes introduction and keyframes editing pages; the drivers introduction; the actions page; the
  decorators page; the scripting security page; the command-line arguments page; the "Rendering
  Animations" page; `bpy.app.handlers`; `bpy.types.Node`; `bpy.types.CompositorNodeOutputFile`;
  `bpy.types.RenderSettings`; `bpy.types.Scene`; `bpy.types.NodeTree`; `bpy.types.CompositorNodeTree`;
  `bpy.types.Property`; `bpy.types.bpy_struct`; `bpy.ops.node`.
- Pictures opened: `render_output_properties_post-processing_panel.png` and
  `interface_controls_nodes_sidebar_item.png` (both described above).
- Ruled out and why: the animation section of the manual (keyframes, drivers, actions, animation
  editors) describes the machinery in general and never lists which properties are eligible; the
  compositing section never uses the word "mute" once; the File Output node page never mentions muting,
  animation or per-frame switching; the release notes for 5.2 record no change in this area. The
  requirements page carries no mention of compositing and does not name 5.2, so it is silent here too.

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

*Several of the items below were settled afterwards by running Blender; the observed section above is where those answers are. What stands here is what the sources did not carry.*

**NOT COVERED.** This answer does not settle: whether `Node.mute` or `RenderSettings.use_compositing`
is flagged animatable in Blender's property system; whether a keyframe or driver on either is honoured
during a render; at what moment during a frame either value is read; what a muted File Output node
does — whether it skips writing entirely, writes nothing, or writes something — since the manual only
describes muting in terms of passing inputs through to outputs and this node has no outputs; whether a
muted *link* feeding the node has the same effect as muting the node; whether a *link's* mute state can
itself be changed from one frame to the next, which is the same question asked of the second thing this
answer treats as "the node's own silencing" and which is not settled here; and whether the compositing node
group assigned to the scene (`Scene.compositing_node_group`, "Compositor Nodes",
`https://docs.blender.org/api/5.2/bpy.types.Scene.html`, read 2026-07-30) could be swapped between
frames, which is a third possible reading of "whatever governs the compositor" that I have not pursued.

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

**Outside the interface.**

**Settled by running Blender — observed, 2026-08-06 on the rented Linux machine and 2026-08-07 on the Creator's Mac, both described above.** Everything in the two observed sections, apart from the two entries that needed a screen, was done with Blender driven from outside its own window: started with `-b` and no window, driven by a script, or handed a saved file and `-a` on a command line. So the answers there hold when Blender is driven from outside. **They hold with the interface open too.** On the Creator's Mac the two saved files were rendered a second time with a window on screen instead of `-b`, and the File Output node wrote the same frames both ways — entry 11 of `Audits/Blender Research/File output nodes/Hand tests — added questions.md`, with the output quoted above.

The sources themselves say nothing either way, and what follows is the record of that.

The sources do not say whether either switch can be changed per frame at
all, so they cannot say whether it differs when Blender is driven from outside its own window. Three
things that are documented and bear on it:

- `frame_change_pre` is documented as being called "for playback and rendering" — the same handler in
  both cases, with no interface-only qualification
  (`https://docs.blender.org/api/5.2/bpy.app.handlers.html`, read 2026-07-30).
- If the per-frame change were made with a driver using a Python expression, script auto-execution has
  to be turned on explicitly when running headless. The manual gives the exact command:

  > blender --background --enable-autoexec my_movie.blend --render-anim

  — manual, `https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html`, read 2026-07-30,
  introduced with "To render an animation in background mode, allowing drivers and other scripts to run:".
  The command-line reference marks the *off* state as the default:

  > -Y, --disable-autoexec
  >
  > Disable automatic Python script execution (Python-drivers & startup scripts), (default).

  — manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`, read 2026-07-30.
  That the off state is the default, and that the switch covers Python drivers and startup scripts, is
  what the words say. **LABEL for that: documented.** They name nothing else. Whether a plain keyframe
  is touched by this switch is not stated either way, and I am not reading the silence as a "no".
  **LABEL for that: not in the sources.**
- The "Lock Interface" advice attached to altering data from handlers is about the viewport thread, and
  the sources do not say what it means when there is no viewport.

**Operating system.**

**Settled for two operating systems by running Blender — observed, 2026-08-06 and 2026-08-07.** Every test was run twice: on Linux (Ubuntu 22.04.5 LTS on the rented machine described above) and on macOS 26.5.2 (the Creator's MacBook Pro with an M1 Pro chip, also described above). **The two machines agreed on every one of them**, so the answers above hold on Linux and on macOS alike. **Windows is untried**: no Windows machine has run it, so no answer here covers it.

The sources themselves say nothing either way, and what follows is the record of that.

The sources say nothing that ties any of this to an operating system, and I found
no OS-specific statement about muting, about the Compositing pipeline switch, or about animating
either. The one OS-specific note anywhere near this subject is unrelated to it:

> TDR on Windows
>
> Timeout Detection Recovery
> (TDR)
> resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit.

— manual, `https://docs.blender.org/manual/en/5.2/compositing/limitations.html`, read 2026-07-30.
That concerns the GPU compositor's running time, not what decides whether a File Output node writes.
The requirements page was searched and contains no mention of compositing, and does not name 5.2, so it
speaks for a different release and I have taken nothing from it.

---

## Things only a running Blender can settle

Every item that stood here has been run — on the rented Linux machine, and again on the Creator's Mac — and the answers are in the two observed sections higher up this document. The tests, their set-ups and their raw output are in `Audits/Blender Research/File output nodes/Hand tests — added questions.md`.

The two items that needed a screen have both been answered, on the Creator's Mac: whether Blender's own interface offers a keyframe on the node's Mute or on the Compositing switch, and whether an animation render started with the interface open agrees with one started with no window. Nothing on this question is left for a running Blender.

One thing is untested: **Windows**. No Windows machine has run it, so every answer above covers Linux and macOS and leaves Windows open rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — File output nodes, Blender 5.2

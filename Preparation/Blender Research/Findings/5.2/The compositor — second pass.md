# The compositor — Blender 5.2 — second pass

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including three of its pictures), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All four were taken on 2026-07-30, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note on the pictures: three manual screenshots are cited. They are small, so I decoded them and enlarged them with a short throwaway script in my scratchpad (pure Python, no libraries) rather than reading them at original size. Nothing was altered but the scale.

A note on how I searched: for every question I searched the whole of `manual_5.2` and `api_5.2` with `grep -rn` before opening anything, and I list the terms under "Where I looked" in each answer. Where a term found nothing, I say so.

---

## 1. What decides whether a file's compositor runs at all?

**Answer.** Four things. *(Citation check, 2026-08-03: this line previously read "Four things, and the first three must all be true," under the documented label below. Each condition is documented on its own page; no source states them as one joint condition. The conjunction is **inferred**, not documented. The "Not covered" paragraph below already records that the list's completeness is not settled.)*

**(a) The scene's Compositing pipeline switch must be on.** It lives in Properties ‣ Output ‣ Post Processing.

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html, read 2026-07-30.

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/introduction.html, read 2026-07-30.

The property behind it:

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30.

**(b) A compositing node group must be assigned to the scene.** That is the "if a compositing node group is assigned to the scene" clause above. The assignment is a single per-scene pointer:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree | None"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Scene.html, read 2026-07-30.

**(c) The node group must contain an active output node.** If it does not, nothing runs:

> "The compositor searches for the active Group Output node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether. Consequently, note that adding a Viewer node will have no effect on the viewport render if there is a Group Output node, since the priority is given to Group Output nodes."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html, read 2026-07-30.

And on which Group Output counts, when there is more than one:

> "If multiple Group Output nodes are present in a node tree, only the active one is used. All others will show a "Unused Output" warning. The active output can be changed in the header of the node."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/group_output.html, read 2026-07-30.

**(d) The old "Use Nodes" switch no longer decides anything.**

> "use_nodes
>
> Enable the compositing node group. (default False)
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

— Python API, https://docs.blender.org/api/5.2/bpy.types.Scene.html, read 2026-07-30.

**Observed, from Cycles' own files.** The Post Processing panel Cycles draws puts these two properties under a heading "Pipeline":

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
```
…
```
        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

— `Blender Sources/cycles_5.2/ui.py`, lines 1169–1170 and 1181–1183, read 2026-07-30. *(Citation check, 2026-08-03: the sentence above previously read "contains exactly these two pipeline switches and nothing else that gates compositing." The quoted lines show two properties drawn under a heading; they do not show that nothing else gates compositing, and the excerpt skips the panel's third control, the Dither slider, which the fuller quotation elsewhere in this topic includes. Narrowed to what the file shows.)*

**Documented, from a picture.** The manual's Post Processing screenshot (`_images/render_output_properties_post-processing_panel.png`, shown on the Post Processing page above) shows a panel titled "Post Processing" containing two ticked checkboxes labelled "Compositing" and "Sequencer", and a "Dither" slider reading 1.000. Read 2026-07-30.

**Label:** documented (all four points).

**Not covered.** The sources do not say what happens when the switch is on and a group is assigned but the group is empty or its output is unconnected — only that with no active output node at all nothing runs. They do not say whether compositing is skipped for a cancelled render, nor whether a still render (F12) and an animation render differ in this. Point (c) is stated on a page about the compositor generally and its closing sentence talks about "the viewport render"; I read the "doesn't run altogether" sentence as applying to the compositor as a whole, but a narrower reading limited to viewport compositing is possible.

The 3D viewport's own preview of compositing is a separate switch and is not part of the render pipeline gate:

> "Compositor
>
> When to preview the result of compositing in the 3D Viewport.
>
> Disabled: Never show the compositing output.
> Camera: Only show the compositing output when in Camera View, which gives the best vantage point for previewing the final result.
> Always: Always show the compositing output."

— Manual, https://docs.blender.org/manual/en/5.2/editors/3dview/display/shading.html, read 2026-07-30.

**Where I looked.** Searched the whole manual for `post processing`, `Compositing Pipeline`, `use_compositing`, `compositing_node_group`, `Use Nodes`, `ompositor` (to catch both cases), `background`. Searched the whole API docs for `use_compositing`, `use_sequencer`, `compositing_node_group`, `node_tree` within `bpy.types.Scene.txt`, `save_output`, `use_output|save_output|use_file_output|use_save|write_output`. Read in full: `compositing/introduction.txt`, `compositing/compositor_system.txt`, `compositing/index.txt`, `compositing/limitations.txt`, `compositing/usage.txt`, `compositing/sidebar.txt`, `compositing/compositing_workspace.txt`, `render/output/properties/post_processing.txt`, `compositing/types/output/*` (all five). Ruled out `editors/compositor.txt` as a gate — it describes the editor, not the pipeline. Searched `cycles_5.2` for `compositor|use_compositing|use_sequencer|save_output|resolution_percentage|use_border|use_crop`.

**Outside the interface.** All four points are scene data saved in the blend-file, not editor state, so they apply to a render driven from outside Blender's window as well. The manual states that a command-line animation render uses the file's settings — "-a / Render the whole animation using all the settings saved in the blend-file" (Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30) — and the API describes compositing as a background job: "bpy.app.handlers.composite_pre / on a compositing background job (before). Accepts one argument: the scene data-block" (Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30). The release notes also record a fix whose title places the GPU compositor in background mode: "Cycles crashes while trying to render second scene in GPU compositor (-b mode)" (Release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30). Taking those together to conclude "the same gates apply headlessly" is a step beyond the words, so: **inferred**. Nothing in the sources states a difference.

**Operating system.** Nothing in the sources ties these switches to an operating system. The one OS-specific statement anywhere near the compositor is about the GPU compositor being interrupted, not about whether it runs: "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." (Manual, https://docs.blender.org/manual/en/5.2/compositing/limitations.html, read 2026-07-30) — that page's heading is "TDR on Windows". Beyond that, the sources do not say.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Answer.** Yes. The Output panel — the panel that holds the file path frames are written to — has a checkbox in its own header that turns off automatic saving, and when it is off Blender writes nothing from that panel.

> "This panel defines where rendered images and animations are saved, along with their output format and quality settings.
>
> By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node.
>
> The automatic saving of renders can be toggled using the checkbox in the panel header.
>
> When enabled, Blender behaves in the standard way: still renders (F12) are not saved automatically, while animation renders (Ctrl-F12) are written to the configured output path.
>
> When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported.
>
> Playblast operations also require an active output method. Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30.

The same thing said again on the animation page, where the switch is given a name, "Save Output":

> "This behavior depends on the Output option in the Output properties. When enabled (default), animation renders are written automatically. When disabled, Blender will not save render output from the Output panel and instead relies on alternative output methods such as Compositor File Output nodes. If no valid output method is available, rendering operations will report errors."

and

> "Blender creates a file for each frame of the animation when Save Output is enabled. If it is disabled, frames will only be written if a Compositor File Output node is configured."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html, read 2026-07-30.

The property:

> "save_output
>
> Write frames to disk for animation renders (default True)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30.

The release notes record it as new in this version:

> "## Render Output
>
> * Saving render output can now be disabled. (blender/blender@342f37566f)"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/rendering/, read 2026-07-30.

The API change log lists it under things added between 5.1 and 5.2: "bpy.types.RenderSettings.save_output" (Python API, https://docs.blender.org/api/5.2/change_log.html, read 2026-07-30).

**Label:** documented.

**Not covered.** Ambiguity, flagged rather than resolved: I read "the place where a scene's rendered frames are written" as the Output panel's automatic saving — the destination as a whole. A different reading is "can the File Path field itself be emptied or disabled", and on that the sources say nothing; they only describe what the field means and that it accepts `//` and `#` and template expressions.

The sources also do not say what happens on the command line when `-o` supplies a path but Save Output is off; do not say whether Save Output affects a still render at all (the property text says "for animation renders", while the panel text says still renders were never saved automatically in the first place); and do not say whether the error raised is fatal to the whole job or per frame.

**Documented, from a picture — with a caveat.** I enlarged the manual's Output panel screenshot (`_images/render_output_properties_output_panel.png`, shown on the Output page above) and looked at its header. What is visible is a disclosure triangle and the word "Output" — no checkbox is visible in the header in that screenshot. The panel body shows a path field reading `/tmp/`, and checkboxes Overwrite (ticked), Placeholders (unticked), File Extensions (ticked), Cache Result (unticked), then File Format PNG, Color BW/RGB/RGBA with RGBA selected, Color Depth 8/16 with 8 selected, and Compression 15%. So the picture does not corroborate the header checkbox the text describes; it neither confirms nor denies it, since the screenshot may predate the feature. Read 2026-07-30.

**Where I looked.** Searched the whole manual for `Save Output`, `automatic saving`, `render output`, `File Output`, `Playblast`. Searched the whole API docs for `save_output`, `use_output|save_output|use_file_output|use_save|write_output`, `render_output`, and listed every `use_*` property on `bpy.types.RenderSettings`. Read in full: `render/output/properties/output.txt`, `render/output/properties/index.txt`, `render/output/index.txt`, `render/output/introduction.txt`, `render/output/animation.txt`, `render/output/properties/metadata.txt`, `compositing/types/output/file_output.txt`, `editors/3dview/viewport_render.txt`. Searched all four release-notes subjects for `output` and `save output`.

**Outside the interface.** Save Output is a scene property in the file, so a headless render reads the same value. The manual does not say this in so many words for this switch; the closest is "-a / Render the whole animation using all the settings saved in the blend-file" (Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). Applying that to Save Output specifically is a step beyond the words: **inferred**. What is genuinely not in the sources is what happens when `-o`/`--render-output` is given on the command line while Save Output is off — the arguments page describes `-o` as "Set the render path and file name" and says nothing about the switch (Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30).

**Operating system.** The sources do not say. Nothing on any of the pages above names an operating system.

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Answer.** Both halves of the answer are true of different things, and they need separating. The *assignment* belongs to one scene. The *node group itself* is a file-level data-block that more than one scene can point at.

The assignment is a single per-scene pointer, and it may be empty:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree | None"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Scene.html, read 2026-07-30.

Said plainly in the manual:

> "Node Tree
>
> Chooses which node group (compositor node tree) to display and edit. Each Scene can have its own compositor node tree. You can also create and switch to custom node groups for reuse."

— Manual, https://docs.blender.org/manual/en/5.2/editors/compositor.html, read 2026-07-30.

And the thing being pointed at is a data-block that can be shared or brought in from elsewhere:

> "To begin compositing, open the Compositor and create a new node tree by clicking the New button in the header. This adds a compositing node tree data block to the scene, which can then be reused across different scenes or linked/appended from other blend-files."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/introduction.html, read 2026-07-30.

Node groups are stored at file level, alongside scenes, not inside them:

> "node_groups
>
> Node group data-blocks (default None, readonly)
>
> Type:
>
> BlendDataNodeTrees[NodeTree]"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendData.html, read 2026-07-30.

Compositing is also handled as a per-scene job: the three compositing handlers each receive a scene.

> "bpy.app.handlers.composite_pre
>
> on a compositing background job (before). Accepts one argument: the scene data-block"

— Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30. (`composite_post` and `composite_cancel` are worded identically.)

One more thing the same tree can be doing elsewhere in the file, which is worth knowing before treating "the scene's compositor" as the only place a compositing tree is used:

> "is_strip_modifier
>
> The node group is used as a sequencer strip modifier (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeTree.html, read 2026-07-30.

> "In addition to the standard Compositor editor, compositor node groups can be used directly in the Video Sequencer through compositor strips and compositor modifiers."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/usage.html, read 2026-07-30.

**Label:** documented.

**Not covered.** The sources do not say what happens when two scenes point at the same node group and both are rendered — whether each evaluation is independent, or whether anything is shared or cached between them. They do not say whether a scene can hold more than one compositing node group (the property is singular, which suggests not, but that is my reading of a type, not a statement). The Compositor editor also has a "Pinned" option — "When enabled, the editor always displays the selected node tree, regardless of the active scene" (same editors page) — which is editor state, not a second assignment.

**Where I looked.** Searched the whole API docs for `compositing_node_group` (hits in `bpy.types.Scene.txt`, `bpy.types.NodeTree.txt`, `bpy.ops.node.txt`, index files) and for `node_groups` in `bpy.data.txt` and `bpy.types.BlendData.txt`. Searched the manual for `different scenes`, `other scenes`, `another scene`, `Usages`, `ompositor`. Read in full: `editors/compositor.txt`, `compositing/introduction.txt`, `compositing/usage.txt`, `interface/controls/nodes/groups.txt` (Usages section), `bpy.types.CompositorNodeTree.txt`, `bpy.types.Scene.txt` (relevant sections).

**Outside the interface.** Data-block ownership is a property of the file, not of the window, so it holds identically when Blender is driven from outside. The sources do not state this; it follows from `compositing_node_group` being scene data and node groups being `BlendData` — **inferred**.

**Operating system.** The sources do not say, and nothing on these pages names an operating system.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**This question is ambiguous and I answer both readings.** A scene can take in another scene's rendered result in two quite different ways — through a Render Layers node that points at the other scene, and through a Scene strip in the Video Sequencer. The sources say opposite-looking things about the two. I take the Render Layers reading as the one meant, because the question is on a compositor sheet, but the Scene strip reading matters and is answered below it.

### Reading A — a Render Layers node pointing at another scene

**Answer: no. What arrives is the other scene's raw render, not its composite.**

> "Render Layers Node
>
> Renders a View Layer and reads its Passes into the compositing node graph."
>
> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."
>
> "Hint
>
> To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html, read 2026-07-30.

Reinforced by what the Combined pass is:

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

— Manual, https://docs.blender.org/manual/en/5.2/render/layers/passes.html, read 2026-07-30.

**Label:** documented, that what you receive through a Render Layers node is the other scene's raw render output and not its compositing output. The hint says so directly, and prescribes a file round-trip as the way to get the compositing output instead.

**Inferred, and separately:** that the second scene's compositor therefore *does not execute at all*. The step: the sources say the result handed over is pre-compositing, and they say the way to get compositing output is to render to files first and read those back; they never say the second compositor runs and its result is discarded. From "you cannot get it this way" to "it never runs" is my step, not the source's. If it matters whether the work is done and thrown away — for instance, for render time — the sources do not settle it.

### Reading B — a Scene strip in the Video Sequencer

**Answer: the sources indicate the strip can carry compositing output, but do not settle whose Compositing switch decides it.**

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html, read 2026-07-30.

That sentence sits on the *current* scene's Post Processing page but talks about Scene strips, which reference *other* scenes. Which scene's "Compositing" is meant is not stated. I am not going to guess.

Two further documented facts about Scene strips:

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

> "Limitations
>
> Scene strips do not render individual Render Passes; only the Combined render pass will be used."

— Manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html, read 2026-07-30.

And a release note that is an explicit statement of a change, and which points the same way — that a Scene strip is expected to carry the strip scene's compositing result:

> "* Scene Strips doesn't preserves glare from scene compositing (Film->Transparency on) on alpha over"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30. That is the title of a fixed issue in the Video Sequencer section of the 5.2 bug-fix list.

**Label:** documented that Scene strips insert another scene's render output and that only Combined is used; documented that a fix in this area concerns scene compositing reaching a Scene strip. **Not in the sources:** which scene's Compositing switch governs it.

**Also relevant, and documented.** Scenes can be rendered indirectly:

> "use_render_cache
>
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30.

And the release notes record a fix in Cycles whose title concerns a second scene being rendered with the GPU compositor in background mode:

> "* Cycles crashes while trying to render second scene in GPU compositor (-b mode)"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30.

**Not covered.** Whether the second scene's own Compositing switch has any effect in either reading. Whether the second scene's compositor is executed and discarded (Reading A). Whether a Render Layers node pointing at another scene renders that scene every time, or reuses an existing render. Whether either arrangement can nest more than one level deep.

**Where I looked.** Searched the whole manual for `another scene`, `other scenes`, `different scenes`, `indirectly rendered`, `raw`, `composited`, `Render Result`. Searched the whole API docs for `indirectly rendered|another scene|other scene`, and read `bpy.types.CompositorNodeRLayers.txt` in full. Read in full: `compositing/types/input/scene/render_layers.txt`, `video_editing/edit/montage/strips/scene.txt`, `video_editing/sequencer_scene.txt`, `render/layers/introduction.txt`, `render/layers/passes.txt` (Data section), `render/output/properties/post_processing.txt`. Searched all release-notes subjects for `compositor|compositing|output|resolution|render region|save output` and read the Sequencer and Rendering notes in full. Ruled out `compositing/compositor_system.txt` as saying anything about cross-scene execution — it covers data types, domains, and which output node wins, nothing about other scenes.

**Outside the interface.** The Render Layers hint and the pass definition are statements about the data, not about the window, so they hold headlessly — **inferred**, since the sources do not say so. The one thing the sources say that touches headless operation here is the bug-fix title naming `-b` mode above, and I will not read a behaviour off a bug title beyond the fact that the arrangement exists.

**Operating system.** The sources do not say.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

Three parts, answered in turn.

### (a) What it arrives as

**Answer.** As the output sockets of a Render Layers node: an Image socket, an Alpha socket, and one socket per enabled render pass.

> "Render Layers Node
>
> Renders a View Layer and reads its Passes into the compositing node graph.
>
> Inputs
>
> This node has no input sockets."
>
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

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html, read 2026-07-30.

**Documented, from a picture.** The node image on that page (`_images/node-types_CompositorNodeRLayers.webp`) shows a node headed "Render Layers" with two output sockets on the right labelled "Image" (yellow) and "Alpha" (grey), and below them a data-block field reading "Scene" with a clear button, and a dropdown reading "ViewLayer" with a button beside it. Read 2026-07-30.

What the data itself is, once inside:

> "Compositing nodes operate on data that is either an image or a dimensionless single value. For instance, the Levels node outputs a single value, while the Render Layers node outputs an image."

> "Three types of data exist, all of which are stored in half precision formats:
>
> Float
>
> A signed floating-point number. Integer data is also stored as floats because no integer type exist.
>
> Vector
>
> A 4D vector. …
>
> Color
>
> A 4D vector storing the Red, Green, Blue, and Alpha of the color."

> "The compositor is designed in such a way as to allow compositing in an infinite compositing space. Consequently, images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations. An identity transformation represents an image that is centered in space. The rectangular area occupied by an image in that space as defined by its transformation and size is called the Domain of the image."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html, read 2026-07-30.

And what the Combined pass contains:

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

— Manual, https://docs.blender.org/manual/en/5.2/render/layers/passes.html, read 2026-07-30.

**Label:** documented.

### (b) Whether it can be told apart from anything else there

**Answer, in two halves.**

*In the node tree, yes.* The render enters through a node of its own type, `CompositorNodeRLayers`, which carries the scene and layer it reads:

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render
>
> layer
>
> (default 'PLACEHOLDER')
>
> scene
>
> Type:
>
> Scene | None"

— Python API, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeRLayers.html, read 2026-07-30.

That is a distinct node type with named sockets, so a program reading the tree can point at it and say "the render comes in here". **Documented.**

*In the data flowing along the links, not according to the sources.* The only thing the sources describe as reporting what an image on a socket is, is socket inspection, and its list is a closed one:

> "Socket inspection displays information about the image contained in a socket after the node tree has been evaluated.
>
> Rather than displaying the image itself, the inspection panel summarizes properties of the image, making it easier to verify intermediate results and debug node trees.
>
> The following information is displayed:
>
> Resolution – Displays the image resolution. If the image has a display window, both the data window and display window are shown instead.
>
> Transform – Displays the image transform. If no transformation has been applied, Identity is shown. Otherwise, the translation, rotation, and scale are displayed.
>
> Sampling – Displays the interpolation and sampling modes used by the image.
>
> Precision – Indicates whether the image is stored using Full or Half precision."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html, read 2026-07-30. (This is new in 5.2: "Support for socket value inspection was added." — Release notes, https://developer.blender.org/docs/release_notes/5.2/compositor/, read 2026-07-30.)

Nothing in that list is where the image came from. **Label for this half: not in the sources**, with the reservation that reading a closed list as proof of absence is itself a small step — the sources do not say "there is no origin marker", they simply never mention one.

**Where I looked for a marker.** Searched the whole manual and API docs for `provenance`, `distinguish`, `tell apart` (only hit: an unrelated Cryptomatte sentence about objects per pixel), and `is_render` in the API (no hits). Read `compositing/types/input/image_info.txt` — the Image Info node reports Dimensions, Resolution, Location, Rotation, Scale, and nothing about origin. Read `bpy.types.NodeSocket.txt` in the relevant parts — it exposes `identifier`, `type`, `is_linked`, `is_output`, `is_multi_input`, `is_inactive`, `links`, `node`; none of these is an origin. Read `compositing/types/output/viewer.txt` and `editors/image/introduction.txt` (the Image Editor's data-block list distinguishes "Render Result" from "Viewer Node", but that is the editor's list of images, not a mark on data inside the tree).

### (c) Whether it can be followed forward

**Answer: yes, structurally — the sources describe every piece needed to walk the graph forward, including through pass-through nodes and one-to-many fan-out.**

*Fan-out is possible from any output socket.*

> "links
>
> List of node links from or to this socket.
>
> Note
>
> Takes O(len(nodetree.links)) time."

— Python API, https://docs.blender.org/api/5.2/bpy.types.NodeSocket.html, read 2026-07-30.

Each link names both ends: `from_node`, `from_socket`, `to_node`, `to_socket` (Python API, https://docs.blender.org/api/5.2/bpy.types.NodeLink.html, read 2026-07-30).

*Reroute — pure pass-along, and one-to-many by design.*

> "A node used primarily for organization. Reroute looks and behaves much like a socket on other nodes in that it supports one input connection while allowing multiple output connections."

— Manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/types/layout/reroute.html, read 2026-07-30.

> "class bpy.types.NodeReroute(NodeInternal)
>
> A single-socket organization tool that supports one input and multiple outputs"

— Python API, https://docs.blender.org/api/5.2/bpy.types.NodeReroute.html, read 2026-07-30.

*Switch — passes one input through untouched.*

> "The Switch node outputs one of two inputs depending on a condition.
>
> When the inputs are regular values, only the input that is passed through the node is evaluated. This means the unused input is not computed."
>
> "Outputs
>
> Output
>
> One of the two inputs without any modifications."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/switch.html, read 2026-07-30.

New in this version, and worth knowing because it changes which branch is even executed:

> "- The compositor now supports lazy evaluation of Switch nodes whose condition is known statically, that is, not connected or connected directly to a Group Input socket. Lazy Evaluation means nodes that are not needed are not executed to save execution time."

— Release notes, https://developer.blender.org/docs/release_notes/5.2/compositor/, read 2026-07-30.

*Muted nodes — the graph records what a muted node passes through.*

> "internal_links
>
> Internal input-to-output connections for muting (default None, readonly)
>
> Type:
>
> bpy_prop_collection[NodeLink]"

and

> "mute
>
> (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Node.html, read 2026-07-30.

Links themselves can be muted: "is_muted / Link is muted and can be ignored (default False)" (Python API, https://docs.blender.org/api/5.2/bpy.types.NodeLink.html, read 2026-07-30).

*Into and out of node groups.* The Group Output node "defines the final output of a node group. It determines which values are passed out of the node group to the parent node tree (or to the renderer, in the case of the Compositor)." (Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/group_output.html, read 2026-07-30). An Enable Output node can make a group output socket vanish: "When False, the corresponding output socket in the group is hidden." (Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/enable_output.html, read 2026-07-30).

*A caution, documented, for anyone following the path by looking at evaluated values rather than at links:*

> "Socket values are only available after the node tree has been evaluated. Nodes that do not contribute to the final output are generally not evaluated and therefore have no inspection data available."

and

> "Pixel sockets are currently not supported."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html, read 2026-07-30.

**Label:** documented, for the existence and behaviour of each of these pass-through and fan-out mechanisms.

**Also on "split it into more than one":** if the question means the node named Split, that node does the opposite — it *combines*:

> "The Split node combines two input images into a single output image, displaying them side by side or along a rotated split line."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/utilities/split.html, read 2026-07-30.

I read "split it into more than one" as a socket feeding several places, which is the Reroute/multiple-links answer above, and flag the other reading here.

**Not covered.** The sources describe the structures a walk would use; they do not say that the pixels arriving at the far end are the same pixels — only Switch is stated to pass its input "without any modifications", and Reroute is described by what it connects rather than by what it does to values. Whether a render's identity survives being mixed with other images is not addressed at all, and could not be. The sources also do not describe any way to ask, at an arbitrary socket, "is this the render". Also not covered: whether `is_inactive` ("Socket is grayed out because it has been detected to not have any effect on the output") is reliable enough to prune a walk.

**Where I looked.** Searched the manual for `Reroute` (four files), `Split`, `Switch`. Listed every node type under `compositing/types/` and read `utilities/split.txt`, `utilities/switch.txt`, `output/group_output.txt`, `output/enable_output.txt`, `output/viewer.txt`, `output/warning.txt`, `output/file_output.txt`, `input/image_info.txt`, `input/scene/render_layers.txt`, `transform/scale.txt`, `transform/crop.txt`, `compositor_system.txt` in full. In the API docs read `bpy.types.NodeLink.txt`, `bpy.types.NodeReroute.txt`, `bpy.types.CompositorNodeRLayers.txt`, and the relevant parts of `bpy.types.NodeSocket.txt` and `bpy.types.Node.txt`. Confirmed by listing `api_5.2` that there is no `bpy.types.CompositorNodeComposite.txt` — the old Composite node is not in the 5.2 API reference, and the manual's Output Nodes list is "Enable Output Node / Group Output / Viewer Node / Warning Node / File Output Node" (Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/index.html, read 2026-07-30).

**Outside the interface.** All of this is graph structure stored in the file and reachable through the Python API, which the manual describes as usable in background mode ("-b / Render in the background (without UI)", Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). Socket inspection is the exception in kind: it is described as a panel display and its data only exists "after the node tree has been evaluated". Whether inspection data is reachable at all without a window is **not in the sources**.

**Operating system.** The sources do not say. The only OS-specific statement anywhere in the compositing pages is the Windows TDR note quoted in answer 1, which concerns the GPU compositor being interrupted, not what data is or how it flows.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Answer: yes — at least two things, and there is a third that decides shape rather than size.**

### (i) Render Region with Crop to Render Region

> "Render Region
>
> Renders just a portion of the view instead of the entire frame. See the Render Region documentation to see how to define the size of the render region.
>
> Crop to Render Region
>
> Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html, read 2026-07-30.

The region is defined in the file and reaches the final render:

> "Allows you to limit rendering to a 2D rectangular area. …
>
> If you define one while in Camera View, it will apply not just to the viewport, but also to the final render. If you want to temporarily disable this region rather than clearing it entirely, you can do so in the Output tab of the Properties editor."

and

> "Note
>
> Render regions only apply to the viewport when using Cycles, not when using EEVEE. However, they always affect the final render."

— Manual, https://docs.blender.org/manual/en/5.2/editors/3dview/navigate/regions.html, read 2026-07-30.

The properties involved, all scene data:

> "use_border / Render a user-defined render region, within the frame size (default False)"
> "use_crop_to_border / Crop the rendered frame to the defined render region size (default False)"
> "border_max_x / Maximum X value for the render region (in [0, 1], default 1.0)" (and `border_min_x`, `border_min_y`, `border_max_y`)

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30.

**Label:** documented that Crop to Render Region crops *the rendered image* to the region's size — "Crops the rendered image to the size of the render region" is an outright statement about the rendered image. **Inferred** for the step from there to the size of the file written to disk: every quoted sentence says "rendered image" or "rendered frame", never "written" or "saved". *(Citation check, 2026-08-03: previously labelled documented outright, with the question being about "the size of what gets written". The step is now marked. This topic's 5.1 second-pass findings flag the same gap.)*

**Documented, from a picture.** The Format panel screenshot (`_images/render_output_properties_format_panel.png`, on the Format page above) shows, in order: Resolution X 1920 px, Y 1080 px, % 100%, Aspect X 1.000, Y 1.000, an unticked "Render Region" checkbox, a greyed-out unticked "Crop to Render Region" checkbox, and Frame Rate 24 fps. Read 2026-07-30. Visible in the picture: Crop to Render Region is greyed out while Render Region is off. Not visible, and not to be assumed from the picture: why.

### (ii) Stereoscopy — the Views Format used when saving

When a scene has more than one view, the two views can be written into one image:

> "Multi-view images can be saved in special formats according to the production requirements. By default the system saves each view as an individual file, thus generating as many files as views to be rendered."

> "Lossy-Formats
>
> Some stereo 3D formats represent a considerable loss of data. For example, the Anaglyph format will cap out entire color channels from the original image. The Top-Bottom compressed will discard half of your vertical resolution data. …
>
> Lossless Formats
>
> … For the Stereo 3D formats the only lossless options are Top-Bottom and Side-by-Side without the Squeezed Frame option."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html, read 2026-07-30.

The settings:

> "views_format / Format of multiview media (default 'INDIVIDUAL')"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html, read 2026-07-30.

> "display_mode … SIDEBYSIDE / Side-by-Side – Render views for left and right eyes side-by-side. TOPBOTTOM / Top-Bottom – Render views for left and right eyes one above another."
> "use_squeezed_frame / Combine both views in a squeezed image (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Stereo3dFormat.html, read 2026-07-30.

**Label:** **inferred**, for the size claim. The step: the sources say two views are put side-by-side or one above another in a single image, and that this is lossless only when Squeezed Frame is off, while squeezed Top-Bottom "will discard half of your vertical resolution data". For both views to be present losslessly in one image, that image must be twice the scene's dimensions along one axis. The sources never write down that number. Also documented outright, and it is a size effect of a different kind: with the default INDIVIDUAL format you get "as many files as views to be rendered".

### (iii) Aspect X/Y — shape, not pixel count

> "Aspect X, Y
>
> Older televisions may have non-square pixels, so this can be used to control the shape of the pixels along the respective axis. This will pre-distort the images which will look stretched on a computer screen, but which will display correctly on a TV set."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html, read 2026-07-30. **Documented**, and it is not a pixel-count change; I list it because it is easy to mistake for one.

### What does *not* decide it, which is the more useful half of the answer

**The compositor does not.** Output nodes are pinned to the scene's render size:

> "Output Nodes
>
> The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html, read 2026-07-30.

Said again from the stereo side:

> "The views to render are defined in the current scene views, in a similar way as you define the composite output resolution in the current scene render panel, regardless of the Image nodes resolutions or Render Layers from different scenes."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html, read 2026-07-30.

So the compositor's own size-changing nodes — Crop ("by either making the cropped area transparent or by resizing the input image"), Scale with Space set to Render Size ("Image dimensions set in the Render panel") — change images inside the compositing space, and the output node's domain is what governs the result. **Label: documented** that output-node domain is the scene render size and that composite output resolution is set in the render panel. **Inferred**, and flagged as a step: that the File Output node follows the same rule as the Viewer node. The passage says "output nodes like the Viewer node" and names Viewer as the example; File Output is listed under Output Nodes in the manual, but the passage does not name it. If a File Output node writes its input at that input's own size instead, this answer is wrong on that point and the sources do not settle it.

**One limit worth knowing.** For the GPU compositor:

> "Resolution
>
> Images with resolution larger than 8k are not supported and are cropped/scaled."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/limitations.html, read 2026-07-30. Whether that reaches what gets written at very large render sizes is not stated.

**One thing that only looks like an answer.** The Video Sequencer has an operator "Set Render Size — Sets the render resolution and aspect to match the strip's resolution" (Manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/editing.html, read 2026-07-30). That is a person pressing a menu item that writes the scene's own resolution fields — not a separate decider.

**Not covered.** Whether the File Output node can write at a size other than the scene render size (see above). Whether metadata "Burn into Image" adds anything to the image's dimensions — the manual says only "Add metadata as text to the render" (Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html, read 2026-07-30) and the API says "use_stamp / Render the stamp info text in the rendered image", which reads as drawn into the image rather than added around it, but neither says so. Whether a codec silently changes dimensions — the manual warns "some codecs do not work with arbitrary dimensions" without saying what happens then. Whether Pixel Density changes anything — it explicitly does not: "The pixel-density is meta-data which doesn't impact the quality of the image."

**Where I looked.** Searched the whole manual for `final render resolution`, `output resolution`, `final compositor output`, `scene render size`, `render resolution`, `Render Region`, `Squeezed`, `Views Format`. Searched the API docs for `resolution_x|resolution_y|resolution_percentage`, `views_format`, `use_squeezed_frame`, and listed every `use_*` on `bpy.types.RenderSettings`. Read in full: `render/output/properties/format.txt`, `render/output/properties/output.txt`, `render/output/properties/metadata.txt`, all three stereoscopy pages, `editors/3dview/navigate/regions.txt` (Render Region section), `editors/3dview/viewport_render.txt`, `compositing/compositor_system.txt`, `compositing/limitations.txt`, `compositing/types/transform/scale.txt`, `compositing/types/transform/crop.txt`, `bpy.types.Stereo3dFormat.txt`. Ruled out: Frame Range, Encoding, Color Management and Pixel Density panels (read, nothing about pixel dimensions); Cycles' own `ui.py`, searched for `resolution_percentage|use_border|use_crop` — its only hit computes render size from the scene's own two settings to warn about tiling, and adds nothing.

**Outside the interface.** Every setting named here is scene or image-format data stored in the file, so it applies to a headless render — **inferred**, the sources do not say it for these settings specifically. One caution the sources do make explicit is about the interface: "By default, when compositing and rendering from the user interface all views are rendered and then composited" (stereoscopy page above), which describes behaviour specifically "from the user interface" and says nothing about the other case.

**Operating system.** The sources do not say.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Answer: yes.** Compositing and Sequencer are two independent switches in the same panel. Turn Compositing off and leave Sequencer on, and the scene's output comes from the Video Sequence editor.

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node.
>
> Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html, read 2026-07-30.

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30.

**Observed, from Cycles' own files** — the two are drawn as two separate properties under one heading:

```
        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

— `Blender Sources/cycles_5.2/ui.py`, lines 1181–1183, read 2026-07-30. *(Citation check, 2026-08-03: the sentence above previously ended "so neither depends on the other." The file shows two properties being drawn; that one can be set without the other is a step past what the file says. Narrowed. The API defaults quoted above, both True, are what carry their being separate settings.)*

**Documented, from a picture.** The Post Processing screenshot described in answer 1 shows "Compositing" and "Sequencer" as two separate ticked checkboxes side by side. Read 2026-07-30.

**Label:** documented, that the two switches exist independently and that Sequencer renders the Video Sequence editor's output "instead of the view from the 3D scene's active camera". **Inferred**, and the step is small but real: that turning Compositing off while leaving Sequencer on is what "bypass the compositor and produce output from the VSE" means in practice. The sources describe each switch on its own; they never spell out that combination.

### What that changes about what gets written

**Documented.**

*The source of the image changes* — it becomes the Sequencer's output rather than the camera's view, per the quote above.

*Passes are lost, if Scene strips are involved.*

> "Limitations
>
> Scene strips do not render individual Render Passes; only the Combined render pass will be used."

— Manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html, read 2026-07-30.

*Multi-layer EXR is not on the table through the Sequencer.*

> "Note
>
> The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html, read 2026-07-30.

*Metadata can come from the strips instead of the scene.*

> "Metadata Input
>
> Where to grab metadata from.
>
> Scene: Use metadata from the current scene.
>
> Sequencer Strips: Use metadata from the strips in the Sequencer."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html, read 2026-07-30.

*And, if Save Output has been switched off, the Sequencer being active changes the failure mode:*

> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30.

**One more thing about 5.2 that bears on "its video sequence editor".** In this version the edit can live in a scene of its own, and rendering it is a separate choice:

> "The video sequencer uses a separate scene data-block that contains the edit. This is the Sequencer Scene."
>
> "Rendering
>
> The render and output settings of the Sequencer Scene are the same as for regular scenes. They can be found in the Output tab in the Properties editor. Note that these settings follow the active scene in the window by default (unless there is a Scene pinned to the properties editor).
>
> When the active scene is different from the Sequencer Scene in the current workspace, the "Render" menu will include separate options to render the Sequencer Scene (and not the active Scene)."

— Manual, https://docs.blender.org/manual/en/5.2/video_editing/sequencer_scene.html, read 2026-07-30.

And in the API: "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene (optional)" — a parameter of `bpy.ops.render.render` (Python API, https://docs.blender.org/api/5.2/bpy.ops.render.html, read 2026-07-30).

**Not covered.** The sources do not say whether the written image's size changes (I found nothing saying it does; answer 6 covers what does decide size). They do not say what happens with Sequencer on and no strips at all — the property text says "if sequencer strips exist", which suggests the pipeline is simply not applied, but does not say what is written instead. They do not say whether the compositor is skipped entirely or merely ignored when Compositing is off and Sequencer is on. And, as in answer 4, the sentence "If Compositing is also enabled, the Scene strip will be the output of the Compositor" does not say whose Compositing switch is meant.

**Where I looked.** Searched the whole manual for `Sequencer`, `Post Processing`, `Scene strip`, `Set Render Size`, `multi-layer EXR`. Searched the API docs for `use_sequencer`, `use_sequencer_override_scene_strip`, `sequence_editor|sequencer` within `bpy.types.Scene.txt`, and read `bpy.ops.render.render`'s parameter list in full. Read in full: `render/output/properties/post_processing.txt`, `render/output/properties/output.txt`, `render/output/animation.txt`, `render/output/properties/metadata.txt`, `video_editing/sequencer_scene.txt`, `video_editing/edit/montage/strips/scene.txt`, `compositing/usage.txt`. Read the 5.2 Sequencer release notes in full and searched the bug-fix list for sequencer and compositing entries.

**Outside the interface.** `use_compositing` and `use_sequencer` are scene properties in the file, so the arrangement holds for a render driven from outside — **inferred** on the same grounds as answers 1 and 2 ("all the settings saved in the blend-file"). One genuine difference the sources do describe: choosing to render the Sequencer Scene rather than the active scene is presented as a menu item in the interface, and as the `use_sequencer_scene` parameter through the API; the command-line arguments page has `-S, --scene <name>` for picking a scene and says nothing about the Sequencer Scene, so how a headless run selects it is **not in the sources**.

**Operating system.** The sources do not say.

---

## Things a person sitting at a computer with Blender 5.2 open could settle in a few minutes

**Most of this list has been run, and the results are settled in the first-pass document**, `Preparation/Blender Research/Findings/5.2/The compositor.md`, where the observed findings and the machine's raw output are recorded under the questions they answer. The set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

Settled there, by number below: **2** (a Render Layers node pointing at another scene brings back that scene's raw render, and that scene's compositor does no work at all — its own File Output node writes nothing, whichever way its Compositing switch is set), **3** (with no active output node the compositor does not run and the raw render is written, and a group with no Render Layers node in it still runs), **4** (a File Output node writes at its input's size, not the scene's — 122×90 from a Scale set to 120×90, against a 400×200 scene), **5** (unsqueezed Side-by-Side doubles the width and unsqueezed Top-Bottom doubles the height, while both squeezed forms keep the scene's dimensions), **6** (Crop to Render Region writes at exactly the region's size), **7** (with Save Output off, an `-o` override on the command line writes nothing from the Output panel while a File Output node still writes both frames), and **8** (turning Compositing off with Sequencer on writes the strip's picture at the scene's own size).

**One correction this pass earned.** Answer 6 above says, as an inferred step, that the File Output node follows the same rule as the Viewer node and is fixed to the scene render size — and adds that if it writes its input at that input's own size instead, "this answer is wrong on that point". The test showed exactly that. The observed finding is in the first-pass document under question 6.

**The last two are settled since, by the Creator looking at a Blender window.** Both are recorded in the same first-pass document, from `Audits/Blender Research/The compositor/Hand tests.md`, entries 16 and 19.

**1** — there is a checkbox in the Output panel header. It sits immediately to the left of the word "Output", carries no words of its own, and starts ticked. Its full data path is `bpy.data.scenes["Scene"].render.save_output`. The Creator gave that path rather than the tooltip, so the tooltip's wording is still not recorded. That is under question 2.

**9** — socket inspection on a link out of a Render Layers node, after a render, shows Resolution (1920x1080), Transformation (Identity), Sampling (Interpolation Bilinear, Extension Clip), Precision (Full) and one further line, "Type: Image". Before a render the same hover shows a different set — a Value, a Type of "Float Color", a colour swatch, Display RGB, HSV, Alpha and Hex. A link out of an ordinary Image node shows only the word "Image". **No line in any of it names where the image came from.** The fuller display only appears once the eye icon in the node's upper-left corner has been clicked. That is under question 5.

Nothing is left on this list.

**A second machine, 2026-08-06.** Every test named above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are recorded beside the Mac's in the first-pass document, never merged with them. All of them agreed with the Mac, including item 4's odd 122×90 and item 7, where Save Output off beat an `-o` on the command line. **One exception:** item 9, socket inspection, could not be settled on that machine at all, because its display is shared with other work, the pointer would not stay put, and a click does not appear to reach Blender there — and the 5.2 reading needs the node's eye icon clicked first. So item 9 still rests on the Mac alone. One new test was added that only the second machine could carry, and it matters most in this version, because 5.2 is the one whose compositor device starts on the graphics card: whether a compositor tree gives the same picture on the card as on the processor. It does, at 320×240, for the four trees tried. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

The original list follows unchanged.

1. **Is there a checkbox in the Output panel header?** Open Blender, go to Properties ‣ Output Properties, and look at the header of the panel titled "Output". Report whether a checkbox is present to the left of the word "Output", and what its tooltip says. (The manual text says there is one; the manual's own screenshot of that panel does not show one.)

2. **Does a Render Layers node pointing at a second scene run that second scene's compositor?** In a new file, create Scene B. In Scene B, add a compositing node group with something unmissable in it — a Group Output fed through an RGB Curves or Hue/Saturation node set to make the image obviously wrong, e.g. fully inverted. Enable Properties ‣ Output ‣ Post Processing ‣ Compositing in Scene B. Then in Scene A, add a Render Layers node with Scene set to B, feed it straight to Scene A's Group Output, and render Scene A (F12). Report whether the result shows Scene B's raw render or Scene B's inverted composite. Then repeat with Scene B's Compositing switch turned off and report whether the result or the render time changes.

3. **Does the compositor run when the group has no active output?** In one scene, assign a compositing node group, delete the Group Output node and any Viewer node, add an expensive node (a large Bokeh Blur) fed from Render Layers but connected to nothing, and render. Report whether render time changes compared with the same file with the expensive node deleted. This tests the sentence "If none was found, the compositor doesn't run altogether."

4. **Does a File Output node write at the scene's render size, or at its input's size?** Set the scene to 1920×1080. In the compositor, feed Render Layers into a Crop node with Alpha Crop *off* and a small crop region, then into a File Output node. Render one frame and report the pixel dimensions of the file written. Repeat with a Scale node set to Absolute 640×360 in place of Crop. This is the one inference in answer 6 I could not close.

5. **What are the pixel dimensions of a stereo Side-by-Side file?** Set the scene to 1920×1080, enable Stereoscopy in Output Properties, set the output Views Format to Stereo 3D with display mode Side-by-Side, Squeezed Frame off, render one frame, and report the written file's pixel dimensions. Then repeat with Squeezed Frame on, and with Top-Bottom in both states.

6. **Crop to Render Region — exact written size.** Set the scene to 1920×1080. In Camera View press Ctrl-B and drag a region over roughly the middle quarter. Enable Crop to Render Region in Properties ‣ Output ‣ Format. Render one frame, write it out, and report the file's pixel dimensions alongside the four Render Region values shown in the Output tab.

7. **Save Output off, plus `-o` on the command line.** Take a file with Save Output turned off in the Output panel and no File Output node, and run `blender -b file.blend -o /tmp/test_ -F PNG -a` with a two-frame range. Report exactly what appears on the console and whether any file is written. Then repeat with a File Output node present, and report again.

8. **Compositing off, Sequencer on.** In a scene with strips in the Video Sequence editor and a compositing node group that visibly alters the image, turn Compositing off and leave Sequencer on, render one frame and note what is written; then turn Compositing back on and render again. Report the difference, and also report the pixel dimensions in both cases (to settle whether the Sequencer route changes size).

9. **Does socket inspection say anything about where an image came from?** With a node tree containing both a Render Layers node and an Image node, hover or open the socket inspection display on a link out of each, and report every line it shows. This settles whether anything beyond Resolution, Transform, Sampling and Precision is displayed.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 5.2

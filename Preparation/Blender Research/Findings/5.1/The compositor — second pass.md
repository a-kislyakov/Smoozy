# The compositor — Blender 5.1 — second pass

**Sources consulted, and when.** Blender's 5.1 manual, Blender's 5.1 Python API documentation, Blender's 5.1 release notes, and Cycles' own files as shipped with Blender 5.1 — all as the plain-file copies in `Blender Sources/`, taken on 2026-07-30. That snapshot date is the date read for every finding below. Nothing from the wider web was used.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.** One exact qualification, so the line above is not overstated: to confirm the folder I was told to save into existed, I listed the directory `Preparation/Blender Research/Findings/5.1/`. That showed me file *names* only. I opened none of them, including the first-pass findings document for this topic, which I was told not to read.

**Helpers.** I put two helper agents to work on the wide searches behind answers 2 and 6. Each was given the full warning block, the same two readable things, and nothing else; each reported back verbatim quotes with file and line, and each declared it read nothing outside `Blender Sources/` and started no helpers of its own. I re-opened and re-verified every quote from them that appears below, in the source files myself, before using it. I vouch for both.

**A note on what "the compositor" is in 5.1.** Several answers below turn on a fact worth stating once at the top, because it is not what a reader coming from an older Blender expects. In 5.1 a scene's compositor is a **node group** assigned to the scene, and its result leaves through a **Group Output** node. There is no node called "Composite". The manual's own list of compositor output nodes is: "Enable Output Node / Group Output / Viewer Node / File Output Node" (manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/index.html, read 2026-07-30).

---

## 1. What decides whether a file's compositor runs at all?

**Label: documented** (four separate gates, each documented outright), with one **observed** confirmation from Cycles' own file. **Inferred** for the sentence below that all four must be satisfied together: each gate is documented on its own page, and no source presents them as one joint condition or as a complete list.

Four things decide it. *(Citation check, 2026-08-03: this sentence previously read "Four things decide it, and they are independent — all four must be satisfied," under the documented label. The conjunction is a step past the quoted words and is now marked inferred. The "Not covered" paragraph below already said the completeness of the list is not documented.)*

### (a) The scene's Compositing switch, in Output ▸ Post Processing

> "To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline
> option must be enabled in the output's Post Processing properties."

Manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

The panel itself:

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node."

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

The picture on that page, `_images/render_output_properties_post-processing_panel.png`, which I opened, shows a panel headed "Post Processing" containing two labelled checkboxes on one row — "Compositing" and "Sequencer", both ticked — and below them a "Dither" slider reading 1.000. **Label for the picture: documented** (it is part of the manual). What is visible is two checkboxes; anything about what ticking them does comes from the words, not the picture.

The property behind the checkbox:

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

### (b) A compositing node group must be assigned to the scene

That is the "if" clause in the sentence just quoted. The property it refers to:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

Python API, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

### (c) The node tree must contain an active output node

> "The compositor searches for the active Group Output node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether."

Manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30.

The same page, immediately above, says only one output can be live:

> "The compositor only supports a single active output target, that is, only one of the
> Group Output nodes or Viewer nodes in the node tree will be considered active and the rest will be
> ignored. In particular, the compositor searches the Active Node Tree Context and falls back to the
> Root Node Tree Context if no active output was found in the active node tree context."

Same page, same address, read 2026-07-30.

### (d) The render engine must declare that it does post-processing at all

> "bl_use_postprocess
>
> Apply compositing on render results (default False)"

Python API, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

This is a class attribute an add-on render engine declares when it registers, not a setting in the file. It is listed here because it is a documented gate on whether compositing is applied. Cycles does not set it in its own registration — Cycles' `__init__.py` declares `bl_idname`, `bl_label`, `bl_use_eevee_viewport`, `bl_use_preview`, `bl_use_exclude_layers`, `bl_use_spherical_stereo` and `bl_use_custom_freestyle`, and nothing else beginning `bl_use_post`. **Label: observed** (I read the file). What that absence means for a Cycles render is a further step I am not taking.

### What no longer decides it

> "use_nodes
>
> Enable the compositing node group. (default False)
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

Python API, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30. This is a documented statement that the old switch is now inert. Consistent with it, I found no "Use Nodes" control anywhere in the 5.1 manual's compositing pages (searched the whole manual, case-insensitively, for `use nodes`; the only hits in the compositing area were ordinary prose — "you use nodes to import the image").

### The viewport preview is a separate, unrelated switch

> "use_compositor
>
> When to preview the compositor output inside the viewport (default 'DISABLED')
>
> DISABLED
> Disabled – The compositor is disabled.
>
> CAMERA
> Camera – The compositor is enabled only in camera view.
>
> ALWAYS
> Always – The compositor is always enabled regardless of the view."

Python API, https://docs.blender.org/api/5.1/bpy.types.View3DShading.html — read 2026-07-30. Matching manual text at https://docs.blender.org/manual/en/5.1/editors/3dview/display/shading.html ("Compositor / When to preview the result of compositing in the 3D Viewport. / Disabled: / Never show the compositing output."). This is a per-viewport display setting; it does not touch what a render does.

### Observed, from Cycles' own file

The Post Processing panel Cycles draws is built from exactly the two properties named above. From `Blender Sources/cycles_5.1/ui.py`, lines 1112–1128 (class `CYCLES_PT_post_processing`):

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

**Label: observed.** What the file says is that when Cycles is the engine, the Output tab's Post Processing panel draws `use_compositing` and `use_sequencer` under the heading "Pipeline". What a render then *does* with those values is a step beyond the file and is not claimed here.

**Not covered.** Whether the four gates are the complete list — the sources describe features, and none of them presents a definitive checklist of "the compositor runs when…". I did not find, and so cannot say, what happens if the active Group Output's image socket is disabled by an Enable Output node, or if the assigned node group is empty of anything but a Group Output. Nor whether a compositor with no Render Layers node still "runs" in the sense meant here. Nothing in the sources states the order in which the gates are checked, or which one produces which message when it fails.

**Outside the interface.** The sources do not say directly. Three nearby documented facts, and the step between them. The command-line page says `-a` means "Render the whole animation using all the settings saved in the blend-file" (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html). Across the whole of `RenderSettings` exactly one property carries a caveat about the interface — "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" (Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html). The manual says the same thing about the same setting in its own words: "Render Single Layer / Only render the active view layer. / Note / This option is ignored when rendering from the command line." (manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html). `use_compositing` carries no such caveat anywhere. Concluding from that silence that it behaves identically outside the window is a step the reader has to supply: **inferred**, and worth confirming by hand — Blender demonstrably *does* skip some render settings outside the window, and documents it when it does. One thing that plainly does differ: the viewport preview switch described two sections above is a property of a 3D Viewport's shading, and a background-mode run has no viewport — but that too is inference, not a documented statement. All dates read 2026-07-30.

**Operating system.** The sources say nothing about the gates differing by operating system. The one OS-specific statement I found anywhere in the compositing chapter is about speed, not about whether it runs:

> "TDR on Windows
>
> Timeout Detection Recovery
> (TDR)
> resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Manual, https://docs.blender.org/manual/en/5.1/compositing/limitations.html — read 2026-07-30. That page also says "General hardware limitations apply when using the GPU compositor." Note the device is a setting: `compositor_device`, "Set how compositing is executed (default 'CPU')" (Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html).

**Where I looked.** Manual pages opened: `compositing/introduction`, `compositing/compositor_system`, `compositing/limitations`, `compositing/compositing_workspace`, `compositing/sidebar`, `compositing/index`, `compositing/types/output/index`, `compositing/types/output/group_output`, `compositing/types/output/viewer`, `compositing/types/output/enable_output`, `render/output/properties/post_processing`, `render/output/introduction`, `editors/compositor`, `editors/3dview/display/shading`, `advanced/command_line/render`, `advanced/command_line/arguments`. Pictures opened: `render_output_properties_post-processing_panel.png`. API pages opened: `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.CompositorNodeTree`, `bpy.types.NodeTree`, `bpy.types.View3DShading`, `bpy.types.RenderEngine`, `bpy.ops.render`, `bpy.ops.node`. Cycles files opened: `ui.py`, `__init__.py`. Release notes read in full: `compositor.md`, `index.md`, `sequencer.md`, `rendering.md`, `pipeline_io.md`. Searches run across the whole of `manual_5.1` and `api_5.1`: `use_compositing`, `use_sequencer`, `use_nodes`, `use nodes`, `post processing`, `compositing_node_group`, `CompositorNodeTree`, `use_compositor`, `viewport compositor`, `bl_use_postprocess`, `compositor_device`, `Windows`, `macOS`, `Linux`, `command line`, `background mode`, `ignored for rendering from command line`.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Reading taken.** "The place where a scene's rendered frames are written" I have read as the scene's own render output destination — Properties ▸ Output ▸ Output ▸ File Path, i.e. `scene.render.filepath`. The other reading available is the compositor's File Output node, which also writes frames but to its own path; I cover that at the end because the answer there is different.

**Label: not in the sources** — for the reading taken. There is no documented setting in a Blender file that switches the scene's render output destination off. What exists instead is one per-call operator argument, one add-on-engine class flag, and an error state. Each is set out below with its exact words, because each is close enough to be mistaken for the thing asked about.

### The Output panel has no such control

The manual's Output panel page enumerates its whole contents — File Path, Saving ▸ File Extensions, Saving ▸ Cache Result, Media Type, File Format, Color, Overwrite, Placeholders, Color Management, Pixel Density — and no enable or disable control appears among them. The page's own opening:

> "This panel provides options for setting the location of rendered frames for animations,
> and the quality of the saved images.
>
> File Path
>
> Choose the location to save rendered frames."

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

I opened the panel picture named on that page, `_images/render_output_properties_output_panel.png`. It shows a panel headed "Output" with a disclosure triangle and the drag-grip dots at the right — **no checkbox in the header** — then a text field reading `/tmp/` with a folder-browse button beside it and **no checkbox beside it**, then the Overwrite / Placeholders / File Extensions / Cache Result checkboxes, File Format PNG, Color BW/RGB/RGBA, Color Depth, Compression. **Label: documented**; what I take from it is only what is visible, namely the absence of a checkbox on the header and on the File Path row. Two cautions I must record. First, other panels in this same manual do render header checkboxes in their screenshots — the Stereoscopy panel picture shows a ticked box in its header — so the convention is visible when present, which makes its absence here meaningful. Second, this particular screenshot looks older than the text beside it: the text documents a **Media Type** dropdown that the picture does not show. So the picture is weaker evidence about 5.1's exact panel than the text is.

The property carries no documented special meaning for being blank:

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)"

Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The two nearest booleans govern *replacement* of a file, not whether the destination is live:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Same page, same address, read 2026-07-30. There is no `use_file_output`, `use_render_output` or `use_output` in `RenderSettings`; the complete set of its `use_*` booleans is `use_border`, `use_compositing`, `use_crop_to_border`, `use_file_extension`, `use_freestyle`, `use_high_quality_normals`, `use_lock_interface`, `use_motion_blur`, `use_multiview`, `use_overwrite`, `use_persistent_data`, `use_placeholder`, `use_render_cache`, `use_sequencer`, `use_sequencer_override_scene_strip`, `use_simplify`, `use_simplify_normals`, `use_single_layer`, `use_spherical_stereo`, `use_stamp` and the `use_stamp_*` family.

And there is no "none" among the media types:

> "media_type
>
> The type of media to save (default 'IMAGE')
>
> Type:
>
> Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']"

Python API, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html — read 2026-07-30. The file-format enum likewise has no `NONE` entry (AVIF, JPEG, OPEN_EXR, PNG, WEBP, BMP, CINEON, DPX, IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, OPEN_EXR_MULTILAYER, FFMPEG).

### The three near misses

**A per-call argument, for stills only.** **Label: documented.**

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
>
> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

Python API, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. So a single-frame render can be told not to write, per call. It is an argument, not a setting stored in the file, and by its own words it does not apply to animation renders. The manual confirms animation frames are written unconditionally:

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

Manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

**A render-engine class flag.** **Label: documented.** This is the only thing in any of the sources that, when off, stops files being written during an animation render:

> "bl_use_image_save
>
> Save images/movie to disk while rendering an animation. Disabling image saving is only supported when bl_use_postprocess is also disabled. (default True)"

Python API, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30. It is declared by a Python-registered render engine add-on at registration time. It is not in the file, not in the interface, and not something a scene can set. Cycles does not declare it (see answer 1).

**An error, not a switch.** **Label: documented.**

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30. That produces no file, but as a failure, not as a supported off state.

### The other reading — the File Output node

If the question means the compositor's File Output node, the answer changes and is **documented**. That node has no enable toggle of its own — its documented properties are Directory, File Name, Media Type, Node Format, Images/Layers, Override Node Format, Item Format and Output Paths (manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html) — but every node can be muted:

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change.
> Links will appear red as an indicator of passing through the muted node."

Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html — read 2026-07-30. The corresponding property is `mute` on `Node` ("(default False)", Python API, https://docs.blender.org/api/5.1/bpy.types.Node.html). Whether muting a File Output node stops it writing is the step past what the words say — that sentence is about contribution to the tree and about links, not about files on disk. **That step is inferred, and I flag it rather than assert it.**

**Not covered.** I cannot say what Blender does when the File Path is left blank — the API documents `""` as the default value and says nothing about what it means at render time. I cannot say whether a scene that is never rendered directly (only pulled in by another scene) writes to its own path. I cannot say what happens when the path points somewhere unwritable. And I cannot settle the muted-File-Output-node question above.

**Outside the interface.** No documented difference, because there is no switch to differ. What the sources do say about output from outside the window is that the *order of arguments* silently decides where frames land:

> "Arguments are executed in the order they are given. eg:
>
> blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
>
> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> blender --background test.blend --render-output /tmp --render-frame 1
>
> …works as expected."

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. That misdirects output; it does not switch it off. The `-o, --render-output <path>` argument only sets a path and has no counterpart that unsets one; the same page notes "When the filename does not contain #, the suffix #### is added to the filename", which is Blender filling a gap in rather than declining to write.

**Operating system.** The sources say nothing about the output destination behaving differently by operating system. The only platform remark on the Output page is about video codecs — "compatibility across platforms, and playback quality" — which is about what will play back, not about whether a file is written.

**Where I looked.** (This answer rests on a helper's sweep, every quote of which I re-opened and confirmed myself.) Manual pages opened: `render/output/properties/output`, `render/output/properties/post_processing`, `render/output/properties/index`, `render/output/properties/format`, `render/output/animation`, `render/output/introduction`, `render/introduction`, `files/file_paths`, `files/media/image_formats`, `video_editing/sequencer_scene`, `advanced/command_line/render`, `advanced/command_line/arguments`, `interface/controls/nodes/editing`, `interface/window_system/topbar`, `editors/preferences/file_paths`, `editors/3dview/viewport_render`, `compositing/types/output/*`. Pictures opened: `render_output_properties_output_panel.png`, and as controls for the checkbox convention `render_output_properties_stereoscopy_usage_views-panel.png` and `render_output_properties_metadata_panel.png`. API pages opened: `bpy.types.RenderSettings` (in full), `bpy.types.ImageFormatSettings`, `bpy.types.RenderEngine`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeGroupOutput`, `bpy.types.ViewLayer`, `bpy.types.Node`, `bpy.types.Scene`, `bpy.types.PreferencesFilePaths`, `bpy.ops.render`, plus the `image_type_all_items` enum list and the general index. Cycles files opened: `__init__.py`, `operators.py`, `ui.py`. Terms searched across all four 5.1 source folders: `use_file_output`, `use_render_output`, `use_output`, `render_output_enabled`, `output_enabled`, `disable_output`, `no_output`, `skip_output`, `use_save`, `use_write`, `render to memory`, `without saving`, `discard`, `not saved to disk`, `no file is written`, `not written`, `does not write`, `will not be saved`, `null output`, `dummy output`, `output is disabled`, `turn off the output`, `Save Output`, `Use Output`, `Enable Output`, `output destination`, `skip writing`, `suppress`, `output path`, `render output`, `save the rendered`, `to disk`, `media_type`, `'NONE'`, `write_still`, `frame_path`, `bl_use_image_save`, `bl_use_postprocess`, `is_active_output`, `render.filepath`, `mute`, `Placeholders`, `Overwrite`, `cannot write`, `unable to save`, `path template`, `filepath`, `File Path`, `Output File`, and `output` case-insensitively across the whole API index. Ruled out along the way, each for the reason given: `use_compositing` and `use_sequencer` (govern pipeline processing, not file writing); `use_render_cache` (writes an *extra* cache elsewhere); `use_single_layer` (restricts which layer renders); `ViewLayer.use` (disables a view layer); `ImageFormatSettings.use_preview` (adds a file, cannot subtract one); `PreferencesFilePaths.render_output_directory` (a default seed for new scenes); `SpaceProperties.show_properties_output` (shows or hides a tab in the interface); `Scene.use_nodes` (documented no-op).

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Label: documented.** Both, in two different senses, and the distinction is the answer.

**The node tree is a file-level thing.** It is a data-block, of the same standing as a mesh or a material, and it lives in the file's list of node groups.

> "class bpy.types.CompositorNodeTree(NodeTree)
>
> Node tree consisting of linked nodes used for compositing"

with, above it, "base classes — bpy_struct, ID, NodeTree". Python API, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeTree.html — read 2026-07-30.

> "node_groups
>
> Node group data-blocks (default None, readonly)"

Python API, https://docs.blender.org/api/5.1/bpy.types.BlendData.html — read 2026-07-30.

**Which tree a scene uses is a per-scene setting.** The scene holds a pointer to one:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

Python API, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

The manual says the same in plain words:

> "Node Tree
>
> Chooses which node group (compositor node tree) to display and edit.
> Each Scene can have its own compositor node tree.
> You can also create and switch to custom node groups for reuse."

Manual, https://docs.blender.org/manual/en/5.1/editors/compositor.html — read 2026-07-30.

**And one tree can serve more than one scene.**

> "This adds a compositing node tree data block to the scene, which can then be reused
> across different scenes or linked/appended from other blend-files."

Manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

That sharing is real enough that Blender ships an operator for breaking it:

> "bpy.ops.node.duplicate_compositing_node_group()
>
> Duplicate the currently assigned compositing node group."

Python API, https://docs.blender.org/api/5.1/bpy.ops.node.html — read 2026-07-30. Alongside it, "bpy.ops.node.new_compositing_node_group(*, name='') / Create a new compositing node group and initialize it with default nodes", same page.

**And compositor node trees are used by things that are not a scene's compositor at all.** A video-sequencer strip modifier takes one:

> "class bpy.types.SequencerCompositorModifierData(StripModifier)
>
> Compositor Modifier
>
> node_group
>
> Node group that controls what this modifier does
>
> Type:
>
> NodeTree"

Python API, https://docs.blender.org/api/5.1/bpy.types.SequencerCompositorModifierData.html — read 2026-07-30. The manual describes the same thing from the user's side: "Compositor modifier is powerful tool to create arbitrary effects. To use it, you need to open compositor editor and change node subtree to sequencer. Then you can add new node tree and use it for any strip. These node trees can be marked as asset and reused across multiple .blend files." (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strip_modifiers.html). Release notes for 5.1 record a change here: "The 'Compositor' modifier can now be created together with a compositing node tree directly from the compositor." (release notes, https://developer.blender.org/docs/release_notes/5.1/sequencer/).

So: **a compositor node tree belongs to the file; the decision to run one belongs to a scene.** Each scene points at its own, and two scenes may point at the same one. There is also a per-editor "Pinned" option that decouples what you are *looking at* from the active scene — "When enabled, the editor always displays the selected node tree, regardless of the active scene." (manual, https://docs.blender.org/manual/en/5.1/editors/compositor.html) — which is a viewing convenience, not ownership.

**Not covered.** The sources do not say what happens on render when two scenes share one tree and both are rendered — whether the tree is evaluated once or twice, or whether a Render Layers node inside a shared tree resolves relative to the scene being rendered or to its own Scene setting. That last one matters and I could not settle it. Nor do the sources say whether a scene may have no tree assigned at all versus always having one; the `use_compositing` wording ("if a compositing node group is assigned to the scene") implies the unassigned state exists, but that is an implication I am drawing, not a statement.

**Outside the interface.** No difference is stated, and none of the ownership facts above is about the interface — they are properties of data-blocks, which are the same whether or not a window is open. The one item on this answer that *is* interface-only is the "Pinned" option, which is a property of an editor. **Label for the "no difference" part: inferred**, on the step that a data-block relationship saved in the file does not depend on a window being open.

**Operating system.** The sources say nothing, and nothing here is platform-dependent in any way the sources describe.

**Where I looked.** Manual: `editors/compositor`, `compositing/introduction`, `compositing/index`, `compositing/compositing_workspace`, `compositing/types/groups`, `interface/controls/nodes/node_editors`, `interface/controls/nodes/groups`, `video_editing/edit/montage/strip_modifiers`, `video_editing/sequencer_scene`. API: `bpy.types.Scene`, `bpy.types.NodeTree`, `bpy.types.CompositorNodeTree`, `bpy.types.BlendData`, `bpy.types.BlendDataNodeTrees`, `bpy.types.SequencerCompositorModifierData`, `bpy.types.SpaceNodeEditor`, `bpy.ops.node`. Release notes: `sequencer.md`, `compositor.md`. Searches across the whole of `manual_5.1` and `api_5.1`: `compositing_node_group`, `CompositorNodeTree`, `node_groups`, `duplicate_compositing_node_group`, `new_compositing_node_group`, `Each Scene`, `per scene`, `data-block`, `linked`, `appended`, `Pinned`, `contains_tree`.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**This question is ambiguous and I flag it rather than choose silently.** In 5.1 a scene can take in another scene's rendered result two ways: through a **Render Layers node** whose Scene property names the other scene, or through a **video-sequencer Scene strip**. The sources answer these differently. I have taken the Render Layers reading as the one meant, because the question is on a compositor sheet and the Render Layers node is the compositor's own way of doing it. The Scene-strip reading follows underneath.

### The Render Layers reading — **no**. **Label: documented.**

The manual states it by telling you how to work around it:

> "Hint
>
> To use the compositing output from another scene rather than its "raw" render output,
> first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
> then load those images into the Compositor of the current scene using the
> Image Node."

Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

That sentence only makes sense if the node gives you the other scene's raw render and not its composited result; it names the raw output as what you get, and names a detour as the way to get the other. The rest of the page is consistent — the node's stated job is "Renders a View Layer and reads its Passes into the compositing node graph.", and its Scene property is "The scene for which to render a view layer." The API describes the node the same way: "class bpy.types.CompositorNodeRLayers(CompositorNode) / Input render passes from a scene render" (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html). And what a pass is:

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

Manual, https://docs.blender.org/manual/en/5.1/render/layers/passes.html — read 2026-07-30.

So: the first scene's render **does** cause the second scene's view layer to be rendered — the node's own description says it renders one — but what arrives is the render before compositing. The second scene's compositor is not run into the first scene's tree.

One further documented trace that indirect rendering of other scenes is a real thing in 5.1: "use_render_cache / Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)" (Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html).

### The Scene-strip reading — the sources say something, and what they say is itself ambiguous

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

The last sentence can be read two ways and the page does not settle which. Either it means the strip delivers the referenced scene's composited result (which would make the answer to this question "yes" for Scene strips), or it means the enclosing scene's own Compositing switch decides what the strip contributes. I will not guess. Whichever it is, the strip is documented as fetching a rendered result — "Scene strips are a way to insert the render output of another scene into your sequence." — with an alternative that fetches the edit instead: "Input / Input type to use for the Scene strip. / Camera: / Use the Scene's 3D camera as input. / Sequencer: / Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene)." (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30). Note there is no "Compositor" among those Input choices.

The same page records a limit on what comes across: "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

**Not covered.** Nothing in the sources states whether the referenced scene's own `use_compositing` setting is consulted at all in either mechanism. Nothing states what happens with a chain — scene A pulling scene B which pulls scene C. Nothing states whether a Render Layers node pointing at another scene honours that scene's Post Processing settings, its render engine, or its resolution. The Scene-strip sentence above is left unresolved on purpose.

**Outside the interface.** The sources say nothing about either mechanism behaving differently when Blender is driven from outside its window. Neither the Render Layers page nor the Scene strip page mentions the interface at all.

**Operating system.** The sources say nothing.

**Where I looked.** Manual: `compositing/types/input/scene/render_layers`, `compositing/types/input/scene/index`, `compositing/types/input/image`, `compositing/introduction`, `render/layers/passes`, `render/layers/introduction`, `render/output/properties/post_processing`, `video_editing/edit/montage/strips/scene`, `video_editing/sequencer_scene`, `scene_layout/scene/properties`. API: `bpy.types.CompositorNodeRLayers`, `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.ops.render`. Searches across the whole of `manual_5.1` and `api_5.1`: `Render Layers`, `another scene`, `other scene`, `different scene`, `scene strip`, `indirectly rendered`, `re-render`, `rerender`, `triggers a render`, `also be rendered`, `will be rendered`, `nested`, `recursively`, `chain of scenes`. Ruled out: the Background Scene feature (`scene_layout/scene/properties`), because it brings in another scene's *contents* — "Allows you to use a scene as a background" — not its rendered result, so it is not the mechanism the question describes.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

Three parts, answered in turn.

### (a) What it arrives as. **Label: documented.**

It arrives through a Render Layers node, as an image on a socket, plus an alpha socket, plus one socket for each enabled pass.

> "Renders a View Layer and reads its
> Passes into the compositing node graph."
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

Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

What kind of thing that image is, in the compositor's own terms:

> "Three types of data exist, all of which are stored in half precision formats:
>
> Float
>
> A signed floating-point number. Integer data is also stored as floats because no integer type
> exist.
>
> Vector
>
> A 4D vector. […]
>
> Color
>
> A 4D vector storing the Red, Green, Blue, and Alpha of the color. The color is free form and does
> not conform to a specific color space or alpha storage model […]"

Manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30.

And an image is not just a grid of pixels — it carries a position and size in the compositing space:

> "The compositor is designed in such a way as to allow compositing in an infinite compositing space.
> Consequently, images are not only represented by their size, but also by their transformation in
> that space, much like 3D objects have transformations. […] The rectangular area occupied by an image in that space as defined
> by its transformation and size is called the Domain of the image."

Same page, same address, read 2026-07-30.

One image at a time:

> "The Compositor processes one image at a time, no matter what kind of input you provide."

Manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

And what is on the Combined socket is the render before any compositing: "The render output before any compositing is applied." (manual, https://docs.blender.org/manual/en/5.1/render/layers/passes.html).

### (b) Whether it can be told apart. **Label: split — documented for the node, inferred for the data.**

**By the node: yes, documented.** The thing that brings a render into the tree is a distinct node type with a distinct name, and it carries properties saying which render it is:

> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30. In the API it is its own class — "class bpy.types.CompositorNodeRLayers(CompositorNode) / Input render passes from a scene render", with "scene / Type: Scene" (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html). Other sources of images in the same tree are different node types with different classes — the Image node ("The Image node injects any image format that is supported by Blender", manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/image.html), the Movie Clip node, the Mask node. So an inspector reading the node tree can say which node is the render.

**By the data itself: no, and this is inferred.** The compositor's data model, quoted above, has exactly three types — Float, Vector, Color — and none of them carries any mark of where it came from. The step: if the only kinds of data in the compositor are those three, and none is documented as carrying provenance, then a render's pixels once inside are the same kind of thing as any other image's pixels and cannot be distinguished from them by looking at the data. The sources do not say this in so many words; the reader supplies the step, so the label is **inferred**. The nearest thing to a marker the sources offer is the Image Info node, which reports geometry only — "Dimensions / The dimensions of the image in pixels with transformations applied. / Resolution / The width and height of the image in pixels. / Location / The position of the image in compositing space." (manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/image_info.html) — nothing about origin.

### (c) Whether it can be followed forward. **Label: documented for the pieces, inferred for the conclusion.**

**Nodes that pass along unchanged.** The Reroute node exists and is available in every node tree type:

> "A node used primarily for organization.
> Reroute looks and behaves much like a socket on other nodes in that it supports one input
> connection while allowing multiple output connections."

Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/types/layout/reroute.html — read 2026-07-30. That page sits under "Common Nodes / These nodes are shared across different node tree types" (manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/types/index.html), and the compositor has its own Layout Nodes section: "These are nodes which help you control the layout and connectivity of nodes within the Compositor." (manual, https://docs.blender.org/manual/en/5.1/compositing/types/layout/index.html).

Note that one sentence answers both halves of the question at once: **one input, many outputs** — that is both "passes along unchanged" and "splits into more than one".

A muted node also passes along unchanged:

> "Muting a node removes its contribution to the node tree,
> and makes all links pass through it without change.
> Links will appear red as an indicator of passing through the muted node."

Manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html — read 2026-07-30. The route it takes through a muted node is itself exposed: "internal_links / Internal input-to-output connections for muting (default None, readonly)" (Python API, https://docs.blender.org/api/5.1/bpy.types.Node.html).

A link can be muted too, and is then documented as absent: "A muted link acts as though it's no longer there" (manual, editing page above), with "is_muted / Link is muted and can be ignored (default False)" (Python API, https://docs.blender.org/api/5.1/bpy.types.NodeLink.html).

**The machinery for following.** Every link states both of its ends, and every socket lists its links:

> "class bpy.types.NodeLink(bpy_struct)
>
> Link between nodes in a node tree
>
> from_node
>
> (readonly)
>
> Type:
>
> Node"

with `from_socket`, `to_node` and `to_socket` alongside it. Python API, https://docs.blender.org/api/5.1/bpy.types.NodeLink.html — read 2026-07-30.

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
> Takes O(len(nodetree.links)) time."

Python API, https://docs.blender.org/api/5.1/bpy.types.NodeSocket.html — read 2026-07-30. The same page has "is_linked / True if the socket is connected (default False, readonly)", "is_output / True if the socket is an output, otherwise input (default False, readonly)" and "node / Node owning this socket (readonly)".

**Where a trace would have to descend.** Node groups nest: the Group Input node "provides access to input sockets of a node group. It acts as the entry point for data passed into the group from the outside" and the Group Output node "determines which values are passed out of the node group to the parent node tree (or to the renderer, in the case of the Compositor)" (manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/group_input.html and https://docs.blender.org/manual/en/5.1/compositing/types/output/group_output.html). Nesting is bounded — "contains_tree(sub_tree) / Check if the node tree contains another. Used to avoid creating recursive node groups." (Python API, https://docs.blender.org/api/5.1/bpy.types.NodeTree.html) — so a trace cannot loop forever inside groups.

**Where a trace would go dark.** Some nodes choose between inputs at evaluation time rather than by wiring. The Switch node: "When it is unchecked, the first input labeled 'Off' is passed to the output. When checked, the second input labeled 'On' is passed to the output." with the tip "Switch state may be animated by adding a keyframe. This makes the Switch node useful for bypassing nodes which are not wanted during part of a sequence." (manual, https://docs.blender.org/manual/en/5.1/compositing/types/utilities/switch.html). Index Switch and Menu Switch nodes are also listed in the 5.1 compositor's node list, and the release notes record "The **Index Switch** node is now supported." (release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/). Following a render past one of those means knowing the switch's value at that frame, not just reading the wiring.

**The conclusion is inferred.** The sources document reroute behaviour, muting behaviour, and a complete set of link and socket relations. They nowhere say "you can trace an image forward through the tree". Assembling those pieces into "yes, it can be followed" is the reader's step, so: **inferred**.

### And where it ends up

> "In the Compositor, this node defines the image result that is sent to the renderer or to
> image output after rendering."
>
> "In the Compositor, only the first image input is used as the final render result.
> Any additional sockets will be ignored."

Manual, Group Output, https://docs.blender.org/manual/en/5.1/compositing/types/output/group_output.html — read 2026-07-30. Also on that page: "If multiple Group Output nodes are present in a node tree, only the active one is used. All others will show a 'Unused Output' warning. The active output can be changed in the header of the node."

**Not covered.** The sources do not say whether the render arriving at a *scene's* compositing node group could also come in through that group's Group Input socket rather than through a Render Layers node — the Group Input page describes group inputs in general ("The default image input for the node group, available when the group is created. It can be used to feed images or render layers into the group.") and never says what, if anything, feeds a scene's top-level compositing group from outside. I searched for this specifically and found nothing that settles it. The sources also do not say whether a Render Layers node's data is tagged internally, what happens when two Render Layers nodes name different scenes in one tree, or whether the Index Switch and Menu Switch nodes behave like the Switch node for tracing purposes. Bundle nodes (Combine Bundle, Separate Bundle, Join Bundle) are listed under "Common Nodes … shared across different node tree types" but appear nowhere in the compositing chapter's own node list, so I cannot say whether they are available in the compositor.

**Outside the interface.** The sources say nothing about any of this differing outside the window. The node-tree relations quoted are properties of data-blocks and links, which are in the file. One documented item here *is* interface-bound: the "active" Viewer node is chosen by selection in the editor — "Only the active Viewer node (with the icon in the header) will be displayed in the backdrop or Image Editor." (manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/viewer.html) — and the Group Output's active flag is likewise "changed in the header of the node". Whether "active" is stored in the file or derived from the editor is not stated; that gap matters for anything reading a file without opening a window, and I could not close it.

**Operating system.** The sources say nothing.

**Where I looked.** Manual: `compositing/compositor_system`, `compositing/introduction`, `compositing/types/input/scene/render_layers`, `compositing/types/input/image`, `compositing/types/input/image_info`, `compositing/types/input/group_input`, `compositing/types/output/group_output`, `compositing/types/output/viewer`, `compositing/types/output/enable_output`, `compositing/types/output/file_output`, `compositing/types/utilities/switch`, `compositing/types/utilities/split`, `compositing/types/groups`, `compositing/types/layout/index`, `compositing/index`, `interface/controls/nodes/types/index`, `interface/controls/nodes/types/layout/index`, `interface/controls/nodes/types/layout/reroute`, `interface/controls/nodes/editing`, `interface/controls/nodes/node_editors`, `interface/controls/nodes/groups`, `render/layers/passes`, `render/layers/introduction`. API: `bpy.types.NodeLink`, `bpy.types.NodeSocket`, `bpy.types.Node`, `bpy.types.NodeTree`, `bpy.types.CompositorNodeRLayers`, `bpy.types.CompositorNodeTree`. Release notes: `compositor.md`. Searches across the whole of `manual_5.1` and `api_5.1`: `reroute`, `Reroute`, `mute`, `muting`, `internal_links`, `from_node`, `to_node`, `is_linked`, `Render Layers`, `bundle`, `Group Input`, `Group Output`, `provenance`, `Split`, `Switch`, `pass through`, `unchanged`, `domain`, `Domain Priority`.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Label: mixed.** One thing is documented outright. Three more are strongly implied but the sources never state the pixel arithmetic, so they are inferred and marked as such. And one candidate the reader might expect — the compositor — is documented as *not* deciding it.

### The one that is documented: Render Region with Crop to Render Region

> "Render Region
>
> Renders just a portion of the view instead of the entire frame.
> See the Render Region
> documentation to see how to define the size of the render region.
>
> Crop to Render Region
>
> Crops the rendered image to the size of the render region,
> instead of rendering a transparent background around it."

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30.

The picture on that page, `_images/render_output_properties_format_panel.png`, which I opened, shows a "Format" panel with Resolution X 1920 px, Y 1080 px, % 100%, Aspect X 1.000, Y 1.000, an unticked "Render Region" checkbox, a greyed-out "Crop to Render Region" checkbox beneath it, and Frame Rate 24 fps. **Label: documented.** What is visible is that Crop to Render Region is subordinate to Render Region — greyed out while the latter is off. Nothing more is claimed from the picture.

In the API:

> "use_border
>
> Render a user-defined render region, within the frame size (default False)"

> "use_crop_to_border
>
> Crop the rendered frame to the defined render region size (default False)"

Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. The region itself is four numbers on the same page: `border_min_x`, `border_max_x`, `border_min_y`, `border_max_y`, each "(in [0, 1]" with defaults 0.0 and 1.0.

That the region reaches the file at all, and is not a viewport-only thing, is stated twice:

> "Render regions only apply to the viewport when using Cycles, not when using EEVEE.
> However, they always affect the final render."

Manual, https://docs.blender.org/manual/en/5.1/editors/3dview/navigate/regions.html — read 2026-07-30. The same page: "If you define one while in Camera View, it will apply not just to the viewport, but also to the final render." And from the sidebar page: "Note that if you're viewing the scene through the active camera, this option has no effect – in this case, you instead need to use the checkbox Output Properties ‣ Format ‣ Render Region in the Properties editor. This will affect not just the viewport, but also the final render." (manual, https://docs.blender.org/manual/en/5.1/editors/3dview/sidebar.html).

**One step remains, and I flag it.** Both sentences say the *rendered image* or *rendered frame* is cropped. Neither uses the words "written" or "saved". Going from "the rendered frame is cropped to the region size" to "the file on disk is that size" is one short step the reader supplies. So: the crop is **documented**, the written file size is **inferred**.

### The compositor does not decide it

> "Output Nodes
>
> The operation domain of output nodes like the Viewer node is
> a domain with an identity transformation and the same size as the final compositor output. For
> viewport compositing, that size would be the viewport size, and for
> final render compositing, that size would be the scene render size."

Manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html — read 2026-07-30.

Corroborated from an unrelated page:

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/stereoscopy/usage.html — read 2026-07-30. (The same sentence appears at https://docs.blender.org/manual/en/5.1/compositing/types/utilities/switch_stereo_view.html.)

And the Image Editor's own overlay keeps the two numbers apart: "Render Size: The resolution of the final render output. This is defined in the Output Properties." / "Image Size: The resolution of the image currently displayed in the Viewer node." (manual, https://docs.blender.org/manual/en/5.1/editors/image/overlays.html).

So nodes that resize images inside the tree — Crop ("The Crop node crops an input image to a selected region by either making the cropped area transparent or by resizing the input image.", manual, https://docs.blender.org/manual/en/5.1/compositing/types/transform/crop.html; in the API, "Crops image to a smaller region, either making the cropped area transparent or resizing the image", https://docs.blender.org/api/5.1/bpy.types.CompositorNodeCrop.html), Scale, Transform — change the image's domain within the compositing space, and the output node then resolves that against the scene render size. **Caution on how far that sentence reaches:** it says "output nodes like the Viewer node", giving the Viewer as its example. It never enumerates which nodes are in that class, and it never names the File Output node. So whether a File Output node's written files are also fixed to the scene render size is **not settled by these words**.

**The File Output node has no size control at all.** Its documented properties are Directory, File Name, Media Type, Node Format, Images/Layers, Override Node Format, Item Format, Output Paths (manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html). In the API its own properties are `active_item_index`, `directory`, `file_name`, `file_output_items`, `format`, `save_as_render` (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html) — none dimensional. The one per-item property that reads as if it might be is not:

> "vector_socket_dimensions
>
> Dimensions of the vector socket (in [2, 4], default 0)"

Python API, https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html — read 2026-07-30. That is how many channels a vector socket has, not pixels.

### Three things that are implied but never stated in pixels

**Stereoscopy layout. Label: inferred.** The sources describe the layouts but never state the arithmetic.

> "Side-by-Side
>
> Render images for left and right eye side-by-side.
> There is an option to support Cross-Eye glasses."
>
> "Top-Bottom
>
> Render images for left and right eye one above another."

> "The Top-Bottom compressed will discard half of your vertical resolution data."

> "For the Stereo 3D formats the only lossless options are
> Top-Bottom and Side-by-Side without the Squeezed Frame option."

All manual, https://docs.blender.org/manual/en/5.1/render/output/properties/stereoscopy/usage.html — read 2026-07-30. In the API, `Stereo3dFormat.display_mode` offers `ANAGLYPH`, `INTERLACE`, `SIDEBYSIDE`, `TOPBOTTOM`, and `use_squeezed_frame` is "Combine both views in a squeezed image (default False)" (https://docs.blender.org/api/5.1/bpy.types.Stereo3dFormat.html). **The step:** if unsqueezed Top-Bottom and Side-by-Side are lossless while squeezed ones discard half the data, the unsqueezed layouts must be putting both views on a larger canvas. Nothing in the sources says "the width doubles". That reasoning is mine.

The sources *do* state outright that multi-view changes the number of files: "By default the system saves each view as an individual file, thus generating as many files as views to be rendered." (same page). The views format enum is `INDIVIDUAL` / `STEREO_3D` / `MULTIVIEW` with "Individual files for each view with the prefix as defined by the scene views." / "Single file with an encoded stereo pair." / "Single file with all the views."

**Rendering a different scene. Label: inferred, and nearly documented.** Which scene is rendered decides whose Resolution values apply. "The render and output settings of the Sequencer Scene are the same as for regular scenes." (manual, https://docs.blender.org/manual/en/5.1/video_editing/sequencer_scene.html), with "Render Sequencer Image Alt-F12 / If a Sequencer Scene exists that differs from the active scene, render that scene instead." (manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html) and, from script, "scene (str) – Scene, Scene to render, current scene if not specified" and "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene" (Python API, https://docs.blender.org/api/5.1/bpy.ops.render.html). **The step:** if a different scene is rendered and each scene has its own resolution settings, that scene's numbers govern. Not stated as such.

**A render engine registered inside the file. Label: inferred, and speculative.** The API lets an engine declare a result of any size:

> "begin_result(x, y, w, h, *, layer='', view='')
>
> Create render result to write linear floating-point render layers and passes"

with "w (int) – Width, (in [0, inf])" and "h (int) – Height, (in [0, inf])". Python API, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30. The documented example on the same page takes those from the scene by convention, not by force:

```
        scale = scene.render.resolution_percentage / 100.0
        self.size_x = int(scene.render.resolution_x * scale)
        self.size_y = int(scene.render.resolution_y * scale)
```
```
        result = self.begin_result(0, 0, self.size_x, self.size_y)
```

**The step:** a `.blend` file can carry a script, and a script can register a render engine, and that engine can pass different numbers. Nothing in the sources says this happens or is supported. I record it because it is the only documented route by which something *inside a file* could set a size the scene did not.

### One thing where the restriction is documented and the response is not

> "Known Limitations
>
> Video Output Size
>
> Some codecs impose output size restrictions.
> For example, H.264 requires both width and height to be divisible by 2."

Manual, https://docs.blender.org/manual/en/5.1/files/media/video_formats.html — read 2026-07-30. And on the Output page: "some codecs do not work with arbitrary dimensions. So, try to stick with common dimensions or research the limitations of the codec you are trying to use." (manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html). **Whether Blender quietly adjusts the size to fit, or refuses, is not in the sources.** Do not read either behaviour into these words.

### The only place the sources do the arithmetic

There is exactly one passage in the 5.1 sources that defines the effective render size numerically, and it counts only the two things the question already names:

> "resolution_x / resolution_y:
>
> The x and y resolution of the rendered image.
> This factors in the resolution scale as well, so if the scene resolution is
> 1000x600 and the scale is 50%, then resolution_x and resolution_y
> will be 500 and 300, respectively."

Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30 (these are the variables available in render output path templates, which are supported "for the render output path in Scene properties and the output paths in the compositor's File Output node"). No border, no stereo layout, nothing else. **Label: documented** for what the variables hold; that this is also the written file's size is **inferred**.

**Observed, from Cycles' own file.** Cycles' own code computes render size the same way, in `Blender Sources/cycles_5.1/ui.py` lines 399–401:

```
            render_scale = scene.render.resolution_percentage / 100.0
            render_size_x = int(scene.render.resolution_x * render_scale)
            render_size_y = int(scene.render.resolution_y * render_scale)
```

**Label: observed**, with a firm limit on what it shows. Reading the surrounding lines, this sits inside a panel-drawing function and feeds a warning label — "May work poorly with render tiling". It is Cycles working out a render size for a user hint, not Cycles deciding what gets written. It is evidence that Cycles' own Python treats resolution × percentage as the render size in that context, and nothing more. A separate search of the whole Cycles add-on for `border` and `crop` found no handling of either in its Python at all.

### Ruled out, each for a stated reason

- **Pixel aspect ratio** — changes pixel shape, not pixel count. "Older televisions may have non-square pixels, so this can be used to control the shape of the pixels along the respective axis." (manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html); "Horizontal aspect ratio - for anamorphic or non-square pixel output (in [1, 200], default 1.0)" (Python API, RenderSettings).
- **Pixel Density / PPM** — "The pixel-density is meta-data which doesn't impact the quality of the image." (manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html).
- **Preview Pixel Size** — "Pixel size for viewport rendering (default 'AUTO')" (Python API, RenderSettings); documented under a "Viewport" heading in the performance pages.
- **EEVEE Overscan** — "Percentage of render size to add as overscan to the internal render buffers (in [0, 50], default 3.0)" (Python API, https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html). The word is "internal".
- **Simplify** — its texture limit scales *textures*: "Automatically scales textures down so that they are no larger than the values chosen." (manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/simplify.html). Nothing about output.
- **Metadata Burn into Image** — "Add metadata as text to the render." with a Font Size; nothing about enlarging the frame (manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html).
- **Proxy Render Size** — "Display preview using full resolution or different proxy resolutions (default 'SCENE')" (Python API, https://docs.blender.org/api/5.1/bpy.types.SpaceSequenceEditor.html); it lives on an editor, not on the scene.
- **Media Type** — Image / Multi-Layer EXR / Video; nothing dimensional.
- **VSE strip fitting** — strips are fitted *into* the frame: "Resolution are scaled to fit inside the render area." (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/movie.html). The one operator that goes the other way, "Set Render Size / Sets the render resolution and aspect to match the strip's resolution." (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/editing.html), *changes Resolution X/Y* rather than bypassing them.
- **Autosplit Output** — changes file count, not size: "This will automatically split the output into multiple files after the first file is 2GiB in size." (manual, Output page).
- **Command-line arguments** — there is no argument that sets resolution, percentage or render region. The flags on the arguments page are `-o/--render-output`, `-F/--render-format`, `-x/--use-extension`, `-S/--scene`, `-E/--engine`, `-t/--threads`, and the frame-range ones. The only "resolution" hit on that page is `--no-native-pixels`, a window flag: "Do not use native pixel size, for high resolution displays (MacBook Retina)."

**Not covered.** Whether crop-to-region and stereo layouts *combine*, and in what order. What size a File Output node writes when its input's domain differs from the scene render size — the sources never name that node in the output-domain rule. Whether the render region is applied before or after the compositor. What Blender does when a codec's size restriction is violated. Whether any of the inferred size changes shows up in the render output path template's `resolution_x`/`resolution_y` variables, or whether those always report the plain scene numbers. Every one of these is a short hand test.

**Outside the interface.** Mostly not stated, with one relevant documented divergence *nearby* — the manual says one render setting is skipped when Blender is driven from outside:

> "Render Single Layer
>
> Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line."

Manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — read 2026-07-30. That is about which layer renders, not about size. **No statement anywhere in the 5.1 sources says the Render Region, Crop to Render Region, stereo Views Format, or compositor domain behaviour differs in background mode.** That is an absence, not a denial — and given that a divergence of exactly this kind is documented for a neighbouring setting, it is worth testing rather than assuming. The command-line page does warn that argument order silently changes where output lands (quoted in answer 2), and notes the practical point that "we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

**Operating system.** Nothing in the sources makes output pixel size depend on the operating system. Two OS-specific statements exist nearby and neither is about size: "TDR on Windows / Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." (manual, https://docs.blender.org/manual/en/5.1/compositing/limitations.html) and "Headless rendering is not supported on headless Windows systems." (manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html). That second one is worth noting for anyone driving Blender from outside on Windows, though it is about EEVEE and not about size.

**Where I looked.** (This answer rests on a helper's sweep, every quote of which I re-opened and confirmed myself.) Manual pages opened: `render/output/properties/format`, `render/output/properties/output`, `render/output/properties/metadata`, `render/output/properties/stereoscopy/usage`, `render/output/properties/stereoscopy/introduction`, `render/layers/view_layer`, `render/layers/passes`, `render/cycles/render_settings/simplify`, `render/cycles/render_settings/performance`, `render/eevee/render_settings/film`, `render/eevee/render_settings/performance`, `render/eevee/limitations/limitations`, `editors/3dview/navigate/regions`, `editors/3dview/sidebar`, `editors/image/navigating`, `editors/image/overlays`, `editors/video_sequencer/preview/sidebar`, `compositing/compositor_system`, `compositing/types/output/*` (all four), `compositing/types/transform/crop`, `compositing/types/transform/scale`, `compositing/types/transform/transform`, `compositing/types/transform/translate`, `compositing/types/transform/rotate`, `compositing/types/input/mask`, `compositing/types/utilities/switch_stereo_view`, `files/media/video_formats`, `files/file_paths`, `video_editing/sequencer_scene`, `video_editing/edit/montage/editing`, `video_editing/edit/montage/strips/movie`, `video_editing/edit/montage/strips/image`, `video_editing/edit/montage/strips/strip_properties`, `advanced/command_line/arguments`, `advanced/command_line/render`, `interface/window_system/topbar`. Pictures opened: `render_output_properties_format_panel.png`, `render_output_properties_output_panel.png`. API pages opened: `bpy.types.RenderSettings` (every property name enumerated), `bpy.types.Scene` (likewise), `bpy.types.ImageFormatSettings` (likewise), `bpy.types.Stereo3dFormat`, `bpy.types.RenderEngine`, `bpy.types.RenderResult`, `bpy.types.CompositorNodeCrop`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeCompositorFileOutputItem`, `bpy.types.SceneEEVEE`, `bpy.types.SpaceSequenceEditor`, `bpy.ops.render`, `bpy.ops.image`, `bpy.ops.view3d`, `bpy.ops.sequencer`, plus the `views_format_multiview_items` enum list. Cycles files searched in full for `resolution`, `border`, `crop`: `properties.py`, `ui.py`, `engine.py`, `operators.py`, `camera.py`, `version_update.py`, `__init__.py`, and the `source/` tree. Release notes read in full: `compositor.md`, `pipeline_io.md`, `rendering.md`, plus `bugfixes.md` scanned. Terms searched across all four 5.1 source folders: `use_crop_to_border`, `crop_to_border`, `Crop to Render Region`, `use_border`, `border_max_x`, `border_min_x`, `border_max_y`, `border_min_y`, `Render Region`, `resolution_x`, `resolution_y`, `resolution_percentage`, `operation domain`, `image domain`, `compositing space`, `File Output`, `Side-by-Side`, `Top-Bottom`, `SIDEBYSIDE`, `TOPBOTTOM`, `Squeezed Frame`, `squeezed`, `Views Format`, `Multi-View`, `use_multiview`, `sequencer_scene`, `Proxy Render Size`, `proxy_render_size`, `Fit Method`, `Scale to Fit`, `Stretch to Fill`, `Scale to Fill`, `output resolution`, `final resolution`, `render resolution`, `size of the rendered`, `rendered image size`, `output image size`, `final image size`, `scene render size`, `render size`, `frame size`, `image dimensions`, `output dimensions`, `downscal`, `upscal`, `resampl`, `resize`, `rescal`, `number of pixels`, `in pixels`, `pixel dimensions`, `pixel size`, `display window`, `data window`, `overscan`, `aspect`, `divisible`, `multiple of`, `must be even`, `render_border`, `clear_render_border`, `generated_width`, `generated_height`, `background mode`, `--background`, `headless`, `command line`, `Windows`, `macOS`, `Linux`, plus a whole-property enumeration of `RenderSettings`, `Scene` and `ImageFormatSettings` and a whole-flag enumeration of the command-line arguments page.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Label: documented**, with one unresolved tension between two source statements that I quote rather than reconcile.

### Yes — two independent switches, in the same panel

The Post Processing panel carries "Compositing" and "Sequencer" as separate checkboxes (see the panel picture described in answer 1). Turning Compositing off and leaving Sequencer on is the setting the question describes.

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

**The tension, stated plainly.** The manual says the sequencer renders *instead of* the camera view. The API says the sequencer processes "the render (and composited) result" — which describes the sequencer as coming *after* the render and the compositor, not in place of them. Both sentences are in the 5.1 sources and I am not able to reconcile them from the sources alone. What both agree on is that the two switches are independent and that turning Compositing off means the compositor is not used, so the "bypass" the question asks about is available either way.

There is also a documented condition on the sequencer switch that is easy to miss: "if sequencer strips exist". A scene with the Sequencer box ticked and no strips is not described by any source I found.

**Observed, from Cycles' own file.** The two switches are drawn as one pair, in `Blender Sources/cycles_5.1/ui.py` lines 1124–1126, inside `class CYCLES_PT_post_processing` (full quote in answer 1):

```
        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

**Label: observed.** What the file shows is that both properties are drawn together under a heading "Pipeline". Nothing about what a render does with them is claimed from this.

### What it changes about what gets written

Four documented consequences, and one non-consequence.

**The picture changes source.** The written frames come from the edit rather than from the camera — that is the first sentence quoted above.

**Multi-layer EXR is out.**

> "Note
>
> The Video Sequencer does not support multi-layer EXR files.
> To render to a video format you will have to skip the next three steps and
> instead use an Image Input node
> in the Compositor."

Manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30. (Read in context: this note is advising a workflow, and says the Sequencer cannot take multi-layer EXR *in*. Whether it also constrains what can be written out is a step past the words, and I do not take it.)

**Passes are out, for anything arriving as a Scene strip.**

> "Limitations
>
> Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

Manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

**The size does not change.** Strips are fitted to the scene's resolution rather than governing it — the Image Transform operators are all described in terms of "the project's Resolution", e.g. "Adjusts the strips Scale Transforms so the visual contents of the strip to fit exactly within the project's Resolution while maintaining the original aspect ratio." (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/editing.html — read 2026-07-30). Going the other way is an explicit, manual action: "Set Render Size / Sets the render resolution and aspect to match the strip's resolution." (same page), which changes the scene's own Resolution values rather than overriding them.

**Everything else about output is the same.** The Sequencer Scene page says so for the 5.1 arrangement where the edit lives in its own scene:

> "The render and output settings of the Sequencer Scene are the same as for regular scenes.
> They can be found in the Output tab in the Properties editor. Note that these settings
> follow the active scene in the window by default (unless there is a Scene pinned to the
> properties editor)."

Manual, https://docs.blender.org/manual/en/5.1/video_editing/sequencer_scene.html — read 2026-07-30.

### The 5.1 wrinkle worth knowing: which scene is "the" scene

In 5.1 the edit is documented as living in a scene of its own:

> "The video sequencer uses a separate scene data-block that contains the edit. This is the Sequencer Scene."

Same page, same address, read 2026-07-30. And rendering it is a separate command:

> "When the active scene is different from the Sequencer Scene in the current workspace,
> the 'Render' menu will include separate options to render the Sequencer Scene (and not the active Scene)."

Same page. The menu entries: "Render Sequencer Image Alt-F12 / If a Sequencer Scene exists that differs from the active scene, render that scene instead." and "Render Sequencer Animation Ctrl-Alt-F12 / Render the animation of the Sequencer Scene." (manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30). From script: "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene (optional)" on `bpy.ops.render.render` (Python API, https://docs.blender.org/api/5.1/bpy.ops.render.html).

This matters for what gets written because it decides *whose* output settings apply. A related setting exists: "use_sequencer_override_scene_strip / Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip (default False)" (Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html).

**Not covered.** The sources do not say what is written when Sequencer is on and the scene has no strips. They do not say whether the compositor still runs when Compositing is on and Sequencer is on together — the tension quoted above is exactly the gap. They do not say whether turning Compositing off changes the render result stored in memory or only what reaches the file. They say nothing about how the Sequencer Scene is chosen when Blender is started without a window, which is the case that matters most for driving this from outside.

**Outside the interface.** Partly answered and partly not. The command-line documentation gives `-S, --scene <name>` — "Set the active scene <name> for rendering." (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30) — and the operator has `use_sequencer_scene` and `scene` arguments. But the Sequencer Scene is documented as belonging to a *workspace* ("the Sequencer Scene in the current workspace"), and there is no documented command-line argument that selects one. Whether a background run can render the Sequencer Scene, and which scene it picks if it cannot, is **not in the sources**. The `use_compositing` and `use_sequencer` properties themselves carry no interface-only caveat, unlike `use_single_layer` — see the same discussion in answer 1.

**Operating system.** The sources say nothing about any of this differing by operating system.

**Where I looked.** Manual: `render/output/properties/post_processing`, `render/output/properties/output`, `render/output/animation`, `render/output/introduction`, `video_editing/introduction`, `video_editing/index`, `video_editing/sequencer_scene`, `video_editing/setup/app_template`, `video_editing/edit/montage/editing`, `video_editing/edit/montage/strips/scene`, `video_editing/edit/montage/strips/strip_properties`, `video_editing/edit/montage/strips/movie`, `video_editing/edit/montage/strips/image`, `video_editing/edit/montage/strip_modifiers`, `editors/image/introduction`, `interface/window_system/topbar`, `interface/window_system/workspaces`, `advanced/command_line/arguments`, `advanced/command_line/render`. Pictures opened: `render_output_properties_post-processing_panel.png`. API: `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.SpaceImageEditor`, `bpy.types.SequencerCompositorModifierData`, `bpy.ops.render`, `bpy.ops.scene`. Cycles files: `ui.py`. Release notes: `sequencer.md`, `compositor.md`, `pipeline_io.md`. Searches across the whole of `manual_5.1` and `api_5.1`: `use_sequencer`, `sequencer_scene`, `Sequencer Scene`, `scene strip`, `Post Processing`, `Pipeline`, `bypass`, `instead of the view`, `resolution` within the video-editing chapter, `Set Render Size`, `multi-layer EXR`, `Render Passes`.

---

## Things a person with Blender 5.1 open could settle by hand in a few minutes

**Most of this list has been run, and the results are settled in the first-pass document**, `Preparation/Blender Research/Findings/5.1/The compositor.md`, where the observed findings and the machine's raw output are recorded under the questions they answer. The set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`.

Settled there, by number below: **1** (the Compositing switch behaves the same from the command line), **2** (a compositor with no Render Layers node still runs, and its result is what lands), **3** (an Enable Output node set off leaves a black frame rather than the camera view, and a file is still written), **4** (muting a File Output node stops it writing), **5** (a blank output path fails with `Couldn't create directory for file 0001.png` and writes nothing), **6** (a Render Layers node pointing at another scene brings back that scene's raw render, and that scene's compositor does no work at all), **7** (a Scene strip follows the strip's own scene's Compositing switch, not the rendering scene's), **8** (a Scale node cannot change the scene's own written size), **9** (a File Output node writes at its input's size — 122×90 from a Scale set to 120×90, against a 400×200 scene), **10** and **13** (Crop to Render Region halves both dimensions, and does so identically in a command-line render), **11** (unsqueezed stereo doubles one axis), **12** (`is_active_output` on a Group Output node reads `True` and is a property of the node, so the active flag is in the file), and **14** (the output-path template reports the scene's Resolution values, not the cropped size).

**Seven window tests have been run since, and are settled in the same first-pass document.** None of them was an item on this list, but they bear on several of the answers above, and each is recorded under the question it answers, from `Audits/Blender Research/The compositor/Hand tests.md`, entries 15 to 21. In short: the Post Processing panel holds exactly the two switches and the Dither slider, both switches on by default, with the Compositing tooltip naming an assigned compositing node group as its condition; the Output panel has the Media Type dropdown its manual text describes and no checkbox in its header; the Compositor editor's header has no "Use Nodes" checkbox but a node-tree sub-type dropdown, a data-block selector and a New button; Crop to Render Region is greyed out until Render Region is ticked, and both start off; hovering a link in 5.1 shows the socket's name, "Image", and nothing about where the image came from; the Render menu in a fresh file holds Render Image, Render Animation, Render Audio, View Render, View Animation and Lock Interface, with no entry naming a Sequencer Scene; and a second Group Output node carries a warning above its header reading "Unused Output", the active one being the last selected.

That last one goes with item **12** above: the `is_active_output` reading settles that the active flag is in the file, and the "Unused Output" warning is how the same thing shows on screen.

Two are still open and need a Blender window: **15**, what Blender does with an odd width in H.264, and **16**, where the render region sits relative to the compositor. Neither was run.

**A second machine, 2026-08-06.** Every test named above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are recorded beside the Mac's in the first-pass document, never merged with them. All of them agreed with the Mac, including item 9's odd 122×90. **One exception:** hovering a link could not be settled on that machine at all, because its display is shared with other work and the pointer would not stay put, so that reading still rests on the Mac alone. Items 15 and 16 stay open — neither was run on either machine. One new test was added that only the second machine could carry: whether a compositor tree gives the same picture on the graphics card as on the processor. At 320×240 it does; at 9000 pixels wide the card writes a black frame, which is 5.1's own oddity and now seen on two different makes of card. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

One correction to item 8's premise, recorded because a later reader will trip on it: a Scale node set to Absolute is wired differently in 5.1 than in 4.5. In 4.5 the node has a `space` property; from 5.0 that moved onto an input socket called Type, whose value is the word `Absolute`. A test written against the 4.5 wording fails outright in 5.1 with `AttributeError: 'CompositorNodeScale' object has no attribute 'space'`.

The original list follows unchanged.

1. **Does the Compositing switch behave the same from the command line?** Make a file with two scenes' worth of nothing: one cube, a compositor node group whose Group Output is fed by a Render Layers node through a strongly visible node (a Hue Saturation node with Hue set to 0.0 and Saturation 0.0, so the result is obviously grey). Save. Render one frame from the window with Output ▸ Post Processing ▸ Compositing **on**, then again with it **off**, saving to two different paths. Then run `blender -b file.blend -o /tmp/cli_on_ -F PNG -f 1` with the switch saved on, and again with a copy of the file saved with it off. Compare the four images. Look for: whether the two command-line images differ from each other in the same way the two window images do.

2. **Does the compositor run with no Render Layers node?** In a scene with Compositing on, delete the Render Layers node, leaving only a Group Output fed by an RGB node set to pure red. Render one frame. Look at: whether the saved file is red (compositor ran) or shows the camera view (it did not).

3. **Does an Enable Output node on the Group Output stop the compositor?** Same setup as 2, but insert an Enable Output node between the RGB node and the Group Output, with its Enable input unticked. Render one frame. Look at: whether a file is written at all, whether it is the camera view, and whether the status bar or console reports anything.

4. **Does muting a File Output node stop it writing?** Add a File Output node with Directory `/tmp/mutetest/` fed from the Render Layers Image socket. Render one frame. Then select the node, press M to mute it, and render again to the same directory with Overwrite off. Look at: whether a second file appears in `/tmp/mutetest/`.

5. **Does a blank render output path stop frames being written?** Clear Properties ▸ Output ▸ Output ▸ File Path entirely. Render an animation of two frames (Ctrl-F12). Look at: whether files appear anywhere, and what the console says. Repeat from the command line with `blender -b file.blend -a` and no `-o`.

6. **Does a Render Layers node pointing at another scene run that scene's compositor?** Two scenes. In scene B, put a red cube and a compositor node group that turns everything blue (Hue Saturation, or an RGB node straight into Group Output), with B's own Compositing switch **on**. In scene A, add a Render Layers node with Scene set to B, wired to A's Group Output. Render scene A. Look at: whether the result is red (B's raw render — B's compositor did not run) or blue (it did).

7. **The same question for a Scene strip.** Same two scenes. In scene A, delete the compositor wiring and instead add a Scene strip referencing B in A's video sequencer, with A's Sequencer switch on. Render scene A with A's Compositing switch on, then again with it off. Look at: whether the strip shows red or blue in each case. This is the sentence in the Post Processing docs that I could not resolve.

8. **Does the compositor change the written pixel size?** With the scene at 1920×1080 and 100%, put a Scale node set to Absolute 640×360 between the Render Layers node and the Group Output. Render one frame. Look at: the pixel dimensions of the written file (in a file browser's info panel, or by reopening it in Blender's Image Editor and reading the size in the sidebar).

9. **Does the same hold for a File Output node?** Same setup, but route the scaled image into a File Output node instead. Render one frame. Look at: the pixel dimensions of the file that node wrote, compared with the main render.

10. **Does Crop to Render Region change the written size?** Set a Render Region in camera view covering roughly the middle quarter of the frame, tick Crop to Render Region in Properties ▸ Output ▸ Format, and render one frame. Look at: the pixel dimensions of the written file against the scene's Resolution values.

11. **Does stereoscopy change the written size?** Enable Stereoscopy in Output properties, set the Views Format to Stereo 3D and the Stereo Mode to Side-by-Side with Squeezed Frame off, and render one frame. Look at: the width of the written file against the scene's Resolution X.

12. **Is the Group Output's "active" flag stored in the file?** Make a compositor tree with two Group Output nodes, set the second one active in its header, save, quit, reopen, and look at which one shows the active marker. Then render from the command line and see which one's input reaches the file.

13. **Does Crop to Render Region survive a command-line render?** Set up test 10, save the file, and run `blender -b file.blend -o /tmp/crop_ -F PNG -f 1`. Look at: the pixel dimensions of the written file, compared with what the same file produced from the window. This matters because the manual documents that Blender *does* skip at least one render setting outside the window (Render Single Layer) — so the safe assumption is neither yes nor no.

14. **Does the render output path template report the cropped size?** With test 10's crop active, set the output path to `/tmp/{resolution_x}x{resolution_y}_` and render one frame. Look at: the numbers in the resulting filename, against the actual pixel size of the file.

15. **What does Blender do with an odd width in H.264?** Set Resolution X to 1921, Y to 1080, Media Type to Video, container and codec to MPEG-4 / H.264, and render two frames. Look at: whether a file appears, what its pixel dimensions are, and what the console says. The manual states the restriction exists but not what Blender does about it.

16. **Where does the render region sit relative to the compositor?** With test 10's crop active, add a compositor tree that puts a full-frame coloured border around the image (an Alpha Over of the render onto a plain RGB node). Render one frame. Look at: whether the border fills the cropped file edge to edge, or whether the crop appears to have happened after the compositing.

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 5.1

# The compositor — Blender 5.0 — second pass

**Sources consulted, and when.** Blender 5.0 manual (including its `_images` screenshots), Blender 5.0 Python API documentation, Blender 5.0 release notes, and the Cycles add-on's own files — all as supplied in `Blender Sources/` (`manual_5.0`, `api_5.0`, `release_notes_5.0`, `cycles_5.0`). Snapshot date, and therefore my date read for everything below: **2026-07-30**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 1, 2, 4, 5, 6, 7. Question 3 was not assigned to me and is not answered.

---

## 1. What decides whether a file's compositor runs at all?

**Reading of the question.** I take "a file's compositor" to mean the compositing node setup that runs when that file is rendered. In 5.0 the compositing node tree is a data-block that is *pointed at* by a scene, so the switches below are per-scene.

**Answer.** Four separate things. All four are documented on their own. *(Citation check, 2026-08-03: this line previously read "Four separate things must line up." No source states them as one joint condition or as a complete list; the conjunction is **inferred**, not documented. This topic's 5.1 first-pass findings mark the same step for the same question.)*

**(a) A compositing node group must be assigned to the scene.**

> "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene"

That is the description of `use_compositing`, on the RenderSettings page of the API documentation. Type: `boolean, default True`.

The scene-side property is:

> "compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree"

- Source: Python API documentation.
- Addresses: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html and https://docs.blender.org/api/5.0/bpy.types.Scene.html
- Date read: 2026-07-30

The manual says the tree is created and assigned from the Compositor's header:

> "To begin compositing, open the Compositor and create a new node tree by clicking the New button in the header. This adds a compositing node tree data block to the scene, which can then be reused across different scenes or linked/appended from other blend-files."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/introduction.html
- Date read: 2026-07-30

The release notes state the data-block change outright:

> "The compositing node tree is now its own data block, making it reusable across blend files."

- Source: release notes. Address: https://developer.blender.org/docs/release_notes/5.0/compositor/
- Date read: 2026-07-30

**(b) The Compositing switch in Output ‣ Post Processing must be on.**

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/introduction.html
- Date read: 2026-07-30

And on the panel's own page:

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html
- Date read: 2026-07-30

I opened the panel screenshot the manual places on that page, `_images/render_output_properties_post-processing_panel.png`. It shows a panel titled "Post Processing" containing exactly two ticked checkboxes, labelled "Compositing" and "Sequencer", and a "Dither" slider reading 1.000. Nothing else. Label for that: **documented** (it is part of the manual).

**(c) The node tree must contain an active output, or the compositor does not run.**

> "The compositor searches for the active Group Output node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html
- Date read: 2026-07-30

Note that the Composite node no longer exists in 5.0:

> "The **Composite** node was removed and replaced by a Group Output node whose first input needs to be a Color socket, and other inputs are ignored."

- Source: release notes. Address: https://developer.blender.org/docs/release_notes/5.0/compositor/
- Date read: 2026-07-30

**(d) The old "Use Nodes" switch decides nothing any more.**

> "use_nodes
>
> Enable the compositing node group.
>
> Deprecated since version 5.0: removal planned in version 6.0
>
> Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.Scene.html
- Date read: 2026-07-30

The release notes say the same twice:

> "`scene.use_nodes` is deprecated and will be removed in 6.0 Currently it always returns `True` and setting it has no effect."

- Source: release notes. Address: https://developer.blender.org/docs/release_notes/5.0/python_api/

> "\"Use Nodes\" in the node editor was removed"

- Source: release notes. Address: https://developer.blender.org/docs/release_notes/5.0/compositor/
- Date read: 2026-07-30

**Observed, from Cycles' own files.** Cycles draws the Post Processing panel itself, and it draws only those two switches plus Dither:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"
...
        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")

        layout.prop(rd, "dither_intensity", text="Dither", slider=True)
```

- Source: `Blender Sources/cycles_5.0/ui.py`, lines 1112–1128 (class `CYCLES_PT_post_processing`).
- Date read: 2026-07-30

**A separate switch, for the viewport only.** Whether the compositor runs *inside the 3D viewport* is decided by a third setting:

> "use_compositor
>
> When to preview the compositor output inside the viewport
>
> DISABLED Disabled – The compositor is disabled.
> CAMERA Camera – The compositor is enabled only in camera view.
> ALWAYS Always – The compositor is always enabled regardless of the view.
>
> Type:
>
> enum in ['DISABLED', 'CAMERA', 'ALWAYS'], default 'DISABLED'"

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.View3DShading.html
- Date read: 2026-07-30

**LABEL: documented.**

**Where I looked.** Searched all of `manual_5.0` for "Post Processing", "Post-Processing", "post_processing"; searched all of `api_5.0` for `use_compositing`, `use_sequencer`, `use_nodes`, `compositing_node_group`; searched `release_notes_5.0` for "composit". Read in full: the compositing introduction page, the Compositor System page, the Post Processing panel page, the compositor and python_api release notes, `bpy.types.Scene`, `bpy.types.RenderSettings`. Searched `cycles_5.0/*.py` for "composit".

**Not covered.** The sources do not say what happens when all the switches are on but the assigned node group is empty, nor whether a *muted* Group Output node still counts as "active", nor whether a scene with `use_compositing` on but no node group assigned takes any different path than one with the switch off. They also do not say whether an assigned tree with no Render Layers node still causes a 3D render to happen.

**Outside the interface.** The switches are properties on the scene and are saved in the file, and the command-line documentation says `-a, --render-anim` will "Render the whole animation using all the settings saved in the blend-file". The documented list of command-line render options (`-b -a -S -f -s -e -j -o -E -t -F -x`) contains no flag that turns compositing on or off, so the file's setting stands. The viewport switch in (d) is described as "preview the compositor output inside the viewport", so it has no bearing outside the interface — that last step is **inferred** from the wording, not stated.

**Operating system.** The sources do not say the switches behave differently per operating system. The only operating-system-specific note anywhere near the compositor is about the GPU compositor on Windows: "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit" (https://docs.blender.org/manual/en/5.0/compositing/limitations.html).

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Reading of the question.** I take "the place where a scene's rendered frames are written" to be the render output File Path in Properties ‣ Output ‣ Output. Two other readings exist and I address them at the end.

**Answer: not in the sources — no switch for it is documented, and no such property exists in the property list.**

What the manual does say about that field:

> "File Path
>
> Choose the location to save rendered frames."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

I opened the panel screenshot on that page, `_images/render_output_properties_output_panel.png`. It shows a panel titled "Output" whose first row is a text field reading `/tmp/` with a folder-browse button at its right. There is no checkbox on the field, and no checkbox in the panel header. The only checkboxes visible in the panel are Overwrite, Placeholders, File Extensions and Cache Result. Label for that: **documented**.

**What *is* documented, and is close enough to be worth stating.**

A still render does not write anything by itself:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

And the render operator has a per-call switch for exactly that:

> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

Its default in the signature is `write_still=False`.

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.ops.render.html
- Date read: 2026-07-30

That is a per-invocation choice, not a setting stored against the scene's output location.

There is one documented way the output path stops working, but it is a failure rather than an off switch:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/files/file_paths.html
- Date read: 2026-07-30

**LABEL: not in the sources** (for an off switch) / **documented** (for the two findings above).

**Where I looked, and what I ruled out.**

- Read the whole Output panel page and opened its screenshot (above).
- Listed every property name on `bpy.types.RenderSettings` and read the description of each boolean. The complete set of booleans touching the output file is `use_file_extension`, `use_overwrite`, `use_placeholder`, `use_render_cache`. `filepath` itself is described as "Directory/name to save animations, # characters define the position and padding of frame numbers", type `string, default ""`. There is no `use_render_output`, `use_filepath`, `use_output` or anything of that shape.
- Searched the whole API index (`genindex-all.txt`) for property names matching `use_*output*`. Every hit belonged to something unrelated: `use_invert_output` (Grease Pencil modifier), `use_lossless_output` (FFmpeg), `use_output_a` / `use_output_b` (Dynamic Paint), `use_output_vertex_group_match_by_name` (Line Art).
- Checked whether the Media Type enum offers a "none": it does not. `media_type` is "enum in ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'], default 'IMAGE'" (https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html).
- Searched all of `release_notes_5.0` for "filepath", "File Path", "output path" — the only hit touching the render output is a bug-fix line, "Assert browsing a non UTF8 render output path".
- Searched `manual_5.0` and `release_notes_5.0` for the phrasings a documented off switch would use: "not written to disk", "without writing", "does not save", "not saved to disk", "no files are written". No relevant hit.
- Checked `cycles_5.0/ui.py` for any drawing of `filepath` — none; Cycles draws the Post Processing panel but not the Output panel.

**The other readings.** If the question means the compositor's File Output node, its manual page lists Directory, File Name, Media Type, Node Format, Images/Layers and Output Paths, and no enable/disable control (https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html); `bpy.types.CompositorNodeOutputFile` likewise exposes `directory`, `file_name`, `file_output_items`, `format`, `save_as_render` and no switch. If the question means "can a render be made to write nothing at all", the `write_still` finding above is the documented answer for stills, and the animation.html quote says animation frames are written automatically.

**Not covered.** The sources do not say what an empty File Path does, they do not say whether a *muted* File Output node stops writing, and they do not say whether a render can be made to produce no file at all when driven as an animation.

**Outside the interface.** From the command line the documented behaviour is the opposite of switched off: `-f, --render-frame <frame>` is described as "Render frame <frame> and save it", and the compositor release notes state "Rendering through the command line always does an animation render" (https://developer.blender.org/docs/release_notes/5.0/compositor/). `-o, --render-output <path>` sets the path. No documented flag suppresses writing.

**Operating system.** The sources do not say this differs by operating system.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

A scene can take in another scene's rendered result in two ways, and the sources treat them differently. I answer both.

### (a) Through a Render Layers node whose Scene is set to the other scene

The Render Layers node's own manual page carries a Hint that speaks directly to this:

> "To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node."

The same page describes the node and its Scene property:

> "Renders a View Layer and reads its Passes into the compositing node graph."

> "Scene
>
> The scene for which to render a view layer."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html
- Date read: 2026-07-30

The API describes the node the same way:

> "Input render passes from a scene render"

with a property `scene`, type `Scene`.

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.CompositorNodeRLayers.html
- Date read: 2026-07-30

So: **documented** that what arrives through a Render Layers node pointed at another scene is that scene's *raw render output*, not its compositing output — the manual says so by telling you to go and do something else if you want the compositing output.

Whether the other scene's compositor is nevertheless executed and thrown away is a further step the sources do not take. My claim that the second scene's compositor does not contribute to the first scene's result is **inferred** from the Hint: the reader has to supply the step "the compositing output is not available, therefore the compositor did not feed the graph". The sources do not say the compositor is skipped.

Two supporting quotations that show the sources do think in terms of one scene pulling on another:

> "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)"

- Description of `use_render_cache`. Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html

> "The views to render are defined in the current scene views, in a similar way as you define the composite output resolution in the current scene render panel, regardless of the Image nodes resolutions or Render Layers from different scenes."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch_stereo_view.html (the same sentence also appears at https://docs.blender.org/manual/en/5.0/render/output/properties/stereoscopy/usage.html)
- Date read: 2026-07-30

### (b) Through a Scene strip in the Video Sequencer

Here the sources are ambiguous and I flag it rather than choose silently.

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html
- Date read: 2026-07-30

That last sentence can be read two ways. Reading one: when Compositing is enabled *in the scene the strip points at*, the strip carries that scene's composited image — which would mean yes, the second scene's compositor runs. Reading two: the sentence is about the current scene's own Compositing switch, and describes the order of the current scene's own pipeline. The page gives no further wording to settle it, and I found nothing elsewhere that does.

**LABEL for (a): documented** (what arrives is the raw render output) **plus inferred** (therefore the other scene's compositor does not feed the first). **LABEL for (b): not in the sources** — the one sentence that bears on it is ambiguous and I cannot resolve it from the supplied documentation.

**Where I looked.** Searched all of `manual_5.0`, `api_5.0` and `release_notes_5.0` for "another scene", "other scene", "different scene", "from other scenes", "indirectly rendered". Read in full: the Render Layers node page, the Scene Strip page, the Post Processing page, the Sequencer Scene page, `bpy.types.CompositorNodeRLayers`, `bpy.types.Scene`, `bpy.types.RenderSettings`, the compositor and sequencer release notes. Also checked the Freestyle view-layer page, which mentions view layers "possibly in different scenes through the Compositor" but says nothing about compositors running.

**Not covered.** Whether the second scene's compositor is executed and discarded; whether the second scene's own `use_compositing` setting has any effect when it is pulled in by the first; whether the second scene's File Output nodes write anything when the scene is rendered indirectly; and the Scene-strip case, which the sources leave ambiguous.

**Outside the interface.** The sources say nothing about whether this differs when Blender is driven from outside its window. The behaviour described above is a property of the node graph and the scene settings, both stored in the file.

**Operating system.** The sources do not say this differs by operating system.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

Three parts; I take them in turn.

### What it arrives as

It arrives through the Render Layers node, as an image on the Image socket plus an Alpha socket plus one socket per enabled pass:

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

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html

The Combined pass is described as:

> "The render output before any compositing is applied."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/layers/passes.html

What "an image" means inside the compositor is set out on the Compositor System page:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats:"

— those three being Float ("A signed floating-point number. Integer data is also stored as floats because no integer type exist."), Vector ("A 4D vector.") and Color ("A 4D vector storing the Red, Green, Blue, and Alpha of the color. The color is free form and does not conform to a specific color space or alpha storage model, instead, appropriate nodes will have settings to control the representation of their output and nodes exist to convert between the different representations.").

An image also carries a position and transformation, not just a size:

> "images are not only represented by their size, but also by their transformation in that space, much like 3D objects have transformations. An identity transformation represents an image that is centered in space. The rectangular area occupied by an image in that space as defined by its transformation and size is called the Domain of the image."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html
- Date read: 2026-07-30

**One conflict inside the sources, worth flagging.** The Compositor System page says all three types are "stored in half precision formats". Two other places say precision depends on whether it is a final render:

> "compositor_precision
>
> The precision of compositor intermediate result
>
> AUTO Auto – Full precision for final renders, half precision otherwise.
> FULL Full – Full precision.
>
> Type:
>
> enum in ['AUTO', 'FULL'], default 'AUTO'"

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html

> "Precision GPU
>
> The precision of compositor intermediate result.
>
> Auto:
>
> Use full precision for final renders, half precision otherwise."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/sidebar.html
- Date read: 2026-07-30

I report both and do not reconcile them.

**LABEL for this part: documented.**

### Can it be told apart from anything else there

**Not in the sources** at the level of the data itself.

What *is* documented is that you can tell by looking at the graph: only the Render Layers node brings a render in, and the API names it "Input render passes from a scene render". Nothing in the supplied documentation describes any marker, tag, identifier or provenance carried by the image data once it has left that node.

The one node that reports anything about an image reports only its geometry:

> "The Image Info node outputs spatial and transformation information about an image in the compositor."

with outputs Dimensions, Resolution, Location, Rotation and Scale — nothing about origin.

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/input/image_info.html
- Date read: 2026-07-30

**Where I looked.** Searched every page under `manual_5.0/compositing` for "provenance", "identify", "distinguish", "tag", "origin of", "which node". Every hit was unrelated: a watermarking aside on the Mix Color page, the number badge on the Viewer node, and a sentence about masks on the Keying index. Read in full the Compositor System page (the only page that describes what data is), the Image Info node page, the Render Layers node page, and `bpy.types.RenderResult` / `bpy.types.RenderLayer` / `bpy.types.RenderPass` in the API listing. Nothing describes an identity carried by compositor data.

**LABEL for this part: not in the sources.**

### Can it be followed forward through pass-along nodes, or split

Yes on both counts, as far as the node system is documented.

Pass-along:

> "A node used primarily for organization. Reroute looks and behaves much like a socket on other nodes in that it supports one input connection while allowing multiple output connections."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/interface/controls/nodes/types/layout/reroute.html

Splitting one output into many:

> "While multiple links can route out of an output socket, typically a single link can be attached to an input socket, that is unless the input is a multi-socket input with looks like a pill shaped socket."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html
- Date read: 2026-07-30

The same page documents that links can be switched off without being removed, and that muting propagates through a Reroute:

> "A muted link acts as though it's no longer there; this also means the input fields for specifying fixed values become visible again."

> "When muting links on the input side of a reroute node, the links on its output side will be muted too."

There is also a node that passes one of two inputs straight through:

> "When it is unchecked, the first input labeled "Off" is passed to the output."

- Source: manual, Switch node. Address: https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch.html

For a person following the data by eye, the documented tool is the Viewer node:

> "The Viewer node is used to preview image data or value maps anywhere within a compositor node graph. It is a diagnostic tool that allows users to inspect intermediate results without affecting the final output."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/output/viewer.html
- Date read: 2026-07-30

**LABEL for this part: documented.**

**Ambiguity flagged.** "Can it be followed forward" has two readings. I answered the reading "does the data survive a Reroute and a fan-out, so that a reader of the file can trace the path". The other reading — "does Blender provide a way to trace or mark a particular image through the graph" — I found nothing on, and that is the "not in the sources" verdict recorded in the middle part above.

**Not covered.** The sources say a Reroute is "primarily for organization" but never state that the data passing through it is unaltered, so "unchanged" is my word for what the page implies rather than what it says. The sources do not say whether the unused branch of a Switch node is evaluated. They do not say whether fanning one output into several inputs copies the data or shares it.

**Outside the interface.** The sources do not say. The two things that are documented to differ between a final render and everything else are precision (`compositor_precision` Auto: "Full precision for final renders, half precision otherwise") and the compositor's output size (see question 6). The Viewer Region is explicitly preview-only: "This is only a preview option, final compositing during a render ignores this region." (https://docs.blender.org/manual/en/5.0/compositing/sidebar.html)

**Operating system.** The sources do not say. The only operating-system note near this material is the Windows TDR warning for the GPU compositor on https://docs.blender.org/manual/en/5.0/compositing/limitations.html.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Answer: yes — at least the render region, and the stereo output format. And it is documented that the compositor node tree does *not*.**

### The render region, with Crop to Render Region

> "Render Region
>
> Renders just a portion of the view instead of the entire frame. See the Render Region documentation to see how to define the size of the render region.
>
> Crop to Render Region
>
> Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/properties/format.html

The properties behind them, with their ranges:

> "use_border
>
> Render a user-defined render region, within the frame size
>
> Type:
>
> boolean, default False"

> "use_crop_to_border
>
> Crop the rendered frame to the defined render region size
>
> Type:
>
> boolean, default False"

and `border_min_x`, `border_max_x`, `border_min_y`, `border_max_y`, each "float in [0, 1]".

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html

And that this is a file-side thing that reaches the final render:

> "If you define one while in Camera View, it will apply not just to the viewport, but also to the final render."

> "Render regions only apply to the viewport when using Cycles, not when using EEVEE. However, they always affect the final render."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/editors/3dview/navigate/regions.html
- Date read: 2026-07-30

**LABEL: documented** that Crop to Render Region crops *the rendered image* to the region's size, and that the region reaches the final render; **inferred** for the step from there to the size of the file written to disk — every quoted sentence says "rendered image" or "rendered frame", never "written" or "saved". *(Citation check, 2026-08-03: previously labelled documented outright, against a question about "the size of what gets written". This topic's 5.1 second-pass findings flag the same gap; the first-pass hand test measures the written file and settles it.)*

### The stereo output format

> "SIDEBYSIDE Side-by-Side – Render views for left and right eyes side-by-side.
>
> TOPBOTTOM Top-Bottom – Render views for left and right eyes one above another."

> "use_squeezed_frame
>
> Combine both views in a squeezed image
>
> Type:
>
> boolean, default False"

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.Stereo3dFormat.html

> "The Top-Bottom compressed will discard half of your vertical resolution data."

> "For the Stereo 3D formats the only lossless options are Top-Bottom and Side-by-Side without the Squeezed Frame option."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/properties/stereoscopy/usage.html
- Date read: 2026-07-30

**LABEL: inferred.** The step I took: the manual says a squeezed top-bottom image throws away half the vertical resolution, and that unsqueezed top-bottom and side-by-side are lossless. Two full-resolution views cannot both fit losslessly in an image of the original size, so the unsqueezed forms must write an image larger than the scene resolution in one axis. The sources never state the pixel numbers, so a reader has to supply that step. Note also that `use_multiview` is "boolean, default False" and `ImageFormatSettings.views_format` defaults to `'INDIVIDUAL'`, which writes separate files per view — that changes how many files are written, not their size.

### What is documented *not* to change it: the compositor node tree

> "The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html

> "in a similar way as you define the composite output resolution in the current scene render panel, regardless of the Image nodes resolutions or Render Layers from different scenes."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/utilities/switch_stereo_view.html
- Date read: 2026-07-30

That matters because nodes exist that plainly change an image's size *inside* the compositing space. The Crop node "crops an input image to a selected region by either making the cropped area transparent or by resizing the input image" (https://docs.blender.org/manual/en/5.0/compositing/types/transform/crop.html) and the Scale node has a "Render Size" space described as "Image dimensions set in the Render panel" (https://docs.blender.org/manual/en/5.0/compositing/types/transform/scale.html). The domain rule above says the final output is nevertheless at the scene render size. That the two combine to mean "a Scale or Crop node cannot change the written size" is **inferred**, one short step from the quoted domain rule.

The path-template variables corroborate the base rule:

> "resolution_x / resolution_y:
>
> The x and y resolution of the rendered image. This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/files/file_paths.html
- Date read: 2026-07-30

### Candidates I checked and ruled out, with the wording that ruled them out

- **Aspect X/Y** — `pixel_aspect_x`: "Horizontal aspect ratio - for anamorphic or non-square pixel output", `float in [1, 200], default 1.0`. Pre-distortion, not a pixel count.
- **Pixel Density** — "The pixel-density is meta-data which doesn't impact the quality of the image." (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html). `ppm_factor` is "The pixel density meta-data written to supported image formats".
- **EEVEE Overscan** — "Percentage of the render size to add to the internal render buffer." (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/film.html); the API says "Percentage of render size to add as overscan to the internal render buffers". Internal buffer, not output.
- **preview_pixel_size**, **Proxy Render Size** (VSE, clip editor, camera) — preview settings.
- **VSE "Set Render Size"** — "Sets the render resolution and aspect to match the strip's resolution." (https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html). This works *by changing* the scene's resolution values, so it acts through the pixel dimensions the question already excludes.
- **Codec limits** — "Some codecs impose limitations on output size, H.264, for example requires both the height and width to be divisible by 2." (https://docs.blender.org/manual/en/5.0/files/media/video_formats.html). This is documented as a limitation; the sources do not say Blender changes the written size to satisfy it. That part is **not in the sources**.
- **The File Output node** — its page lists Directory, File Name, Media Type, Node Format, Images/Layers and Output Paths, and the API adds only `save_as_render` and `file_output_items`. No size setting. The release notes say the node "now take[s] translation into consideration and relevant inputs are clipped if translated" (https://developer.blender.org/docs/release_notes/5.0/compositor/), which speaks of clipping rather than resizing, but the sources never state what size a File Output node writes at. **Not in the sources.**

**Where I looked.** Searched all of `manual_5.0` for "render size", "output size", "output resolution", "resolution" (restricted to the compositing and video-editing trees), "overscan", "squeez". Listed and read every property on `bpy.types.RenderSettings`. Read `bpy.types.Stereo3dFormat` and `bpy.types.ImageFormatSettings` in full. Read the Format, Output, Metadata and Stereoscopy Usage pages; the Compositor System page; the Scale, Crop and Relative To Pixel node pages; the Render Region section of the 3D-viewport navigation page; the video formats limitations section; the path templates page. Searched `cycles_5.0/*.py` for "resolution", "border", "crop" — the only relevant hit is `ui.py` lines 399–401, which compute `render_size_x = int(scene.render.resolution_x * render_scale)` from `resolution_percentage / 100.0` for a path-guiding tiling warning, confirming the base formula but adding no new size decider.

**LABEL overall: documented** (render region and crop; the compositor domain rule) **plus inferred** (the stereo formats; that compositor nodes cannot change the written size).

**Not covered.** The exact pixel size of an unsqueezed stereo composite. What size the File Output node writes at. Whether a render region *without* Crop to Render Region leaves the written size untouched — the manual's phrase "instead of rendering a transparent background around it" implies it does, but that is an implication, not a statement. Whether Blender adjusts a size to satisfy a codec's constraints.

**Outside the interface.** Everything above is stored in the file, and the render region is documented to "always affect the final render". The documented command-line render options are `-b, -a, -S, -f, -s, -e, -j, -o, -E, -t, -F, -x`; none of them sets a resolution or a region, so nothing in that list overrides the file. Whether a rendered image differs in size between a windowed and a headless run is not stated anywhere I found.

**Operating system.** The sources do not say. The render-region note about Cycles versus EEVEE is per render engine, not per operating system.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Answer: yes.** The Post Processing panel carries two independent switches; turning Compositing off and Sequencer on does exactly this.

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

- Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html
- Date read: 2026-07-30

The two properties, both defaulting on and both independent:

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
>
> Type:
>
> boolean, default True"

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist
>
> Type:
>
> boolean, default True"

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Date read: 2026-07-30

I opened `_images/render_output_properties_post-processing_panel.png`, the screenshot the manual places on the Post Processing page. It shows both "Compositing" and "Sequencer" as separate, independently ticked checkboxes. Label: **documented**.

**What that changes about what gets written.**

1. **The picture.** The written frame is the Video Sequence editor's output "instead of the view from the 3D scene's active camera" — quoted above. **Documented.**

2. **Only if there are strips.** `use_sequencer` acts "if sequencer strips exist". **Documented.**

3. **Passes are reduced where scene strips are involved.**

   > "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

   - Source: manual. Address: https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html
   - **Documented**, though note it is stated about scene strips specifically, not about the sequencer pipeline as a whole.

4. **The metadata can come from a different place.**

   > "metadata_input
   >
   > Where to take the metadata from
   >
   > SCENE Scene – Use metadata from the current scene.
   > STRIPS Sequencer Strips – Use metadata from the strips in the sequencer.
   >
   > Type:
   >
   > enum in ['SCENE', 'STRIPS'], default 'SCENE'"

   and a separate stamp field:

   > "use_stamp_sequencer_strip
   >
   > Include the name of the foreground sequence strip in image metadata
   >
   > Type:
   >
   > boolean, default False"

   - Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
   - The manual's Metadata page carries the same two options ("Sequencer Strips: Use metadata from the strips in the Sequencer", "Strip Name: Includes the name of the foreground sequence strip") at https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html
   - **Documented.**

5. **Size, format and output path do not change.**

   > "The render and output settings of the Sequencer Scene are the same as for regular scenes. They can be found in the Output tab in the Properties editor."

   - Source: manual. Address: https://docs.blender.org/manual/en/5.0/video_editing/sequencer_scene.html
   - **Documented.**

6. **One further switch affects how strips' own scenes are rendered:**

   > "use_sequencer_override_scene_strip
   >
   > Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip
   >
   > Type:
   >
   > boolean, default False"

   - Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html

**New in 5.0 and relevant to which scene is rendered.** The sequencer no longer follows the active scene:

> "The sequencer no longer uses the active scene in the window for its context. Instead it now uses the _Sequencer Scene_. This is a reference to a scene stored per workspace."

> "If the sequencer scene differs from the active scene, the Render menu will show new options for \"Render Sequencer Image\" / \"Render Sequencer Animation\" on top of the normal scene render operators, allowing the user to specify which scene they'd like to render."

- Source: release notes. Address: https://developer.blender.org/docs/release_notes/5.0/sequencer/
- The manual matches: "Render Sequencer Image Alt-F12 — If a Sequencer Scene exists that differs from the active scene, render that scene instead." (https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html)
- Date read: 2026-07-30

**LABEL: documented.**

**Where I looked.** Read in full: the Post Processing page and its screenshot, the Scene Strip page, the Sequencer Scene page, the Metadata page, the Rendering Animations page, the topbar Render menu section, `bpy.types.RenderSettings`, `bpy.ops.render`, and the sequencer and compositor release notes. Searched all sources for "sequencer_scene", "Sequencer Scene", "use_sequencer", and searched `manual_5.0/render/output` for "sequencer".

**Not covered.** The sources do not say what happens to a scene's own compositor when Compositing is off and Sequencer is on — whether the node tree is skipped entirely or evaluated and discarded. They do not say whether the sequencer's output can carry more than one layer, or what happens to render passes generally (only the scene-strip statement in point 3). And the ambiguity flagged under question 4 — whose Compositing switch "If Compositing is also enabled" refers to — bears on this answer too.

**Outside the interface.** The render operator carries the choice explicitly:

> "use_sequencer_scene (boolean, (optional)) – Use Sequencer Scene, Render the sequencer scene instead of the active scene"

- Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.ops.render.html

From the command line, `-S, --scene <name>` "Set the active scene <name> for rendering" is the documented way to pick a scene; the documented option list contains nothing for the Sequencer Scene. Since the release notes describe the Sequencer Scene as "a reference to a scene stored per workspace", a headless run would have to name the scene directly with `-S` — that last step is **inferred**, not stated. The Post Processing switches themselves are saved in the file and `-a` uses "all the settings saved in the blend-file".

**Operating system.** The sources do not say this differs by operating system.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

Items 1 to 9 have been run and are **settled in the first-pass document**, `Preparation/Blender Research/Findings/5.0/The compositor.md`, where the observed findings and the machine's raw output are recorded under the questions they answer. In short: an assigned tree with no Group Output writes the raw render; there is no off switch on the render output path in 5.0 and an emptied path fails with `Couldn't create directory for file 0001.png`; a Render Layers node pointing at another scene brings back that scene's raw render and that scene's compositor does no work at all; a Scene strip follows the strip's own scene's Compositing switch; a File Output node writes at its input's size, not the scene's; Crop to Render Region halves both dimensions as expected while Render Region alone leaves them alone; and unsqueezed stereo doubles one axis. The set-ups and commands are in `Audits/Blender Research/The compositor/Hand tests.md`, entries 4, 6, 7, 8 and 10.

**Two more have been settled since, by the Creator looking at a Blender window**, and both are recorded in the same first-pass document.

**What the Output panel looks like in 5.0** is answered in full, and is no longer on the list below. It does have the Media Type dropdown its manual text describes, with three items — Image, Multi-Layer EXR, Video — so where that page's own screenshot disagrees with its words, the screenshot is the part that is out of date. There is no checkbox in the panel's header. That is under question 2, from `Audits/Blender Research/The compositor/Hand tests.md`, entry 16.

**Whether anything in the interface names where an image came from** is answered for the socket-inspection half only. Hovering a link in 5.0 shows the socket's name, "Image", and nothing that names where the image came from — under question 5, from entry 19. The Image Info node half is untried, so that item stays on the list.

Two are still open, and both need somebody looking at a Blender window rather than a terminal:

1. **Is there anything in the interface that names where an image came from?** The socket-inspection half of this is answered above. Still untried: in the compositor, select an Image Info node fed from a Render Layers node, and then the same node fed from an Image node. Open the sidebar (N) and read every field on the node in both cases. Report whether any field differs in a way that names the source.

2. **What does a Reroute look like when it is made by hand**, with the Shift and right-drag shortcut, rather than by a script. Only the script route has been tried.

**A second machine, 2026-08-06.** Every test named above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and the results are recorded beside the Mac's in the first-pass document, never merged with them. All of them agreed with the Mac. The one exception is the socket-inspection hover: it could not be settled on that machine at all, because its display is shared with other work and the pointer would not stay put, so that reading still rests on the Mac alone. The full list is `Audits/Blender Research/The compositor/Hand tests — Linux.md`.

**Cannot be run with the available equipment.** The operating-system lines above are all source-silence lines — they say the sources do not tie this to an operating system, and no test changes that. What the two machines settle is written into the first-pass document, and it covers macOS and Linux for everything except the socket-inspection hover. No Windows machine has run it, so Windows is untried, *(Linux citation check, 2026-08-06: this previously read "The operating-system lines above now cover macOS and Linux", which none of this document's own operating-system lines carries — they all say the sources are silent. Corrected to point at where the machine results actually live.)* and the two machines have Apple's own graphics and one NVIDIA card between them, so no card of any other make can be reached. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

2026-08-03 — Opus 5 (Claude Code) — The compositor, Blender 5.0

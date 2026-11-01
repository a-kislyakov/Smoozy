# The compositor — Blender 5.3

- Sources consulted: the Blender 5.3 manual (plain-text copy and its pictures), the Blender 5.3 Python API documentation and the Blender 5.3 release notes, all taken 2026-10-11; Cycles' own files for 5.3, taken 2026-10-16. The work was done on 2026-10-17. As instructed, every "date read" below is the snapshot date.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note on context I did not open. Before I started, the tool I run in loaded some text into my context without being asked. It was the project's top-level signpost file, a short index of the user's saved notes, the repository's file status and its recent commit titles. I did not open any of those files, and I did not follow anything they point to. Nothing below draws on them. Inside my notes folder I converted two manual pictures from WebP to PNG so I could view them. I checked whether the output file already existed (it did not), without opening anything.

Version note. Every quote below comes from the 5.3 sources. I did not use the other versions' folders.

---

## 1. What decides whether a file's compositor runs at all?

**Label:** documented (each switch below); inferred (how the switches combine, where marked).

**Answer.** For a final render, the sources name these deciders. All of them are set per scene, not per file (see question 3).

**a) The scene's Compositing switch, in Output Properties ‣ Post Processing.**
- "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties." — manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html
- "Pipeline – Compositing / Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html
- Picture `_images/render_output_properties_post-processing_panel.png` on that page: a panel titled "Post Processing". It has a ticked "Compositing" checkbox, a ticked "Sequencer" checkbox, and "Dither 1.000".
- `use_compositing`: "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html

**b) A node group must be assigned to the scene.** The API sentence above says the switch applies "if a compositing node group is assigned to the scene". In 5.3 a scene holds its node groups in a stack of "effects":
- "The compositor now uses an effects stack instead of a single compositor node group." — release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/
- `compositor_effects`: "Compositor effects for this scene (default None, readonly)" — API, https://docs.blender.org/api/5.3/bpy.types.Scene.html
- `node_group`: "Node group that controls what this effect does" — API, https://docs.blender.org/api/5.3/bpy.types.SceneCompositorEffect.html

**c) Each effect has its own render switch.**
- `enable_for_render`: "Use effect during render (default False)". `enable_for_preview`: "Use effect during preview (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneCompositorEffect.html
- Inferred: an effect whose render switch is off takes no part in a final render. The step: "Use effect during render" is the only render-time switch named for an effect, so off should mean not used. The sources do not spell this out.

**d) The old "Use Nodes" switch no longer decides anything.**
- `use_nodes`: "Deprecated since version 5.0: removal planned in version 6.0 / Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing." — API, https://docs.blender.org/api/5.3/bpy.types.Scene.html

**e) Inside a node group, only what reaches the output is worked out.**
- "A compositor node tree is evaluated from the outputs backwards to determine which nodes contribute to the final result. Nodes that are not connected to a Group Output are ignored during evaluation." — manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html
- "If multiple Group Output nodes are present in a node tree, only the active one is used." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/group_output.html

**Other times the compositor runs (not the final render).** These are tied to editors:
- The 3D Viewport shading option "Compositor" has these settings: "Disabled: Never show the compositing output. / Camera: Only show the compositing output when in Camera View … / Always: Always show the compositing output." — manual, https://docs.blender.org/manual/en/5.3/editors/3dview/display/shading.html
- "Viewport compositor is now enabled by default for the startup file in the default 'Layout' workspace and is set to *Always*." — release notes, https://developer.blender.org/docs/release_notes/5.3/user_interface/
- "Rendering no longer writes to viewer images, however, the interactive compositor is executed after rendering if the viewer is visible." — release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/

**Not covered:**
- The API lists `enable_for_render` with "(default False)". The sources do not say what value a newly made effect actually gets, or what the interface sets.
- When a scene has several effects, the sources do not say in what order they run, or whether each one receives the previous one's result.
- There is a flag on a node group: `allow_usage_in_scene_compositor_effect`, "The node group can be used as a scene compositor effect (default False)" (API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeTree.html). The sources do not say whether a group with this flag off can still run as an effect.
- The manual says nodes not connected to a Group Output are ignored. It also says the File Output node "writes one or more images to disk during rendering or compositing". A File Output node has no output sockets, so it cannot be connected to a Group Output. The sources do not reconcile these two statements.
- Where the sequencer also runs, see question 7.

**Where I looked:**
- I searched the whole 5.3 manual for "Compositing Pipeline", "post processing", "compositing is enabled", "disable compositing", "use nodes", "effects stack", "compositor effect", "effect stack", "enable for render", "scene effect", "layered compositing" and "compositor stack".
- The manual has no page describing the effects stack. The only "compositor effect" hits are on the Sequencer's compositor strip page.
- I read these API pages in full: Scene, RenderSettings, SceneCompositorEffect, SceneCompositorEffects, SceneCompositorEffectProperties, CompositorNodeTree and BlendData. BlendData has nothing about compositing.
- I read all of the 5.3 release notes.

**Outside the interface:**
- `use_compositing` and `enable_for_render` are data properties. A script can read or set them, for example one passed with `--python` or `--python-expr`.
- The command-line arguments page lists no argument that turns compositing on or off. I searched it for "compos".
- The viewport compositor and the interactive compositor depend on an editor being shown. Inferred: neither runs in a background render (`-b`), because no editor is shown there. The sources do not say this outright.

**Operating system:** The sources do not say this differs between operating systems.

**Driven from outside — observed, 2026-10-18.** Every switch above was exercised in renders started from a terminal with no window (`-b`), entries 5 to 11 in the observed section below: the Compositing switch, a group given or not, each effect's render switch, the order of several effects, a missing or muted Group Output, a File Output node joined to nothing, and the compositor device. They behaved as written there. That neither the viewport compositor nor the interactive compositor runs in a background render was not tested as such; entry 40 found that with no window a render does not fill the Viewer node's image.

**Operating system — observed, 2026-10-18.** Settled on macOS alone: every test below ran on the Creator's Mac (Apple M1 Pro, Apple's own graphics). No Linux or Windows machine was available to this sitting, so both stay open, and the GPU-device results (entries 8 to 10) speak for Apple's own graphics and no other make of card.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for everything in this section and for nothing above it. What the sources say, above, stands unchanged; this is the separate claim of what Blender 5.3 did.

**On what kind of computer.** The Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83), arm64. The Blender was 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Unless an entry says otherwise it was started from a terminal with no window (`-b`) and with `--factory-startup`, and with its temporary folder and every user folder pointed at a work folder of the test's own, which Blender was asked about and confirmed before the first test. The entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`, where every script is printed in full. Nothing here was run on Linux or Windows.

**Entry 1 — what the switches are called, what they start as, and how the effects list behaves.** Blender was started on its own factory start-up file and asked to print each named property's value, or `MISSING` where the property does not exist. Word for word, Blender's own warnings included (each warning line was followed by the line of the script that touched the property, left out here):

```
DeprecationWarning: 'Scene.use_nodes' is expected to be removed in Blender 6.0
DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0
BLENDER_VERSION 5.3.0 Beta
Scene.use_nodes = True
Scene.node_tree = MISSING
Scene.compositing_node_group = None
Scene.compositor_effects = bpy.data.scenes['Scene'].compositor_effects
RenderSettings.use_compositing = True
RenderSettings.use_sequencer = True
RenderSettings.save_output = True
RenderSettings.compositor_device = 'GPU'
RenderSettings.compositor_precision = 'AUTO'
bpy.types.SceneCompositorEffect = PRESENT
bpy.types.SceneCompositorEffects = PRESENT
bpy.types.SceneCompositorEffectProperties = PRESENT
bpy.types.CompositorNodeTree = PRESENT
bpy.types.CompositorNodeComposite = MISSING
bpy.types.CompositorNodeRLayers = PRESENT
bpy.types.NodeGroupInput = PRESENT
bpy.types.NodeGroupOutput = PRESENT
SceneCompositorEffect properties: ['rna_type', 'name', 'enable_for_render', 'enable_for_preview', 'is_active', 'show_expanded', 'show_node_group_selector', 'node_group', 'properties']
SceneCompositorEffects functions: ['new', 'remove', 'clear']
  function new parameters: [('name', 'STRING', True), ('effect', 'POINTER', False)]
  function remove parameters: [('effect', 'POINTER', True)]
  function clear parameters: []
CompositorNodeTree own properties: ['use_viewer_border', 'is_strip_modifier', 'allow_usage_in_scene_compositor_effect']
compositor_effects on a fresh scene: count 0 []
node_groups in a fresh file: []
```

So in a fresh file the effects list is empty and no node group exists. The older per-scene pointer, `compositing_node_group`, is still there and starts empty, and Blender warns that it is to be removed. Each other switch reads as its API default.

A second script then gave a scene a node group through that older pointer, emptied the pointer, made an effect with `compositor_effects.new`, and gave that effect the same group. Word for word:

```
new tree allow_usage_in_scene_compositor_effect: False
new tree nodes: []
new tree interface: []
after setting compositing_node_group: compositing_node_group = <bpy_struct, CompositorNodeTree("TreeOne") at 0x8c320c308>
  effects: [('Scene Effect', True, True, True, 'TreeOne')]
after clearing compositing_node_group: effects: [('Scene Effect', None)]
effects.new ->  <bpy_struct, SceneCompositorEffect("EffectOne") at 0x8c73bd098> name EffectOne enable_for_render True enable_for_preview True is_active True node_group None
after assigning node_group to the effect: compositing_node_group = <bpy_struct, CompositorNodeTree("TreeOne") at 0x8c320c308>
  effect props: ['rna_type', 'inputs', 'panels']
bpy.ops.scene compositor-ish operators: ['add_compositor_effect', 'add_compositor_effect_menu', 'add_compositor_effect_node_group_asset', 'duplicate_compositor_effect', 'duplicate_compositor_effect_node_group', 'move_compositor_effect_to_index', 'new_compositor_effect_node_group', 'remove_compositor_effect', 'set_active_compositor_effect']
bpy.ops.node compositor-ish operators: ['duplicate_compositing_modifier_node_group', 'duplicate_compositing_node_group', 'new_compositing_node_group', 'new_compositor_sequencer_node_group']
```

The effects line reads, in order: name, render switch, preview switch, whether it is the active effect, its group. **This answers what the "Not covered" list above leaves open about a new effect's switches**: an effect made with `compositor_effects.new` started with its render switch **on** and its preview switch on, and giving the scene a group through `compositing_node_group` made an effect called `Scene Effect` with both switches on. Neither matches the "(default False)" the API page prints. Setting the older pointer showed up as an effect, and giving an effect a group showed up through the older pointer.

**Entry 2 — which node types exist, what a new node group holds, and what the two "new group" operators make.** Blender was asked whether each type exists, then to add each node to an empty compositor group and print its sockets, then to run the two operators that make a compositor group. Word for word:

```
BLENDER_VERSION 5.3.0 Beta
bpy.types.CompositorNodeComposite = MISSING
bpy.types.CompositorNodeMixRGB = MISSING
bpy.types.CompositorNodeMix = MISSING
bpy.types.ShaderNodeMix = PRESENT
bpy.types.ShaderNodeMixRGB = PRESENT
bpy.types.CompositorNodeAlphaOver = PRESENT
TREE_TYPE CompositorNodeTree
TREE_STARTING_NODES []
ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']
ADD CompositorNodeSwitch -> inputs=['Switch', 'Off', 'On'] outputs=['Image']
ADD CompositorNodeRLayers -> inputs=[] outputs=['Image', 'Alpha']
ADD CompositorNodeOutputFile -> inputs=[''] outputs=[]
ADD CompositorNodeViewer -> inputs=['Image'] outputs=[]
ADD CompositorNodeComposite -> ERROR Error: Node type CompositorNodeComposite undefined

ADD NodeGroupInput -> inputs=[] outputs=['']
ADD NodeGroupOutput -> inputs=[''] outputs=[]
node.new_compositing_node_group -> {'FINISHED'}
scene.new_compositor_effect_node_group -> {'FINISHED'}
GROUP 'NodeTree' nodes [('Group Output', 'NodeGroupOutput'), ('Group Input', 'NodeGroupInput')] links [('Group Input', 'Image', 'Group Output', 'Image')] interface [('Image', 'OUTPUT'), ('Image', 'INPUT')] allow_usage_in_scene_compositor_effect False
GROUP 'Scene Compositor' nodes [('Group Output', 'NodeGroupOutput'), ('Group Input', 'NodeGroupInput')] links [('Group Input', 'Image', 'Group Output', 'Image')] interface [('Image', 'OUTPUT'), ('Image', 'INPUT')] allow_usage_in_scene_compositor_effect True
scene effects after the two operators: [('Scene Effect', True, True, 'Scene Compositor')]
scene.compositing_node_group: Scene Compositor
```

So a group made with `bpy.data.node_groups.new` starts with no nodes at all, and a scene can be given one, which is a state entry 6 renders. Both operators made a group holding a Group Input wired to a Group Output, with no Render Layers node — the arrangement the 5.3 release note quoted in question 5 describes. Only the second operator gave the scene an effect, and only its group came out with the "can be used as a scene compositor effect" flag on. Which operator a button in the window calls is entry 35's.

**Entry 5 — whether "Use Nodes" still decides anything.** A 64×64 Workbench scene whose render is one flat green was given a node group of Render Layers → Invert Color → Group Output through `compositing_node_group`, with Compositing on. `scene.use_nodes` was set each way, read back, and a frame rendered and written each time. Word for word:

```
BLENDER_VERSION 5.3.0 Beta
bl_use_postprocess in RNA: True
bl_use_image_save in RNA: True
RAW (compositing off) -> (64, 64, [0.4235, 0.7686, 0.3137, 1.0])
RESULT set scene.use_nodes=True -> reads back True -> written pixel [0.9333, 0.702, 0.9647, 1.0]
RESULT set scene.use_nodes=False -> reads back True -> written pixel [0.9333, 0.702, 0.9647, 1.0]
```

The first line of results is the raw green, written with Compositing off; the other two are the green inverted. Setting `use_nodes` to False did not take — it read back True — and the compositor ran either way, as the API sentence quoted above says.

**Entry 6 — what makes the compositor run in a render with no window.** A 64×64 Workbench scene whose raw render is flat black (its world colour black, no objects in view), rendered and written in turn: (1) a group of Render Layers → Invert Color → Group Output, with Compositing on and then off; (2) the same group with its Group Output node deleted; (3) a red colour node into the Group Output, with no Render Layers node; (4) the Group Output node muted; (5) no group given to the scene; (6) the render taken in through a Group Input node — Group Input → Invert Color → Group Output; (7) case 1's group given, and then the scene's effects list emptied. Each group was given through `compositing_node_group`, the scene's effects list being emptied before each new one. Word for word:

```
RESULT case1 invert, use_compositing=True -> 64x64 first pixel RGBA [1.0, 1.0, 1.0, 1.0]
RESULT case1 invert, use_compositing=False -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 0.0]
RESULT case3 red RGB into output, no Render Layers -> 64x64 first pixel RGBA [0.8549, 0.2196, 0.1294, 1.0]
RESULT case4 output node muted -> 64x64 first pixel RGBA [1.0, 1.0, 1.0, 1.0]
case5 effects now: ['Scene Effect'] compositing_node_group: None
RESULT case5 no compositor tree assigned -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case6 Group Input -> Invert -> Group Output -> 64x64 first pixel RGBA [1.0, 1.0, 1.0, 1.0]
case7 before clearing, effects: [('Scene Effect', 'Case1')]
case7 after clearing, effects: [] compositing_node_group: None
RESULT case7 tree given, then effects list emptied -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
```

Opaque black is the raw render, white is black inverted, red is the red. The Compositing switch turned the compositor off (case 1). A group with no Render Layers node still ran and its result is what was written (case 3). Muting the Group Output node did not stop its input reaching the file (case 4). With no group (case 5) — where emptying the pointer left one effect named `Scene Effect` with no group — and with the effects list emptied (case 7), the raw render was written. **The render arrived at a Group Input node**, as the manual sentence quoted in question 5 says: case 6 came out inverted with no Render Layers node in the group.

**Case 2 came out transparent** — alpha 0, not the raw render's alpha 1 — so it was run again over a raw render that is not black. Word for word:

```
RAW black, compositing off -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 1.0]
RESULT case2 no output node, raw black, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 0.0]
RAW green, compositing off -> 64x64 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0]
RESULT case2 no output node, raw green, use_compositing=True -> 64x64 first pixel RGBA [0.0, 0.0, 0.0, 0.0]
```

With a group given and no Group Output node in it, the written frame was transparent black over a green raw render too. So what was written was not the raw render. This bears on the manual sentence in answer 1(e): nodes not joined to a Group Output are not evaluated, and here nothing was joined to one.

**Entry 7 — the effects stack.** A 64×64 Workbench scene of one flat colour, written as 32-bit OpenEXR with no compression so that pictures could be compared value by value. Four groups, each starting from a Group Input: *Inv* (Invert Color), *Half* (an Exposure node at −1), and two references that chain both inside one group, *InvThenHalf* and *HalfThenInv*. Each was first rendered as the scene's only effect, to give a reference picture. Effects were made with `compositor_effects.new` and given a group. `maxdiff` is the largest difference in any channel of any pixel between two written pictures. Word for word:

```
BLENDER_VERSION 5.3.0 Beta
Exposure node sockets: ['Image', 'Exposure'] ['Image']
RESULT raw render, Compositing off -> first pixel [0.1498, 0.5483, 0.0799, 0.998]
RESULT one effect, group Inv -> first pixel [0.8502, 0.4517, 0.9201, 0.998]
RESULT one effect, group Half -> first pixel [0.0749, 0.2742, 0.0399, 0.998]
RESULT one effect, group InvThenHalf -> first pixel [0.4251, 0.2258, 0.4601, 0.998]
RESULT one effect, group HalfThenInv -> first pixel [0.9251, 0.7258, 0.9601, 0.998]
NEW EFFECT switches: enable_for_render=True enable_for_preview=True is_active=True
RESULT Inv effect, enable_for_render=False -> first pixel [0.1498, 0.5483, 0.0799, 0.998]
   maxdiff vs raw 0.0 ; vs Inv 0.8402099609375
RESULT Inv effect, enable_for_render=True enable_for_preview=False -> first pixel [0.8502, 0.4517, 0.9201, 0.998]
   maxdiff vs raw 0.8402099609375 ; vs Inv 0.0
RESULT two effects, list order ['Inv', 'Half'] -> first pixel [0.4251, 0.2258, 0.4601, 0.998]
   maxdiff vs one-effect reference Inv = 0.4600525498390198
   maxdiff vs one-effect reference Half = 0.4201049245893955
   maxdiff vs one-effect reference InvThenHalf = 0.0
   maxdiff vs one-effect reference HalfThenInv = 0.5000000894069672
   maxdiff vs raw = 0.3801574110984802
   which effect is_active: [('Inv', False), ('Half', True)]
RESULT two effects, list order ['Half', 'Inv'] -> first pixel [0.9251, 0.7258, 0.9601, 0.998]
   maxdiff vs one-effect reference Inv = 0.274169921875
   maxdiff vs one-effect reference Half = 0.9201049841940403
   maxdiff vs one-effect reference InvThenHalf = 0.5000000596046448
   maxdiff vs one-effect reference HalfThenInv = 5.960464477539063e-08
   maxdiff vs raw = 0.880157470703125
   which effect is_active: [('Half', False), ('Inv', True)]
RESULT two effects Inv(render off), Half(on) -> first pixel [0.0749, 0.2742, 0.0399, 0.998]
   maxdiff vs Half 0.0 ; vs InvThenHalf 0.4201049245893955 ; vs raw 0.2741699516773224
effect with no node group: [('NoGroup', None)]
RESULT one effect with no node group -> first pixel [0.1498, 0.5483, 0.0799, 0.998]
   maxdiff vs raw 0.0
RESULT effect with no group, then Inv -> first pixel [0.8502, 0.4517, 0.9201, 0.998]
   maxdiff vs raw 0.8402099609375 ; vs Inv 0.0
RESULT Inv group allow_usage_in_scene_compositor_effect=False -> first pixel [0.8502, 0.4517, 0.9201, 0.998]
   maxdiff vs Inv reference (made with the flag at its new-group value False) 0.0
RESULT Inv group allow_usage_in_scene_compositor_effect=True -> first pixel [0.8502, 0.4517, 0.9201, 0.998]
   maxdiff vs Inv reference (made with the flag at its new-group value False) 0.0
```

**This settles four of the "Not covered" points above, in a render with no window.**

- **An effect's render switch decides whether it takes part in the render.** Off, the raw render was written to the last figure; turning only its preview switch off changed nothing in the render.
- **Several effects run in list order, each receiving the one before's result.** With *Inv* listed before *Half*, the picture matched the one-group *InvThenHalf* reference exactly; listed the other way, it matched *HalfThenInv* to within one step of a floating-point number. It matched neither single effect, so neither effect was dropped. In both cases the last-made effect was the one marked active, and the order followed the list, not which effect was active. An effect whose render switch is off is left out of the chain: *Inv* off and *Half* on gave exactly *Half*.
- **An effect with no node group** passed the render through unchanged, and did not stop a later effect running.
- **A group with "can be used as a scene compositor effect" off still ran as an effect**, giving the same picture as with it on.

**Entry 8 — the compositor device, and the Enable Output node.** Entry 5's flat-green scene and invert group, rendered with the compositor device set to CPU and to GPU (Apple's own graphics); then an Enable Output node put between the Invert node and the Group Output, rendered with its Enable input on and off. Word for word:

```
BLENDER_VERSION 5.3.0 Beta
RESULT compositor_device=CPU -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0]
RESULT compositor_device=GPU -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0]
Enable Output node inputs: ['Enable', 'Value'] outputs: ['Value']
Enable Output node own properties: ['data_type']
RESULT Enable Output set True -> [0.7804, 0.7804, 0.7804, 1.0]
RESULT Enable Output set False -> [0.0, 0.0, 0.0, 1.0]
```

Both devices gave the inverted green. The Enable Output node was left at its own data type, which is a single value, so with Enable on the colour reached the output as a grey. With Enable off the written frame was black — not the green camera view — so the compositor ran and delivered nothing rather than being skipped.

**Entry 9 — whether a compositor tree gives the same picture on the graphics card as on the processor.** A 320×240 Workbench scene holding a sphere, a cube and a cone under the studio light, written as 32-bit OpenEXR, was given four groups in turn — an Invert; a Blur of size 12; a Scale set to an absolute 160×120; and Invert → Blur of size 6 → Alpha Over laying the blurred copy over the original at a factor of 0.5. Each was rendered once with the compositor device on CPU and once on GPU (Apple's own graphics), and the two written pictures compared value by value. Word for word (the Blur and Scale socket lists were printed so that the script set the right inputs):

```
BLUR inputs [('Image', 'RGBA'), ('Size', 'VECTOR'), ('Type', 'MENU'), ('Extend Bounds', 'BOOLEAN'), ('Separable', 'BOOLEAN')]
RESULT tree=invert device=CPU -> 320x240 first pixel [0.9492, 0.9492, 0.9492, 0.998]
RESULT tree=invert device=GPU -> 320x240 first pixel [0.9492, 0.9492, 0.9492, 0.998]
COMPARE tree=invert -> 307200 values compared, largest difference 0.000000000, 0 values differ by more than one step of 255
RESULT tree=blur device=CPU -> 320x240 first pixel [0.0508, 0.0508, 0.0508, 0.998]
RESULT tree=blur device=GPU -> 320x240 first pixel [0.0508, 0.0508, 0.0508, 0.998]
COMPARE tree=blur -> 307200 values compared, largest difference 0.000000238, 0 values differ by more than one step of 255
SCALE inputs [('Image', 'RGBA'), ('Type', 'MENU'), ('X', 'VALUE'), ('Y', 'VALUE'), ('Frame Type', 'MENU'), ('Interpolation', 'MENU'), ('Extension X', 'MENU'), ('Extension Y', 'MENU')]
RESULT tree=scale device=CPU -> 320x240 first pixel [0.0, 0.0, 0.0, 0.0]
RESULT tree=scale device=GPU -> 320x240 first pixel [0.0, 0.0, 0.0, 0.0]
COMPARE tree=scale -> 307200 values compared, largest difference 0.000005424, 0 values differ by more than one step of 255
ALPHAOVER inputs=['Background', 'Foreground', 'Factor', 'Type', 'Straight Alpha'] outputs=['Image']
RESULT tree=invert+blur+alphaover device=CPU -> 320x240 first pixel [0.5, 0.5, 0.5, 0.999]
RESULT tree=invert+blur+alphaover device=GPU -> 320x240 first pixel [0.5, 0.5, 0.5, 0.999]
COMPARE tree=invert+blur+alphaover -> 307200 values compared, largest difference 0.000000268, 0 values differ by more than one step of 255
```

On Apple's own graphics the two devices gave the same picture to within about five millionths on every tree, and identical to the last figure on the Invert. Two smaller and heavier trees, and what a socket exposes, are under question 8.

**Entry 10 — frames above 8k.** Written out in full under question 6. In short: the compositor ran on both devices at 9000×400, 400×9000 and 10000×5000, and every file came out at the full size; on CPU the whole frame was composited, while on GPU only 8,192 times the shorter side's worth of pixels were, and the top of the frame held the raw render.

**Entry 11 — a File Output node that is not connected to the Group Output.** A 64×64 scene with Compositing on, given three groups in turn, each File Output node aimed at an empty folder of its own: (a) Render Layers → Group Output, and Render Layers → File Output; (b) the same with the Group Output node deleted; (c) Group Input → Group Output, and Group Input → File Output. Each was rendered as a still twice, once with nothing asked to be saved and once with saving asked for (`write_still`), the folders emptied in between. Word for word:

```
RESULT case a, still render, nothing asked to be saved -> File Output folder: ['fo_.exr'] ; main output files: []
RESULT case a, still render, saving asked for -> File Output folder: ['fo_.exr'] ; main output files: ['main_a_.png']
RESULT case b, still render, nothing asked to be saved -> File Output folder: ['fo_.exr'] ; main output files: []
RESULT case b, still render, saving asked for -> File Output folder: ['fo_.exr'] ; main output files: ['main_b_.png']
RESULT case c, still render, nothing asked to be saved -> File Output folder: ['fo_.exr'] ; main output files: []
RESULT case c, still render, saving asked for -> File Output folder: ['fo_.exr'] ; main output files: ['main_c_.png']
```

**This settles the "Not covered" point above about the two manual statements.** A File Output node with no path to any Group Output wrote its file in every case — with a Group Output elsewhere in the group, with none at all, and fed from the Group Input — and it wrote during a still render that was not asked to save anything. The node's factory folder is `/tmp/`; every folder here was set by the script.

### Observed — window tests on the Creator's Mac, 2026-10-18

**Label: observed**, and these are looks at what Blender's window shows. Same Mac and same Blender 5.3.0 Beta as above, started from a terminal **with its window open** (no `-b`), with `--factory-startup`, its own settings folders, and `--enable-event-simulate`, which Blender's own help describes as "Enable event simulation testing feature 'bpy.types.Window.event_simulate'". A script running on Blender's own timer closed the splash screen with a simulated Escape, arranged the window, moved a simulated pointer or pressed simulated keys where an entry says so, and photographed the window with Blender's own screenshot command. Nobody touched the Mac. The photographs were kept only in the test's own work folder outside this project, which is not kept, so what each shows is written out in words here. The window was 3024×1724 pixels.

**Entry 33 — what the Post Processing panel holds.** The Properties editor was made to fill the window and put on its Output tab. With every panel at its factory state, the last panel's header reads `> Post Processing` with only the fold arrow — **no checkbox** — while the Stereoscopy and Output headers in the same picture each carry one. A simulated Ctrl-click on that header opened it and folded the rest. The opened panel holds a row labelled **Pipeline** with two ticked checkboxes, **Compositing** and **Sequencer**, and below them a slider labelled **Dither** reading **1.00**. Nothing else.

**Entry 35 — what the compositor's node editor shows in a fresh file, and what its New button makes.** The window was switched to the Compositing workspace and its node editor made to fill the window. The script printed `NODE EDITOR tree_type CompositorNodeTree node_tree None` and `EFFECTS [] GROUPS []`. The header holds, left to right: the editor-type dropdown; a dropdown reading **Scene**; the menus **View**, **Select**, **Add** and **Node**; a node-group chooser with a browse button, **+ New** and a pin; and at the right a **Backdrop** button and overlay buttons. **There is no Use Nodes checkbox.** Below the header, at the top left, the word **Scene**; the canvas empty; and along the bottom a shelf of ready-made items with tabs reading All, Camera & Lens Effects, Creative and Utilities (the last three shown twice), its thumbnails labelled Bloom, Chrom…, Film G…, Senso…, Vignette, Sepia, Split T…, Tune I…, Unsha… and Retime. A simulated click on **New** was then made at that button's place in the picture. Word for word, what the script then printed:

```
AFTER THE CLICK, EFFECTS [('Scene Effect', True, True, 'Scene Compositor')]
GROUP 'Scene Compositor' nodes [('Group Output', 'NodeGroupOutput'), ('Group Input', 'NodeGroupInput')] links [('Group Input', 'Group Output')] allow_usage_in_scene_compositor_effect True
NODE EDITOR node_tree <bpy_struct, CompositorNodeTree("Scene Compositor") at 0xc64c58f08>
```

and the picture shows the chooser now reading **Scene Compositor**, a path along the top reading **Scene › Scene Effect › Scene Compositor**, and on the canvas a **Group Input** node whose **Image** output is wired to a **Group Output** node's **Image** input — no Render Layers node. So the New button gave the scene one effect, render and preview switches on, holding a new group of Group Input → Group Output: what the operator `scene.new_compositor_effect_node_group` made in entry 2, and what the 5.3 release note quoted under question 5 describes.

**Entry 39 — how a second Group Output node is marked.** A group of one Render Layers node wired to two Group Output nodes, `Group Output` and `Group Output.001`, given to the scene and shown in the compositor's node editor filling the window. Word for word, what the script printed at each stage:

```
ACTIVE NODE None ; is_active_output: [('Group Output', True), ('Group Output.001', False)]
ACTIVE NODE Group Output ; is_active_output: [('Group Output', True), ('Group Output.001', False)]
ACTIVE NODE Group Output.001 ; is_active_output: [('Group Output', True), ('Group Output.001', False)]
clicked the second Group Output's header at 2170, 1237
ACTIVE NODE Group Output.001 ; is_active_output: [('Group Output', False), ('Group Output.001', True)]
```

The first line is the group as built; in the second and third the script made each node the editor's active node; for the last, a simulated click was made on the second node's header. In the pictures, while `is_active_output` named the first node, a small strip directly above the second node's header carried a round icon with a cross in it and the words **Unused Output**, and the first node carried nothing — including when the script had made the second node the active one. After the click the strip had moved: it sat above the first node, and the second carried nothing. Neither node's own header carried any other control, only the fold arrow and the name. So the live output followed the click, not the script's choice of active node, and the warning is drawn on the one that is not live.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**Label:** documented.

**Answer.** Yes. The scene's Output panel has a checkbox in its header that turns off automatic writing. Blender then relies on File Output nodes in the compositor. If there are none, it reports an error.
- "The automatic saving of renders can be toggled using the checkbox in the panel header." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html
- "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported." — same page.
- "Playblast operations also require an active output method. Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel." — same page.
- "By default, animation renders are saved automatically, while still renders are not." — same page.
- "When enabled (default), animation renders are written automatically. When disabled, Blender will not save render output from the Output panel and instead relies on alternative output methods such as Compositor File Output nodes. If no valid output method is available, rendering operations will report errors." — manual, https://docs.blender.org/manual/en/5.3/render/output/animation.html
- "Blender creates a file for each frame of the animation when Save Output is enabled. If it is disabled, frames will only be written if a Compositor File Output node is configured." — same page.
- `save_output`: "Write frames to disk for animation renders (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- Picture `_images/render_output_properties_output_panel.png` on the Output page: the panel header reads "Output" and **no checkbox is visible** in it. The fields are a path "/tmp/", Overwrite, Placeholders, File Extensions, Cache Result, File Format PNG, Color, Color Depth and Compression. So the picture does not show the switch the text describes.

**Not covered:**
- The API text speaks only of "animation renders". The sources do not say what happens to a single-frame render that is told to save while the switch is off. That covers `-f` ("Render frame <frame> and save it") and `bpy.ops.render.render(write_still=True)` ("Save the rendered image to the output path").
- The sources do not say whether giving `-o` on the command line turns writing back on.
- The sources do not say whether an empty file path is another way to switch writing off.
- The sources do not say how the "error is raised / reported" shows up in a background render, for example as a message, an exit code, or a stop.

**Where I looked:**
- I read the Output, Rendering Animations and Format pages in full.
- I searched the manual and the API for "save output", "save_output", "automatic saving" and "output is disabled".
- I searched the command-line arguments page for "save" and "output".

**Outside the interface:**
- `save_output` is a data property that a script can set.
- The command-line pages list no argument for this switch. `-o, --render-output <path>` is described only as "Set the render path and file name." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). How `-o` and the switch interact is not stated.

**Operating system:** The sources do not say this differs.

**Driven from outside — observed, 2026-10-18.** The switch is honoured in renders started from the command line: with it off, `-a`, `-f 1`, and either with `-o` wrote nothing from the Output panel; with no File Output node Blender printed `ERROR Render output disabled in Output properties and no active compositing File Output nodes`, rendered nothing and exited with status 0 (entry 12 below).

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 1 — the switch is there.** On a fresh file Blender printed `RenderSettings.save_output = True`: the property exists and starts on. (Full printout under question 1.)

**Entry 12 — Save Output off, from the command line; an empty output path; a muted File Output node.** Five files were built and saved by Blender 5.3 itself, each two frames of a 64×64 flat-green scene with Compositing on and a group of Render Layers → Group Output: `saveoff.blend`, with Save Output off, an output path of its own, and Render Layers also feeding a File Output node (set to PNG) aimed at an empty folder; `saveoff_nofo.blend`, the same with no File Output node; `emptypath.blend`, with the output path empty and Save Output on; and `mutedfo.blend` and `unmutedfo.blend`, Save Output on, with a File Output node muted and not muted. (The first attempt to build them stopped at `TypeError: bpy_struct: item.attr = val: enum "PNG" not found in ('OPEN_EXR_MULTILAYER')` — a new File Output node starts as a multi-layer EXR — so no file existed and every run below first came back `ERROR Cannot read file ... No such file or directory`, exit status 1. The builder was changed to set the node's media type to Image first, and everything was run again.) Each file was then rendered from the terminal in the form `blender -b --factory-startup <file> [arguments]`, from a working folder that is not the files' folder, the output folders emptied before each run. Every line Blender printed that names a frame, a save or an error, the exit status, and every file present afterwards — word for word:

```
----- saveoff.blend -o .../t12/cli_saveoff_ -F PNG -a
00:00.331  render           | Rendering frame 1
00:00.465  render           | Saved: '.../t12/saveoff_fileoutput/saveoffBeauty0001.png'
00:00.465  render           | Rendering frame 2
00:00.470  render           | Saved: '.../t12/saveoff_fileoutput/saveoffBeauty0002.png'
EXIT STATUS 0
FILES NOW:
./saveoff_fileoutput/saveoffBeauty0001.png
./saveoff_fileoutput/saveoffBeauty0002.png
----- saveoff.blend -a
00:00.329  render           | Rendering frame 1
00:00.466  render           | Saved: '.../t12/saveoff_fileoutput/saveoffBeauty0001.png'
00:00.466  render           | Rendering frame 2
00:00.471  render           | Saved: '.../t12/saveoff_fileoutput/saveoffBeauty0002.png'
EXIT STATUS 0
FILES NOW:
./saveoff_fileoutput/saveoffBeauty0001.png
./saveoff_fileoutput/saveoffBeauty0002.png
----- saveoff.blend -f 1
00:00.333  render           | Rendering frame 1
00:00.468  render           | Saved: '.../t12/saveoff_fileoutput/saveoffBeauty0001.png'
EXIT STATUS 0
FILES NOW:
./saveoff_fileoutput/saveoffBeauty0001.png
----- saveoff.blend -o .../t12/cli_saveoff_f1_ -F PNG -f 1
00:00.333  render           | Rendering frame 1
00:00.466  render           | Saved: '.../t12/saveoff_fileoutput/saveoffBeauty0001.png'
EXIT STATUS 0
FILES NOW:
./saveoff_fileoutput/saveoffBeauty0001.png
----- saveoff_nofo.blend -a
00:00.330  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
EXIT STATUS 0
FILES NOW:
----- saveoff_nofo.blend -f 1
00:00.332  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
EXIT STATUS 0
FILES NOW:
----- emptypath.blend -a
00:00.328  render           | Rendering frame 1
00:00.430  image            | ERROR Couldn't create directory for file 0001.png: No such file or directory
00:00.430  reports          | ERROR Render error (No such file or directory) cannot save: '0001.png'
EXIT STATUS 1
FILES NOW:
----- mutedfo.blend -a
00:00.331  render           | Rendering frame 1
00:00.458  render           | Saved: '.../t12/mutedfo_main_0001.png'
00:00.458  render           | Rendering frame 2
00:00.462  render           | Saved: '.../t12/mutedfo_main_0002.png'
EXIT STATUS 0
FILES NOW:
./mutedfo_main_0001.png
./mutedfo_main_0002.png
----- unmutedfo.blend -a
00:00.330  render           | Rendering frame 1
00:00.459  render           | Saved: '.../t12/unmuted_fileoutput/unmutedfoBeauty0001.png'
00:00.460  render           | Saved: '.../t12/unmutedfo_main_0001.png'
00:00.460  render           | Rendering frame 2
00:00.464  render           | Saved: '.../t12/unmuted_fileoutput/unmutedfoBeauty0002.png'
00:00.464  render           | Saved: '.../t12/unmutedfo_main_0002.png'
EXIT STATUS 0
FILES NOW:
./unmuted_fileoutput/unmutedfoBeauty0001.png
./unmuted_fileoutput/unmutedfoBeauty0002.png
./unmutedfo_main_0001.png
./unmutedfo_main_0002.png
```

(`...` stands for the work folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/The compositor/out/5.3`, and the file names for their full paths under `.../blends/5.3/`; both are printed in full in the hand-test list.)

**This answers the four "Not covered" points above, for a render with no window.**

- **A single-frame render told to save, with the switch off, did not write from the Output panel.** `-f 1` wrote only the File Output node's file — not to the stored path.
- **`-o` on the command line did not turn writing back on**, for `-a` or for `-f 1`: no `cli_saveoff_` file appeared, while the File Output node wrote its frames.
- **An empty output path is not a way of switching writing off**: the render stopped after frame 1 with `Couldn't create directory for file 0001.png`, wrote nothing, and Blender exited with status 1.
- **How the error shows up**: with the switch off and no File Output node, Blender printed `ERROR Render output disabled in Output properties and no active compositing File Output nodes`, rendered no frame, wrote nothing — and exited with status **0**, for `-a` and for `-f 1` alike.

And a muted File Output node wrote nothing, while the scene's own output still wrote.

**Entry 6, case 2 — taking the Group Output node out of the group.** Written out in full under question 1: with a group given and no Group Output node in it, the frame written was transparent black, not the raw render.

**Entry 34 — the Output panel and its header checkbox, in the window.** Started and photographed as described in question 1's window-test section. The Properties editor filled the window on its Output tab. The Output panel's header reads, left to right, the fold arrow, **a ticked checkbox** with no words of its own, and **Output** — the switch the manual's text describes and its picture lacks. Below it, top to bottom: a path field reading `/tmp/` with a folder button; **Saving** with **File Extensions** (ticked) and **Cache Result** (unticked); **Media Type** reading **Image**; **File Format** reading **PNG (.png)**; **Color** as BW / RGB / RGBA with RGBA chosen; **Color Depth** as 8 / 16 with 8 chosen; **Compression** reading 15%; **Image Sequence** with **Overwrite** (ticked) and **Placeholders** (unticked); and two folded sub-panels, **Color Management** and **Pixel Density**. The script printed the Media Type choices as `MEDIA TYPE ITEMS: [('IMAGE', 'Image'), ('MULTI_LAYER_IMAGE', 'Multi-Layer EXR'), ('VIDEO', 'Video')]`.

**The checkbox's tooltip.** A first try moved the simulated pointer onto the checkbox in one jump and photographed it three seconds later: no tooltip appeared. A second try moved it there in six small steps and photographed it about three and about seven seconds later; both pictures show a tooltip under the checkbox reading, in two lines:

```
Save Output
Write frames to disk for animation renders.
```

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**Ambiguity:** "a compositor" can be read two ways.
- (a) The node group itself.
- (b) The scene's arrangement for compositing: which groups it uses, and its on/off switches.

The sources treat these two differently, so I answer both.

**Label:** documented (each piece); inferred (the combined answer).

**Answer.** The arrangement belongs to one scene. The node groups it uses are file-wide data. More than one scene can share them, and other files can link or append them.

Per scene:
- `compositor_effects`: "Compositor effects for this scene" — API, https://docs.blender.org/api/5.3/bpy.types.Scene.html
- RenderSettings is described as "Rendering settings for a Scene data-block". It holds `use_compositing`, `compositor_device`, `compositor_precision` and the compositor denoise settings — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- "Node Tree — Chooses which node group (compositor node tree) to display and edit. Each Scene can have its own compositor node tree. You can also create and switch to custom node groups for reuse." — manual, https://docs.blender.org/manual/en/5.3/editors/compositor.html
- "Pinned — When enabled, the editor always displays the selected node tree, regardless of the active scene." — same page.

File-wide:
- "This adds a compositing node tree data block to the scene, which can then be reused across different scenes or linked/appended from other blend-files." — manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html
- `CompositorNodeTree(NodeTree)`, "base classes — bpy_struct, ID, NodeTree" — API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeTree.html. ID is the data-block type.
- `node_groups`: "Node group data-blocks" — API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html
- Compositor node groups are also used outside scenes: "compositor node groups can be used directly in the Video Sequencer through compositor strips and compositor modifiers." — manual, https://docs.blender.org/manual/en/5.3/compositing/usage.html

**Inferred:**
- The decision to composite, and the choice of node groups, is made scene by scene.
- A node group used by one scene's compositor can also be used by another scene and by Sequencer strips, so a change to that group reaches all of them.
- The step: the groups are file-level data-blocks referenced from each scene's effect list. The sources do not describe what editing a shared group does.
- The API's file-level data (BlendData) has no compositor switch.

**Not covered:**
- What happens when two scenes share one node group but have different switches. Presumably each scene uses its own switches, but this is not stated.
- Whether the effect list itself can be shared between scenes. Nothing found says it can.

**Where I looked:** the Scene, RenderSettings, BlendData, CompositorNodeTree, SceneCompositorEffect and SceneCompositorEffects API pages; the Compositor editor page; the compositing introduction and usage pages. I searched the manual for "each scene", "other scene", "another scene" and "different scene".

**Outside the interface:**
- The same data can be reached from a script: the node groups through `bpy.data.node_groups` and the per-scene list through `scene.compositor_effects`.
- `-S, --scene <name>`: "Set the active scene <name> for rendering." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). Inferred: in a command-line render, the scene picked this way decides whose compositor arrangement applies.

**Operating system:** The sources do not say this differs.

**Driven from outside — observed, 2026-10-18.** `-S` picked the scene and that scene's own group ran (entry 14); one group served two scenes, each following its own switch, with no window (entry 13).

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 1 — where the pieces live.** On a fresh file the scene carries `compositor_effects` (empty) and the older `compositing_node_group` (None), and the file holds no node group; a group made with `bpy.data.node_groups.new` is file-level data that a scene's effect then points at. (Full printout under question 1.)

**Entry 13 — two scenes sharing one node group.** Two 64×64 scenes, A rendering blue and B rendering green. One group — a Render Layers node with its Scene set to A → Invert Color → Group Output — was given to both through `compositing_node_group`. Each scene was rendered with Compositing on in both; then again with A's on and B's off. Word for word:

```
CONTROL A alone, compositing off -> [0.0706, 0.3333, 0.8196, 1.0]
CONTROL B alone, compositing off -> [0.4235, 0.7686, 0.3137, 1.0]
RESULT sharing one tree between two scenes -> True
   effects: A [('Scene Effect', 'Shared')] B [('Scene Effect', 'Shared')]
RESULT shared tree, rendering scene A -> [1.0, 0.9608, 0.6353, 1.0]
RESULT shared tree, rendering scene B -> [1.0, 0.9608, 0.6353, 1.0]
RESULT shared tree, A compositing on, rendering A -> [1.0, 0.9608, 0.6353, 1.0]
RESULT shared tree, B compositing off, rendering B -> [0.4235, 0.7686, 0.3137, 1.0]
```

`[1.0, 0.9608, 0.6353, 1.0]` is A's blue inverted. One group was accepted by both scenes, each getting an effect of its own pointing at it. With both switches on, both scenes wrote the same picture — A's render through the shared group — so the Render Layers node in a shared group follows the scene named on the node, not the scene being rendered. **This settles the "Not covered" point above about two scenes sharing a group with different switches**: with B's Compositing off, rendering B wrote B's own raw green while A, with its switch on, still wrote the shared group's result. Each scene's own switch decided for it.

**Entry 14 — whether choosing a scene from the command line picks that scene's group.** `scenes.blend`, built and saved by Blender 5.3: scene A rendering blue with its own group inverting it, scene B rendering green with its own group inverting it, Compositing on in both, A active. Rendered from the terminal as `blender -b --factory-startup scenes.blend -S A -o <folder>/scene_A_ -F PNG -f 1` and the same with `-S B`, both exiting with status 0, and the two frames read back. Word for word:

```
READ scene_A_0001.png -> 64x64 first pixel RGBA [1.0, 0.9608, 0.6353, 1.0]
READ scene_B_0001.png -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0]
```

A came out as its blue inverted, B as its green inverted: `-S` picked the scene, and that scene's own group was the one that ran. This is the step marked inferred under *Outside the interface* above, now observed.

**Entry 35 — the window.** Written out under question 1's window tests: the compositor's node editor offers a dropdown reading **Scene** beside its menus and a node-group chooser with a **New** button; in a fresh file nothing is chosen, and pressing New gave the scene an effect called **Scene Effect** holding a new file-level group called **Scene Compositor**, shown as the path **Scene › Scene Effect › Scene Compositor**.

**Entry 38 — the Render menu.** Written out under question 7: in a fresh file it offers Render Image, Render Animation, Render Audio…, View Render, View Animation and Lock Interface; once the workspace's Sequencer Scene is a scene other than the active one, it also offers Render Sequencer Image and Render Sequencer Animation.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**Ambiguity:** a scene can "take in" another scene's render in two ways the sources describe.
- (a) A Render Layers node in its compositor that is set to the other scene.
- (b) A Scene strip in its sequencer.

I answer both.

**(a) Render Layers node set to another scene.**

**Label:** inferred (that only raw render passes arrive, not the composited result); not in the sources (whether the second scene's compositor runs anyway).
- "The Render Layers node renders a View Layer from a scene and outputs its image and any enabled render passes to the compositor." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html
- "Hint / To use the composited output of another scene instead of its raw render passes, first render that scene to a multilayer image format such as OpenEXR. The resulting image can then be loaded using the Image node." — same page.
- `CompositorNodeRLayers`: "Input render passes from a scene render" — API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeRLayers.html
- "The compositor now renders scenes referenced inside node groups." — release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/
- The step: to get the other scene's composited output, the manual sends you to a separate render and an Image node. So the Render Layers node itself delivers that scene's raw passes. The release note says the referenced scene is *rendered*. It says nothing about that scene's compositor.

**(b) Scene strip in the sequencer.**

**Label:** documented (that a Scene strip can output the compositor's result when compositing is enabled); inferred (that the second scene's compositor runs in that case); not in the sources (which scene's Compositing switch controls this).
- "Pipeline – Sequencer / … If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html
- "Scene strips do not render individual Render Passes; only the Combined render pass will be used." — manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/scene.html
- A Scene strip can also take the other scene's sequencer timeline instead of its render: "Sequencer: Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene)." — same page.
- Inferred: when "Compositing is also enabled", the strip shows the other scene's compositor output, so that compositor does run.
- The sentence does not say whose Compositing switch it means: the scene doing the rendering, or the strip's scene. I cannot settle that from the sources.

**Not covered:**
- Whether, in case (a), the other scene's compositor runs anyway and causes side effects, such as its File Output nodes writing files.
- How the other scene's effects-stack switches (`enable_for_render`) are treated when it is rendered this way.
- What happens with chains: a scene that takes in a scene that takes in a third.

**Where I looked:**
- I read the Render Layers node, Post Processing, Scene strip, Sequencer Scene, View Layers introduction and Compositor System pages.
- I searched the manual for "composited output", "raw render", "output of the compositor", "compositor output", "indirectly", "rendered indirectly", "nested scene", "recursive", "another scene" and "other scene".
- The only "indirectly rendered" wording in any source is the API's `use_render_cache`: "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)" (https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html). That is about the render cache, not about compositors.

**Outside the interface:** The sources make no difference between inside and outside for either route.

**Operating system:** The sources do not say this differs.

**Driven from outside — observed, 2026-10-18.** Both routes were tested with no window (entries 15 and 16) and behaved as written below.

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 15 — (a), a Render Layers node pointing at another scene.** Two 64×64 scenes. B renders a flat green; its group sends Render Layers through Invert Color to its Group Output — so B's own result is the green inverted — and also straight into a File Output node aimed at an empty folder, so that any work B's group does leaves a file behind. A renders blue on its own, and its whole group is one Render Layers node with Scene set to B, wired to A's Group Output. A was rendered with B's Compositing switch on and then off, B's folder checked each time; then three controls. Word for word:

```
RESULT B.use_compositing=True -> A wrote 64x64 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=True -> files in B's File Output folder: []
RESULT B.use_compositing=False -> A wrote 64x64 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0]
RESULT B.use_compositing=False -> files in B's File Output folder: []
CONTROL A alone, compositing off -> [0.0706, 0.3333, 0.8196, 1.0]
CONTROL B alone, compositing off -> [0.4235, 0.7686, 0.3137, 1.0]
CONTROL B alone, compositing on  -> [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: ['B_.exr']
```

A wrote B's raw green both times, not B's inverted result. B's File Output folder stayed empty both times, though rendering B directly fills it. **This settles the "Not covered" point above about side effects**: on this evidence B's group did no work when A took B in through a Render Layers node, whichever way B's switch was set.

**Entry 16 — (b), a Scene strip pointing at another scene.** The same scene B. Scene A takes B in through a Scene strip in A's own sequencer, with A's Sequencer switch on and A's own group a plain Render Layers → Group Output.

*First run, all four combinations of the two Compositing switches in one Blender, one after another,* B's folder emptied before each. Word for word:

```
STRIP B_strip SCENE scene B frames 1 251
RESULT A.use_compositing=True  B.use_compositing=True  -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: ['B_.exr']
RESULT A.use_compositing=True  B.use_compositing=False -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=False -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: []
RESULT both on, B's effect enable_for_render=False -> first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: []
CONTROLS: A raw blue, B raw green and B inverted are in entry 15's CONTROL lines; A here:
CONTROL A alone, sequencer off, compositing off -> [0.0706, 0.3333, 0.8196, 1.0]
```

After the first render every frame came out inverted, whatever the switches, and B's File Output node never wrote again. Because only the first render of the session differed from the rest, the test was run a second way.

*Second run, one combination per Blender*, each a fresh process building the same two scenes, with a fifth case — both switches on and B's effect's render switch off. Word for word:

```
RESULT A.use_compositing=True  B.use_compositing=True  B effect render switch=True  -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: ['B_.exr']
RESULT A.use_compositing=True  B.use_compositing=False B effect render switch=True  -> 64x64 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=False B.use_compositing=True  B effect render switch=True  -> 64x64 first pixel RGBA [0.9333, 0.702, 0.9647, 1.0] ; files in B's File Output folder: ['B_.exr']
RESULT A.use_compositing=False B.use_compositing=False B effect render switch=True  -> 64x64 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
RESULT A.use_compositing=True  B.use_compositing=True  B effect render switch=False -> 64x64 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0] ; files in B's File Output folder: []
```

**This settles what (b) above could not.** With each render in a fresh Blender, "If Compositing is also enabled" meant the **strip's own scene's** switch: the written frame followed B's switch and not A's, and B's File Output node wrote exactly when B's switch was on, so B's group really ran then. **And it settles how B's effects-stack switch is treated** (a "Not covered" point above): with B's Compositing on but its one effect's render switch off, the strip carried B's raw green and B's File Output node did not write. The first run records something separate, and it is recorded rather than explained: within one Blender session, once A had been rendered with B's group running, later renders of A carried that same inverted picture and B's group left no further file, whatever the switches.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

**Label:**
- (5a) documented.
- (5b) inferred (by entry node type); not in the sources (by the data itself).
- (5c) inferred, built from documented structure.

**Ambiguity:** "split it into more than one" can be read two ways.
- (i) One output feeding several nodes (branching).
- (ii) A node that breaks the picture into parts, such as colour channels.

I answer (i) mainly and note (ii).

**5a. What it arrives as.**
- Through the Group Input node: "The sockets on the Group Input node provide the inputs to the compositor. By default, the Image socket contains the rendered image, equivalent to the Combined render pass. Additional input sockets can be added to expose custom inputs to the node tree." — manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html
- "The default compositor node tree now uses a simple **Group Input** connected to a **Group Output** with no **Render Layers** node." — release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/
- Or through a Render Layers node: "Image — The combined rendered image. / Alpha — The alpha channel of the rendered image. / Additional outputs corresponding to the render passes enabled for the selected view layer." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html
- Kind of data. "Color — A 4D vector storing the Red, Green, Blue, and Alpha of the color. The color is free form and does not conform to a specific color space or alpha storage model" — manual, https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html
- Pictures `_images/node-types_NodeGroupInput.webp` and `_images/node-types_CompositorNodeRLayers.webp`, on the Group Input and Render Layers pages: on both nodes the "Image" socket is yellow. On Render Layers, the "Alpha" socket is grey.
- The socket-colour legend says "Color (yellow) — Indicates that the socket accepts/produces color information." and "Float (light gray)" — manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/parts.html
- Precision. The two sources disagree, so I give both:
  - "Three types of data exist, all of which are stored in half precision formats" — manual, https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html
  - `compositor_precision`: "AUTO — Auto – Full precision for final renders, half precision otherwise." — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- Size and placement: "The operation domain of input nodes like the Image node is a domain with an identity transformation and the same size as their outputs" — manual, compositor_system page.

**5b. Can it be told apart?**

By where it enters, yes. Inferred: the entry nodes are different node types, and a script can read a node's type.
- `NodeGroupInput`: "Expose connected data from inside a node group as inputs to its interface" — API, https://docs.blender.org/api/5.3/bpy.types.NodeGroupInput.html
- `CompositorNodeRLayers`: "Input render passes from a scene render". It has a `scene` property — API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeRLayers.html
- `type`: "Legacy unique node type identifier, redundant with bl_idname property" — API, https://docs.blender.org/api/5.3/bpy.types.Node.html

Things that blur this:
- A tree may hold several Group Input nodes: "You can have multiple of these nodes in your tree to keep it clean" — manual, https://docs.blender.org/manual/en/5.3/compositing/types/groups.html
- Inside a nested node group, a Group Input node carries whatever the parent passes in: "When the node group is used in another compositor tree, the sockets defined in Group Input appear as input sockets on the Group node itself." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/group_input.html. Inferred: a Group Input inside a nested group is not, on its own, proof that the render is there.

By the data itself: **not in the sources.**
- The sources describe no marker that travels with a picture and says where it came from.
- Socket inspection lists only "Resolution", "Transform", "Sampling" and "Precision" (compositor_system page).
- The data types are only Float, Vector and Color.

**5c. Can it be followed forward?** Yes. Inferred from these documented pieces.

Links can be read:
- `NodeLink` has `from_node`, `from_socket`, `to_node` and `to_socket` — API, https://docs.blender.org/api/5.3/bpy.types.NodeLink.html
- `links` on a socket: "List of node links from or to this socket." — API, https://docs.blender.org/api/5.3/bpy.types.NodeSocket.html
- `NodeTree.links` — API, https://docs.blender.org/api/5.3/bpy.types.NodeTree.html

Branching (reading i):
- "Multiple branches can process the same input independently before being combined later in the node tree." — manual, compositing introduction.
- Reroute "supports one input connection while allowing multiple output connections" — manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/types/layout/reroute.html
- `NodeReroute`: "A single-socket organization tool that supports one input and multiple outputs" — API, https://docs.blender.org/api/5.3/bpy.types.NodeReroute.html

Nodes that pass it along unchanged:
- Muted nodes: "Muting a node removes its contribution to the node tree, and makes all links pass through it without change." — manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/editing.html
- `internal_links`: "Internal input-to-output connections for muting" (API, Node page). `NodeInternalLink`: "Internal link used by muted nodes to connect input to output sockets" (API, https://docs.blender.org/api/5.3/bpy.types.NodeInternalLink.html).
- Muted links: "A muted link acts as though it's no longer there … When muting links on the input side of a reroute node, the links on its output side will be muted too." — manual, editing page. `is_muted`: "Link is muted and can be ignored" — API, NodeLink page.
- Switch: "Output — One of the two inputs without any modifications." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/utilities/switch.html. Index Switch: "without any modification". Menu Switch: "unchanged".
- Into and out of node groups: `CompositorNodeGroup.node_tree` (API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeGroup.html). "Data enters through a Group Input node, flows through the internal node network, and leaves through a Group Output node." — manual, compositing introduction.

Reading (ii):
- "The Separate Color Node splits an image into its composite color channels." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/color/separate_color.html
- Its outputs are links like any other, so they can be followed. They carry channels, not the picture unchanged.

**Not covered:**
- When a Switch's condition comes in through a link, the choice is made while the tree is worked out. The sources describe no way to read that choice back from outside.
- What `is_valid` on a link means in practice ("Link is valid").
- How hidden links (`is_hidden`, "Link is hidden due to invisible sockets") behave.

**Where I looked:**
- Pages: compositing introduction, Compositor System, Group Input, Group Output, Render Layers, compositor Groups page, Node Parts, Node Editing, Reroute, Switch, Index Switch, Menu Switch, Separate Color.
- API pages: NodeLink, NodeSocket, Node, NodeTree, NodeReroute, NodeGroupInput, NodeGroupOutput, NodeInternalLink, CompositorNodeGroup, CompositorNodeRLayers.
- I searched the manual for "reroute" and "mute".

**Outside the interface:**
- Everything in 5b and 5c is read through the Python data API. Inferred: a script run in a background render (`-b` with `--python`) can do it.
- On Blender used as a Python module, the API says it is "for the most part … equivalent to running a script in background-mode" (https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html).

**Operating system:** The sources do not say this differs.

**Driven from outside — observed, 2026-10-18.** The node identifiers, the link walk, the muted flags and the internal links were all read with no window (entries 2, 23, 27, 28), and read the same with a window open (entry 40). Socket inspection is drawn in the window: no property a script can reach names anything it shows, with or without a window (entries 9 and 40), and what it draws was seen only in the window (entry 37).

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 2 — the Split node, the other reading of "split it into more than one".** Added to a compositor group, it printed `ADD CompositorNodeSplit -> inputs=['Position', 'Rotation', 'Image', 'Image'] outputs=['Image']`: two picture inputs and one picture output, so something can be followed forward past it. The Switch node printed `ADD CompositorNodeSwitch -> inputs=['Switch', 'Off', 'On'] outputs=['Image']`, and the Group Input node, added to a group with no inputs defined, printed `ADD NodeGroupInput -> inputs=[] outputs=['']`. The full printout is under question 1.

**Entry 6, case 6 — the render arrives at a Group Input node.** Written out in full under question 1: a group of Group Input → Invert Color → Group Output, with no Render Layers node, wrote the render inverted.

**Entry 23 — telling the render apart, and following it forward, with no window.** One group holding a Render Layers node named `TheRender`, an Image node `TheImage`, a Group Input `TheInput`, a Reroute `TheReroute`, a Blur node `MutedBlur` that was muted, a Viewer `TheViewer`, and the Group Output. The render's Image socket goes into the Reroute; the Reroute feeds the muted Blur, which feeds the Group Output; the Reroute also feeds the Viewer through a link that was itself muted. Word for word:

```
--- identifiers
node TheRender  bl_idname=CompositorNodeRLayers    type=R_LAYERS       bl_static_type=R_LAYERS       name=TheRender
node TheImage   bl_idname=CompositorNodeImage      type=IMAGE          bl_static_type=IMAGE          name=TheImage
node TheInput   bl_idname=NodeGroupInput           type=GROUP_INPUT    bl_static_type=GROUP_INPUT    name=TheInput
RenderLayers.scene = 'Scene' ; RenderLayers.layer = 'ViewLayer'
RenderLayers output sockets = ['Image', 'Alpha']
--- every link in the tree
LINK TheRender.Image -> TheReroute.Input  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> MutedBlur.Image  is_muted=False is_valid=True is_hidden=False
LINK MutedBlur.Image -> Group Output.Image  is_muted=False is_valid=True is_hidden=False
LINK TheReroute.Output -> TheViewer.Image  is_muted=True is_valid=True is_hidden=False
--- walking forward from the Render Layers Image socket
WALK TheRender.Image -> TheReroute.Input (muted link=False, muted node=False)
  WALK TheReroute.Output -> MutedBlur.Image (muted link=False, muted node=True)
    WALK MutedBlur.Image -> Group Output.Image (muted link=False, muted node=False)
  WALK TheReroute.Output -> TheViewer.Image (muted link=True, muted node=False)
--- what the muted node exposes
MutedBlur.mute = True ; internal_links = [('Image', 'Image')]
--- is_active_output on output nodes
ACTIVE Group Output.is_active_output = True
```

The three entry nodes carry three different identifiers, so a program can tell the Render Layers node, an Image node and a Group Input apart. The walk forward from the render's socket reached both ends through the Reroute, with the muted link and the muted node each flagged on the way, so the trace this answer marks inferred does work with no window. The muted node names which input it passes, and `is_active_output` on the Group Output reads True.

**Entry 39 — which output node is live, on screen and in the file.** Written out under question 1's window tests: with two Group Output nodes, the one that is not live carries an **Unused Output** warning above its header, `is_active_output` reads True on the live one, and a simulated click on the other's header moved both.

**Entry 27 — walking into a node group.** Written out in full under question 8: a walk forward from a picture reaches the Group node in the outer tree and carries on only by stepping into that node's own group; no outer link reaches inside.

**Entry 28 — the whole graph read from a saved file by a second Blender**, with no window: every node, every link with both ends, and each node's internal link came back. Written out in full under question 8.

**Entry 37 — what socket inspection shows when a socket is hovered (in the window).** Blender 5.3.0 Beta was started from a terminal with its window open, `--factory-startup`, its own settings folders and `--enable-event-simulate`; a script on Blender's own timer closed the splash with a simulated Escape, built the group below, gave it to the scene, filled the window with the compositor's node editor showing it, rendered one 320×240 frame of the factory scene from the script, and then moved a simulated pointer onto each place in turn — in six small steps, the pointer first parked on empty canvas — and photographed the window after about 1.5 and about 3 seconds, with the window made to redraw just before each photograph. Nobody touched the Mac; the photographs were kept only in the test's work folder outside this project, so what they show is written out here. The group: a Render Layers node and an Image node holding a 64×64 picture (`picA.exr`), each feeding its own Viewer, and the Render Layers node also feeding the Group Output. In the photographs both the Render Layers node and the Image node carry an open-eye icon at the right end of their headers. What appeared:

- **The Render Layers node's Image output socket**: nothing at 1.5 seconds; at 3 seconds, in a run of its own and again later in the same run, a display reading, line by line:

```
Image
Value: 0 0 0 0 (Linear)
Type: Float Color
[a colour swatch, black on the left half, a transparency checkerboard on the right]
Display RGB: 0.000  0.000  0.000
HSV:         0.000  0.000  0.000
Alpha:       0.000
Hex:         #00000000
```

- **The Image node's Image output socket**: a box reading only `Image` at 1.5 seconds, gone at 3 seconds.
- **The middle of the link out of either node**: nothing, at either time.

**Not one line names where the picture came from** — no node, scene, layer or file. The display that appeared was the one-value kind, not one listing a resolution, a transform, a sampling or a precision, although a frame had been rendered from the script before the hover. An earlier run of the same set-up, which took one photograph per hold and did not force a redraw, caught the same Render Layers display only in the photograph taken after the pointer had already moved on to the link, and a simulated click on the Render Layers node's eye icon in that run turned it to a closed-eye shape; what the display shows with the eye closed was not caught in either run.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Label:** documented (a); inferred (b, d); not in the sources (whether the written pixel dimensions are altered for c, e, and f; the size written by File Output nodes in d).

**Answer.** Yes for render region with cropping (documented). Stereo output is inferred to produce a larger file when unsqueezed formats are used. Whether the GPU compositor's 8k limit, video codec divisible-by-2 rules, or pixel aspect pre-distortion change the written pixel dimensions is not in the sources. Compositor nodes do not change the main output's size (inferred from domain rules); what size File Output nodes write is not in the sources.

**a) Render Region with "Crop to Render Region".** Documented.
- "Render Region — Renders just a portion of the view instead of the entire frame." / "Crop to Render Region — Crops the rendered image to the size of the render region, instead of rendering a transparent background around it." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/format.html
- `use_border`: "Render a user-defined render region, within the frame size (default False)". `use_crop_to_border`: "Crop the rendered frame to the defined render region size (default False)". `border_min_x` and the other three edges are each "(in [0, 1] …)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- "If you define one while in Camera View, it will apply not just to the viewport, but also to the final render." … "However, they always affect the final render." — manual, https://docs.blender.org/manual/en/5.3/editors/3dview/navigate/regions.html

**b) Stereo and multi-view output.** Inferred.
- "By default the system saves each view as an individual file, thus generating as many files as views to be rendered." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html
- "The Top-Bottom compressed will discard half of your vertical resolution data." / "For the Stereo 3D formats the only lossless options are Top-Bottom and Side-by-Side without the Squeezed Frame option." — same page.
- `use_squeezed_frame`: "Combine both views in a squeezed image (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.Stereo3dFormat.html
- The step: an unsqueezed Side-by-Side or Top-Bottom file keeps both full views without loss, so it must be larger than one view. The sources give no dimensions.

**c) The GPU compositor's size limit.** The limit is documented; its effect on the written file is not in the sources.
- "General hardware limitations apply when using the GPU compositor." … "Resolution — Images with resolution larger than 8k are not supported and are cropped/scaled." — manual, https://docs.blender.org/manual/en/5.3/compositing/limitations.html
- `compositor_device`: "Set how compositing is executed (default 'GPU')" — API, RenderSettings page.
- The sources do not say whether "cropped/scaled" changes the written file's pixel size or only its content. They also do not say whether "8k" means width, height, or either.

**d) Compositor nodes do not set the main output's size.** Inferred.
- "The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size." — manual, https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html
- "The views to render are defined in the current scene views, in a similar way as you define the composite output resolution in the current scene render panel, regardless of the Image nodes resolutions or Render Layers from different scenes." — manual, stereoscopy usage page; the same sentence is on https://docs.blender.org/manual/en/5.3/compositing/types/utilities/switch_stereo_view.html
- The step: Scale and Crop nodes change a picture's size inside the tree, but the final composite takes the scene render size.
- For the **File Output node**, the sources do not state the size it writes. It is listed among "Output Nodes", and the rule above is worded for "output nodes like the Viewer node". Whether File Output follows that rule is not stated.

**e) Video codecs.** The restriction is documented; what Blender does about it is not.
- "Video Output Size — Some codecs impose output size restrictions. For example, H.264 requires both width and height to be divisible by 2." — manual, https://docs.blender.org/manual/en/5.3/files/media/video_formats.html
- The sources do not say whether Blender adjusts the size or fails.

**f) Pixel aspect.** Not settled.
- "Aspect X, Y … This will pre-distort the images which will look stretched on a computer screen, but which will display correctly on a TV set." — manual, Format page.
- The sources do not say whether this changes the pixel count written.

**Ruled out:**
- Pixel density: "The pixel-density is meta-data which doesn't impact the quality of the image" (Output page).
- EEVEE Overscan: "Percentage of the render size to add to the internal render buffer" (https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/film.html). This is an internal buffer; the sources say nothing about it changing the written size.
- Sequencer proxy size: "Controls the preview resolution" (https://docs.blender.org/manual/en/5.3/editors/video_sequencer/preview/sidebar.html).
- Sequencer "Set Render Size": "Sets the render resolution and aspect to match the strip's resolution." It works by changing the scene's own resolution (https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/editing.html).
- Sequencer strips: they are fitted to "the scene's Resolution" (https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/movie.html).

**Not covered:**
- Whether the Burn-into-Image stamp changes the size.
- Whether anything else in the sequencer changes the output size.
- The exact dimensions for any of (b) to (f).

**Where I looked:**
- Pages: Format, Output, Compositor System, Limitations, the Scale and Crop node pages, stereoscopy pages, Switch View, Render Region, video and image format pages, Working Limits, EEVEE Film, camera Safe Areas, Sequencer preview sidebar, movie and image strip pages, Sequencer editing page.
- Searches: "overscan", "image size", "output size", "output resolution", "render size", "resolution", "squeeze", "dimension", "divisible", "65536", "16384".
- I searched the API for "squeeze".

**Outside the interface:**
- The region and crop are scene properties, so a script can set them.
- The command-line pages list no argument for size. I searched them for "resolution" and "percent".

**Operating system:** The sources do not say the size rules differ. The GPU limit sits under "Hardware Limitations". Inferred: it may depend on the graphics hardware, but nothing is said per operating system. The only operating-system note there, "TDR on Windows", is about timeouts, not size.

**Driven from outside — observed, 2026-10-18.** Every size result below was got with no window: the crop, stereo, the File Output node, the Crop node, pixel aspect, Burn Into Image, the GPU compositor above 8k, and H.264 at an odd size (entries 10, 20, 21, 22).

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting. The above-8k result (entry 10) is Apple's own graphics only; no other make of card could be reached.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`. The second-pass document on this question points here for these results.

**Entry 10 — frames above 8k, on each compositor device.** A Workbench scene rendering one flat green, with a group of Render Layers → Invert Color → Group Output, written as PNG at 9000×400, at 400×9000 and at 10000×5000, once with the compositor device on CPU and once on GPU (Apple's own graphics). The written file's size was read from its own header. The first two sizes were run four times, each run reading more of the picture than the last, because the first reading (the first pixel only, as in the earlier versions' form of this test) could not see what the later ones found. An earlier attempt at the first command failed before Blender rendered anything, `ValueError: invalid literal for int() with base 10: '9000 400'`, because the shell passed the two sizes as one word; it was retyped with the sizes as separate words. Word for word, the first reading:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.9333, 0.702, 0.9647, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.9333, 0.702, 0.9647, 1.0]
RESULT 400x9000 scene, compositor_device=CPU -> written 400x9000 first pixel [0.9333, 0.702, 0.9647, 1.0]
RESULT 400x9000 scene, compositor_device=GPU -> written 400x9000 first pixel [0.9333, 0.702, 0.9647, 1.0]
```

The second reading added the pixel in the far corner:

```
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 first pixel [0.9333, 0.702, 0.9647, 1.0] last pixel (far corner) [0.9333, 0.702, 0.9647, 1.0]
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 first pixel [0.9333, 0.702, 0.9647, 1.0] last pixel (far corner) [0.4235, 0.7686, 0.3137, 1.0]
RESULT 400x9000 scene, compositor_device=CPU -> written 400x9000 first pixel [0.9333, 0.702, 0.9647, 1.0] last pixel (far corner) [0.9333, 0.702, 0.9647, 1.0]
RESULT 400x9000 scene, compositor_device=GPU -> written 400x9000 first pixel [0.9333, 0.702, 0.9647, 1.0] last pixel (far corner) [0.4235, 0.7686, 0.3137, 1.0]
```

`[0.9333, 0.702, 0.9647, 1.0]` is the green inverted; `[0.4235, 0.7686, 0.3137, 1.0]` is the raw green, un-inverted. The third reading looked for where the colour changes along the edges; the fourth counted every pixel of each kind and gave the box the un-inverted ones fill (x and y counted in pixels from the bottom-left corner). The fourth, word for word:

```
   pixels inverted 3600000, un-inverted (raw green) 0, other 0; un-inverted pixels fill x 9000..-1, y 400..-1
RESULT 9000x400 scene, compositor_device=CPU -> written 9000x400 ...
   pixels inverted 3276800, un-inverted (raw green) 323200, other 0; un-inverted pixels fill x 0..8999, y 364..399
RESULT 9000x400 scene, compositor_device=GPU -> written 9000x400 ...
   pixels inverted 3600000, un-inverted (raw green) 0, other 0; un-inverted pixels fill x 400..-1, y 9000..-1
RESULT 400x9000 scene, compositor_device=CPU -> written 400x9000 ...
   pixels inverted 3276800, un-inverted (raw green) 323200, other 0; un-inverted pixels fill x 0..399, y 8192..8999
RESULT 400x9000 scene, compositor_device=GPU -> written 400x9000 ...
```

(`x 9000..-1` means there were none.) The third reading had printed, for the GPU file at 400×9000, `up the left column: rows where the colour changes: [8192]`. Then the 5.3 researcher's own size, one device per Blender, counted the same way:

```
RESULT 10000x5000 scene, compositor_device=CPU -> written 10000x5000 first pixel [0.9333000183105469, 0.7020000219345093, 0.9646999835968018, 1.0]
   pixels inverted 50000000, un-inverted (raw green) 0, other 0; un-inverted pixels fill nothing
RESULT 10000x5000 scene, compositor_device=GPU -> written 10000x5000 first pixel [0.9333000183105469, 0.7020000219345093, 0.9646999835968018, 1.0]
   pixels inverted 40960000, un-inverted (raw green) 9040000, other 0; un-inverted pixels fill x 0..9999, y 4096..4999
```

**This answers what (c) above leaves open, for this Mac.** Every file was written at the full scene size on both devices: above 8k, the written pixel size did not change. **But on GPU the compositor's result did not fill the frame above 8k.** On CPU every pixel was inverted at all three sizes. On GPU exactly 3,276,800 pixels were inverted at 9000×400 and at 400×9000, and 40,960,000 at 10000×5000 — in each case 8,192 times the shorter side — and the rest of the frame, the top of it, held the raw render. Neither a black frame nor a smaller file came out; the file is full size and part of it is not composited. The limit applies to the height as well as the width. This is on Apple's own graphics; another make of card cannot be reached here.

**Entry 20 — what decides the pixel size of what gets written.** A 400×200 Workbench scene with a cube in view, Compositing on, its group Render Layers → Group Output, written as PNG, rendered case by case: (1) a Scale node set to Absolute 120×90 between the Render Layers node and a File Output node (set to PNG), the unscaled render going to the scene's own output; (1b) the same Scale node set to Relative 0.5×0.5; (1c) an 800×600 picture from an Image node straight into a File Output node; (2) the render region set to the middle half, 0.25 to 0.75 both ways, with Crop to Render Region off and then on; (3) stereoscopy with both views written into one file, Side-by-Side and Top-Bottom, squeezed and not; (4) a Crop node with Alpha Crop off, set to a 200×100 region, between the Render Layers node and the Group Output; (5) pixel aspect X set to 2; (6) Burn Into Image on. Word for word:

```
RESULT case1 File Output after Scale Absolute 120x90 -> scaledBeauty.png is 122x90 ; scene output 400x200 (scene is 400x200)
RESULT case1b File Output after Scale Relative 0.5x0.5 -> scaledBeauty.png is 200x100 ; scene output 400x200 (scene is 400x200)
SCALE node inputs: [('Image', 'RGBA'), ('Type', 'MENU'), ('X', 'VALUE'), ('Y', 'VALUE'), ('Frame Type', 'MENU'), ('Interpolation', 'MENU'), ('Extension X', 'MENU'), ('Extension Y', 'MENU')]
RESULT case1c File Output fed an 800x600 picture from an Image node -> pictureBeauty.png is 800x600 ; scene output 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=False -> 400x200 (scene is 400x200)
RESULT case2 use_border=True use_crop_to_border=True -> 200x100 (scene is 400x200)
RESULT case3 stereo SIDEBYSIDE squeezed=False -> 800x200 (scene is 400x200) ; files: ['stereo_SIDEBYSIDE_False.png']
RESULT case3 stereo SIDEBYSIDE squeezed=True -> 400x200 (scene is 400x200) ; files: ['stereo_SIDEBYSIDE_True.png']
RESULT case3 stereo TOPBOTTOM squeezed=False -> 400x400 (scene is 400x200) ; files: ['stereo_TOPBOTTOM_False.png']
RESULT case3 stereo TOPBOTTOM squeezed=True -> 400x200 (scene is 400x200) ; files: ['stereo_TOPBOTTOM_True.png']
CROP node inputs: [('Image', 'RGBA', None), ('X', 'INT', 0), ('Y', 'INT', 0), ('Width', 'INT', 1920), ('Height', 'INT', 1080), ('Alpha Crop', 'BOOLEAN', False)]
   set X = 100
   set Y = 50
   set Width = 200
   set Height = 100
   set Alpha Crop = False
RESULT case4 Crop node (Alpha Crop off, 200x100 region) before the Group Output -> main output 400x200 (scene is 400x200) ; first pixel [0.0, 0.0, 0.0, 0.0]
RESULT case5 pixel_aspect_x=2.0 -> 400x200 (scene is 400x200)
RESULT case6 use_stamp (Burn Into Image)=True -> 400x200 (scene is 400x200)
```

**This settles, for this Mac, the "not in the sources" and "Not covered" points above about size.**

- **(d) A File Output node writes at the size of the picture plugged into it**, not at the scene's size: 122×90 after a Scale set to an absolute 120×90, 200×100 after a relative 0.5, and 800×600 from an 800×600 picture, while the scene's own output stayed 400×200 in the same render. The Scale node set to 120 pixels wide gave a file 122 pixels wide; nothing in the test explains the two extra pixels.
- **(d) A Crop node before the Group Output did not change the main output's size**: it stayed 400×200, its corner pixel transparent.
- **(a) Crop to Render Region wrote exactly the region's size**, 200×100; a render region without the crop left the size at 400×200.
- **(b) Stereo**: unsqueezed Side-by-Side doubled the width and unsqueezed Top-Bottom doubled the height; both squeezed forms kept the scene's size. One file was written each time.
- **(f) Pixel aspect X of 2 did not change the pixel count written**, and **Burn Into Image did not change the size**.

**Entry 21 — the output-path template under a crop, and where the crop sits relative to the compositor.** Entry 20's 400×200 scene with the render region cropped to the middle half (0.25 to 0.75 both ways). (a) The output path set to `{resolution_x}x{resolution_y}_` with a plain Render Layers → Group Output group. (b) A group whose only picture is a 400×200 float picture from an Image node, its red running from 0 at the left column to 1 at the right (red = x/399), straight into the Group Output, written as 32-bit OpenEXR; and as a control, the same with no render region. (c) Group Input → Invert Color → Group Output under the crop. Word for word:

```
RESULT path template with crop on -> files written: ['400x200_.png']
RESULT   400x200_.png is actually 200x100
RESULT gradient group with crop on -> written 200x100
   pixel (0,0) -> [0.25063, 0.0, 0.0, 1.0]
   pixel (199,0) -> [0.74938, 0.0, 0.0, 1.0]
   pixel (0,99) -> [0.25063, 0.0, 0.0, 1.0]
   pixel (199,99) -> [0.74938, 0.0, 0.0, 1.0]
   pixel (100,50) -> [0.50124, 0.0, 0.0, 1.0]
CONTROL gradient group, no render region -> written 400x200 ; pixel (0,0) [0.0, 0.0, 0.0, 1.0] ; pixel (399,0) [1.00004, 0.0, 0.0, 1.0]
RESULT Group Input -> Invert -> Group Output with crop on -> written 200x100 ; pixel (0,0) [0.85022, 0.45166, 0.9201, 0.99805]
```

The path template named the file for the scene's own Resolution values, 400×200, on a file that is 200×100. Under the crop, the group's full-frame gradient came out 200×100 with its left edge at about 0.25 and its right edge at about 0.75 — the middle half of the gradient, not the whole of it squeezed in and not its left half. The group taking the render in through a Group Input also came out 200×100.

**Entry 22 — what Blender does with an odd frame size in H.264.** Two files built and saved by Blender 5.3, each two frames of a Workbench scene with a cube, Media Type Video, container MPEG-4, codec H.264: one at 1921×1081, and as a control one at 1920×1080. Each was rendered from the terminal as `blender -b --factory-startup <file> -a`. Every line Blender printed that names a frame, a save, an error, a codec or a size, the exit status, the files left, and each file's size read back by Blender — word for word:

```
----- video_1921x1081.blend -a
00:00.330  blend            | Read blend: ".../blends/5.3/video_1921x1081.blend"
00:00.338  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library
00:00.338  reports          | ERROR width not divisible by 2 (1921x1081)
EXIT STATUS 0
----- video_1920x1080.blend -a
00:00.318  blend            | Read blend: ".../blends/5.3/video_1920x1080.blend"
00:00.326  render           | Rendering frame 1
00:00.466  render           | Video append frame 1
00:00.466  render           | Rendering frame 2
00:00.496  render           | Video append frame 2
EXIT STATUS 0
----- files
total 16
-rw-r--r--@ 1 artist     wheel  5979 Oct 18 05:05 video_1920x1080_0001-0002.mp4
READ video_1920x1080_0001-0002.mp4 -> size (1920, 1080) frames 2
```

**This answers (e) above, for this Mac.** At 1921×1081 Blender did not adjust the size: it printed `ERROR width not divisible by 2 (1921x1081)`, rendered no frame, wrote no file — and exited with status 0. The 1920×1080 control wrote a two-frame video at 1920×1080.

**Entry 36 — whether Crop to Render Region is greyed out until Render Region is ticked (in the window).** Started and photographed as described in question 1's window-test section, with the Properties editor filling the window on its Output tab. In the factory state the Format panel holds two checkboxes one above the other, **Render Region** and **Crop to Render Region**, both unticked, and **Crop to Render Region is drawn dimmed** — label and box — while Render Region is drawn at full strength. The script then set Render Region on (`use_border = True`) and photographed again: Render Region shows ticked and **Crop to Render Region is drawn at full strength, still unticked**. (A photograph meant to show the first state a second time, taken in the same run as the second, came out showing the window as it was before the Properties editor was enlarged; the first state is the one photographed in an earlier run of the same set-up.)

**Entry 34 — the Output panel.** Written out under question 2: it has a Media Type control offering Image, Multi-Layer EXR and Video, and nothing in it offers another way of setting the written pixel size.

**Entry 17 — the sequencer route.** Written out in full under question 7: with and without a Scene strip, with each switch on and off, every frame was written at the scene's 400×200.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**Label:** documented (the setting and several effects); inferred where marked.

**Answer.** Yes. In Output Properties ‣ Post Processing, turn **Compositing** off and leave **Sequencer** on. The sequencer only takes over if the scene has strips.
- "Pipeline – Sequencer / Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html
- `use_sequencer`: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- The picture `_images/render_output_properties_post-processing_panel.png` shows both checkboxes side by side.

**What it changes in what gets written:**
1. **Content.** It is the sequencer's output, "instead of the view from the 3D scene's active camera" (documented).
2. **Scene strips.** They are rendered as part of the pipeline. "Scene strips do not render individual Render Passes; only the Combined render pass will be used." — https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/scene.html (documented).
3. **Size.** Strip content is fitted to "the scene's Resolution" (movie strip page). Inferred: the written frame stays at the scene's resolution.
4. **Colour handling.** "Sequencer — The color space that the Sequencer operates in. By default, the Sequencer operates in sRGB space, but it can also be set to work in Linear space like the Compositing nodes, or another color space. Different color spaces will give different results for color correction, crossfades, and other operations." — https://docs.blender.org/manual/en/5.3/render/color_management/color_spaces.html (documented).
5. **Metadata.** "Metadata Input — Where to grab metadata from. / Scene: Use metadata from the current scene. / Sequencer Strips: Use metadata from the strips in the Sequencer." — https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html. Also `use_stamp_sequencer_strip`: "Include the name of the foreground sequence strip in image metadata" (API). Documented.
6. **File Output nodes.** Inferred: they do not write when Compositing is off. The step: they are compositor nodes, and the Output page puts them to use only "if compositing is enabled".
7. **Error.** If automatic saving is also off: "For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (documented).
8. **Workbench settings of strips.** `use_sequencer_override_scene_strip`: "Use Workbench render and world settings from the sequencer scene, instead of each strip's scene (default False)" — API (documented).

**Which scene's sequencer.** The manual describes a separate "Sequencer Scene":
- "The video sequencer uses a separate scene data-block that contains the edit. This is the Sequencer Scene." … "When the active scene is different from the Sequencer Scene in the current workspace, the 'Render' menu will include separate options to render the Sequencer Scene (and not the active Scene)." — https://docs.blender.org/manual/en/5.3/video_editing/sequencer_scene.html
- `bpy.ops.render.render(... use_sequencer_scene=False ...)`: "Use Sequencer Scene, Render the sequencer scene instead of the active scene" — API, https://docs.blender.org/api/5.3/bpy.ops.render.html

**Not covered:**
- With Compositing **and** Sequencer both on and strips present, the sources do not say whether the scene's own compositor still runs on anything. The Post Processing sentence only says Scene strips then carry "the output of the Compositor".
- The frame range used, and audio, are not covered here.

**Where I looked:**
- Pages: Post Processing, Output, Rendering Animations, Scene strip, movie strip, Sequencer Scene, Video Editing Workspaces, Metadata, Color Spaces, OpenColorIO, Sequencer preview sidebar.
- The RenderSettings API page.
- I searched the manual for "scene strip", "resolution" and "render size" in the video editing and sequencer folders.

**Outside the interface:**
- `use_compositing` and `use_sequencer` can be set from a script.
- The command-line pages list no argument for either. I searched them for "sequenc" and "compos".
- The Render-menu choice of Sequencer Scene is an interface feature. From a script, `bpy.ops.render.render(use_sequencer_scene=True)` is the documented route. The sources do not say how a plain `-b … -a` render chooses between the active scene and a Sequencer Scene, beyond `-S` setting "the active scene".

**Operating system:** The sources do not say this differs.

**Driven from outside — observed, 2026-10-18.** With no window both switches took effect (entry 17), a strip followed its own scene's switch (entry 16), and a multi-layer EXR lost its depth pass through the sequencer (entry 18). **And it settles how a plain render picks a scene**: `-b … -f 1` rendered the file's active scene even though every workspace named another as its Sequencer Scene; `-S` with that scene's name, or the render operator with `use_sequencer_scene=True`, rendered the Sequencer Scene (entry 19).

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 17 — what the sequencer route writes, and whether the scene's own group still runs under it.** A 400×200 scene A whose camera view is blue, whose group replaces the picture with a flat red colour node into the Group Output and also sends its Render Layers node into a File Output node aimed at an empty folder; and a scene G rendering green, added to A's sequencer as a Scene strip for half the cases. Six cases, each in its own fresh Blender (entry 16 showed that renders made one after another in one Blender can carry a strip's picture over). Word for word:

```
RESULT no strips, sequencer ON , compositing OFF -> 400x200 first pixel RGBA [0.0706, 0.3333, 0.8196, 1.0] ; A's File Output folder: []
RESULT no strips, sequencer OFF, compositing OFF -> 400x200 first pixel RGBA [0.0706, 0.3333, 0.8196, 1.0] ; A's File Output folder: []
RESULT no strips, sequencer ON , compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1294, 1.0] ; A's File Output folder: ['A_.exr']
RESULT strip,    sequencer ON , compositing OFF -> 400x200 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0] ; A's File Output folder: []
RESULT strip,    sequencer ON , compositing ON  -> 400x200 first pixel RGBA [0.4235, 0.7686, 0.3137, 1.0] ; A's File Output folder: []
RESULT strip,    sequencer OFF, compositing ON  -> 400x200 first pixel RGBA [0.8549, 0.2196, 0.1294, 1.0] ; A's File Output folder: ['A_.exr']
```

Blue is A's camera view, red is A's group's result, green is the strip. With Sequencer on and no strips, the scene's ordinary picture was written — the camera view with Compositing off, the group's result with it on. With a strip, the strip's picture was written whether Compositing was on or off. **This settles the "Not covered" point above about Compositing and Sequencer both on with strips present**: A's own File Output node wrote nothing in that case, so A's own group did not run; it wrote only when A's group's result was what reached the file. The written size stayed 400×200 in all six.

**Entry 18 — what survives into a multi-layer EXR through the sequencer.** A Workbench scene with the Z (depth) and Mist passes turned on for its view layer, written as multi-layer OpenEXR, once plainly and once with a Scene strip of a second scene and Sequencer on, each in its own Blender. The channel names were read straight from each file's own header. Word for word:

```
RESULT multi-layer EXR, sequencer=False -> file exists: True size 2686 ; header flags 0x1002 ; parts 2
--- channels in exr_seqFalse.exr, part 0
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
--- channels in exr_seqFalse.exr, part 1
ViewLayer.Depth.Z
RESULT multi-layer EXR, sequencer=True -> file exists: True size 2105 ; header flags 0x2 ; parts 1
--- channels in exr_seqTrue.exr, part 0
Composite.Combined.A
Composite.Combined.B
Composite.Combined.G
Composite.Combined.R
```

Without the sequencer the file held the view layer's Combined and Depth passes, in two parts; no Mist channel appeared in either file. With the sequencer supplying the picture the file held one layer named `Composite` with the combined picture only, and the depth pass was gone. A multi-layer EXR was written both ways.

**Entry 19 — which scene a render with no window uses when the file names a different Sequencer Scene.** `seqscene.blend`, built and saved by Blender 5.3: scene A, the active scene, rendering blue with no strips; scene S, whose own camera view is magenta and whose sequencer holds a Scene strip of a scene G rendering green, Sequencer on; Compositing off in all three; and every workspace in the file given S as its Sequencer Scene. The builder printed `WORKSPACE SEQUENCER SCENES: ['S']` and `ACTIVE SCENE: A`. It was rendered from the terminal three ways — `blender -b --factory-startup seqscene.blend --log "render" -o <folder>/plain_ -F PNG -f 1`; the same with `-S S`; and with a script calling `bpy.ops.render.render(write_still=True, use_sequencer_scene=True)` — and the frames read back. Word for word:

```
----- plain: -f 1, no -S
00:00.330  render           | Start rendering: A, ViewLayer
00:00.459  render           | Saved: '.../t19/plain_0001.png'
exit 0
----- -S S -f 1
00:00.329  render           | Start rendering: G, ViewLayer
00:00.455  render           | Saved: '.../t19/S_0001.png'
exit 0
----- render operator with use_sequencer_scene=True, from a script
00:00.332  render           | Start rendering: G, ViewLayer
00:00.459  render           | Saved: '.../t19/op_S_.png'
CONTEXT SCENE A ; context.workspace <bpy_struct, WorkSpace("Layout") at 0xbf1a3dba0> ; context.window <bpy_struct, Window at 0xbf66734a0>
OPERATOR RETURNED {'FINISHED'}
exit 0
----- read back
READ S_0001.png -> first pixel [0.4235, 0.7686, 0.3137, 1.0]
READ op_S_.png -> first pixel [0.4235, 0.7686, 0.3137, 1.0]
READ plain_0001.png -> first pixel [0.0706, 0.3333, 0.8196, 1.0]
```

(`...` is the work folder.) **This answers what *Outside the interface* above leaves open.** A plain `-b … -f 1` rendered the file's active scene, A, and wrote its blue camera view; the workspaces' Sequencer Scene played no part. Naming the Sequencer Scene with `-S S` rendered S's edit, the green strip. And the render operator with `use_sequencer_scene=True`, run from a script with no window, rendered S — writing to S's own output path — even though the current scene was A; a Blender started with `-b` on this Mac still reported a workspace and a window to the script.

**Entry 38 — what the Render menu offers, in a fresh file and with a different Sequencer Scene (in the window).** Started and photographed as described in question 1's window-test section, in the Layout workspace. The top bar's **Render** menu was opened with a simulated click on it and photographed, the window made to redraw just before. In a fresh file — where the script printed `WORKSPACE Layout sequencer_scene None active scene Scene` — the menu holds, in order: **Render Image** (F12), **Render Animation** (⌘ F12), a dividing line, **Render Audio…**, a line, **View Render** (F11), **View Animation** (⌘ F11), a line, and **Lock Interface** as an unticked checkbox. No entry names a Sequencer Scene. In a second Blender the script first made a new scene `Edit` holding a Scene strip and gave the workspace `Edit` as its Sequencer Scene — it printed `WORKSPACE Layout sequencer_scene Edit active scene Scene` — and then opened the menu: it now holds **Render Image** (F12), **Render Animation** (⌘ F12), a line, **Render Sequencer Image** (⌥ F12), **Render Sequencer Animation** (⌥ ⌘ F12), a line, **Render Audio…**, a line, **View Render**, **View Animation**, a line, and **Lock Interface**. **This settles what the earlier versions left open**: the menu grows its two Sequencer entries once the workspace's Sequencer Scene differs from the active scene. Three earlier tries are recorded in the hand-test list; their pictures showed no menu — once because the click only closed the splash screen, and once on a second opening after the menu had been opened and closed in the same run.

**Entry 33 — the two switches in the window.** Written out under question 1's window tests: the Post Processing panel holds a Pipeline row with two separate ticked checkboxes, Compositing and Sequencer, and a Dither slider, and its header has no checkbox.

**Entry 16 — whose Compositing switch a Scene strip follows.** Written out in full under question 4: the strip's own scene's switch, not the rendering scene's; and with the strip's scene's one effect switched off for render, its raw picture.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

**Label:**
- Following: inferred (from documented structure).
- What each two-input node does: documented.
- A general way to tell "which one came out": not in the sources.

**Answer, colour-changing nodes.**
- Yes, a picture can be followed through them. They have a picture input and a picture output, joined by ordinary links (see 5c).
- Hue/Saturation/Value: "Inputs — Image/Color — Standard color input." … "Outputs — Image/Color — Standard color output." … "Factor — The amount of influence the node exerts on the image." — https://docs.blender.org/manual/en/5.3/compositing/types/color/adjust/hue_saturation.html
- Exposure: "Image — Standard color input." / "Image — Standard color output." — https://docs.blender.org/manual/en/5.3/compositing/types/color/adjust/exposure.html
- Inferred: the output keeps the input picture's size and placement. The step: "the operation domain of the node is identical to the domain of that designated input", and for such nodes the main picture input is the domain input. That rule is on https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html

**Answer, two-pictures-in, one-out nodes.** A picture can be followed into them along links. Which picture "comes out" depends on the node:
- **Switch:** one whole input comes out, chosen by a condition. "The Switch node outputs one of two inputs depending on a condition." … "Switch — Determines which of the two inputs below will be passed through." … "Output — One of the two inputs without any modifications." — https://docs.blender.org/manual/en/5.3/compositing/types/utilities/switch.html. Inferred: if the condition is set on the node rather than fed by a link, it can be read from the file to tell which input passes.
- **Mix:** both contribute. "Factor — Controls the amount of mixing between the A and B inputs." — https://docs.blender.org/manual/en/5.3/compositing/types/utilities/math/mix.html. The size and placement follow the first input: "the first Image input in the Mix node has the highest domain priority" — compositor_system page.
- **Alpha Over:** both contribute, and the result takes the Background's frame. "composites two images by layering the Foreground image over the Background" (https://docs.blender.org/manual/en/5.3/compositing/types/color/alpha_over.html). "if the Foreground input is bigger than the Background input in the Alpha Over node, the output will be clipped to the Background input because it is the domain input" (compositor_system page).
- **Split:** both appear, divided by a line. "combines two input images into a single output image, displaying them side by side or along a rotated split line." — https://docs.blender.org/manual/en/5.3/compositing/types/utilities/split.html
- **Depth Combine:** chosen per pixel. "Whichever depth value is less decides which image pixel is used." — https://docs.blender.org/manual/en/5.3/compositing/types/color/depth_combine.html

The general rule for whose frame the output takes: "the operation domain of the node is the domain of the non single value input with the highest domain priority" — compositor_system page. The page also covers the case where only one input is a picture.

**Not in the sources:**
- A way, from outside, to read which input "won". Domain priority is not exposed in the API: I searched the whole API for "domain_priority", "domain priority" and "domain" on the Node, NodeSocket and CompositorNode pages, and found nothing.
- A Switch whose condition is fed by a link decides while the tree is worked out. The sources describe no way to read back what it chose.

**Not covered:**
- The compositor's Switch node exists under two API classes: `CompositorNodeSwitch`, "Switch between two images using a checkbox", and the generic `GeometryNodeSwitch`, "Switch between two inputs". The manual page's picture is named `node-types_GeometryNodeSwitch.webp`. The sources do not say which class a new compositor Switch node uses in 5.3.

**Where I looked:**
- Pages: Color nodes index, Adjust index, Hue/Saturation/Value, Exposure, Mix, Mix Color, Alpha Over, Switch, Index Switch, Menu Switch, Split, Depth Combine, Compositor System.
- API pages: CompositorNodeSwitch, GeometryNodeSwitch, ShaderNodeMix, NodeLink, NodeSocket, Node.

**Outside the interface:**
- Links, node types and a node's unlinked input values are all data. Inferred: a script can read them in a background run.
- The sources say nothing about this differing from the interface.

**Operating system:** The sources do not say this differs.

**Driven from outside — observed, 2026-10-18.** Every reading and every picture in entries 24 to 28 was got with no window, and entry 40 found the same graph printout and the same four pictures, to the last figure, with a window open.

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac); Linux and Windows were not tried in this sitting.

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with no window and `--factory-startup`, its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 3 — which node types a compositor tree accepts, which take two pictures and give one, which are switch-like, and which type the Add menu's "Switch" entry adds.** Every name Blender lists among its types was offered to an empty compositor group; the ones it accepted were kept. Word for word:

```
BLENDER_VERSION 5.3.0 Beta
ADDABLE NODE TYPE COUNT 187
RULE FOR THE NEXT LINE: at least two inputs of type RGBA and exactly one output of type RGBA
TWO-PICTURE-IN, ONE-PICTURE-OUT COUNT 24
TWO-PICTURE-IN, ONE-PICTURE-OUT ['CompositorNodeAlphaOver', 'CompositorNodeBilateralblur', 'CompositorNodeBokehBlur', 'CompositorNodeChromaMatte', 'CompositorNodeColorBalance', 'CompositorNodeColorMatte', 'CompositorNodeColorSpill', 'CompositorNodeConvolve', 'CompositorNodeCurveRGB', 'CompositorNodeDenoise', 'CompositorNodeDiffMatte', 'CompositorNodeDistanceMatte', 'CompositorNodeKeying', 'CompositorNodeSplit', 'CompositorNodeSwitch', 'CompositorNodeSwitchView', 'CompositorNodeZcombine', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch', 'GeometryNodeSwitch', 'ShaderNodeMix', 'ShaderNodeMixRGB', 'ShaderNodeTexBrick', 'ShaderNodeTexChecker']
SWITCH IN THE TYPE NAME ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch', 'GeometryNodeSwitch']
THE OLDER, WIDER RULE — 'Switch' or 'Index' or 'Menu' in the type name: ['CompositorNodeSwitch', 'CompositorNodeSwitchView', 'FunctionNodeInputMenu', 'GeometryNodeIndexSwitch', 'GeometryNodeMenuSwitch', 'GeometryNodeSwitch']
PRESENT CompositorNodeSwitch label: Switch
   IN  [('Switch', 'Switch', 'BOOLEAN'), ('Off', 'Off', 'RGBA'), ('On', 'On', 'RGBA')]
   OUT [('Image', 'Image', 'RGBA')]
PRESENT CompositorNodeAlphaOver label: Alpha Over
   IN  [('Background', 'Background', 'RGBA'), ('Foreground', 'Foreground', 'RGBA'), ('Factor', 'Fac', 'VALUE'), ('Type', 'Type', 'MENU'), ('Straight Alpha', 'Straight Alpha', 'BOOLEAN')]
   OUT [('Image', 'Image', 'RGBA')]
PRESENT CompositorNodeSplit label: Split
   IN  [('Position', 'Position', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Image', 'Image', 'RGBA'), ('Image', 'Image_001', 'RGBA')]
   OUT [('Image', 'Image', 'RGBA')]
PRESENT CompositorNodeZcombine label: Depth Combine
   IN  [('A', 'A', 'RGBA'), ('Depth A', 'Depth A', 'VALUE'), ('B', 'B', 'RGBA'), ('Depth B', 'Depth B', 'VALUE'), ('Use Alpha', 'Use Alpha', 'BOOLEAN'), ('Anti-Alias', 'Anti-Alias', 'BOOLEAN')]
   OUT [('Result', 'Result', 'RGBA'), ('Depth', 'Depth', 'VALUE')]
PRESENT CompositorNodeCurveRGB label: RGB Curves
   IN  [('Image', 'Image', 'RGBA'), ('Factor', 'Fac', 'VALUE'), ('Black Level', 'Black Level', 'RGBA'), ('White Level', 'White Level', 'RGBA')]
   OUT [('Image', 'Image', 'RGBA')]
```

The Switch node offers `Switch`, `Off`, `On` and `Image`; the manual page quoted above names them `Switch`, `False`, `True` and `Output`. The two same-named picture inputs of the Split node carry different identifiers, `Image` and `Image_001`.

**Which type the Add menu's "Switch" adds — this settles the "Not covered" point above about the two Switch classes.** The compositor's Add menus were drawn by their own code into a recorder that keeps every entry instead of putting it on screen. A first attempt failed for every menu with errors such as `AttributeError("'FakeSelf' object has no attribute 'node_operator'")`, because the stand-in lacked the menus' own helper methods; it was rewritten to lend the menu its own methods, and drew the Utilities menu. What the Utilities menu holds that names a switch, word for word:

```
{'menu': 'NODE_MT_category_compositor_utilities', 'op': 'node.add_node', 'text': 'Index Switch', 'type': 'GeometryNodeIndexSwitch', 'use_transform': True}
{'menu': 'NODE_MT_category_compositor_utilities', 'op': 'node.add_node', 'text': 'Menu Switch', 'type': 'GeometryNodeMenuSwitch', 'use_transform': True}
{'menu': 'NODE_MT_category_compositor_utilities', 'op': 'node.add_node', 'text': 'Switch', 'type': 'GeometryNodeSwitch', 'use_transform': True}
{'menu': 'NODE_MT_category_compositor_utilities', 'op': 'node.add_node', 'text': 'Switch Stereo View', 'type': 'CompositorNodeSwitchView', 'use_transform': True}
```

So the menu entry labelled "Switch" adds a `GeometryNodeSwitch`. `CompositorNodeSwitch` is still accepted by a compositor tree, but no entry of this menu adds it. Nothing was clicked: this is what the menu's own code asked to draw, not a look at the screen.

**Entry 9 — the processor against the card on a two-picture tree, and what a socket exposes.** Two 64×64 float pictures were made and loaded by Image nodes: picA running 0 to 1 in red from left to right, picB running 0 to 1 in green from bottom to top. A Hue/Saturation/Value node on picA fed an Alpha Over's Background, picB its Foreground, at a factor of 0.5, written as 32-bit OpenEXR with the device on CPU and then on GPU (Apple's own graphics). Then a heavy group: a 1024×1024 checker picture through sixty Blur nodes of size 4, rendered at 2048×2048. Word for word:

```
DEVICE CPU: rendered, corner sample [0.05, 0.25, 0.525, 1.0]
DEVICE GPU: rendered, corner sample [0.05, 0.25, 0.525, 1.0]
MAXDIFF between the processor's picture and the card's picture: 2.980232238769531e-07
OUTPUT SOCKET PROPERTY NAMES: ['bl_idname', 'bl_label', 'bl_subtype_label', 'default_value', 'description', 'display_shape', 'enabled', 'hide', 'hide_value', 'identifier', 'inferred_structure_type', 'is_icon_visible', 'is_inactive', 'is_linked', 'is_multi_input', 'is_output', 'is_unavailable', 'label', 'link_limit', 'name', 'node', 'pin_gizmo', 'rna_type', 'select', 'show_expanded', 'type']
ANY NAMING INSPECTION, RESOLUTION, TRANSFORM, PRECISION, SAMPLING, VALUE OR IMAGE: ['hide_value', 'default_value']
HEAVY CPU SIZE (2048, 2048) HEAVY GPU SIZE (2048, 2048)
HEAVY MAXDIFF processor against card: 1.0132789611816406e-06
HEAVY MIDDLE PIXEL, processor: [0.49927, 0.49999, 0.25, 1.0]
HEAVY MIDDLE PIXEL, card     : [0.49927, 0.49999, 0.25, 1.0]
```

The two devices gave the same picture to about three parts in ten million on the small tree and one part in a million on the heavy one: not identical, but no test coarser than a millionth can tell them apart. The property names are those of the Hue/Saturation/Value node's output socket after the render; none names an inspection, a resolution, a transform, a precision or a sampling, and `default_value` holds the socket's own fixed value rather than the picture that went through it. **Whether the run marked GPU really ran on the card was not checked**: this Mac has no tool that names the processes on its graphics, unlike the NVIDIA tool the earlier versions used on Linux. The script also timed the heavy renders, but the Creator's own Blender was open on the Mac throughout, so those times are not given as findings.

**Entries 24 to 28 share two pictures.** Two 64×64 float pictures were made and saved as OpenEXR: **picA**, whose red runs 0 to 1 from left to right and green 0 to 1 from bottom to top, blue 0.25; and **picB**, red 0.1, green running 1 to 0 from left to right, blue 0.8; both fully opaque. Image nodes load them, the scene is 64×64, and each group's result is written as 32-bit OpenEXR with no compression and loaded back. `maxdiff` is the largest difference in any channel of any pixel between two pictures; zero means they match to the last figure. Pictures were compared as pictures, never as files.

**Entry 24 — pictures through two-input and colour nodes.** The script first stopped at `KeyError: 'bpy_prop_collection[key]: key "A_Color" not found'` — a node's sockets are looked up by name, and the Mix node's names repeat — and was changed to look its sockets up by identifier. Its first two lines, printed before the stop, were the same in both runs. The second run, word for word:

```
BLENDER_VERSION 5.3.0 Beta
MAXDIFF render vs source 0.0
MAXDIFF render vs same render again 0.0
FRESH MIX Factor_Float held: 1.0
ShaderNodeMix factor=0.0  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
ShaderNodeMix factor=1.0  maxdiff vs picA=1.000038743019104  maxdiff vs picB=0.0
AlphaOver Fac=0.0  background=picA foreground=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
AlphaOver Fac=1.0  background=picA foreground=picB  maxdiff vs picA=1.000038743019104  maxdiff vs picB=0.0
Switch=False  Off=picA On=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
Switch=True  Off=picA On=picB  maxdiff vs picA=1.000038743019104  maxdiff vs picB=0.0
GeometryNodeSwitch input_type before: RGBA ; inputs [('Switch', 'Switch', 'BOOLEAN'), ('False', 'False', 'RGBA'), ('True', 'True', 'RGBA')]
GeometryNodeSwitch after input_type='RGBA': inputs [('Switch', 'Switch', 'BOOLEAN'), ('False', 'False', 'RGBA'), ('True', 'True', 'RGBA')] outputs [('Output', 'Output', 'RGBA')]
GeometryNodeSwitch Switch=False  False=picA True=picB  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
GeometryNodeSwitch Switch=True  False=picA True=picB  maxdiff vs picA=1.000038743019104  maxdiff vs picB=0.0
MUTED AlphaOver          first picture into Background, second into Foreground  internal_links=[('Background', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
MUTED Mix                first picture into A_Color, second into B_Color  internal_links=[('A_Color', 'Result_Color')]  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
MUTED Switch             first picture into Off, second into On  internal_links=[('Off', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
MUTED Split              first picture into Image, second into Image_001  internal_links=[('Image', 'Image')]  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
MUTED Z/DepthCombine     first picture into A, second into B  internal_links=[('A', 'Result'), ('Depth B', 'Depth')]  maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
COLOUR NODE HueSat     muted=False maxdiff vs the picture that went in = 1.000038743019104
COLOUR NODE HueSat     muted=True maxdiff vs the picture that went in = 0.0
COLOUR NODE RGBCurves  muted=False maxdiff vs the picture that went in = 1.0
COLOUR NODE RGBCurves  muted=True maxdiff vs the picture that went in = 0.0
```

(The Hue/Saturation/Value node had its Hue set to 0 and the RGB Curves node its Black Level set to grey 0.5, so that each changes the picture when not muted.) The control came back zero both ways, so a zero below means the pictures match. A Mix node at Factor 0 gave back its A picture and at Factor 1 its B picture, to the last figure; a fresh Mix node held a Factor of 1.0. Alpha Over at Factor 0 gave back its Background and at 1 its (opaque) Foreground. **The compositor's Switch node, and the `GeometryNodeSwitch` that the Add menu's "Switch" adds (entry 3), each gave back the chosen input exactly.** Every muted two-input node passed its first picture through unchanged, and its `internal_links` named that input; a muted colour node passed its picture through unchanged.

**Entry 25 — what the file says about which input reaches the output.** For each two-input node, picA into its first picture input and picB into its second, the output to the Group Output; then the same read muted. Then a Mix node's `is_inactive` before and after a render and across its Factor; then a Switch whose choice is fed by a link. Word for word:

```
=== AlphaOver (CompositorNodeAlphaOver) ===
  unmuted internal_links: [('Background', 'Image')]
  unmuted is_inactive: [('Background', 'Background', False), ('Foreground', 'Foreground', False), ('Factor', 'Fac', False), ('Type', 'Type', False), ('Straight Alpha', 'Straight Alpha', False)]
  muted   internal_links: [('Background', 'Image')]
  muted   is_inactive: [('Background', 'Background', False), ('Foreground', 'Foreground', False), ('Factor', 'Fac', False), ('Type', 'Type', False), ('Straight Alpha', 'Straight Alpha', False)]
  ANY NAMING PRIORITY OR DOMAIN: []
=== Switch (CompositorNodeSwitch) ===
  unmuted internal_links: [('Off', 'Image')]
  unmuted is_inactive: [('Switch', 'Switch', False), ('Off', 'Off', False), ('On', 'On', False)]
  muted   internal_links: [('Off', 'Image')]
  muted   is_inactive: [('Switch', 'Switch', False), ('Off', 'Off', False), ('On', 'On', False)]
  ANY NAMING PRIORITY OR DOMAIN: []
=== Split (CompositorNodeSplit) ===
  unmuted internal_links: [('Image', 'Image')]
  unmuted is_inactive: [('Position', 'Position', False), ('Rotation', 'Rotation', False), ('Image', 'Image', False), ('Image', 'Image_001', False)]
  muted   internal_links: [('Image', 'Image')]
  muted   is_inactive: [('Position', 'Position', False), ('Rotation', 'Rotation', False), ('Image', 'Image', False), ('Image', 'Image_001', False)]
  ANY NAMING PRIORITY OR DOMAIN: []
=== Z/Depth Combine (CompositorNodeZcombine) ===
  unmuted internal_links: [('A', 'Result'), ('Depth B', 'Depth')]
  unmuted is_inactive: [('A', 'A', False), ('Depth A', 'Depth A', False), ('B', 'B', False), ('Depth B', 'Depth B', False), ('Use Alpha', 'Use Alpha', False), ('Anti-Alias', 'Anti-Alias', False)]
  muted   internal_links: [('A', 'Result'), ('Depth B', 'Depth')]
  muted   is_inactive: [('A', 'A', False), ('Depth A', 'Depth A', False), ('B', 'B', False), ('Depth B', 'Depth B', False), ('Use Alpha', 'Use Alpha', False), ('Anti-Alias', 'Anti-Alias', False)]
  ANY NAMING PRIORITY OR DOMAIN: []
=== Mix (ShaderNodeMix) ===
  unmuted internal_links: [('A_Color', 'Result_Color')]
  unmuted is_inactive: [('Factor', 'Factor_Float', False), ('Factor', 'Factor_Vector', True), ('A', 'A_Float', True), ('B', 'B_Float', True), ('A', 'A_Vector', True), ('B', 'B_Vector', True), ('A', 'A_Color', True), ('B', 'B_Color', False), ('A', 'A_Rotation', True), ('B', 'B_Rotation', True)]
  muted   internal_links: [('A_Color', 'Result_Color')]
  muted   is_inactive: [same list as unmuted]
  ANY NAMING PRIORITY OR DOMAIN: []
A_Color is_linked: True   B_Color is_linked: True   Factor_Float held: 1.0
  before a render, is_inactive: [same list as above]
  after a render,  is_inactive: [same list as above]
FACTOR 0.0  A_Color is_inactive=False  B_Color is_inactive=True  (both linked: True, True)
FACTOR 0.25  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)
FACTOR 0.5  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)
FACTOR 0.75  A_Color is_inactive=False  B_Color is_inactive=False  (both linked: True, True)
FACTOR 1.0  A_Color is_inactive=True  B_Color is_inactive=False  (both linked: True, True)
VALUE NODE CompositorNodeValue refused: Error: Node type CompositorNodeValue undefined
VALUE NODE USED ShaderNodeValue
LINK FROM VALUE NODE (value 1.0) ACCEPTED, is_valid: True
WITH A VALUE OF 1.0 ON THE SWITCH INPUT: maxdiff vs picA=1.000038743019104  maxdiff vs picB=0.0
LINK FROM A PICTURE (0 at the left, 1 at the right) ACCEPTED, is_valid: True
WITH A PICTURE ON THE SWITCH INPUT: maxdiff vs picA=0.0  maxdiff vs picB=1.000038743019104
   left column x=5 y=32 -> [0.07937, 0.50792, 0.25, 1.0]   right column x=58 y=32 -> [0.92062, 0.50792, 0.25, 1.0]
```

(Where the printout repeated the Mix node's ten-item list unchanged, it is written here as "same list"; the uncut lines are in the hand-test list.) `internal_links` names one picture input on every one of these nodes, muted or not. No property on these nodes or their sockets names a priority or a domain. **`is_inactive` on a Mix node follows the Factor, not the wiring**: with both pictures linked, A reads inactive at Factor 1 and B at Factor 0, neither in between, and a fresh node sits at 1.0 — so it does not on its own say which picture reached the far side. **A Switch whose choice is fed by a link**: a Value node of 1.0 sent the whole frame to the On input; a picture running 0 to 1 across the frame sent the whole frame to the Off input, the left and right columns both being picA's. **This bears on the "not in the sources" point above**: in neither case did anything in the file record which input the linked choice picked; only the written picture shows it.

**Entry 26 — the Split node's two sides, Depth Combine pixel by pixel, and which input decides a node's frame.** picA into the Split node's first picture input and picB into its second, left at its own Position and Rotation; Depth Combine with picA on A and picB on B, at two flat depths each way round and then with picA's depth running 0 on the left to 1 on the right against picB's flat 0.5. Then a 32×32 picture and a 128×128 picture, both fully opaque, rendered at 128×128 through Alpha Over and through a Mix node, small-first and large-first, at Mix Factors 0, 0.5 and 1. The frame was read two ways: by the area whose alpha is above zero, and by feeding the node's output into an Image Info node and writing its Dimensions output. Word for word:

```
SPLIT position and rotation left at: (0.5, 0.5) 0.7853981852531433
   SPLIT left column x=5 y=32 -> [0.07937, 0.50792, 0.25, 1.0]   (picA there [0.07937, 0.50792, 0.25, 1.0], picB there [0.1, 0.92062, 0.80001, 1.0])
   SPLIT right column x=58 y=32 -> [0.1, 0.07937, 0.80001, 1.0]   (picA there [0.92062, 0.50792, 0.25, 1.0], picB there [0.1, 0.07937, 0.80001, 1.0])
DEPTH COMBINE first=picA(depth 0.2) second=picB(depth 0.8): maxdiff vs picA=0.0 maxdiff vs picB=1.000038743019104
DEPTH COMBINE first=picA(depth 0.8) second=picB(depth 0.2): maxdiff vs picA=1.000038743019104 maxdiff vs picB=0.0
DEPTH COMBINE with a depth running 0 on the left to 1 on the right for picA, 0.5 flat for picB:
   DEPTHGRAD left column x=5 y=32 -> [0.07937, 0.50792, 0.25, 1.0]   (picA there [0.07937, 0.50792, 0.25, 1.0], picB there [0.1, 0.92062, 0.80001, 1.0])
   DEPTHGRAD right column x=58 y=32 -> [0.1, 0.07937, 0.80001, 1.0]   (picA there [0.92062, 0.50792, 0.25, 1.0], picB there [0.1, 0.07937, 0.80001, 1.0])
   maxdiff vs picA=1.000038743019104 maxdiff vs picB=1.000038743019104
picSmall lowest alpha 1.0 ; picLarge lowest alpha 1.0
AlphaOver(bg,fg) Fac 1.0 first input = picSmall(32), second = picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]
AlphaOver(bg,fg) Fac 1.0 first input = picLarge(128), second = picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)   [render size (128, 128)]
Mix(A,B)  FACTOR SET 0.0  FACTOR HELD ON THE NODE 0.0  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]
IMAGE INFO NODE USED CompositorNodeImageInfo outputs [('Dimensions', 'Dimensions', 'INT_VECTOR'), ('Resolution', 'Resolution', 'INT_VECTOR'), ('Location', 'Location', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Scale', 'Scale', 'VECTOR')]
Mix(A,B)  FACTOR 0.0  A=picSmall(32) B=picLarge(128) -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]   at pixel (64,64) [32.0, 32.0, 0.0, 1.0]
Mix(A,B)  FACTOR SET 0.0  FACTOR HELD ON THE NODE 0.0  A=picLarge(128) B=picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)   [render size (128, 128)]
Mix(A,B)  FACTOR 0.0  A=picLarge(128) B=picSmall(32) -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]   at pixel (64,64) [128.0, 128.0, 0.0, 1.0]
Mix(A,B)  FACTOR SET 0.5  FACTOR HELD ON THE NODE 0.5  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]
Mix(A,B)  FACTOR 0.5  A=picSmall(32) B=picLarge(128) -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]   at pixel (64,64) [32.0, 32.0, 0.0, 1.0]
Mix(A,B)  FACTOR SET 0.5  FACTOR HELD ON THE NODE 0.5  A=picLarge(128) B=picSmall(32) -> alpha covers x 0..127, y 0..127  (128 by 128)   [render size (128, 128)]
Mix(A,B)  FACTOR 0.5  A=picLarge(128) B=picSmall(32) -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]   at pixel (64,64) [128.0, 128.0, 0.0, 1.0]
Mix(A,B)  FACTOR SET 1.0  FACTOR HELD ON THE NODE 1.0  A=picSmall(32) B=picLarge(128) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]
Mix(A,B)  FACTOR 1.0  A=picSmall(32) B=picLarge(128) -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]   at pixel (64,64) [32.0, 32.0, 0.0, 1.0]
Mix(A,B)  FACTOR SET 1.0  FACTOR HELD ON THE NODE 1.0  A=picLarge(128) B=picSmall(32) -> alpha covers x 48..79, y 48..79  (32 by 32)   [render size (128, 128)]
Mix(A,B)  FACTOR 1.0  A=picLarge(128) B=picSmall(32) -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]   at pixel (64,64) [128.0, 128.0, 0.0, 1.0]
CONTROL  picSmall(32) alone -> Image Info Dimensions at pixel (0,0) [32.0, 32.0, 0.0, 1.0]
CONTROL  picLarge(128) alone -> Image Info Dimensions at pixel (0,0) [128.0, 128.0, 0.0, 1.0]
```

The Split node put its first picture on the left of the frame and its second on the right. Depth Combine took, pixel by pixel, the picture whose depth was less: picA whole where its depth was 0.2, picB whole where picA's was 0.8, and with a depth gradient picA on the left and picB on the right. Alpha Over's frame followed its Background. **The Mix node's frame, read straight off the Image Info node, followed its first colour input, A, at every Factor**, both ways round, with the controls reporting 32 and 128; read by alpha instead, the large-first arrangement at Factor 1 covered only 32 by 32, so the alpha reading and the frame reading part company there. No property of any of these nodes names which input decides the frame; the Image Info node is what reported it, with no window.

**Entry 27 — the Viewer node with no window, walking into a node group, a File Output item's name, a faint mark through a colour change, and metadata on nodes.** A 64×64 scene. A group with a Viewer fed by the render, read before and after a render; picA into a Group node whose inner group is Group Input → Hue/Saturation/Value → Group Output, the links walked from the picture node; a File Output node aimed at an empty folder, given one input item named `MyBranchName` fed by picA, its file name left at its factory value; picA with and without a faint square mark (0.02 brighter in red, green and blue over x and y 20 to 39), each put through a Hue/Saturation/Value node with its Hue at 0; and the property names of three nodes. The first run stopped at `RuntimeError: Error: Cannot read '.../t27/graded_marked.exr': No such file or directory`, because a helper had put the scene's file format back to PNG; the format was set to OpenEXR again and the whole script rerun. Its lines before the stop were the same in both runs. The second run, word for word:

```
BLENDER_VERSION 5.3.0 Beta ; background: True
VIEWER IMAGE before the render: size=(256, 256) pixels=65536  min=0.0 max=0.0
VIEWER IMAGE after the render: size=(256, 256) pixels=65536  min=0.0 max=0.0
   image 'Render Result' source=VIEWER size=(0, 0)
   image 'Viewer Node' source=VIEWER size=(256, 256)
WALK FROM THE PICTURE NODE, in the outer tree:
   link -> node 'Group' (CompositorNodeGroup), socket 'Socket_0'
   the group node's node_tree is 'Inner'; walking inside it:
      inner link: NodeGroupInput.Socket_0 -> CompositorNodeHueSat.Image
      inner link: CompositorNodeHueSat.Image -> NodeGroupOutput.Socket_1
   any link in the OUTER tree that reaches a node inside the group: []
file_output_items after: ['MyBranchName']
file_name is '{blend_name}'
FILE OUTPUT input sockets: [('MyBranchName', 'Item_1'), ('', '__extend__')]
WHAT LANDED IN THE FILE OUTPUT FOLDER: ['Unsaved.exr']
BEFORE any colour change: biggest difference inside the mark = 0.020008, outside it = 0.000000
AFTER the colour change  : biggest difference inside the mark = 0.020013, outside it = 0.000000
MARKED-AND-GRADED against the UNMARKED ORIGINAL: inside the mark = 0.389058, outside it = 1.000039
CompositorNodeImage: properties naming metadata -> none
   its input socket names: []
   its output socket names: [('Image', 'Image', 'RGBA'), ('Alpha', 'Alpha', 'VALUE')]
CompositorNodeOutputFile: properties naming metadata -> none
   its input socket names: [('', '__extend__', 'CUSTOM')]
   its output socket names: []
CompositorNodeImageInfo: properties naming metadata -> none
   its input socket names: [('Image', 'Image', 'RGBA')]
   its output socket names: [('Dimensions', 'Dimensions', 'INT_VECTOR'), ('Resolution', 'Resolution', 'INT_VECTOR'), ('Location', 'Location', 'VECTOR'), ('Rotation', 'Rotation', 'VALUE'), ('Scale', 'Scale', 'VECTOR')]
```

With no window, the Viewer node's image existed at 256×256 with every value zero both before and after a 64×64 render. A walk forward from a picture reached the Group node in the outer tree, and continued only by stepping into the Group node's own group; no outer link reaches inside. The File Output item's name did not reach the written file's name: the file was named from the node's file-name setting, `{blend_name}`, which gave `Unsaved` for an unsaved file. The faint mark kept its size through the colour change — 0.02 brighter inside, nothing outside — while the graded picture as a whole differed from the original. None of the three nodes has a property naming metadata.

**Entry 28 — reading the whole graph out of a saved file with a second Blender.** One Blender built a group and saved it as `graph.blend`, rendering nothing: picA's Image node → a Hue/Saturation/Value node `Colour` → an Alpha Over `TwoIn` (Background), picB's node `Image.001` → `TwoIn` (Foreground), `TwoIn` → Group Output; and a Switch `Chooser` with picA on Off and picB on On. A second, separate Blender opened that file with no window and printed it. Word for word:

```
VERSION 5.3.0 Beta ; background: True
FILE /private/tmp/claude-501/smoozy-hand-tests-5.3/The compositor/blends/5.3/graph.blend
TREE bpy.data.node_groups['Comp']
NODES [('Chooser', 'CompositorNodeSwitch'), ('Colour', 'CompositorNodeHueSat'), ('Group Output', 'NodeGroupOutput'), ('Image', 'CompositorNodeImage'), ('Image.001', 'CompositorNodeImage'), ('TwoIn', 'CompositorNodeAlphaOver')]
LINK Colour.Image -> TwoIn.Background   muted=False valid=True
LINK Image.Image -> Chooser.Off   muted=False valid=True
LINK Image.Image -> Colour.Image   muted=False valid=True
LINK Image.001.Image -> Chooser.On   muted=False valid=True
LINK Image.001.Image -> TwoIn.Foreground   muted=False valid=True
LINK TwoIn.Image -> Group Output.Socket_0   muted=False valid=True
CHOOSER stored choice: [('Switch', 'Switch', False), ('Off', 'Off', 'colour'), ('On', 'On', 'colour')]
CHOOSER internal_links: [('Off', 'Image')]
TWOIN inputs: [('Background', 'Background', True, False), ('Foreground', 'Foreground', True, False), ('Factor', 'Fac', False, False), ('Type', 'Type', False, False), ('Straight Alpha', 'Straight Alpha', False, False)]
TWOIN internal_links: [('Background', 'Image')]
SCENE EFFECTS: [('Scene', [('Scene Effect', 'Comp')])]
```

Every node and every link came back with both its ends named, along with the Switch's stored choice (`False`, so Off) and each node's internal link, read from a saved file by a Blender with no window. The scene's effects list, also saved, named the group.

**Entry 40 — whether a window changes anything.** One script was run twice on entry 28's saved `graph.blend`: once in a Blender started with `-b`, and once in a Blender started with its window open (`--enable-event-simulate`, the splash closed with a simulated Escape, the script run on Blender's own timer). It printed the saved graph, the property names on a Hue/Saturation/Value node's output socket, the Viewer node's image before and after a 64×64 render of a scene whose group feeds Render Layers to a Viewer and to the Group Output, and four renders — Alpha Over at Factor 0 and 1, Switch off and on, over picA and picB — each compared with entry 24's no-window render of the same tree. The windowed run then switched to the Compositing workspace and did the Viewer check again. The two runs' printouts were identical line for line apart from the background flag and the Compositing-workspace lines, which only the windowed run has. Word for word, the lines that differ or carry the answer:

```
--- with -b
BACKGROUND FLAG: True ; window present: True
VERSION 5.3.0 Beta ; background: True
VIEWER IMAGE before the render (Layout workspace): size=(256, 256) pixels=65536  min=0.0 max=0.0
VIEWER IMAGE after the render (Layout workspace): size=(256, 256) pixels=65536  min=0.0 max=0.0
PAIR ao_0.0.exr   vs entry 24's ao_0.0.exr     sizes (64, 64) (64, 64)  MAXDIFF = 0.0
PAIR ao_1.0.exr   vs entry 24's ao_1.0.exr     sizes (64, 64) (64, 64)  MAXDIFF = 0.0
PAIR sw_False.exr vs entry 24's sw_False.exr   sizes (64, 64) (64, 64)  MAXDIFF = 0.0
PAIR sw_True.exr  vs entry 24's sw_True.exr    sizes (64, 64) (64, 64)  MAXDIFF = 0.0
--- with the window
BACKGROUND FLAG: False ; window present: True
VERSION 5.3.0 Beta ; background: False
VIEWER IMAGE before the render (Layout workspace): size=(256, 256) pixels=65536  min=0.0 max=0.0
VIEWER IMAGE after the render (Layout workspace): size=(256, 256) pixels=65536  min=0.0 max=0.0
PAIR ao_0.0.exr   vs entry 24's ao_0.0.exr     sizes (64, 64) (64, 64)  MAXDIFF = 0.0
PAIR ao_1.0.exr   vs entry 24's ao_1.0.exr     sizes (64, 64) (64, 64)  MAXDIFF = 0.0
PAIR sw_False.exr vs entry 24's sw_False.exr   sizes (64, 64) (64, 64)  MAXDIFF = 0.0
PAIR sw_True.exr  vs entry 24's sw_True.exr    sizes (64, 64) (64, 64)  MAXDIFF = 0.0
VIEWER IMAGE before the render (Compositing workspace): size=(64, 64) pixels=4096  min=0.0 max=1.000038743019104
VIEWER IMAGE after the render (Compositing workspace): size=(64, 64) pixels=4096  min=0.0 max=0.0
```

The graph printout (every node, link, stored choice and internal link — the same lines as entry 28) and the socket property names (the same list as entry 9) came back identical with the window and without; nothing about the inspection display became reachable from a script with a window open. **The four rendered pictures were the same with the window and without, to the last figure.** **The Viewer node's image**: with no window, and with the window in the Layout workspace, it stayed at 256×256 with every value zero after the render. In the Compositing workspace, which shows an image editor and the compositor, it was already 64×64 with values up to about 1 before the render was started, and was 64×64 with every value zero after it. A second windowed run of the Viewer half alone, on a fresh file rather than `graph.blend`, gave the same pattern: Layout `size=(256, 256) ... max=0.0` after the render; Compositing `size=(64, 64) pixels=4096  min=0.07989501953125 max=0.998046875` before it and `size=(64, 64) pixels=4096  min=0.0 max=0.0` after it. This bears on the 5.3 release note quoted under question 1, "Rendering no longer writes to viewer images, however, the interactive compositor is executed after rendering if the viewer is visible"; it is recorded rather than explained.

On this Mac a Blender started with `-b` still reported a window to the script (`window present: True`), so that line does not tell the two runs apart; the background flag and the Viewer lines do.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

**Label:**
- Where the settings live: documented and observed.
- Which scene's settings are used for the second scene's render: **not in the sources**.
- The effect of outside values: **not in the sources**, apart from what the arguments themselves say.

**Where engine and sample settings live.** On each scene, separately.
- `engine`: "Engine to use for rendering (default 'BLENDER_EEVEE')", on RenderSettings, "Rendering settings for a Scene data-block" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- "The active render engine can be selected in the Render Properties." / "Each renderer has its own render settings to control render quality and performance." — manual, https://docs.blender.org/manual/en/5.3/render/render_engines.html
- **Cycles samples (observed).** Cycles' file `properties.py`, class `CyclesRenderSettings`:
  ```
  samples: IntProperty(
      name="Samples",
      description="Number of samples to render for each pixel",
      min=1, max=(1 << 24),
      default=4096,
  )
  ```
  In the same class's `register`:
  ```
  bpy.types.Scene.cycles = PointerProperty(
      name="Cycles Render Settings",
      description="Cycles render settings",
      type=cls,
  )
  ```
  So each scene carries its own Cycles sample count. The file is `cycles_5.3/properties.py`, in class `CyclesRenderSettings`: the `samples` property sits near line 533 and `register` near line 1190. The file was taken 2026-10-16.
- **EEVEE samples.** `taa_render_samples`: "Number of samples per pixel for rendering (in [1, inf], default 64)", on SceneEEVEE, reached as `Scene.eevee` — API, https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html
- **Per view layer override.** `samples`: "Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html
- "Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used." — manual, https://docs.blender.org/manual/en/5.3/render/layers/override.html
- Cycles' `use_layer_samples`: default `'USE'`, described as "Per render layer number of samples override scene samples" (observed, `properties.py`, `enum_use_layer_samples` near line 66 and `use_layer_samples` near line 604).
- The Render Layers node picks which view layer to render: "View Layer — The view layer to render." — manual, render_layers page. Inferred: the chosen view layer's override can change the count.

**Which scene's settings render the second scene.** Not in the sources.
- The sources do not say whether a scene rendered through a Render Layers node, or a Scene strip, uses its own engine and samples or the first scene's.
- Closest statements:
  - `use_sequencer_override_scene_strip`: "Use Workbench render and world settings from the sequencer scene, instead of each strip's scene (default False)" (API). Inferred: for Scene strips, by default each strip's own scene's Workbench settings are used. This says nothing about the engine choice, Cycles or EEVEE samples, or Render Layers nodes.
  - The Sequencer preview's "Override Scene Settings — Use the Workbench render settings from the current scene rather than the scenes referenced by the strips. Only available for the Wireframe and Solid shading modes." — https://docs.blender.org/manual/en/5.3/editors/video_sequencer/preview/sidebar.html. This is preview only.
  - `use_render_cache`: "… Note: affects indirectly rendered scenes" (API). This is the only statement found where a setting reaches "indirectly rendered scenes". It concerns the render cache, not the engine or samples.
  - The API's example render engine reads its settings from the scene of the dependency graph it is given: `scene = depsgraph.scene` then `scene.render.resolution_percentage` (https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html). Cycles' `engine.py` likewise passes the dependency graph it receives: `def render(engine, depsgraph): … _cycles.render(engine.session, depsgraph.as_pointer())` (observed, `cycles_5.3/engine.py`). The sources do not say which scene's dependency graph an indirect render is given.

**Values given from outside.**
- **Engine.**
  - "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html
  - "-E CYCLES — Use the 'Cycles Render' engine." — https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html
  - Neither page says which scene or scenes `-E` applies to. So its effect on a second scene that is rendered indirectly is not in the sources.
  - "-S, --scene <name> — Set the active scene <name> for rendering."
- **Samples.**
  - No command-line argument for samples is documented. I searched both command-line pages for "sample".
  - Cycles' own argument reader accepts only `--cycles-print-stats` and `--cycles-device` (observed, `cycles_5.3/engine.py`, function `_configure_argument_parser`). The `--cycles-device` help reads: "Set the device to use for Cycles, overriding user preferences and the scene setting." That covers the device, not samples.
  - A sample count could only come in through a script (for example `--python-expr`), which sets the property on whichever scene the script names. Inferred: because the setting is per scene, setting it on the first scene does not change the second scene's own value. Which value the second scene's render then uses is not in the sources.
- Argument order: "Arguments are executed in the order they are given" and "loading the blend-file overwrites the render output that was set" (arguments page). This is stated for the output path. Whether `-E` given before the file name is likewise overwritten is not stated.

**Not covered:**
- Everything about what actually happens. The sources give where each setting lives, but not how an indirect render picks between the first scene's settings and the second's, with or without values from outside.

**Where I looked:**
- Pages: command-line Arguments and Rendering From The Command Line, Render Engines, Cycles Sampling, EEVEE Sampling, View Layer Override, View Layer, Render Layers node, Scene strip, Sequencer Scene, Sequencer preview sidebar.
- API pages: RenderSettings, Scene, SceneEEVEE, ViewLayer, RenderEngine, `bpy.ops.render`, `bpy.app.handlers`, info_advanced_blender_as_bpy.
- Cycles files: `__init__.py`, `engine.py`, and in `properties.py` the samples and registration parts.
- The release notes for 5.3: compositor, Cycles, rendering, core, sequencer, Python API.
- Searches: "another scene", "other scene", "each scene", "indirectly", "own render settings", "render settings of", "command line" (API), "sample" (command-line pages).
- I looked at the Cycles `source/` folder and ruled it out. It holds only kernel and utility headers about the rendering itself, not how scenes are picked.

**Outside the interface:**
- This whole question is about outside values. The only documented outside routes are `-E`, `-S`, `--cycles-device`, and Python scripts. None of their documentation addresses a second, indirectly rendered scene.

**Operating system:** The sources do not say engine or sample choice differs by operating system. The `--cycles-device` values ("CPU CUDA OPTIX HIP ONEAPI METAL") name device kinds, which differ by platform, but that is the device, not the engine or samples.

**Driven from outside — observed, 2026-10-18.** Entries 29 to 32 ran with no window throughout and answer the outside values directly: `-E` reaches the active scene only, a sampling argument is neither taken nor refused, a script reaches the scene behind the node. A render started inside a windowed Blender gave the same picture as the command line (entry 41). The node's re-render button, pressed by a simulated click, rendered the scene behind the node with that scene's engine; its sample count was not read.

**Operating system — observed, 2026-10-18.** Settled on macOS alone (the Creator's Mac, Apple M1 Pro); Linux and Windows were not tried in this sitting, and no third-party render engine was involved (every run used `--factory-startup`).

### Observed — hand tests on the Creator's Mac, 2026-10-18

**Label: observed**, for this section only. Everything above stays as it is, including each *not in the sources*, which is a statement about the documentation; what follows is what Blender 5.3 did. Same machine, same Blender and same way of starting it as question 1's observed section: the Creator's Mac (Apple M1 Pro, 14-core Apple graphics, 16 GB, macOS 26.6.2), Blender 5.3.0 Beta (hash `5d8151335b6b`) from a terminal with `--factory-startup` and its own settings folders. Entry numbers are those of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`.

**Entry 4 — what the Render Layers node carries, where an engine and a sample count can live, and what the command line accepts.** Asked of the running program, word for word:

```
BLENDER_VERSION 5.3.0 Beta
RLAYERS_OWN_PROPS: ['scene', 'layer']
VIEWLAYER_ENGINE_PROPS: []
VIEWLAYER_SAMPLE_PROPS: ['samples']
RENDERSETTINGS_ENGINE: ['engine', 'has_multiple_engines']
```

The node has two settings of its own; a view layer has no engine setting and one sample setting. `--help` (exit status 0) lists under *Render Options* exactly `-b`/`--background`, `-a`/`--render-anim`, `-S`/`--scene`, `-f`/`--render-frame`, `-s`/`--frame-start`, `-e`/`--frame-end`, `-j`/`--frame-jump`, `-o`/`--render-output` and `-E`/`--engine`, `-t`/`--threads`; its *Cycles Render Options* section, word for word:

```
Cycles Render Options:
	Cycles add-on options must be specified following a double dash.

--cycles-device <device>
	Set the device used for rendering.
	Valid options are: 'CPU' 'CUDA' 'OPTIX' 'HIP' 'ONEAPI' 'METAL'.

	Append +CPU to a GPU device to render on both CPU and GPU.

	Example:
	# blender -b file.blend -f 20 -- --cycles-device OPTIX
--cycles-print-stats
	Log statistics about render memory and time usage.
```

Nothing in either section sets a sample count. The same help also lists `--log <match>` and `--log-level <level>` under *Logging Options*. `-E help` printed, and exited with status 0:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

**What entries 29 to 32 were run against.** One set of files, built by Blender 5.3.0 Beta itself so that no file crossed a version boundary. Each holds two scenes: **Comp**, whose compositor runs, and **Sub**, the scene pulled in. Both hold a camera, one area light, a metallic sphere of roughness 0.35 and a blue floor, with the two spheres in different places. Comp's compositor group is one Render Layers node with its Scene set to Sub and its View Layer set to Sub's `ViewLayer`, wired straight to the Group Output — so the written image is entirely Sub's render. Both scenes 160×120, one frame, 32-bit OpenEXR with no compression, Compositing on, stamping off, Cycles on the processor with denoising off, adaptive sampling off, no time limit and a seed of 7. Only the engines, sample counts, view-layer overrides and Layer Samples choices differ from file to file; Comp is the active scene in every file. Every render was made from the terminal as `blender --factory-startup -b <file>.blend --log "render" [arguments] -o <output>_ -f 1`, with `-f 1` last. A **reference** is the same file rendered with `-S Sub` added, so that Sub renders directly and no compositor is involved. Pictures were compared value by value: `maxdiff` is the largest difference in any channel of any pixel, and 0 means the same picture to the last figure. Of Blender's progress lines, only the last sample line of each render is kept below.

**Entry 29 — the engine the scene behind the node is rendered with.** Four files, each with a different engine on Comp and on Sub (wb Workbench, cy Cycles at 64 samples, ee EEVEE at 16): `eng_Awb_Bcy`, `eng_Acy_Bwb`, `eng_Aee_Bcy`, `eng_Acy_Bee`. Each rendered through the node and directly. Word for word:

```
----- eng_Awb_Bcy.blend  -> eng_Awb_Bcy_0001.exr
00:00.331  render           | Start rendering: Sub, ViewLayer
00:00.331  render           | Engine: Cycles
00:00.578  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- eng_Awb_Bcy.blend -S Sub -> eng_Awb_Bcy_direct_0001.exr
00:00.329  render           | Start rendering: Sub, ViewLayer
00:00.329  render           | Engine: Cycles
00:00.577  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- eng_Acy_Bwb.blend  -> eng_Acy_Bwb_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Workbench
exit 0
----- eng_Acy_Bwb.blend -S Sub -> eng_Acy_Bwb_direct_0001.exr
00:00.326  render           | Start rendering: Sub, ViewLayer
00:00.326  render           | Engine: Workbench
exit 0
----- eng_Aee_Bcy.blend  -> eng_Aee_Bcy_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:00.575  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- eng_Aee_Bcy.blend -S Sub -> eng_Aee_Bcy_direct_0001.exr
00:00.331  render           | Start rendering: Sub, ViewLayer
00:00.331  render           | Engine: Cycles
00:00.580  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- eng_Acy_Bee.blend  -> eng_Acy_Bee_0001.exr
00:00.330  render           | Start rendering: Sub, ViewLayer
00:00.330  render           | Engine: EEVEE
00:01.402  render           | Fra: 1 | Rendering 1 / 16 samples
00:01.431  render           | Fra: 1 | Rendering 16 / 16 samples
exit 0
----- eng_Acy_Bee.blend -S Sub -> eng_Acy_Bee_direct_0001.exr
00:00.335  render           | Start rendering: Sub, ViewLayer
00:00.335  render           | Engine: EEVEE
00:00.474  render           | Fra: 1 | Rendering 1 / 16 samples
00:00.553  render           | Fra: 1 | Rendering 16 / 16 samples
exit 0
CMP eng_Awb_Bcy_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=0 meandiff=0
CMP eng_Awb_Bcy_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=28.2201 meandiff=0.409324
CMP eng_Acy_Bwb_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=0 meandiff=0
CMP eng_Acy_Bwb_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=28.2201 meandiff=0.409324
CMP eng_Aee_Bcy_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=0 meandiff=0
CMP eng_Aee_Bcy_0001.exr vs eng_Acy_Bee_direct_0001.exr  maxdiff=1.87869 meandiff=0.047416
CMP eng_Acy_Bee_0001.exr vs eng_Acy_Bee_direct_0001.exr  maxdiff=0 meandiff=0
CMP eng_Acy_Bee_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=1.87869 meandiff=0.047416
```

The references used: Sub on Cycles at 64 is `eng_Awb_Bcy_direct`; Sub on Workbench is `eng_Acy_Bwb_direct`; Sub on EEVEE at 16 is `eng_Acy_Bee_direct`. **The scene pulled in through the node was rendered with the engine stored on that scene**, in both directions for all three engines: Blender named Sub's engine every time, and each image matched the reference for Sub's engine at `maxdiff=0` while differing from the reference for Comp's. Only one `Start rendering:` line appeared in any run, always naming Sub; Comp's own view layer was not rendered.

**Entry 30 — the sample count the scene behind the node is rendered with.** Cycles files `smp_A1_B512`, `smp_A512_B1`, `smp_A4096_B128`, `smp_A1_B128` and EEVEE files `ee_A4_B64`, `ee_A64_B4`, A being Comp's count and B Sub's. Word for word (the EEVEE progress lines are kept whole; for Cycles only the last sample line):

```
----- smp_A1_B512.blend  -> smp_A1_B512_0001.exr
00:00.330  render           | Start rendering: Sub, ViewLayer
00:00.330  render           | Engine: Cycles
00:02.249  render           | Fra: 1 | Mem: 1M | Sample 512/512
exit 0
----- smp_A512_B1.blend  -> smp_A512_B1_0001.exr
00:00.332  render           | Start rendering: Sub, ViewLayer
00:00.332  render           | Engine: Cycles
00:00.342  render           | Fra: 1 | Mem: 1M | Sample 1/1
exit 0
----- smp_A4096_B128.blend  -> smp_A4096_B128_0001.exr
00:00.326  render           | Start rendering: Sub, ViewLayer
00:00.326  render           | Engine: Cycles
00:00.810  render           | Fra: 1 | Mem: 1M | Sample 128/128
exit 0
----- smp_A1_B128.blend  -> smp_A1_B128_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:00.818  render           | Fra: 1 | Mem: 1M | Sample 128/128
exit 0
----- ee_A4_B64.blend  -> ee_A4_B64_0001.exr
00:00.329  render           | Start rendering: Sub, ViewLayer
00:00.329  render           | Engine: EEVEE
00:00.462  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.557  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.594  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.614  render           | Fra: 1 | Rendering 64 / 64 samples
exit 0
----- ee_A64_B4.blend  -> ee_A64_B4_0001.exr
00:00.332  render           | Start rendering: Sub, ViewLayer
00:00.332  render           | Engine: EEVEE
00:00.465  render           | Fra: 1 | Rendering 1 / 4 samples
00:00.523  render           | Fra: 1 | Rendering 4 / 4 samples
exit 0
CMP smp_A1_B512_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0 meandiff=0
CMP smp_A1_B512_0001.exr vs smp_A512_B1_direct_0001.exr  maxdiff=39.0279 meandiff=0.165835
CMP smp_A512_B1_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=39.0279 meandiff=0.165835
CMP smp_A512_B1_0001.exr vs smp_A512_B1_direct_0001.exr  maxdiff=0 meandiff=0
CMP smp_A4096_B128_0001.exr vs smp_A1_B128_0001.exr  maxdiff=0 meandiff=0
CMP smp_A4096_B128_0001.exr vs smp_A1_B128_direct_0001.exr  maxdiff=0 meandiff=0
CMP smp_A1_B128_0001.exr vs smp_A1_B128_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_A4_B64_0001.exr vs ee_A4_B64_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_A4_B64_0001.exr vs ee_A64_B4_direct_0001.exr  maxdiff=0.665039 meandiff=0.0045333
CMP ee_A64_B4_0001.exr vs ee_A4_B64_direct_0001.exr  maxdiff=0.665039 meandiff=0.0045333
CMP ee_A64_B4_0001.exr vs ee_A64_B4_direct_0001.exr  maxdiff=0 meandiff=0
```

(The direct renders printed the same sample counts as the through-node ones; they are in the hand-test list.) Under Cycles and under EEVEE, **the count used was Sub's own** and Comp's never appeared; each image matched the reference made at Sub's count and differed from the one made at the other. With Sub held at 128, raising Comp's own count from 1 to 4096 changed no pixel of the written image.

**Entry 31 — a view layer's sample override through the node, whose Layer Samples policy governs it, and EEVEE's override.** Cycles files with Comp at 64 and Sub at 512, Sub's view layer given a sample override, and Sub's (and in two files Comp's) Layer Samples policy set: `vl_use` (override 4, Use), `vl_ignore` (4, Ignore), `vl_bounded_lo` (4, Bounded), `vl_bounded_hi` (4096, Bounded), `vl_use_hi` (4096, Use), `pol_Bignore_Ause` (Sub Ignore, Comp Use, override 4), `pol_Buse_Aignore` (Sub Use, Comp Ignore, override 4); and `ee_vl`, EEVEE, Comp at 4, Sub at 64, Sub's view layer overridden to 8. References: Sub at 512 (`smp_A1_B512_direct`), Sub at 4 (`ref_cy4`, a file with Sub at 4 and no override, rendered directly), EEVEE Sub at 64 (`ee_A4_B64_direct`) and at 8 (`ref_ee8`, Sub at 8 and no override, rendered directly). Word for word:

```
----- vl_use.blend  -> vl_use_0001.exr
00:00.328  render           | Start rendering: Sub, ViewLayer
00:00.328  render           | Engine: Cycles
00:00.349  render           | Fra: 1 | Mem: 1M | Sample 4/4
exit 0
----- vl_ignore.blend  -> vl_ignore_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:02.250  render           | Fra: 1 | Mem: 1M | Sample 512/512
exit 0
----- vl_bounded_lo.blend  -> vl_bounded_lo_0001.exr
00:00.329  render           | Start rendering: Sub, ViewLayer
00:00.329  render           | Engine: Cycles
00:00.351  render           | Fra: 1 | Mem: 1M | Sample 4/4
exit 0
----- vl_bounded_hi.blend  -> vl_bounded_hi_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:02.268  render           | Fra: 1 | Mem: 1M | Sample 512/512
exit 0
----- vl_use_hi.blend  -> vl_use_hi_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:15.645  render           | Fra: 1 | Mem: 1M | Sample 4096/4096
exit 0
----- pol_Bignore_Ause.blend  -> pol_Bignore_Ause_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:02.260  render           | Fra: 1 | Mem: 1M | Sample 512/512
exit 0
----- pol_Buse_Aignore.blend  -> pol_Buse_Aignore_0001.exr
00:00.325  render           | Start rendering: Sub, ViewLayer
00:00.325  render           | Engine: Cycles
00:00.347  render           | Fra: 1 | Mem: 1M | Sample 4/4
exit 0
----- ee_vl.blend  -> ee_vl_0001.exr
00:00.330  render           | Start rendering: Sub, ViewLayer
00:00.330  render           | Engine: EEVEE
00:00.463  render           | Fra: 1 | Rendering 1 / 8 samples
00:00.531  render           | Fra: 1 | Rendering 8 / 8 samples
exit 0
CMP vl_use_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=13.7012 meandiff=0.0668337
CMP vl_use_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=0 meandiff=0
CMP vl_ignore_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0 meandiff=0
CMP vl_ignore_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=13.7012 meandiff=0.0668337
CMP vl_bounded_lo_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=13.7012 meandiff=0.0668337
CMP vl_bounded_lo_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=0 meandiff=0
CMP vl_bounded_hi_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0 meandiff=0
CMP vl_bounded_hi_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=13.7012 meandiff=0.0668337
CMP vl_use_hi_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0.655035 meandiff=0.00400674
CMP vl_use_hi_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=13.4448 meandiff=0.0667016
CMP pol_Bignore_Ause_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0 meandiff=0
CMP pol_Bignore_Ause_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=13.7012 meandiff=0.0668337
CMP pol_Buse_Aignore_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=13.7012 meandiff=0.0668337
CMP pol_Buse_Aignore_0001.exr vs ref_cy4_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_vl_0001.exr vs ref_ee8_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_vl_0001.exr vs ee_A4_B64_direct_0001.exr  maxdiff=0.298828 meandiff=0.00264319
```

**The view layer's override reached through the node, and Sub's own policy governed it**: Use took the override at either size, Ignore discarded it, Bounded took 4 and kept 512 where the override was 4096 — five distinct outcomes, each matching its reference at `maxdiff=0` except `vl_use_hi`, whose 4,096 samples are a picture of their own. With the two scenes given opposite policies the count followed Sub's and never Comp's. **Under EEVEE the override was honoured**: `ee_vl` rendered 8 samples and matched the 8-sample reference exactly.

**Entries 29 to 31 — a scene reached through the node renders as it does when rendered directly.** Ten of the files were rendered both ways, with Comp active and with `-S Sub`. Word for word:

```
CMP eng_Awb_Bcy_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=0 meandiff=0
CMP eng_Acy_Bwb_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=0 meandiff=0
CMP eng_Acy_Bee_0001.exr vs eng_Acy_Bee_direct_0001.exr  maxdiff=0 meandiff=0
CMP smp_A1_B512_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0 meandiff=0
CMP smp_A512_B1_0001.exr vs smp_A512_B1_direct_0001.exr  maxdiff=0 meandiff=0
CMP smp_A1_B128_0001.exr vs smp_A1_B128_direct_0001.exr  maxdiff=0 meandiff=0
CMP vl_use_0001.exr vs vl_use_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_A4_B64_0001.exr vs ee_A4_B64_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_A64_B4_0001.exr vs ee_A64_B4_direct_0001.exr  maxdiff=0 meandiff=0
CMP ee_vl_0001.exr vs ee_vl_direct_0001.exr  maxdiff=0 meandiff=0
```

All ten pairs, covering all three engines, four Cycles counts, two EEVEE counts and an override under each engine, were the same picture to the last figure.

**Entry 32 — values given from outside.** Files `E_bothcy`, `E_bothwb`, `E_bothee` hold the same engine on both scenes (Cycles at 64, Workbench, EEVEE at 16), so any difference comes from the argument; `smp_A1_B512` as in entry 30; and the 5.3 researcher's own `r7_A_ee_B_cy2`, Comp on EEVEE and Sub on Cycles at 2 samples, rendered with the three commands that researcher wrote (with `-S Comp`, Comp being this file's name for the researcher's scene A). References as in entry 29 (Cycles 64 `eng_Awb_Bcy_direct`, Workbench `eng_Acy_Bwb_direct`, EEVEE 16 `eng_Acy_Bee_direct`) and Sub rendered directly. `<print>` is a one-line script printing each scene's stored engine. Word for word:

```
----- E_bothcy.blend -E BLENDER_WORKBENCH -> E_cy_wb_0001.exr
00:00.330  render           | Start rendering: Sub, ViewLayer
00:00.330  render           | Engine: Cycles
00:00.574  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- E_bothcy.blend -E BLENDER_EEVEE -> E_cy_ee_0001.exr
00:00.328  render           | Start rendering: Sub, ViewLayer
00:00.328  render           | Engine: Cycles
00:00.574  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- E_bothwb.blend -E CYCLES -> E_wb_cy_0001.exr
00:00.333  render           | Start rendering: Sub, ViewLayer
00:00.333  render           | Engine: Workbench
exit 0
----- E_bothee.blend -E CYCLES -> E_ee_cy_0001.exr
00:00.328  render           | Start rendering: Sub, ViewLayer
00:00.328  render           | Engine: EEVEE
00:00.462  render           | Fra: 1 | Rendering 1 / 16 samples
00:00.539  render           | Fra: 1 | Rendering 16 / 16 samples
exit 0
CMP E_cy_wb_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=0 meandiff=0
CMP E_cy_wb_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=28.2201 meandiff=0.409324
CMP E_cy_ee_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=0 meandiff=0
CMP E_cy_ee_0001.exr vs eng_Acy_Bee_direct_0001.exr  maxdiff=1.87869 meandiff=0.047416
CMP E_wb_cy_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=0 meandiff=0
CMP E_wb_cy_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=28.2201 meandiff=0.409324
CMP E_ee_cy_0001.exr vs eng_Acy_Bee_direct_0001.exr  maxdiff=0 meandiff=0
CMP E_ee_cy_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=1.87869 meandiff=0.047416
$ blender --factory-startup -b E_bothcy.blend --python-expr <print>   (no -E)
SCENES: [('Comp', 'CYCLES'), ('Sub', 'CYCLES')]
$ blender --factory-startup -b E_bothcy.blend -E BLENDER_WORKBENCH --python-expr <print>
SCENES: [('Comp', 'BLENDER_WORKBENCH'), ('Sub', 'CYCLES')]
$ blender --factory-startup -b E_bothcy.blend --python-expr <print> -E BLENDER_WORKBENCH
SCENES: [('Comp', 'CYCLES'), ('Sub', 'CYCLES')]
$ (after the -E BLENDER_WORKBENCH render above) blender --factory-startup -b E_bothcy.blend --python-expr <print>
SCENES: [('Comp', 'CYCLES'), ('Sub', 'CYCLES')]
----- E_bothcy.blend -S Comp -E BLENDER_WORKBENCH -> E_SComp_wb_0001.exr
00:00.334  render           | Start rendering: Sub, ViewLayer
00:00.334  render           | Engine: Cycles
00:00.584  render           | Fra: 1 | Mem: 1M | Sample 64/64
exit 0
----- E_bothcy.blend -S Sub -E BLENDER_WORKBENCH -> E_SSub_wb_0001.exr
00:00.326  render           | Start rendering: Sub, ViewLayer
00:00.326  render           | Engine: Workbench
exit 0
CMP E_SComp_wb_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=0 meandiff=0
CMP E_SComp_wb_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=28.2201 meandiff=0.409324
CMP E_SSub_wb_0001.exr vs eng_Awb_Bcy_direct_0001.exr  maxdiff=28.2201 meandiff=0.409324
CMP E_SSub_wb_0001.exr vs eng_Acy_Bwb_direct_0001.exr  maxdiff=0 meandiff=0
----- E_bothcy.blend -E NO_SUCH_ENGINE -> E_bad_0001.exr
Error: engine not found 'NO_SUCH_ENGINE'
exit 1
files named E_bad*: 0
----- smp_A1_B512.blend -o smp_cyclessamples16_ -f 1 -- --cycles-samples 16
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:02.254  render           | Fra: 1 | Mem: 1M | Sample 512/512
exit 0
CMP smp_cyclessamples16_0001.exr vs smp_A1_B512_0001.exr  maxdiff=0 meandiff=0
----- smp_A1_B512.blend --python-expr import bpy; bpy.data.scenes["Sub"].cycles.samples = 7 -> smp_py7_0001.exr
00:00.329  render           | Start rendering: Sub, ViewLayer
00:00.329  render           | Engine: Cycles
00:00.361  render           | Fra: 1 | Mem: 1M | Sample 7/7
exit 0
CMP smp_py7_0001.exr vs smp_A1_B512_direct_0001.exr  maxdiff=14.1479 meandiff=0.0517274
----- r7_A_ee_B_cy2.blend  -> r7_plain_0001.exr
00:00.441  render           | Start rendering: Sub, ViewLayer
00:00.441  render           | Engine: Cycles
00:00.458  render           | Fra: 1 | Mem: 1M | Sample 2/2
exit 0
----- r7_A_ee_B_cy2.blend -S Sub -> r7_direct_0001.exr
00:00.331  render           | Start rendering: Sub, ViewLayer
00:00.331  render           | Engine: Cycles
00:00.345  render           | Fra: 1 | Mem: 1M | Sample 2/2
exit 0
----- r7_A_ee_B_cy2.blend -S Comp -E CYCLES -> r7_SComp_Ecy_0001.exr
00:00.330  render           | Start rendering: Sub, ViewLayer
00:00.330  render           | Engine: Cycles
00:00.347  render           | Fra: 1 | Mem: 1M | Sample 2/2
exit 0
----- r7_A_ee_B_cy2.blend -S Comp -E BLENDER_WORKBENCH -> r7_SComp_Ewb_0001.exr
00:00.333  render           | Start rendering: Sub, ViewLayer
00:00.333  render           | Engine: Cycles
00:00.348  render           | Fra: 1 | Mem: 1M | Sample 2/2
exit 0
----- r7_A_ee_B_cy2.blend -S Comp --python-expr import bpy; bpy.data.scenes["Comp"].cycles.samples = 4096 -> r7_py4096_0001.exr
00:00.327  render           | Start rendering: Sub, ViewLayer
00:00.327  render           | Engine: Cycles
00:00.341  render           | Fra: 1 | Mem: 1M | Sample 2/2
exit 0
CMP r7_plain_0001.exr vs r7_direct_0001.exr  maxdiff=0 meandiff=0
CMP r7_SComp_Ecy_0001.exr vs r7_direct_0001.exr  maxdiff=0 meandiff=0
CMP r7_SComp_Ewb_0001.exr vs r7_direct_0001.exr  maxdiff=0 meandiff=0
CMP r7_py4096_0001.exr vs r7_direct_0001.exr  maxdiff=0 meandiff=0
```

(Some section headings the runner printed between groups, and the first three progress lines of the 7-sample render, are in the hand-test list.) **This answers the "not in the sources" points above about values from outside, for this Mac.**

- **`-E` did not reach the scene behind the node.** In every run that scene was rendered with its own stored engine, both by Blender's own engine line and by the pixels. What `-E` did was write the engine onto the active scene and that scene only, at the point on the command line where it appeared: read after it, Comp had changed and Sub had not; read before it, neither had. Nothing was saved back into the file. Naming Sub with `-S Sub` made `-E` apply to Sub — but that renders Sub itself, with no compositor.
- **An engine name that does not exist was refused**: `Error: engine not found 'NO_SUCH_ENGINE'`, nothing rendered, exit status 1.
- **A sampling argument offered anyway** — `-- --cycles-samples 16` — produced no message, exit status 0, a count of 512 and a picture identical to the plain run: neither taken nor refused.
- **A script given on the command line did reach the scene behind the node**: setting Sub's Cycles samples to 7 gave `Sample 7/7` and a different picture from the 512-sample reference.
- **The researcher's three commands**: on a file whose scene behind the node is on Cycles at 2 samples, `-S Comp -E CYCLES`, `-S Comp -E BLENDER_WORKBENCH`, and a script setting Comp's own samples to 4096 each still rendered Sub on Cycles at 2 samples, and each picture was identical to rendering Sub directly.

**Entry 41 — a render started inside a windowed Blender, and the Render Layers node's re-render button (in the window).** Blender 5.3.0 Beta was started from a terminal with its window open (no `-b`), `--factory-startup`, its own settings folders, `--enable-event-simulate` and `--log "render"`, on entry 30's `smp_A1_B512.blend` (Comp at 1 sample, Sub at 512); a script on Blender's own timer closed the splash with a simulated Escape. Nobody touched the Mac.

*A render started from the script inside the windowed Blender.* Word for word:

```
GUI_BACKGROUND_FLAG: False ; current scene: Comp
00:07.613  render           | Rendering frame 1
00:07.613  render           | Start rendering: Sub, ViewLayer
00:07.613  render           | Engine: Cycles
00:09.480  render           | Executing compositor
00:09.483  render           | Saved: '.../out/5.3/q9/win_render_.exr'
RENDER FROM THE SCRIPT IN THE WINDOWED BLENDER: {'FINISHED'}
CMP win_render_.exr vs smp_A1_B512_0001.exr  maxdiff=0.0
CMP win_render_.exr vs smp_A1_B512_direct_0001.exr  maxdiff=0.0
CMP win_render_.exr vs smp_A512_B1_direct_0001.exr  maxdiff=39.027889251708984
```

Blender was not in background mode, and the written image was the same picture to the last figure as the command-line render of the same file and as Sub rendered directly at 512, and unlike Sub at 1. A second run of the same script printed the same. The windowed log carried no sample lines, so the count is read from the picture.

*The node's own operators.* With the node editor showing Comp's group and the Render Layers node selected and active, Blender listed `NODE OPERATORS NAMING RENDER OR VIEW LAYER: ['read_viewlayers', 'render_changed']` and returned:

```
poll render_changed: True
render_changed EXEC: {'CANCELLED'}
poll read_viewlayers: True
read_viewlayers EXEC: {'FINISHED'}
render_changed INVOKE: {'CANCELLED'}
```

and no render line followed either call.

*A simulated click on the button beside the node's View Layer dropdown*, at its place in a photograph of the editor. In the first run with the click, the log then read, word for word, `Rendering frame 1`, `Start rendering: Sub, ViewLayer`, `Engine: Cycles`, and a photograph ten seconds later showed the status bar reading **Rendering…** with a bar at **50%**; by forty seconds the bar had gone. **So a press of the button starts a render of the scene the node names.** To see which engine it used, the click was repeated on two files whose scenes have different engines, with nothing rendered first: on `eng_Awb_Bcy` (Comp on Workbench, Sub on Cycles) the log read `Start rendering: Sub, ViewLayer` and `Engine: Cycles`; on `eng_Acy_Bwb` (Comp on Cycles, Sub on Workbench) it read `Start rendering: Sub, ViewLayer` and `Engine: Workbench`. **The button's render used the engine of the scene behind the node**, both ways round. **Its sample count was not read**: the windowed log prints no sample lines; the Render Result afterwards was present but held no picture (`RENDER RESULT after the click: present, has_data=True size=(0, 0)`, and saving it failed with `Error: Image 'Render Result' does not have any image data`); and a Viewer node added to Comp's group and fed by the Render Layers node still held its empty 256×256 image forty seconds after the click. What the button renders with, beyond the engine, stays open.

---

## Things a running Blender could settle

*(**Run, and pruned — 2026-10-18.** All nine items this list held were run on the Creator's Mac on 2026-10-18 and are written into the observed sections above, under the questions they answer; the answered items have been taken off this list. In the numbering of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`, this list's items were entries 7 (item 1), 11 (item 2), 12 (item 3), 15 (item 4), 16 (item 5), 7 (item 6), 29 to 32 (item 7), 10, 20 and 22 (item 8) and 3 (item 9).)*

What is still open, and why:

- **What the Render Layers node's re-render button renders with, beyond the engine.** A simulated click pressed it and the log named the engine (question 9, entry 41), but nothing a script could read held the result, so its sample count is not known.
- **What socket inspection shows in its fuller form.** The simulated pointer brought up only the one-value display on the Render Layers node's socket (question 5, entry 37); whether a real pointer held still, or the node's eye icon in its other state, gives the display listing a resolution and a transform was not caught.
- **Linux and Windows.** Nothing on this list was run on either; no machine of either kind was available to this sitting.
- **A graphics card of another make.** The GPU compositor's behaviour above 8k (question 6, entry 10) was seen on Apple's own graphics only.

---

2026-10-17 — Claude Opus 5.5 (Claude Code) — The compositor, Blender 5.3

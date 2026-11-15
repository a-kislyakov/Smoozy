Sources consulted: supplied Blender 5.3 manual, Python API documentation and release notes; date read: 2026-11-08 (the assigned snapshot date). Supplied Cycles files searched; date read: 2026-10-16 (their assigned snapshot date).
I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

# What a move works out — Blender 5.3

Research findings answering the "What a move works out" question sheet for Blender 5.3 from the supplied Blender documentation and Cycles files.

No Blender process was run. No helpers were used. The supplied brief was read before the question sheet; it is outside the project. Dates attached to documentary evidence below use the snapshot date required by that brief.

## 1. When a script moves a scene to a frame, which data does Blender work out, and what decides which data that is?

**Label: documented** for the API and manual descriptions quoted below; **inferred** for connecting the dependency-graph, driver-variable, and restriction-toggle descriptions to a particular frame move, linking the collection-exclusion UI and API descriptions, and applying the general API contract in background mode *(Note — citation check, 2026-11-13: narrowed the collection-exclusion inference summary to match the narrowed bullet below)*; **not in the sources** for a complete rule specifying every data-block evaluated in a background move or what assigning `scene.frame_current` alone updates or skips. **Observed** (hand tests, 2026-11-13, on macOS with no window): what assigning `frame_current`, `view_layer.update()`, `evaluated_depsgraph_get()`, `Depsgraph.update()` and `frame_set` worked out in the test files, which graphs, and which handlers ran — in the observed block at the end of this answer.

**Reading of the question.** “Moves a scene to a frame” is ambiguous. I take it to mean calling `scene.frame_set(frame, subframe=...)`. Assigning `scene.frame_current` is another reading and is answered separately below from its quoted description. “Works out” means evaluating animation and dependent data, rather than producing a rendered image.

The API describes `Scene.frame_set` as:

> Set scene frame updating all objects and view layers immediately

Its signature is:

> frame_set(frame, *, subframe=0.0)

The same page describes `frame_current` as:

> Current frame, to update animation data from Python frame_set() instead (in [-1048574, 1048574], default 1)

Source: Python API, [Scene — frame_set and frame_current](https://docs.blender.org/api/5.3/bpy.types.Scene.html). Date read: 2026-11-08.

This documents an immediate update of objects and View Layers for `frame_set`. It does not enumerate every scene setting, compositor property, simulation, or other data-block touched by that update. It also does not state the complete rules for dependencies, excluded objects, or previously unbuilt graphs during this call.

For the second reading (assigning `scene.frame_current`): the quoted `frame_current` description documents the instruction to use `frame_set()` instead to update animation data. The sources quoted here do not state what assigning `scene.frame_current` alone immediately updates, defers, or leaves unchanged (**not in the sources**).

The dependency-graph documentation explains the kinds of evaluated object state:

> This is needed every time one needs to access state with animation, constraints, and modifiers
> taken into account.

Its example says:

> # This object has animation and drivers applied on it, together with constraints and
> # modifiers.

It also says:

> # For mesh objects the object.data will be a mesh with all modifiers applied.

and:

> # For other types of objects the object_eval.data does not have modifiers applied on it,
> # but has animation applied.

Source: Python API, [Depsgraph — Evaluated ID example](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html). Date read: 2026-11-08.

**Inference:** these passages identify what evaluated object state can include after an update. Connecting them to `frame_set` means combining the promise that it updates objects with the description of evaluated object state. They are not an exhaustive specification of the work performed by `frame_set`.

A graph has a particular scene, View Layer and evaluation mode. The API describes `Depsgraph.scene` as:

> Original scene dependency graph is built for (readonly)

and `Depsgraph.view_layer` as:

> Original view layer dependency graph is built for (readonly)

For `mode`, it gives:

> Evaluation mode (default 'VIEWPORT', readonly)

> Viewport – Viewport non-rendered mode.

> Render – Render.

Source: Python API, [Depsgraph — scene, view_layer and mode](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html). Date read: 2026-11-08.

The context accessor is described as:

> Get the dependency graph for the current scene and view layer, to access to data-blocks with animation and modifiers applied. If any data-blocks have been edited, the dependency graph will be updated. This invalidates all references to evaluated data-blocks from the dependency graph.

Source: Python API, [Context — evaluated_depsgraph_get](https://docs.blender.org/api/5.3/bpy.types.Context.html). Date read: 2026-11-08.

**Inference:** scene, View Layer, evaluation mode and dependencies are relevant to interpreting evaluated data. The accessor's reference to the *current* scene and View Layer does not establish which graphs `frame_set` updates on a different scene. Nor does a graph's documented default mode prove the mode used for every background frame move.

What decides which data is worked out also involves View Layer membership and driver dependency tracking, though background-mode details are incomplete in the sources:

- **Disabling objects in viewports.** Under “Disable in Viewports”, the manual says: “Objects hidden this way are no longer part of the view layer,” followed by “so they no longer get evaluated and don’t affect playback performance.” Under “Hide in Viewports”, it instead says: “Objects hidden this way are still part of the view layer,” followed by “so they still get evaluated and affect playback performance.” Source: Manual, [Outliner Interface — Restriction Toggles](https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html), read 2026-11-08. The API exposes `Object.hide_viewport` and `Collection.hide_viewport`, both described as “Globally disable in viewports (default False)”. Sources: Python API, [Object](https://docs.blender.org/api/5.3/bpy.types.Object.html) and [Collection](https://docs.blender.org/api/5.3/bpy.types.Collection.html), read 2026-11-08. **Inference:** linking the API properties to “Disable in Viewports” and applying that viewport/playback exclusion rule to a background `Scene.frame_set` move are inferred steps rather than stated for background mode.
- **Excluding collections per View Layer.** `LayerCollection.exclude` is described as “Exclude from view layer (default False)”. Source: Python API, [LayerCollection](https://docs.blender.org/api/5.3/bpy.types.LayerCollection.html), read 2026-11-08. The manual says: “When disabled, all objects in the collection will be excluded from the render, the 3D Viewport, and the Outliner” and “for the current view layer.” Source: Manual, [Collections — Include](https://docs.blender.org/manual/en/5.3/scene_layout/collections/collections.html), read 2026-11-08. **Inference:** linking “Include” to `LayerCollection.exclude` is an inferred step. The quoted passages describe exclusion from the view layer, the render, the 3D Viewport, and the Outliner; they do not establish whether `Scene.frame_set` skips evaluating excluded objects or dependencies they feed. *(Note — citation check, 2026-11-13: narrowed this inference to what the quoted passages carry; they do not support inferring that excluded collections are skipped by `Scene.frame_set`.)*
- **Driver dependency tracking.** The manual says:

> Drivers should access object data via Driver Variables, rather than direct references in the Python expression,
> in order for dependencies to be correctly tracked.

Source: Manual, [Drivers Panel — Driver Variables](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html). Date read: 2026-11-08. **Inference:** this documents a requirement for dependency tracking on drivers; inferring from it which driver targets Blender selects or skips during `Scene.frame_set` is an inferred step.

**Search record.** Searched the entire supplied manual, API, release notes and Cycles files for `frame_set`, `frame_current`, `dependency graph`, `depsgraph`, `evaluated_depsgraph_get`, `scene` with `evaluation`/`update`, and `frame` with `evaluation`/`update`. Followed with `exclude`, `visibility`, `visible`, `hidden`, `disabled`, `driver` and `view layer` around evaluation terms. Opened the Scene, Depsgraph, Context, ID, ViewLayer, LayerCollection, RenderEngine and application-handler API pages; the API page [Internal Data & Their Python Objects](https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html); and the manual's [Outliner Interface](https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html), [View Layers Introduction](https://docs.blender.org/manual/en/5.3/render/layers/introduction.html), Drivers Panel and [Driver Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html). Opened the [Python API](https://developer.blender.org/docs/release_notes/5.3/python_api/), [Core](https://developer.blender.org/docs/release_notes/5.3/core/) and [Animation & Rigging](https://developer.blender.org/docs/release_notes/5.3/animation_rigging/) release notes after searching all release-note subjects.

Ruled out `RenderEngine.frame_set` as a general scene-move answer: its description is specifically “Evaluate scene at a different frame (for motion blur)”. Ruled out `ViewLayer.update` and the stale-data example as a complete frame-change specification: they describe updating tagged/dependent data after edits. Handler descriptions discuss playback/rendering and do not enumerate which data a background `Scene.frame_set` evaluates. Cycles search hits passed dependency graphs to rendering functions or exposed render settings; they supplied no complete frame-move evaluation rule.

**Not covered.** The exact data-block set, execution order, dependency closure, graph creation policy and amount of geometry/simulation work for a particular file. The quoted API sentence must not be expanded into a claim that every data-block in the file is evaluated.

**Outside the interface.** The API describes a Python call without stating an interface requirement. The manual describes `--background` as “Run in background (often used for UI-less rendering).” Source: Manual, [Command Line Arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html), read 2026-11-08. Applying the general API contract in background mode is an inference; these sources do not give a separate complete background evaluation rule or establish equality with interactive evaluation. **Observed (hand tests, 2026-11-13):** every result in the observed block below was got with Blender started from the terminal with no window (`-b --factory-startup`), so it holds when Blender is driven from outside its window, on the machine named there. Nothing was run with the window open, so whether the same happens inside the window was not tested.

**Operating system.** The cited frame-update and graph descriptions do not specify operating-system exceptions. The sources do not establish that the full behavior is identical on every supported operating system. **Observed (hand tests, 2026-11-13):** on macOS 26.6.2 on Apple silicon only (the Creator's Mac, an Apple M1 Pro). Linux and Windows were not tested: no such machine was available for that sitting, so whether these results hold there is still open.

**Observed — hand tests, 2026-11-13** (Claude Opus 5.5, in Claude Code). Every result in this block was got on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`) and with `--debug-depsgraph-eval`, which Blender's `--help` describes as "Enable debug messages from dependency graph related on evaluation." Each debug line starts with the graph it comes from, `[SC<scene> :: <View Layer>]`; `BKE_animsys_eval_animdata on SCA (…) at time 5.000000` means scene A's animation was worked out for frame 5 in that graph, `deg_create_eval_copy on SC…` that the graph was new, `BKE_object_eval_uber_data on OBHeavy` that it worked out the object Heavy's modifiers. A script given before the file printed a `HANDLER` line whenever a file loaded, a frame changed or a graph finished updating; the test scripts printed `STEP-BEGIN`/`STEP-END <seconds>` around each step, `R` lines with each scene's frame number and stored values, and `RE` lines with the values in each View Layer's graph, read with `evaluated_get` on `ViewLayer.depsgraph`, which does not ask for an update and gives `None` where a View Layer has no graph. The test files were written by a script with `bpy.data.libraries.write`, so every keyed value has a stored value of its own (42) that no frame gives. Quoted output leaves out the debug lines `Operation is entry point…`, `Accumulated recalc bits…`, Blender's start-up lines for its own factory scene, routine per-object lines (`eval_base_flags`, `node_visibility`, `light_linking`, `eval_shading`, `local_transform`, `transform_final`, `eval_strips`, `layer_eval_view_layer`) and `deg_create_eval_copy` lines for anything but a scene; the work folder is printed as `…/`. The same tests ran in 4.5, 5.0, 5.1 and 5.2; those results are recorded in the hand-test list only. Not tested: Blender with its window open; Linux; Windows.

*Assigning the frame number alone, then asking for an update.* File: three scenes `A` (View Layers `L1`, `L2`), `B` and `C` (`L1` each), each with an empty (`CubeA`, …) whose location X is keyed 1 at frame 1 and 10 at frame 10, and a custom property `p` keyed the same way, all stored at 42; the file opens on `A`. Opening it worked `A`'s `L1` graph out at frame 1 (the stored values read 1 afterwards). A script then set `A.frame_current = 5`, then called `bpy.context.view_layer.update()`, then `A.frame_set(5)`, reading after each step. A second run called `bpy.context.evaluated_depsgraph_get()` in place of `view_layer.update()`; a third did the same steps on `B` (`B.frame_current = 5`, `B.view_layers["L1"].update()`, `B.view_layers["L1"].depsgraph.update()`, `B.frame_set(5)`). What came back, first run:
```
HANDLER load_pre (None,)
00:00.343  blend            | Read blend: "…/files/5.3/abc.blend"
00:00.346  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCA (0xc45a32808)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xc45a32808) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xc487d5208) at time 1.000000
SCRIPT mode t1a version 5.3.0 file abc.blend
STEP-BEGIN A.frame_current = 5
STEP-END A.frame_current = 5 0.000
R after frame_current | scene A frame 5 | stored CubeA X 1 | stored p 1
R after frame_current | scene B frame 1 | stored CubeB X 42 | stored p 42
R after frame_current | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after frame_current | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 1 | CubeA X 1 | CubeA in graph objects True | p 1
RE after frame_current | scene A View Layer L2 | ViewLayer.depsgraph is None
STEP-BEGIN bpy.context.view_layer.update()
[SCA :: L1]: deg_create_eval_copy on SCA (0xc45a32808)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xc45a32808) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xc487d5208) at time 5.000000
HANDLER depsgraph_update_post scene A graph scene A view layer L1 mode VIEWPORT
STEP-END bpy.context.view_layer.update() 0.000
R after view_layer.update | scene A frame 5 | stored CubeA X 5 | stored p 5
R after view_layer.update | scene B frame 1 | stored CubeB X 42 | stored p 42
R after view_layer.update | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after view_layer.update | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after view_layer.update | scene A View Layer L2 | ViewLayer.depsgraph is None
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xc45a32808) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xc487d5208) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0xc45a34408)
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xc45a34408) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xc487d4c08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after frame_set | scene A frame 5 | stored CubeA X 5 | stored p 5
R after frame_set | scene B frame 1 | stored CubeB X 42 | stored p 42
R after frame_set | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after frame_set | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after frame_set | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
```
Second run — its `STEP`, `R` and `RE` lines, handler lines, animation lines (`animsys`) and new-graph lines, in order:
```
[SCScene :: ViewLayer]: deg_create_eval_copy on SCScene (0xb5377cc08)
[SCA :: L1]: deg_create_eval_copy on SCA (0xb5377cc08)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xb5377cc08) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xb55690008) at time 1.000000
STEP-BEGIN A.frame_current = 5
STEP-END A.frame_current = 5 0.000
R after frame_current | version 5.3.0 | context scene A | window scenes ['A']
R after frame_current | scene A frame 5 | stored CubeA X 1 | stored p 1
R after frame_current | scene B frame 1 | stored CubeB X 42 | stored p 42
R after frame_current | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN bpy.context.evaluated_depsgraph_get()
[SCA :: L1]: deg_create_eval_copy on SCA (0xb5377cc08)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xb5377cc08) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xb55690008) at time 5.000000
HANDLER depsgraph_update_post scene A graph scene A view layer L1 mode VIEWPORT
STEP-END bpy.context.evaluated_depsgraph_get() 0.000
R after evaluated_depsgraph_get | version 5.3.0 | context scene A | window scenes ['A']
R after evaluated_depsgraph_get | scene A frame 5 | stored CubeA X 5 | stored p 5
R after evaluated_depsgraph_get | scene B frame 1 | stored CubeB X 42 | stored p 42
R after evaluated_depsgraph_get | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after evaluated_depsgraph_get | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after evaluated_depsgraph_get | scene A View Layer L2 | ViewLayer.depsgraph is None
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xb5377cc08) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xb55690008) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0xb5377e808)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xb55690608) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xb5377e808) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after frame_set | version 5.3.0 | context scene A | window scenes ['A']
R after frame_set | scene A frame 5 | stored CubeA X 5 | stored p 5
R after frame_set | scene B frame 1 | stored CubeB X 42 | stored p 42
R after frame_set | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after frame_set | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after frame_set | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
```
Third run, on `B` — the same kinds of line:
```
[SCScene :: ViewLayer]: deg_create_eval_copy on SCScene (0x891ad2008)
[SCA :: L1]: deg_create_eval_copy on SCA (0x891ad2008)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x891ad2008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x892f47608) at time 1.000000
STEP-BEGIN B.frame_current = 5
STEP-END B.frame_current = 5 0.000
R after frame_current | version 5.3.0 | context scene A | window scenes ['A']
R after frame_current | scene A frame 1 | stored CubeA X 1 | stored p 1
R after frame_current | scene B frame 5 | stored CubeB X 42 | stored p 42
R after frame_current | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after frame_current | scene B View Layer L1 | ViewLayer.depsgraph is None
STEP-BEGIN B.view_layers["L1"].update()
[SCB :: L1]: deg_create_eval_copy on SCB (0x891ad3c08)
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x892f47008) at time 5.000000
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x891ad3c08) at time 5.000000
STEP-END B.view_layers["L1"].update() 0.000
R after B L1 update | version 5.3.0 | context scene A | window scenes ['A']
R after B L1 update | scene A frame 1 | stored CubeA X 1 | stored p 1
R after B L1 update | scene B frame 5 | stored CubeB X 5 | stored p 5
R after B L1 update | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after B L1 update | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeB X 5 | CubeB in graph objects True | p 5
STEP-BEGIN B.view_layers["L1"].depsgraph.update()
STEP-END B.view_layers["L1"].depsgraph.update() 0.000
R after B L1 depsgraph.update | version 5.3.0 | context scene A | window scenes ['A']
R after B L1 depsgraph.update | scene A frame 1 | stored CubeA X 1 | stored p 1
R after B L1 depsgraph.update | scene B frame 5 | stored CubeB X 5 | stored p 5
R after B L1 depsgraph.update | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after B L1 depsgraph.update | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeB X 5 | CubeB in graph objects True | p 5
STEP-BEGIN B.frame_set(5)
HANDLER frame_change_pre scene B frame 5 no graph
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x891ad3c08) at time 5.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x892f47008) at time 5.000000
HANDLER frame_change_post scene B frame 5 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(5) 0.000
R after frame_set | version 5.3.0 | context scene A | window scenes ['A']
R after frame_set | scene A frame 1 | stored CubeA X 1 | stored p 1
R after frame_set | scene B frame 5 | stored CubeB X 5 | stored p 5
R after frame_set | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after frame_set | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeB X 5 | CubeB in graph objects True | p 5
```

*What `frame_set` worked out, and the frame-change handlers.* Same file, opened plainly; `A.frame_set(5)`, then `B.frame_set(7)`, reading after each and reading every graph at the end:
```
HANDLER load_pre (None,)
00:00.337  blend            | Read blend: "…/files/5.3/abc.blend"
00:00.340  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCA (0x8f2df4008)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x8f2df4008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x8f244e408) at time 1.000000
SCRIPT mode moves version 5.3.0 file abc.blend
R as opened | scene A frame 1 | stored CubeA X 1 | stored p 1
R as opened | scene B frame 1 | stored CubeB X 42 | stored p 42
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x8f2df4008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x8f244e408) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0x8f2df5c08)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0x8f244de08) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0x8f2df5c08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 42 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0x8f2df7808)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x8f2df7808) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x8f244d808) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 42 | stored p 42
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```

*A View Layer with Use for Rendering off, and a heavy model.* File: three scenes `S1`, `S2`, `S3`, each one frame long (10, 30, 70), each stored at frame number 1, each holding the same collection with one heavy object (`Heavy`: a cube with Subdivision Surface at level 8, a Smooth modifier repeated 600 times and a Wave modifier, about 393,000 faces worked out), and each with two View Layers: `ViewLayer` (Use for Rendering on) and `Off` (Use for Rendering off). A script moved each scene to its frame with `S.frame_set(f)`, timing each move. Every `STEP` line and every line showing a graph working out `Heavy`'s modifiers, in order:
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x86ac1b008)
STEP-BEGIN move S1 to 10
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x86ac1b008)
[SCS1 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x8698f8008)
STEP-END move S1 to 10 5.488
STEP-BEGIN move S2 to 30
[SCS2 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8698faa08)
[SCS2 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x8698fb608)
STEP-END move S2 to 30 5.491
STEP-BEGIN move S3 to 70
[SCS3 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8696dd808)
[SCS3 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x8696dd208)
STEP-END move S3 to 70 5.492
```
and, at the end of the run, `Heavy` in each graph:
```
HEAVY S1 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 10
HEAVY S1 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 10
HEAVY S2 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 30
HEAVY S2 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 30
HEAVY S3 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 70
HEAVY S3 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 70
```

## 2. Can a script reduce what Blender works out on such a move? If so, by what means, and which values — the scene's own settings, its View Layers' settings, its compositor, and drivers on any of these — still come out the same as after a full move?

**Label: documented** for the controls, panel labels, and limits quoted below; **inferred** for connecting those controls to a reduced background `Scene.frame_set` move, for evaluating individual curves as a partial substitute, and for driver dependency and context risks; **not in the sources** for a method guaranteed to preserve all the requested values or for whether `Scene.frame_set` evaluates compositor effects. **Observed** (hand tests, 2026-11-13, on macOS with no window): sixteen ways of moving, or of making the move smaller, tried on one heavy test file of three scenes, each timed and each compared setting by setting with a full move — in the observed block at the end of this answer.

**Reading of the question.** I take a “full move” to mean `Scene.frame_set` on the original file with its original objects and View Layers enabled. The phrase could instead mean a render-mode evaluation. The sources distinguish viewport and render graph modes, so those meanings cannot silently be treated as equivalent. I take compositor “values” to include animated settings and driver results; composited pixels are addressed under the compositor controls below.

The documented `Scene.frame_set(frame, *, subframe=0.0)` signature has no argument selecting data-blocks, omitting geometry, or evaluating settings alone. Its promise remains “updating all objects and view layers immediately.” Source: Python API, [Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html), read 2026-11-08. This is a statement about the documented signature, not proof that no other technique exists.

There are controls that change evaluation or its inputs:

- **Disabling objects in viewports.** Under “Disable in Viewports”, the manual says: “Objects hidden this way are no longer part of the view layer,” followed by “so they no longer get evaluated and don’t affect playback performance.” Under “Hide in Viewports”, it instead says: “Objects hidden this way are still part of the view layer,” followed by “so they still get evaluated and affect playback performance.” Source: Manual, [Outliner Interface — Restriction Toggles](https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html), read 2026-11-08. The API exposes `Object.hide_viewport` and `Collection.hide_viewport`, both described as “Globally disable in viewports (default False)”. Sources: Python API, [Object](https://docs.blender.org/api/5.3/bpy.types.Object.html) and [Collection](https://docs.blender.org/api/5.3/bpy.types.Collection.html), read 2026-11-08. **Inference:** linking the API properties to “Disable in Viewports” is an inferred step across pages, a script can set these controls, and the manual establishes an evaluation reduction in its viewport/playback setting. Extending that reduction to a particular background frame move requires confirming its graph mode and behavior.
- **Excluding collections per View Layer.** `LayerCollection.exclude` is described as “Exclude from view layer (default False)”. Source: Python API, [LayerCollection](https://docs.blender.org/api/5.3/bpy.types.LayerCollection.html), read 2026-11-08. The manual says: “When disabled, all objects in the collection will be excluded from the render, the 3D Viewport, and the Outliner” and “for the current view layer.” Source: Manual, [Collections — Include](https://docs.blender.org/manual/en/5.3/scene_layout/collections/collections.html), read 2026-11-08. **Inference:** linking the manual’s “Include” toggle to `LayerCollection.exclude` and treating exclusion as a candidate for reducing `Scene.frame_set` evaluation are inferred steps. These words do not specify the dependencies that may still be required or guarantee the requested settings and drivers remain equal.
- **Simplification.** `RenderSettings.use_simplify` is described as “Enable simplification of scene for quicker preview renders (default False)”. Source: Python API, [RenderSettings](https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html), read 2026-11-08. The manual describes Max Subdivision as “Maximum number of subdivision by the Subdivision Surface modifiers.” and Child Particles as “Show only a subset of all child hairs and particles.” Source: Manual, [Cycles Render Settings — Simplify](https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/simplify.html), read 2026-11-08. These document preview-render simplification and limits on Subdivision Surface modifiers and child hairs and particles, not a settings-only frame move or preservation guarantee. **Inference:** whether enabling simplification reduces what `Scene.frame_set` works out in background mode is inferred rather than stated.
- **Evaluating an individual animation curve.** `FCurve.evaluate(frame)` is described as “Evaluate F-Curve”, returning “Value, Value of F-Curve specific frame (in [-inf, inf])”. Source: Python API, [FCurve](https://docs.blender.org/api/5.3/bpy.types.FCurve.html), read 2026-11-08. **Inference:** a script can ask for a curve value separately. Nothing in that description promises the scene, View Layers, compositor or their dependent drivers will be updated as by `frame_set`.

The manual's restriction-toggle image and View Layer panel image were also inspected. The latter shows “Use for Rendering” and “Render Single Layer”; it does not show a settings-only frame evaluation control (**Label: documented** for those visible controls in the View Layer panel image only). Sources: Manual images [Restriction Toggles](https://docs.blender.org/manual/en/5.3/_images/editors_outliner_interface_restriction-toggles.png) and [View Layer Panel](https://docs.blender.org/manual/en/5.3/_images/render_layers_view-layer_panel.png), read 2026-11-08. [PENDING: exact manual page addresses where each image sits, and what is visible in the Restriction Toggles image — sent back for research.]

Render switches do not establish a frame-move shortcut. `ViewLayer.use` says “Enable or disable rendering of this View Layer (default True)”. Source: Python API, [ViewLayer](https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html), read 2026-11-08. That does not say it suppresses `frame_set` updates. `RenderSettings.use_single_layer` says:

> Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)

Source: Python API, [RenderSettings](https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html). Date read: 2026-11-08.

The compositor has its own controls. The release notes state:

> **Effects Stack**: The compositor now uses an effects stack instead of a single compositor node group.

Source: Release notes, [Blender 5.3 — Compositor](https://developer.blender.org/docs/release_notes/5.3/compositor/). Date read: 2026-11-08.

The manual says:

> Each effect has independent controls for enabling or disabling its
> evaluation in viewport previews and final renders.

and:

> Disabling an effect passes the previous effect’s result to the next
> enabled effect.

Source: Manual, [Scene Effects](https://docs.blender.org/manual/en/5.3/compositing/scene_effects.html). Date read: 2026-11-08. The API exposes `enable_for_preview` (“Use effect during preview (default False)”) and `enable_for_render` (“Use effect during render (default False)”). Source: Python API, [SceneCompositorEffect](https://docs.blender.org/api/5.3/bpy.types.SceneCompositorEffect.html), read 2026-11-08. These are compositor execution controls for previews and renders. They do not state whether `Scene.frame_set` evaluates compositor effects at all, whether disabling effects reduces `frame_set` work, or whether animated compositor settings and drivers are updated by a reduced scene frame move (**not in the sources** for those frame-move effects).

**Which requested values remain equal?** No general equality guarantee was found for any of the four categories: scene settings, View Layer settings, compositor settings, or drivers on these. This does not establish that all such values change. It leaves their equivalence unresolved.

**Inference:** dependencies in drivers may involve the objects being omitted. The Driver Variables instruction quoted in answer 1 documents that drivers should access object data through Driver Variables so dependencies are tracked; inferring that omitting an object can affect drivers that depend on it is an inferred step rather than stated directly in that passage. Drivers can also depend on the evaluation context. The manual says:

> Scripted driver expressions have access to a built-in depsgraph variable containing the dependency graph
> that is currently evaluating the driver. This provides the correct scene and view layer when multiple contexts
> are evaluated at the same time, such as a render running while another view layer is displayed in a viewport.

> Avoid using bpy.context in driver expressions and custom driver functions. It represents the interface context
> and may not match the context evaluating the driver. Pass depsgraph to a custom function instead.

Source: Manual, [Driver Workflow & Examples — Dependency Graph Context](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html). Date read: 2026-11-08.

**Inference:** substituting a different View Layer cannot be assumed to preserve every driver result, because the documented driver interface exposes the evaluating View Layer (note that substituting or using an empty View Layer was not established above as a documented background `frame_set` reduction control). This does not establish how any particular driver behaves after collection exclusion or object disabling.

Merely retrieving an evaluated ID is not a substitute for performing evaluation. The API says:

> Get corresponding evaluated ID from the given dependency graph. Note that this does not ensure the dependency graph is fully evaluated, it just returns the result of the last evaluation.

Source: Python API, [ID — evaluated_get](https://docs.blender.org/api/5.3/bpy.types.ID.html). Date read: 2026-11-08.

**Search record.** Searched the entire supplied manual, API, release notes and Cycles files for the frame/dependency terms in answer 1, then `skip`/`disable` with `evaluation`/`update`/`depsgraph`, `only` with `evaluation`/`update`, `minimal`, `partial`, `selective`, `empty scene`, `empty view layer`, `exclude`, `hidden`, `visibility`, `simplify`, `mute`, `use_viewport`, `use_render`, `scene settings`, `driver` and `compositor`. Opened the API pages Scene, ViewLayer, LayerCollection, Object, Collection, Modifier, RenderSettings, FCurve, AnimData, ID, DriverTarget, DriverVariable, and SceneCompositorEffect; the manual pages Outliner Interface, Collections, View Layers Introduction, [View Layer](https://docs.blender.org/manual/en/5.3/render/layers/view_layer.html), Simplify, Drivers Panel, Driver Workflow & Examples, [Scripting & Security](https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html), [Compositing Introduction](https://docs.blender.org/manual/en/5.3/compositing/introduction.html), Scene Effects and [Compositor System](https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html); and the Compositor release notes. The two inspected images are linked above.

Ruled out render-only layer toggles as evidence of reduced frame-change evaluation; simple hiding is expressly still evaluated in the manual. Ruled out curve evaluation and muting as a promise of fully updated dependent values. Ruled out compositor preview/final-render controls, output reachability and socket inspection as proof of animation/driver equivalence after a frame move. The manual says “Socket values are only available after the node tree has been evaluated.” Source: Manual, [Compositor System](https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html), read 2026-11-08. It does not equate that evaluation with changing the scene frame. Cycles searches found render settings, simplification UI and graph hand-offs, not an implementation of general `Scene.frame_set` or a settings-equivalence guarantee. The file-load shortcut discussed in answer 3 concerns loading, not a documented selective frame-move API.

**Not covered.** A proven background settings-only shortcut; which dependencies remain evaluated under exclusions; exact preserved properties; arbitrary driver behavior; compositor pixels; timings or speedups. No listed candidate has been exercised here.

**Outside the interface.** Script-accessible controls are documented, but applying the viewport/playback evaluation reduction to a background move is inferred. `use_single_layer` is explicitly ignored for command-line rendering. Script-execution settings also matter: the manual says “Simple expressions are evaluated even when Python script execution is disabled.” Source: Manual, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), read 2026-11-08. [PENDING: exact quoted words, date read, and background default from Scripting & Security for `--enable-autoexec` and `--disable-autoexec` — sent back for research.] These facts do not supply the missing equality guarantee. **Observed (hand tests, 2026-11-13):** every result in the observed block below was got with Blender started from the terminal with no window (`-b --factory-startup`), so it holds when Blender is driven from outside its window, on the machine named there. Nothing was run with the window open, so whether the same happens inside the window was not tested.

**Operating system.** The cited controls do not document operating-system exceptions relevant to a reduced frame move. The sources do not establish identical behavior or preserved values on every supported operating system. **Observed (hand tests, 2026-11-13):** on macOS 26.6.2 on Apple silicon only (the Creator's Mac, an Apple M1 Pro). Linux and Windows were not tested: no such machine was available for that sitting, so whether these results hold there is still open.

**Observed — hand tests, 2026-11-13** (Claude Opus 5.5, in Claude Code). Same machine, Blender, start-up and conventions as the observed block under question 1: the Creator's Mac (Apple M1 Pro with its own 14-core graphics, Metal; 16 GB; macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), no window, `-b --factory-startup --debug-depsgraph-eval`. The same tests ran in 4.5, 5.0, 5.1 and 5.2 and are recorded in the hand-test list only. Not tested: Blender with its window open; Linux; Windows; a render's composited picture.

*The file.* `farm.blend`, written by a script with `bpy.data.libraries.write`: three scenes, `S1` (frames 10–10, Cycles), `S2` (30–30, Cycles), `S3` (70–70, Workbench), each stored at frame number 1, each holding the same two collections — `Heavy` (one object, a cube with Subdivision Surface level 8, Smooth repeated 600 times and Wave; about 393,000 faces worked out, about 2.7 seconds a working-out) and `Rig` (an empty `Par` keyed X = frame − 1; its child `Mover`, own X 2, with a Limit Location constraint at X 50, and a custom property `k` keyed k = frame; a camera). In every scene: `render.filter_size` keyed (= frame), stored 1.5; `cycles.samples` driven by the simple expression `frame * 2`, stored 7; View Layer `ViewLayer` (Use for Rendering on) with samples keyed (= frame), stored 5; View Layer `Off` (Use for Rendering off) with samples driven by `Mover`'s world X (Transform Channel), stored 6; custom property `ctx` driven by a Context Property variable (Active View Layer, `samples`), stored 42; custom property `py` driven by the scripted expression `len(depsgraph.view_layer.name)`, stored 42; a compositor (5.3: one effect `Comp` in the effects stack) of Render Layers → Bright/Contrast `Bright` → Blur `Muted` (muted) → output, plus a File Output node fed from `Bright`, with `Bright`'s brightness keyed (= frame, stored 0.5), its contrast driven by `Mover`'s `k` (Single Property), stored 0.25, and the effect's input `Amount` keyed (= frame), stored 0.75. `S2`'s `render.dither_intensity` is driven by `S1`'s (0.75), stored 1; `S3`'s `cycles.preview_samples` by `S1`'s `render.filter_size`, stored 33.

*What was done.* Nineteen runs, each a fresh opening of the file. Each did one thing before any move (or nothing), then moved `S1`, `S2`, `S3` in turn to their own frames, timing each move, and wrote down every setting of the scene just moved — every render, Cycles, EEVEE and Workbench setting, every View Layer setting and switch and each collection's exclude switch, every compositor node's settings, inputs and mute, the effect's switches and input, every custom property — as stored and as evaluated in each of the scene's graphs, then compared them, name by name, with the run `full`. The runs: `full` and `full_again` (`S.frame_set(f)`); `obj_hide` (`Heavy.hide_viewport = True` first); `coll_hide` (the collection's `hide_viewport = True`); `exclude_all` (the collection's `exclude = True` in every View Layer); `exclude_vl` and `exclude_off` (in `ViewLayer` only, in `Off` only); `simplify` (`render.use_simplify = True`, `render.simplify_subdivision = 0`); `mods_off` (`show_viewport = False` on Heavy's three modifiers); `comp_off` (`render.use_compositing = False`, and the effect's `enable_for_render` and `enable_for_preview` off); `single_layer` (`render.use_single_layer = True`) — each followed by `S.frame_set(f)`; `fc_update` (`S.frame_current = f`, then `S.view_layers["ViewLayer"].update()`); `fc_dgupdate` (`S.frame_current = f`, then that View Layer's `depsgraph.update()` where the graph exists); `fc_evalget` (`bpy.context.window.scene = S`, `S.frame_current = f`, `bpy.context.evaluated_depsgraph_get()`); `fcurve` (no move; each keyed curve read with `FCurve.evaluate(f)`); `nodeps` (`frame_set` with `--disable-depsgraph-on-file-load` on the command line); `rig_hide` (`Mover.hide_viewport = True` first — `Mover` is what the Transform Channel driver and the contrast driver read) and `rig_exclude` (the `Rig` collection's `exclude = True` in every View Layer), each followed by `S.frame_set(f)`; and `fc_update_again`, `fc_update` run a second time (its comparison printed the same lines as `fc_update`'s).

*Times* — each move by Python's clock, and the whole Blender run (`real`, from `/usr/bin/time -p`, including opening the file and all the reading and writing):

| run | S1 move (s) | S2 move (s) | S3 move (s) | all three (s) | whole Blender run, real (s) |
|---|---|---|---|---|---|
| full | 5.488 | 5.491 | 5.492 | 16.471 | 20.08 |
| full_again | 5.489 | 5.518 | 5.504 | 16.511 | 20.12 |
| obj_hide | 0.003 | 0.002 | 0.002 | 0.007 | 3.68 |
| coll_hide | 0.002 | 0.002 | 0.003 | 0.006 | 3.67 |
| exclude_all | 0.004 | 0.002 | 0.002 | 0.009 | 3.70 |
| exclude_vl | 2.759 | 2.773 | 2.755 | 8.287 | 11.92 |
| exclude_off | 2.844 | 2.787 | 2.855 | 8.486 | 12.18 |
| simplify | 0.002 | 0.003 | 0.003 | 0.008 | 4.03 |
| mods_off | 0.002 | 0.002 | 0.002 | 0.006 | 3.70 |
| comp_off | 5.603 | 5.659 | 6.641 | 17.904 | 21.65 |
| single_layer | 6.177 | 6.507 | 6.607 | 19.291 | 23.31 |
| fc_update | 2.780 | 2.858 | 2.759 | 8.397 | 12.59 |
| fc_dgupdate | 2.757 | 0.000 | 0.000 | 2.757 | 6.49 |
| fc_evalget | 2.845 | 5.744 | 5.593 | 14.182 | 18.21 |
| fcurve | 0.000 | 0.000 | 0.000 | 0.000 | 3.87 |
| nodeps | 5.530 | 5.714 | 5.629 | 16.873 | 17.98 |
| rig_hide | 5.512 | 5.517 | 6.142 | 17.171 | 21.39 |
| rig_exclude | 6.246 | 6.089 | 6.906 | 19.242 | 23.57 |
| fc_update_again | 2.964 | 2.858 | 2.829 | 8.651 | 12.60 |
| full_enable | 5.536 | 5.601 | 5.651 | 16.788 | 20.65 |
| full_disable | 5.632 | 5.596 | 5.639 | 16.867 | 20.74 |

*Which graphs worked out the heavy model* — every `STEP` line and every `BKE_object_eval_uber_data on OBHeavy` line, in order. `full`:
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x86ac1b008)
STEP-BEGIN move S1 to 10
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x86ac1b008)
[SCS1 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x8698f8008)
STEP-END move S1 to 10 5.488
STEP-BEGIN move S2 to 30
[SCS2 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8698faa08)
[SCS2 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x8698fb608)
STEP-END move S2 to 30 5.491
STEP-BEGIN move S3 to 70
[SCS3 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8696dd808)
[SCS3 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x8696dd208)
STEP-END move S3 to 70 5.492
```
`obj_hide` (`coll_hide` and `exclude_all` printed the same pattern):
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8a6feb608)
STEP-BEGIN move S1 to 10
STEP-END move S1 to 10 0.003
STEP-BEGIN move S2 to 30
STEP-END move S2 to 30 0.002
STEP-BEGIN move S3 to 70
STEP-END move S3 to 70 0.002
```
`exclude_vl`:
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x76ac55e08)
STEP-BEGIN move S1 to 10
[SCS1 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x76ac55e08)
STEP-END move S1 to 10 2.759
STEP-BEGIN move S2 to 30
[SCS2 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x769fa0608)
STEP-END move S2 to 30 2.773
STEP-BEGIN move S3 to 70
[SCS3 :: Off]: BKE_object_eval_uber_data on OBHeavy (0x769fda408)
STEP-END move S3 to 70 2.755
```
`simplify` (`mods_off` printed the same pattern):
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0xb2d92a408)
STEP-BEGIN move S1 to 10
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0xb2d92a408)
[SCS1 :: Off]: BKE_object_eval_uber_data on OBHeavy (0xb2d92b008)
STEP-END move S1 to 10 0.002
STEP-BEGIN move S2 to 30
[SCS2 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0xb2ea84008)
[SCS2 :: Off]: BKE_object_eval_uber_data on OBHeavy (0xb2ea85808)
STEP-END move S2 to 30 0.003
STEP-BEGIN move S3 to 70
[SCS3 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0xb2ea87008)
[SCS3 :: Off]: BKE_object_eval_uber_data on OBHeavy (0xb2eaa4c08)
STEP-END move S3 to 70 0.003
```
`fc_update`:
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8810ad208)
STEP-BEGIN move S1 to 10
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8810ad208)
STEP-END move S1 to 10 2.780
STEP-BEGIN move S2 to 30
[SCS2 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8810ade08)
STEP-END move S2 to 30 2.858
STEP-BEGIN move S3 to 70
[SCS3 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x8810af608)
STEP-END move S3 to 70 2.759
```
`fc_evalget`, with its handler lines:
```
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x93677f608)
STEP-BEGIN move S1 to 10
[SCS1 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x93677f608)
HANDLER depsgraph_update_post scene S1 graph scene S1 view layer ViewLayer mode VIEWPORT
STEP-END move S1 to 10 2.845
STEP-BEGIN move S2 to 30
HANDLER frame_change_pre scene S2 frame 1 no graph
[SCS2 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x938992a08)
HANDLER frame_change_post scene S2 frame 1 graph scene S2 view layer ViewLayer mode VIEWPORT
[SCS2 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x938992a08)
HANDLER depsgraph_update_post scene S2 graph scene S2 view layer ViewLayer mode VIEWPORT
STEP-END move S2 to 30 5.744
STEP-BEGIN move S3 to 70
HANDLER frame_change_pre scene S3 frame 1 no graph
[SCS3 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x93839f608)
HANDLER frame_change_post scene S3 frame 1 graph scene S3 view layer ViewLayer mode VIEWPORT
[SCS3 :: ViewLayer]: BKE_object_eval_uber_data on OBHeavy (0x93839f608)
HANDLER depsgraph_update_post scene S3 graph scene S3 view layer ViewLayer mode VIEWPORT
STEP-END move S3 to 70 5.593
```
`Heavy` in each graph at the end, `simplify` and `exclude_vl`:
```
HEAVY S1 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 8 | graph scene frame 10
HEAVY S1 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 8 | graph scene frame 10
HEAVY S2 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 8 | graph scene frame 30
HEAVY S2 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 8 | graph scene frame 30
HEAVY S3 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 8 | graph scene frame 70
HEAVY S3 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 8 | graph scene frame 70
```
```
HEAVY S1 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects False | worked-out vertices 8 | graph scene frame 10
HEAVY S1 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 10
HEAVY S2 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects False | worked-out vertices 8 | graph scene frame 30
HEAVY S2 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 30
HEAVY S3 ViewLayer | graph mode VIEWPORT | Heavy among the graph's objects False | worked-out vertices 8 | graph scene frame 70
HEAVY S3 Off | graph mode VIEWPORT | Heavy among the graph's objects True | worked-out vertices 393218 | graph scene frame 70
```

*Which settings came out different from `full`* — the comparison's output for the readings straight after each scene's own move (the readings after all three moves gave the same names and values). `full_again`, `obj_hide`, `coll_hide` and `mods_off` each printed one line, `COMPARED … 0 differences`; for example:
```
COMPARED obj_hide with full: 14784 names in full, 14784 in obj_hide, 0 differences
```
`exclude_all`, `exclude_vl`, `exclude_off`, `simplify`, `comp_off` and `single_layer` printed only lines naming the switches the run had set itself; for example `simplify`:
```
DIFF | own stored S1 render.use_simplify | full: False | simplify: True
DIFF | own stored S1 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own evaluated[ViewLayer] S1 render.use_simplify | full: False | simplify: True
DIFF | own evaluated[ViewLayer] S1 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own evaluated[Off] S1 render.use_simplify | full: False | simplify: True
DIFF | own evaluated[Off] S1 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own stored S2 render.use_simplify | full: False | simplify: True
DIFF | own stored S2 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own evaluated[ViewLayer] S2 render.use_simplify | full: False | simplify: True
DIFF | own evaluated[ViewLayer] S2 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own evaluated[Off] S2 render.use_simplify | full: False | simplify: True
DIFF | own evaluated[Off] S2 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own stored S3 render.use_simplify | full: False | simplify: True
DIFF | own stored S3 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own evaluated[ViewLayer] S3 render.use_simplify | full: False | simplify: True
DIFF | own evaluated[ViewLayer] S3 render.simplify_subdivision | full: 6 | simplify: 0
DIFF | own evaluated[Off] S3 render.use_simplify | full: False | simplify: True
DIFF | own evaluated[Off] S3 render.simplify_subdivision | full: 6 | simplify: 0
COMPARED simplify with full: 14784 names in full, 14784 in simplify, 36 differences
```
`comp_off`:
```
DIFF | own stored S1 render.use_compositing | full: True | comp_off: False
DIFF | own stored S1 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own stored S1 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S1 render.use_compositing | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S1 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S1 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own evaluated[Off] S1 render.use_compositing | full: True | comp_off: False
DIFF | own evaluated[Off] S1 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own evaluated[Off] S1 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own stored S2 render.use_compositing | full: True | comp_off: False
DIFF | own stored S2 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own stored S2 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S2 render.use_compositing | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S2 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S2 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own evaluated[Off] S2 render.use_compositing | full: True | comp_off: False
DIFF | own evaluated[Off] S2 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own evaluated[Off] S2 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own stored S3 render.use_compositing | full: True | comp_off: False
DIFF | own stored S3 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own stored S3 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S3 render.use_compositing | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S3 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own evaluated[ViewLayer] S3 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
DIFF | own evaluated[Off] S3 render.use_compositing | full: True | comp_off: False
DIFF | own evaluated[Off] S3 compositor_effects['Comp'].enable_for_render | full: True | comp_off: False
DIFF | own evaluated[Off] S3 compositor_effects['Comp'].enable_for_preview | full: True | comp_off: False
COMPARED comp_off with full: 14784 names in full, 14784 in comp_off, 54 differences
```
`fc_update` (`fc_evalget` printed the same `DIFF` lines; its `ONLY-IN` lines name the `Off` graphs, which it left without an evaluated scene):
```
DIFF | own stored S2 render.dither_intensity | full: 1 | fc_update: 0.75
DIFF | own stored S2 render.filter_size | full: 1.5 | fc_update: 30
DIFF | own stored S2 cycles.samples | full: 7 | fc_update: 60
DIFF | own stored S2 scene['ctx'] | full: 42 | fc_update: 30
DIFF | own stored S2 scene['py'] | full: 42 | fc_update: 0
DIFF | own stored S2 view_layers['ViewLayer'].samples | full: 5 | fc_update: 30
DIFF | own stored S2 view_layers['Off'].samples | full: 6 | fc_update: 31
DIFF | own stored S2 compositor_effects['Comp'].inputs.Socket_1(Amount).value | full: 0.75 | fc_update: 30
DIFF | own stored S2 compositor(effect Comp).nodes['Bright'].inputs[1](Brightness).default_value | full: 0.5 | fc_update: 30
DIFF | own stored S2 compositor(effect Comp).nodes['Bright'].inputs[2](Contrast).default_value | full: 0.25 | fc_update: 30
DIFF | own stored S3 render.filter_size | full: 1.5 | fc_update: 70
DIFF | own stored S3 cycles.samples | full: 7 | fc_update: 140
DIFF | own stored S3 cycles.preview_samples | full: 33 | fc_update: 10
DIFF | own stored S3 scene['ctx'] | full: 42 | fc_update: 70
DIFF | own stored S3 scene['py'] | full: 42 | fc_update: 0
DIFF | own stored S3 view_layers['ViewLayer'].samples | full: 5 | fc_update: 70
DIFF | own stored S3 view_layers['Off'].samples | full: 6 | fc_update: 50
DIFF | own stored S3 compositor_effects['Comp'].inputs.Socket_1(Amount).value | full: 0.75 | fc_update: 70
DIFF | own stored S3 compositor(effect Comp).nodes['Bright'].inputs[1](Brightness).default_value | full: 0.5 | fc_update: 70
DIFF | own stored S3 compositor(effect Comp).nodes['Bright'].inputs[2](Contrast).default_value | full: 0.25 | fc_update: 70
ONLY-IN-full | own evaluated[Off] S1 | 854 names
ONLY-IN-full | own evaluated[Off] S2 | 855 names
ONLY-IN-full | own evaluated[Off] S3 | 855 names
ONLY-IN-fc_update | own evaluated[Off] S1 | 1 names (own evaluated[Off] S1 (no graph))
ONLY-IN-fc_update | own evaluated[Off] S2 | 1 names (own evaluated[Off] S2 (no graph))
ONLY-IN-fc_update | own evaluated[Off] S3 | 1 names (own evaluated[Off] S3 (no graph))
COMPARED fc_update with full: 14784 names in full, 9890 in fc_update, 5174 differences
```
`fc_dgupdate`:
```
ONLY-IN-full | own evaluated[Off] S1 | 854 names
ONLY-IN-full | own evaluated[ViewLayer] S2 | 855 names
ONLY-IN-full | own evaluated[Off] S2 | 855 names
ONLY-IN-full | own evaluated[ViewLayer] S3 | 855 names
ONLY-IN-full | own evaluated[Off] S3 | 855 names
ONLY-IN-fc_dgupdate | own evaluated[Off] S1 | 1 names (own evaluated[Off] S1 (no graph))
ONLY-IN-fc_dgupdate | own evaluated[ViewLayer] S2 | 1 names (own evaluated[ViewLayer] S2 (no graph))
ONLY-IN-fc_dgupdate | own evaluated[Off] S2 | 1 names (own evaluated[Off] S2 (no graph))
ONLY-IN-fc_dgupdate | own evaluated[ViewLayer] S3 | 1 names (own evaluated[ViewLayer] S3 (no graph))
ONLY-IN-fc_dgupdate | own evaluated[Off] S3 | 1 names (own evaluated[Off] S3 (no graph))
COMPARED fc_dgupdate with full: 14784 names in full, 6626 in fc_dgupdate, 8558 differences
```
`nodeps` (`session_uid` is a number Blender gives each data-block for the length of one run):
```
DIFF | own stored S1 scene.session_uid | full: 135 | nodeps: 134
DIFF | own stored S2 scene.session_uid | full: 137 | nodeps: 136
DIFF | own stored S3 scene.session_uid | full: 139 | nodeps: 138
COMPARED nodeps with full: 14784 names in full, 14784 in nodeps, 6 differences
```
`rig_hide`:
```
COMPARED rig_hide with full: 14784 names in full, 14784 in rig_hide, 0 differences
```
`rig_exclude`:
```
DIFF | own stored S1 view_layers['ViewLayer'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own stored S1 view_layers['Off'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own evaluated[ViewLayer] S1 view_layers['ViewLayer'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own evaluated[Off] S1 view_layers['Off'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own stored S2 view_layers['ViewLayer'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own stored S2 view_layers['Off'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own evaluated[ViewLayer] S2 view_layers['ViewLayer'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own evaluated[Off] S2 view_layers['Off'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own stored S3 view_layers['ViewLayer'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own stored S3 view_layers['Off'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own evaluated[ViewLayer] S3 view_layers['ViewLayer'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
DIFF | own evaluated[Off] S3 view_layers['Off'].layer_collection['Rig'].exclude | full: False | rig_exclude: True
COMPARED rig_exclude with full: 14784 names in full, 14784 in rig_exclude, 24 differences
```
and, from `rig_exclude`, every `STEP` line and every line showing a graph working out `Par`'s or `Mover`'s animation or `Mover`'s constraint (the `Rig` collection excluded in every View Layer):
```
[SCS1 :: ViewLayer]: BKE_animsys_eval_animdata on OBPar (0x8a15d8008) at time 1.000000
[SCS1 :: ViewLayer]: BKE_object_eval_constraints on OBMover (0x8a15d8608)
[SCS1 :: ViewLayer]: BKE_animsys_eval_animdata on OBMover (0x8a15d8608) at time 1.000000
STEP-BEGIN move S1 to 10
[SCS1 :: ViewLayer]: BKE_animsys_eval_animdata on OBPar (0x8a15d8008) at time 10.000000
[SCS1 :: ViewLayer]: BKE_animsys_eval_animdata on OBMover (0x8a15d8608) at time 10.000000
[SCS1 :: ViewLayer]: BKE_object_eval_constraints on OBMover (0x8a15d8608)
[SCS1 :: Off]: BKE_animsys_eval_animdata on OBMover (0x8a15d1e08) at time 10.000000
[SCS1 :: Off]: BKE_animsys_eval_animdata on OBPar (0x8a15d2408) at time 10.000000
[SCS1 :: Off]: BKE_object_eval_constraints on OBMover (0x8a15d1e08)
STEP-END move S1 to 10 6.246
STEP-BEGIN move S2 to 30
[SCS2 :: ViewLayer]: BKE_animsys_eval_animdata on OBMover (0x8a15cb608) at time 30.000000
[SCS2 :: ViewLayer]: BKE_animsys_eval_animdata on OBPar (0x8a15c0008) at time 30.000000
[SCS2 :: ViewLayer]: BKE_object_eval_constraints on OBMover (0x8a15cb608)
[SCS2 :: Off]: BKE_animsys_eval_animdata on OBPar (0x8a15b9808) at time 30.000000
[SCS2 :: Off]: BKE_animsys_eval_animdata on OBMover (0x8a15b9208) at time 30.000000
[SCS2 :: Off]: BKE_object_eval_constraints on OBMover (0x8a15b9208)
STEP-END move S2 to 30 6.089
STEP-BEGIN move S3 to 70
[SCS3 :: ViewLayer]: BKE_animsys_eval_animdata on OBMover (0x8a15aea08) at time 70.000000
[SCS3 :: ViewLayer]: BKE_animsys_eval_animdata on OBPar (0x8a15af008) at time 70.000000
[SCS3 :: ViewLayer]: BKE_object_eval_constraints on OBMover (0x8a15aea08)
[SCS3 :: Off]: BKE_animsys_eval_animdata on OBMover (0x8a159c608) at time 70.000000
[SCS3 :: Off]: BKE_animsys_eval_animdata on OBPar (0x8a159cc08) at time 70.000000
[SCS3 :: Off]: BKE_object_eval_constraints on OBMover (0x8a159c608)
STEP-END move S3 to 70 6.906
```

*The keyed and driven values after `full`*, stored and in each graph, straight after each scene's own move:
```
WATCH after own move | S1 | render.filter_size | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | render.dither_intensity | stored 0.75 | evaluated (per View Layer graph, no update asked) ViewLayer=0.75 Off=0.75
WATCH after own move | S1 | cycles.samples | stored 20 | evaluated (per View Layer graph, no update asked) ViewLayer=20 Off=20
WATCH after own move | S1 | cycles.preview_samples | stored 1024 | evaluated (per View Layer graph, no update asked) ViewLayer=1024 Off=1024
WATCH after own move | S1 | view_layers['ViewLayer'].samples | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | view_layers['Off'].samples | stored 11 | evaluated (per View Layer graph, no update asked) ViewLayer=11 Off=11
WATCH after own move | S1 | scene['ctx'] | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=11
WATCH after own move | S1 | scene['py'] | stored 0 | evaluated (per View Layer graph, no update asked) ViewLayer=0 Off=0
WATCH after own move | S1 | effect Comp Amount | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | Bright.bright | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | Bright.contrast | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S2 | render.filter_size | stored 1.5 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | render.dither_intensity | stored 1 | evaluated (per View Layer graph, no update asked) ViewLayer=0.75 Off=0.75
WATCH after own move | S2 | cycles.samples | stored 7 | evaluated (per View Layer graph, no update asked) ViewLayer=60 Off=60
WATCH after own move | S2 | cycles.preview_samples | stored 1024 | evaluated (per View Layer graph, no update asked) ViewLayer=1024 Off=1024
WATCH after own move | S2 | view_layers['ViewLayer'].samples | stored 5 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | view_layers['Off'].samples | stored 6 | evaluated (per View Layer graph, no update asked) ViewLayer=31 Off=31
WATCH after own move | S2 | scene['ctx'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=31
WATCH after own move | S2 | scene['py'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=0 Off=0
WATCH after own move | S2 | effect Comp Amount | stored 0.75 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | Bright.bright | stored 0.5 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | Bright.contrast | stored 0.25 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S3 | render.filter_size | stored 1.5 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | render.dither_intensity | stored 1 | evaluated (per View Layer graph, no update asked) ViewLayer=1 Off=1
WATCH after own move | S3 | cycles.samples | stored 7 | evaluated (per View Layer graph, no update asked) ViewLayer=140 Off=140
WATCH after own move | S3 | cycles.preview_samples | stored 33 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S3 | view_layers['ViewLayer'].samples | stored 5 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | view_layers['Off'].samples | stored 6 | evaluated (per View Layer graph, no update asked) ViewLayer=50 Off=50
WATCH after own move | S3 | scene['ctx'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=50
WATCH after own move | S3 | scene['py'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=0 Off=0
WATCH after own move | S3 | effect Comp Amount | stored 0.75 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | Bright.bright | stored 0.5 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | Bright.contrast | stored 0.25 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
```

*`FCurve.evaluate` against the full move.* The `fcurve` run's readings, then `full`'s readings of the same curves' settings after the move:
```
FCURVE S1 | scene S1 render.filter_size[0] | evaluate(10) = 10
FCURVE S1 | scene S1 view_layers["ViewLayer"].samples[0] | evaluate(10) = 10
FCURVE S1 | scene S1 compositor_effects["Comp"].properties.inputs.Socket_1.value[0] | evaluate(10) = 10
FCURVE S1 | compositor S1 effect Comp nodes["Bright"].inputs[1].default_value[0] | evaluate(10) = 10
FCURVE S1 | object Par location[0] | evaluate(10) = 9
FCURVE S1 | object Mover ["k"][0] | evaluate(10) = 10
FCURVE S2 | scene S2 render.filter_size[0] | evaluate(30) = 30
FCURVE S2 | scene S2 view_layers["ViewLayer"].samples[0] | evaluate(30) = 30
FCURVE S2 | scene S2 compositor_effects["Comp"].properties.inputs.Socket_1.value[0] | evaluate(30) = 30
FCURVE S2 | compositor S2 effect Comp nodes["Bright"].inputs[1].default_value[0] | evaluate(30) = 30
FCURVE S2 | object Par location[0] | evaluate(30) = 29
FCURVE S2 | object Mover ["k"][0] | evaluate(30) = 30
FCURVE S3 | scene S3 render.filter_size[0] | evaluate(70) = 70
FCURVE S3 | scene S3 view_layers["ViewLayer"].samples[0] | evaluate(70) = 70
FCURVE S3 | scene S3 compositor_effects["Comp"].properties.inputs.Socket_1.value[0] | evaluate(70) = 70
FCURVE S3 | compositor S3 effect Comp nodes["Bright"].inputs[1].default_value[0] | evaluate(70) = 70
FCURVE S3 | object Par location[0] | evaluate(70) = 69
FCURVE S3 | object Mover ["k"][0] | evaluate(70) = 70
```
```
FCPATH S1 | scene S1 render.filter_size[0] | after the move: stored 10 | evaluated in ViewLayer graph 10
FCPATH S1 | scene S1 view_layers["ViewLayer"].samples[0] | after the move: stored 10 | evaluated in ViewLayer graph 10
FCPATH S1 | scene S1 compositor_effects["Comp"].properties.inputs.Socket_1.value[0] | after the move: stored 10 | evaluated in ViewLayer graph 10
FCPATH S1 | compositor S1 effect Comp nodes["Bright"].inputs[1].default_value[0] | after the move: stored 10 | evaluated in ViewLayer graph 10
FCPATH S1 | object Par location[0] | after the move: stored 9 | evaluated in ViewLayer graph 9
FCPATH S1 | object Mover ["k"][0] | after the move: stored 10 | evaluated in ViewLayer graph 10
FCPATH S2 | scene S2 render.filter_size[0] | after the move: stored 1.5 | evaluated in ViewLayer graph 30
FCPATH S2 | scene S2 view_layers["ViewLayer"].samples[0] | after the move: stored 5 | evaluated in ViewLayer graph 30
FCPATH S2 | scene S2 compositor_effects["Comp"].properties.inputs.Socket_1.value[0] | after the move: stored 0.75 | evaluated in ViewLayer graph 30
FCPATH S2 | compositor S2 effect Comp nodes["Bright"].inputs[1].default_value[0] | after the move: stored 0.5 | evaluated in ViewLayer graph 30
FCPATH S2 | object Par location[0] | after the move: stored 9 | evaluated in ViewLayer graph 29
FCPATH S2 | object Mover ["k"][0] | after the move: stored 10 | evaluated in ViewLayer graph 30
FCPATH S3 | scene S3 render.filter_size[0] | after the move: stored 1.5 | evaluated in ViewLayer graph 70
FCPATH S3 | scene S3 view_layers["ViewLayer"].samples[0] | after the move: stored 5 | evaluated in ViewLayer graph 70
FCPATH S3 | scene S3 compositor_effects["Comp"].properties.inputs.Socket_1.value[0] | after the move: stored 0.75 | evaluated in ViewLayer graph 70
FCPATH S3 | compositor S3 effect Comp nodes["Bright"].inputs[1].default_value[0] | after the move: stored 0.5 | evaluated in ViewLayer graph 70
FCPATH S3 | object Par location[0] | after the move: stored 9 | evaluated in ViewLayer graph 69
FCPATH S3 | object Mover ["k"][0] | after the move: stored 10 | evaluated in ViewLayer graph 70
```

*The two drivers that read the evaluating View Layer.* While opening the file, `full` printed (and printed the cycle warning again before each scene's first move):
```
00:00.351  depsgraph        | WARNING Dependency cycle detected:
                            |   SCS1/PARAMETERS_EVAL() depends on
                            |   SCS1/ID_PROPERTY(ctx) via 'Driven Property -> Properties'
                            |   SCS1/DRIVER(["ctx"]) via 'Driver -> Driven Property'
                            |   SCS1/PARAMETERS_EVAL() via 'RNA Target -> Driver'
                            | 
	BPY_driver_exec: restricted access disallows name 'view_layer', enable auto-execution to support
Error in PyDriver: expression failed: len(depsgraph.view_layer.name)
For target: (type=Scene, name="S1", property=py, property_index=-1)
00:05.842  depsgraph        | WARNING Dependency cycle detected:
                            |   SCS1/PARAMETERS_EVAL() depends on
                            |   SCS1/ID_PROPERTY(ctx) via 'Driven Property -> Properties'
                            |   SCS1/DRIVER(["ctx"]) via 'Driver -> Driven Property'
                            |   SCS1/PARAMETERS_EVAL() via 'RNA Target -> Driver'
                            | 
00:08.592  depsgraph        | WARNING Dependency cycle detected:
                            |   SCS2/PARAMETERS_EVAL() depends on
                            |   SCS2/ID_PROPERTY(ctx) via 'Driven Property -> Properties'
                            |   SCS2/DRIVER(["ctx"]) via 'Driver -> Driven Property'
                            |   SCS2/PARAMETERS_EVAL() via 'RNA Target -> Driver'
                            | 
	BPY_driver_exec: restricted access disallows name 'view_layer', enable auto-execution to support
Error in PyDriver: expression failed: len(depsgraph.view_layer.name)
For target: (type=Scene, name="S2", property=py, property_index=-1)
00:11.332  depsgraph        | WARNING Dependency cycle detected:
                            |   SCS2/PARAMETERS_EVAL() depends on
                            |   SCS2/ID_PROPERTY(ctx) via 'Driven Property -> Properties'
                            |   SCS2/DRIVER(["ctx"]) via 'Driver -> Driven Property'
                            |   SCS2/PARAMETERS_EVAL() via 'RNA Target -> Driver'
                            | 
	BPY_driver_exec: restricted access disallows name 'view_layer', enable auto-execution to support
Error in PyDriver: expression failed: len(depsgraph.view_layer.name)
For target: (type=Scene, name="S2", property=py, property_index=-1)
00:14.088  depsgraph        | WARNING Dependency cycle detected:
                            |   SCS3/PARAMETERS_EVAL() depends on
                            |   SCS3/ID_PROPERTY(ctx) via 'Driven Property -> Properties'
                            |   SCS3/DRIVER(["ctx"]) via 'Driver -> Driven Property'
                            |   SCS3/PARAMETERS_EVAL() via 'RNA Target -> Driver'
                            | 
	BPY_driver_exec: restricted access disallows name 'view_layer', enable auto-execution to support
Error in PyDriver: expression failed: len(depsgraph.view_layer.name)
For target: (type=Scene, name="S3", property=py, property_index=-1)
00:16.835  depsgraph        | WARNING Dependency cycle detected:
                            |   SCS3/PARAMETERS_EVAL() depends on
                            |   SCS3/ID_PROPERTY(ctx) via 'Driven Property -> Properties'
                            |   SCS3/DRIVER(["ctx"]) via 'Driver -> Driven Property'
                            |   SCS3/PARAMETERS_EVAL() via 'RNA Target -> Driver'
                            | 
	BPY_driver_exec: restricted access disallows name 'view_layer', enable auto-execution to support
Error in PyDriver: expression failed: len(depsgraph.view_layer.name)
For target: (type=Scene, name="S3", property=py, property_index=-1)
```
The same full move run again with `-y` (`full_enable`) and with `-Y` (`full_disable`), compared with `full`:
```
DIFF | own stored S1 scene['py'] | full: 0 | full_enable: 9
DIFF | own evaluated[ViewLayer] S1 scene['py'] | full: 0 | full_enable: 9
DIFF | own evaluated[Off] S1 scene['py'] | full: 0 | full_enable: 3
DIFF | own evaluated[ViewLayer] S2 scene['py'] | full: 0 | full_enable: 9
DIFF | own evaluated[Off] S2 scene['py'] | full: 0 | full_enable: 3
DIFF | own evaluated[ViewLayer] S3 scene['py'] | full: 0 | full_enable: 9
DIFF | own evaluated[Off] S3 scene['py'] | full: 0 | full_enable: 3
COMPARED full_enable with full: 14784 names in full, 14784 in full_enable, 14 differences
```
```
COMPARED full_disable with full: 14784 names in full, 14784 in full_disable, 0 differences
```

## 3. In a file with several scenes, which scenes' data does Blender work out when the file is opened with no window, and when a script moves a scene other than the one the file opened on?

**Label: documented** for the background-load option and API scope below; **inferred** for the limited conclusions drawn from them; **not in the sources** for the exact set of scenes evaluated in either case. **Observed** (hand tests, 2026-11-13, on macOS with no window): which scenes' graphs opening a three-scene test file and moving its scenes worked out, with and without `--disable-depsgraph-on-file-load`, with `-S`, and with five kinds of tie between scenes — in the observed block at the end of this answer.

For background file loading, the manual documents `--disable-depsgraph-on-file-load`:

> Background mode: Do not systematically build and evaluate ViewLayers’ dependency graphs
> when loading a blend-file in background mode (-b or -c options).

> Scripts requiring evaluated data then need to explicitly ensure that
> an evaluated depsgraph is available
> (e.g. by calling depsgraph = context.evaluated_depsgraph_get()).

> NOTE: this is a temporary option, in the future depsgraph will never be
> automatically generated on file load in background mode.

Source: Manual, [Command Line Arguments — Other Options](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). Date read: 2026-11-08.

**Inference:** the option's wording indicates that automatic View Layer graph construction/evaluation normally occurs on background load. It does not identify whether the graphs belong only to the opening scene, to every scene, or to some other set. The future-tense note is not evidence that automatic evaluation is already absent in this snapshot. With the option enabled, the manual removes systematic evaluation; it does not promise that no evaluation can occur for any other reason.

The API also warns:

> Since drivers may be evaluated immediately after loading a blend-file it is necessary
> to ensure the driver name-space is initialized beforehand.

Source: Python API, [Application Data — driver_namespace, File Loading & Order of Initialization](https://docs.blender.org/api/5.3/bpy.app.html). Date read: 2026-11-08. “May” does not specify which scenes' drivers run.

For a subsequent move, the Scene API says:

> Set scene frame updating all objects and view layers immediately

Source: Python API, [Scene — frame_set](https://docs.blender.org/api/5.3/bpy.types.Scene.html). Date read: 2026-11-08.

**Inference:** calling this instance method on the other Scene is described as setting that scene's frame. The description does not state that the scene must be current. It also does not specify graph creation for an inactive scene, cross-scene dependencies, whether the opening scene is evaluated too, or whether its context changes. No exact scene list can be supplied from these passages.

For the second reading (assigning `other_scene.frame_current`): as quoted in answer 1, `Scene.frame_current` instructs callers to update animation data with `frame_set()` instead (**documented**), and the sources do not state what assigning `frame_current` on a non-current scene evaluates, defers, or skips (**not in the sources**).

Do not substitute the current-context accessor for evidence about the other scene. `Context.evaluated_depsgraph_get` explicitly returns the graph “for the current scene and view layer” (full quote in answer 1). The API distinguishes the original and evaluated scene/View Layer on a graph. Those descriptions identify a graph already obtained; they do not describe all graphs built during load or during a non-current scene's frame move.

There are documented cross-scene relationships, but no general frame-move rule here. The manual says:

> Each blend-file can contain multiple scenes,
> which may share other data such as objects, materials, and collections.

Source: Manual, [Scene Introduction](https://docs.blender.org/manual/en/5.3/scene_layout/scene/introduction.html). Date read: 2026-11-08.

It also says:

> Background Scenes can themselves have a Background Scene (they’re recursively included).

Source: Manual, [Scene Properties — Background Scene](https://docs.blender.org/manual/en/5.3/scene_layout/scene/properties.html). Date read: 2026-11-08. “Background Scene” here is a scene relationship, not the no-window startup mode.

The release notes state:

> **Node Groups**: The compositor now renders scenes referenced inside node groups.

Source: Release notes, [Blender 5.3 — Compositor](https://developer.blender.org/docs/release_notes/5.3/compositor/). Date read: 2026-11-08. This concerns compositor rendering. It does not say those scenes are evaluated merely by file opening or a `frame_set` call.

**Search record.** Searched all supplied manual, API, release-note and Cycles text for `background`, `headless`, `no window`, `file load`, `loading`, `open_mainfile`, `disable-depsgraph-on-file-load`, `active scene`, `inactive scene`, `non-active scene`, `other scenes`, `all scenes`, `each scene`, `multiple scenes`, `several scenes`, `frame_set` and `frame_current`. Also searched combinations of `scene` with `evaluation`/`update`/`frame`/`load`, including all/each/other/active qualifiers and the terms in reverse order. Opened the manual's Command Line Arguments, Scene Introduction, Scene Properties, Driver Workflow & Examples and Scripting & Security; the API's Scene, Context, Depsgraph, ViewLayer, ID, [Window Manager Operators — open_mainfile](https://docs.blender.org/api/5.3/bpy.ops.wm.html), Application Data and [Application Handlers](https://docs.blender.org/api/5.3/bpy.app.handlers.html); and the Core, Python API, Animation & Rigging and Compositor release notes. Searches covered every release-note subject, not just those pages.

Ruled out `open_mainfile.load_ui` (paraphrased without a direct quote; [PENDING: exact quoted words and date read if needed — sent back for research]) as a statement of graph-evaluation scope. Ruled out the existence of several scenes or shared data as proof that every scene is evaluated. Ruled out Background Scene inclusion, Scene Strip playback/rendering (uncited here) and compositor referenced-scene rendering as a general rule for background file loading or moving a non-current scene. Driver namespace initialization establishes possible evaluation during loading, not the scene set. Cycles graph hand-offs and file-version migration loops do not establish Blender's general scene evaluation policy.

**Not covered.** Whether startup evaluates only the opening scene, all scenes or a subset; which View Layers participate; the effect of existing versus missing graphs; cross-scene/shared-data evaluation; the exact behavior of an inactive scene's `frame_set` or `frame_current`; or whether results equal those obtained after making it current.

**Outside the interface.** The load option expressly applies to background mode (`-b` or `-c`). The non-current-scene frame-move questions remain unanswered for that mode. The sources do not establish equality with an interactive scene switch and frame move. **Observed (hand tests, 2026-11-13):** every result in the observed block below was got with Blender started from the terminal with no window (`-b --factory-startup`), so it holds when Blender is driven from outside its window, on the machine named there. Nothing was run with the window open, so whether the same happens inside the window was not tested.

**Operating system.** The background-load option is not marked with an operating-system restriction. The sources do not establish the exact scene-evaluation policy on every supported operating system or identify differences in that policy. **Observed (hand tests, 2026-11-13):** on macOS 26.6.2 on Apple silicon only (the Creator's Mac, an Apple M1 Pro). Linux and Windows were not tested: no such machine was available for that sitting, so whether these results hold there is still open.

**Observed — hand tests, 2026-11-13** (Claude Opus 5.5, in Claude Code). Same machine, Blender, start-up and conventions as the observed block under question 1: the Creator's Mac (Apple M1 Pro with its own 14-core graphics, Metal; 16 GB; macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), no window, `-b --factory-startup --debug-depsgraph-eval`, with the same handler script and the same leaving-out in quotations. The same tests ran in 4.5, 5.0, 5.1 and 5.2 and are recorded in the hand-test list only. Not tested: Blender with its window open; Linux; Windows.

*The file.* Three scenes `A` (View Layers `L1`, `L2`), `B` and `C` (`L1` each), each frames 1–10 and stored at frame 1, each with its own empty (`CubeA`, `CubeB`, `CubeC`) whose location X is keyed 1 at frame 1 and 10 at frame 10, and a custom property `p` keyed the same way; stored values 42. Written with `bpy.data.libraries.write`, it opens on `A`. A second copy, `abc_onB.blend`, was saved with Blender's ordinary save after the window's scene had been set to `B`, so it opens on `B` (setting the window's scene worked `B` out at frame 1 before the save: the build printed `stored Cube X 1 stored p 1` for `B` and 42 for `A` and `C`).

*Opening the file and moving the scene it opened on, then another* — a script read every scene as opened, called `A.frame_set(5)`, then `B.frame_set(7)`, then read every graph:
```
HANDLER load_pre (None,)
00:00.337  blend            | Read blend: "…/files/5.3/abc.blend"
00:00.340  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCA (0x8f2df4008)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x8f2df4008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x8f244e408) at time 1.000000
SCRIPT mode moves version 5.3.0 file abc.blend
R as opened | scene A frame 1 | stored CubeA X 1 | stored p 1
R as opened | scene B frame 1 | stored CubeB X 42 | stored p 42
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x8f2df4008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x8f244e408) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0x8f2df5c08)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0x8f244de08) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0x8f2df5c08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 42 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0x8f2df7808)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x8f2df7808) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x8f244d808) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 42 | stored p 42
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```
The same with `--disable-depsgraph-on-file-load` before the file name:
```
HANDLER load_pre (None,)
00:00.335  blend            | Read blend: "…/files/5.3/abc.blend"
00:00.337  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
SCRIPT mode moves version 5.3.0 file abc.blend
R as opened | scene A frame 1 | stored CubeA X 42 | stored p 42
R as opened | scene B frame 1 | stored CubeB X 42 | stored p 42
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: deg_create_eval_copy on SCA (0x795d14008)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x795d14008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x79942de08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0x795d15c08)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0x79942e408) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0x795d15c08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.001
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 42 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0x795d17808)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x795d17808) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x79942ea08) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 42 | stored p 42
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```
The same with `-S B` after the file name:
```
HANDLER load_pre (None,)
00:00.336  blend            | Read blend: "…/files/5.3/abc.blend"
00:00.338  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCA (0xa194dc408)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xa194dc408) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xa1b480008) at time 1.000000
HANDLER frame_change_pre scene B frame 1 no graph
Scene switch for render: 'B' in file: '…/files/5.3/abc.blend'
[SCB :: L1]: deg_create_eval_copy on SCB (0xa194de008)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0xa194de008) at time 1.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xa1b480608) at time 1.000000
HANDLER frame_change_post scene B frame 1 graph scene B view layer L1 mode VIEWPORT
SCRIPT mode moves version 5.3.0 file abc.blend
R as opened | version 5.3.0 | context scene B | window scenes ['B']
R as opened | scene A frame 1 | stored CubeA X 1 | stored p 1
R as opened | scene B frame 1 | stored CubeB X 1 | stored p 1
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xa194dc408) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xa1b480008) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0xa194dfc08)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xa1b480c08) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xa194dfc08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after A.frame_set(5) | version 5.3.0 | context scene B | window scenes ['B']
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 1 | stored p 1
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0xa194de008) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xa1b480608) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | version 5.3.0 | context scene B | window scenes ['B']
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 7 | stored p 7
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | graph mode VIEWPORT | graph scene_eval frame None | CubeC X 42 | CubeC in graph objects False | p 42
```
`abc_onB.blend`:
```
HANDLER load_pre (None,)
00:00.338  blend            | Read blend: "…/files/5.3/abc_onB.blend"
HANDLER load_post (None,)
[SCB :: L1]: deg_create_eval_copy on SCB (0xa746ac008)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0xa746ac008) at time 1.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xa70429e08) at time 1.000000
SCRIPT mode moves version 5.3.0 file abc_onB.blend
R as opened | version 5.3.0 | context scene B | window scenes ['B']
R as opened | scene A frame 1 | stored CubeA X 42 | stored p 42
R as opened | scene B frame 1 | stored CubeB X 1 | stored p 1
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: deg_create_eval_copy on SCA (0xa746adc08)
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xa70429808) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xa746adc08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0xa746af808)
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xa746af808) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xa70428c08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.001
R after A.frame_set(5) | version 5.3.0 | context scene B | window scenes ['B']
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 42 | stored p 42
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 1 | stored p 1
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0xa746ac008) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xa70429e08) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | version 5.3.0 | context scene B | window scenes ['B']
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 42 | stored p 42
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 7 | stored p 7
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```
The file opened from the script with `bpy.ops.wm.open_mainfile(filepath=…)` printed the same lines as the plain run, apart from its own `STEP` lines around the opening and Blender's `Warning: Library file, loading empty scene` printed after the opening's debug lines. Moving only `B` (`B.frame_set(7)`, then reading every graph):
```
HANDLER load_pre (None,)
00:00.338  blend            | Read blend: "…/files/5.3/abc.blend"
00:00.341  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCA (0x791094c08)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x791094c08) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x7935ade08) at time 1.000000
SCRIPT mode moveB version 5.3.0 file abc.blend
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0x791096808)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x791096808) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x7935ad808) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 1 | stored CubeA X 1 | stored p 1
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 42 | stored p 42
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after B.frame_set(7) | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 1 | CubeA X 1 | CubeA in graph objects True | p 1
RE after B.frame_set(7) | scene A View Layer L2 | ViewLayer.depsgraph is None
RE after B.frame_set(7) | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE after B.frame_set(7) | scene C View Layer L1 | ViewLayer.depsgraph is None
```

*Moving a scene that is not the window's, in the heavy three-scene file of question 2* (each scene one frame long, stored at frame 1; `S1` is the window's scene). After `S.frame_set(f)` for each in turn, the keyed and driven values, stored and in each graph:
```
WATCH after own move | S1 | render.filter_size | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | render.dither_intensity | stored 0.75 | evaluated (per View Layer graph, no update asked) ViewLayer=0.75 Off=0.75
WATCH after own move | S1 | cycles.samples | stored 20 | evaluated (per View Layer graph, no update asked) ViewLayer=20 Off=20
WATCH after own move | S1 | cycles.preview_samples | stored 1024 | evaluated (per View Layer graph, no update asked) ViewLayer=1024 Off=1024
WATCH after own move | S1 | view_layers['ViewLayer'].samples | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | view_layers['Off'].samples | stored 11 | evaluated (per View Layer graph, no update asked) ViewLayer=11 Off=11
WATCH after own move | S1 | scene['ctx'] | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=11
WATCH after own move | S1 | scene['py'] | stored 0 | evaluated (per View Layer graph, no update asked) ViewLayer=0 Off=0
WATCH after own move | S1 | effect Comp Amount | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | Bright.bright | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S1 | Bright.contrast | stored 10 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S2 | render.filter_size | stored 1.5 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | render.dither_intensity | stored 1 | evaluated (per View Layer graph, no update asked) ViewLayer=0.75 Off=0.75
WATCH after own move | S2 | cycles.samples | stored 7 | evaluated (per View Layer graph, no update asked) ViewLayer=60 Off=60
WATCH after own move | S2 | cycles.preview_samples | stored 1024 | evaluated (per View Layer graph, no update asked) ViewLayer=1024 Off=1024
WATCH after own move | S2 | view_layers['ViewLayer'].samples | stored 5 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | view_layers['Off'].samples | stored 6 | evaluated (per View Layer graph, no update asked) ViewLayer=31 Off=31
WATCH after own move | S2 | scene['ctx'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=31
WATCH after own move | S2 | scene['py'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=0 Off=0
WATCH after own move | S2 | effect Comp Amount | stored 0.75 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | Bright.bright | stored 0.5 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S2 | Bright.contrast | stored 0.25 | evaluated (per View Layer graph, no update asked) ViewLayer=30 Off=30
WATCH after own move | S3 | render.filter_size | stored 1.5 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | render.dither_intensity | stored 1 | evaluated (per View Layer graph, no update asked) ViewLayer=1 Off=1
WATCH after own move | S3 | cycles.samples | stored 7 | evaluated (per View Layer graph, no update asked) ViewLayer=140 Off=140
WATCH after own move | S3 | cycles.preview_samples | stored 33 | evaluated (per View Layer graph, no update asked) ViewLayer=10 Off=10
WATCH after own move | S3 | view_layers['ViewLayer'].samples | stored 5 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | view_layers['Off'].samples | stored 6 | evaluated (per View Layer graph, no update asked) ViewLayer=50 Off=50
WATCH after own move | S3 | scene['ctx'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=50
WATCH after own move | S3 | scene['py'] | stored 42 | evaluated (per View Layer graph, no update asked) ViewLayer=0 Off=0
WATCH after own move | S3 | effect Comp Amount | stored 0.75 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | Bright.bright | stored 0.5 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
WATCH after own move | S3 | Bright.contrast | stored 0.25 | evaluated (per View Layer graph, no update asked) ViewLayer=70 Off=70
```

*Scenes tied together.* Five more copies of the three-scene file, each with one tie; a script called `A.frame_set(5)` and read every graph straight after (`moveA`), and in a second run did the full `moves` sequence. Shared object `Shared` (X keyed 100 × frame, stored 4200) in both `A` and `B`, `moves`:
```
HANDLER load_pre (None,)
00:00.331  blend            | Read blend: "…/files/5.3/abc_shared.blend"
00:00.333  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCA (0x7eb448008)
[SCA :: L1]: BKE_animsys_eval_animdata on OBShared (0x7e74dd808) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x7eb448008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x7e74dde08) at time 1.000000
SCRIPT mode moves version 5.3.0 file abc_shared.blend
R as opened | scene A frame 1 | stored CubeA X 1 | stored p 1
R as opened | scene B frame 1 | stored CubeB X 42 | stored p 42
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
R as opened | stored Shared X 100
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x7eb448008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBShared (0x7e74dd808) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x7e74dde08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0x7eb449c08)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0x7e74dcc08) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBShared (0x7e74de408) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0x7eb449c08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 42 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
R after A.frame_set(5) | stored Shared X 500
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0x7eb44b808)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0x7eb44b808) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBShared (0x7e74df008) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0x7e74dea08) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 42 | stored p 42
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
R after B.frame_set(7) | stored Shared X 500
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | Shared X 500 | Shared in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | Shared X 500 | Shared in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | Shared X 700 | Shared in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```
`A`'s Background Scene set to `B`, `moves`:
```
HANDLER load_pre (None,)
00:00.339  blend            | Read blend: "…/files/5.3/abc_bgset.blend"
00:00.341  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCB (0xb1e54a808)
[SCA :: L1]: deg_create_eval_copy on SCA (0xb1e548c08)
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xb21540008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCB (0xb1e54a808) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xb1e548c08) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xb21540608) at time 1.000000
SCRIPT mode moves version 5.3.0 file abc_bgset.blend
R as opened | scene A frame 1 | stored CubeA X 1 | stored p 1
R as opened | scene B frame 1 | stored CubeB X 1 | stored p 1
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCB (0xb1e54a808) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xb21540608) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xb1e548c08) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xb21540008) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0xb1e54c408)
[SCA :: L2]: deg_create_eval_copy on SCB (0xb1e54e008)
[SCA :: L2]: BKE_animsys_eval_animdata on SCB (0xb1e54e008) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeB (0xb21541208) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xb21540c08) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xb1e54c408) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.001
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 5 | stored p 5
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0xb1e54fc08)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0xb1e54fc08) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xb21541808) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 5 | stored p 5
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```
`A`'s compositor with a Render Layers node reading scene `B`, `moveA`:
```
HANDLER load_pre (None,)
00:00.339  blend            | Read blend: "…/files/5.3/abc_comp.blend"
00:00.341  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCB (0x80ea2dc08)
[SCA :: L1]: deg_create_eval_copy on SCA (0x80ea2c008)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x80ea2c008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x80b013008) at time 1.000000
SCRIPT mode moveA version 5.3.0 file abc_comp.blend
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0x80ea2c008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0x80b013008) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCB (0x80ea31408)
[SCA :: L2]: deg_create_eval_copy on SCA (0x80ea2f808)
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0x80ea2f808) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0x80b012a08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 42 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after A.frame_set(5) | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after A.frame_set(5) | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after A.frame_set(5) | scene B View Layer L1 | ViewLayer.depsgraph is None
RE after A.frame_set(5) | scene C View Layer L1 | ViewLayer.depsgraph is None
```
`A`'s sequencer with a scene strip showing `B`, `moveA`:
```
HANDLER load_pre (None,)
00:00.330  blend            | Read blend: "…/files/5.3/abc_strip.blend"
00:00.332  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCB (0xadc205c08)
[SCA :: L1]: deg_create_eval_copy on SCA (0xadc204008)
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xadc204008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xadc339808) at time 1.000000
SCRIPT mode moveA version 5.3.0 file abc_strip.blend
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xadc204008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xadc339808) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCB (0xadc209408)
[SCA :: L2]: deg_create_eval_copy on SCA (0xadc207808)
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xadc207808) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xadc339208) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.000
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 42 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE after A.frame_set(5) | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after A.frame_set(5) | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5
RE after A.frame_set(5) | scene B View Layer L1 | ViewLayer.depsgraph is None
RE after A.frame_set(5) | scene C View Layer L1 | ViewLayer.depsgraph is None
```
`A`'s custom properties `fromB` (driven by `B`'s `p`, Single Property) and `fromCubeB` (driven by `CubeB`'s world X, Transform Channel), both stored 42, `moves`:
```
HANDLER load_pre (None,)
00:00.336  blend            | Read blend: "…/files/5.3/abc_driver.blend"
00:00.339  reports          | WARNING Library file, loading empty scene
HANDLER load_post (None,)
[SCA :: L1]: deg_create_eval_copy on SCB (0xac6ef1c08)
[SCA :: L1]: deg_create_eval_copy on SCA (0xac6ef0008)
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xac3f45808) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xac3f45e08) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xac6ef0008) at time 1.000000
[SCA :: L1]: BKE_animsys_eval_driver on SCA (0xac6ef0008) fcu ["fromCubeB"][0] (0xac704a3c0)
[SCA :: L1]: BKE_animsys_eval_driver on SCA (0xac6ef0008) fcu ["fromB"][0] (0xac704a320)
SCRIPT mode moves version 5.3.0 file abc_driver.blend
R as opened | scene A frame 1 | stored CubeA X 1 | stored p 1 | stored fromB 42 | stored fromCubeB 1
R as opened | scene B frame 1 | stored CubeB X 1 | stored p 42
R as opened | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN A.frame_set(5)
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xac3f45808) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_animdata on SCA (0xac6ef0008) at time 5.000000
[SCA :: L1]: BKE_animsys_eval_driver on SCA (0xac6ef0008) fcu ["fromCubeB"][0] (0xac704a3c0)
[SCA :: L1]: BKE_animsys_eval_driver on SCA (0xac6ef0008) fcu ["fromB"][0] (0xac704a320)
[SCA :: L1]: BKE_animsys_eval_animdata on OBCubeA (0xac3f45e08) at time 5.000000
HANDLER frame_change_post scene A frame 5 graph scene A view layer L1 mode VIEWPORT
HANDLER frame_change_pre scene A frame 5 no graph
[SCA :: L2]: deg_create_eval_copy on SCA (0xac6ef3808)
[SCA :: L2]: deg_create_eval_copy on SCB (0xac6ef5408)
[SCA :: L2]: BKE_animsys_eval_animdata on SCA (0xac6ef3808) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_driver on SCA (0xac6ef3808) fcu ["fromB"][0] (0xac704b4a0)
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeA (0xac3f44c08) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_animdata on OBCubeB (0xac3f46408) at time 5.000000
[SCA :: L2]: BKE_animsys_eval_driver on SCA (0xac6ef3808) fcu ["fromCubeB"][0] (0xac704b680)
HANDLER frame_change_post scene A frame 5 graph scene A view layer L2 mode VIEWPORT
STEP-END A.frame_set(5) 0.001
R after A.frame_set(5) | scene A frame 5 | stored CubeA X 5 | stored p 5 | stored fromB 42 | stored fromCubeB 5
R after A.frame_set(5) | scene B frame 1 | stored CubeB X 5 | stored p 42
R after A.frame_set(5) | scene C frame 1 | stored CubeC X 42 | stored p 42
STEP-BEGIN B.frame_set(7)
HANDLER frame_change_pre scene B frame 7 no graph
[SCB :: L1]: deg_create_eval_copy on SCB (0xac6ef7008)
[SCB :: L1]: BKE_animsys_eval_animdata on SCB (0xac6ef7008) at time 7.000000
[SCB :: L1]: BKE_animsys_eval_animdata on OBCubeB (0xac3f46a08) at time 7.000000
HANDLER frame_change_post scene B frame 7 graph scene B view layer L1 mode VIEWPORT
STEP-END B.frame_set(7) 0.000
R after B.frame_set(7) | scene A frame 5 | stored CubeA X 5 | stored p 5 | stored fromB 42 | stored fromCubeB 5
R after B.frame_set(7) | scene B frame 7 | stored CubeB X 5 | stored p 42
R after B.frame_set(7) | scene C frame 1 | stored CubeC X 42 | stored p 42
RE at the end | scene A View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5 | fromB 42 | fromCubeB 5
RE at the end | scene A View Layer L2 | graph mode VIEWPORT | graph scene_eval frame 5 | CubeA X 5 | CubeA in graph objects True | p 5 | fromB 42 | fromCubeB 5
RE at the end | scene B View Layer L1 | graph mode VIEWPORT | graph scene_eval frame 7 | CubeB X 7 | CubeB in graph objects True | p 7
RE at the end | scene C View Layer L1 | ViewLayer.depsgraph is None
```

## Proposed hand tests (to be confirmed in a running Blender)

All five checks that stood here were run on 2026-11-13 as hand tests 1 to 5 (list: `Audits/Blender Research/What a move works out/Hand tests — 5.3.md`), and their results are the observed blocks under questions 1 to 3 — on macOS, with no window, in Blender 5.3.0. Still open:

- The same checks on Linux and on Windows (no such machine was available for that sitting; a Windows result must name its edition).
- The same moves made with Blender's window open, against the results got with no window.
- What a render's composited picture shows after each way of making the move smaller (the tests compared settings, not pictures).

2026-11-13 — GPT-6.1-Sol High (Codex) — What a move works out, Blender 5.3.

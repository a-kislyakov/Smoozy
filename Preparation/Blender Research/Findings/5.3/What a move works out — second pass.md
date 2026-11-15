# What a move works out — second pass, Blender 5.3

- Sources consulted: Blender 5.3 manual, Python API documentation and release notes (copies taken 2026-11-08), and Cycles' own files for 5.3 (copy taken 2026-10-16), all inside `Blender Sources/`. Read on 2026-11-13; dates read below are the snapshot dates, as instructed.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure, so the line above can be weighed properly. Before I started, the tool I run in placed two things into my context on its own: the project's `CLAUDE.md` (a short signpost telling agents to read a planning file, and telling Blender researchers not to) and a one-line-per-entry index of the user's saved notes. I did not open either file, and I did not open what they point to. Nothing in that automatic text describes how Blender works out frames, scenes, View Layers, compositors or drivers. I also read `Blender Sources/About this folder.md`, which sits inside the allowed folder and describes only how the folder is laid out. No helpers were used; all of the work below is my own.

Reading of the sheet: "works out" is read as "evaluates" (Blender's own word). "Moves a scene to a frame" is read as a script changing a scene's current frame. That is ambiguous: it could mean calling `Scene.frame_set()`, or assigning `Scene.frame_current` directly. Both are covered where the sources speak to them.

---

## 1. When a script moves a scene to a frame, which data does Blender work out, and what decides which data that is?

**Label:** documented for the plain statement of what `frame_set()` updates, and for the rule that objects disabled in viewports are not evaluated. Inferred for everything that ties these together (see the steps below). The finer detail is not in the sources. **Observed** (hand tests, 2026-11-13, on macOS with no window): what assigning `frame_current`, `view_layer.update()`, `evaluated_depsgraph_get()`, `Depsgraph.update()` and `frame_set` worked out in the test files, which graphs, and which handlers ran — in the observed block at the end of this answer.

**What the sources say**

1. The method a script uses to move a scene's frame says it updates all objects and all View Layers.
   - "frame_set(frame, *, subframe=0.0) — Set scene frame updating all objects and view layers immediately"
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html (Scene.frame_set). Read 2026-11-08.

2. Setting the frame number directly is not the way to update animation.
   - frame_current: "Current frame, to update animation data from Python frame_set() instead"
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html (Scene.frame_current). Read 2026-11-08.

3. View Layers expose a dependency graph reference (`Depsgraph | None`), and a dependency graph references one original scene and one original View Layer. *(Note — citation check, 2026-11-13: narrowed from "Each View Layer has its own dependency graph, built for one scene and one View Layer" to what the quoted API fields state directly, including `Depsgraph | None`; combining `ViewLayer.depsgraph` and `Depsgraph.view_layer` into "each View Layer has its own" belongs in the inferred steps below.)*
   - ViewLayer.depsgraph: "Dependencies in the scene data"
   - Depsgraph.scene: "Original scene dependency graph is built for"; Depsgraph.view_layer: "Original view layer dependency graph is built for"; Depsgraph.scene_eval: "Scene at its evaluated state"; Depsgraph.view_layer_eval: "View layer at its evaluated state"; Depsgraph.mode: "Evaluation mode (default 'VIEWPORT', readonly)" with the choices "Viewport – Viewport non-rendered mode." and "Render – Render."
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html and https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html. Read 2026-11-08.

4. What goes into an object's evaluated state.
   - "Consider the calculations that might contribute to the object's final transformation, this includes: Animation function curves. Drivers and their Python expressions. Constraints. Parent objects and all of their F-Curves, constraints, etc."
   - "To avoid expensive recalculations every time a property is modified, Blender defers the evaluation until the results are needed."
   - "Now all dependent data (child objects, modifiers, drivers, etc.) have been recalculated and are available to the script within the active view layer." (said of `bpy.context.view_layer.update()`)
   - API documentation, https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html ("Stale Data"). Read 2026-11-08.
   - From the dependency graph examples: "This object has animation and drivers applied on it, together with constraints and modifiers." and "NOTE: All ID types have `evaluated_get()`, including materials, node trees, worlds."
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html. Read 2026-11-08.

5. Outliner visibility and exclusion toggles (viewport evaluation and view-layer inclusion). *(Note — citation check, 2026-11-13: narrowed from "What decides whether an object is evaluated at all: its place in the View Layer" to match what the quotes under item 5 actually state; the view-layer membership generalization is in the inferred section below.)*
   - Hide in Viewports: "Objects hidden this way are still part of the view layer, so they still get evaluated and affect playback performance."
   - Disable in Viewports: "Toggles the visibility of the object or collection in (only) the 3D Viewport, for all view layers." … "Objects hidden this way are no longer part of the view layer, so they no longer get evaluated and don't affect playback performance."
   - Exclude from View Layer: "Toggles the collection's inclusion in the current View Layer. When excluded, contents will be hidden in the 3D Viewport, the render, and the Outliner."
   - Manual, https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html. Read 2026-11-08.

**Inferred, with the steps written out**

- Which data: from (1), a `frame_set()` call updates the objects and the View Layers. The method belongs to a scene, so I take "all objects and view layers" to mean that scene's objects and that scene's View Layers. The text does not say "of this scene"; that is my step.
- Which data, per View Layer: from (3), `ViewLayer.depsgraph` exposes a `Depsgraph | None` and a `Depsgraph` is built for one original scene and one original View Layer, so I infer that each View Layer has its own dependency graph (when built), holding an evaluated scene and an evaluated View Layer. From there, I take it that "updating … view layers" means each View Layer's dependency graph is brought to the new frame, and that this covers the scene's own evaluated state as well as the objects. The text does not say this outright. *(Note — citation check, 2026-11-13: made the cross-page inference step from `ViewLayer.depsgraph` and `Depsgraph.view_layer` explicit here.)*
- What decides it: from (5), an object or collection marked Disable in Viewports is "no longer part of the view layer" and is "no longer" evaluated. An object only hidden (Hide in Viewports) is still evaluated. So membership of the View Layer decides what gets evaluated under that viewport/playback rule, and Disable in Viewports removes things from it. The manual says this in terms of playback; it does not mention `frame_set()` or a script. *(Note — citation check, 2026-11-13: narrowed "decides what is evaluated" to the viewport/playback rule stated in the quotes.)*
- Exclude from View Layer: the manual says excluded contents are hidden "in the 3D Viewport, the render, and the Outliner". It does not say whether they are evaluated. I do not infer either way.
- `frame_current` versus `frame_set()`: from (2), assigning `frame_current` alone is not the documented way to update animation data. The sources do not say what, if anything, is worked out when only `frame_current` is assigned.

**Not covered**

- Whether the scene's own animated settings, its compositor node trees, the View Layers' own settings, or drivers placed on any of these are part of what `frame_set()` evaluates. The sources say "all objects and view layers" and nothing more specific.
- Whether a View Layer switched off for rendering ("Enable or disable rendering of this View Layer", ViewLayer.use) is still updated by `frame_set()`. Not stated.
- Whether contents of an excluded collection are evaluated. Not stated.
- Which evaluation mode (Viewport or Render) is used when a script calls `frame_set()` with no window. The default for a dependency graph is stated as Viewport; how that applies here is not.
- Whether the frame change handlers run on a `frame_set()` call. They are documented as "Called after frame change for playback and rendering, after the data has been evaluated for the new frame" (https://docs.blender.org/api/5.3/bpy.app.handlers.html); scripts are not mentioned.
- Release notes for 5.3: no entry on frame changes or evaluation (searched; see the record).

**Record of where I looked**

- Searched every file of the 5.3 API documentation, manual, release notes and Cycles files for: `frame_set`, `frame_current`, `subframe`, `depsgraph`, `dependency graph`, `evaluat`, `frame change`, `frame_change`, `update(`, `update_tag`, `evaluated_depsgraph_get`, `view_layer.update`, `recalculat`, `recomput`, `background mode`, `--background`.
- `frame_set` appears only in Scene (the move itself), RenderEngine (a different method: "Evaluate scene at a different frame (for motion blur)", for render engines) and two unrelated operators (`anim.start_frame_set`, `anim.end_frame_set`). No manual page mentions it.
- Opened in full or in the relevant part: API pages Scene, Depsgraph, ViewLayer, Context (`evaluated_depsgraph_get`), ID (`evaluated_get`, `update_tag`), LayerCollection, Object, Collection, bpy.app.handlers, the "Internal Data" gotchas page, RenderEngine; manual pages Outliner interface, View Layer introduction, View Layer panel, Collections, Drivers (panel, introduction, workflow examples), command-line arguments, Spreadsheet, Visibility, Simulation Zone, Baking.
- Ruled out: the RenderEngine `frame_set` (render engines only, about motion blur); "Render Single Layer" / `use_single_layer` ("Only affects rendering from the interface, ignored for rendering from command line") because it is about rendering, not frame moves; Simplify settings (they change geometry detail; nothing about what a frame change evaluates); simulation and baking pages (they describe caches for physics, not which data a move evaluates).
- Cycles' files: `depsgraph` appears only where Cycles receives a dependency graph to render or draw, and in two comments about forcing an update from the interface (`properties.py` line 412, `ui.py` line 33). Nothing on frame moves.

**Outside the interface.** The `frame_set()` text and the dependency graph pages are Python API documentation and make no distinction between running with or without a window. The Outliner text about Disable in Viewports is written about the interface and playback; the sources do not say whether it holds the same way for a script with no window. The sources do not say. **Observed (hand tests, 2026-11-13):** every result in the observed block below was got with Blender started from the terminal with no window (`-b --factory-startup`), so it holds when Blender is driven from outside its window, on the machine named there. Nothing was run with the window open, so whether the same happens inside the window was not tested.

**Operating system.** No statement in the sources limits any of this to one operating system. The sources do not say. **Observed (hand tests, 2026-11-13):** on macOS 26.6.2 on Apple silicon only (the Creator's Mac, an Apple M1 Pro). Linux and Windows were not tested: no such machine was available for that sitting, so whether these results hold there is still open.

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

---

## 2. Can a script reduce what Blender works out on such a move? If so, by what means, and which values — the scene's own settings, its View Layers' settings, its compositor, and drivers on any of these — still come out the same as after a full move?

**Label:** documented that Disable in Viewports stops evaluation in viewport playback (and that `--disable-depsgraph-on-file-load` suppresses automatic dependency-graph building on background load). Inferred that Disable in Viewports applies to a script's frame move, and inferred that single-View-Layer or single-dependency-graph update calls reduce evaluation compared with `frame_set()` updating all objects and View Layers. Which of the four kinds of values come out the same: not in the sources. *(Note — citation check, 2026-11-13: separated the documented reduction controls from the inferred contrast of updating a single View Layer or dependency graph instead of all View Layers.)* **Observed** (hand tests, 2026-11-13, on macOS with no window): sixteen ways of moving, or of making the move smaller, tried on one heavy test file of three scenes, each timed and each compared setting by setting with a full move — in the observed block at the end of this answer.

**Controls and update calls cited from the sources** *(Note — citation check, 2026-11-13: renamed from "Means the sources name" so item 2 is not presented as a documented evaluation-reduction mechanism.)*

1. Disable in Viewports, on objects or collections.
   - "Objects hidden this way are no longer part of the view layer, so they no longer get evaluated and don't affect playback performance."
   - Manual, https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html. Read 2026-11-08.
   - The script-side setting: Object.hide_viewport "Globally disable in viewports (default False)"; Collection.hide_viewport "Globally disable in viewports (default False)".
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.Object.html and https://docs.blender.org/api/5.3/bpy.types.Collection.html. Read 2026-11-08.
   - Inferred: that `hide_viewport` is the same switch as the manual's Disable in Viewports. The step: the manual's switch acts "for all view layers" and is the long-term, global one; the API's is "Globally disable in viewports". The sources do not name them as the same thing.
   - Inferred: that this also cuts what `frame_set()` evaluates. The step: question 1 found that `frame_set()` updates View Layers, and these objects are "no longer part of the view layer". The manual talks only about playback.

2. Updating a single View Layer or dependency graph (inferred as a way to update only one View Layer instead of all of them, by contrast with `frame_set()` updating "all objects and view layers"). *(Note — citation check, 2026-11-13: narrowed and labelled inferred here because the four API/gotcha quotes below describe updating a single View Layer or dependency graph after edits, not reducing frame-move evaluation.)*
   - ViewLayer.update(): "Update data tagged to be updated from previous access to data or operators"
   - "you need to call bpy.types.ViewLayer.update after modifying values … Now all dependent data (child objects, modifiers, drivers, etc.) have been recalculated and are available to the script within the active view layer."
   - Context.evaluated_depsgraph_get(): "Get the dependency graph for the current scene and view layer … If any data-blocks have been edited, the dependency graph will be updated."
   - Depsgraph.update(): "Re-evaluate any modified data-blocks, for example for animation or modifiers."
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html, https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html, https://docs.blender.org/api/5.3/bpy.types.Context.html, https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html. Read 2026-11-08.
   - Not in the sources: whether assigning `frame_current` and then calling one of these on one View Layer gives that View Layer the same result as `frame_set()`. The `frame_current` text says only "to update animation data from Python frame_set() instead". No source describes this combination.

3. Not building dependency graphs when the file is opened (this is about opening, not moving; it is listed because it changes what is in place before the first move).
   - "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options). Scripts requiring evaluated data then need to explicitly ensure that an evaluated depsgraph is available (e.g. by calling depsgraph = context.evaluated_depsgraph_get()). NOTE: this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."
   - Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html. Read 2026-11-08.

**Which values come out the same: what bears on it, and what does not**

- Drivers can give different results depending on which scene and View Layer evaluate them:
  - "Scripted driver expressions have access to a built-in depsgraph variable containing the dependency graph that is currently evaluating the driver. This provides the correct scene and view layer when multiple contexts are evaluated at the same time, such as a render running while another view layer is displayed in a viewport."
  - Manual, https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html. Read 2026-11-08.
  - Context Property variables: "Provides the value of a property that is implicitly referring to either a scene or a view layer of the currently evaluating animation system." with "Context: Active Scene, Active View Layer."
  - Manual, https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html. Read 2026-11-08.
  - Inferred: a driver that reads the evaluating View Layer can come out differently depending on which View Layer's dependency graph runs it. So if a reduced move evaluates fewer View Layers, such a driver has a value only for the View Layers that were evaluated. The step from "the driver knows which View Layer evaluates it" to "this changes under a reduced move" is mine.
- The scene's own settings, the View Layers' own settings, the compositor: no source says whether any reduced move leaves these the same as `frame_set()` does. Not in the sources.

**Not covered**

- Whether animated or driven scene settings, View Layer settings or compositor values are evaluated by `frame_set()` at all (question 1 could not settle it), and so whether any reduced route changes them.
- Whether Disable in Viewports also changes values outside the hidden objects (for example a driver that reads a hidden object). Not stated.
- Whether `--disable-depsgraph-on-file-load` changes anything about later moves. The text speaks only of file load.
- Found but not a way to reduce a move: FCurve.evaluate(frame), "Evaluate F-Curve … Value of F-Curve specific frame" (https://docs.blender.org/api/5.3/bpy.types.FCurve.html). It reads one curve at a frame without moving the scene. Whether it gives the same value as a full move for a curve on any of the four kinds of values is not stated.

**Record of where I looked**

- Searched the whole 5.3 API documentation and manual for: `evaluat`, `update(`, `update_tag`, `hide_viewport`, `exclude`, `performance`, `faster`, `speed up`, `calculat`, `computed`, `disable-depsgraph`, `driver`, `Context Property`, `compositor` with `driver`/`animat`/`evaluat`, `use_nodes`, `use_compositing`, `compositing_node_group`, `Simplify`.
- Opened: Outliner interface page, Collections page, View Layer introduction, API pages for ViewLayer, LayerCollection, Object, Collection, Context, Depsgraph, ID, FCurve, Scene, RenderSettings; manual compositing pages that mention evaluation (introduction, compositor system, scene effects, sidebar, Is Viewport, Scene Time, Render Layers).
- Ruled out: the compositing pages describe how a compositor node tree is evaluated when compositing ("A compositor node tree is evaluated from the outputs backwards", "Nodes that do not contribute to the final output are generally not evaluated"); none of them speaks of a frame move. Simplify reduces subdivision, particles and textures; it changes results rather than leaving them the same, and none of the four kinds of values is mentioned. `use_single_layer` is about rendering from the interface only. Release notes 5.3: searched all subject files for `depsgraph`, `dependency graph`, `evaluat`, `frame`, `handler`, `update`, `view layer`; nothing about reducing evaluation on a frame move.
- Also seen and judged outside the question: Python-script drivers depend on automatic script execution, which is off by default from the command line ("-Y, --disable-autoexec — Disable automatic Python script execution (Python-drivers & startup scripts), (default)"; "Simple expressions are evaluated even when Python script execution is disabled."). This affects whether such drivers run at all, not whether a reduced move matches a full one.

**Outside the interface.** The means in items 1 and 2 are Python API settings and calls and are not limited to the window by their text. The manual's statement that disabled objects are not evaluated is written about the interface and playback; the sources do not say it holds identically with no window. Item 3 exists only with no window ("background mode (-b or -c options)"). Otherwise the sources do not say. **Observed (hand tests, 2026-11-13):** every result in the observed block below was got with Blender started from the terminal with no window (`-b --factory-startup`), so it holds when Blender is driven from outside its window, on the machine named there. Nothing was run with the window open, so whether the same happens inside the window was not tested.

**Operating system.** No statement in the sources limits any of these to one operating system. The sources do not say. **Observed (hand tests, 2026-11-13):** on macOS 26.6.2 on Apple silicon only (the Creator's Mac, an Apple M1 Pro). Linux and Windows were not tested: no such machine was available for that sitting, so whether these results hold there is still open.

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

---

## 3. In a file with several scenes, which scenes' data does Blender work out when the file is opened with no window, and when a script moves a scene other than the one the file opened on?

**Label:** documented that `--disable-depsgraph-on-file-load` suppresses systematic View Layer dependency-graph building and evaluation on background load; **inferred** (by reading that option's negation, as written out below) that, with no window, Blender by default builds and evaluates View Layers' dependency graphs when it loads a file. Which scenes that covers, and which scenes a move of another scene works out: not in the sources. Additional inferred points are written out below. *(Note — citation check, 2026-11-13: changed the default background-load behavior label from documented to inferred to match item 1 below, since the quoted option states the suppression flag rather than the default behavior outright.)* **Observed** (hand tests, 2026-11-13, on macOS with no window): which scenes' graphs opening a three-scene test file and moving its scenes worked out, with and without `--disable-depsgraph-on-file-load`, with `-S`, and with five kinds of tie between scenes — in the observed block at the end of this answer.

**What the sources say**

1. Opening with no window.
   - "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)." … "NOTE: this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."
   - Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html. Read 2026-11-08.
   - Inferred: without that option, Blender in background mode does "systematically build and evaluate ViewLayers' dependency graphs" on load. The step is reading the option's negation. The text does not say whose View Layers (one scene's, or every scene's).

2. Moving a scene.
   - "Set scene frame updating all objects and view layers immediately"
   - API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html. Read 2026-11-08.
   - Inferred: the method is called on a particular scene, so the objects and View Layers it updates are taken to be that scene's. The text does not say whether the scene the file opened on, or any other scene, is also evaluated.

3. Scenes that pull in another scene.
   - Background Scene: "Allows you to use a scene as a background" … "Background Scenes can themselves have a Background Scene (they're recursively included)."
   - Manual, https://docs.blender.org/manual/en/5.3/scene_layout/scene/properties.html. Read 2026-11-08. API: Scene.background_set, "Background set scene" (https://docs.blender.org/api/5.3/bpy.types.Scene.html).
   - Inferred, weakly: a scene with a Background Scene "includes" that scene, so moving it may involve the background scene's data as well. The text says "included" and does not say evaluated.
   - Scene strips: "When used as strips, scenes are rendered dynamically during playback or rendering, and they share the same data-blocks unless they were created as full copies."
   - Manual, https://docs.blender.org/manual/en/5.3/scene_layout/scene/introduction.html. Read 2026-11-08. This is about playback and rendering, not a script's move.
   - Compositor Render Layers node: "Scene — The scene for which to render a view layer."
   - Manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html. Read 2026-11-08.
   - Release note: "The compositor now renders scenes referenced inside node groups."
   - Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/. Read 2026-11-08. Again about rendering, not a move.

4. Shared data between scenes.
   - "Each blend-file can contain multiple scenes, which may share other data such as objects, materials, and collections."
   - Manual, https://docs.blender.org/manual/en/5.3/scene_layout/scene/introduction.html. Read 2026-11-08.
   - Inferred: a shared object exists once in the file, but question 1 found that each View Layer has its own dependency graph with its own evaluated state, so a shared object could have an evaluated state in more than one scene's graph. Whether a move of one scene changes the evaluated state seen through another scene is not stated.

**Not covered**

- Whether loading with no window builds and evaluates the View Layers of every scene in the file, or only of the scene the file opened on. Not stated.
- Whether moving a scene that is not the one the file opened on also re-evaluates the opened-on scene, or any other scene. Not stated.
- Whether a Background Scene, a scene used by a scene strip, or a scene named in a Render Layers node is evaluated on a move (as opposed to on a render). Not stated.
- Whether `bpy.context.scene` changes when a script moves another scene. Not stated.

**Record of where I looked**

- Searched the whole 5.3 manual, API documentation and release notes for: `all scenes`, `other scenes`, `every scene`, `each scene`, `multiple scenes`, `inactive scene`, `background scene`, `background set`, `background_set`, `set scene`, `--scene`, `-S`, `background mode`, `--background`, `no window`, `without a window`, `bpy.app.background`, `disable-depsgraph`, `load_post`, `open_mainfile`.
- Opened: command-line arguments page (all options, including `-S, --scene <name>`: "Set the active scene <name> for rendering."), Scene properties, Scene introduction, Files introduction ("This database contains all scenes, objects, meshes, textures, etc. that are in the file."), Render Layers node page, API pages Scene, Context, Window (`scene`: "Active scene to be edited in the window"), bpy.app, bpy.ops.wm (`open_mainfile`, which has no parameter about evaluation), bpy.app.handlers, "Blender as a Python module" page, tips and tricks page (background use).
- Ruled out: `-S, --scene` is described for rendering only. The sequencer, compositor and Render Layers sources speak of rendering or playback, not a script's move. The "Blender as a Python module" page says it is "for the most part … equivalent to running a script in background-mode" and adds nothing on scenes or evaluation. Release notes 5.3: the only scene-related entry touching evaluation is the compositor one quoted above. Cycles' files: no mention of scenes being evaluated on load or on a move.

**Outside the interface.** The file-load statement applies only with no window ("background mode (-b or -c options)"); the sources do not describe what happens on load with a window in the same terms. The `frame_set()` text makes no distinction. Otherwise the sources do not say. **Observed (hand tests, 2026-11-13):** every result in the observed block below was got with Blender started from the terminal with no window (`-b --factory-startup`), so it holds when Blender is driven from outside its window, on the machine named there. Nothing was run with the window open, so whether the same happens inside the window was not tested.

**Operating system.** No statement in the sources limits any of this to one operating system. The sources do not say. **Observed (hand tests, 2026-11-13):** on macOS 26.6.2 on Apple silicon only (the Creator's Mac, an Apple M1 Pro). Linux and Windows were not tested: no such machine was available for that sitting, so whether these results hold there is still open.

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

---

## Things a person at a computer could settle in a few minutes

All six checks that stood here were run on 2026-11-13 as hand tests 1 to 5 (list: `Audits/Blender Research/What a move works out/Hand tests — 5.3.md`), and their results are the observed blocks under questions 1 to 3 — on macOS, with no window, in Blender 5.3.0. Still open:

- The same checks on Linux and on Windows (no such machine was available for that sitting; a Windows result must name its edition).
- The same moves made with Blender's window open, against the results got with no window.

---

2026-11-13 — Claude Opus 5.5 (Claude Code) — What a move works out, Blender 5.3

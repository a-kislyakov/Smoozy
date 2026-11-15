Sources consulted: Blender 5.3 manual, Python API documentation and release notes — date read 2026-11-08; supplied Cycles files — date read 2026-10-16. The question sheet supplied the questions only.
I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

# Reading at a frame — second pass, Blender 5.3

Only questions 1, 2, 3, 4, 5 and 7 are answered. No earlier findings were opened. No helpers were used. No Blender executable was run. Searches and reading of supplied files are not runtime tests of Blender.

## Search scope and shared evidence

In each question's search record, **whole-source search** means recursive, case-insensitive `rg` searches of these supplied folders only: `Blender Sources/manual_5.3`, `Blender Sources/api_5.3`, `Blender Sources/release_notes_5.3` and `Blender Sources/cycles_5.3`. The final search sweep selected `*.txt`, `*.md` and `*.py`, excluding `genindex*`; earlier searches also included the API indexes. The folders contain 2,452 manual text files, 2,248 API text files, 17 release-note Markdown files and 10 Cycles Python files. A matching-file count records search coverage, not how many pages were read in full. The opened pages and relevant sections are named separately below.

Before opening source pages, I searched across the supplied sources for `evaluate\(frame`, `frame_set`, `frame_change`, camera/marker combinations, NLA/evaluation combinations and driver/evaluation combinations. Follow-up searches expanded the vocabulary. No web sources were used.

The following documented command-line facilities support the background-script discussion in each answer. They do not, by themselves, guarantee that every API result equals a render result:

> -b, --background
>
> Run in background (often used for UI-less rendering).

Manual: [Command Line Arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html); Render Options; snapshot lines 9–11. Date read: 2026-11-08.

> -P, --python <filepath>
>
> Run the given Python script file.

Manual: [Command Line Arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html); Python Options; snapshot lines 194–196. Date read: 2026-11-08.

The manual also gives this exact background rendering example:

> To render an animation in background mode, allowing drivers and other scripts to run:
>
> blender --background --enable-autoexec my_movie.blend --render-anim

Manual: [Scripting & Security](https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html); Command Line; snapshot lines 90–92. Date read: 2026-11-08.

## 1. Can the value an animated setting has at a given frame be learned from an open file without moving the scene to that frame? If so, by what means, and how does the result compare with the value a render uses at that frame?

**Label: inferred** for obtaining a curve value without moving the scene; **documented** for the curve evaluation API; **not in the sources** for a general guarantee that it is the final setting value used by a render. **Update 2026-11-13:** observed by hand tests 2, 3 and 4 on the Creator's Mac — see *Observed* below.

I read “animated setting” here as a setting animated by keyframes in an action. Questions 2 and 3 ask separately about drivers and the NLA. The broader reading includes those mechanisms, so the limited curve answer below would not answer that broader reading.

An F-Curve can be asked for its value at a supplied frame with `fcurve.evaluate(frame)`. Its property path and component index identify what it animates. The inference is that supplying the frame directly permits a curve query without first setting the scene frame: the documented call takes a frame and returns a float, with no scene-frame operation in its signature or description. The documentation does not explicitly promise absence of all side effects. This was not exercised.

> evaluate(frame)
>
> Evaluate F-Curve
>
> Parameters:
>
> frame (float) – Frame, Evaluate F-Curve at given frame (in [-inf, inf])
>
> Returns:
>
> Value, Value of F-Curve specific frame (in [-inf, inf])
>
> Return type:
>
> float

Python API: [FCurve.evaluate](https://docs.blender.org/api/5.3/bpy.types.FCurve.html); snapshot lines 163–177. Date read: 2026-11-08.

> data_path
>
> RNA Path to property affected by F-Curve (default “”, never None)

Python API: [FCurve](https://docs.blender.org/api/5.3/bpy.types.FCurve.html); data_path; snapshot lines 53–55. Date read: 2026-11-08.

> array_index
>
> Index to the specific property affected by F-Curve if applicable (in [0, inf], default 0)

Python API: [FCurve](https://docs.blender.org/api/5.3/bpy.types.FCurve.html); array_index; snapshot lines 9–11. Date read: 2026-11-08.

> Blender lets you animate almost any property, going from the X coordinate of an object to the
> transparency of a material. The evolution of a property’s value over time is described by
> a function curve, or F-Curve for short.
>
> An important aspect of F-Curves is that they can interpolate. This saves you the effort of
> manually configuring a value on every single frame, which would be highly impractical.
> Instead, you define just a few values on key frames, and let the curve calculate the
> values on all the other frames.

Manual: [F-Curves: Introduction](https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/introduction.html); snapshot lines 3–10. Date read: 2026-11-08.

The correct action slot matters when locating the curve:

> Each action has a set of slots, and different animation data for each of those slots.
> An animated data-block then specifies both an action and a slot within that
> action, determining which animation data the data-block is animated by.

Manual: [Actions](https://docs.blender.org/manual/en/5.3/animation/actions.html); Action Slots; snapshot lines 30–32. Date read: 2026-11-08.

> fcurves
>
> The individual F-Curves that animate the slot (default None, readonly)
>
> Type:
>
> ActionChannelbagFCurves[FCurve]

Python API: [ActionChannelbag](https://docs.blender.org/api/5.3/bpy.types.ActionChannelbag.html); fcurves; snapshot lines 9–15. Date read: 2026-11-08.

This establishes a curve sample, not a general render-equivalence test. The render API receives an evaluated dependency graph, and the API distinguishes evaluated state from original data:

> render(depsgraph)
>
> Render scene into an image
>
> Parameters:
>
> depsgraph (Depsgraph | None) – Evaluated dependency graph

Python API: [RenderEngine.render](https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html); snapshot lines 407–413. Date read: 2026-11-08.

> This example demonstrates access to the evaluated ID (such as object, material, etc.) state from
> an original ID.
> This is needed every time one needs to access state with animation, constraints, and modifiers
> taken into account.

Python API: [Depsgraph](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html); Evaluated ID example; snapshot lines 5–8. Date read: 2026-11-08.

There is also a documented distinction between a scene's frame number and its time-remapped frame:

> frame_current_final
>
> Current frame with subframe and time remapping applied (in [-1.04857e+06, 1.04857e+06], default 0.0, readonly)

Python API: [Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html); frame_current_final; snapshot lines 178–180. Date read: 2026-11-08.

**Inference:** equality of the numeric frame passed to `FCurve.evaluate` and the frame named in a render request is not sufficient evidence of equality of the final setting values. The quotes establish additional evaluation and a time-remapped scene frame; they do not specify a universal conversion or comparison procedure. I therefore do not claim that the two values always agree, or always differ.

**Search record.** Whole-source final pattern: `f.?curve|interpolat|sampled_points|evaluate\(frame|time.?remap|time.?stretch|discrete` — 421 matching files. Follow-ups searched F-Curve/render, clamp, round, integer and discrete combinations, and evaluation at an arbitrary, specific, given or different time/frame. Opened the FCurve API sections for property addressing, extrapolation, modifiers, mute, sampled points, `evaluate`, `update` and `update_autoflags`; Actions through Action Slots and F-Curves & Channels; ActionChannelbag; the F-Curves introduction; [F-Curve Properties](https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/properties.html), lines 125–137; [Frame Range](https://docs.blender.org/manual/en/5.3/render/output/properties/frame_range.html), Time Stretching, lines 21–38; Scene frame properties and `frame_set`; Depsgraph evaluated-ID example; RenderEngine.render and frame_set. Searched the Graph Editor and NLA manual trees, driver pages, and release notes for evaluation and blending. Ruled out interpolation and discrete-curve descriptions as proof of final render parity: they describe curves. Ruled out `RenderEngine.frame_set` as the requested no-move query: its description is “Evaluate scene at a different frame (for motion blur)”.

**Not covered.** A guarantee of unchanged scene state after `evaluate`; final property conversion or limits; complete action-slot lookup code; driver/NLA composition; a universal mapping from render frame to curve time; equality with any particular render engine's final values.

**Observed — hand tests 2, 3, 4 and 5, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** A script-made file: Workbench, 128 × 128, an orthographic camera, a cube whose X location is keyed 0 at frame 1 and 4 at frame 11 (linear) with a Noise modifier on the curve, stored X set to 1.0, written at frame 1. With the scene at frame 1 a script read the curve's `evaluate(6.0)`; then `frame_set(6)` and the evaluated X; and a command-line render of frame 6 (`-f 6`) was compared pixel by pixel with renders of copies whose animation was removed and X fixed by hand (`max_diff=0` means the same picture). Then the same with the curve muted, and with Time Stretching Old 2, New 1. The machine's words — modifier, then muted, then Time Stretching:
```
READ fc.evaluate(6.0) 2.062980890274048
READ frame_current after evaluate 1 stored location.x after evaluate 0.06107
READ after frame_set(6): evaluated location.x 2.062981
CMP T2 curve frame 6 vs static at evaluate(6.0) max_diff=0 differing_pixels=0 of 16384
CMP T2 curve frame 6 vs static at 2.0 (the keys alone, no Noise) max_diff=63 differing_pixels=56 of 16384
READ fc.evaluate(6.0) 2.062980890274048
READ after frame_set(6): evaluated location.x 1.0
CMP T3 muted frame 6 vs static at evaluate(6.0) max_diff=156 differing_pixels=238 of 16384
CMP T3 muted frame 6 vs static at 1.0 (the stored value) max_diff=0 differing_pixels=0 of 16384
READ fc.evaluate(6.0) 2.062980890274048
READ after frame_set(6): frame_current 6 frame_current_final 12.0 stored location.x 4.297497
READ after frame_set(6): evaluated location.x 4.297497
READ fc.evaluate(frame_current_final) 4.297497272491455
CMP T4 stretch frame 6 vs static at evaluate(frame_current_final) max_diff=0 differing_pixels=0 of 16384
CMP T4 stretch frame 6 vs static at evaluate(6.0) max_diff=156 differing_pixels=406 of 16384
```
The whole set-up, every line, and the keyframes-and-driver test 5, are under question 1 of the first pass beside this document, `Preparation/Blender Research/Findings/5.3/Reading at a frame.md`.

**Outside the interface.** Background execution of the Python call is inferred from the shared command-line facilities and the API call, not separately guaranteed or tested. No background-specific render-parity guarantee was found. **Update 2026-11-13 — observed** (hand tests 2–4): `FCurve.evaluate` ran in a script with no window and left the frame number where it was; the renders compared were command-line renders.
**Operating system.** The sources do not state whether this result and its side-effect limits hold identically on every supported operating system. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; whether the same holds on another operating system is still open.

## 2. Where a setting is controlled by a driver, can its value at a given frame be learned without moving the scene to that frame? If so, by what means?

**Label: not in the sources** for a general arbitrary-frame, no-move driver query. The driver mechanism below is **documented**; the reason a curve sample alone is insufficient is **inferred**. **Update 2026-11-13:** observed by hand tests 6 and 7 on the Creator's Mac — see *Observed* below.

I found no documented call that takes a target frame and returns a complete driven setting value without moving the scene. This is not evidence that such a method is impossible. A driver has an input computation as well as an F-Curve:

> A driver configuration that specifies zero, one, or more input values using
> other properties or object transformation channels, and combines them using
> a predefined mathematical function or a custom Python expression.
>
> An animation F-Curve
> that maps the output of the driver configuration to the final value to apply
> to the driven property.

Manual: [Drivers: Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html); snapshot lines 8–14. Date read: 2026-11-08.

> The X axis maps to the output value of the driver configuration. The units depend on the setup.
>
> The Y axis shows the value applied to the target property. The units depend on the property.

Manual: [Drivers: Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html); Graph View; snapshot lines 37–39. Date read: 2026-11-08.

> This driver configuration outputs a single value which changes when the variables change.
> This value is then evaluated through the driver function curve to produce the result
> to be applied to the driven property.

Manual: [Drivers: Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html); Driver Configuration; snapshot lines 66–68. Date read: 2026-11-08.

**Inference:** passing a scene frame to an F-Curve query does not establish that the driver inputs have been computed for that frame. The driver curve's X coordinate is described as the output of the driver configuration. The sources do not document `FCurve.evaluate(frame)` as a replacement for evaluating that configuration and its dependencies.

Inputs can also require evaluated transformations or the current evaluation context:

> SINGLE_PROP
> Single Property – Use the value from some RNA property.
>
> TRANSFORMS
> Transform Channel – Final transformation value of object or bone.
>
> ROTATION_DIFF
> Rotational Difference – Use the angle between two bones.
>
> LOC_DIFF
> Distance – Distance between two bones or objects.
>
> CONTEXT_PROP
> Context Property – Use the value from some RNA property within the current evaluation context.

Python API: [DriverVariable](https://docs.blender.org/api/5.3/bpy.types.DriverVariable.html); type; snapshot lines 37–50. Date read: 2026-11-08.

> Scripted driver expressions have access to a built-in depsgraph variable containing the dependency graph
> that is currently evaluating the driver. This provides the correct scene and view layer when multiple contexts
> are evaluated at the same time, such as a render running while another view layer is displayed in a viewport.
>
> Avoid using bpy.context in driver expressions and custom driver functions. It represents the interface context
> and may not match the context evaluating the driver. Pass depsgraph to a custom function instead.

Manual: [Driver Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html); Dependency Graph Context; snapshot lines 101–106. Date read: 2026-11-08.

**Search record.** Whole-source final pattern: `driver|driven|expression|driver_namespace|CONTEXT_PROP|TRANSFORMS` — 2,050 matching files; many are inherited `driver_add`/`driver_remove` entries, not evaluators. The earlier whole-source searches paired driver/animation with evaluate/evaluation and sample, and searched arbitrary/specific/given/different frame evaluation. Opened Drivers Introduction; Drivers Panel sections for context properties, expressions, `depsgraph`, `self` and simple expressions; Workflow & Examples, Dependency Graph Context; the entire Driver API's own properties and methods, through its inherited-properties boundary; DriverVariable through its types and methods; FCurve.evaluate; AnimData.drivers; Depsgraph's evaluated-ID example; Context.evaluated_depsgraph_get; ID.evaluated_get. Also searched [bpy.app](https://docs.blender.org/api/5.3/bpy.app.html) for driver evaluation and namespace setup, and the supplied Cycles Python files for driver/evaluation references. Ruled out “can be evaluated without using the full Python interpreter”: that concerns expression execution, not evaluating at another frame. Ruled out `ID.evaluated_get`: it returns the last evaluation, as quoted in question 7. Ruled out the global expression name `frame` as an arbitrary-frame query API: the manual lists it as an expression global, not as a method for evaluating all dependencies at an independently supplied time.

I also opened the manual's [driver-curve image](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_introduction_fcurve.png), associated with Drivers Introduction. **Documented image finding:** the displayed guide marks X = 2.0 and Y = 0.5. It illustrates the curve mapping; it shows no arbitrary-frame driver evaluation procedure. Date read: 2026-11-08.

**Not covered.** Special cases where a person can calculate a simple driver by hand; a custom implementation of driver evaluation; complete dependency resolution at a target frame; whether an undocumented route exists.

**Observed — hand tests 6 and 7, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** Test 6: a cube's Y driven by the simple expression `frame * 2`; a scene custom property `src` keyed 1 at frame 1 and 10 at frame 10; the cube's Z driven by one Single Property variable reading `src` through a driver curve with linear keys (0, 0) and (10, 100). At frame 1 a script read the Y driver curve's `evaluate(5.0)`, worked Z out by hand for frame 10 from `src`'s curve and the driver curve, then moved:
```
READ driver curve Y .evaluate(5.0) 5.0
READ hand route for Z at frame 10: src_curve.evaluate(10.0) 10.0 then Z driver curve .evaluate(that) 100.0
READ after frame_set(5): stored location (0.0, 10.0, 50.0) evaluated location (0.0, 10.0, 50.0)
READ after frame_set(10): stored location (0.0, 20.0, 100.0) evaluated location (0.0, 20.0, 100.0) stored src 10.0
```
Test 7: the cube's X driven by `-3.0 if bpy.app else 0.0` (stored 3.0) and its Z by `[-2.0][0]` (stored 2.0), opened with no script flag, with `-Y` and with `-y`. With no flag (and the same with `-Y`):
```
	BPY_driver_exec: restricted access disallows name 'app', enable auto-execution to support
Error in PyDriver: expression failed: -3.0 if bpy.app else 0.0
READ after frame_set(5): stored location x z 0.0 -2.0 evaluated location x z 0.0 -2.0
```
With `-y`:
```
READ after frame_set(5): stored location x z -3.0 -2.0 evaluated location x z -3.0 -2.0
```
The command-line renders of frame 5 matched, pixel for pixel, a copy at X 0.0 and Z −2.0 with no flag and with `-Y`, and a copy at X −3.0 and Z −2.0 with `-y`. The whole set-up and every line are under question 2 of the first pass, `Preparation/Blender Research/Findings/5.3/Reading at a frame.md`.

**Outside the interface.** The manual explicitly shows background rendering with drivers and scripts enabled in the shared example. That does not provide the requested no-move query. Complex Python expressions and execution permissions are distinct from a query method; the Drivers Panel says “Simple expressions are evaluated even when Python script execution is disabled.” [Manual: Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Simple Expressions, line 374; date read 2026-11-08. **Update 2026-11-13 — observed** (hand test 7): with no window and no script flag, or with `-Y`, a scripted driver reading `bpy.app` was refused (`restricted access disallows name 'app'`) and its setting read 0.0, not its stored 3.0, in the script and in the render; a scripted driver `[-2.0][0]` outside the simple subset still ran. With `-y` both ran.
**Operating system.** The sources do not give an operating-system-specific answer to the no-move query, or guarantee identical behavior across all supported systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; whether the same holds on another operating system is still open.

## 3. Where a data-block's animation is made of several actions in the NLA, can the combined value at a given frame be learned without moving the scene to that frame? If so, by what means?

**Label: not in the sources** for a general combined-value, arbitrary-frame query without moving the scene. The stack and time controls below are **documented**; the limits of sampling one curve are **inferred**. **Update 2026-11-13:** observed by hand test 8 on the Creator's Mac — see *Observed* below.

I found no documented method returning the NLA stack's combined setting value at a separately supplied frame. The sources describe multiple layers, blending, influence and action sampling time:

> The editor displays a stack of tracks which work like layers in an image editing
> program. Higher tracks take precedence over lower ones, although you can also choose to blend them.
>
> Each track can contain any number of strips – typically Action Strips,
> which are instances of actions.

Manual: [NLA Editor: Introduction](https://docs.blender.org/manual/en/5.3/editors/nla/introduction.html); Main Region; snapshot lines 13–17. Date read: 2026-11-08.

> action_blend_type
>
> Method used for combining Active Action’s result with result of NLA stack (default 'REPLACE')

Python API: [AnimData](https://docs.blender.org/api/5.3/bpy.types.AnimData.html); action_blend_type; snapshot lines 17–19. Date read: 2026-11-08.

> How to combine the action’s property values with those of the tracks below.
>
> Replace
>
> Overwrites the values produced by the lower tracks. If Influence is less than 1,
> a linear interpolation between the previous and new values is used instead.
>
> Multiply, Subtract, Add
>
> Blends the action’s values with those of the lower tracks using a simple calculation.
> If Influence is less than 1, a linear interpolation between the previous values
> and these calculated values is used.

Manual: [NLA Sidebar](https://docs.blender.org/manual/en/5.3/editors/nla/sidebar.html); Blending; snapshot lines 50–61. Date read: 2026-11-08.

> Animated Influence
>
> Lets you manually specify, and animate, how strongly the strip affects the animation.
> This is an alternative to using the (Auto) Blend In/Out settings above.
>
> To create an influence keyframe, first type a value, then either click
> Insert Keyframe in its context menu or press I
> while hovering over it. You can see the keyframes in e.g. the
> Graph Editor.
>
> Animated Strip Time
>
> Lets you manually specify, and animate, the frame at which the underlying action
> is sampled.

Manual: [NLA Sidebar](https://docs.blender.org/manual/en/5.3/editors/nla/sidebar.html); Animated Influence and Animated Strip Time; snapshot lines 170–183. Date read: 2026-11-08.

The one AnimData method found with an explicit frame argument converts time for a tweaked strip:

> nla_tweak_strip_time_to_scene(frame, *, invert=False)
>
> Convert a time value from the local time of the tweaked strip to scene time, exactly as done by built-in key editing tools. Returns the input time unchanged if not tweaking.
>
> Parameters:
>
> frame (float) – Input time (in [-1.04857e+06, 1.04857e+06])
>
> invert (bool) – Invert, Convert scene time to action time (optional)
>
> Returns:
>
> Converted time (in [-1.04857e+06, 1.04857e+06])

Python API: [AnimData.nla_tweak_strip_time_to_scene](https://docs.blender.org/api/5.3/bpy.types.AnimData.html); snapshot lines 153–165. Date read: 2026-11-08.

**Inference:** sampling one action's F-Curve cannot, by itself, establish a value combined from several strips: the quoted mechanism also combines tracks and can animate both the strip's influence and the point where its action is sampled. The time-conversion method returns a time, not a combined property value. No complete independent stack evaluator was found in the supplied API or Cycles files.

**Search record.** Whole-source final pattern: `\bnla\b|nla_|non.?linear|blend_type|strip_time|action_influence|nla_tweak_strip_time_to_scene` — 96 matching files. Earlier searches paired NLA/action/animation with evaluation and sample, and searched blend, strip time, influence, extrapolation and remap across the NLA and Graph Editor manual trees and relevant API pages. Opened NLA Introduction; NLA Sidebar through active-action blending and strip playback/time controls; AnimData active-action controls, tracks, `use_nla` and time conversion; NlaStrip blending, extrapolation, `fcurves`, `strip_time`, animated influence/time and method list; FCurve.evaluate; Actions slots/layers explanation; [NLA Strip Editing](https://docs.blender.org/manual/en/5.3/editors/nla/editing/strip.html), Full Stack/Lower Stack tweaking, lines 493–528; [Pose](https://docs.blender.org/api/5.3/bpy.types.Pose.html), action application/blending, lines 71–91. Searched NlaTrack, NLA operators and animation/API release notes for evaluation. Ruled out strip `fcurves` as the combined output: the API calls them “F-Curves for controlling the strip’s influence and timing”. Ruled out Pose action application/blending: it applies a given action to selected/all bones, not a data-block's complete NLA stack. Ruled out Lower Stack tweak mode: it mutes tracks and changes which animation contributes.

I opened the [Edited Action panel image](https://docs.blender.org/manual/en/5.3/_images/editors_nla_sidebar_animation-data-panel.png), associated with NLA Sidebar. **Documented image finding:** it shows Action, Slot, Extrapolation, Blending and Influence controls. It does not show an arbitrary-frame result query. This image is not evidence that no such API exists. Date read: 2026-11-08.

**Not covered.** A hand-written NLA evaluator; all blending and time-remapping edge cases; undocumented methods; proof that an independent query is impossible.

**Observed — hand test 8, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** A cube with no active action and two NLA tracks: lower, strip `Base` (X 0 at 1, 10 at 11) from scene frame 1, Replace; upper, strip `Over` (X 0 at 1, 20 at 11) from scene frame 5, Add, Animated Influence with one key of 0.5. At frame 1 a script read each action's curve, then moved to frame 10 and also called `AnimData.nla_tweak_strip_time_to_scene(10.0)` with and without `invert=True` (not in tweak mode), then ran an NLA bake of frame 10:
```
READ action Base curve location[0].evaluate(10.0) 9.0
READ action Over curve location[0].evaluate(6.0) 10.0
READ after frame_set(10): stored location.x 14.0 evaluated location.x 14.0
READ after frame_set(10): strip Base strip_time 0.0 influence 0.0
READ after frame_set(10): strip Over strip_time 0.0 influence 0.5
READ after frame_set(10): evaluated copy's strip Base strip_time 10.0 influence 1.0
READ after frame_set(10): evaluated copy's strip Over strip_time 6.0 influence 0.5
READ after frame_set(10): AnimData.nla_tweak_strip_time_to_scene(10.0) 10.0 with invert=True 10.0 use_tweak_mode False
READ bpy.ops.nla.bake returned {'FINISHED'}
READ after bake: frame_current 1 active action Action
READ baked action location[0] keys [(10.0, 14.0)]
```
The whole set-up and every line are under question 3 of the first pass, `Preparation/Blender Research/Findings/5.3/Reading at a frame.md`.

**Outside the interface.** The supplied Python API exposes the NLA data, and background Python execution is documented in the shared evidence. Neither establishes a complete no-move NLA query in a background script. No separate interface/background rule for such a query was found. **Update 2026-11-13 — observed** (hand test 8): the NLA data, `FCurve.evaluate` and `bpy.ops.nla.bake` (cube selected and active, every argument named) all ran in a script with no window.
**Operating system.** The sources do not establish identical no-move NLA-query behavior on every supported operating system. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; whether the same holds on another operating system is still open.

## 4. When timeline markers switch a scene's camera, which camera is active at a frame before the first marker, between two markers, on a marker, and after the last? Can that be learned without moving the scene to the frame?

**Label: documented** for the camera bound to a marker becoming active on that marker's frame; **inferred** for inspecting the stored marker binding without moving the frame; **not in the sources** for the full boundary/interval selection rule. **Update 2026-11-13:** observed by hand tests 9 and 10 on the Creator's Mac — see *Observed* below.

I read “marker” as a scene timeline marker with a camera binding. The question could also include unbound markers or several camera bindings at the same frame. Those cases are not settled by the quoted rule.

> These markers can be moved to change the frame at which
> the active camera is changed to the object the marker is bound to.

Manual: [Markers](https://docs.blender.org/manual/en/5.3/animation/markers.html); Bind Camera to Markers; snapshot lines 349–350. Date read: 2026-11-08.

> camera
>
> Camera that becomes active on this frame
>
> Type:
>
> Object | None
>
> frame
>
> The frame on which the timeline marker appears (in [-inf, inf], default 0)
>
> Type:
>
> int

Python API: [TimelineMarker](https://docs.blender.org/api/5.3/bpy.types.TimelineMarker.html); camera and frame; snapshot lines 9–23. Date read: 2026-11-08.

> timeline_markers
>
> Markers used in all timelines for the current scene (default None, readonly)
>
> Type:
>
> TimelineMarkers[TimelineMarker]

Python API: [Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html); timeline_markers; snapshot lines 440–446. Date read: 2026-11-08.

> camera
>
> Active camera, used for rendering the scene

Python API: [Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html); camera; snapshot lines 102–104. Date read: 2026-11-08.

| Position of requested frame | What the supplied sources establish |
| --- | --- |
| Before the first camera-bound marker | **Not in the sources.** No rule was found choosing the first bound camera, the scene's stored camera, or some other result. |
| Between two camera-bound markers | **Not in the sources.** The text identifies change frames but does not state a complete lookup/persistence rule for arbitrary jumps. |
| On a camera-bound marker | **Documented.** Its camera is the camera that becomes active on that frame. Conflicting bindings and other code changing the camera are not covered. |
| After the last camera-bound marker | **Not in the sources.** No explicit endpoint or persistence rule was found. |

**Inference:** a script can inspect `scene.timeline_markers`, compare each stored `frame` with the requested frame and read its stored `camera`, without first moving the scene. This can identify a unique binding on that exact frame. It does not establish the missing interval rules. Reading `scene.camera` tells which camera the scene currently names; its description does not say that it answers an independent future/past-frame query.

The release notes add editor-context marker access, without specifying the selection rule:

> - **Markers**: New `context.markers` and `context.selected_markers` were added for simplified access to the relevant markers for the editor. (blender/blender@3048200244b8acb082c5399bd8b2aab8170035dc)

Release notes: [Blender 5.3: Python API](https://developer.blender.org/docs/release_notes/5.3/python_api/); Animation; snapshot lines 61–61. Date read: 2026-11-08.

**Search record.** Whole-source final pattern: `marker|camera_bind|camera.*switch|switch.*camera|camera.*cut|cut.*camera` — 119 matching files. Earlier whole-source searches included camera/marker pairs in both orders and camera with first/last/before/after/active. Follow-ups searched camera references in Timeline, Scene Properties, Markers, TimelineMarkers, Scene and Context. Opened Markers, opening definitions and Bind Camera to Markers; TimelineMarker.camera/frame; Scene.camera/timeline_markers; [Camera View](https://docs.blender.org/manual/en/5.3/editors/3dview/navigate/camera_view.html), lines 67–78, Animated Camera Switching; Marker Operators, camera_bind; Python API release notes, Animation. Searched Render Cameras and Scene Properties camera descriptions. Ruled out motion-tracking marker hits: they concern tracking/reconstruction, not timeline camera choice. Ruled out motion-path camera-baking hits: they state a limitation of baking, not the switching algorithm. Ruled out local viewport cameras as a scene camera rule: Camera View says an individual viewport can have a Local Camera. The panel and operator text did not state how to select a camera before, between or after bindings.

**Not covered.** Boundary and interval rules; backwards or out-of-order frame jumps; equal-frame ties; unbound markers; interaction with handlers or drivers; a complete no-move camera-selection algorithm.

**Observed — hand tests 9 and 10, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** Four cameras, each filling its picture with one colour (`CamA` red, `CamB` blue, `CamC` green, `CamD` yellow); marker `mA` at frame 10 bound to `CamA`, `mB` at frame 20 bound to `CamB`; scene camera `CamC`; written at frame 1. A script moved with `frame_set` through frames 1, 10, 15, 20, 30 and back to 1:
```
READ at load, before any call: frame_current 1 scene.camera CamC
READ at load, after evaluated_depsgraph_get: frame_current 1 scene.camera CamC scene_eval.camera CamC
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ after frame_set(10): frame_current 10 scene.camera CamA scene_eval.camera CamA
READ after frame_set(15): frame_current 15 scene.camera CamA scene_eval.camera CamA
READ after frame_set(20): frame_current 20 scene.camera CamB scene_eval.camera CamB
READ after frame_set(30): frame_current 30 scene.camera CamB scene_eval.camera CamB
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
```
Command-line renders of the same file, centre pixel (`mf1back` is frame 30 then frame 1 in one run):
```
COLOUR mf1_0001 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf1_0010 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf1_0015 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf1_0020 size=16x16 centre_rgb=(0, 0, 255)
COLOUR mf1_0030 size=16x16 centre_rgb=(0, 0, 255)
COLOUR mf1back_0030 size=16x16 centre_rgb=(0, 0, 255)
COLOUR mf1back_0001 size=16x16 centre_rgb=(255, 0, 0)
```
A tie (a second marker at frame 10 bound to `CamD`, made after `mA`) gave `CamA` at frame 10, by script and by render; an unbound marker at 15 left `CamA`; with no scene camera, frame 1 gave `CamA`. Every line and the other files are under question 4 of the first pass, `Preparation/Blender Research/Findings/5.3/Reading at a frame.md`.

**Outside the interface.** Inspection of the stored marker data from a background Python script is inferred from the API and shared command-line evidence. The binding operator is described using selected objects/current frame; no background-context guarantee for invoking that operator was found. The supplied switching text does not separately specify background behavior. **Update 2026-11-13 — observed** (hand tests 9 and 10): marker camera switching happened on `frame_set` in a script with no window and in command-line renders; it did not happen on opening the file or on `evaluated_depsgraph_get()` straight after.
**Operating system.** The sources do not specify whether marker-selection boundary behavior is identical on all supported operating systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; whether the same holds on another operating system is still open.

## 5. When a scene is moved to a frame, which settings, of the scene and of other data-blocks, can change, and through what?

**Label: documented** for the mechanisms and examples below; **inferred** for the conclusion that there is no fixed small set of settings that can safely be assumed unchanged. An exhaustive list of changed properties is **not in the sources**. **Update 2026-11-13:** observed by hand tests 1, 11 and 12 on the Creator's Mac — see *Observed* below.

“Settings” can mean stored/editable properties, or include calculated transforms and geometry. I use the first meaning for the main answer and identify the evaluated-state distinction separately.

The documented frame-setting operation updates objects and View Layers:

> frame_set(frame, *, subframe=0.0)
>
> Set scene frame updating all objects and view layers immediately
>
> Parameters:
>
> frame (int) – Frame number to set (in [-1048574, 1048574])
>
> subframe (float) – Subframe time, between 0.0 and 1.0 (in [0, 1], optional)

Python API: [Scene.frame_set](https://docs.blender.org/api/5.3/bpy.types.Scene.html); snapshot lines 584–592. Date read: 2026-11-08.

The sources establish these routes by which values can vary with the frame:

- **Action animation and NLA.** Animatable properties have animation curves; actions hold that animation, and NLA controls how actions contribute. Scene properties are included, not only object movement. See the exact curve and NLA quotes in questions 1 and 3, and the scene-specific quote below.
- **Drivers.** Properties can be calculated from other properties, transformations, expressions and evaluation context. The exact mechanism and input kinds are quoted in question 2.
- **Camera-marker switches.** The scene's active camera can change at camera-bound timeline markers, as quoted in question 4.
- **Frame-change handlers.** Python callbacks can change data and relations before evaluation and run after evaluation. The documented example changes mesh geometry. This is a wider mechanism than changing only keyed settings.

> Controls animation data for scene-level properties, including active Actions
> and their assigned Slot.
>
> See Manually Assigning Actions and Slots for more information.
>
> Scene
>
> Specifies the action and slot where animation data for scene properties is stored/retrieved.

Manual: [Scene Properties](https://docs.blender.org/manual/en/5.3/scene_layout/scene/properties.html); Animation; snapshot lines 319–326. Date read: 2026-11-08.

> Actions are Blender’s container for animation data. For example, when you
> animate the location of an object, that animation is stored in an action rather
> than directly on the object itself. The object then uses the action to get
> animated, much the same way that a mesh uses a material to get shaded. All
> animatable data-blocks
> (objects,
> meshes,
> materials, etc.) are animated this way:
> they don’t store their own animation data, but instead use an action that stores
> the animation data for them.

Manual: [Actions](https://docs.blender.org/manual/en/5.3/animation/actions.html); snapshot lines 3–12. Date read: 2026-11-08.

> is_animatable
>
> Property is animatable through RNA (default False, readonly)
>
> Type:
>
> bool

Python API: [Property](https://docs.blender.org/api/5.3/bpy.types.Property.html); is_animatable; snapshot lines 73–79. Date read: 2026-11-08.

Node trees have their own animation data, and the manual specifically describes animating compositor node parameters:

> class bpy.types.NodeTree(ID)
>
> Node tree consisting of linked nodes used for shading, textures and compositing
>
> animation_data
>
> Animation data for this data-block (readonly)
>
> Type:
>
> AnimData | None

Python API: [NodeTree](https://docs.blender.org/api/5.3/bpy.types.NodeTree.html); animation_data; snapshot lines 38–48. Date read: 2026-11-08.

> Timeline
>
> At the bottom of the workspace is the Timeline.
> It allows scrubbing through frames, controlling playback,
> and animating values that affect the compositing process.
>
> This is especially useful when compositing image sequences or video,
> or when animating node parameters.

Manual: [Compositing Workspace](https://docs.blender.org/manual/en/5.3/compositing/compositing_workspace.html); Timeline; snapshot lines 49–56. Date read: 2026-11-08.

The two frame-change callbacks have different documented positions relative to evaluation:

> bpy.app.handlers.frame_change_pre
>
> Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame. Note that this handler is not to be used as ‘before the frame changes’ event. The dependency graph is not available in this handler, as data and relations may have been altered and the dependency graph has not yet been updated for that. Accepts one argument: the scene data-block

Python API: [Application Handlers](https://docs.blender.org/api/5.3/bpy.app.handlers.html); frame_change_pre; snapshot lines 176–178. Date read: 2026-11-08.

> bpy.app.handlers.frame_change_post
>
> Called after frame change for playback and rendering, after the data has been evaluated for the new frame. Accepts one or two arguments: The scene data-block, and optionally the dependency graph being updated

Python API: [Application Handlers](https://docs.blender.org/api/5.3/bpy.app.handlers.html); frame_change_post; snapshot lines 168–170. Date read: 2026-11-08.

> Below is an example of a mesh that is altered from a handler:
>
> def frame_change_pre(scene):
> # A triangle that shifts in the z direction.
> zshift = scene.frame_current * 0.1
> vertices = [(-1, -1, zshift), (1, -1, zshift), (0, 1, zshift)]
> triangles = [(0, 1, 2)]
>
> object = bpy.data.objects["The Object"]
> object.data.clear_geometry()
> object.data.from_pydata(vertices, [], triangles)

Python API: [Application Handlers](https://docs.blender.org/api/5.3/bpy.app.handlers.html); Note on Altering Data; snapshot lines 60–70. Date read: 2026-11-08.

**Inference:** checking only whether a property has a keyframe does not establish that it stays fixed during a frame change. The sources also permit driver calculations, marker camera changes and handler changes to data or relations. The actual changed properties depend on the file's animation and callback code. `Property.is_animatable` describes whether RNA animation is possible; it is not a list of everything callbacks can change.

For the broader meaning of “settings”, the dependency-graph example expressly includes animation, constraints and modifiers in evaluated state (quoted in question 1). It also says that all ID types have `evaluated_get`, “including materials, node trees, worlds.” [Python API: Depsgraph](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html), Evaluated ID example, line 43; date read 2026-11-08. This does not establish that every such data-block's original editable properties change, or that every data-block is recalculated on every frame move.

**Search record.** Whole-source final pattern: `frame_change|frame_set|is_animatable|animation_data|dependenc|update_tag|handler|callback` — 201 matching files. Follow-ups searched animation/keyframe/driver/frame/evaluation across compositor pages, render layers, ViewLayer, bpy_struct, scene properties, output properties, interface controls and bpy.props. One guessed manual path, `interface/controls/properties.txt`, did not exist; the search continued through the existing controls/buttons files and the API. Opened Scene frame properties/frame_set; Scene Properties Animation; Actions through slots and channels; NodeTree.animation_data/update; Compositing Workspace Timeline; Application Handlers opening examples, data-altering example and frame-change definitions; Property.is_animatable; Depsgraph evaluated-ID example and evaluated scene/View Layer properties; [Internal Data & Python Objects](https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html), Stale Data, lines 91–120; [ViewLayer](https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html), render switches and update methods; bpy.props property signatures; Cycles `properties.py`, render-setting declarations and `update_world`, lines 411–439, 533–569 and 943–955. Searched animation and Python API release notes. Ruled out property type/default listings and inherited keyframe/driver methods as an exhaustive animation-capability inventory: they do not say that every listed property is animatable. Ruled out an animation-data reference as proof that original and evaluated copies always have identical values.

**Not covered.** An exhaustive property-by-property change list; the exact set touched in a particular file; whether all original data-blocks receive evaluated values; every simulation/cache side effect; complete callback ordering beyond the quoted pre/post definitions; a guarantee that every View Layer switch is keyframeable.

**Observed — hand tests 1, 11 and 12, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** Test 1 listed what `is_animatable` says for the scene, `scene.render` and the first View Layer; seventeen settings picked out (`A` animatable, `no` not):
```
5.3 CHECK scene.render.resolution_x=no scene.render.resolution_percentage=no scene.render.filepath=no scene.render.engine=no scene.render.fps=no scene.render.use_motion_blur=no scene.render.motion_blur_shutter=A scene.render.film_transparent=A scene.render.filter_size=A scene.render.use_compositing=no scene.frame_start=no scene.frame_end=no scene.frame_step=no scene.view_layers[0].use=A scene.view_layers[0].samples=A scene.view_layers[0].use_pass_z=A scene.view_layers[0].use_motion_blur=A
```
Test 12: a file with values keyed at frames 1 and 11 on the scene, its View Layers, the compositor, a camera's lens, a material, an object and a second scene, each with a stored value that is neither key, and the frame number set to 6 without moving. Opened with `--disable-depsgraph-on-file-load`, a script added frame-change handlers and called `scene.frame_set(11)`:
```
READ version 5.3.0 mode frameset background True bpy.app.autoexec_fail True active scene Scene
READ at script start STORED | frame_current 6 | shutter 0.33 | filter_size 1.23 | drv_script 7.0 | VL.samples 33 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
HANDLER frame_change_pre scene Scene frame_current 11 stored shutter 0.33 stored Cube x 3.0
HANDLER frame_change_post scene Scene frame_current 11 stored shutter 0.9 stored Cube x 10.0 updates None
HANDLER frame_change_pre scene Scene frame_current 11 stored shutter 0.9 stored Cube x 10.0
HANDLER frame_change_post scene Scene frame_current 11 stored shutter 0.9 stored Cube x 10.0 updates None
READ after frame_set(11) STORED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamB | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark 11.0
READ after frame_set(11), evaluated_depsgraph_get EVALUATED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamB | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark 11.0
```
Test 11, step 1: Blender's ordinary save (`bpy.ops.wm.save_as_mainfile`) of the same scene, stored values before and after:
```
BUILD stored before save: Scene shutter 0.33000001311302185 filter_size 1.2300000190734863 drv_script 7.0 samples 33 use_pass_z True Second.use False Value 0.33000001311302185 lens 45.0 Mat r 0.25 Cube x 3.0 Scene2 shutter 0.7699999809265137 camera CamC frame_current 6 active scene Scene
BUILD stored after bpy.ops.wm.save_as_mainfile: Scene shutter 0.5 filter_size 0.6 drv_script 0.0 VL.samples 15 Value 0.5 lens 50.0 Mat r 0.5 Cube x 5.0 Scene2 shutter 0.77 camera CamC
```
The whole set-up and every line are under question 5 of the first pass, `Preparation/Blender Research/Findings/5.3/Reading at a frame.md`.

**Outside the interface.** The manual documents background scripts and background animation with drivers enabled. No full equivalence guarantee for all callbacks and evaluation contexts was found. There is an explicit render-use difference: `RenderSettings.use_single_layer` says “Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line.” [Python API: RenderSettings](https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html), use_single_layer, line 749; date read 2026-11-08. That describes how a switch is used, not whether its stored value changes. **Update 2026-11-13 — observed** (hand test 12): frame-change handlers added by a script ran on `scene.frame_set` with no window, each twice for the one call.
**Operating system.** No exhaustive cross-platform guarantee about frame-change side effects was found. The sources do not say that arbitrary file-provided callbacks behave identically on every operating system. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; whether the same holds on another operating system is still open.

## 7. Can a scene be brought to a given frame so that its own settings, its View Layers' switches and its compositor take that frame's values, without the objects in the scene being worked out again? If so, by what means, and how do those values compare with a full move to that frame?

**Label: not in the sources** for a settings-only frame move and for equality with a full frame move. The individual APIs below are **documented**. **Update 2026-11-13:** observed by hand test 13 on the Creator's Mac — see *Observed* below.

No documented route satisfying all those conditions was found. This does not prove that one is impossible.

I read “its compositor take that frame's values” as updating animated compositor settings. The other reading is executing the compositor to produce that frame's image or computed socket results. Those are different requests; the latter has an explicit evaluation requirement below.

The ordinary frame move, quoted in question 5, says it updates **all objects and view layers immediately**. Its signature offers a frame and subframe, with no documented object-evaluation switch. Simply assigning the current-frame property is not documented as the requested alternative:

> frame_current
>
> Current frame, to update animation data from Python frame_set() instead (in [-1048574, 1048574], default 1)

Python API: [Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html); frame_current; snapshot lines 170–172. Date read: 2026-11-08.

These other candidates do not document a settings-only move:

- `ID.evaluated_get` returns a result from the last evaluation. It does not take a frame.
- `Context.evaluated_depsgraph_get` accesses animation/modifier results for the current scene and View Layer and can update the graph. It does not promise to avoid object work.
- `ViewLayer.update`, `Depsgraph.update` and `ID.update_tag` concern tagged data and graph updates, not independently specified scene/compositor time with object work excluded.
- `NodeTree.update` is described as updating on editor changes, not as sampling scene time.

> evaluated_get(depsgraph)
>
> Get corresponding evaluated ID from the given dependency graph. Note that this does not ensure the dependency graph is fully evaluated, it just returns the result of the last evaluation.

Python API: [ID.evaluated_get](https://docs.blender.org/api/5.3/bpy.types.ID.html); snapshot lines 324–326. Date read: 2026-11-08.

> evaluated_depsgraph_get()
>
> Get the dependency graph for the current scene and view layer, to access to data-blocks with animation and modifiers applied. If any data-blocks have been edited, the dependency graph will be updated. This invalidates all references to evaluated data-blocks from the dependency graph.

Python API: [Context.evaluated_depsgraph_get](https://docs.blender.org/api/5.3/bpy.types.Context.html); snapshot lines 901–903. Date read: 2026-11-08.

> update()
>
> Update data tagged to be updated from previous access to data or operators

Python API: [ViewLayer.update](https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html); snapshot lines 493–495. Date read: 2026-11-08.

> update()
>
> Re-evaluate any modified data-blocks, for example for animation or modifiers. This invalidates all references to evaluated data-blocks from this dependency graph.

Python API: [Depsgraph.update](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html); snapshot lines 513–515. Date read: 2026-11-08.

> update_tag(*, refresh=set())
>
> Tag the ID to update its display data, e.g. when calling bpy.types.ViewLayer.update
>
> Parameters:
>
> refresh (set[Literal['OBJECT', 'DATA', 'TIME']]) – Type of updates to perform (optional)

Python API: [ID.update_tag](https://docs.blender.org/api/5.3/bpy.types.ID.html); snapshot lines 481–487. Date read: 2026-11-08.

> update()
>
> Update on editor changes

Python API: [NodeTree.update](https://docs.blender.org/api/5.3/bpy.types.NodeTree.html); snapshot lines 267–269. Date read: 2026-11-08.

The background load option addresses file loading, not a selective move to another frame:

> --disable-depsgraph-on-file-load
>
> Background mode: Do not systematically build and evaluate ViewLayers’ dependency graphs
> when loading a blend-file in background mode (-b or -c options).
>
> Scripts requiring evaluated data then need to explicitly ensure that
> an evaluated depsgraph is available
> (e.g. by calling depsgraph = context.evaluated_depsgraph_get()).

Manual: [Command Line Arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html); Other Options; snapshot lines 609–616. Date read: 2026-11-08.

I did not treat that quote as an answer to question 6, which is outside this pass. It was examined only as a possible way of meeting question 7's no-object-work condition.

For the compositor-image reading, the manual states:

> Note
>
> Socket values are only available after the node tree has been evaluated.
> Nodes that do not contribute to the final output are generally not evaluated
> and therefore have no inspection data available.

Manual: [Compositor System](https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html); Socket Inspection; snapshot lines 294–298. Date read: 2026-11-08.

The version's release notes also define the compositor's scope as an effects stack:

> - **Effects Stack**: The compositor now uses an effects stack instead of a single compositor node group. (blender/blender@a45205780d)

Release notes: [Blender 5.3: Compositor](https://developer.blender.org/docs/release_notes/5.3/compositor/); Effects Stack and Viewer; snapshot lines 86–86. Date read: 2026-11-08.

Neither quote supplies a settings-only time update or an equality guarantee. No general comparison with a full frame move can be made from these sources. Driver inputs that depend on evaluated objects, documented in question 2, are an additional reason a proposed shortcut would need checking; this is an inference about a dependency requirement, not proof that a particular shortcut fails.

**Search record.** Whole-source final pattern: `frame_set|frame_current|animation.only|only.animation|partial.*evaluat|evaluat.*partial|skip.*evaluat|evaluat.*skip|without.*(?:object|evaluat)|disable-depsgraph|view_layer_eval|scene_eval` — 39 matching files. Earlier whole-source searches also used update/evaluate with only, skip, without and partial; no-depsgraph/depsgraph-on-load; animation-data/evaluation; and frame/update/evaluation combinations. Follow-ups searched ID update_tag/refresh/TIME; Scene, ViewLayer, Depsgraph, NodeTree, LayerCollection and animation operators; compositor animation and socket inspection; the Cycles Python files for frame/evaluation/update paths. Opened Scene.frame_current/frame_set; ID.evaluated_get/update_tag; Context.evaluated_depsgraph_get; ViewLayer switches/update/update_render_passes; Depsgraph modes, scene_eval/view_layer_eval and update; NodeTree.animation_data/update; Internal Data & Python Objects Stale Data; Command Line Arguments background, Python and graph-load options; Compositor System Socket Inspection; compositor release notes, Effects Stack and Viewer; [Animation Editors](https://docs.blender.org/manual/en/5.3/animation/animation_editors.html), Editors/Play In, lines 222–233; NLA Strip Editing Full/Lower Stack; RenderEngine.frame_set.

Ruled out editor playback update switches: the manual describes which editors update, not a selective scene-data evaluator. Ruled out “partial evaluation” hits in spreadsheet/geometry descriptions as an API for moving scene settings alone. Ruled out Lower Stack tweaking: it changes which NLA tracks contribute. Ruled out `RenderEngine.frame_set`: it is described as scene evaluation for motion blur, with no no-object-work promise. Ruled out the file-load flag: its documented scope is dependency graphs on load. The current/evaluated scene and View Layer properties expose states, not an independently timed settings-only evaluation operation. No image was used to infer that an unshown API does not exist.

**Not covered.** An undocumented or custom selective evaluator; whether assigning frame_current leaves some particular settings stale; whether a given optimization happens to avoid object work; equality of scene values, every View Layer switch and every compositor input after a proposed shortcut; compositor execution without object evaluation.

**Observed — hand test 13, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** The file of question 5's test 12. In a fresh Blender each time: `scene.frame_current = 11`, then one of `bpy.context.view_layer.update()`, `bpy.context.view_layer.depsgraph.update()` or `bpy.context.evaluated_depsgraph_get()`; then the stored values and what the View Layer's dependency graph held; a `depsgraph_update_post` handler listed each updated data-block with its transform and geometry flags. All three printed the same values; `view_layer.update()` on a plain opening:
```
HANDLER depsgraph_update_post scene Scene updates [('Scene', 'Scene', False, False), ('CamA', 'Object', False, True), ('CamAAction', 'Action', False, False), ('CamA', 'Camera', False, False), ('Cube', 'Object', True, True), ('Cube', 'Mesh', False, False), ('Mat', 'Material', False, False), ('MatAction', 'Action', False, False), ('CubeAction', 'Action', False, False), ('SceneAction', 'Action', False, False), ('Compositor', 'CompositorNodeTree', False, False), ('CompositorAction', 'Action', False, False)]
READ called bpy.context.view_layer.update()
READ after the call STORED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
READ after the call, view_layer.depsgraph as it stands EVALUATED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
```
`scene.frame_set(11)` on the same file printed the same values except `camera CamB` (and the `handler_mark` its own handler set). The whole set-up and every line are under question 7 of the first pass, `Preparation/Blender Research/Findings/5.3/Reading at a frame.md`.

**Outside the interface.** The examined graph-load option explicitly applies in background mode. It does not supply the requested frame move. The sources do not establish a background-only settings-update method or equivalence with a full frame move. Compositor evaluation can also depend on context: “For a final render, the output is false.” [Manual: Is Viewport Node](https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/is_viewport.html), line 16; date read 2026-11-08. This is the Is Viewport output, not a guarantee about stored settings. **Update 2026-11-13 — observed** (hand test 13): all three calls ran in a script with no window, on a plain opening and with `--disable-depsgraph-on-file-load`.
**Operating system.** The sources do not specify a platform-dependent settings-only move, or guarantee a common shortcut and equal results on every supported operating system. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; whether the same holds on another operating system is still open.

## Checks a person can carry out in Blender

*Pruned 2026-11-13 after the hand tests, run on the Creator's Mac with Blender 5.3.0: checks 1, 5 and 6 were run in full (as hand tests 2–4, 11–12 and 13), and checks 2, 3 and 4 in part (as tests 6, 8 and 9–10); the results sit as observed findings under the questions above and, in full, in the first pass beside this document. Check 1 used an object's location rather than a scene custom property, and pixel comparison of command-line renders rather than a render-time logger. Check 6 used a depsgraph-update handler's list of updated data-blocks rather than debug output. What is left:*

1. **Question 2: a Transform Channel input from a constrained object.** Only a Single Property input was tried.
2. **Question 3: Animated Strip Time.** Only Animated Influence was tried.
3. **Question 4: the full reverse order, isolated jumps from the saved frame to every target, and a tie made in the other order.** Tried: forward through 1, 10, 15, 20, 30 and back to 1; from a file saved at 25 straight to 1; one tie, with the first-made marker also first by name. Nothing was run with Blender's window for this question.
4. **Everything above, on Linux and on Windows.** All observed results came from macOS on Apple silicon.

2026-11-13 — GPT-6.1-Sol High (Codex) — Reading at a frame, Blender 5.3.

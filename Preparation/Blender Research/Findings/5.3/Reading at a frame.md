# Reading at a frame — findings, Blender 5.3

Answers from Blender 5.3 sources to the Reading at a frame question sheet.

- Sources consulted: the Blender 5.3 manual, Python API documentation and release notes (snapshot taken 2026-11-08), and Cycles' own files (snapshot taken 2026-10-16), all inside `Blender Sources/` (also searched the 4.5–5.2 release notes folders in Q6, finding nothing). Work done on 2026-11-13. Every "date read" below is the snapshot date, as the brief requires.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One note for honesty. When this session started, the tool I run in placed two things into my context without my asking: the project's top-level signpost file and the user's memory index. I did not open either one, and I did not follow the signpost's pointer. Neither contains any claim about how Blender behaves that I used. Inside `Blender Sources/` I also read its own note, `About this folder.md`; I did not follow its links out of the folder. I started no helpers.

Nothing was run. The brief allows "observed" only for things supplied in the folder. The only implementation files there are Cycles' own files. I searched them; nothing in them bears on these questions. So no answer below is labelled observed. *(Citation check note: this paragraph records the initial source-reading pass on 2026-11-13; **observed** hand-test updates were added under questions 1–7 later that day.)*

---

## 1. Can the value an animated setting has at a given frame be learned from an open file without moving the scene to that frame? If so, by what means, and how does the result compare with the value a render uses at that frame?

**Label:** documented (that an F-Curve has `evaluate(frame)` taking `frame (float)`); inferred (passing in-between float frames, how that compares with a render, and windowless API use); not in the sources (whole-scene or whole-data-block evaluation, modifiers and muting under `evaluate()`, driver-vs-keyframe priority, time-remapping formula). *(Citation check narrowed "at any frame" and moved passing in-between frames to inferred.)* **Update 2026-11-13:** observed by hand tests 2, 3, 4 and 5 on the Creator's Mac — see *Observed* below.

**What the sources say.** An F-Curve has a function that returns its value for a given frame parameter. Nothing in its description mentions the scene's current frame.

- "evaluate(frame) — Evaluate F-Curve … frame (float) – Frame, Evaluate F-Curve at given frame (in [-inf, inf]) … Returns: Value, Value of F-Curve specific frame"
  API documentation, https://docs.blender.org/api/5.3/bpy.types.FCurve.html — read 2026-11-08.
- The parameter is `frame (float)` (API documentation, same page). *(Inferred: because `frame` takes a float, non-integer / in-between frames can be passed, though the API page states only `frame (float)` — citation check labelled this inferred and narrowed it to what the page states.)*
- For an animated data-block using an Action and assigned slot, the API offers a helper that returns the first channelbag: "animdata_get_channelbag_for_assigned_slot(anim_data) — Return the first channelbag used in the given anim_data or None if there is no Action + Slot combination defined."
  API documentation, https://docs.blender.org/api/5.3/bpy_extras.anim_utils.html — read 2026-11-08. (Whether an Action can hold more than one channelbag, layer, or strip is not quoted here.)

**How it compares with what a render uses.** The sources never compare the two. What they do say shows several places where the two can part:

- The NLA. "Method used for combining Active Action's result with result of NLA stack" and "Amount the Active Action contributes to the result of the NLA stack."
  API documentation, https://docs.blender.org/api/5.3/bpy.types.AnimData.html — read 2026-11-08. Inferred: one curve read on its own does not include other actions blended in the NLA. (See question 3.)
- Time stretching. The scene has a "Current frame with subframe and time remapping applied" (`frame_current_final`).
  API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html — read 2026-11-08. The manual: "Time Stretching — Use to remap the length of an animation; making it run slower or faster."
  Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/frame_range.html — read 2026-11-08.
- Motion blur. The render engine can "Evaluate scene at a different frame (for motion blur)".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-11-08. And: "Shutter — Time (in frames) between when the shutter starts to open and fully closed."
  Manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/motion_blur.html — read 2026-11-08.
- Muting. "mute — Disable F-Curve evaluation". The F-Curve page, address above.
- Drivers, if the same setting also has a driver. (See question 2.)
- Scripts that run on a frame change. "frame_change_pre — Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations …"
  API documentation, https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-11-08.

The inference, step by step. `evaluate(frame)` gives the value of one F-Curve at the number passed. The sources say a render's evaluated state can also involve NLA blending, time remapping, sub-frames for motion blur, F-Curve muting, drivers and frame-change scripts, and the sources do not say whether `evaluate()` includes F-Curve modifiers or honours `mute`. So `evaluate()` can differ from a render's value whenever any of those apply (and whether they agree even when none apply is not directly stated in the sources, with F-Curve modifiers, muting, and extra Action channelbags/layers/strips left open). To match a scene with time stretching, the number passed would have to be the remapped frame rather than N; the sources do not give the remapping formula.

**Not covered.**
- Whether `evaluate()` includes the curve's F-Curve modifiers. The page says only "Evaluate F-Curve". The manual says "Modifiers are evaluated from top to bottom" (Manual, https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/modifiers.html — read 2026-11-08) but does not tie that to this function.
- Whether `evaluate()` honours `mute`.
- Which drives the final value when a setting has both keyframes and a driver.
- The exact time-remapping formula.
- How an Action's slots, layers, strips, and multiple channelbags are structured when more than the first channelbag exists.
- The sources offer no function that evaluates a whole data-block, or a whole scene, at a chosen frame without moving to it. I found only the per-curve one.

**Where I looked.** Searched all API pages for `evaluate(`: only FCurve, ColorRamp, CurveMapping, CurveProfile and Texture have one. Searched all API pages for every function whose arguments include a frame. The only ones that evaluate anything are `FCurve.evaluate`, `Scene.frame_set` and `RenderEngine.frame_set`. The last two move the scene. Read: AnimData, Action, NlaStrip, Depsgraph, bpy_extras.anim_utils. Searched the 5.3 release notes for "evaluat", "fcurve", "frame_set", "frame change": nothing on this. Searched Cycles' files for `frame_set`, `frame_current`, `evaluate(`: nothing on this.

**Observed — hand tests 2, 3, 4 and 5, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** Every file was made by a script: Workbench, 128 × 128 pixels, one orthographic camera looking along +Y (ortho scale 20), a 2-unit cube named Cube, written standing at frame 1. A separate Blender opened the file for each step. Where a render is compared, a command-line render of the frame (`-f 6`) was compared pixel by pixel with renders of copies of the file whose animation was removed and whose value was fixed by hand; `max_diff` is the largest difference in any colour channel of any pixel (0–255), and `max_diff=0` means the same picture.

*Test 2 — a curve with a modifier.* The cube's X location keyed 0 at frame 1 and 4 at frame 11 (linear), with a Noise modifier on that curve (strength 2, scale 2, phase 3); the stored X then set to 1.0 without moving the frame. With the scene standing at frame 1, a script called the curve's `evaluate(6.0)` and read the frame number; then it moved with `scene.frame_set(6)` and read the stored and the evaluated X. The render of frame 6 was compared with copies at the `evaluate(6.0)` value and at 2.0 (the two keys alone). The machine's words:
```
READ fc.evaluate(6.0) 2.062980890274048
READ frame_current after evaluate 1 stored location.x after evaluate 0.06107
READ after frame_set(6): frame_current 6 frame_current_final 6.0 stored location.x 2.062981
READ after frame_set(6): evaluated location.x 2.062981
READ fc.evaluate(frame_current_final) 2.062980890274048
CMP T2 curve frame 6 vs static at evaluate(6.0) max_diff=0 differing_pixels=0 of 16384
CMP T2 curve frame 6 vs static at 2.0 (the keys alone, no Noise) max_diff=63 differing_pixels=56 of 16384
```
*Test 3 — the same curve muted* (`mute = True`), copies at the `evaluate(6.0)` value and at the stored 1.0:
```
READ fc.evaluate(6.0) 2.062980890274048
READ frame_current after evaluate 1 stored location.x after evaluate 1.0
READ after frame_set(6): frame_current 6 frame_current_final 6.0 stored location.x 1.0
READ after frame_set(6): evaluated location.x 1.0
READ fc.evaluate(frame_current_final) 2.062980890274048
CMP T3 muted frame 6 vs static at evaluate(6.0) max_diff=156 differing_pixels=238 of 16384
CMP T3 muted frame 6 vs static at 1.0 (the stored value) max_diff=0 differing_pixels=0 of 16384
```
*Test 4 — Time Stretching set to Old 2, New 1*, copies at the curve's value at `scene.frame_current_final` (read after `frame_set(6)`) and at `evaluate(6.0)`:
```
READ fc.evaluate(6.0) 2.062980890274048
READ frame_current after evaluate 1 stored location.x after evaluate 0.15678
READ after frame_set(6): frame_current 6 frame_current_final 12.0 stored location.x 4.297497
READ after frame_set(6): evaluated location.x 4.297497
READ fc.evaluate(frame_current_final) 4.297497272491455
CMP T4 stretch frame 6 vs static at evaluate(frame_current_final) max_diff=0 differing_pixels=0 of 16384
CMP T4 stretch frame 6 vs static at evaluate(6.0) max_diff=156 differing_pixels=406 of 16384
```
*Test 5 — keyframes and a driver on the same setting.* The cube's X keyed 0 at frame 1 and 4 at frame 11 (2.0 at frame 6), and also driven by the simple expression `frame * 0.5` (3.0 at frame 6); copies at 2.0 and at 3.0:
```
READ driver location 0 type SCRIPTED expression 'frame * 0.5' is_simple_expression True is_valid True keys [(0.0, 0.0), (1.0, 1.0)] modifiers []
READ fc.evaluate(6.0) 2.0
READ after frame_set(6): frame_current 6 frame_current_final 6.0 stored location.x 3.0
READ after frame_set(6): evaluated location.x 3.0
READ fc.evaluate(frame_current_final) 2.0
CMP T5 frame 6 vs static at 2.0 (the keys) max_diff=156 differing_pixels=224 of 16384
CMP T5 frame 6 vs static at 3.0 (the driver) max_diff=0 differing_pixels=0 of 16384
```
These are single curves on one object in one action; how several actions in the NLA combine is under question 3, drivers under question 2, and the camera a render uses under question 4.

**Outside the interface.** Inferred: `FCurve.evaluate` and `animdata_get_channelbag_for_assigned_slot` are non-operator Python API functions, and the sources say nothing that limits them to the interface, so they appear usable from a script in a Blender with no window (not stated outright). For the comparison with a render, with no window and default options (`-Y`, `--disable-autoexec`), Scripted Expression drivers outside the Simple Expressions subset and start-up scripts are disabled (see question 2), and the sources do not say whether frame-change handlers run on `Scene.frame_set` (see question 5). **Update 2026-11-13 — observed** (hand tests 2–5): `FCurve.evaluate` ran in a script in a Blender with no window and left the frame number where it was (`frame_current after evaluate 1`); the renders compared above were command-line renders (`-b … -f 6`).

**Operating system.** The sources do not say it differs between systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 2. Where a setting is controlled by a driver, can its value at a given frame be learned without moving the scene to that frame? If so, by what means?

**Label:** documented (driver variables, F-Curve axis mapping, and CLI script restrictions); not in the sources (no ready means to evaluate a driver at a chosen frame); inferred (what "current" Driver Value refers to, what a script could rebuild itself, and CLI Simple Expression vs Scripted Expression driver behaviour). *(Citation check added "what 'current' Driver Value refers to" to inferred.)* **Update 2026-11-13:** observed by hand tests 5, 6 and 7 on the Creator's Mac — see *Observed* below.

**What the sources say.**
- The driver's Python object is recorded as having no function that returns its result; its listed members are noted as `expression`, `is_simple_expression`, `is_valid`, `type`, `use_self` and `variables` (exact page text not quoted here; how a driver is paired with its F-Curve on `AnimData` is also not quoted).
  API documentation, https://docs.blender.org/api/5.3/bpy.types.Driver.html — read 2026-11-08.
- A driver's F-Curve maps the driver configuration's output to the target property: "The X axis maps to the output value of the driver configuration … The Y axis shows the value applied to the target property."
  Manual, https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html — read 2026-11-08. *(Citation check narrowed this from "does not run on time" because that page does not state that phrase; the contrast with scene frame is drawn under Inferred below.)*
- A driver can depend on the current frame. "frame is a Simple Expression that corresponds to bpy.context.scene.frame_current."
  Manual, https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html — read 2026-11-08.
- Among a driver's variable types are: "Single Property – Use the value from some RNA property", "Transform Channel – Final transformation value of object or bone", "Context Property – Use the value from some RNA property within the current evaluation context".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.DriverVariable.html — read 2026-11-08.
- The manual's panel shows "Driver Value — The current result of the driver setup." Manual, https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html — read 2026-11-08. *(Inferred: the page does not define "current"; this finding takes it to mean the scene's active frame — citation check labelled this inferred.)*

**Inferred.** Calling `FCurve.evaluate(n)` on a driver's F-Curve does not give the value at frame n. From the manual's words, the number passed stands for the driver's output on the X axis, not for a frame. A script could in principle work the value out by hand: find each input variable's value at frame n (for example with `FCurve.evaluate` on the input's own curve), combine them by the driver's type or expression, then pass that result through the driver's F-Curve with `evaluate`. That works only when every input can itself be found without moving the scene. A "Transform Channel" input is the "Final transformation value", which can include parents and constraints (the drivers panel page above notes that Transform Channel "will take parenting and/or constraints into account as needed"; citation clarified by citation check). The sources give no way to get that at another frame without moving.

**Not covered.** No built-in way to evaluate a driver at a chosen frame. How Python expressions behave when evaluated outside Blender's own driver system (and any driver namespace entries). Whether drivers run before or after keyframe animation.

**Where I looked.** Read: Driver, DriverVariable, DriverTarget, FCurve, AnimData, Depsgraph. Read the manual's driver pages: introduction, drivers panel, workflow examples, troubleshooting. Searched the API for "evaluate" and for every function that takes a frame. Searched the 5.3 release notes for "driver": only GPU-driver hits, not relevant.

**Observed — hand tests 5, 6 and 7, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The files are the ones described under question 1 (a cube, an orthographic camera, Workbench, written at frame 1). A driver made by a script in this version came with its own curve holding two keys, (0, 0) and (1, 1), and no modifier (the `keys` and `modifiers` in the lines below).

*Test 6 — a driver's own curve, and a driver worked out by hand.* The cube's Y location driven by the simple expression `frame * 2`. A scene custom property `src` keyed 1 at frame 1 and 10 at frame 10 (linear); the cube's Z driven by an Averaged Value driver with one Single Property variable reading `src`, whose own curve was cleared and given two linear keys, (0, 0) and (10, 100), with linear extrapolation. With the scene at frame 1, the script read the Y driver curve's `evaluate(5.0)`, then worked Z out for frame 10 by hand (`src`'s curve `evaluate(10.0)`, then the Z driver curve's `evaluate` of that), then moved with `frame_set(5)` and `frame_set(10)`:
```
READ driver location 1 type SCRIPTED expression 'frame * 2' is_simple_expression True is_valid True keys [(0.0, 0.0), (1.0, 1.0)] modifiers []
READ driver location 2 type AVERAGE expression '0.0' is_simple_expression False is_valid True keys [(0.0, 0.0), (10.0, 100.0)] modifiers []
READ scene src curve found by anim_utils.animdata_get_channelbag_for_assigned_slot
READ driver curve Y .evaluate(5.0) 5.0
READ frame_current after evaluate 1
READ hand route for Z at frame 10: src_curve.evaluate(10.0) 10.0 then Z driver curve .evaluate(that) 100.0
READ frame_current after hand route 1 stored location (0.0, 2.0, 10.0)
READ after frame_set(5): stored location (0.0, 10.0, 50.0) evaluated location (0.0, 10.0, 50.0)
READ after frame_set(10): stored location (0.0, 20.0, 100.0) evaluated location (0.0, 20.0, 100.0) stored src 10.0
```
*Test 5 — keys and a driver on the same setting* (set-up under question 1): with the cube's X keyed (2.0 at frame 6) and driven by `frame * 0.5` (3.0 at frame 6), the value after `frame_set(6)` and the render:
```
READ after frame_set(6): evaluated location.x 3.0
CMP T5 frame 6 vs static at 2.0 (the keys) max_diff=156 differing_pixels=224 of 16384
CMP T5 frame 6 vs static at 3.0 (the driver) max_diff=0 differing_pixels=0 of 16384
```
*Test 7 — scripted drivers outside the simple subset, with scripts not allowed and allowed.* The cube's X driven by `-3.0 if bpy.app else 0.0` (it reads an attribute of `bpy`), stored X 3.0; its Z driven by `[-2.0][0]` (a list, no names), stored Z 2.0. The file was opened three ways — with no script flag, with `-Y`, with `-y` — and each time read by a script (at load, after `bpy.context.evaluated_depsgraph_get()`, after `frame_set(5)`) and rendered at frame 5 from the command line; each render was compared with copies at (X 3.0, Z 2.0), (X 0.0, Z −2.0) and (X −3.0, Z −2.0). With no flag:
```
	BPY_driver_exec: restricted access disallows name 'app', enable auto-execution to support
Error in PyDriver: expression failed: -3.0 if bpy.app else 0.0
READ at load: frame_current 1 stored location (0.0, 0.0, -2.0)
READ at load: evaluated location x z 0.0 -2.0 stored location x z after evaluated_depsgraph_get 0.0 -2.0
READ after frame_set(5): stored location x z 0.0 -2.0 evaluated location x z 0.0 -2.0
```
With `-Y` the script printed the same lines as with no flag, word for word. With `-y`:
```
READ at load: frame_current 1 stored location (-3.0, 0.0, -2.0)
READ at load: evaluated location x z -3.0 -2.0 stored location x z after evaluated_depsgraph_get -3.0 -2.0
READ after frame_set(5): stored location x z -3.0 -2.0 evaluated location x z -3.0 -2.0
```
The renders:
```
CMP T7 render (default) frame 5 vs st_ds_stored max_diff=156 differing_pixels=392 of 16384
CMP T7 render (default) frame 5 vs st_ds_blocked max_diff=0 differing_pixels=0 of 16384
CMP T7 render (default) frame 5 vs st_ds_allran max_diff=156 differing_pixels=392 of 16384
CMP T7 render (disable) frame 5 vs st_ds_stored max_diff=156 differing_pixels=392 of 16384
CMP T7 render (disable) frame 5 vs st_ds_blocked max_diff=0 differing_pixels=0 of 16384
CMP T7 render (disable) frame 5 vs st_ds_allran max_diff=156 differing_pixels=392 of 16384
CMP T7 render (enable) frame 5 vs st_ds_stored max_diff=156 differing_pixels=392 of 16384
CMP T7 render (enable) frame 5 vs st_ds_blocked max_diff=156 differing_pixels=392 of 16384
CMP T7 render (enable) frame 5 vs st_ds_allran max_diff=0 differing_pixels=0 of 16384
```
(`default` is no flag, `disable` is `-Y`, `enable` is `-y`; `st_ds_stored` is X 3.0 and Z 2.0, `st_ds_blocked` X 0.0 and Z −2.0, `st_ds_allran` X −3.0 and Z −2.0.)

**Outside the interface.** Relevant line: "By default Blender will restrict execution of Python scripts. If using a Scripted Expression Driver Type that doesn't follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts" (manual, https://docs.blender.org/manual/en/5.3/animation/drivers/troubleshooting.html — read 2026-11-08). "Simple expressions are evaluated even when Python script execution is disabled." (drivers panel page, address above). The command line gives "-Y, --disable-autoexec — Disable automatic Python script execution (Python-drivers & startup scripts), (default)." (manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-11-08). Inferred: with no window and default options (`-Y`), Scripted Expression drivers outside the Simple Expressions subset do not run, while Simple Expressions still evaluate. The sources do not say what value a blocked driver leaves on its setting. **Update 2026-11-13 — observed** (hand test 7): with no window, both with no script flag and with `-Y`, the driver reading `bpy.app` printed `restricted access disallows name 'app'` and `Error in PyDriver`, and its setting read 0.0 — not its stored 3.0 — in the script and in the command-line render; the driver `[-2.0][0]`, also outside the simple subset, gave −2.0 in all three ways. With `-y` both drivers ran. *(Citation check removed "reported `is_valid False`" because Test 7's recorded output does not include `is_valid`.)*

**Operating system.** The sources do not say it differs between systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 3. Where a data-block's animation is made of several actions in the NLA, can the combined value at a given frame be learned without moving the scene to that frame? If so, by what means?

**Label:** documented (NLA blending descriptions and listed strip/AnimData properties); not in the sources (no built-in function to evaluate the NLA stack at a frame, and missing conversion, blend, and extrapolation rules); inferred (that a script could attempt partial reimplementation, and windowless API use). **Update 2026-11-13:** observed by hand test 8 on the Creator's Mac — see *Observed* below.

**What the sources say.**
- The NLA stacks actions: "Higher tracks take precedence over lower ones, although you can also choose to blend them."
  Manual, https://docs.blender.org/manual/en/5.3/editors/nla/introduction.html — read 2026-11-08.
- The manual writes out the blending rules. "Replace — Overwrites the values produced by the lower tracks. If Influence is less than 1, a linear interpolation between the previous and new values is used instead." "Multiply, Subtract, Add … result = mix(previous, previous (+-×) value, influence)". "Combine — Depending on the type of each property, one of the following methods is automatically chosen", followed by separate formulas for axis/angle, quaternion, scale and other channels (formulas not quoted here). Extrapolation: "Hold — The property values at the action's first keyframe also apply to the earlier frames (if the strip is the first in the track)…"; "Nothing — The animated properties return to their default values outside of the strip boundaries."
  Manual, https://docs.blender.org/manual/en/5.3/editors/nla/sidebar.html — read 2026-11-08.
- Strip settings listed on `NlaStrip` include `action_frame_start`, `action_frame_end`, `frame_start`, `frame_end`, `scale`, `repeat`, `blend_in`, `blend_type`, `extrapolation`, `influence`, `mute`, `use_animated_influence`, `use_animated_time` (individual descriptions not quoted here), plus `fcurves` ("F-Curves for controlling the strip's influence and timing") and `modifiers` ("Modifiers affecting all the F-Curves in the referenced Action"). Not every NLA or track input is quoted here (`NlaTrack`, `ActionLayer`, and `ActionStrip` were not opened — see Not covered and Where I looked).
  API documentation, https://docs.blender.org/api/5.3/bpy.types.NlaStrip.html — read 2026-11-08.
- At the data-block level: `use_nla`, `action_blend_type`, `action_influence`, `action_extrapolation` (individual descriptions not quoted here).
  API documentation, https://docs.blender.org/api/5.3/bpy.types.AnimData.html — read 2026-11-08.

**Inferred.** There is no documented function that returns the NLA's combined value at a chosen frame. A script could attempt to rebuild simple cases: for each unmuted track (track muting: Manual, https://docs.blender.org/manual/en/5.3/editors/nla/tracks.html — read 2026-11-08, "Mute ... When unchecked, the track stops contributing to the animation"; citation added by citation check), find the strip covering frame n, turn n into the action's own frame, read each curve with `FCurve.evaluate`, then fold the results together using the manual's formulas and the influences. This is reimplementation, and it cannot be complete from the sources read here, as listed below.

**Not covered.**
- How scene time becomes action time inside a strip, beyond the setting names (`scale`, `repeat`, `action_frame_start`). No formula is given.
- How transition strips and meta strips are evaluated.
- Exactly how blend-in and blend-out shape the influence over time.
- What "default value" means for each setting under "Nothing".
- `NlaStrip.strip_time` reads "Frame of referenced Action to evaluate" (NlaStrip page, address above). The page does not say which scene frame it refers to; I take it as the current one, but that is not stated.
- `bpy.ops.nla.bake` ("Bake all selected objects location/scale/rotation animation to an action"; API documentation, https://docs.blender.org/api/5.3/bpy.ops.nla.html — read 2026-11-08) does not say how it calculates its values, operates on selected objects (`bake_types`: `'POSE'`, `'OBJECT'`; `channel_types`: `'LOCATION'`, `'ROTATION'`, `'SCALE'`, `'BBONE'`, and `'PROPS'` / `"Custom Properties"`), and writes baked animation to an action rather than querying arbitrary settings without modifying the file. *(Citation check added the full URL and read date, and corrected "covers only object and bone transforms" to match `channel_types`, which includes `BBONE` and `PROPS` / Custom Properties.)* I ruled it out as an answer.
- Track-level and action-layer/strip details (`NlaTrack`, `ActionLayer`, `ActionStrip` were not opened).

**Where I looked.** Manual NLA pages: introduction, tracks, strips, sidebar, editing/strip. Manual actions page. API: AnimData, NlaStrip, Action, bpy_extras.anim_utils, bpy.ops.nla (the bake entry). The NlaTrack, ActionLayer and ActionStrip pages I saw only in the file list and did not open. Searched for "evaluat", "combin", "stack". Release notes 5.3: searched "nla": nothing.

**Observed — hand test 8, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with no window (`-b --factory-startup`).** A script made a cube with no active action and two NLA tracks. Lower track: strip `Base` (action `Base`: X 0 at frame 1, 10 at frame 11, linear), starting at scene frame 1, blend Replace. Upper track: strip `Over` (action `Over`: X 0 at frame 1, 20 at frame 11, linear), starting at scene frame 5, blend Add, Animated Influence on with one influence key of 0.5 at frame 1. The stored X was set to 3.0 and the file written standing at frame 1. A second Blender opened it; with the scene at frame 1 its script read `Base`'s curve with `evaluate(10.0)` and `Over`'s with `evaluate(6.0)` (scene frame 10 is `Over`'s own frame 6); then moved with `frame_set(10)` and read the stored and evaluated X and each strip's `strip_time` on the stored strip and on the evaluated copy, and called `AnimData.nla_tweak_strip_time_to_scene(10.0)` with and without `invert=True` (not in tweak mode); moved back to frame 1; then, with the cube selected and active, ran `bpy.ops.nla.bake(frame_start=10, frame_end=10, step=1, only_selected=True, visual_keying=False, use_current_action=False, bake_types={'OBJECT'}, channel_types={'LOCATION'})`. The machine's words:
```
READ version 5.3.0 frame_current 1 stored location.x 0.0 active action None use_nla True
READ track Lower mute False is_solo False strip Base frame_start 1.0 frame_end 11.0 blend_type REPLACE influence 0.0 use_animated_influence False strip_time 0.0
READ track Upper mute False is_solo False strip Over frame_start 5.0 frame_end 15.0 blend_type ADD influence 0.5 use_animated_influence True strip_time 0.0
READ action Base curve location[0].evaluate(10.0) 9.0
READ action Over curve location[0].evaluate(6.0) 10.0
READ frame_current after evaluate 1 stored location.x 0.0
READ after frame_set(10): stored location.x 14.0 evaluated location.x 14.0
READ after frame_set(10): strip Base strip_time 0.0 influence 0.0
READ after frame_set(10): strip Over strip_time 0.0 influence 0.5
READ after frame_set(10): evaluated copy's strip Base strip_time 10.0 influence 1.0
READ after frame_set(10): evaluated copy's strip Over strip_time 6.0 influence 0.5
READ after frame_set(10): AnimData.nla_tweak_strip_time_to_scene(10.0) 10.0 with invert=True 10.0 use_tweak_mode False
READ back at frame_set(1): stored location.x 0.0
READ back at frame 1: strip Base strip_time 0.0
READ back at frame 1: strip Over strip_time 0.0
READ bpy.ops.nla.bake returned {'FINISHED'}
READ after bake: frame_current 1 active action Action
READ baked action location[0] keys [(10.0, 14.0)]
```

**Outside the interface.** Inferred: the `AnimData` and `NlaStrip` data properties and `FCurve.evaluate` are non-operator Python API and the sources say nothing restricting them to the interface (not stated outright). Whether `bpy.ops.nla.bake` works with no window is not stated in the sources. **Update 2026-11-13 — observed** (hand test 8): reading the NLA data, `FCurve.evaluate` and `bpy.ops.nla.bake` all ran in a script with no window; the bake returned `{'FINISHED'}` and left the frame number at 1.

**Operating system.** The sources do not say it differs between systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 4. When timeline markers switch a scene's camera, which camera is active at a frame before the first marker, between two markers, on a marker, and after the last? Can that be learned without moving the scene to the frame?

**Label:** documented (on a marker); inferred (between two markers, after the last); not in the sources (before the first marker). Inferred (learning it without moving). **Update 2026-11-13:** observed by hand tests 9 and 10 on the Creator's Mac — see *Observed* below.

**What the sources say.**
- "camera — Camera that becomes active on this frame" and "frame — The frame on which the timeline marker appears".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.TimelineMarker.html — read 2026-11-08.
- "These markers can be moved to change the frame at which the active camera is changed to the object the marker is bound to."
  Manual, https://docs.blender.org/manual/en/5.3/animation/markers.html — read 2026-11-08.
- "Scene.camera — Active camera, used for rendering the scene".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html — read 2026-11-08.
- "Scene.timeline_markers — Markers used in all timelines for the current scene".
  Same Scene page.

**Each case.**
- **On a marker (when one marker on that frame is bound to a camera):** the marker's camera. Documented: "Camera that becomes active on this frame".
- **Between two markers:** the camera of the earlier marker. Inferred: a camera "becomes active" at its marker's frame. Nothing says it stops being active until another marker makes a different camera active.
- **After the last marker:** the last marker's camera. Same inference.
- **Before the first marker:** not in the sources. Nothing says whether the scene keeps the camera stored in its Camera setting, takes the first marker's camera, or keeps whatever was active last.

Other gaps: two markers on the same frame with different cameras; markers that have no camera.

**Without moving the scene.** Inferred: the markers' frames and cameras can be read from `Scene.timeline_markers`, so a script could work out the camera for any frame by the rule above, except before the first marker and the other gaps above. The sources offer no function that does this. They also do not say whether the switch rewrites the stored `Scene.camera` or only the value used while evaluating.

**Related, not an answer.** "Switching cameras using markers is not supported. It will only bake to the camera that is active when the bake was started." (Manual, https://docs.blender.org/manual/en/5.3/animation/motion_paths.html — read 2026-11-08.) This is about motion paths only.

**Where I looked.** Manual: markers page in full, camera view page ("Animated Camera Switching"; its link "See Animating Cameras" names no page I could find under that title), motion paths, scene properties. API: TimelineMarker, TimelineMarkers, bpy.ops.marker (`camera_bind`: "Bind the selected camera to a marker on the current frame"; API documentation, https://docs.blender.org/api/5.3/bpy.ops.marker.html — read 2026-11-08; URL and date added by citation check), Scene, AnimVizMotionPaths. Searched the manual, API, release notes and Cycles' files for "bind camera", "camera_bind", "switch camera", "camera switch", "marker" near "camera", "active camera". Release notes 5.3 on markers mention only renaming, "Convert to Pose Markers", and `context.markers`, nothing on camera switching.

**Observed — hand tests 9 and 10, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** A script made five files. Each has four cameras, each looking straight down at its own flat-coloured plane that fills its picture — `CamA` red, `CamB` blue, `CamC` green, `CamD` yellow — Workbench with flat light and object colour, 16 × 16 pixels, frames 1–30. `m_f1`: marker `mA` at frame 10 bound to `CamA`, `mB` at frame 20 bound to `CamB`, scene camera `CamC`, written at frame 1. `m_f25`: the same, written with the frame number set to 25 (no move). `m_none`: the same markers, no scene camera. `m_unbound`: as `m_f1` plus a marker `mX` at frame 15 bound to no camera. `m_tie`: as `m_f1` plus a marker `mD` at frame 10 bound to `CamD`, made after `mA`. While building, the script read the scene camera after binding the markers and before setting the scene camera itself:
```
BUILD m_f1 after binding the markers, before setting the scene camera: scene.camera CamA frame_current 1
BUILD m_f1 scene.camera CamC frame_current 1 markers [('mA', 10, 'CamA'), ('mB', 20, 'CamB')]
```
*Test 9 — moving by script.* A second Blender opened each file and printed the stored `scene.camera` at load, then the stored and the evaluated scene's camera after `bpy.context.evaluated_depsgraph_get()`, then both after `frame_set` to each frame in turn:
```
READ at load, before any call: frame_current 1 scene.camera CamC
READ at load, after evaluated_depsgraph_get: frame_current 1 scene.camera CamC scene_eval.camera CamC
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ after frame_set(10): frame_current 10 scene.camera CamA scene_eval.camera CamA
READ after frame_set(15): frame_current 15 scene.camera CamA scene_eval.camera CamA
READ after frame_set(20): frame_current 20 scene.camera CamB scene_eval.camera CamB
READ after frame_set(30): frame_current 30 scene.camera CamB scene_eval.camera CamB
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ at load, before any call: frame_current 25 scene.camera CamC
READ at load, after evaluated_depsgraph_get: frame_current 25 scene.camera CamC scene_eval.camera CamC
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ at load, before any call: frame_current 1 scene.camera None
READ at load, after evaluated_depsgraph_get: frame_current 1 scene.camera None scene_eval.camera None
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ after frame_set(10): frame_current 10 scene.camera CamA scene_eval.camera CamA
READ after frame_set(15): frame_current 15 scene.camera CamA scene_eval.camera CamA
READ after frame_set(20): frame_current 20 scene.camera CamB scene_eval.camera CamB
READ after frame_set(30): frame_current 30 scene.camera CamB scene_eval.camera CamB
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ at load, before any call: frame_current 1 scene.camera CamC
READ at load, after evaluated_depsgraph_get: frame_current 1 scene.camera CamC scene_eval.camera CamC
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ after frame_set(10): frame_current 10 scene.camera CamA scene_eval.camera CamA
READ after frame_set(15): frame_current 15 scene.camera CamA scene_eval.camera CamA
READ after frame_set(20): frame_current 20 scene.camera CamB scene_eval.camera CamB
READ at load, before any call: frame_current 1 scene.camera CamC
READ at load, after evaluated_depsgraph_get: frame_current 1 scene.camera CamC scene_eval.camera CamC
READ after frame_set(1): frame_current 1 scene.camera CamA scene_eval.camera CamA
READ after frame_set(10): frame_current 10 scene.camera CamA scene_eval.camera CamA
READ after frame_set(15): frame_current 15 scene.camera CamA scene_eval.camera CamA
READ after frame_set(20): frame_current 20 scene.camera CamB scene_eval.camera CamB
```
*Test 10 — a command-line render.* `m_f1` rendered at frames 1, 10, 15, 20, 30 in one run (`-f 1 -f 10 -f 15 -f 20 -f 30`); `m_f1` again, frame 30 then frame 1 in one run (`mf1back`); `m_f25` at frame 1; `m_none` at 1, 15, 30; `m_unbound` at 15; `m_tie` at 10 and 15. The centre pixel of each picture (red `CamA`, blue `CamB`, green `CamC`, yellow `CamD`):
```
COLOUR mf1_0001 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf1_0010 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf1_0015 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf1_0020 size=16x16 centre_rgb=(0, 0, 255)
COLOUR mf1_0030 size=16x16 centre_rgb=(0, 0, 255)
COLOUR mf1back_0030 size=16x16 centre_rgb=(0, 0, 255)
COLOUR mf1back_0001 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mf25_0001 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mnone_0001 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mnone_0015 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mnone_0030 size=16x16 centre_rgb=(0, 0, 255)
COLOUR munb_0015 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mtie_0010 size=16x16 centre_rgb=(255, 0, 0)
COLOUR mtie_0015 size=16x16 centre_rgb=(255, 0, 0)
```

**Outside the interface.** Inferred: reading `Scene.timeline_markers` is non-operator Python API and the sources state no interface restriction (not stated outright). The sources do not say whether marker camera switching happens during a render started from the command line, or after `Scene.frame_set` in a script. **Update 2026-11-13 — observed** (hand tests 9 and 10): with no window, `frame_set` in a script and a command-line render both changed the camera at the markers as shown above; opening the file, and `evaluated_depsgraph_get()` straight after opening, did not (`CamC` stayed at frame 1 and at frame 25).

**Operating system.** The sources do not say it differs between systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 5. When a scene is moved to a frame, which settings, of the scene and of other data-blocks, can change, and through what?

**Label:** documented (each mechanism named below); inferred (where View Layer, render, and compositor node animations live); not in the sources (whether the list is complete and whether stored values change). **Update 2026-11-13:** observed by hand tests 1, 9, 11 and 12 on the Creator's Mac — see *Observed* below.

**Through keyframe animation (actions, including the NLA).**
- "Blender lets you animate almost any property, going from the X coordinate of an object to the transparency of a material."
  Manual, https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/introduction.html — read 2026-11-08.
- "All animatable data-blocks (objects, meshes, materials, etc.) are animated this way: they don't store their own animation data, but instead use an action".
  Manual, https://docs.blender.org/manual/en/5.3/animation/actions.html — read 2026-11-08.
- The data-block types that can carry animation data are those listed as having `animation_data`: Annotation, Armature, CacheFile, Camera, Curve, Curves, FreestyleLineStyle, GreasePencil, Key, Lattice, Light, LightProbe, Mask, Material, Mesh, MetaBall, MovieClip, NodeTree, Object, ParticleSettings, PointCloud, Scene, Speaker, Texture, Volume, World. *(Citation check added `PointCloud`, which is listed as `PointCloud.animation_data` on that page.)*
  API documentation, "References" list on https://docs.blender.org/api/5.3/bpy.types.AnimData.html — read 2026-11-08.
- Settings that are not data-blocks themselves are animated from the data-block that holds them: "when keying data paths which contain nested properties this must be done from the ID subclass".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.bpy_struct.html — read 2026-11-08. Inferred: since `Scene` and `NodeTree` appear in the `AnimData` References list above while View Layers, render settings, and individual compositor nodes do not, a View Layer's or render setting's animation lives on the Scene, and compositor node settings on their node tree.

**Through drivers.** "Drivers are a way to control values of properties by means of a function, or a mathematical expression." (Manual, https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html — read 2026-11-08.) They can read the frame directly (`frame`, question 2) or read other animated settings.

**Through timeline markers:** the scene's active camera (question 4).

**Through scripts that run on a frame change.** `frame_change_pre` and `frame_change_post` handlers run custom Python code on frame changes: `frame_change_pre` can "change data and relations" (quoted in question 1), and `frame_change_post` is "Called after frame change for playback and rendering, after the data has been evaluated for the new frame."
API documentation, https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-11-08.

**Through things tied to the scene's clock (evaluation and outputs, not always stored settings).**
- Cache files (cache lookup, not a stored setting): "override_frame — Whether to use a custom frame for looking up data in the cache file, instead of using the current scene frame".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.CacheFile.html — read 2026-11-08.
- Images: "use_auto_refresh — Always refresh image on frame changes"; "frame_current — Current frame number in image sequence or movie".
  API documentation, https://docs.blender.org/api/5.3/bpy.types.ImageUser.html — read 2026-11-08. (The quotes state that image refresh follows frame changes and `frame_current` holds the current frame in an image sequence or movie, without stating outright whether `ImageUser.frame_current` itself updates on a scene frame move.)
- Simulation nodes (simulation evaluation, not a stored setting): "The simulation is tied to the animation system. It will only be evaluated while the animation frame changes".
  Manual, https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/simulation/simulation_zone.html — read 2026-11-08.
- Nodes whose output follows the frame. The Scene Time node "outputs the current time in the scene's animation", and the compositor's Time Curve gives "The Y value of the curve at the current frame".
  Manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/scene_time.html and https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/time_curve.html — read 2026-11-08. These are node outputs, not stored settings.

**What the frame move itself is documented to do.** "frame_set(frame, *, subframe=0.0) — Set scene frame updating all objects and view layers immediately". And for `frame_current`: "Current frame, to update animation data from Python frame_set() instead" (plus `frame_current_final`, quoted in question 1).
API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html — read 2026-11-08.

**Not covered.**
- Whether the list above is complete. The sources do not give one list of everything that follows the frame.
- Whether a frame move writes animated values back into the stored settings that a script reads, or only into the evaluated copies. The sources describe evaluated copies ("ID.evaluated_get") and say "All ID types have evaluated_get(), including materials, node trees, worlds" (API documentation, https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html — read 2026-11-08). Neither page says what happens to the stored values.
- The manual says the property colour cue "currently doesn't work with NLA" (Manual, https://docs.blender.org/manual/en/5.3/animation/introduction.html — read 2026-11-08). That concerns display only.

**Where I looked.** Searched the manual for "animatable", "can be animated", "almost any", "not animatable". Read AnimData references, bpy_struct keyframe functions, Depsgraph, ID, bpy.app.handlers, Scene, CacheFile, ImageUser. Searched the manual and API for "frame change", "frame_change", "current frame". Searched the release notes 5.3 for "frame", "animat", "evaluat".

**Observed — hand tests 1, 9, 11 and 12, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).**

*Test 1 — which settings Blender marks as animatable.* A script listed every editable setting of the scene, of `scene.render` and of the first View Layer, split by the Python API's `is_animatable`. Seventeen of them, picked out (`A` animatable, `no` not):
```
5.3 CHECK scene.render.resolution_x=no scene.render.resolution_percentage=no scene.render.filepath=no scene.render.engine=no scene.render.fps=no scene.render.use_motion_blur=no scene.render.motion_blur_shutter=A scene.render.film_transparent=A scene.render.filter_size=A scene.render.use_compositing=no scene.frame_start=no scene.frame_end=no scene.frame_step=no scene.view_layers[0].use=A scene.view_layers[0].samples=A scene.view_layers[0].use_pass_z=A scene.view_layers[0].use_motion_blur=A
```
The full lists:
```
PROBE scene animatable: audio_volume gravity is_runtime_data playback_loop_mode show_keys_from_selected_only sync_mode tag use_audio use_audio_scrub use_extra_user use_fake_user use_gravity use_nodes
PROBE scene not animatable: allow_preroll audio_distance_model audio_doppler_factor audio_doppler_speed frame_current frame_end frame_float frame_preview_end frame_preview_start frame_start frame_step frame_subframe lock_frame_selection_to_range name show_subframe simulation_frame_end simulation_frame_start time_jump_delta time_jump_unit use_custom_simulation_range use_preview_range use_stamp_note wrap_timeline_navigation
PROBE scene.render animatable: anisotropic_filter compositor_denoise_device compositor_denoise_final_quality compositor_denoise_preview_quality compositor_device compositor_precision dither_intensity film_transparent filter_size hair_subdiv hair_type line_thickness line_thickness_mode metadata_input motion_blur_position motion_blur_shutter preview_pixel_size sequencer_gl_preview simplify_child_particles simplify_child_particles_render simplify_gpencil simplify_gpencil_antialiasing simplify_gpencil_modifier simplify_gpencil_onplay simplify_gpencil_shader_fx simplify_gpencil_tint simplify_gpencil_view_fill simplify_subdivision simplify_subdivision_render simplify_volumes stamp_background stamp_font_size stamp_foreground threads threads_mode use_compositor_frames_cache use_high_quality_normals use_multiview use_overwrite use_persistent_data use_sequencer_override_scene_strip use_simplify use_simplify_normals use_stamp use_stamp_camera use_stamp_date use_stamp_filename use_stamp_frame use_stamp_frame_range use_stamp_hostname use_stamp_labels use_stamp_lens use_stamp_marker use_stamp_memory use_stamp_note use_stamp_render_time use_stamp_scene use_stamp_sequencer_strip use_stamp_time
PROBE scene.render not animatable: border_max_x border_max_y border_min_x border_min_y engine filepath fps fps_base frame_map_new frame_map_old pixel_aspect_x pixel_aspect_y ppm_base ppm_factor resolution_percentage resolution_x resolution_y save_output stamp_note_text use_auto_generate_texture_cache use_border use_compositing use_crop_to_border use_file_extension use_freestyle use_lock_interface use_motion_blur use_placeholder use_render_cache use_sequencer use_single_layer use_texture_cache views_format
PROBE scene.view_layers[0] animatable: active_aov_index active_lightgroup_index pass_alpha_threshold pass_cryptomatte_depth samples use use_ao use_freestyle use_grease_pencil use_motion_blur use_pass_ambient_occlusion use_pass_combined use_pass_cryptomatte_accurate use_pass_cryptomatte_asset use_pass_cryptomatte_material use_pass_cryptomatte_object use_pass_diffuse_color use_pass_diffuse_direct use_pass_diffuse_indirect use_pass_emit use_pass_environment use_pass_glossy_color use_pass_glossy_direct use_pass_glossy_indirect use_pass_grease_pencil use_pass_material_index use_pass_mist use_pass_normal use_pass_object_index use_pass_position use_pass_shadow use_pass_subsurface_color use_pass_subsurface_direct use_pass_subsurface_indirect use_pass_transmission_color use_pass_transmission_direct use_pass_transmission_indirect use_pass_uv use_pass_vector use_pass_z use_sky use_solid use_strand use_volumes
PROBE scene.view_layers[0] not animatable: name
```
An earlier build script that tried to keyframe `scene.render.resolution_x` was stopped with `TypeError: bpy_struct.keyframe_insert() property "render.resolution_x" not animatable`.

*Test 12 — what one `frame_set` changes.* A script made a file whose animated values each had keys at frame 1 and frame 11 (linear) and a stored value that is neither key, written with its frame number set to 6 without moving there: the scene's `render.motion_blur_shutter` (keys 0.1 / 0.9, stored 0.33); `render.filter_size` driven by the simple expression `frame * 0.1` (stored 1.23); a scene custom property `drv_script` driven by `-3.0 if bpy.app else 0.0` (stored 7.0); View Layer `ViewLayer`'s `samples` (10 / 20, stored 33) and `use_pass_z` (False / True, stored True); View Layer `Second`'s `use` (True / False, stored False); a compositor Value node's output (0.1 / 0.9, stored 0.33); camera data `CamA`'s `lens` (30 / 70, stored 45); material `Mat`'s first `diffuse_color` channel (0.1 / 0.9, stored 0.25); object `Cube`'s X (0 / 10, stored 3.0); a second scene `Scene2`'s `motion_blur_shutter` (0.2 / 0.4, stored 0.77); scene camera `CamC` with marker `m1` at frame 1 bound to `CamA` and `m11` at frame 11 bound to `CamB`; `render.filepath` `//render_out/`; a File Output node with folder `//fo_out/`. A second Blender opened it, with no script flag, both plainly and with `--disable-depsgraph-on-file-load`; its script added a `frame_change_pre` handler (prints, and sets the custom property `handler_mark` to the frame number) and a `frame_change_post` handler (prints), called `scene.frame_set(11)`, and printed the stored values and then the evaluated ones. With `--disable-depsgraph-on-file-load`:
```
READ version 5.3.0 mode frameset background True bpy.app.autoexec_fail True active scene Scene
READ at script start STORED | frame_current 6 | shutter 0.33 | filter_size 1.23 | drv_script 7.0 | VL.samples 33 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
HANDLER frame_change_pre scene Scene frame_current 11 stored shutter 0.33 stored Cube x 3.0
HANDLER frame_change_post scene Scene frame_current 11 stored shutter 0.9 stored Cube x 10.0 updates None
HANDLER frame_change_pre scene Scene frame_current 11 stored shutter 0.9 stored Cube x 10.0
HANDLER frame_change_post scene Scene frame_current 11 stored shutter 0.9 stored Cube x 10.0 updates None
READ after frame_set(11) STORED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamB | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark 11.0
HANDLER depsgraph_update_post scene Scene updates [('Scene', 'Scene', False, False)]
READ after frame_set(11), evaluated_depsgraph_get EVALUATED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamB | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark 11.0
```
The plain opening printed the same lines from `HANDLER frame_change_post` on; its first two lines were:
```
READ at script start STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script 0.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
HANDLER frame_change_pre scene Scene frame_current 11 stored shutter 0.5 stored Cube x 5.0
```
*Test 9 — markers rewrite the stored camera* (set-up under question 4): after `frame_set(20)` both the stored `scene.camera` and the evaluated scene's camera read `CamB`.

*Test 11, step 1 — Blender's ordinary save.* The same build script, after writing the file above by `bpy.data.libraries.write`, saved the same scene again with `bpy.ops.wm.save_as_mainfile` and printed the stored values before and after:
```
BUILD stored before save: Scene shutter 0.33000001311302185 filter_size 1.2300000190734863 drv_script 7.0 samples 33 use_pass_z True Second.use False Value 0.33000001311302185 lens 45.0 Mat r 0.25 Cube x 3.0 Scene2 shutter 0.7699999809265137 camera CamC frame_current 6 active scene Scene
BUILD stored after libraries.write: Scene shutter 0.33 filter_size 1.23 drv_script 7.0 Cube x 3.0 camera CamC
BUILD stored after bpy.ops.wm.save_as_mainfile: Scene shutter 0.5 filter_size 0.6 drv_script 0.0 VL.samples 15 Value 0.5 lens 50.0 Mat r 0.5 Cube x 5.0 Scene2 shutter 0.77 camera CamC
```

**Outside the interface.** Inferred: `Scene.frame_set` is non-operator Python API and the sources state no interface restriction (not stated outright). The handler page says frame-change handlers run "for playback and rendering"; it does not say whether they run for `Scene.frame_set` from a script. The sources do not say whether timeline marker camera switching happens in CLI rendering or on `Scene.frame_set` (see question 4), nor whether clock-tied evaluation (`ImageUser.use_auto_refresh`, cache lookups, simulation zones) runs on `Scene.frame_set` with no window. The Python-driver limit in question 2 (`-Y` / `--disable-autoexec`) applies with no window. **Update 2026-11-13 — observed** (hand test 12): frame-change handlers added by a script ran on `scene.frame_set` in a Blender with no window — `frame_change_pre` and `frame_change_post` each printed twice for the one call. Marker camera switching ran on `frame_set` and in command-line renders (question 4).

**Operating system.** The sources do not say it differs between systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 6. When a file is opened with no window and with the option not to build and evaluate dependency graphs on load, what does a script that then reads the file's settings see, compared with a normal opening? Consider scenes, View Layers, compositor nodes, render settings and file paths, after the file's own start-up scripts have run.

**Label:** documented (what the option does and load/autoexec order); inferred (stored vs evaluated behaviour and OS availability); not in the sources (what each named kind of setting then reads as). **Update 2026-11-13:** observed by hand test 11 on the Creator's Mac — see *Observed* below.

**What the sources say.**
- "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options). Scripts requiring evaluated data then need to explicitly ensure that an evaluated depsgraph is available (e.g. by calling depsgraph = context.evaluated_depsgraph_get()). NOTE: this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."
  Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-11-08.
- What "evaluated data" means: "This is needed every time one needs to access state with animation, constraints, and modifiers taken into account."
  API documentation, https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html — read 2026-11-08.
- "evaluated_depsgraph_get() — Get the dependency graph for the current scene and view layer, to access to data-blocks with animation and modifiers applied."
  API documentation, https://docs.blender.org/api/5.3/bpy.types.Context.html — read 2026-11-08.
- "ID.evaluated_get(depsgraph) — … Note that this does not ensure the dependency graph is fully evaluated, it just returns the result of the last evaluation."
  API documentation, https://docs.blender.org/api/5.3/bpy.types.ID.html — read 2026-11-08.
- On a normal opening, "drivers may be evaluated immediately after loading a blend-file".
  API documentation, https://docs.blender.org/api/5.3/bpy.app.html — read 2026-11-08.
- About the file's start-up scripts. A text marked Register "Runs the text data-block as a Python script on loading the blend-file" (Manual, https://docs.blender.org/manual/en/5.3/editors/text_editor.html — read 2026-11-08). Such scripts run "before drivers are evaluated" (bpy.app page above). They run only if allowed: "-Y, --disable-autoexec — Disable automatic Python script execution (Python-drivers & startup scripts), (default)" (arguments page above).

**Inferred.** With the option on, View Layers' dependency graphs are not built and evaluated automatically at load time. Anything that exists only in evaluated data is not ready until a script asks for it. The documented way to get it is `context.evaluated_depsgraph_get()`, which is documented for `"the current scene and view layer"` (not other scenes or view layers). The option speaks only of dependency graphs, so for un-animated, non-driven settings not touched by start-up scripts, it gives no documented reason for stored values to differ. But no source says whether a normal opening writes animated or driven values back into stored settings, nor whether start-up scripts (if allowed to run) run before or after the point where dependency graphs would have been built. So for animated or driven settings, or settings read or modified by a start-up script that inspects evaluated data, the sources do not say whether a stored read matches between the two kinds of opening.

**For each kind of setting the question names** (scenes, View Layers, compositor nodes, render settings, file paths): the sources say nothing specific about any of them under this option. The only general statement is the option's own text above. Nor do the sources compare a normal `-b` opening against a normal opening with a window under this topic.

**Not covered.** Every per-setting comparison. Whether `evaluated_depsgraph_get()` after such a load gives the same result as a normal load (including across multiple scenes and View Layers). Whether start-up scripts run before or after the point where the depsgraph would have been built.

**Where I looked.** Searched the manual, API, release notes 5.3 and Cycles' files for "depsgraph-on", "on file load", "depsgraph_on", "dependency graph", "load_post". Read the command-line arguments page around this option, `bpy.ops.wm.open_mainfile` (it has no such parameter: `load_ui`, `use_scripts` and others, nothing on dependency graphs), `bpy.app` (driver namespace and load order), `bpy.app.handlers` (load_pre, load_post, load_post_fail), Depsgraph, Context, ID, ViewLayer, the gotcha page on internal data. Also checked the 4.5–5.2 release notes folders for the option's name, only to see whether its arrival was announced there; nothing.

**Observed — hand test 11, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), started from the terminal with `--factory-startup`; four openings with no window (`-b`) and one with Blender's window.** The file is the one described under question 5, test 12: every animated value keyed at frames 1 and 11 with a stored value that is neither key, the frame number set to 6 without moving there. It also holds a text `startup.py` with **Register** ticked, which prints the stored values it reads (only stored values, so that it does not itself ask for any working out) and then sets `render.filepath` to `//from_startup/` and the scene's custom property `startup_mark` to 1.0. The file was written by `bpy.data.libraries.write`, because Blender's ordinary save was seen to overwrite stored values with animated ones (question 5, test 11 step 1); on opening, Blender printed `WARNING Library file, loading empty scene`, and the reads below show the file's scenes and settings. A script given with `--python` printed every stored value first thing, then called `bpy.context.evaluated_depsgraph_get()` and printed the evaluated values, then the stored ones again. The values at frame 6 by the keys are: shutter 0.5, filter_size 0.6, View Layer samples 15, Value node 0.5, lens 50, material 0.5, Cube X 5, Scene2 0.3.

Plain opening, no script flag:
```
Error in PyDriver: expression failed: -3.0 if bpy.app else 0.0
READ at script start STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script 0.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
READ after evaluated_depsgraph_get EVALUATED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script 0.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
READ after evaluated_depsgraph_get STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script 0.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
scripts disabled for "…/files/5.3/settings.blend", skipping 'startup.py'
```
With `--disable-depsgraph-on-file-load`, no script flag:
```
Error in PyDriver: expression failed: -3.0 if bpy.app else 0.0
READ at script start STORED | frame_current 6 | shutter 0.33 | filter_size 1.23 | drv_script 7.0 | VL.samples 33 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
READ after evaluated_depsgraph_get EVALUATED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script 0.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
READ after evaluated_depsgraph_get STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script 0.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
scripts disabled for "…/files/5.3/settings.blend", skipping 'startup.py'
```
Plain opening, `-y` (the start-up script allowed):
```
STARTUP ran: frame_current 6 motion_blur_shutter 0.33 filter_size 1.23 drv_script 7.0 ViewLayer.samples 33 Second.use False Value node 0.33 CamA lens 45.0 Cube x 3.0 scene.camera CamC render.filepath //render_out/
READ at script start STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
READ after evaluated_depsgraph_get EVALUATED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
READ after evaluated_depsgraph_get STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
```
With `--disable-depsgraph-on-file-load` and `-y`:
```
STARTUP ran: frame_current 6 motion_blur_shutter 0.33 filter_size 1.23 drv_script 7.0 ViewLayer.samples 33 Second.use False Value node 0.33 CamA lens 45.0 Cube x 3.0 scene.camera CamC render.filepath //render_out/
READ at script start STORED | frame_current 6 | shutter 0.33 | filter_size 1.23 | drv_script 7.0 | VL.samples 33 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
READ after evaluated_depsgraph_get EVALUATED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
READ after evaluated_depsgraph_get STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.33 | CamA lens 45.0 | Mat r 0.25 | Cube x 3.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
```
With Blender's window and `-y`; the script printed at once, then again 3 seconds later, then quit Blender:
```
STARTUP ran: frame_current 6 motion_blur_shutter 0.33 filter_size 1.23 drv_script 7.0 ViewLayer.samples 33 Second.use False Value node 0.33 CamA lens 45.0 Cube x 3.0 scene.camera CamC render.filepath //render_out/
READ at script start STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
READ window, 3 s after start STORED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
READ window, 3 s after start, evaluated_depsgraph_get EVALUATED | frame_current 6 | shutter 0.5 | filter_size 0.6 | drv_script -3.0 | VL.samples 15 | VL.use_pass_z False | Second.use True | camera CamC | Value node 0.5 | CamA lens 50.0 | Mat r 0.5 | Cube x 5.0 | Scene2 shutter 0.77 | render.filepath //from_startup/ | File Output directory //fo_out/ | startup_mark 1.0 | handler_mark None
```
In the file of question 1's test 2 (written with the cube's stored X at 1.0, standing at frame 1), opened plainly with no window, the stored X read at load was:
```
READ at load: frame_current 1 stored location (0.06107, 0.0, 0.0)
```

**Outside the interface.** The option exists only for opening with no window: "in background mode (-b or -c options)". The sources give no equivalent inside the interface. With no window and default options (`-Y`, `--disable-autoexec`), the file's own start-up scripts and Scripted Expression drivers outside Simple Expressions do not run unless execution is enabled. Also, "a normal opening" can mean opening in `-b` without the flag or opening with a window; the sources do not compare them per setting. **Update 2026-11-13 — observed** (hand test 11): with no window, the file's start-up script ran only with `-y`; with no flag Blender printed `scripts disabled for "…/settings.blend", skipping 'startup.py'`. With `-y` its `STARTUP` line printed the values written in the file (0.33, 45.0, 3.0) in all three ways of opening — no window plainly, no window with the option, and with the window — and its `render.filepath` change read back as `//from_startup/` in all three.

**Operating system.** The option is listed under "Other Options" without any system note. Other entries on the same page carry notes such as "(Windows only)". Inferred: it is not limited to one system. The sources do not say so outright. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open. The option was not tried on any other system.

---

## 7. Can a scene be brought to a given frame so that its own settings, its View Layers' switches and its compositor take that frame's values, without the objects in the scene being worked out again? If so, by what means, and how do those values compare with a full move to that frame?

**Label:** documented (what `frame_set`, `frame_current`, `RenderEngine.frame_set`, `ViewLayer.update`, and `Depsgraph.update` say); not in the sources (no documented partial frame move); inferred (a hand-made way and windowless API use). **Update 2026-11-13:** observed by hand tests 12 and 13 on the Creator's Mac — see *Observed* below.

**What the sources say.**
- The only documented Python method on `Scene` to move a scene to a frame and update animation updates all objects and view layers immediately: "frame_set(frame, *, subframe=0.0) — Set scene frame updating all objects and view layers immediately". *(Citation check narrowed "updates everything" to what the quoted docstring states.)*
  API documentation, https://docs.blender.org/api/5.3/bpy.types.Scene.html — read 2026-11-08.
- Setting the frame number alone is described as not updating animation: "frame_current — Current frame, to update animation data from Python frame_set() instead". Same page.
- The render engine's `frame_set(frame, subframe)` — "Evaluate scene at a different frame (for motion blur)" — belongs to a render engine while it renders (API documentation, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-11-08). Nothing says it can be limited to part of a scene.
- `ViewLayer.update()` — "Update data tagged to be updated from previous access to data or operators" (API documentation, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html — read 2026-11-08). `Depsgraph.update()` — "Re-evaluate any modified data-blocks" (API documentation, https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html — read 2026-11-08). Neither is described as able to skip objects.

**Inferred, a hand-made way.** A script could leave the scene's frame where it is, read the values of the Scene's own animation curves and of the compositor node tree's curves at frame n with `FCurve.evaluate(n)`, and write them into the settings by hand (assuming the target properties are writable, and noting that `animdata_get_channelbag_for_assigned_slot` only returns the first channelbag if an Action has more than one). The scene and node trees each have their own `animation_data` (question 5). Since View Layer settings are animated from the Scene (question 5), they would come from the Scene's curves. Objects would not be touched, because no move happened. Compared with a full move, this misses everything listed in question 1 (NLA blending, time remapping, F-Curve modifiers and `mute` if `evaluate()` differs, drivers, frame-change scripts), the camera switching from markers (question 4), compositor `Scene Time` and `Time Curve` nodes that follow the scene's current frame (question 5), and anything a scene setting gets from an object through a driver. This is not a documented feature; it is reimplementation built from documented parts. It also changes the stored settings, which a full move may not do (question 5, not covered).

**Not covered.** Whether Blender has any built-in partial evaluation. Whether setting `frame_current` and then calling `ViewLayer.update()` or `Depsgraph.update()` re-evaluates animation, and if so, of what.

**Where I looked.** API: Scene (frame_set, frame_current, frame_float, frame_subframe, frame_current_final), RenderEngine, ViewLayer, Depsgraph, Context, ID (`update_tag`: "Tag the ID to update its display data"; https://docs.blender.org/api/5.3/bpy.types.ID.html — read 2026-11-08), bpy.ops.anim (`change_frame`: "Interactively change the current frame number"; https://docs.blender.org/api/5.3/bpy.ops.anim.html — read 2026-11-08), bpy.ops.screen (`frame_jump`, `frame_offset`; https://docs.blender.org/api/5.3/bpy.ops.screen.html — read 2026-11-08), the internal-data gotcha page (https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html — read 2026-11-08), RenderSettings (`use_compositor_frames_cache` — "Cache the result of the interactive compositor across frames"; https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-11-08) and the 5.3 API change log (`bpy.types.RenderSettings.use_compositor_frames_cache` listed as Added; https://docs.blender.org/api/5.3/change_log.html — read 2026-11-08). Manual: the arguments page, compositor input nodes. Searched API and manual for "frame_set" and "frame_current", and the anim, screen, scene and wm operator pages for operators whose names hold "update", "refresh", "evaluat" or "frame". Release notes 5.3: compositor (effects stack, playback caching), core, rendering: nothing on partial frame moves. *(Citation check added the missing API URLs and read dates, and moved `use_compositor_frames_cache` from Release notes 5.3: compositor — where only Playback Caching appears — to API RenderSettings and `change_log`.)*

**Observed — hand test 13, with hand test 12 as the full move, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The file is the one described under question 5, test 12 (scene, View Layer, compositor, camera, material and object values keyed at frames 1 and 11, stored values that are neither key, frame number 6). Each way below ran in a fresh Blender: set `scene.frame_current = 11` (no `frame_set`), then call one of `bpy.context.view_layer.update()`, `bpy.context.view_layer.depsgraph.update()` or `bpy.context.evaluated_depsgraph_get()`; then print the stored values, and what the View Layer's dependency graph holds without asking it for anything new. A `depsgraph_update_post` handler printed every data-block each update listed, with whether its transform and its geometry were updated. Plain opening:
```
READ set frame_current = 11; now 11
HANDLER depsgraph_update_post scene Scene updates [('Scene', 'Scene', False, False), ('CamA', 'Object', False, True), ('CamAAction', 'Action', False, False), ('CamA', 'Camera', False, False), ('Cube', 'Object', True, True), ('Cube', 'Mesh', False, False), ('Mat', 'Material', False, False), ('MatAction', 'Action', False, False), ('CubeAction', 'Action', False, False), ('SceneAction', 'Action', False, False), ('Compositor', 'CompositorNodeTree', False, False), ('CompositorAction', 'Action', False, False)]
READ called bpy.context.view_layer.update()
READ after the call STORED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
READ after the call, view_layer.depsgraph as it stands EVALUATED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamC | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark None
HANDLER depsgraph_update_post scene Scene updates [('Scene', 'Scene', False, False), ('CamA', 'Object', False, True), ('CamAAction', 'Action', False, False), ('CamA', 'Camera', False, False), ('Cube', 'Object', True, True), ('Cube', 'Mesh', False, False), ('Mat', 'Material', False, False), ('MatAction', 'Action', False, False), ('CubeAction', 'Action', False, False), ('SceneAction', 'Action', False, False), ('Compositor', 'CompositorNodeTree', False, False), ('CompositorAction', 'Action', False, False)]
READ called bpy.context.view_layer.depsgraph.update()
HANDLER depsgraph_update_post scene Scene updates [('Scene', 'Scene', False, False), ('CamA', 'Object', False, True), ('CamAAction', 'Action', False, False), ('CamA', 'Camera', False, False), ('Cube', 'Object', True, True), ('Cube', 'Mesh', False, False), ('Mat', 'Material', False, False), ('MatAction', 'Action', False, False), ('CubeAction', 'Action', False, False), ('SceneAction', 'Action', False, False), ('Compositor', 'CompositorNodeTree', False, False), ('CompositorAction', 'Action', False, False)]
READ called bpy.context.evaluated_depsgraph_get()
```
(`dgupdate` and `dgget` printed the same two `READ … after the call` lines as `vlupdate`.) With `--disable-depsgraph-on-file-load`, each of the three printed the same two `READ … after the call` lines as above, and this handler line:
```
HANDLER depsgraph_update_post scene Scene updates [('Scene', 'Scene', True, True), ('CamA', 'Object', True, True), ('CamAAction', 'Action', False, False), ('CamA', 'Camera', False, False), ('CamB', 'Object', True, True), ('CamB', 'Camera', False, False), ('CamC', 'Object', True, True), ('CamC', 'Camera', False, False), ('Cube', 'Object', True, True), ('Cube', 'Mesh', True, True), ('Mat', 'Material', False, False), ('MatAction', 'Action', False, False), ('Shader Nodetree', 'ShaderNodeTree', False, False), ('CubeAction', 'Action', False, False), ('Scene Collection', 'Collection', True, True), ('SceneAction', 'Action', False, False), ('Compositor', 'CompositorNodeTree', False, False), ('CompositorAction', 'Action', False, False)]
```
The full move for comparison, `scene.frame_set(11)` on the same file (question 5, test 12):
```
READ after frame_set(11) STORED | frame_current 11 | shutter 0.9 | filter_size 1.1 | drv_script 0.0 | VL.samples 20 | VL.use_pass_z True | Second.use False | camera CamB | Value node 0.9 | CamA lens 70.0 | Mat r 0.9 | Cube x 10.0 | Scene2 shutter 0.77 | render.filepath //render_out/ | File Output directory //fo_out/ | startup_mark None | handler_mark 11.0
```

**Outside the interface.** Inferred: `Scene.frame_set`, `FCurve.evaluate`, `ViewLayer.update` and `Depsgraph.update` are non-operator Python API, and the sources state no interface restriction (not stated outright). `bpy.ops.anim.change_frame` is described as "Interactively", which suggests it is meant for the interface; the sources do not say whether it works with no window. For the comparison with a full move, the `-Y` driver limit (question 2) and unknown frame-change handler behaviour on `Scene.frame_set` (question 5) apply. **Update 2026-11-13 — observed** (hand test 13): all three calls ran in a script with no window, on a plain opening and with `--disable-depsgraph-on-file-load`.

**Operating system.** The sources do not say it differs between systems. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## Things a person with Blender 5.3 open could settle in a few minutes

*Pruned 2026-11-13 after the hand tests: all seven items were run on the Creator's Mac with Blender 5.3.0 (items 1 and 2 as hand tests 2–4, item 3 as test 6, item 4 as tests 9 and 10, item 5 as test 12, item 6 as tests 11 and 12, item 7 as test 13), and their results now sit as observed findings under the questions above. Item 6's render size and output path turned out not to be animatable (question 5, test 1), so animatable settings stood in for them. What is left:*

1. **A driver whose input is a Transform Channel of an object moved by a constraint** (Q2). Only a Single Property input was tried.
2. **Animated Strip Time on an NLA strip** (Q3). Only Animated Influence was tried.
3. **Two markers on one frame made in the other order** (Q4). The one tie tried had the marker made first also named first.
4. **Moving the frame inside Blender's window** (dragging the timeline, playing), and the frame operators `bpy.ops.screen.frame_jump`, `bpy.ops.screen.frame_offset` and `bpy.ops.anim.change_frame` with no window (Q5, Q7).
5. **Things that follow the clock without keys** (Q5): image sequences and movies, cache files, simulation zones, the compositor's Scene Time and Time Curve nodes, motion-blur sub-frames.
6. **What `evaluated_depsgraph_get()` gives for a scene or View Layer that is not the active one** (Q6).
7. **Everything above, on Linux and on Windows.** All the observed results came from macOS on Apple silicon.

---

2026-11-13 — Claude Opus 5.5 (Claude Code) — Reading at a frame, Blender 5.3

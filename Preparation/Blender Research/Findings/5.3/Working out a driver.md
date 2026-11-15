# Working out a driver — findings, Blender 5.3

- **Sources consulted:** Blender 5.3 manual, Blender 5.3 Python API documentation and Blender 5.3 release notes, all from the copies taken on 2026-11-08; Cycles' own files for 5.3 (taken 2026-10-16) were searched but held nothing on drivers. Date read for every quote below: 2026-11-08. The work was done on 2026-11-13.
- **Declaration:** I opened only the question sheet and files inside `Blender Sources/`, and I did not open any other part of this project. I can't sign the standard line exactly as written, though, because three things reached me without my opening them. Before I started, the tool I run in loaded these into my context on its own: (1) the project's `CLAUDE.md`, which points other agents to a planning file I did not open; (2) a short index of the user's saved notes, which names topics but says nothing about how drivers work; (3) the project's recent commit titles. One of those titles says, in a few words, that the project treats drivers, NLA stacks and frame-change scripts in a particular way. I did not use any of this. Every claim below rests on a quoted line from `Blender Sources/`. Inside `Blender Sources/` I also read its own `About this folder.md`. That note links to a method file outside the folder, and I did not open it. I used no helpers.
- **Hand tests (added 2026-11-13):** observed results from hand tests 1–11, run that day on the Creator's Mac by Claude Opus 5.5 (Claude Code), were added under questions 1–6 and the hand-check list at the end was pruned; the tests, their exact commands and every script are in `Audits/Blender Research/Working out a driver/Hand tests — 5.3.md`.

---

## 1. How does each driver type — Averaged Value, Sum Values, Scripted Expression, Minimum Value and Maximum Value — combine its variables into one value?

**Label:** documented (for Average, Sum, Min and Max, for Scripted Expression in general, and for what the two pictures show); inferred (that the question's Minimum Value and Maximum Value mean the manual's Min and Max, and for using the two pictures to check the one-variable rule). The case of zero variables is not in the sources. *(Changed by citation check 2026-11-13: moved equating Minimum Value and Maximum Value with Min and Max from documented to inferred from the docs, because the manual writes "Min and Max" and only hand test 1 observed the UI enum names.)* **Update 2026-11-13:** observed by hand tests 1, 4 and 5 on the Creator's Mac — see *Observed* below.

**Answer.** Averaged Value takes the average of its variables' values. Sum Values takes their sum. Minimum Value takes the lowest and Maximum Value the highest. With one variable, all four give the same result. Scripted Expression runs an expression that can use the variables by name. True or false results become 1.0 or 0.0. Whichever type is used, the single value that comes out then goes through the driver's own F-Curve (see question 3). Blender always works out the four built-in functions natively.

**Quotes.**
- Manual, Drivers Panel — https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html
  - "Built-in functions (Average, Sum, Min and Max)"
  - "The driven property will have the value of the average, sum, lowest or highest (respectively) of the values of the referenced Driver Variables. If there is only one driver variable, these functions will yield the same result."
  - "Custom (Scripted Expression). An arbitrary Python expression that can refer to the Driver Variables by name."
- Manual, Drivers Introduction — https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html
  - "The driver type determines how the variables are used. The type can be: a built-in function: for example, the sum of the variables’ values, or a scripted expression: an arbitrary Python expression that refers to the variables by their names."
  - "This driver configuration outputs a single value which changes when the variables change. This value is then evaluated through the driver function curve to produce the result to be applied to the driven property."
  - "The built-in functions are always evaluated natively."
  - "A driver can have zero, one, or more variables."
- Manual, Workflow & Examples — https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html
  - "The Boolean result is converted to 1.0 when true and 0.0 when false."
- API, Driver — https://docs.blender.org/api/5.3/bpy.types.Driver.html
  - "type — Driver type (default 'AVERAGE')", allowed values "‘AVERAGE’, ‘SUM’, ‘SCRIPTED’, ‘MIN’, ‘MAX’"
  - "expression — Expression to use for Scripted Expression (default “”, never None)"

**Pictures.** `_images/animation_drivers_drivers-panel_panel.png` (on https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html) shows Type "Sum Values" with one variable whose Value is 5.000 and a Driver Value of 5.000. `_images/animation_drivers_workflow-examples_shape-key-method1.png` (on https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html) shows Type "Averaged Value" with one variable reading 1.323 and a Driver Value of 1.323. What is visible in both pictures is documented. Concluding from them that the one-variable rule holds is inferred, because the pictures cover only two of the four types and reading Driver Value as the combined value before the F-Curve (note that `shape-key-method1.png` has a curved F-Curve in question 3) is an unstated step.

**Note on names.** The question's names "Averaged Value" and "Sum Values" appear in the manual's screenshots and walkthroughs. The panel text uses "Average, Sum, Min and Max". The API uses AVERAGE, SUM, SCRIPTED, MIN, MAX. "Minimum Value" and "Maximum Value" do not appear as menu labels anywhere I searched. I took them to mean Min and Max.

**Observed — hand tests 1, 4 and 5, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** Every test file was made by a script and written with `bpy.data.libraries.write` rather than Blender's ordinary save (which the topic *Reading at a frame* saw writing worked-out values over stored ones), so each driven setting went into the file with a stored value no driver would give.

*Hand test 1.* An empty `Src` held custom properties `a` = 1.0, `b` = 2.0, `c` = 6.0. For each of the four built-in types, three empties had their X location driven: with three Single Property variables reading `a`, `b` and `c`; with one reading `c`; and with none (named `T_<TYPE>_3`, `_1`, `_0`). Two more had Scripted Expression with no variables, one with the expression `1.0` and one with an empty expression. Every driven X was stored at 42.0. A second Blender opened the file and a script printed the names Blender's data gives the types, then every driven X after `bpy.context.evaluated_depsgraph_get()` and after `frame_set(2)`, then each driver. The machine's words:
```
READ Driver.type items [('AVERAGE', 'Averaged Value'), ('SUM', 'Sum Values'), ('SCRIPTED', 'Scripted Expression'), ('MIN', 'Minimum Value'), ('MAX', 'Maximum Value')]
```
```
READ after evaluated_depsgraph_get, evaluated X: T_AVERAGE_0=0.0 T_AVERAGE_1=6.0 T_AVERAGE_3=3.0 T_MAX_0=0.0 T_MAX_1=6.0 T_MAX_3=6.0 T_MIN_0=0.0 T_MIN_1=6.0 T_MIN_3=1.0 T_SCRIPTED_0_blank=0.0 T_SCRIPTED_0_one=1.0 T_SUM_0=0.0 T_SUM_1=6.0 T_SUM_3=9.0
```
```
READ after frame_set(2), evaluated X: T_AVERAGE_0=0.0 T_AVERAGE_1=6.0 T_AVERAGE_3=3.0 T_MAX_0=0.0 T_MAX_1=6.0 T_MAX_3=6.0 T_MIN_0=0.0 T_MIN_1=6.0 T_MIN_3=1.0 T_SCRIPTED_0_blank=0.0 T_SCRIPTED_0_one=1.0 T_SUM_0=0.0 T_SUM_1=6.0 T_SUM_3=9.0
```
```
READ T_AVERAGE_0 type AVERAGE expression '0.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables [] keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
READ T_MAX_0 type MAX expression '0.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables [] keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
READ T_MIN_0 type MIN expression '0.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables [] keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
READ T_SCRIPTED_0_blank type SCRIPTED expression '' is_simple_expression False driver.is_valid True fcurve.is_valid True variables [] keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
READ T_SCRIPTED_0_one type SCRIPTED expression '1.0' is_simple_expression True driver.is_valid True fcurve.is_valid True variables [] keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
READ T_SUM_0 type SUM expression '0.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables [] keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
```
The stored X printed the same numbers as the evaluated X at each step (none kept 42.0). The names printed are the names Blender's data holds; the menu itself, in the window, was not looked at.

*Hand test 4 (true and false).* A Scripted Expression driver with one variable `var` (2.5), with Python scripts not allowed (no `-y`):
```
READ E10                'True'                     simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E11                '2 > 1'                    simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E13                'not var'                  simple=True  valid=True  name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
```
The three numbers after `X:` are the driven X at frame 1, after `frame_set(5)` and after `frame_set(5, subframe=0.5)`.

*Hand test 5 (one failing variable among several; a result that is not a number).* An Averaged Value driver with one variable reading a property of 5.0 and one reading a path that does not exist (`["nonexistent"]`, fallback off), and Scripted Expressions `'abc'` and `None`, each on an empty whose X was stored at 3.0. Opened with `-y` (Python allowed), Blender printed, among other lines:
```
Error in PyDriver: expression failed: None
TypeError: must be real number, not NoneType
Error in PyDriver: expression failed: 'abc'
TypeError: must be real number, not str
```
and the script read (stored as opened; evaluated and stored at frame 1; at frame 2; back at frame 1):
```
READ F_none        opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_str         opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_two         opened stored 2.5   | frame 1 evaluated 2.5                  stored 2.5                  | frame 2 evaluated 2.5                  stored 2.5                  | back at frame 1 evaluated 2.5                  stored 2.5                 
READ F_none        type SCRIPTED expression 'None' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_str         type SCRIPTED expression "'abc'" is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_two         type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['a:SINGLE_PROP[id=Src path=\'["good"]\' fallback_used=False]', 'b:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=False]']
```

*What these settle from the list under "Not covered":* zero variables (the four built-in types and the empty expression gave 0.0, the expression `1.0` gave 1.0, and every one stayed `driver.is_valid True`); whether the average is a plain mean (1, 2 and 6 gave 3.0); a result that is not a number (a `TypeError` and 0.0); one failing variable among several (the average came out 2.5 and the driver was marked `driver.is_valid False`). *Still open:* the Type menu as drawn in the window; non-finite inputs; precision.

**Not covered.** The sources don't say what each type gives when a driver has **zero** variables (the manual allows zero). They don't say whether the average is a plain mean or weighted (the plain word "average" is all they give). They don't say how a non-number expression result other than true/false is handled. They don't say what happens if one variable among several fails (see question 5).

**Outside the interface.** The sources do not say whether the four built-in driver types behave the same when Blender is driven from outside its own window (nothing in the sources ties them to the window, and "evaluated natively" only describes how they are worked out rather than whether a window is needed). For Scripted Expressions run from the command line, see question 4 and question 5: anything outside the Simple Expressions subset needs Python script execution to be allowed. **Update 2026-11-13 — observed** (hand tests 1, 4 and 5, Creator's Mac, Blender 5.3.0): every combination above was worked out in a Blender started from the terminal with no window (`-b --factory-startup`) — on opening, after `evaluated_depsgraph_get()` and after `frame_set`. Scripted Expressions that Python was not allowed to run are under questions 4 and 5.

**Operating system.** The sources do not say anything operating-system specific about this. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

**Where I looked.** I read in full every page under `animation/drivers/` (index, introduction, usage, drivers_panel, workflow_examples, troubleshooting), plus `editors/drivers_editor`. In the API: `bpy.types.Driver`, `DriverVariable`, `DriverTarget` and `ChannelDriverVariables`. I searched the whole manual, the API and the 5.3 release notes for "driver", "averaged value", "sum values", "minimum value", "maximum value", "Scripted Expression" and "simple expression". The release notes have no driver entries at all.

---

## 2. What value does each driver variable type — Single Property, Transform Channel, Rotational Difference, Distance and Context Property — take, and where does that value come from? Include the case where the property a variable reads is itself animated or driven, and where it belongs to another scene or another data-block.

**Label:** documented (what each type reads); inferred (another specific scene via Single Property, and dependency-graph update order for a variable's target); not in the sources (whether a variable reads the animated or driven value, and in what order). **Update 2026-11-13:** observed by hand tests 5, 7, 9, 10 and 11 on the Creator's Mac — see *Observed* below.

**Answer, per type.**

- **Single Property.** It reads one property, named by a data-block type and reference plus a path. For transform settings, it gives the value exactly as the interface shows it, before parenting or constraints. The API's ID Type list for DriverTarget includes Scene, Object, Material, Key, Image, World, NodeTree and Action, and the manual mentions Key, Image, Object and Material as examples. Because a Single Property variable names a data-block reference and path, it can read a property on another data-block (documented); reading from another specific named scene is inferred from Scene appearing on the API's ID Type list (though no example or cross-scene rule is stated). A fallback value can be switched on. It is used when the path can't be resolved.
- **Transform Channel.** It reads one transform channel of an object or bone: X/Y/Z location, X/Y/Z/W rotation, X/Y/Z scale, or Average Scale. The API calls it the "Final transformation value". Unlike Single Property, it takes parenting and constraints into account, depending on the chosen space. World Space includes parenting/rest pose and constraints. Transform Space includes neither. Local Space includes constraints but not parenting/rest pose. Average Scale is the cube root of the total change in volume and can be negative. Rotation channels can be read as Auto Euler (the target's own order), explicit Euler orders, Quaternion, or Swing and Twist (the API quote below names Swing and X Twist while eliding the rest of the list, and the manual quote names Y as the twist axis in one mode).
- **Rotational Difference.** The angle between two objects or bones, in radians. The workflow page calls it "the minimum angle between two objects in World Space". Its panel picture shows two object/bone slots and no space setting.
- **Distance.** The distance between two objects or bones. Each of the two targets has its own space setting: World, Transform or Local.
- **Context Property.** It reads a property through a path that starts at "the currently evaluating" scene or view layer, not at a named data-block. The manual calls this a weak reference: linking the animation does not drag the referenced scene or view layer along with it. Its stated purpose is to let a driver in one file pick up, for example, the active camera of whatever scene it ends up linked into. It also has a fallback value.

**Animated or driven inputs.** The sources don't say outright whether a variable reads its target's animated/driven value or its stored value, or that the target is worked out first. Two lines come close. (a) The manual says drivers should use variables "in order for dependencies to be correctly tracked", and that properties reached through `self` "may not be fully tracked". (b) The API calls Transform Channel the "Final transformation value". From (a) I infer that Blender records each variable's target as something the driver depends on. The usual purpose of recording that is so the target is brought up to date before the driver reads it. The sources never say this step, so it stays an inference. For Single Property, "exact value of the UI property ... (before parenting or constraints)" says parenting and constraints are left out. It says nothing about keyframes or drivers on that property. For Transform Channel, calling it the "Final transformation value" states that parenting and constraints can be included, but does not say whether keyframes or drivers on the target are worked out first.

**Quotes.**
- Manual, Drivers Panel — https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html
  - Single Property: "Retrieves the value of an RNA property, specified by a data-block reference and a path string." / "In case of transform properties, this will return the exact value of the UI property, while Transform Channel will take parenting and/or constraints into account as needed." / "ID Type — The ID-block type. For example: Key, Image, Object, Material." / "For example: location.x or location[0] for the X location animation channel value (before parenting or constraints), or [\"prop_name\"] for a custom property." / "Fallback — If enabled, allows specifying a fallback value to use as the variable value if the RNA Path cannot be resolved, instead of causing a driver evaluation failure."
  - Transform Channel: "Retrieves the value of a Transform channel from an object or bone." / "The Average Scale option retrieves the combined scale value, computed as the cubic root of the total change in volume. Unlike X/Y/Z Scale, this value can be negative if the object is flipped by negative scaling." / "For rotation channels, specifies the type of rotation data to use, including different explicit Euler orders. Defaults to using the Euler order of the target." / "Space — World Space, Transform Space, Local Space."
  - Rotational Difference: "Provides the value of the rotational difference between two objects or bones, in radians."
  - Distance: "Provides the value of the distance between two objects or bones." / "Space — World Space, Transform Space, Local Space."
  - Context Property: "Provides the value of a property that is implicitly referring to either a scene or a view layer of the currently evaluating animation system. This is a weak reference which does not lead to the scene or view layer referenced from the driver to be linked when linking animation data." / "It is possible to set up a driver in a character file, and make the driver use the set camera when the character is linked into a set." / "When the object is linked into a different scene, these custom properties may not exist there, and the fallbacks can be used to provide sensible default values."
  - Tip: "Although the values of the x/y/z animation channels for the camera location can be accessed via camera.location[0/1/2], retrieving its world space location and orientation after parenting and constraints currently requires using camera.matrix_world."
  - "Drivers should access object data via Driver Variables, rather than direct references in the Python expression, in order for dependencies to be correctly tracked."
  - "Note that dependencies for properties accessed via self may not be fully tracked."
  - Swing and Twist: "Y Rotation — True angle of the twist rotation." / "W Rotation — True angle of the swing rotation, independent of its direction." / "X Rotation, Z Rotation — Weighted angles that represent the amount of swing around the X/Z axis." / "the swing angles are computed from quaternion components, using \(2 \arccos(w)\) for W and \(2 \arcsin(x)\) etc. for the others."
- Manual, Workflow & Examples — https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html
  - "A rotational difference is the minimum angle between two objects in World Space."
  - "Context Properties of drivers don’t implement this behavior, so if necessary it has to be manually emulated via fallback values and a conditional expression" (note: the quoted sentence does not say what "this behavior" refers to)
- API, DriverVariable — https://docs.blender.org/api/5.3/bpy.types.DriverVariable.html
  - "SINGLE_PROP Single Property – Use the value from some RNA property." / "TRANSFORMS Transform Channel – Final transformation value of object or bone." / "ROTATION_DIFF Rotational Difference – Use the angle between two bones." / "LOC_DIFF Distance – Distance between two bones or objects." / "CONTEXT_PROP Context Property – Use the value from some RNA property within the current evaluation context." / type "(default 'SINGLE_PROP')"
- API, DriverTarget — https://docs.blender.org/api/5.3/bpy.types.DriverTarget.html
  - "ACTIVE_SCENE Active Scene – Currently evaluating scene." / "ACTIVE_VIEW_LAYER Active View Layer – Currently evaluating view layer."
  - "id — ID-block that the specific property used can be found from (id_type property must be set first)" / "id_type — Type of ID-block that can be used (default 'OBJECT')"
  - "WORLD_SPACE World Space – Transforms include effects of parenting/restpose and constraints." / "TRANSFORM_SPACE Transform Space – Transforms don’t include parenting/restpose or constraints." / "LOCAL_SPACE Local Space – Transforms include effects of constraints but not parenting/restpose."
  - "fallback_value — The value to use if the data path cannot be resolved (in [-inf, inf], default 0.0)"
  - transform_type values "‘LOC_X’, ‘LOC_Y’, ‘LOC_Z’, ‘ROT_X’, ‘ROT_Y’, ‘ROT_Z’, ‘ROT_W’, ‘SCALE_X’, ‘SCALE_Y’, ‘SCALE_Z’, ‘SCALE_AVG’"
- API, ID Type Items — https://docs.blender.org/api/5.3/bpy_types_enum_items/id_type_items.html — the list includes, among others, "SCENE:", "OBJECT:", "MATERIAL:", "KEY:", "WORLD:", "IMAGE:", "NODETREE:", "ACTION:".
- API, Driver Target Rotation Mode Items — https://docs.blender.org/api/5.3/bpy_types_enum_items/driver_target_rotation_mode_items.html — "AUTO: Auto Euler. Euler using the rotation order of the target." … "QUATERNION: Quaternion." … "SWING_TWIST_X: Swing and X Twist. Decompose into a swing rotation to aim the X axis, followed by twist around it."
- Manual, Drivers Introduction — https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html — "Variables specify which properties, object transformation channels, or relative distances between objects, are used as inputs by the driver."

**Pictures (documented, as shown).** `animation_drivers_drivers-panel_single-property.png`: fields "Prop" (data-block type plus "Camera"), "Path: location.x", a "Use Fallback" checkbox, "Value: 5.000". `animation_drivers_drivers-panel_rotational-difference.png`: two object slots ("Camera"; "Armature" with "Bone") and no space field, "Value: 0.930 (53.3°)". `animation_drivers_drivers-panel_distance.png`: two object slots ("Camera", "Light"), each with its own "World Space" transform-space field, "Value: 1.000". `animation_drivers_drivers-panel_context-property.png`: "Active Scene", "Path: camera.location.x", Fallback ticked at -1.000, "Value: 5.000". All four pictures are on the Drivers Panel page.

**Ambiguity.** "Belongs to another scene" can be read two ways. (a) A Single Property variable that names a specific Scene data-block, which the sources allow by type list but don't describe further. (b) A Context Property used after linking into a different scene, which the sources do describe: it follows the "currently evaluating" scene. I answered both.

**Observed — hand tests 5, 7, 9, 10 and 11, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** Every test file was made by a script and written with `bpy.data.libraries.write` rather than Blender's ordinary save (which the topic *Reading at a frame* saw writing worked-out values over stored ones), so each driven setting went into the file with a stored value no driver would give.

*Hand test 7 (animated and driven inputs, a chain).* Frames 1–11. `A`: X keyed 2 at frame 1 and 6 at frame 11, linear. `B`: Scripted `v * 2`, `v` a Single Property variable reading `A`'s `location[0]`. `C`: Scripted `v + 1` reading `B`'s `location[0]`. `D`: Averaged Value, a Transform Channel variable on `A`, X Location, World Space. `A2`: Scripted `frame`. `B2`: Averaged Value reading `A2`'s `location[0]`. (`Y`, `L1` and `L2` belong to question 6.) Every X stored at 100.0. A script moved the scene with `frame_set` to 6, 11, 1, 11, 6 and 1 in that order and printed the evaluated and stored X each time; the moves to 11 and to 6 (each made twice), as printed:
```
READ after frame_set(11) | frame_current 11 | evaluated X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(11) | frame_current 11 | evaluated X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0
```
```
READ after frame_set(6) | frame_current 6 | evaluated X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(6) | frame_current 6 | evaluated X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0
```

*Hand test 9 (another scene, another data-block, Context Property).* Two scenes, frames 1–30: `SA` at frame 10 (custom property `tag` 1.0, View Layer samples 11) and `SB` at frame 25 (`tag` 2.0, samples 22, custom property `sp` keyed so that it equals the frame, stored 7.0). `EB`, an empty in `SB` only, X keyed to equal the frame, stored 7.0. `Mat`, a material used by nothing, custom property `mp` keyed to equal the frame (stored 7.0) and `md` driven by `frame` (stored 7.0). `EA`, an empty in `SA` only, had custom properties stored −1.0 and driven by: `v_sbframe` (Single Property, Scene `SB`, `frame_current`), `v_sbsp` (Scene `SB`, `["sp"]`), `v_eb` (Object `EB`, `location[0]`), `v_mat` and `v_matd` (Material `Mat`, `["mp"]` and `["md"]`), `v_ctxframe`, `v_ctxtag` (Context Property, Active Scene, `frame_current` and `["tag"]`), `v_ctxvl` (Context Property, Active View Layer, `samples`), and `v_frame` (Scripted `frame`). `EAB`, an empty linked into both scenes, had the last four. Opened plainly (in `SA`), after `evaluated_depsgraph_get()`:
```
READ after evaluated_depsgraph_get EVALUATED | EA (is the evaluated copy a different datablock: True) v_ctxframe=10.0 v_ctxtag=1.0 v_ctxvl=11.0 v_eb=10.0 v_frame=10.0 v_mat=10.0 v_matd=10.0 v_sbframe=25.0 v_sbsp=7.0
READ after evaluated_depsgraph_get EVALUATED | EAB (is the evaluated copy a different datablock: True) v_ctxframe=10.0 v_ctxtag=1.0 v_ctxvl=11.0 v_frame=10.0
READ after evaluated_depsgraph_get EVALUATED | EB 10.0 (a different datablock from the stored one: True)
READ after evaluated_depsgraph_get EVALUATED | Mat (10.0, 10.0) (a different datablock from the stored one: True)
READ after evaluated_depsgraph_get EVALUATED | SB 7.0 (a different datablock from the stored one: True)
```
Opened with `-S SB`, after `evaluated_depsgraph_get()`:
```
READ after evaluated_depsgraph_get EVALUATED | EA (is the evaluated copy a different datablock: False) v_ctxframe=10.0 v_ctxtag=1.0 v_ctxvl=11.0 v_eb=10.0 v_frame=10.0 v_mat=10.0 v_matd=10.0 v_sbframe=25.0 v_sbsp=7.0
READ after evaluated_depsgraph_get EVALUATED | EAB (is the evaluated copy a different datablock: True) v_ctxframe=25.0 v_ctxtag=2.0 v_ctxvl=22.0 v_frame=25.0
READ after evaluated_depsgraph_get EVALUATED | EB 25.0 (a different datablock from the stored one: True)
READ after evaluated_depsgraph_get EVALUATED | Mat (10.0, 10.0) (a different datablock from the stored one: False)
READ after evaluated_depsgraph_get EVALUATED | SB 25.0 (a different datablock from the stored one: True)
```
`frame_set` to each scene's own frame printed the same values, and neither scene's frame number changed.

*Hand test 10 (Transform Channel, Rotational Difference, Distance).* `Par` at (10, 0, 0); `Ch` parented to it, X keyed 1 at frame 1 and 3 at frame 11, with a Limit Location constraint (maximum X 11.5, World Space), stored X 50.0; `O` at the origin; `R1` with rotation Z keyed 0 to 1.5708 over frames 1–11, `R2` unrotated; an armature with bone `Bone`, pose rotation X keyed 0 to 1.0 and pose location Y keyed 0 to 2 over frames 1–11. An empty `Rd` had custom properties, stored −1.0, driven by Averaged Value with one variable each (names in the output: `tc_` Transform Channel on `Ch` X Location in World, Transform and Local Space; `sp_ch` Single Property `location[0]`; `dist_` Distance `Ch`–`O` with the two spaces named; `rot_obj` Rotational Difference `R1`–`R2`; `bone_rotx_tr` Transform Channel on `Bone`, X Rotation, Transform Space; `bone_locy_world` Y Location, World Space; `rot_bone_obj` Rotational Difference `Bone`–`R2`). After `frame_set(11)` and then `frame_set(6)`:
```
READ after frame_set(11) | frame_current 11
READ    Rd bone_locy_world=2.0 bone_rotx_tr=1.0 dist_tr_loc=3.0 dist_tr_tr=3.0 dist_world=11.5 rot_bone_obj=1.0 rot_obj=1.5708 sp_ch=3.0 tc_local=1.5 tc_transform=3.0 tc_world=11.5
READ    Ch evaluated location X 3.0 matrix_world X 11.5 | stored location X 3.0
READ    R1 evaluated rotation Z 1.5708 | Bone evaluated rotation X 1.0 location Y 2.0 pose matrix translation (0.0, 2.0, 0.0) | angle of Bone's pose matrix to identity (rad) 1.0
READ after frame_set(6) | frame_current 6
READ    Rd bone_locy_world=1.0 bone_rotx_tr=0.5 dist_tr_loc=2.0 dist_tr_tr=2.0 dist_world=11.5 rot_bone_obj=0.5 rot_obj=0.7854 sp_ch=2.0 tc_local=1.5 tc_transform=2.0 tc_world=11.5
READ    Ch evaluated location X 2.0 matrix_world X 11.5 | stored location X 2.0
READ    R1 evaluated rotation Z 0.7854 | Bone evaluated rotation X 0.5 location Y 1.0 pose matrix translation (0.0, 1.0, 0.0) | angle of Bone's pose matrix to identity (rad) 0.5
```

*Hand test 11 (a scene's View Layer samples read from another scene).* Three scenes, each one frame long: `S1` (frame 1, samples stored 16), `S2` (frame 5, stored 3), `S3` (frame 9, stored 4). `S2` and `S3` each had a driver on their own `view_layers["ViewLayer"].samples`: Averaged Value, one Single Property variable, ID type Scene, ID `S1`, path `view_layers["ViewLayer"].samples`. In the keyed files `S1`'s samples were also keyed 10 at frame 1 and 90 at frame 9 (so 50 at frame 5), with the stored value set back to 16. One keyed file was saved the ordinary way with the window's scene set to `S3`, so that `S1` was never the scene being worked out; it opened in `S3`. Opened with `-S S2`:
```
READ version 5.3.0 | file samples_keyed_saved_onS3.blend | context scene S2 | window scenes ['S2']
READ as opened, stored samples: S1=16 S2=16 S3=16
READ after evaluated_depsgraph_get | context scene S2 frame 5 | evaluated samples of S2 = 16 | stored samples: S1=16 S2=16 S3=16
```
A keyed file written with `bpy.data.libraries.write`, which opened in `S1` (opening worked `S1` out and wrote its frame-1 value, 10, into its stored samples), then opened with `-S S3`:
```
READ as opened, stored samples: S1=10 S2=3 S3=4
```
```
READ as opened, stored samples: S1=10 S2=3 S3=10
READ after evaluated_depsgraph_get | context scene S3 frame 9 | evaluated samples of S3 = 10 | stored samples: S1=10 S2=3 S3=10
```

*Hand test 5 (missing targets).* Each on an empty whose X was stored at 3.0, Averaged Value with one variable: Single Property with no data-block (`F_noid`); Transform Channel with no object (`F_tc_noid`) and on an armature with a bone name that does not exist (`F_tc_nobone`); Rotational Difference and Distance with the second object empty (`F_rot_one`, `F_dist_one`). As opened; at frame 1; at frame 2; back at frame 1:
```
READ F_dist_one    opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_noid        opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_rot_one     opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_tc_nobone   opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_tc_noid     opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
```
Each of the five drivers then printed `driver.is_valid False`.

*What these settle from the list under "Not covered":* whether a variable reads its target's animated or driven value on the same frame (it did, in every order of frames, through a chain of two); a target in another scene (an object in `SB` only was read at `SA`'s frame, 10; `SB`'s own keyed `sp` was read as its stored 7.0; `SB`'s `frame_current` as 25; and in test 11 another scene's samples were read as that scene's stored value, not its keyed value at either scene's frame); how Rotational Difference reads a bone (its pose rotation, 1.0 at frame 11); the three spaces (above); a missing target or bone (0.0 and `driver.is_valid False`). *Still open:* a source linked into both scenes (only the reader was linked into both).

**Not covered.**
- Whether a Single Property variable reads a target's keyframed or driven value, or its stored value.
- Whether the target is worked out before the driver reads it.
- What happens when a variable on another scene reads that scene's state at that scene's frame or at the evaluating scene's frame.
- How Rotational Difference picks its rotation for bones, and what "Transform Space"/"Local Space" mean for Distance beyond the API one-liners.
- What a Transform Channel reads when the target object or bone is missing (see question 5).

**Outside the interface.** The Context Property's reference is to "the currently evaluating" scene or view layer, not the one on screen. The workflow page says the dependency graph a driver sees "provides the correct scene and view layer when multiple contexts are evaluated at the same time, such as a render running while another view layer is displayed in a viewport" (https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html). For all five variable types, the sources do not say whether variable reading holds identically when Blender is driven from outside its own window (nothing in the sources ties them to the window). **Update 2026-11-13 — observed** (hand tests 5, 7, 9, 10 and 11, Creator's Mac, Blender 5.3.0): every variable type was read in a Blender with no window. The "currently evaluating" scene and View Layer of a Context Property followed the scene chosen on the command line with `-S` (test 9: `EAB` read `SB`'s `tag`, samples and frame when run with `-S SB`, and `SA`'s otherwise).

**Operating system.** The sources do not say anything operating-system specific about this. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

**Where I looked.** The same driver pages as question 1, read in full. `DriverVariable`, `DriverTarget`, `id_type_items` and `driver_target_rotation_mode_items` in the API. I searched the manual and API for "another scene", "other scene", "different scene", "dependency", "self" and "matrix_world". The glossary entry for Swing and Twist confirms these channels exist and adds nothing on values. `animation/armatures` and `constraints` hold no driver-variable detail.

---

## 3. What does a driver's own F-Curve do to the value its variables combine into? Cover a curve with no keyframes, one with keyframes, and one with modifiers, and say what a newly made driver comes with.

**Label:** documented (the curve's role, the default identity map, that a driver can have a modifier stack with polynomial or noise/cyclic modulation, what Add Driver creates); inferred (applying general F-Curve extrapolation and general F-Curve modifier rules to driver curves, and that a newly made Add Driver passes its variable through unchanged once filled in); not in the sources (what a curve with no keyframes returns, what a driver made from Python comes with on its curve, whether frame-based modifier settings read the combined value or the scene frame, and whether general curve interpolation and extrapolation behave identically on a driver curve). *(Changed by citation check 2026-11-13: removed the unsupported inference that "default" means "no keyframes" and moved what a curve with no keyframes returns to not in the sources.)* **Update 2026-11-13:** observed by hand tests 2 and 3 on the Creator's Mac — see *Observed* below.

**Answer.**
- **Role.** The driver's F-Curve maps the combined value (horizontal axis) to the value given to the driven setting (vertical axis). It does not plot value over time.
- **No keyframes.** The manual says the default curve is an identity map: the combined value passes through unchanged. It does not say whether the default curve has keyframes or what a curve with no keyframes returns (an identity map could be keyed, and hand test 2 observed linear-extrapolated keys `(0, 0)` and `(1, 1)` on `driver_add`, while hand test 3 `C_empty` observed what a curve with no keys returns). *(Changed by citation check 2026-11-13: removed the unsupported claim that inferring "default" means "no keyframes" follows from the identity-map quote.)*
- **With keyframes.** The curve is a hand-shaped function of the combined value. The manual's example: if the driver value is 2.0, the property is 0.5. Shape-key examples key a curve from 0 to 1 against a bone rotation in radians. Before the first key and after the last, F-Curves generally hold the end value (Constant, the default) or continue the end slope (Linear). The manual states this for F-Curves in general, not for driver curves, so applying it here is inferred. The shape-key picture does show the curve flat beyond its last key.
- **With modifiers.** Drivers Introduction states that a driver can use "a modifier stack" and be procedurally modulated with noise or cyclic repetitions. The driver example uses a Generator of type Extended/Expanded Polynomial `y = a + bx` to define a linear map, with the driven Shape Key Value clamped by its own Range. On the general F-Curve Modifiers page (not specific to drivers, so applying these rules to driver curves is inferred), modifiers run top to bottom, Generator replaces the curve unless Additive is set, Influence blends between the original and modified curve, Restrict Frame Range sets the frame range of the modifier's effect, and Limits clamps Y values. (Other modifier types from that page were not quoted here.)
- **A newly made driver.** Add Driver "adds a driver with a single variable (which needs to be filled in)". Per the API, a new driver's type defaults to AVERAGE and a new variable's type defaults to SINGLE_PROP. Combined with the identity default curve, a fresh driver passes its one variable through unchanged once that variable is filled in (inferred by putting those three documented defaults together; before the variable is filled in, it has no target or path — see question 5). The sources don't say whether a newly made driver's curve holds keyframes or a modifier. The manual only calls it "default" and an "identity map".

**Quotes.**
- Manual, Drivers Introduction — https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html
  - "An animation F-Curve that maps the output of the driver configuration to the final value to apply to the driven property."
  - "The X axis maps to the output value of the driver configuration. The units depend on the setup." / "The Y axis shows the value applied to the target property. The units depend on the property."
  - "In the example image, if the driver value is 2.0 the property value will be 0.5."
  - "The default F-Curve is an identity map, i.e. the value produced by the driver configuration is applied to the driven property unchanged. If the driver output value is 2.0, the property will be 2.0."
  - "The driver function can be defined artistically with Bézier curve handles or mathematically with trigonometric functions or polynomial expressions such as \(y = a + bx\). Furthermore, the function can also be procedurally modulated with noise or cyclic repetitions."
  - "they can also combine multiple values using a fixed function or a Python expression and further modulate it with a manually defined curve and/or a modifier stack."
- Manual, Drivers Editor — https://docs.blender.org/manual/en/5.3/editors/drivers_editor.html — "The curve doesn’t represent the property’s value over time, but a mapping from the above intermediate value (X axis) to the final value (Y axis)."
- Manual, Usage — https://docs.blender.org/manual/en/5.3/animation/drivers/usage.html — "This operation adds a driver with a single variable (which needs to be filled in), and displays the Edit Driver popover." / "Many drivers don’t use their F-Curve component, so this reduced interface is sufficient."
- Manual, Workflow & Examples — https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html — "The X axis is usually the frame, but for this driver it represents the rotation value in radians." / "In the Modifiers tab, add a Generator of type Extended Polynomial for both drivers." / "Note that the functions go outside the range [0.0, 1.0] for the shape keys’ Value, but that has no effect because Value is clamped in a Range in the Shape Keys panel."
- Manual, F-Curve Modifiers — https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/modifiers.html — "Modifiers are evaluated from top to bottom." / Generator "Additive — Add the polynomial to the curve rather than replacing it." / "Influence — Lets you blend between the original curve and the modified one." / "Restrict Frame Range … The frame on which the modifier’s effect starts/ends." / Limits "Minimum, Maximum Y — Clamps the curve values, never letting them go below the minimum or above the maximum."
- Manual, Graph Editor channel editing — https://docs.blender.org/manual/en/5.3/editors/graph_editor/channels/editing.html — "Changes how the curve behaves before its first keyframe and after its last keyframe." / "Constant: … Continue in a straight line, keeping the same value as the first/last keyframe. This is the default." / "Linear: … Continue in a straight line, keeping the same slope as on the first/last keyframe."
- API, FCurve — https://docs.blender.org/api/5.3/bpy.types.FCurve.html — "driver — Channel Driver (only set for Driver F-Curves) (readonly)" / "extrapolation — Method used for evaluating value of F-Curve outside first and last keyframes (default 'CONSTANT')" / "modifiers — Modifiers affecting the shape of the F-Curve" / "is_empty — True if the curve contributes no animation due to lack of keyframes or useful modifiers, and should be deleted (default False, readonly)"
- API, Driver — https://docs.blender.org/api/5.3/bpy.types.Driver.html — "type — Driver type (default 'AVERAGE')"
- API, DriverVariable — https://docs.blender.org/api/5.3/bpy.types.DriverVariable.html — "type — Driver variable type (default 'SINGLE_PROP')"
- API, bpy_struct — https://docs.blender.org/api/5.3/bpy.types.bpy_struct.html — "driver_add(path, index=-1, /) Adds driver(s) to the given property" (nothing about the curve's contents).
- API, AnimDataDrivers — https://docs.blender.org/api/5.3/bpy.types.AnimDataDrivers.html — "new(data_path, *, index=0)" returns "Newly Driver F-Curve" (nothing about the curve's contents).

**Pictures (documented, as shown).** `animation_drivers_introduction_fcurve.png` (https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html): a straight line through (0, 0), with the dashed guide at X = 2 meeting Y = 0.5. `animation_drivers_workflow-examples_shape-key-method1.png` (https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html): a Bézier curve with keys at about (0, 0) and (2.0, 1.0). To the right of the last key, the drawn line stays flat at 1.0. `animation_drivers_workflow-examples_chained-shape-keys-driver-setup.png` (https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html; *(changed by citation check 2026-11-13: added the manual page address where this image appears)*): a Modifiers tab with one "Generator", "Expanded Polynomial", "Poly Order: 1", "y = 0.000 + 0.600 x". The panel label in that picture reads "Expanded Polynomial", while the text says "Extended Polynomial".

**Observed — hand tests 2 and 3, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).**

*Hand test 2 (what a new driver comes with).* In an empty file a script gave three empties a driver: on X location by `obj.driver_add("location", 0)`; on X location by `obj.animation_data_create()` then `obj.animation_data.drivers.new("location", index=0)`; and on a custom property `p` by `obj.driver_add('["p"]')`. Each driven value was 42.0 beforehand. It printed each driver as made; the driven value with nothing filled in; then filled in one Single Property variable reading another empty's X (2.5) on each, moved to frame 2 and printed again:
```
NEW N_driver_add as made: keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers [] | variables 0 | type SCRIPTED expression '42.0' is_simple_expression True driver.is_valid True fcurve.is_valid True variables []
NEW N_drivers_new as made: keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers [] | variables 0 | type AVERAGE expression '' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
NEW N_prop as made: keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers [] | variables 0 | type SCRIPTED expression '42.0' is_simple_expression True driver.is_valid True fcurve.is_valid True variables []
NEW N_driver_add nothing filled in: (evaluated, stored) = (42.0, 42.0) | driver.is_valid True
NEW N_drivers_new nothing filled in: (evaluated, stored) = (0.0, 0.0) | driver.is_valid True
NEW N_prop nothing filled in: (evaluated, stored) = (42.0, 42.0) | driver.is_valid True
NEW N_driver_add after filling in one variable reading Src location X 2.5 and frame_set(2): (evaluated, stored) = (42.0, 42.0) | type SCRIPTED expression '42.0' is_simple_expression True driver.is_valid True fcurve.is_valid True variables ["var:SINGLE_PROP[id=Src path='location[0]' fallback_used=False]"] | keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
NEW N_drivers_new after filling in one variable reading Src location X 2.5 and frame_set(2): (evaluated, stored) = (2.5, 2.5) | type AVERAGE expression '' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ["var:SINGLE_PROP[id=Src path='location[0]' fallback_used=False]"] | keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
NEW N_prop after filling in one variable reading Src location X 2.5 and frame_set(2): (evaluated, stored) = (42.0, 42.0) | type SCRIPTED expression '42.0' is_simple_expression True driver.is_valid True fcurve.is_valid True variables ["var:SINGLE_PROP[id=Src path='location[0]' fallback_used=False]"] | keys [(0.0, 0.0), (1.0, 1.0)] interpolation ['BEZIER', 'BEZIER'] extrapolation LINEAR modifiers []
```
The window's Add Driver menu was not tried.

*Hand test 3 (keys, extrapolation, an empty curve, modifiers).* An empty `In` whose X is the input, not animated. Eleven empties, X stored at 42.0, each driven by Averaged Value with one Single Property variable reading `In`'s X; only the driver's own curve differed: `C_default` as `driver_add` made it; `C_empty` with every key and modifier removed; `C_keys_const` keys (0, 0) and (2, 1), linear, Constant extrapolation; `C_keys_lin` the same, Linear extrapolation; `C_gen_alone` no keys and a Generator y = 5 + 0x, Additive off; `C_keys_gen` keys (0, 0), (2, 1) and that Generator; `C_keys_gen_add` the same with Additive on; `C_restrict` keys (0, 0), (10, 10), Linear extrapolation, and that Generator with Restrict Frame Range 2 to 4; `C_stepped` keys (0, 0), (10, 10) and Stepped Interpolation, step 2; `C_cycles` keys (0, 0), (2, 1) and Cycles; `C_noise` keys (0, 0), (10, 10) and Noise (strength 2, scale 1, phase 1). With the scene at frame 1, a script set the input and called `view_layer.update()`, then printed each driven X beside that curve's `evaluate(input)`. Input 3.0 and input 0.25:
```
READ A input 3.0 at frame 1 | frame_current 1 | In X 3.0
READ    C_cycles        evaluated X 0.5                    fcurve.evaluate(3.0) 0.5
READ    C_default       evaluated X 3.0                    fcurve.evaluate(3.0) 3.0
READ    C_empty         evaluated X 3.0                    fcurve.evaluate(3.0) 0.0
READ    C_gen_alone     evaluated X 5.0                    fcurve.evaluate(3.0) 5.0
READ    C_keys_const    evaluated X 1.0                    fcurve.evaluate(3.0) 1.0
READ    C_keys_gen      evaluated X 5.0                    fcurve.evaluate(3.0) 5.0
READ    C_keys_gen_add  evaluated X 6.0                    fcurve.evaluate(3.0) 6.0
READ    C_keys_lin      evaluated X 1.5                    fcurve.evaluate(3.0) 1.5
READ    C_noise         evaluated X 2.702502727508545      fcurve.evaluate(3.0) 2.702502727508545
READ    C_restrict      evaluated X 5.0                    fcurve.evaluate(3.0) 5.0
READ    C_stepped       evaluated X 2.0                    fcurve.evaluate(3.0) 2.0
```
```
READ    C_default       evaluated X 0.2499999850988388     fcurve.evaluate(0.25) 0.2499999850988388
READ    C_empty         evaluated X 0.25                   fcurve.evaluate(0.25) 0.0
READ    C_stepped       evaluated X 0.0                    fcurve.evaluate(0.25) 0.0
```
With the input held at 1.0, `frame_set` to frames 1, 2, 3, 4 and 5 printed, at every frame, the same eleven lines as input 1.0 at frame 1; at frame 5:
```
READ B input 1.0, after frame_set(5) | frame_current 5 | In X 1.0
READ    C_cycles        evaluated X 0.5                    fcurve.evaluate(1.0) 0.5
READ    C_default       evaluated X 1.0                    fcurve.evaluate(1.0) 1.0
READ    C_empty         evaluated X 1.0                    fcurve.evaluate(1.0) 0.0
READ    C_gen_alone     evaluated X 5.0                    fcurve.evaluate(1.0) 5.0
READ    C_keys_const    evaluated X 0.5                    fcurve.evaluate(1.0) 0.5
READ    C_keys_gen      evaluated X 5.0                    fcurve.evaluate(1.0) 5.0
READ    C_keys_gen_add  evaluated X 5.5                    fcurve.evaluate(1.0) 5.5
READ    C_keys_lin      evaluated X 0.5                    fcurve.evaluate(1.0) 0.5
READ    C_noise         evaluated X 0.7025026679039001     fcurve.evaluate(1.0) 0.7025026679039001
READ    C_restrict      evaluated X 1.0                    fcurve.evaluate(1.0) 1.0
READ    C_stepped       evaluated X 0.0                    fcurve.evaluate(1.0) 0.0
```

*What these settle from the list under "Not covered":* what a script-made driver's curve holds (two Bézier keys, (0, 0) and (1, 1), Linear extrapolation, no modifier, by both script routes), and that the two routes make different drivers (`driver_add` a Scripted Expression whose text is the setting's value at that moment, `drivers.new` an Averaged Value); an empty curve (the input passed through unchanged, while `evaluate` on it printed 0.0); extrapolation on a driver curve (Constant held 1.0 at input 3, Linear gave 1.5); Restrict Frame Range, Stepped and Noise followed the driver's input, not the scene frame. *Still open:* what the window's Add Driver, Copy As New Driver and `#frame` make.

**Not covered.**
- Whether "default curve" means a curve with no keyframes, or one that carries something such as a modifier.
- What a driver made from Python (`driver_add`, `drivers.new`) comes with on its curve.
- Whether frame-based modifier settings (such as Restrict Frame Range and time/frame-based modulation) read the combined value or the scene frame on a driver curve.
- Whether interpolation and extrapolation behave on a driver curve exactly as on a time curve.

**Outside the interface.** The sources do not say whether a driver's F-Curve mapping holds identically when Blender is driven from outside its own window (nothing in the sources ties it to the window). **Update 2026-11-13 — observed** (hand tests 2 and 3, Creator's Mac, Blender 5.3.0): drivers made by `driver_add` and `drivers.new`, and every curve shape above, behaved as described under *Observed* in a Blender with no window.

**Operating system.** The sources do not say anything operating-system specific about this. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

**Where I looked.**
- Driver pages, read in full.
- `editors/drivers_editor`, `editors/graph_editor/fcurves/introduction` and `fcurves/modifiers`, read in full.
- `editors/graph_editor/channels/editing` (Extrapolation, Revive), and `graph_editor/introduction` searched for "driver" and "extrapolat".
- API: `FCurve`, `Driver`, `DriverVariable`, `AnimDataDrivers`, `bpy_struct.driver_add`.
- Searched the manual for "identity", "no keyframes", "without keyframes", "no keys" and "Generator". The only driver-related hit for "identity" is the line quoted above.

---

## 4. Which expressions count as Simple Expressions? What may they contain, and how are they worked out?

**Label:** documented (which features count, Python fallback, `is_simple_expression`, `frame` and `bl_math` definitions); inferred (`self` excluded, `bl_math` match, command-line `-Y` blocking non-simple expressions); not in the sources (arithmetic precision, integer vs float division, `int`/`round` return values, error handling, `e`, unary minus/parentheses/`**`/`%`, subframe inclusion, identifier rules beyond starting with a letter, and which frame `frame` uses when interface and evaluation contexts differ). **Update 2026-11-13:** observed by hand tests 4 on the Creator's Mac — see *Observed* below.

**Answer.** A Scripted Expression counts as simple if it uses only the features listed below. Blender then works it out directly, without the full Python interpreter. It is faster, especially with many processor cores, and it runs even when Python script execution is turned off. Anything outside the list is run by Python. That is slower, can be blocked by the script-safety setting, and makes Blender show a "Slow Python expression" warning. A read-only property on the driver says whether its expression qualifies.

Allowed contents, exactly as listed:
- Variable names: ASCII characters only.
- Literals: floating-point and decimal integer.
- Globals: `frame`.
- Constants: `pi`, `True`, `False`.
- Operators: `+ - * /`, `== != < <= > >=`, `and`, `or`, `not`, and the conditional (ternary) `if`.
- Standard functions: `min max radians degrees abs fabs floor ceil trunc round int sin cos tan asin acos atan atan2 exp log sqrt pow fmod`.
- Blender-provided functions: `lerp clamp smoothstep`.

Stated exclusions: custom functions added to the driver namespace, and anything using the `depsgraph` variable. `self` (with Use Self on) is not on the list, so an expression using it is not simple (inferred from its absence). `frame` "corresponds to `bpy.context.scene.frame_current`". `lerp`, `clamp` and `smoothstep` are documented in `bl_math`. The sources don't say that direct Simple Expressions evaluation gives exactly the same numbers as `bl_math` (inferred only by sharing the name).

**Quotes.**
- Manual, Drivers Panel — https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html
  - "Blender can evaluate a useful subset of Python driver expressions directly, which significantly improves performance, especially on multi-core systems. To take advantage of this, the driver expression must only use the following features:"
  - "Variable Names — Use only ASCII characters." / "Literals — Floating-point and decimal integer." / "Globals — frame" / "Constants — pi, True, False" / "Operators — +, -, *, /, ==, !=, <, <=, >, >=, and, or, not, conditional operator/ ternary if" / "Standard Functions — min, max, radians, degrees, abs, fabs, floor, ceil, trunc, round, int, sin, cos, tan, asin, acos, atan, atan2, exp, log, sqrt, pow, fmod" / "Blender Provided Functions — lerp, clamp, smoothstep"
  - "Simple expressions are evaluated even when Python script execution is disabled."
  - "When an expression outside of this subset is used, Blender displays a “Slow Python expression” warning."
  - "For performance reasons it is best to use the Simple Expressions subset as much as possible."
  - Use Self: "If this option is enabled, the variable self can be used for drivers to reference their own data."
- Manual, Drivers Introduction — https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html — "When a driver uses a Scripted Expression, Blender can evaluate it without using the fully featured Python interpreter if it is simple enough." / "When the expression is not simple, it will be evaluated using Python. As a consequence, the driver will be slower and there is a security risk if the author of the Python code is unknown."
- Manual, Workflow & Examples — https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html — "frame is a Simple Expression that corresponds to bpy.context.scene.frame_current." / "Since Simple Expressions cannot access custom functions, using them only makes sense for complex computations." / "Expressions using depsgraph require Python evaluation and are not part of the Simple Expressions subset." / "(conditions are Simple Expressions)"
- Manual, Troubleshooting — https://docs.blender.org/manual/en/5.3/animation/drivers/troubleshooting.html — "If using a Scripted Expression Driver Type that doesn’t follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files."
- API, Driver — https://docs.blender.org/api/5.3/bpy.types.Driver.html — "is_simple_expression — The scripted expression can be evaluated without using the full Python interpreter (default False, readonly)"
- API, bl_math — https://docs.blender.org/api/5.3/bl_math.html — "bl_math.clamp(value, min=0, max=1) Clamps the float value between minimum and maximum." / "bl_math.lerp(from_value, to_value, factor) Linearly interpolate between two float values based on factor." / "bl_math.smoothstep(from_value, to_value, value) Performs smooth interpolation between 0 and 1 as value changes between from and to values. Outside the range the function returns the same value as the nearest edge."

**Observed — hand test 4, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** A script made one empty per expression, its X stored at 42.0 and driven by a Scripted Expression with one Single Property variable named `var` reading a custom property of 2.5; `E_self` had Use Self on and scale X 4.0; `E_name_accent` and `E_name_underscore` used a variable named `vé` and `_u` as the whole expression. The file was written with `bpy.data.libraries.write`. A second Blender opened it with no flag — so Python scripts were not allowed — and a script printed, for each, `is_simple_expression`, `driver.is_valid`, the variable's name and `is_name_valid`, and the driven X at frame 1, after `frame_set(5)` and after `frame_set(5, subframe=0.5)`. Blender's messages and the script's lines:
```
00:00.670  anim.fcurve      | ERROR Division by Zero in Driver: '1 / 0'
00:00.670  anim.fcurve      | ERROR Math Domain Error in Driver: 'sqrt(-1)'
00:00.673  anim.fcurve      | ERROR Division by Zero in Driver: 'log(0)'
00:00.673  anim.fcurve      | ERROR Math Domain Error in Driver: 'asin(2)'
	BPY_driver_exec: restricted access disallows name 'myfn', enable auto-execution to support
Error in PyDriver: expression failed: myfn(var)
For target: (type=Object, name="E39", property=location, property_index=0)
	BPY_driver_exec: restricted access disallows name 'scale', enable auto-execution to support
Error in PyDriver: expression failed: self.scale[0]
For target: (type=Object, name="E_self", property=location, property_index=0)
READ version 5.3.0 bpy.app.autoexec False autoexec_fail True autoexec_fail_message "Driver 'self.scale[0]'"
READ step: as opened, after evaluated_depsgraph_get | frame_current 1 frame_subframe 0.0 frame_current_final 1.0
READ step: after frame_set(5) | frame_current 5 frame_subframe 0.0 frame_current_final 5.0
READ step: after frame_set(5, subframe=0.5) | frame_current 5 frame_subframe 0.5 frame_current_final 5.5
READ E01                'var'                      simple=True  valid=True  name='var' name_valid=True  X: 2.5 | 2.5 | 2.5
READ E02                '-var'                     simple=True  valid=True  name='var' name_valid=True  X: -2.5 | -2.5 | -2.5
READ E03                '(var)'                    simple=True  valid=True  name='var' name_valid=True  X: 2.5 | 2.5 | 2.5
READ E04                'var ** 2'                 simple=False valid=True  name='var' name_valid=True  X: 6.25 | 6.25 | 6.25
READ E05                'var % 2'                  simple=False valid=True  name='var' name_valid=True  X: 0.4999999403953552 | 0.4999999403953552 | 0.4999999403953552
READ E06                'pow(var, 2)'              simple=True  valid=True  name='var' name_valid=True  X: 6.25 | 6.25 | 6.25
READ E07                'fmod(var, 2)'             simple=True  valid=True  name='var' name_valid=True  X: 0.4999999403953552 | 0.4999999403953552 | 0.4999999403953552
READ E08                'e'                        simple=False valid=True  name='var' name_valid=True  X: 2.7182817459106445 | 2.7182817459106445 | 2.7182817459106445
READ E09                'pi'                       simple=True  valid=True  name='var' name_valid=True  X: 3.1415927410125732 | 3.1415927410125732 | 3.1415927410125732
READ E10                'True'                     simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E11                '2 > 1'                    simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E12                'var if var > 1 else 0'    simple=True  valid=True  name='var' name_valid=True  X: 2.5 | 2.5 | 2.5
READ E13                'not var'                  simple=True  valid=True  name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E14                '3 / 2'                    simple=True  valid=True  name='var' name_valid=True  X: 1.5 | 1.5 | 1.5
READ E15                '7 // 2'                   simple=False valid=True  name='var' name_valid=True  X: 3.0 | 3.0 | 3.0
READ E16                'int(1.9)'                 simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E17                'int(-1.9)'                simple=True  valid=True  name='var' name_valid=True  X: -1.0 | -1.0 | -1.0
READ E18                'round(2.5)'               simple=True  valid=True  name='var' name_valid=True  X: 3.0 | 3.0 | 3.0
READ E19                'round(3.5)'               simple=True  valid=True  name='var' name_valid=True  X: 4.0 | 4.0 | 4.0
READ E20                'trunc(-1.5)'              simple=True  valid=True  name='var' name_valid=True  X: -1.0 | -1.0 | -1.0
READ E21                'floor(-1.5)'              simple=True  valid=True  name='var' name_valid=True  X: -2.0 | -2.0 | -2.0
READ E22                'clamp(5)'                 simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E23                'clamp(var, 0, 1)'         simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E24                'lerp(0, 10, 0.25)'        simple=True  valid=True  name='var' name_valid=True  X: 2.5 | 2.5 | 2.5
READ E25                'smoothstep(0, 10, 5)'     simple=True  valid=True  name='var' name_valid=True  X: 0.4999999403953552 | 0.4999999403953552 | 0.4999999403953552
READ E26                'min(1, 2, 3)'             simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E27                'max(var, 4)'              simple=True  valid=True  name='var' name_valid=True  X: 4.0 | 4.0 | 4.0
READ E28                'abs(-var)'                simple=True  valid=True  name='var' name_valid=True  X: 2.5 | 2.5 | 2.5
READ E29                '1e3'                      simple=True  valid=True  name='var' name_valid=True  X: 1000.0 | 1000.0 | 1000.0
READ E30                '0x10'                     simple=False valid=True  name='var' name_valid=True  X: 16.0 | 16.0 | 16.0
READ E31                '1 / 0'                    simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E32                'fmod(1, 0)'               simple=True  valid=True  name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E33                'sqrt(-1)'                 simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E34                'log(0)'                   simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E35                'asin(2)'                  simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E36                'frame'                    simple=True  valid=True  name='var' name_valid=True  X: 1.0 | 5.0 | 5.5
READ E37                'sin(frame / 8) * 4'       simple=True  valid=True  name='var' name_valid=True  X: 0.4986989200115204 | 2.3403890132904053 | 2.53842830657959
READ E38                'depsgraph is not None'    simple=False valid=True  name='var' name_valid=True  X: 1.0 | 1.0 | 1.0
READ E39                'myfn(var)'                simple=False valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E40                'len([1, 2])'              simple=False valid=True  name='var' name_valid=True  X: 2.0 | 2.0 | 2.0
READ E_name_accent      'vé'                       simple=False valid=True  name='vé' name_valid=True  X: 2.5 | 2.5 | 2.5
READ E_name_underscore  '_u'                       simple=True  valid=True  name='_u' name_valid=False  X: 2.5 | 2.5 | 2.5
READ E_self             'self.scale[0]'            simple=False valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
```
Opened again with `-y` (Python allowed), every `READ E…` line was the same except:
```
READ E_self             'self.scale[0]'            simple=False valid=True  name='var' name_valid=True  X: 4.0 | 4.0 | 4.0
```
and Blender printed for `myfn(var)`:
```
Error in PyDriver: expression failed: myfn(var)
Traceback (most recent call last):
  File "<bpy driver>", line 1, in <module>
NameError: name 'myfn' is not defined
READ E39                'myfn(var)'                simple=False valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
```

*What this settles from the list under "Not covered":* `e`, `**`, `%`, `//` and `0x10` are not simple; unary minus and parentheses are; `3 / 2` gave 1.5; `int(-1.9)` gave −1.0, `round(2.5)` 3.0; `clamp(5)` 1.0; division by zero and domain errors printed an error, gave 0.0 and marked the driver invalid, except `fmod(1, 0)`, which gave 0.0 with no error; `frame` included the subframe (5.5); a variable name with `é` was accepted but made the expression not simple, and `_u` was marked not valid but still worked. *Still open:* precision beyond these lines; the "Slow Python expression" warning, which shows only in the window.

**Not covered.**
- How the direct Simple Expressions evaluator does arithmetic: number precision, integer versus float division, what `int`/`round` return, and what happens on divide-by-zero or a math domain error (such as `sqrt(-1)`).
- Whether `e` or other constants count (only `pi`, `True` and `False` are on the Constants list; an uncited reference to another page was removed and sent back for research).
- Whether unary minus, parentheses, exponent operators (`**`) or `%` are allowed. Only the listed operators are named (an uncited example with parentheses was removed).
- Whether `frame` includes the subframe.
- Whether variable names must also follow Python's identifier rules beyond "start with a letter, and only contain letters, digits, or underscores" (Drivers Panel, Name).

**Outside the interface.** The sources say simple expressions run even with Python script execution disabled. The command line disables that execution by default (`-Y, --disable-autoexec … (Python-drivers & startup scripts), (default)`, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). So, inferred: in a command-line run without `-y` and without trusting preferences, simple expressions still work and non-simple ones do not. `frame` is tied to `bpy.context.scene.frame_current`. Separately, the manual warns that `bpy.context` "represents the interface context and may not match the context evaluating the driver" (https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html; *(changed by citation check 2026-11-13: added the missing manual page address)*). The sources don't say which frame `frame` takes when the two differ, for example in a render. **Update 2026-11-13 — observed** (hand test 4, Creator's Mac, Blender 5.3.0): with no window and no `-y` (Python scripts not allowed), every expression marked simple gave its value. So did the expressions marked not simple `var ** 2`, `var % 2`, `e`, `7 // 2`, `0x10`, `depsgraph is not None`, `len([1, 2])` and `vé`. Only `self.scale[0]` and `myfn(var)` were refused, with Blender printing `restricted access disallows name 'scale'` and `… 'myfn'`.

**Operating system.** The sources do not say anything operating-system specific about this. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

**Where I looked.** All driver pages, read in full. `bl_math` and `Driver` in the API. `bpy.app.driver_namespace` in `bpy.app`. I searched the manual, API and release notes for "simple expression", "Slow Python", "is_simple_expression", "natively", "bl_math" and "driver_namespace".

---

## 5. What value does a driven setting take when its driver cannot be worked out — for example a path that leads nowhere, a missing target, or an expression that fails or is not allowed to run?

**Label:** not in the sources (the value the driven setting ends up with). Documented: what marks a driver as failed, the fallback option, and the "skipped" wording. Inferred where noted. **Update 2026-11-13:** observed by hand tests 4, 5 and 6 on the Creator's Mac — see *Observed* below.

**Answer.** The sources never state the value a driven setting takes when its driver fails. Here is what they do say:
- **Path that leads nowhere / missing target (variable side).** Without a fallback, an unresolvable path causes "a driver evaluation failure". With the fallback switched on, the variable takes the fallback value instead (default 0.0) and the driver carries on. Blender records that the fallback was used. An editor filter can list such drivers among the errors.
- **What a failure does to the driver.** A driver that "could not be evaluated in past" is marked not valid and "should be skipped". Its F-Curve carries a matching flag. The API's driver-namespace note gives one cause: an expression that calls a function not yet in the namespace marks "the driver as invalid - preventing further evaluation". "Skipped" suggests the driver stops writing to the setting, which would leave the setting at whatever value it otherwise has. But the sources don't say what that value is. This is an inference, and a weak one.
- **Expression not allowed to run.** By default Blender restricts Python. A non-simple expression needs the file trusted or Auto Run turned on (quoted in question 4 from Troubleshooting). The manual's screenshot of this case shows "Python restricted for security" under an expression, with Driver Value "0.000". This is the documented display in one picture. It doesn't say that the driven setting itself becomes 0.
- **Driven property path itself broken.** F-Curves that reference a property that doesn't exist count as broken/"disabled". They can be revived with an operator. There is also an operator that deletes all visible invalid drivers.

**Quotes.**
- API, Driver — https://docs.blender.org/api/5.3/bpy.types.Driver.html — "is_valid — Driver could not be evaluated in past, so should be skipped (default True)"
- API, FCurve — https://docs.blender.org/api/5.3/bpy.types.FCurve.html — "is_valid — False when F-Curve could not be evaluated in past, so should be skipped when evaluating (default True)" / "mute — Disable F-Curve evaluation (default False)"
- API, DriverTarget — https://docs.blender.org/api/5.3/bpy.types.DriverTarget.html — "use_fallback_value — Use the fallback value if the data path cannot be resolved, instead of failing to evaluate the driver (default False)" / "fallback_value — The value to use if the data path cannot be resolved (in [-inf, inf], default 0.0)" / "is_fallback_used — Indicates that the most recent variable evaluation used the fallback value (default False, readonly)"
- Manual, Drivers Panel — https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html — "If enabled, allows specifying a fallback value to use as the variable value if the RNA Path cannot be resolved, instead of causing a driver evaluation failure."
- API, bpy.app — https://docs.blender.org/api/5.3/bpy.app.html — "Using The --python command line argument to populate name-space often fails to achieve the desired goal because the initial evaluation will lookup a function that doesn’t exist yet, marking the driver as invalid - preventing further evaluation."
- Manual, Troubleshooting — https://docs.blender.org/manual/en/5.3/animation/drivers/troubleshooting.html — "By default Blender will restrict execution of Python scripts." Picture `animation_drivers_troubleshooting_autorun-graph-editor.png`, captioned "A security warning in the Drivers panel.", shows Type "Scripted Expression", Driver Value "0.000", Expression "[0][0]", and the warning "Python restricted for security".
- Manual, Scripting & Security — https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html — "Continue Safely — Keeps the file open without running its scripts." / "Animation Drivers — Python expressions can be used to Drive values and are often used in more advanced rigs and animations."
- Manual, Graph Editor channel editing — https://docs.blender.org/manual/en/5.3/editors/graph_editor/channels/editing.html — "Revive Disabled F-Curves … Clears the “disabled” tag from all F-Curves to get broken F-Curves working again. (A curve is broken if it references a property that doesn’t exist.)"
- API, bpy.ops.anim — https://docs.blender.org/api/5.3/bpy.ops.anim.html — "bpy.ops.anim.channels_fcurves_enable() Clear ‘disabled’ tag from all F-Curves to get broken F-Curves working again"
- API, bpy.ops.graph — https://docs.blender.org/api/5.3/bpy.ops.graph.html — "bpy.ops.graph.driver_delete_invalid() Delete all visible drivers considered invalid"
- API, DopeSheet — https://docs.blender.org/api/5.3/bpy.types.DopeSheet.html — "show_driver_fallback_as_error — Include drivers that relied on any fallback values for their evaluation in the Only Show Errors filter, even if the driver evaluation succeeded (default False)" / "show_only_errors — Only include F-Curves and drivers that are disabled or have errors (default False)"
- Manual, Timeline — https://docs.blender.org/manual/en/5.3/editors/timeline.html — "Only show curves and drivers that are disabled or have errors."

**Observed — hand tests 4, 5 and 6, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** Every test file was made by a script and written with `bpy.data.libraries.write` rather than Blender's ordinary save (which the topic *Reading at a frame* saw writing worked-out values over stored ones), so each driven setting went into the file with a stored value no driver would give.

*Hand test 5 (each failure on its own).* An empty `Src` with custom property `good` = 5.0, and an armature `Arm` with bone `Bone`. One empty per case, its X stored at 3.0: `F_path` Averaged Value reading `Src["nonexistent"]`, fallback off; `F_path_fb` the same with fallback on at 7.0; `F_path_fb_x2` Scripted `v * 2` with that variable, fallback on at 7.0; `F_two` one variable reading `good` and one reading `nonexistent`; `F_noid` no data-block; `F_tc_noid` Transform Channel with no object; `F_tc_nobone` a bone name that does not exist; `F_rot_one`, `F_dist_one` Rotational Difference and Distance with the second object empty; `F_ctx` and `F_ctx_fb` Context Property, Active Scene, `["nonexistent"]`, fallback off and on at 7.0; Scripted with no variables: `F_undef` `undefined_name + 1`, `F_syntax` `1 +`, `F_import` `__import__('math').pi`, `F_str` `'abc'`, `F_none` `None`, `F_len` `len([1, 2])`; `F_keyed` X keyed 3.0 at frame 1 and 5.0 at frame 11 (linear), then the `F_path` driver, stored X 9.0; `F_keyed_blk` the same keys, then the `F_import` driver, stored X 9.0; `F_later` reading `good`. A second Blender opened the file with no flag (Python not allowed). Blender printed, and the script read (stored as opened | evaluated and stored at frame 1 | at frame 2 | back at frame 1), then each driver's flags, then broke and mended `F_later`:
```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Error in PyDriver: expression failed: __import__('math').pi
For target: (type=Object, name="F_keyed_blk", property=location, property_index=0)
Error in PyDriver: expression failed: 'abc'
For target: (type=Object, name="F_str", property=location, property_index=0)
TypeError: must be real number, not str
Error in PyDriver: expression failed: 1 +
For target: (type=Object, name="F_syntax", property=location, property_index=0)
  File "<bpy driver>", line 1
    1 +
SyntaxError: invalid syntax
Error in PyDriver: expression failed: None
For target: (type=Object, name="F_none", property=location, property_index=0)
TypeError: must be real number, not NoneType
	BPY_driver_exec: restricted access disallows name 'undefined_name', enable auto-execution to support
Error in PyDriver: expression failed: undefined_name + 1
For target: (type=Object, name="F_undef", property=location, property_index=0)
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Error in PyDriver: expression failed: __import__('math').pi
For target: (type=Object, name="F_import", property=location, property_index=0)
READ version 5.3.0 bpy.app.autoexec False autoexec_fail True autoexec_fail_message "Driver '__import__('math').pi'"
READ F_ctx         opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_ctx_fb      opened stored 7.0   | frame 1 evaluated 7.0                  stored 7.0                  | frame 2 evaluated 7.0                  stored 7.0                  | back at frame 1 evaluated 7.0                  stored 7.0                 
READ F_dist_one    opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_import      opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_keyed       opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 3.200000047683716    stored 3.200000047683716    | back at frame 1 evaluated 3.0                  stored 3.0                 
READ F_keyed_blk   opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 3.200000047683716    stored 3.200000047683716    | back at frame 1 evaluated 3.0                  stored 3.0                 
READ F_later       opened stored 5.0   | frame 1 evaluated 5.0                  stored 5.0                  | frame 2 evaluated 5.0                  stored 5.0                  | back at frame 1 evaluated 5.0                  stored 5.0                 
READ F_len         opened stored 2.0   | frame 1 evaluated 2.0                  stored 2.0                  | frame 2 evaluated 2.0                  stored 2.0                  | back at frame 1 evaluated 2.0                  stored 2.0                 
READ F_noid        opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_none        opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_path        opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_path_fb     opened stored 7.0   | frame 1 evaluated 7.0                  stored 7.0                  | frame 2 evaluated 7.0                  stored 7.0                  | back at frame 1 evaluated 7.0                  stored 7.0                 
READ F_path_fb_x2  opened stored 14.0  | frame 1 evaluated 14.0                 stored 14.0                 | frame 2 evaluated 14.0                 stored 14.0                 | back at frame 1 evaluated 14.0                 stored 14.0                
READ F_rot_one     opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_str         opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_syntax      opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_tc_nobone   opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_tc_noid     opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_two         opened stored 2.5   | frame 1 evaluated 2.5                  stored 2.5                  | frame 2 evaluated 2.5                  stored 2.5                  | back at frame 1 evaluated 2.5                  stored 2.5                 
READ F_undef       opened stored 0.0   | frame 1 evaluated 0.0                  stored 0.0                  | frame 2 evaluated 0.0                  stored 0.0                  | back at frame 1 evaluated 0.0                  stored 0.0                 
READ F_ctx         type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['v:CONTEXT_PROP[id=None path=\'["nonexistent"]\' fallback_used=False]']
READ F_ctx_fb      type AVERAGE expression '3.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ['v:CONTEXT_PROP[id=None path=\'["nonexistent"]\' fallback_used=True]']
READ F_dist_one    type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ["v:LOC_DIFF[id=Src path='' fallback_used=False; id=None path='' fallback_used=False]"]
READ F_import      type SCRIPTED expression "__import__('math').pi" is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_keyed       type AVERAGE expression '5.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=False]']
READ F_keyed_blk   type SCRIPTED expression "__import__('math').pi" is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_later       type AVERAGE expression '3.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["good"]\' fallback_used=False]']
READ F_len         type SCRIPTED expression 'len([1, 2])' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ F_noid        type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['v:SINGLE_PROP[id=None path=\'["good"]\' fallback_used=False]']
READ F_none        type SCRIPTED expression 'None' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_path        type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=False]']
READ F_path_fb     type AVERAGE expression '3.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=True]']
READ F_path_fb_x2  type SCRIPTED expression 'v * 2' is_simple_expression True driver.is_valid True fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=True]']
READ F_rot_one     type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ["v:ROTATION_DIFF[id=Src path='' fallback_used=False; id=None path='' fallback_used=False]"]
READ F_str         type SCRIPTED expression "'abc'" is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_syntax      type SCRIPTED expression '1 +' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_tc_nobone   type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ["v:TRANSFORMS[id=Arm path='' fallback_used=False]"]
READ F_tc_noid     type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ["v:TRANSFORMS[id=None path='' fallback_used=False]"]
READ F_two         type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['a:SINGLE_PROP[id=Src path=\'["good"]\' fallback_used=False]', 'b:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=False]']
READ F_undef       type SCRIPTED expression 'undefined_name + 1' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ F_later before the change (frame 2) | evaluated X 5.0 stored X 5.0 | type AVERAGE expression '3.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["good"]\' fallback_used=False]']
READ F_later path set to ["nonexistent"], view_layer.update() | evaluated X 0.0 stored X 0.0 | type AVERAGE expression '3.0' is_simple_expression False driver.is_valid False fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["nonexistent"]\' fallback_used=False]']
READ F_later path set back to ["good"], view_layer.update() | evaluated X 5.0 stored X 5.0 | type AVERAGE expression '3.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["good"]\' fallback_used=False]']
READ F_later then frame_set(3) | evaluated X 5.0 stored X 5.0 | type AVERAGE expression '3.0' is_simple_expression False driver.is_valid True fcurve.is_valid True variables ['v:SINGLE_PROP[id=Src path=\'["good"]\' fallback_used=False]']
```
Opened again with `-y` (Python allowed), every `READ F_…` line was the same except those for `F_import` and `F_keyed_blk`:
```
READ F_import      opened stored 3.1415927410125732 | frame 1 evaluated 3.1415927410125732   stored 3.1415927410125732   | frame 2 evaluated 3.1415927410125732   stored 3.1415927410125732   | back at frame 1 evaluated 3.1415927410125732   stored 3.1415927410125732  
READ F_keyed_blk   opened stored 3.1415927410125732 | frame 1 evaluated 3.1415927410125732   stored 3.1415927410125732   | frame 2 evaluated 3.1415927410125732   stored 3.1415927410125732   | back at frame 1 evaluated 3.1415927410125732   stored 3.1415927410125732  
READ F_import      type SCRIPTED expression "__import__('math').pi" is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ F_keyed_blk   type SCRIPTED expression "__import__('math').pi" is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
```

*Hand test 4 (arithmetic failures in simple expressions).* With no flag and with `-y` alike:
```
00:00.670  anim.fcurve      | ERROR Division by Zero in Driver: '1 / 0'
00:00.670  anim.fcurve      | ERROR Math Domain Error in Driver: 'sqrt(-1)'
00:00.673  anim.fcurve      | ERROR Division by Zero in Driver: 'log(0)'
00:00.673  anim.fcurve      | ERROR Math Domain Error in Driver: 'asin(2)'
READ E31                '1 / 0'                    simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E32                'fmod(1, 0)'               simple=True  valid=True  name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E33                'sqrt(-1)'                 simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E34                'log(0)'                   simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
READ E35                'asin(2)'                  simple=True  valid=False name='var' name_valid=True  X: 0.0 | 0.0 | 0.0
```

*Hand test 6 (a driver marked invalid, then the missing function supplied).* An empty `R`, X stored at 3.0, driven by Scripted `my_missing_fn()`; nothing of that name existed when the file was written. A script given after the file name read `R`, put `my_missing_fn` (returning 9.0) into `bpy.app.driver_namespace` if it was not there, moved to frame 2 and read; set the expression to the same text again, moved to frame 3 and read; set `driver.is_valid = True`, moved to frame 4 and read. (a) With `-y` and nothing else:
```
Error in PyDriver: expression failed: my_missing_fn()
NameError: name 'my_missing_fn' is not defined
READ version 5.3.0 bpy.app.autoexec True autoexec_fail False
READ at script start (frame 1) | 'my_missing_fn' in driver_namespace False | evaluated X 0.0 stored X 0.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ my_missing_fn put in driver_namespace by this script
READ after frame_set(2) | 'my_missing_fn' in driver_namespace True | evaluated X 0.0 stored X 0.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ after setting the expression again and frame_set(3) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ after driver.is_valid = True and frame_set(4) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
```
(b) With `-y` and, before the file name, `--python` with a script that registers a `load_post` handler putting the function in the namespace:
```
PRE load_post handler registered
HANDLER load_post ran: my_missing_fn put in driver_namespace
READ version 5.3.0 bpy.app.autoexec True autoexec_fail False
READ at script start (frame 1) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ after frame_set(2) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ after setting the expression again and frame_set(3) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ after driver.is_valid = True and frame_set(4) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
```
(c) With `-y` and, before the file name, `--python` with a script that puts the function in the namespace straight away:
```
PRE my_missing_fn put in driver_namespace before the file loads
Error in PyDriver: expression failed: my_missing_fn()
NameError: name 'my_missing_fn' is not defined
READ version 5.3.0 bpy.app.autoexec True autoexec_fail False
READ at script start (frame 1) | 'my_missing_fn' in driver_namespace False | evaluated X 0.0 stored X 0.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ my_missing_fn put in driver_namespace by this script
READ after frame_set(2) | 'my_missing_fn' in driver_namespace True | evaluated X 0.0 stored X 0.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid False fcurve.is_valid True variables []
READ after setting the expression again and frame_set(3) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
READ after driver.is_valid = True and frame_set(4) | 'my_missing_fn' in driver_namespace True | evaluated X 9.0 stored X 9.0 | type SCRIPTED expression 'my_missing_fn()' is_simple_expression False driver.is_valid True fcurve.is_valid True variables []
```
(d) As (b) but with no flag (Python not allowed): every `READ` line the same as in (b) except the first, which read `bpy.app.autoexec False`.

*What these settle from the list under "Not covered":* the value after a failure: 0.0 in every failing case without keys apart from `F_two`, not the stored 3.0; with keys, 0.0 on opening, then the keyed value from the next frame on, the driver staying `driver.is_valid False`; a fallback is used as the variable's value (14.0 through `v * 2`); one failing variable among several marks the driver invalid but the average still came out (2.5); a missing object or bone gave 0.0; a mended path recovered on the next update; an invalid driver was not tried again on a frame change, but was after its expression was set again; a blocked expression and one that raised an error gave the same 0.0. *Still open:* what the window shows ("Python restricted for security", Driver Value, the red channel); opening in the window with Continue Safely.

**Not covered.**
- The value the driven setting takes in each failure case: its stored value, its keyframed value, 0, or its last good value.
- Whether one failing variable fails the whole driver or only drops that variable.
- Whether "skipped" lasts until the user revives or edits the driver, or is re-tried each frame. The namespace note says "preventing further evaluation", which points to the former for that one cause.
- What a Transform Channel, Rotational Difference or Distance variable does when its object or bone is missing. Fallback is documented only for Single Property and Context Property.
- What a failing Python expression (one that raises an error) gives, as opposed to one that is blocked.

**Outside the interface.**
- From the command line, Python execution is off by default: "-Y, --disable-autoexec — Disable automatic Python script execution (Python-drivers & startup scripts), (default)" and "-y, --enable-autoexec — Enable automatic Python script execution." The preferences "are still used but you may want to override them" (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html; https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html). The security page gives the example "To render an animation in background mode, allowing drivers and other scripts to run: blender --background --enable-autoexec my_movie.blend --render-anim".
- A script can read `bpy.app.autoexec` ("True when auto-execution is allowed"), `bpy.app.autoexec_fail` ("True when auto-execution of scripts failed"), `bpy.app.autoexec_fail_message`, and `bpy.app.autoexec_override` ("The auto-execution set by the command line, None when the preference isn’t overridden") (https://docs.blender.org/api/5.3/bpy.app.html).
- The namespace note in `bpy.app` (quoted above) says populating the namespace with `--python` "often fails", which is a command-line-specific way for a driver to end up invalid. It also says registering a `load_post` handler from `--python` "works for background tasks" (exact `load_post` quote not reproduced here).
- The "Allow Execution" dialog is described only for the interface. The sources don't say what replaces it without a window.
- **Update 2026-11-13 — observed** (hand tests 4, 5 and 6, Creator's Mac, Blender 5.3.0), all with no window: with no flag, `__import__('math').pi` and `undefined_name + 1` were refused (`restricted access disallows name '__import__'`, `… 'undefined_name'`) and gave 0.0; with `-y` the first gave 3.14159… and the second failed with `NameError` and gave 0.0. A function put into `bpy.app.driver_namespace` by a `load_post` handler, registered with `--python` before the file name, was there before the driver first ran, with and without `-y`; one put there by `--python` before the file name was gone once the file had opened.

**Operating system.** The sources do not say anything operating-system specific about this. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

**Where I looked.**
- All driver pages and `advanced/scripting/security`, read in full.
- `advanced/command_line/arguments` (Python options; `--disable-depsgraph-on-file-load`).
- `editors/preferences/save_load` (Auto Run, Excluded Paths), `files/blend/open_save` (Trusted Source), `editors/timeline`, `editors/graph_editor/introduction` (Show Only Errors), `editors/graph_editor/channels/editing` (Revive).
- API: `Driver`, `FCurve`, `DriverTarget`, `DopeSheet`, `bpy.app`, `bpy.ops.anim`, `bpy.ops.graph`, `bpy.ops.outliner` (only "REFRESH_DRIVERS — Refresh Drivers.", no further text).
- Searched for "invalid", "error", "fallback", "cannot be resolved", "could not be evaluated", "restricted", "autoexec", "revive", "disabled" and "skipped".

---

## 6. When drivers and keyframe animation read settings that others set, in what order are they worked out? Can a script learn the value a driver gives at a given frame without moving the scene to that frame?

**Label:** not in the sources (an explicit order between keyframe animation and drivers; any documented way to get a driver's value at another frame without moving the scene). Documented: the surrounding facts below. Inferred where noted. **Update 2026-11-13:** observed by hand tests 3, 7 and 8 on the Creator's Mac — see *Observed* below.

**Answer, first part (order).** No page states the order in which keyframe animation and drivers are worked out relative to each other, or relative to constraints and parenting. The sources do say:
- Blender works out a "dependency graph". The manual says drivers should access object data through Driver Variables so dependencies are tracked. Direct references inside the expression, and properties reached through `self`, may not be tracked. The Drivers panel has an "Update Dependencies" button.
- Blender defers working things out until results are needed. After a change, `view_layer.update()` recalculates "all dependent data (child objects, modifiers, drivers, etc.)".
- Around a frame change during playback and rendering: `frame_change_pre` runs "before any data is evaluated for the new frame", and `frame_change_post` runs "after the data has been evaluated for the new frame".
- On file load, "drivers may be evaluated immediately after loading a blend-file". Registered text blocks run "before drivers are evaluated".
- A pose bone's channel matrix includes "animation and drivers" and "the effect of bone constraints". Its final matrix is "after constraints and drivers are applied". These name what is included, not an order.
- The Bake Action page lists keyframed animation, F-Curve modifiers, drivers and constraints as all feeding "the final motion". That is a list, not an order.

Inferred, and only that: because drivers declare their inputs as dependencies, a setting that a driver reads is presumably worked out before the driver that reads it. That is the usual purpose of a dependency graph. The sources never state the step.

**Answer, second part (value at another frame without moving the scene).** No documented function gives a driver's result at a chosen frame.
- `Scene.frame_set` moves the scene ("updating all objects and view layers immediately").
- The dependency graph object has no frame setting in its documented properties or functions.
- `Driver` has no value or evaluate member in the API. The "Driver Value" readout is an interface field, "Useful for debug purposes".
- `FCurve.evaluate(frame)` does exist and evaluates "F-Curve at given frame". For a driver F-Curve, though, the manual says the horizontal axis is the driver's combined value, not the frame. So calling it on a driver's F-Curve would, by inference, apply only the mapping curve to whatever number is passed in. It would not work out the variables at that frame.
- Inferred: for a driver whose combined value a script can compute itself, such as a Simple Expression using only `frame`, a script could reproduce the expression and then call `FCurve.evaluate` on the result. The sources describe no such route.
- `bpy_extras.anim_utils.bake_action` takes "Frames to bake". The sources don't say whether it moves the scene.

**Quotes.**
- Manual, Drivers Panel — https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html — "Drivers should access object data via Driver Variables, rather than direct references in the Python expression, in order for dependencies to be correctly tracked." / "Note that dependencies for properties accessed via self may not be fully tracked." / "Update Dependencies — Forces an update for the Driver Value dependencies." / "Driver Value — The current result of the driver setup. Useful for debug purposes." / "The built-in depsgraph variable refers to the dependency graph currently evaluating the driver."
- API, Python gotchas (internal data) — https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html — "Consider the calculations that might contribute to the object’s final transformation, this includes: Animation function curves. Drivers and their Python expressions. Constraints Parent objects and all of their F-Curves, constraints, etc." / "To avoid expensive recalculations every time a property is modified, Blender defers the evaluation until the results are needed." / "Now all dependent data (child objects, modifiers, drivers, etc.) have been recalculated and are available to the script within the active view layer."
- API, bpy.app.handlers — https://docs.blender.org/api/5.3/bpy.app.handlers.html — "frame_change_pre — Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame." / "frame_change_post — Called after frame change for playback and rendering, after the data has been evaluated for the new frame."
- API, bpy.app — https://docs.blender.org/api/5.3/bpy.app.html — "Since drivers may be evaluated immediately after loading a blend-file it is necessary to ensure the driver name-space is initialized beforehand." / "This can be done by registering text data-blocks to execute on startup, which executes the scripts before drivers are evaluated."
- API, PoseBone — https://docs.blender.org/api/5.3/bpy.types.PoseBone.html — "matrix — Final 4×4 matrix after constraints and drivers are applied, in the armature object space" / "matrix_channel — 4×4 matrix of the bone’s location/rotation/scale channels (including animation and drivers) and the effect of bone constraints"
- Manual, NLA strip editing (Bake Action) — https://docs.blender.org/manual/en/5.3/editors/nla/editing/strip.html — "The final motion of objects and bones depends not only on the keyframed animation, but also on F-Curve modifiers, drivers, and constraints. The Bake Action operator computes this final motion and creates a corresponding keyframe on every scene frame."
- API, Scene — https://docs.blender.org/api/5.3/bpy.types.Scene.html — "frame_set(frame, *, subframe=0.0) Set scene frame updating all objects and view layers immediately"
- API, FCurve — https://docs.blender.org/api/5.3/bpy.types.FCurve.html — "evaluate(frame) Evaluate F-Curve … frame (float) – Frame, Evaluate F-Curve at given frame … Returns: Value, Value of F-Curve specific frame"
- Manual, Drivers Editor — https://docs.blender.org/manual/en/5.3/editors/drivers_editor.html — "The curve doesn’t represent the property’s value over time, but a mapping from the above intermediate value (X axis) to the final value (Y axis)."
- API, Depsgraph — https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html — documented members: "ids", "mode", "object_instances", "objects", "scene", "scene_eval", "updates", "view_layer", "view_layer_eval", "update() Re-evaluate any modified data-blocks, for example for animation or modifiers.", "id_eval_get", "id_type_updated" (no frame member). The example says: "This object has animation and drivers applied on it, together with constraints and modifiers."
- API, Context — https://docs.blender.org/api/5.3/bpy.types.Context.html — "evaluated_depsgraph_get() Get the dependency graph for the current scene and view layer, to access to data-blocks with animation and modifiers applied."
- API, bpy_extras.anim_utils — https://docs.blender.org/api/5.3/bpy_extras.anim_utils.html — "bpy_extras.anim_utils.bake_action(obj, *, action, frames, bake_options)" / "frames (Iterable[int]) – Frames to bake."

**Ambiguity.** "In what order are they worked out" could mean (a) keyframe animation versus drivers on the same data-block or the same setting, or (b) one driver or animated setting versus another that it reads. The sources answer neither outright. The only material that touches (b) is the dependency-tracking lines above.

**Observed — hand tests 3, 7 and 8, 2026-11-13, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** Every test file was made by a script and written with `bpy.data.libraries.write` rather than Blender's ordinary save (which the topic *Reading at a frame* saw writing worked-out values over stored ones), so each driven setting went into the file with a stored value no driver would give.

*Hand test 7 (order, asking without moving, a loop, a bake).* Frames 1–11, written at frame 1. `A`: X keyed 2 at frame 1 and 6 at frame 11 (linear). `B`: Scripted `v * 2` reading `A`'s X; `C`: Scripted `v + 1` reading `B`'s X; `D`: Transform Channel on `A`, World Space; `A2`: Scripted `frame`; `B2`: Averaged Value reading `A2`'s X; `Y`: Scripted `frame * 2` whose driver curve had keys (0, 0) and (2, 1), linear, Constant extrapolation; `L1` and `L2`: Scripted `v + 1`, each reading the other's X. Every X stored at 100.0, `L1` and `L2` at 0.0. Opening, Blender printed:
```
00:00.388  depsgraph        | WARNING Dependency cycle detected:
                            |   OBL2/DRIVER(location) depends on
                            |   OBL1/TRANSFORM_FINAL() via 'RNA Target -> Driver'
                            |   OBL1/TRANSFORM_SIMULATION_INIT() via 'Simulation -> Final Transform'
                            |   OBL1/TRANSFORM_EVAL() via 'Transform Eval -> Simulation Init'
                            |   OBL1/TRANSFORM_LOCAL() via 'Eval'
                            |   OBL1/TRANSFORM_INIT() via 'Transform Init'
                            |   OBL1/DRIVER(location) via 'Driver -> Driven Property'
                            |   OBL2/TRANSFORM_FINAL() via 'RNA Target -> Driver'
                            |   OBL2/TRANSFORM_SIMULATION_INIT() via 'Simulation -> Final Transform'
                            |   OBL2/TRANSFORM_EVAL() via 'Transform Eval -> Simulation Init'
                            |   OBL2/TRANSFORM_LOCAL() via 'Eval'
                            |   OBL2/TRANSFORM_INIT() via 'Transform Init'
                            |   OBL2/DRIVER(location) via 'Driver -> Driven Property'
                            | 
00:00.388  depsgraph        | WARNING Detected 1 dependency cycles
```
A script then printed, at frame 1, then asking each driver curve's `evaluate` without moving, then after `frame_set` to 6, 11, 1, 11, 6 and 1:
```
READ as opened: frame_current 1 stored X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0
READ after evaluated_depsgraph_get at frame 1 | frame_current 1 | evaluated X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0
READ without moving: B driver curve .evaluate(11.0) = 11.0 ; frame_current after 1
READ without moving: C driver curve .evaluate(11.0) = 11.0 ; frame_current after 1
READ without moving: Y driver curve .evaluate(1.0) = 0.5 ; frame_current after 1
READ without moving: Y driver curve .evaluate(10.0) = 1.0 ; frame_current after 1
READ after frame_set(6) | frame_current 6 | evaluated X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(11) | frame_current 11 | evaluated X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(1) | frame_current 1 | evaluated X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(11) | frame_current 11 | evaluated X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=6.0 B=12.0 C=13.0 D=6.0 A2=11.0 B2=11.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(6) | frame_current 6 | evaluated X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=4.0 B=8.0 C=9.0 D=4.0 A2=6.0 B2=6.0 Y=1.0 L1=2.0 L2=1.0
READ after frame_set(1) | frame_current 1 | evaluated X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0 | stored X A=2.0 B=4.0 C=5.0 D=2.0 A2=1.0 B2=1.0 Y=1.0 L1=2.0 L2=1.0
```
Then, at frame 1, it ran `bpy_extras.anim_utils.bake_action` on `A2` for frames 1–5 (object, location and visual keying on; arguments read from the function itself in this version) with a `frame_change_post` handler collecting the frame number each time it ran:
```
READ bake_action signature (obj, *, action, frames, bake_options)
READ before bake: frame_current 1 A2 action None
READ after bake: frame_current 1 frame_change_post calls during bake [1, 2, 3, 4, 5, 1]
READ baked location[0] keys [(1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0), (5.0, 5.0)]
READ A2 drivers after bake ["type SCRIPTED expression 'frame' is_simple_expression True driver.is_valid True fcurve.is_valid True variables []"]
```

*Hand test 8 (keys, an NLA strip and a constraint against a driver).* `K`: X keyed 5.0 at frame 1, then a Scripted driver `2` on the same X. `N`: an action with X keyed 10 at frames 1 and 11, on an NLA track as a strip (Replace) with no active action, then a Scripted driver `2` on the same X. An armature with bone `Bone`, its pose bone with a Copy Location constraint targeting an empty at (5, 0, 0) and a Scripted driver `1` on its X location. `BRw`, `BRt`, `BRl`: Transform Channel on `Bone`, X Location, in World, Transform and Local Space. Every driven X stored at 42.0. The script's lines:
```
READ version 5.3.0
READ as opened: stored K X 2.0 N X 2.0 pose Bone location X 1.0
READ after evaluated_depsgraph_get at frame 1 | frame_current 1
READ    K evaluated X 2.0 stored 2.0
READ    N evaluated X 2.0 stored 2.0
READ    pose Bone evaluated location X 1.0 stored 1.0 | evaluated pose matrix translation (5.0, 0.0, 0.0) | evaluated matrix_channel translation (5.0, 0.0, 0.0)
READ    BRw evaluated X 5.0 BRt 1.0 BRl 5.0
READ after frame_set(6) | frame_current 6
READ    K evaluated X 2.0 stored 2.0
READ    N evaluated X 2.0 stored 2.0
READ    pose Bone evaluated location X 1.0 stored 1.0 | evaluated pose matrix translation (5.0, 0.0, 0.0) | evaluated matrix_channel translation (5.0, 0.0, 0.0)
READ    BRw evaluated X 5.0 BRt 1.0 BRl 5.0
```

*Hand test 3 (`evaluate` on a driver curve).* On eleven driver curves of different shapes (question 3), `evaluate(x)` on the driver's curve gave the same number as the driven value whenever the driver's input was x and the curve had keys or modifiers; on a curve with neither, it gave 0.0 while the driven value was x.

*What these settle from the list under "Not covered":* a driver over keys on the same setting (the driver's 2.0 at every frame); a driver over an NLA strip (2.0); a driver and a constraint on the same bone (the bone's own X 1.0, its pose position at the constraint's 5.0); a loop between two drivers (Blender printed a warning and both kept the same numbers at every frame); asking a driver's value at another frame without moving (no call found: `evaluate` on the driver's curve maps the number given to it, not a frame; `bake_action` moved the frame to 1–5 and back). *Still open:* moving the frame in the window.

**Not covered.**
- Whether a driver overrides keyframes on the same setting, or the reverse.
- Whether drivers run before or after the NLA/action result for their data-block.
- Whether drivers run before or after constraints and parenting of the same object.
- What happens with circular dependencies between drivers.
- Whether any API call works out a driver at a frame other than the current one.

**Outside the interface.**
- The handlers above are described "for playback and rendering". The API also offers `Scene.frame_set`, `Depsgraph.update`, `ViewLayer.update` and `Context.evaluated_depsgraph_get` to scripts.
- From the command line, the manual lists `--disable-depsgraph-on-file-load`: "Background mode: Do not systematically build and evaluate ViewLayers’ dependency graphs when loading a blend-file in background mode (-b or -c options). Scripts requiring evaluated data then need to explicitly ensure that an evaluated depsgraph is available (e.g. by calling depsgraph = context.evaluated_depsgraph_get())." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html; *(changed by citation check 2026-11-13: added the full option wording from the cited page, which speaks of ViewLayers' dependency graphs and evaluated data without naming drivers separately)*).
- The namespace note in `bpy.app` says: "It is possible to run a script via the --python command line argument, before the blend file. This can register a load-post handler (bpy.app.handlers.load_post) that initializes the name-space. While this works for background tasks..." (https://docs.blender.org/api/5.3/bpy.app.html; *(changed by citation check 2026-11-13: added the exact command-line order wording from the cited page)*).
- The workflow page warns that `bpy.context` may not match the context evaluating the driver, "such as a render running while another view layer is displayed in a viewport" (https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html; *(changed by citation check 2026-11-13: added the missing manual page address)*).
- **Update 2026-11-13 — observed** (hand tests 7 and 8, Creator's Mac, Blender 5.3.0): every read above ran in a Blender with no window; `bpy_extras.anim_utils.bake_action` ran there too, and its `frame_change_post` handler saw frames 1, 2, 3, 4, 5 and then 1.

**Operating system.** The sources do not say anything operating-system specific about this. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

**Where I looked.**
- Manual: all driver pages; `advanced/command_line/arguments` (depsgraph options); `editors/nla/editing/strip` (Bake Action); `physics/rigid_body/introduction` (only says rigid bodies "can be used like regular objects and be part of parent-child relationships, animation constraints and drivers", no order).
- API: `info_gotchas_internal_data_and_python_objects`, `bpy.app`, `bpy.app.handlers`, `bpy.types.Scene` (`frame_set`), `Depsgraph`, `Context`, `ViewLayer` (`update`), `AnimData` (nothing on drivers beyond the `drivers` collection), `FCurve`, `PoseBone`, `bpy_extras.anim_utils`.
- Searched the manual, API and release notes for "dependency graph", "depsgraph", "evaluation order", "order of evaluation", "evaluated before", "evaluated after", "precedence", "priority", "override", "frame_set", "evaluate(" and "driver" alongside "keyfram"/"order"/"before"/"after".
- The 5.3 release notes contain no driver or evaluation-order entries.

---

## Things a person at a computer with Blender 5.3 open could settle in a few minutes

*Pruned 2026-11-13 after the hand tests: all eight items were run on the Creator's Mac with Blender 5.3.0, from the terminal with no window, and their results now sit as observed findings under the questions above (item 1 as hand test 1, item 2 as test 2, item 3 as test 3, item 4 as test 7, item 5 as tests 5 and 6 — sub-test (d) with Python not allowed from the command line rather than by reopening in the window — item 6 as test 4, item 7 as test 8, item 8 as test 7). What is left needs Blender's window:*

1. **The Type menu's labels as drawn (Q1).** Open the Type menu of a driver in the Drivers editor and note the five labels exactly as shown.
2. **What right-click ‣ Add Driver makes (Q3).** On a fresh cube's Location X, right-click ‣ Add Driver; in the Drivers editor note the driver Type, the number of variables, and the F-Curve and Modifiers tabs' contents.
3. **What the window shows for a failed driver (Q5).** For a driver whose path does not exist, and for `__import__('math').pi` reopened with Continue Safely: whether the channel turns red, what Driver Value reads, and whether "Python restricted for security" shows.

---

2026-11-13 — Claude Opus 5.5 (Claude Code) — Working out a driver, Blender 5.3

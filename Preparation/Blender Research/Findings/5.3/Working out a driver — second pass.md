Sources consulted: Blender 5.3 manual, Python API documentation and release notes, date read 2026-11-08; supplied Cycles files, date read 2026-10-16. These are the snapshot dates required by the brief. This second pass was written on 2026-11-13.
I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

**Hand tests (added 2026-11-13):** observed results from hand tests 1–11, run that day on the Creator's Mac by Claude Opus 5.5 (Claude Code), were added under questions 1–6 and the hand-check list at the end was pruned; the tests, their exact commands and every script are in `Audits/Blender Research/Working out a driver/Hand tests — 5.3.md`.

# Working out a driver — Blender 5.3 — second pass

The supplied brief outside the project was read as instructions. The earlier findings were not opened. No helpers were used. Blender was not run. The searches below concern the supplied snapshots, not the live websites.

Search scope: **all four collections** means every searchable file recursively inside `manual_5.3`, `api_5.3`, `release_notes_5.3` and `cycles_5.3` in the supplied Blender Sources folder. Manual/API searches used the whole collections before opening relevant pages. Search terms below were case-insensitive unless stated otherwise. Release-note searches found no statement about animation-driver evaluation relevant to these questions; matches for GPU drivers were ruled out. Cycles matches concerned rendering, GPU drivers and passing dependency graphs to the renderer, rather than implementing animation-driver evaluation. No behavioural finding is taken from those matches.

## 1. How does each driver type — Averaged Value, Sum Values, Scripted Expression, Minimum Value and Maximum Value — combine its variables into one value?

**Label: documented** (for Average, Sum, Scripted Expression, Min and Max); **inferred** (that the question's Minimum Value and Maximum Value match Min and Max in the docs). *(Changed by citation check 2026-11-13: marked equating "Minimum Value" and "Maximum Value" with "Min" and "Max" as inferred from the documentation, as the manual writes "Min and Max" and only hand test 1 observed the UI enum names.)* **Update 2026-11-13:** observed by hand tests 1, 4 and 5 on the Creator's Mac — see *Observed* below.

| Driver type | Combination |
| --- | --- |
| Averaged Value | The average of the referenced variables' values. |
| Sum Values | Their sum. |
| Scripted Expression | The result of the supplied Python expression, which can refer to variables by name. |
| Minimum Value | Their lowest value. |
| Maximum Value | Their highest value. |

With one variable, the four built-in functions give the same result. These are the results of the driver configuration; the driver's F-Curve then maps that result to the driven property's value, as covered in question 3.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Driver Settings → Type; date read **2026-11-08**:

> Built-in functions (Average, Sum, Min and Max)

> The driven property will have the value of the average, sum, lowest or highest (respectively)
> of the values of the referenced Driver Variables.
> If there is only one driver variable, these functions will yield the same result.

> An arbitrary Python expression that can refer to the Driver Variables by name. See Expressions.

Exact evidence for the second stage, **Manual**, [Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html), Driver Configuration; date read **2026-11-08**:

> This driver configuration outputs a single value which changes when the variables change.
> This value is then evaluated through the driver function curve to produce the result
> to be applied to the driven property.

**Search record.** Searched all four collections for `averaged value`, `sum values`, `minimum value`, `maximum value`, `rotational difference`, `simple expressions` and `context property`. Followed with `arithmetic`, `average`, `sum`, `minimum`, `maximum`, `scripted` in the manual's complete drivers section and the Driver API page, and `AVERAGE`, `SUM`, `SCRIPTED`, `MIN`, `MAX` in driver API pages. Opened Manual Drivers Panel, Introduction and Workflow & Examples, and API [Driver](https://docs.blender.org/api/5.3/bpy.types.Driver.html). Viewed the Drivers Panel image [panel.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_drivers-panel_panel.png): it shows “Sum Values”, a variable value of “5.000”, and “Driver Value: 5.000”. This is **documented** evidence of the displayed example, not proof of other input cases. The API enum lists the types but does not explain their mathematics; the manual does. Unrelated minimum/maximum limits on node sockets and Cycles samples were ruled out.

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

*What these settle from the list under "Not covered" and additional edge cases:* zero variables (under "Not covered": the four built-in types and the empty expression gave 0.0, the expression `1.0` gave 1.0, and every one stayed `driver.is_valid True`), plus additional observed cases not listed under "Not covered" below: whether the average is a plain mean (1, 2 and 6 gave 3.0), a result that is not a number (a `TypeError` and 0.0), and one failing variable among several (the average came out 2.5 and the driver was marked `driver.is_valid False`). *(Changed by citation check 2026-11-13: separated zero variables, which appears in "Not covered" below, from the three extra observed cases that were not in this document's "Not covered" list.)* *Still open:* the Type menu as drawn in the window; non-finite inputs; precision.

**Not covered.** The result of a built-in combination with zero variables, treatment of non-finite inputs, and numerical precision are not specified by these passages. The searches did not supply an answer to those cases.

**Outside the interface.** Background execution of drivers is documented by the Manual [Scripting & Security](https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html), Command Line, date read **2026-11-08**: “To render an animation in background mode, allowing drivers and other scripts to run:” followed by `blender --background --enable-autoexec my_movie.blend --render-anim`. Applying the same combination definitions there is **inferred** from that support and the definitions above; no separate background arithmetic rule is stated. Scripted execution restrictions are covered in questions 4–5. **Update 2026-11-13 — observed** (hand tests 1, 4 and 5, Creator's Mac, Blender 5.3.0): every combination above was worked out in a Blender started from the terminal with no window (`-b --factory-startup`) — on opening, after `evaluated_depsgraph_get()` and after `frame_set`. Scripted Expressions that Python was not allowed to run are under questions 4 and 5.

**Operating system.** The sources do not explicitly establish identical arithmetic on every supported operating system or name an OS-specific combination rule. No cross-platform execution was performed. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 2. What value does each driver variable type — Single Property, Transform Channel, Rotational Difference, Distance and Context Property — take, and where does that value come from? Include the case where the property a variable reads is itself animated or driven, and where it belongs to another scene or another data-block.

**Label: documented** for the variable definitions, target selection and transform spaces below. **Inferred** for applying the dependency-graph context warning to externally driven work under Outside the interface. **Not in the sources** for the exact evaluation stage or frame used when the source is itself animated/driven, especially across scenes. *(Changed by citation check 2026-11-13: added "Inferred" to match the Outside the interface step below.)* **Update 2026-11-13:** observed by hand tests 5, 7, 9, 10 and 11 on the Creator's Mac — see *Observed* below.

**Single Property.** Reads the RNA property selected by a data-block reference and an RNA path. For a transform property this reads the UI property's value before parenting or constraints. It can name another data-block; the manual gives Key, Image, Object and Material as ID types and “Material.001” as an ID example.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Driver Variables → Single Property; date read **2026-11-08**:

> Retrieves the value of an RNA property, specified by a data-block reference and a path string.

> In case of transform properties, this will return the exact value of the UI property,
> while Transform Channel will take parenting and/or constraints into account as needed.

> The ID-block type. For example: Key, Image, Object, Material.

> The ID of the ID-block type. For example: “Material.001”.

> For example: location.x or location[0] for the X location animation channel
> value (before parenting or constraints), or ["prop_name"] for a custom property.

**Transform Channel.** Reads a selected location, rotation or scale channel of an object or bone, in the selected transform space. Rotation mode controls how rotation is represented. Average Scale is the cubic root of the total volume change and can be negative for a flipped object.

Exact evidence, **Manual**, same page, Transform Channel; date read **2026-11-08**:

> Retrieves the value of a Transform channel from an object or bone.

> The Average Scale option retrieves the combined scale value,
> computed as the cubic root of the total change in volume.
> Unlike X/Y/Z Scale, this value can be negative if the object is flipped by negative scaling.

> For rotation channels, specifies the type of rotation data to use, including
> different explicit Euler orders. Defaults to using the Euler order of
> the target. See Rotation Channel Modes.

Exact space definitions, **Python API**, [DriverTarget](https://docs.blender.org/api/5.3/bpy.types.DriverTarget.html), `transform_space`; date read **2026-11-08**:

> World Space – Transforms include effects of parenting/restpose and constraints.

> Transform Space – Transforms don’t include parenting/restpose or constraints.

> Local Space – Transforms include effects of constraints but not parenting/restpose.

**Rotational Difference.** Reads the angle between two objects or bones, in radians. The workflow page calls this the minimum angle in World Space.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Rotational Difference; date read **2026-11-08**:

> Provides the value of the rotational difference between two objects or bones, in radians.

Exact evidence, **Manual**, [Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html), Method 2 – Rotational difference to a target bone; date read **2026-11-08**:

> A rotational difference is the minimum angle between two objects in World Space.

**Distance.** Reads the distance between two objects or bones. The panel lists World Space, Transform Space and Local Space. Its screenshot shows a separate space selector for each target; the example has World Space selected for both.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Distance; date read **2026-11-08**:

> Provides the value of the distance between two objects or bones.

> World Space, Transform Space, Local Space.

The separate target selectors are **documented** by [distance.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_drivers-panel_distance.png), viewed in this pass; date read **2026-11-08**. This image does not establish a formula for comparing differently selected spaces.

**Context Property.** Reads a path from the scene or view layer of the currently evaluating animation system. It is a weak context reference rather than a fixed linked scene/view-layer reference. It can therefore refer to the set's camera when linked animation is used in a set. Single Property and Context Property can use an enabled fallback when the path cannot be resolved; see question 5.

Exact evidence, **Manual**, same page, Context Property; date read **2026-11-08**:

> Provides the value of a property that is implicitly referring to either a scene
> or a view layer of the currently evaluating animation system.
> This is a weak reference which does not lead to the scene or view layer
> referenced from the driver to be linked when linking animation data.

> It is possible to set up a driver in a character file,
> and make the driver use the set camera when the character is linked into a set.

Exact evidence, **Python API**, [DriverTarget](https://docs.blender.org/api/5.3/bpy.types.DriverTarget.html), `context_property`; date read **2026-11-08**:

> Active Scene – Currently evaluating scene.

> Active View Layer – Currently evaluating view layer.

**Animated or driven inputs; another scene.** The sources tell us to use variables so dependencies are tracked. They also describe evaluated data with animation and drivers applied. They do not explicitly connect those statements into a guarantee of which evaluation stage each variable reads from an animated/driven property. In particular, the permitted sources do not settle which scene's time is used for an explicit target in another scene, whether an otherwise absent target is evaluated, or the ordering of chained drivers. No answer about those cases is claimed from the general definition “value of an RNA property”.

Exact evidence about dependencies, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Driver Variables; date read **2026-11-08**:

> Drivers should access object data via Driver Variables, rather than direct references in the Python expression,
> in order for dependencies to be correctly tracked.

Exact evidence about evaluated objects, **Python API**, [Depsgraph](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html), Evaluated ID example; date read **2026-11-08**:

> # This object has animation and drivers applied on it, together with constraints and
> # modifiers.

That last quotation describes objects obtained from an evaluated dependency graph. It does not say how a driver variable internally retrieves its source.

**Search record.** Searched all four collections for the five variable names, then the API identifiers `SINGLE_PROP`, `TRANSFORMS`, `ROTATION_DIFF`, `LOC_DIFF`, `CONTEXT_PROP`. Searched all four for `cross.scene`, `other scene`, `another scene`, `animated propert`, `driven propert`, `dependency cycle`, `cyclic depend`, `circular depend`, and driver/evaluation/dependency combinations. Also searched the entire manual/API/release-note collections for `animated`, `driven`, `original`, `evaluated` within five lines of “driver”; inspected relevant hits. Opened Manual Drivers Panel, Introduction, Workflow & Examples, and API DriverVariable, DriverTarget and Depsgraph. Viewed Drivers Panel images [transform-channel2.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_drivers-panel_transform-channel2.png), [rotational-difference.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_drivers-panel_rotational-difference.png), distance.png and [context-property.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_drivers-panel_context-property.png). The images show selected targets, spaces, values and a context fallback; they do not show evaluation order or cross-scene time. Opened Manual [Library Overrides](https://docs.blender.org/manual/en/5.3/files/linked_libraries/library_overrides.html), Animation & Overrides, and [Link & Append](https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html), Known Limitations: their rules concern editing linked data and links between files, not the value a driver reads across scenes. Opened the animation section of [glTF 2.0](https://docs.blender.org/manual/en/5.3/addons/scene_gltf2.html): its shape-key sampling restriction concerns export, not general driver inputs.

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

**Not covered.** Exact timing for animated/driven inputs; cross-scene evaluation and missing-scene membership; mixed-space distance calculations; and results for missing targets. The definitions do not establish these behaviours. Question 6 addresses the related API and ordering limits.

**Outside the interface.** Context means the evaluating system, not necessarily the displayed scene. The Manual [Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html), Dependency Graph Context, date read **2026-11-08**, explicitly says: “Avoid using bpy.context in driver expressions and custom driver functions. It represents the interface context and may not match the context evaluating the driver. Pass depsgraph to a custom function instead.” Inferring from that render-vs-viewport note that externally driven work should use the evaluation context is **inferred** (added to the source quote). *(Changed by citation check 2026-11-13: fixed the broken split quote into the continuous passage from the manual and marked the step to externally driven work as inferred.)* It does not settle cross-scene input timing. Background drivers are supported as cited in question 1. **Update 2026-11-13 — observed** (hand tests 5, 7, 9, 10 and 11, Creator's Mac, Blender 5.3.0): every variable type was read in a Blender with no window. The "currently evaluating" scene and View Layer of a Context Property followed the scene chosen on the command line with `-S` (test 9: `EAB` read `SB`'s `tag`, samples and frame when run with `-S SB`, and `SA`'s otherwise).

**Operating system.** The sources do not specify an OS-dependent variable definition or explicitly guarantee identical results across all operating systems. The unresolved timing cases remain unresolved on each OS. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 3. What does a driver's own F-Curve do to the value its variables combine into? Cover a curve with no keyframes, one with keyframes, and one with modifiers, and say what a newly made driver comes with.

**Label: documented** for mapping, the default identity map, keyed curves and the presence of a modifier stack on drivers; **inferred** for applying general F-Curve modifier evaluation rules (top-to-bottom order and Additive replacement) to driver curves and for concluding that a modifier can define a curve without manual points. **Not in the sources** for the general result of an entirely empty driver curve and the exact stored contents created by each creation method. *(Changed by citation check 2026-11-13: moved general F-Curve modifier evaluation rules on driver curves and the modifier-only curve conclusion from documented to inferred.)* **Update 2026-11-13:** observed by hand tests 2 and 3 on the Creator's Mac — see *Observed* below.

The F-Curve maps the configuration's output on its X axis to the driven property's value on its Y axis. Its horizontal input is the driver's result, rather than the scene frame.

Exact evidence, **Manual**, [Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html), Graph View; date read **2026-11-08**:

> The X axis maps to the output value of the driver configuration. The units depend on the setup.

> The Y axis shows the value applied to the target property. The units depend on the property.

**With keyframes.** The points and their curve handles shape this mapping. The manual's shape-key example allows more than two points and says to adjust their transitions using the handles. Thus the keyed curve is a mapping of driver input to property output.

Exact evidence, **Manual**, [Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html), Method 1; date read **2026-11-08**:

> Manually set points in the driver curve by selecting a handle and
> dragging it or inserting values in the F-Curve tab.
> The Y axis represents the shape key Value, which should go from 0.0 to 1.0.
> The X axis is usually the frame, but for this driver it represents the rotation value in radians.
> You can have more than two points in the curve and tweak the transitions
> with the handles in the curve view (G to move).

The general F-Curve documentation describes interpolation. For example, **Manual**, [F-Curve Properties](https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/properties.html), Linear interpolation; date read **2026-11-08**:

> The curve goes from one keyframe to the next in a straight line, which prevents abrupt
> changes in value but not in speed.

**With modifiers.** A modifier stack can further change the mapping, including polynomial functions, noise or cyclic repetition (**documented** in Drivers Introduction, which links to F-Curve Modifiers via "See Modifiers for more details"). Applying the general F-Curve Modifiers rules directly to driver curves — that modifiers are evaluated from top to bottom and that a Generator can replace the curve or add its polynomial to it — is **inferred** because the F-Curve Modifiers page describes F-Curves in general rather than drivers specifically. *(Changed by citation check 2026-11-13: marked applying general F-Curve modifier evaluation rules to driver curves as inferred.)*

Exact evidence, **Manual**, [Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html); date read **2026-11-08**:

> they can also combine multiple values using a fixed function or a Python expression
> and further modulate it with a manually defined curve and/or a modifier stack.

Exact evidence, **Manual**, [F-Curve Modifiers](https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/modifiers.html), introduction and Generator → Additive; date read **2026-11-08**:

> Modifiers are evaluated from top to bottom.

> Add the polynomial to the curve rather than replacing it.

A concrete driver example on the Manual [Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html) page (Chained Relative Shape Keys, date read **2026-11-08**) explicitly instructs: “In the Modifiers tab, add a Generator of type Extended Polynomial for both drivers.” From that example and the Introduction's “and/or a modifier stack”, it is **inferred** that a curve can be defined through a modifier without manually drawn points. *(Changed by citation check 2026-11-13: added the exact quote from Workflow & Examples and marked the curve-via-modifier conclusion as inferred.)* **Not in the sources:** a universal rule for every modifier on a curve with no points; some modifiers require keyframe positions, so a single blanket result is not established.

**Newly made driver.** The manual states that the default F-Curve is an identity map: the configuration's value passes through unchanged. Add Driver from the property context menu adds one variable that needs to be filled in. These statements do not specify whether the identity map is stored as keyframes, a Generator, another representation, or a special empty-curve case. The API's Driver type default is `AVERAGE`; this is an API property default, not evidence that every UI creation method initializes the same settings.

Exact evidence, **Manual**, [Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html), Graph View; date read **2026-11-08**:

> The default F-Curve is an identity map, i.e. the value produced by the driver configuration
> is applied to the driven property unchanged. If the driver output value is 2.0,
> the property will be 2.0.

Exact evidence, **Manual**, [Usage](https://docs.blender.org/manual/en/5.3/animation/drivers/usage.html), Add Driver; date read **2026-11-08**:

> This operation adds a driver with a single variable (which needs to be filled in),
> and displays the Edit Driver popover.

Exact evidence, **Python API**, [Driver](https://docs.blender.org/api/5.3/bpy.types.Driver.html), `type`; date read **2026-11-08**:

> Driver type (default 'AVERAGE')

**No keyframes.** The sources do not explicitly state what happens after removing all keyframes and all useful modifiers from an existing driver F-Curve. An identity default does not establish the result of deleting its contents. The generic FCurve API description is also insufficient to settle the driver-specific case:

Exact evidence, **Python API**, [FCurve](https://docs.blender.org/api/5.3/bpy.types.FCurve.html), `is_empty`; date read **2026-11-08**:

> True if the curve contributes no animation due to lack of keyframes or useful modifiers, and should be deleted (default False, readonly)

This does not specify a returned number or whether a driver configuration bypasses an empty mapping curve.

**Search record.** Searched all four collections for `identity map`, `identity`, `no keyframes`, `without keyframes`, empty/curve/driver combinations, driver/default/generator combinations, and `simple expression`. Searched the complete drivers section, Driver F-Curve creation API pages and FModifierGenerator for `keyframe`, `generator`, `modifier`, `default`, `empty`, `linear`; searched FCurve and the Graph Editor F-Curve section for evaluation and interpolation. Opened Manual Introduction, Usage, Workflow & Examples, F-Curve Introduction, F-Curve Properties, F-Curve Modifiers; API FCurve, FModifierGenerator, [AnimDataDrivers](https://docs.blender.org/api/5.3/bpy.types.AnimDataDrivers.html), [bpy_struct](https://docs.blender.org/api/5.3/bpy.types.bpy_struct.html) at `driver_add`, [animation operators](https://docs.blender.org/api/5.3/bpy.ops.anim.html) at `driver_button_add`, and [UI operators](https://docs.blender.org/api/5.3/bpy.ops.ui.html) at `eyedropper_driver`. Creation methods describe adding a driver but do not list its initial keyframes/modifiers. FModifierGenerator's property defaults do not prove a newly created driver contains one. Viewed [introduction_fcurve.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_introduction_fcurve.png) and Workflow & Examples [transform-driver-1.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_workflow-examples_transform-driver-1.png), [transform-driver-2.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_workflow-examples_transform-driver-2.png). They show example graphs and properties, not an inventory of a new driver's stored points/modifiers. The first image documents a graph mapping 2.0 to 0.5; it is not the identity default.

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

**Not covered.** Exact behaviour of an empty driver F-Curve, initial keyframe/modifier counts and settings for each creation route, and the result of every possible modifier combination. Those need a direct check. General FCurve extrapolation is documented, but is not evidence of an empty driver's behaviour.

**Outside the interface.** The API can add driver F-Curves: `driver_add` says “Adds driver(s) to the given property” and returns an FCurve or list of FCurves (**Python API**, [bpy_struct](https://docs.blender.org/api/5.3/bpy.types.bpy_struct.html), date read **2026-11-08**). Applying the documented mapping model to those curves is **inferred**; the sources do not establish that their initial stored contents match UI-created drivers. Background execution is documented as cited in question 1. **Update 2026-11-13 — observed** (hand tests 2 and 3, Creator's Mac, Blender 5.3.0): drivers made by `driver_add` and `drivers.new`, and every curve shape above, behaved as described under *Observed* in a Blender with no window.

**Operating system.** The sources do not explicitly establish an OS-specific curve rule or identical creation contents on every supported OS. No OS comparison was run. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 4. Which expressions count as Simple Expressions? What may they contain, and how are they worked out?

**Label: documented** (for the Simple Expressions subset, Python fallback and `is_simple_expression`); **inferred** (for Simple Expressions running in background mode under Outside the interface). *(Changed by citation check 2026-11-13: added "inferred" to the top label to match the inference made under Outside the interface.)* **Update 2026-11-13:** observed by hand tests 4 on the Creator's Mac — see *Observed* below.

Simple Expressions use the subset listed below. Blender evaluates them directly without the full Python interpreter. More complex expressions use Python. Simple Expressions still run when Python script execution is disabled.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Simple Expressions; date read **2026-11-08**:

> Blender can evaluate a useful subset of Python driver expressions directly,
> which significantly improves performance, especially on multi-core systems.
> To take advantage of this, the driver expression must only use the following features:

| Feature | Exact words from the same source |
| --- | --- |
| Variable Names | “Use only ASCII characters.” |
| Literals | “Floating-point and decimal integer.” |
| Globals | “frame” |
| Constants | “pi, True, False” |
| Operators | “+, -, *, /,”; “==, !=, <, <=, >, >=,”; “and, or, not, conditional operator/ ternary if” |
| Standard Functions | “min, max, radians, degrees,”; “abs, fabs, floor, ceil, trunc, round, int,”; “sin, cos, tan, asin, acos, atan, atan2,”; “exp, log, sqrt, pow, fmod” |
| Blender Provided Functions | “lerp, clamp, smoothstep” |

The semicolons separate source lines; they are not expression syntax. Driver variable names also have the general naming restriction on the same page:

> The name must start with a letter, and only contain letters, digits, or underscores.

Exact execution statements from the same page:

> Simple expressions are evaluated even when Python script execution is disabled.

> When an expression outside of this subset is used, Blender displays a “Slow Python expression”
> warning.

Exact evidence, **Manual**, [Introduction](https://docs.blender.org/manual/en/5.3/animation/drivers/introduction.html), Notes on Scripted Expressions; date read **2026-11-08**:

> When a driver uses a Scripted Expression, Blender can evaluate it without using
> the fully featured Python interpreter if it is simple enough.

> When the expression is not simple, it will be evaluated using Python.

`frame` is the current frame in the manual's example. Custom driver-namespace functions and expressions using `depsgraph` require Python rather than the Simple Expressions evaluator.

Exact evidence, **Manual**, [Workflow & Examples](https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html), Orbit a Point, Square Value and Dependency Graph Context; date read **2026-11-08**:

> frame is a Simple Expression that corresponds to
> bpy.context.scene.frame_current.

> Since Simple Expressions cannot access
> custom functions, using them only makes sense for complex computations.

> Expressions using depsgraph require Python evaluation and are not part of the
> Simple Expressions subset.

The API provides a read-only classification flag, `Driver.is_simple_expression`. Exact evidence, **Python API**, [Driver](https://docs.blender.org/api/5.3/bpy.types.Driver.html); date read **2026-11-08**:

> The scripted expression can be evaluated without using the full Python interpreter (default False, readonly)

**Search record.** Searched all four collections for `simple expression`, `simple.expression`, native/driver/expression combinations, `python interpreter`, `driver namespace`, `driver name.space`. Opened Manual Drivers Panel, Introduction, Workflow & Examples, Troubleshooting, Scripting & Security, and API Driver and bpy.app at driver_namespace. Ruled out the full driver namespace's larger set of functions/constants as a definition of the simple subset: Workflow & Examples explicitly excludes custom functions, while Drivers Panel supplies the allowed list. bpy.app describes namespace initialization, not the simple evaluator's grammar. The API classification flag is useful for checking an expression but does not itself enumerate the grammar.

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

**Not covered.** Detailed floating-point precision, domain-error results, short-circuit implementation details, limits on expression length/complexity, and a complete formal parser specification. The manual's current-frame example does not settle subframe or time-remapping semantics.

**Outside the interface.** The subset's permission to run when Python execution is disabled is stated without a UI restriction. Its use in background mode is **inferred** from that rule and the documented background-driver support cited in question 1. The CLI has `--enable-autoexec` and `--disable-autoexec` for Python execution (**Manual**, [Scripting & Security](https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html), Command Line; date read **2026-11-08**). The sources do not describe a different simple-expression grammar for external control. **Update 2026-11-13 — observed** (hand test 4, Creator's Mac, Blender 5.3.0): with no window and no `-y` (Python scripts not allowed), every expression marked simple gave its value. So did the expressions marked not simple `var ** 2`, `var % 2`, `e`, `7 // 2`, `0x10`, `depsgraph is not None`, `len([1, 2])` and `vé`. Only `self.scale[0]` and `myfn(var)` were refused, with Blender printing `restricted access disallows name 'scale'` and `… 'myfn'`.

**Operating system.** The sources do not give OS-specific simple-expression syntax or evaluation rules, nor an explicit cross-platform numerical guarantee. No OS-specific result is claimed. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 5. What value does a driven setting take when its driver cannot be worked out — for example a path that leads nowhere, a missing target, or an expression that fails or is not allowed to run?

**Label: not in the sources** for the general value assigned to the driven setting after failure. **Documented** for the optional unresolved-path fallback and the execution restrictions. **Inferred** for the fallback value acting as an input through the two-stage pipeline rather than setting the driven property directly. *(Changed by citation check 2026-11-13: added "Inferred" to the top label to match the inference explicitly marked below.)* **Update 2026-11-13:** observed by hand tests 4, 5 and 6 on the Creator's Mac — see *Observed* below.

The permitted sources do not establish a general answer such as zero, the previous value, or the property's default. They distinguish driver evaluation failure, an invalid driver that should be skipped, and an optional fallback that avoids failure. None of those descriptions specifies the final property's value for all the listed failures.

**Unresolved path with fallback enabled.** Single Property and Context Property allow a chosen fallback as the **variable's** value. That value is still an input to the driver's combination/expression and F-Curve; the manual does not say it is assigned directly to the final property. The last sentence is **inferred** from the explicit “variable value” wording and the two-stage pipeline quoted in questions 1 and 3.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Fallback under Single Property and Context Property; date read **2026-11-08**:

> If enabled, allows specifying a fallback value to use as the variable value if the RNA Path cannot
> be resolved, instead of causing a driver evaluation failure.

Exact evidence, **Python API**, [DriverTarget](https://docs.blender.org/api/5.3/bpy.types.DriverTarget.html), `use_fallback_value` and `fallback_value`; date read **2026-11-08**:

> Use the fallback value if the data path cannot be resolved, instead of failing to evaluate the driver (default False)

> The value to use if the data path cannot be resolved (in [-inf, inf], default 0.0)

The fallback is disabled by default. Its numeric default of 0.0 is not evidence that every failed driver writes zero. These passages also do not explicitly settle whether a missing target ID uses that fallback.

**Expression failure or skipped evaluation.** The API documents an invalid/skipped state. It does not specify the property's numeric value while skipped.

Exact evidence, **Python API**, [Driver](https://docs.blender.org/api/5.3/bpy.types.Driver.html), `is_valid`; date read **2026-11-08**:

> Driver could not be evaluated in past, so should be skipped (default True)

The wording is quoted as supplied; it is not expanded into a numerical failure rule. The API also gives a concrete missing-function failure during startup:

Exact evidence, **Python API**, [bpy.app](https://docs.blender.org/api/5.3/bpy.app.html), driver_namespace → File Loading & Order of Initialization; date read **2026-11-08**:

> Using The --python command line argument to populate name-space often fails to achieve the desired goal
> because the initial evaluation will lookup a function that doesn’t exist yet,
> marking the driver as invalid - preventing further evaluation.

**Expression blocked from running.** Simple Expressions still run with Python execution disabled. Other scripted expressions need a trusted source or Auto Run Python Scripts. Neither rule specifies the final setting value when execution is refused.

Exact evidence, **Manual**, [Troubleshooting](https://docs.blender.org/manual/en/5.3/animation/drivers/troubleshooting.html), Scripted Expression; date read **2026-11-08**:

> If using a Scripted Expression Driver Type that doesn’t follow
> the Simple Expressions
> subset, you will have to open the file as Trusted Source,
> or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files.

Viewed its [autorun-graph-editor.png](https://docs.blender.org/manual/en/5.3/_images/animation_drivers_troubleshooting_autorun-graph-editor.png), date read **2026-11-08**. **Documented:** the screenshot displays “Python restricted for security” and “Driver Value: 0.000”. It does not show the driven setting's value or establish that every blocked driver produces zero. It cannot answer the general question.

**Search record.** Searched all four collections for driver/expression paired with `fail`, `error`, `invalid`, `skip`, `zero`, `fallback`, `disable`, and the reverse pairings. Additional whole-collection terms: `missing target`, `unresolved`, `cannot be resolved`, `division by zero`, `zero division`, `ZeroDivision`, `non-finite`, `NaN`. A paragraph search across all four collections paired driver/scripted-expression with `previous value`, `last value`, `retain`, `unchanged`, `reset`, `zero`, `missing`, `invalid`, `exception`, `error`, `fail`, `disable`, `not allowed`. Opened Manual Drivers Panel, Introduction, Troubleshooting, Workflow & Examples at fallback lookup, Scripting & Security, Command Line Arguments; API Driver, DriverTarget, FCurve (`is_valid`, `mute`), bpy.app at driver_namespace, and graph operators at `driver_delete_invalid`. Inspected error-filter search hits in DopeSheet/Timeline documentation. Error flags, deletion and filtering describe diagnostics, not assigned values. The node-math “division by zero results in zero” hits concern shader/compositor/geometry/texture nodes, not driver expressions, so were ruled out. GPU-driver failure and Cycles arithmetic matches were also ruled out. The examined screenshot adds a displayed driver value for one example, not a final-property rule.

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

**Not covered.** Final setting values after an unresolved path without fallback, missing ID/bone, syntax error, arithmetic exception or security refusal; whether a last successful value persists; recovery after repair; whether different failure classes behave differently. No numeric result is asserted for those cases.

**Outside the interface.** The CLI controls Python execution with `--enable-autoexec` / `--disable-autoexec`, as documented in Manual Scripting & Security, date read **2026-11-08**. The bpy.app startup warning above explicitly concerns `--python`. The same API page documents initializing the namespace with registered text blocks before driver evaluation, and a load-post-handler alternative that works for background tasks. These are documented external-control considerations, but do not settle the final property's failure value. **Update 2026-11-13 — observed** (hand tests 4, 5 and 6, Creator's Mac, Blender 5.3.0), all with no window: with no flag, `__import__('math').pi` and `undefined_name + 1` were refused (`restricted access disallows name '__import__'`, `… 'undefined_name'`) and gave 0.0; with `-y` the first gave 3.14159… and the second failed with `NameError` and gave 0.0. A function put into `bpy.app.driver_namespace` by a `load_post` handler, registered with `--python` before the file name, was there before the driver first ran, with and without `-y`; one put there by `--python` before the file name was gone once the file had opened.

**Operating system.** The sources do not specify an OS-dependent numeric failure result or establish one common result for all operating systems. The general result remains not in the sources. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 6. When drivers and keyframe animation read settings that others set, in what order are they worked out? Can a script learn the value a driver gives at a given frame without moving the scene to that frame?

**Label: not in the sources** for a full dependency-order rule and a general complete-driver query at an arbitrary frame without changing scene time. **Documented** for dependency tracking, deferred updates, curve queries and evaluated-data access below. **Inferred** for the limited distinction between a curve query and full driver evaluation. **Update 2026-11-13:** observed by hand tests 3, 7 and 8 on the Creator's Mac — see *Observed* below.

**Reading of the question.** I read “in what order” as asking how dependencies between animated properties and drivers are resolved, including chains. It could also ask which writer wins when both keyframes and a driver affect one setting. The permitted sources do not settle either general rule.

**Order.** The manual says variables are needed for correctly tracked dependencies. It warns that dependencies accessed through `self` may not be fully tracked. These statements do not specify a universal “keyframes first, then drivers” rule, ordering among drivers, a topological algorithm, or results for dependency cycles.

Exact evidence, **Manual**, [Drivers Panel](https://docs.blender.org/manual/en/5.3/animation/drivers/drivers_panel.html), Driver Variables and Use Self; date read **2026-11-08**:

> Drivers should access object data via Driver Variables, rather than direct references in the Python expression,
> in order for dependencies to be correctly tracked.

> Note that dependencies for properties accessed via self may not be fully tracked.

**Updates after script changes.** Blender can defer evaluation until needed. The API's gotchas page describes requesting an update after modifying values; it does not state an internal ordering of keyframe animation and drivers.

Exact evidence, **Python API**, [Internal Data & Python Objects](https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html), Stale Data → No updates after setting values; date read **2026-11-08**:

> To avoid expensive recalculations every time a property is modified,
> Blender defers the evaluation until the results are needed.
> However, while the script runs you may want to access the updated values.
> In this case you need to call bpy.types.ViewLayer.update after modifying values, for example:

> bpy.context.object.location = 1, 2, 3
> bpy.context.view_layer.update()

> Now all dependent data (child objects, modifiers, drivers, etc.)
> have been recalculated and are available to the script within the active view layer.

**Querying a curve at an argument.** `FCurve.evaluate(frame)` is documented. Its parameter is called a frame, and the return is the curve value there. No passage says that calling it on a driver's F-Curve resolves all driver variables and their animated dependencies at that scene frame.

Exact evidence, **Python API**, [FCurve](https://docs.blender.org/api/5.3/bpy.types.FCurve.html), `evaluate(frame)`; date read **2026-11-08**:

> Evaluate F-Curve

> frame (float) – Frame, Evaluate F-Curve at given frame (in [-inf, inf])

> Value, Value of F-Curve specific frame (in [-inf, inf])

**Inferred distinction.** The manual puts the driver's configuration output, rather than scene time, on its mapping F-Curve's X axis (question 3). Therefore that curve and the complete time-dependent driver calculation are different operations. Passing a desired scene-frame number to a generic curve query is not established by these sources as a way to obtain the complete driver's result at that frame. The sources do not explicitly describe the driver-specific implementation of `FCurve.evaluate`; I do not claim its actual runtime behaviour from this inference.

**Reading an evaluated setting.** The API documents obtaining a dependency graph for the current scene/view layer, and obtaining an evaluated ID from it. The evaluated-object example explicitly says animation and drivers are applied. These methods do not document a requested-frame argument.

Exact evidence, **Python API**, [Context](https://docs.blender.org/api/5.3/bpy.types.Context.html), `evaluated_depsgraph_get()`; date read **2026-11-08**:

> Get the dependency graph for the current scene and view layer, to access to data-blocks with animation and modifiers applied. If any data-blocks have been edited, the dependency graph will be updated. This invalidates all references to evaluated data-blocks from the dependency graph.

Exact evidence, **Python API**, [ID](https://docs.blender.org/api/5.3/bpy.types.ID.html), `evaluated_get(depsgraph)`; date read **2026-11-08**:

> Get corresponding evaluated ID from the given dependency graph. Note that this does not ensure the dependency graph is fully evaluated, it just returns the result of the last evaluation.

Exact evidence, **Python API**, [Depsgraph](https://docs.blender.org/api/5.3/bpy.types.Depsgraph.html), Evaluated ID example; date read **2026-11-08**:

> # This object has animation and drivers applied on it, together with constraints and
> # modifiers.

**Moving scene time.** `Scene.frame_set` is the documented method to set the frame and immediately update objects/view layers. That is a route to an updated scene at a chosen frame, but it changes scene time and therefore does not satisfy the question's condition.

Exact evidence, **Python API**, [Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html), `frame_set(frame, *, subframe=0.0)`; date read **2026-11-08**:

> Set scene frame updating all objects and view layers immediately

At `frame_current`, the same page says:

> Current frame, to update animation data from Python frame_set() instead (in [-1048574, 1048574], default 1)

**Answer to the no-movement question.** A generic F-Curve query is documented. A general API that evaluates the complete driver and all upstream animation for an arbitrary scene frame while leaving that scene's time unchanged was not found. This is a source gap, not a claim that such an operation is impossible. A frame-independent driver or an already evaluated current-frame value is a narrower case than the arbitrary-frame question.

**Search record.** Searched all four collections for dependency/driver/animation/evaluation combinations; then `dependency graph`, `dependency cycle`, `cyclic depend`, `circular depend`, `evaluation order`, `order of evaluation`, and evaluation paired with driver/keyframe/animation. Searched the whole manual/API/release-note collections for driver/keyframe/animation within 60 characters of `order`, `dependency`, `priority`, `precedence`, `override`, in both directions. Also searched paragraphs and five-line neighbourhoods for driver/animation paired with `topological`, cross-scene phrases, `animated`, `driven`, `original`, `evaluated`. For querying, searched all four collections for `arbitrary frame`, `given frame`, `specified frame`, `specific frame`, without/frame/scene combinations, and driver/evaluate/sample combinations. Searched API information pages and the manual advanced section for `no updates`, `stale data`, `frame_set`, frame/evaluation and scene/view-layer updates. An initial information-page glob was passed literally and failed; it was rerun successfully using the `info*.txt` glob.

Opened Manual Drivers Panel, Introduction, Workflow & Examples, Command Line Arguments, and API Driver, FCurve, AnimDataDrivers, Scene, Context, ID, Depsgraph, bpy.app, bpy.app.handlers at frame_change_pre/post, and Internal Data & Python Objects at Stale Data. Driver and AnimDataDrivers expose no documented arbitrary-frame complete-driver evaluation method on those pages. ID and Context expose evaluated data for a graph/current scene, not a requested frame. Scene exposes a frame-setting operation. Frame-change handlers describe before/after evaluation around a frame change, not the order within driver/keyframe evaluation. Opened Manual [World Space Animation](https://docs.blender.org/manual/en/5.3/editors/3dview/animation/world_space_animation_copy_paste.html): its “correct order” is about copy/pasting world-space animation, not runtime dependency ordering. Library Override animation rules concern library editing; the glTF sampling note concerns export. Neither supplies the requested order. Unrelated marker/track queries at a frame and modifier-stack ordering were ruled out.

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

**Not covered.** Ordering of keyframes, chained drivers, NLA and constraints; conflicting writers; dependency cycles; cross-scene time; and the actual implementation/result of `FCurve.evaluate` on a driver. The source gap does not prove that no other route can query a complete driver without moving its scene.

**Outside the interface.** The methods above are Python API operations. Background startup has an additional documented consideration. Manual [Command Line Arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html), `--disable-depsgraph-on-file-load`, date read **2026-11-08**, says “Background mode: Do not systematically build and evaluate ViewLayers’ dependency graphs” and “when loading a blend-file in background mode (-b or -c options).” It then says: “Scripts requiring evaluated data then need to explicitly ensure that” / “an evaluated depsgraph is available” / “(e.g. by calling depsgraph = context.evaluated_depsgraph_get()).” These are source lines separated here by slashes. Python namespace initialization also needs to precede driver evaluation, as quoted in question 5. None of this establishes a no-time-change arbitrary-frame query. **Update 2026-11-13 — observed** (hand tests 7 and 8, Creator's Mac, Blender 5.3.0): every read above ran in a Blender with no window; `bpy_extras.anim_utils.bake_action` ran there too, and its `frame_change_post` handler saw frames 1, 2, 3, 4, 5 and then 1.

**Operating system.** The sources do not specify different dependency ordering or arbitrary-frame query behaviour for Windows, macOS or Linux, and do not explicitly guarantee identical results. No cross-platform check was performed. **Update 2026-11-13 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## Separate list: short checks a person can perform in Blender 5.3

*Pruned 2026-11-13 after the hand tests: these checks were run on the Creator's Mac with Blender 5.3.0, from the terminal with no window, and their results now sit as observed findings under the questions above — check 1 as hand test 1; check 2 as test 7 (with the Transform Channel variant); check 3 as test 9, and test 11 for another scene's animated setting; check 4 as tests 2 and 3, for the two routes a script has; check 5 as test 4; check 6 as tests 4, 5 and 6; check 7 as test 7; check 8 as all of them, since every test ran from a script with no window. One machine cannot settle other operating systems: that part of check 8 is still open. What is left:*

1. **Q2 — a source linked into both scenes.** Check 3's repeat "with the source linked into both scenes" was not run; only the reader was linked into both.
2. **Q3 — the window's routes.** Add Driver from the context menu, Copy As New Driver / Paste Driver, and typing `#frame` into a field; and `sampled_points` on each new curve.
3. **All questions — other operating systems.** The same scripts on Linux and on Windows (naming the Windows edition).

2026-11-13 — GPT-6.1-Sol High (Codex) — Working out a driver, Blender 5.3.

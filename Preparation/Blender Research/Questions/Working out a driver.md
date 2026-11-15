# Working out a driver

These questions are about drivers: how Blender works out the value a driver gives the setting it controls. Answer each one from your sources alone.

1. How does each driver type — Averaged Value, Sum Values, Scripted Expression, Minimum Value and Maximum Value — combine its variables into one value?
2. What value does each driver variable type — Single Property, Transform Channel, Rotational Difference, Distance and Context Property — take, and where does that value come from? Include the case where the property a variable reads is itself animated or driven, and where it belongs to another scene or another data-block.
3. What does a driver's own F-Curve do to the value its variables combine into? Cover a curve with no keyframes, one with keyframes, and one with modifiers, and say what a newly made driver comes with.
4. Which expressions count as Simple Expressions? What may they contain, and how are they worked out?
5. What value does a driven setting take when its driver cannot be worked out — for example a path that leads nowhere, a missing target, or an expression that fails or is not allowed to run?
6. When drivers and keyframe animation read settings that others set, in what order are they worked out? Can a script learn the value a driver gives at a given frame without moving the scene to that frame?

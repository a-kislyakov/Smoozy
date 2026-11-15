# Reading at a frame

These questions are about learning, from a script running inside Blender with no window, what a scene's settings are at a given frame of its animation. Answer each one from your sources alone.

1. Can the value an animated setting has at a given frame be learned from an open file without moving the scene to that frame? If so, by what means, and how does the result compare with the value a render uses at that frame?
2. Where a setting is controlled by a driver, can its value at a given frame be learned without moving the scene to that frame? If so, by what means?
3. Where a data-block's animation is made of several actions in the NLA, can the combined value at a given frame be learned without moving the scene to that frame? If so, by what means?
4. When timeline markers switch a scene's camera, which camera is active at a frame before the first marker, between two markers, on a marker, and after the last? Can that be learned without moving the scene to the frame?
5. When a scene is moved to a frame, which settings, of the scene and of other data-blocks, can change, and through what?
6. When a file is opened with no window and with the option not to build and evaluate dependency graphs on load, what does a script that then reads the file's settings see, compared with a normal opening? Consider scenes, View Layers, compositor nodes, render settings and file paths, after the file's own start-up scripts have run.
7. Can a scene be brought to a given frame so that its own settings, its View Layers' switches and its compositor take that frame's values, without the objects in the scene being worked out again? If so, by what means, and how do those values compare with a full move to that frame?

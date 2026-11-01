# File output nodes

These questions ask what File Output nodes in Blender's compositor can write, how their outputs are set up, and how the nodes behave. Answer each one from your sources alone.

1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?
2. Which settings belong to an individual output, and which belong to the node as a whole?
3. Does each output carry its own folder as well as its own name?
4. Can a folder be expressed inside an output's name rather than separately?
5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?
6. Can several outputs be gathered into one file rather than written separately?
7. What happens to a File Output node inside a group of nodes?
8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?
9. What restrictions, if any, apply to the kind of picture a File Output node can write?
10. What happens when two outputs are aimed at the same name in the same place?
11. Can a File Output node write a video file?
12. What can a File Output node be fed, and does it have to come from the render?
13. What causes a File Output node to write, and when during a render does it write each file?
14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?
15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?
16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?
17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

Questions 16 and 17 are about a scene brought in through another scene's video sequence editor, which none of questions 1 to 15 covers.

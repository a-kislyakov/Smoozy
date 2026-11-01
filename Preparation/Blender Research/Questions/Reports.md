# Reports

These questions are about what Blender says while it is working, and what it says when a render does not succeed. Answer each one from your sources alone.

1. What can cause a render to fail, and what does Blender do in each case?
2. Where does Blender say that a render failed, and what does it say there?
3. What does Blender report about how far through a piece of work it has got?
4. What does Blender write out as it works, what is that stream called, and what does it contain?
5. Does Blender estimate the time left, for what unit of work, and how does it describe that estimate?
6. What does Blender report about the memory it is using on a graphics card?
7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?
8. What does a render print over the whole of a run, from its first line to its last?
9. What does a render report about its progress under each of the ways a render can be started?
10. What does a render do when the machine's own memory runs out while it is sampling?
11. What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?
12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used? The status wanted is the one a program that started Blender receives, not anything Blender prints about itself.
13. When a file the scene points at is not there, what does a render put on each of its output streams?

Questions 7 to 11 are not questions 3 and 4 asked again. Those two ask what Blender reports and what its stream contains; these ask about conditions neither of them names.

Questions 12 and 13 are not questions 1, 2 or 4 asked again. Those ask what Blender says and where it says it. Question 12 asks what a run hands back to the program that started it rather than anything it prints, and question 13 asks which stream carries what, under one named condition.

14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

*Question 14 is not questions 3, 4, 8 or 9 asked again. Those ask what a render prints, what its stream contains, what it prints over a whole run, and what each way of starting one reports. This one asks only when a line that has been printed reaches a reader that is not a terminal.*

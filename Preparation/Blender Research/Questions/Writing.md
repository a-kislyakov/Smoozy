# Writing

These questions are about where the files a Blender render produces end up, and how the places a render writes to are recorded. Answer each one from your sources alone.

1. Can where a render writes be settled at the moment the render is started, without changing the file?
2. How is a place to write recorded in a file — what parts does the recorded thing have?
3. What characters or marks can a recorded place to write carry that stand for something else rather than for themselves, and what does each of them do?
4. Does Blender make folders that are not already there when it writes?
5. What does a write do when its folder is not there, when there is no room left, or when it cannot be written to?
6. What are the ways a render can be started, and what does each of them write?
7. If a render can be set up so that it writes no files, what does it leave behind?
8. How does the place a file records for writing become the name of a file on disk? Say where a frame's number appears in it, and what decides the end of the name.
9. Can the place a file output node writes to be settled for a render — at the moment the render is started, and by writing it into the file from outside Blender?
10. Can the place a render records for writing carry characters that the computer it lands on will not accept, and what does a render do when it does?
11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?
12. What does a render do when it must make a folder and cannot?
13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

Question 13 is not question 1 asked again. That one asks whether the place a render writes can be settled at the moment the render is started; this one asks what a script handed in at the same launch sees.

14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

*Question 14 is not question 11 asked again. That one asks what is present where a render writes while it is still working. This one asks about the order in time of one frame's writes against the next frame's, and takes no view on either.*

15. While a render is writing an animation as a single video file, what is in that file at each point of the run, and what decides that?

*Question 15 is not question 11 asked again. That one asks what is present in the places a render writes to while it is still working, and how that changes with the size of the picture. This one asks about one video file across a whole run.*

16. Under what conditions does a render report that a file has been written, and what is on disk at the moment it reports it?

*Question 16 is not questions 5 or 14 asked again. Those ask what a write does when it cannot finish, and how one frame's writes stand in time against the next frame's. This one asks what the report itself corresponds to on disk.*

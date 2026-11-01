# Frames

These questions are about the frames a Blender render works through, and what happens when a render does not run to the end. Answer each one from your sources alone.

1. Can a render that was interrupted by the computer going down be picked up from where it stopped?
2. Can a render that was stopped on purpose be picked up later?
3. Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?
4. Can Blender be asked to render only the frames that are not already on disk, and if so, how does it decide what counts as already there?
5. What does an animation written as a single video file leave behind when the render stops part-way?
6. Is a file existing proof that the frame finished writing?
7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?
8. What does a frame range consist of?
9. What does a frame write when several view layers are active on it?
10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?
11. Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?
12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?
13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?
14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?
15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

Question 15 is not questions 3, 11 or 13 asked again. Those ask what a stop leaves where the render writes, and whether a stop can be held until the frame in hand is finished; this asks what the render is doing in between.

16. When a render working through an animation is stopped at no particular moment, what is in the place it writes to for the frame it had not finished?

*Question 16 is not questions 3, 10, 13 or 15 asked again. Those ask whether the frames already written survive, in what order the frames are worked through, what is left when a stop is aimed at the exact moment a frame's own file is being written, and what the render does between being asked to stop and ending. This one asks only what is there for the frame that was in hand.*

17. When a render writing an animation as a single video file is stopped before the end, what does each of the ways a render can be stopped leave of that file?

*Question 17 is not question 5 asked again. That one asks what a stopped video render leaves behind. This one asks it one way of stopping at a time.*

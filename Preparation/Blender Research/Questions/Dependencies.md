# Dependencies

These questions are about what a Blender file points at outside itself, and how those locations are recorded and found again. Answer each one from your sources alone.

1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.
2. What kinds of library can a file point at, including one that does not sit on the same computer?
3. Can a file hold the things it depends on inside itself partly rather than wholly, and what does a partly-held file report when it is read?
4. Can one thing a file points at mean several files on disk?
5. In what ways can a recorded location be written?
6. What does a location recorded on one kind of computer mean on another, including a kind that names its disks?
7. What does a location pointing somewhere other than the computer the file sits on mean, at either end?
8. Does the difference between capital and small letters in a name matter, and does it matter the same on every kind of computer?
9. Do names written in characters outside the Latin alphabet survive being read out of a file, moved to another computer, and found again by a render?
10. Are some of the things a file points at places it writes to rather than reads from, and where does such a write land?
11. Can a reference point at a named thing inside another file, and what happens when that thing is renamed or is gone?
12. What does a render do when something the file points at is not there, and does it say so anywhere?
13. Does a file pointing at another `.blend` file bring that file's own references with it, how deep does that go, and can they be reached the same way?
14. Does rendering one scene of a file pull only that scene's references?
15. Can a file rely on a colour management profile that lives on the computer rather than inside the file, and how does Blender find it?
16. How is a font installed on one kind of computer recorded in the file, and what does Blender do when a font it points at is not on the computer?
17. What use does a `.blend` file make of the places it records for writing, besides writing to them?
18. What does a render do when a cache or a bake that a file records is absent?
19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

*Question 20 is not questions 1, 3 or 12 asked again. Those ask what a file records about outside files, how it records it, and what a repair writes back. This one asks only what an untouched entry looks like in the file that results from a save.*

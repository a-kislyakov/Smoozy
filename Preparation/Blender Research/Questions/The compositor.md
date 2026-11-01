# The compositor

These questions are about a Blender file's compositor — what makes it run, what it belongs to, what a render becomes once it reaches it, and what decides the size of what gets written. Answer each one from your sources alone.

1. What decides whether a file's compositor runs at all?
2. Can the place where a scene's rendered frames are written be switched off entirely?
3. Does a compositor belong to one scene, or to the whole file it is part of?
4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?
5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?
6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?
7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?
8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?
9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

*Question 9 was added after questions 1 to 8 were answered. It is not a repeat of question 4, which asks whether the second scene's compositor runs; this one asks what the second scene is rendered with. Nothing already written for this topic is being re-asked.*

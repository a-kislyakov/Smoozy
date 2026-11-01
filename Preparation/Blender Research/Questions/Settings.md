# Settings

These questions are about the settings a Blender render uses, and how each of them can be set. Answer each one from your sources alone.

1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file? Cover at least a frame range, an interval to skip frames by, a list of separate frames, a sample count, whether a view layer renders at a given frame, a render engine, and a resolution percentage.
2. Can a render be set to stop once the picture is clean enough, and can it be set to stop after a length of time rather than after a count?
3. Which of a render's settings belong to a particular render engine rather than to every engine?
4. Does a file carry a separate sample count for each render engine?
5. Can a sample count change from frame to frame?
6. Does every render engine render on a computer with no screen attached?
7. Does a scene set up for one render engine come out the same in another?
8. Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?
9. Does a file record whether it renders on the computer's graphics cards or on its processor?
10. What does Blender state about opening a file that was saved by a different version of Blender?
11. Can a file rely on scripts stored inside it to render correctly, and what decides whether those scripts run when a render is started from outside Blender?
12. Can a render started from outside Blender be pointed at one named scene of a file, and what does it render when it is not told which?
13. What decides the list of render engines a file can choose between, and can that list grow on a given computer?
14. What values can a render's resolution percentage be set to?
15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?
16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

Questions 14 and 15 ask what values that one setting will take, and what happens when it is given something else. Question 1 asked which settings can be settled at the moment a render is started; that is a different question, and these two do not repeat it.

Question 16 asks about the render that then runs. Question 1 asked whether a sample count can be settled from outside at all; question 16 does not repeat that and takes no view on it.

17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

*Question 17 was added after questions 1 to 16 were answered. Question 1 asked which settings can be settled from outside at all and question 16 asked what the render then does; neither is repeated here, and this question takes no view on either. What is asked is the name, and nothing else about the three settings is being re-asked.*

18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

*Question 18 is not questions 14, 15 or 16 asked again. Questions 14 and 15 ask this same shape of the resolution percentage and of nothing else. Question 16 asks what a render does with a sample count given from outside and takes no view on which counts are acceptable. This question asks about the sample count itself.*

19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

*Question 19 is not question 3 or question 13 asked again. Those ask which settings belong to a particular render engine and what decides the list of engines. This one asks about denoising settings and nothing else.*

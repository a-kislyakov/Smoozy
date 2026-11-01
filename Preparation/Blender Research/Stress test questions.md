# Stress tests

These questions test whether the completed Blender Research covers complex situations.

1. What if the main `.blend` links `set.blend`, which links `props.blend`, and all three use a different file named `metallic.png`, while the server already has two more unrelated `metallic.png` files?

2. What if `scene.blend` links the same `character.blend` twice through different folders, and each copy points at a different `skin.exr` with the same name?

3. What if the user supplies `character.blend` as a dependency, but it links `rig.blend`, which the user did not supply and which exists on the server only under a different name?

4. What if a linked `.blend` has been remapped on the server, then the user uploads a newer copy over it through the ordinary file browser?

5. What if the main file and a linked file both point at `textures/logo.png`, but one needs the user's version and the other needs a pre-existing server version?

6. What if two missing textures share both a filename and size, but have different pictures, and each is needed by a different linked `.blend`?

7. What if `scene.blend` uses a texture path written on Windows, its linked library uses a relative Linux-style path, and the user is working on a Mac?

8. What if the main `.blend` is checked and remapped successfully, then the user replaces only a nested linked `.blend` with a version that adds three new textures?

9. What if the user renders a linked `.blend` directly after it was previously uploaded only as a dependency of another project?

10. What if one asset is shared by five projects on the server, and the user replaces it while only one of those projects has a waiting render job?

11. What if a dependency's filename uses Cyrillic characters, its parent folder uses Chinese characters, and its linked `.blend` has a name close to Blender's filename-length limit?

12. What if two files differ only by upper- and lower-case letters, were made on a case-insensitive computer, and both are needed after upload to Linux?

13. What if an image sequence is found except for frames 0147 and 0148, while another sequence with the same base name is complete elsewhere on the server?

14. What if one scene needs `smoke_####.vdb`, another needs `smoke_####.vdb` from a different simulation, and their frame ranges overlap?

15. What if the `.blend` contains three scenes: one needs missing assets, one is fully ready, and one has no active view layers on the requested frames?

16. What if the selected scene has two view layers on frame 1, none on frame 2, and one on frame 3, with a File Output node writing a different number of files for each?

17. What if a job renders frames 1–100, but one scene starts at frame -20, another ends at frame 50, and the user requests frames 0–75?

18. What if a scene's regular output goes to the job folder, but two File Output nodes use absolute paths to unrelated folders and a third uses a relative path containing `..`?

19. What if two scenes and three File Output nodes all resolve to the same filename on frame 42?

20. What if a File Output node has four slots: two connected, one muted, and one connected only on alternating frames?

21. What if the normal render output is a multilayer EXR, the compositor writes PNG previews, and another node writes a video file for the whole frame range?

22. What if a movie output is interrupted after frame 80 of 100, but individual image outputs for all 100 frames also exist?

23. What if a prior failed job left outputs for frames 1–30, the user re-runs frames 1–100, and a compositor node writes files whose names do not include frame numbers?

24. What if the output folder already contains files made by another project with exactly the same names the new job will produce?

25. What if Blender finishes rendering every frame, but runs out of disk space while writing one compositor output at the end?

26. What if the render is stopped while the beauty pass is complete but one File Output node is still writing utility passes?

27. What if a connection drops just after Blender starts, then the server reboots after frame 12, and the render is meant to resume without the user connected?

28. What if the job uses an add-on that creates files during the render, those files are read by another part of the same render, and they did not exist when dependencies were checked?

29. What if the file opens on the server with a missing colour-management setting and a missing font, while every texture and linked library is present?

30. What if Blender can render the chosen scene successfully despite a missing asset in another scene, but the dependency check reports the whole `.blend` as incomplete?

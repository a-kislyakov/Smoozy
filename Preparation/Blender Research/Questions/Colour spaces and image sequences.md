# Colour spaces and image sequences

These questions are about two things in a Blender file: the colour spaces it names, and images stored as image sequences. Answer each one from your sources alone.

1. What does Blender do when a file names a colour space that its colour management configuration does not contain? Cover opening the file, a script reading the setting, and rendering.
2. Where does Blender's colour management configuration come from, and how can a script learn which configuration is in use and which colour spaces it contains?
3. For an image whose source is an image sequence, which part of the stored file name stands for the frame number, and how does Blender find the file for a given frame? Include file names whose number has a different count of digits.
4. Which file of an image sequence does a render use at a given scene frame, taking into account the image's own frame settings?
5. How can a script list the files an image sequence will use over a range of scene frames?

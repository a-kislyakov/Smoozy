# Reading a file

These questions are about what can be read out of a Blender file, and what can be written back into one. Answer each one from your sources alone.

1. What does a `.blend` file give up without a Blender to open it, and what needs one?
2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?
3. Can a single read of a file tell, for every view layer, whether that layer renders at any frame later asked about?
4. Can the places a file is set up to write to be read out of it?
5. Can the locations a file records for the things it points at be read out and written back, and by what?
6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?
7. Can something a file points at be traced to the scene that uses it?
8. Can a file's full chain of references be read while some of the referenced files are absent?
9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?
10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?
11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

# File output nodes — later questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles add-on files supplied in this folder. All four were read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

**What this pass was asked.** Question 15 only. The other questions on the sheet were answered in earlier passes and are not missing from here.

---

## 15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?

**Label: observed** — settled by hand test on 2026-08-09. The sources remain silent, and the record of that silence is kept below, because it is what sent this question to the hand tests.

### What the hand tests found — Blender 4.5, 2026-08-09

Every test below was run on both of the project's machines, and each is written up as two results.

**The rented Linux machine.** Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. Blender 4.5.12 at `/opt/blender/4.5/blender`.

**The Creator's Mac.** MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics. Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.

Every run added `--factory-startup` and pointed `BLENDER_USER_RESOURCES` at a throwaway folder made just before it.

**What was set up.** A scene built by script with one File Output node whose Base Path was a writable folder and which held three outputs, their slot paths `aaa`, `bbb` and `ccc`, each fed from the Render Layers node's Image output; PNG, 64 by 64, Cycles on the CPU at one sample. The scene's own render output path was a *different* folder, so the node's files and the scene's own file could be told apart. Rendered with `<blender> --factory-startup -b <file>.blend -f 1`, followed by `echo $?`.

**The control run, both machines.** Three files from the node and one from the scene output, a `Saved:` line for each, exit status 0. On the rented Linux machine, word for word from `Compositing` onward:

```
Fra:1 Mem:8.50M (Peak 9.21M) | Time:00:00.13 | Compositing
Saved: '/root/ht-fon/e2-4.5/out/ccc0001.png'
Saved: '/root/ht-fon/e2-4.5/out/aaa0001.png'
Saved: '/root/ht-fon/e2-4.5/out/bbb0001.png'
Saved: '/root/ht-fon/e2-4.5/scene/f0001.png'
Time: 00:00.24 (Saving: 00:00.00)
```

In 4.5 the node names its files from the slot path plus the frame number: `aaa0001.png`. The Mac gave the same names and the same four lines.

**One output of three blocked, by a folder standing where its file goes.** A directory was made at `bbb0001.png`, which refuses the write even to `root`. The directory was still an empty directory afterwards, so the block bit. On the rented Linux machine:

```
Fra:1 Mem:8.50M (Peak 9.21M) | Time:00:00.14 | Compositing
/root/ht-fon/e3-4.5/out/bbb0001.png: Is a directory
Saved: '/root/ht-fon/e3-4.5/out/ccc0001.png'
Saved: '/root/ht-fon/e3-4.5/out/aaa0001.png'
Error: Render error (Invalid argument) cannot save: '/root/ht-fon/e3-4.5/out/bbb0001.png'
Saved: '/root/ht-fon/e3-4.5/scene/f0001.png'
Time: 00:00.28 (Saving: 00:00.00)
```

On the Creator's Mac the same run gave the same two lines with a different reason in the brackets:

```
<HT>/e3-4.5/out/bbb0001.png: Is a directory
Error: Render error (Is a directory) cannot save: '<HT>/e3-4.5/out/bbb0001.png'
```

**The two machines agreed** on everything the question asks: `aaa` and `ccc` were written at the same byte size as in the control, nothing was left for `bbb`, the scene's own output was written, an error naming the path appeared, the render finished, and `echo $?` gave 0. They disagreed only on the words inside the brackets — `(Invalid argument)` on the rented Linux machine, `(Is a directory)` on the Mac.

**Across a five-frame animation.** Same block, rendered with `-a`. The control run with nothing blocked printed 20 `Saved:` lines and left five files each of `aaa` and `ccc`. The blocked run printed 19 — one fewer, the missing one being `bbb` for frame 1 — reached frame 5, and wrote `bbb` normally for frames 2 to 5, because only frame 1's name was occupied. The error appeared once, not once per frame. Exit status 0. Same on both machines.

**A folder that refuses writing, for a user who is not root.** On the rented Linux machine this was run as the user `tester`, because root writes into a read-only folder without complaint; on the Mac as the ordinary user. In both cases `touch` into the folder was refused with `Permission denied` before Blender was run, so the condition really bit. Where the whole node's folder was read-only and a second File Output node pointed at a writable one, on the rented Linux machine:

```
Error: Render error (Invalid argument) cannot save: '<HT>/e6a-4.5/blocked/ccc0001.png'
Saved: '<HT>/e6a-4.5/good/ddd0001.png'
Error: Render error (Permission denied) cannot save: '<HT>/e6a-4.5/blocked/aaa0001.png'
Saved: '<HT>/e6a-4.5/good/eee0001.png'
Error: Render error (Permission denied) cannot save: '<HT>/e6a-4.5/blocked/bbb0001.png'
Saved: '<HT>/e6a-4.5/scene/f0001.png'
```

Recorded exactly as it came: within that one run 4.5 gave two different reasons in brackets for the same cause. On the Mac it gave `(Permission denied)` for all three. The blocked folder was empty afterwards on both machines, the second node wrote both of its files, the scene output was written, exit status 0.

Where only one output of three sat inside a read-only subfolder — a slot path of `sub/bbb` — the other two were written, nothing was left in the blocked folder, and the exit status was 0, on both machines.

**An output's name carrying a folder.** With slot paths `aaa`, `sub/bbb`, `ccc` and no `sub` folder existing beforehand, 4.5 made the folder and wrote into it, on both machines: `Saved: '<HT>/e5-4.5/out/sub/bbb0001.png'`. This was run in all four versions to the same script, so that 4.5's behaviour here is a measurement rather than an absence.

**Storage running out part-way through the write.** Three outputs, OpenEXR at 32-bit with no compression, 2000 by 2000, aimed at a volume with about 1.5 MB free — on the rented Linux machine the 56 MB filesystem at `/root/small` filled with `dd if=/dev/zero of=/root/small/filler bs=1048576 count=50`, on the Mac a 12 MB disk image filled with `count=9`. The same node writing to an ordinary disk produced complete files of **64,032,556 bytes** on the rented Linux machine and **64,032,662 bytes** on the Mac, and `df` showed the small volume at 0 available and 100% used after each blocked run.

On the rented Linux machine:

```
OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "/root/small/out/bbb0001.exr". No space left on device.
Saved: '/root/small/out/bbb0001.exr'
IMB_exr_begin_write: ERROR: Cannot open image file "/root/small/out/ccc0001.exr". No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/small/out/ccc0001.exr'
IMB_exr_begin_write: ERROR: Cannot open image file "/root/small/out/aaa0001.exr". No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/small/out/aaa0001.exr'
Saved: '<HT>/e7-4.5/scene/f0001.png'
```

and what was left on the volume:

```
-rw-r--r-- 1 root root       0 Aug  9 14:37 aaa0001.exr
-rw-r--r-- 1 root root 4878336 Aug  9 14:37 bbb0001.exr
-rw-r--r-- 1 root root       0 Aug  9 14:37 ccc0001.exr
```

On the Creator's Mac:

```
OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "<HT>/smallmnt/out/ccc0001.exr". No space left on device.
Saved: '<HT>/smallmnt/out/ccc0001.exr'
IMB_exr_begin_write: ERROR: No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '<HT>/smallmnt/out/aaa0001.exr'
IMB_exr_begin_write: ERROR: No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '<HT>/smallmnt/out/bbb0001.exr'
Saved: '<HT>/e7-4.5/scene/f0001.png'
```

and what was left:

```
-rw-r--r--@ 1 artist     staff  2813952 Aug  9 16:40 ccc0001.exr
```

**The two machines agreed** that one file is left **part-written** on disk, that Blender prints a `Saved:` line for that part-written file, that the outputs which could not be started are not written, that the scene's own output on the ordinary disk is written in full, that the render finishes, and that the exit status is 0. They disagreed on two details: the rented Linux machine left **zero-byte files** for the two that could not be opened, where the Mac left **no file at all**; and the Linux message names the path it could not open, where the Mac message does not.

**Two File Output nodes, one unable to write at all.** The second node was aimed inside a path that cannot be a folder, because a plain file sat there; that path was still a plain file of zero bytes afterwards. The healthy node wrote all three of its files, the scene output was written, exit status 0, on both machines. On the rented Linux machine:

```
Error: Render error (Invalid argument) cannot save: '<HT>/e8-4.5/notadir/inside/eee0001.png'
Saved: '<HT>/e8-4.5/out/bbb0001.png'
Saved: '<HT>/e8-4.5/out/ccc0001.png'
Saved: '<HT>/e8-4.5/out/aaa0001.png'
Error: Render error (Not a directory) cannot save: '<HT>/e8-4.5/notadir/inside/ddd0001.png'
Saved: '<HT>/e8-4.5/scene/f0001.png'
```

On the Mac, `(Not a directory)` for both.

**Multi-Layer EXR.** With the node's format set to `OPEN_EXR_MULTILAYER`, the node wrote **exactly one file** for the frame — `0001.exr` — not three, on both machines. So in that mode there are no other files from that node for that frame. With a directory standing at that one name, nothing at all was left for the node, the scene's own output was still written, and the exit status was 0:

```
IMB_exr_begin_write: ERROR: Is a directory.
Error: Error writing render result, Is a directory (see console)
Error: Render error (Is a directory) cannot save: '<HT>/e9-4.5/out/0001.exr'
Saved: '<HT>/e9-4.5/scene/f0001.png'
```

Same on both machines.

**A broken path template in the node's own path.** The control, a valid `{blend_name}_`, was substituted for the blend file's name on both machines — the files came out as `t_aaa0001.png` from `t.blend` — so path templates are live in 4.5 and a broken one failing is not "templates are switched off". With the unclosed `{blend_name_` in **one slot's path**, that one output was skipped and the node's other two were written, alongside a second File Output node which wrote both of its files, the scene output, and exit status 0. One line appeared:

```
Error: Invalid path template in File Output node. Skipping writing file.
```

With the unclosed template in the node's **Base Path**, all three of that node's outputs were skipped — the same line three times — while the second node still wrote both of its files and the scene output was written. The render was not refused and did not stop; over five frames it ran to the end. Exit status 0. Identical on both machines.

**What a script driving the render is handed.** On entry 3's blocked scene, `bpy.ops.render.render(animation=False, write_still=True)` returned **`{'FINISHED'}`** and raised nothing, on both machines, while the same run printed the error naming the blocked file:

```
Error: Render error (Is a directory) cannot save: '<HT>/e11b-4.5/out/bbb0001.png'
Saved: '<HT>/e11b-4.5/out/ccc0001.png'
Saved: '<HT>/e11b-4.5/out/aaa0001.png'
Saved: '<HT>/e11b-4.5/scene/f.png'
VS_RENDER_BEGIN
VS_RENDER_RETURN {'FINISHED'}
VS_RENDER_REPORTS window_manager has no reports list
VS_RENDER_END
```

**What the node and the handler list carry.** Read off a running Blender on both machines, the File Output node's full property list is `base_path`, `active_input_index`, `format`, `save_as_render`, `file_slots`, `layer_slots`, plus the general node properties including `warning_propagation`. **There is no property reporting whether a write succeeded or failed, and none naming the last path written.** The handler list holds `composite_cancel`, `composite_post`, `composite_pre`, `load_post_fail`, `render_cancel`, `render_complete`, `render_init`, `render_post`, `render_pre`, `render_stats`, `render_write`, `save_post`, `save_post_fail`, `save_pre`. There is a failure hook for loading a blend file and one for saving one, and **no failure hook for writing a render frame**. That is an absence in a list demonstrably being read, since the same listing returned the settings and returned `save_post_fail`. `bpy.context.window_manager.reports` does not exist in 4.5 — `PROBE_WM_HAS_REPORTS False` on both machines.

### A second batch of hand tests, sent back by the citation check — Blender 4.5, 2026-08-09

The citation check of this document re-ran every test above and sent three of them back — not because an answer was wrong, but because three written recipes had gaps a fresh pair of hands would trip on. All three were run again by me on 2026-08-09, on the machines each names, with `--factory-startup` and a fresh throwaway settings folder every run as before. **Nothing here changes any answer to question 15.** My working folders were `/root/b2-fon` on the rented Linux machine and a throwaway folder in my own scratch space on the Creator's Mac, written `<B2>` below; both were removed afterwards, and the rented machine's shared 56 MB filesystem was not needed by any of the three and was left as found.

**1. The read-only folder test, and where the test tree has to sit.** The record above says this ran as the user `tester` on the rented Linux machine but not where the files were. It matters: `/root` is `drwx------`, so with the tree under `/root` the run as `tester` never reaches the compositor at all —

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: Cannot read file "/root/b2-fon/e13trap-4.5/t.blend": Permission denied

Blender quit
```

— with exit status **1**, which is a test that did not run rather than a result. Re-run with the whole tree (blend file, both output folders, scene output folder and throwaway settings folder) at `/tmp/b2-e13-4.5`, which `tester` can read and write, the earlier result comes back unchanged. Both controls held first, before Blender was started: `touch` into the blocked folder was refused (`touch: cannot touch '/tmp/b2-e13-4.5/blocked/canary': Permission denied`) while `touch` into the second node's folder succeeded, so the block bit and the user was not simply unable to write anywhere. Rented Linux machine:

```
Error: Render error (Invalid argument) cannot save: '/tmp/b2-e13-4.5/blocked/ccc0001.png'
Saved: '/tmp/b2-e13-4.5/good/ddd0001.png'
Error: Render error (Permission denied) cannot save: '/tmp/b2-e13-4.5/blocked/aaa0001.png'
Saved: '/tmp/b2-e13-4.5/good/eee0001.png'
Error: Render error (Permission denied) cannot save: '/tmp/b2-e13-4.5/blocked/bbb0001.png'
Saved: '/tmp/b2-e13-4.5/scene/f0001.png'
```

Exit status 0; the blocked folder empty afterwards; the second node's two files written at 4,516 bytes each; the scene's own output written. The oddity recorded above came back — within one run 4.5 gives `(Invalid argument)` for one output and `(Permission denied)` for the other two — and in this run the odd one out was `ccc`. On the Creator's Mac, with the tree at `<B2>/e13-4.5` and the blocked folder at `chmod 500`, the same run gave `(Permission denied)` for all three:

```
Error: Render error (Permission denied) cannot save: '<B2>/e13-4.5/blocked/ccc0001.png'
Saved: '<B2>/e13-4.5/good/ddd0001.png'
Error: Render error (Permission denied) cannot save: '<B2>/e13-4.5/blocked/aaa0001.png'
Saved: '<B2>/e13-4.5/good/eee0001.png'
Error: Render error (Permission denied) cannot save: '<B2>/e13-4.5/blocked/bbb0001.png'
Saved: '<B2>/e13-4.5/scene/f0001.png'
```

Exit status 0, blocked folder empty, both of the second node's files written, scene output written.

**2. What the complete-file byte size actually depends on.** The figure of **64,032,556 bytes** quoted above for a complete OpenEXR file is not a fixed measurement, and this test says what moves it. The same scene was rendered three times on each machine, changing exactly one thing at a time. On the rented Linux machine, with the blend file at `/root/b2-fon/e14-4.5/t.blend` (28 characters) the three complete files came to **64,032,554 bytes** each; with the blend file moved to `…/ttttttttttt.blend` (38 characters, nothing else changed) they came to **64,032,564** — ten characters more in the path, ten bytes more in the file. With the blend path held fixed and the *output folder* lengthened by ten characters instead, the size did not move at all (64,032,555 both times). The blend file's own path is inside each image; `grep -a -o` finds it word for word. On the Creator's Mac the same pattern: a 134-character blend path gave **64,032,660 bytes**, a 144-character one **64,032,670**, and lengthening the output folder changed nothing.

Subtracting the blend path's length from the figure leaves **64,032,526** on both machines. So the roughly 106-byte gap this document records between the rented Linux machine and the Mac is the gap between the two working-folder names, not a difference between the machines. As arithmetic on that relation and nothing more: the 64,032,556 above corresponds to a 30-character blend path, and the citation check's repeat of 64,032,553 to a 27-character one. Neither figure is being corrected — I was not in the room for either run — but the number as printed cannot be checked by anyone whose folders are named differently, and the point it carries here (a complete file of about 64 MB against roughly 5 MB left part-written) is untouched.

**3. What the disk image's file system changes when the volume fills.** The citation check noted that the record above does not say what file system the Mac's 12 MB disk image carried. Re-run on the Creator's Mac on both, with the file system written down from `hdiutil create … -fs HFS+` / `-fs APFS` and confirmed by `diskutil info` on the mounted volume (`File System Personality: HFS+`, `Type (Bundle): hfs`; and `APFS` / `apfs`), each filled to leave exactly 1500 KB free by `df`, 4.5 printed the same wording on both:

```
OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "<B2>/mnt-HFS-4.5/out/ccc0001.exr". No space left on device.
Saved: '<B2>/mnt-HFS-4.5/out/ccc0001.exr'
IMB_exr_begin_write: ERROR: No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '<B2>/mnt-HFS-4.5/out/aaa0001.exr'
IMB_exr_begin_write: ERROR: No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '<B2>/mnt-HFS-4.5/out/bbb0001.exr'
Saved: '<B2>/e12-HFS-4.5/scene/f0001.png'
```

Exit status 0 both times, and the scene's own output written on the ordinary disk both times. **But what is left behind is not the same on the two file systems.** On HFS+ the volume ended with one part-written file, `ccc0001.exr`, of **1,536,000 bytes**. On APFS, from the same 1500 KB free, it ended with **two** part-written files — `bbb0001.exr` at 463,129 bytes and `ccc0001.exr` at 57,625 bytes — and a second `OpenEXR-writePixels: ERROR: Failed to write pixel data…` line to match. And the two file systems do not even report the volume the same way: after the run `df -k` read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS, although the writes had plainly failed for want of space.

So any figure for what a full volume leaves behind has to say which file system it was measured on. This document's "a single part-written file" matches HFS+ and not APFS, and the document does not say which it was.

**Nothing in this batch opens anything on the list of things still needing a person at a computer.** The network folder, the locked file, Windows and the checks needing a window are all untouched by these three tests and stay as they are below.

### What the hand tests did not settle

A network folder taken away part-way through a write could not be run: it needs a file server between the two machines, which the project does not have. A file held open and locked by another program could not be run: neither macOS nor Linux enforces that kind of lock. Nothing was run on Windows: no Windows machine has run it, so that operating system is untested rather than answered. Whether the path field is shown highlighted in red and what its tooltip says, and whether the node shows a warning badge in the compositor, both need eyes on a window and were dropped this round — the rented machine has no display server and the Creator ran nothing; the node property list was checked first and holds nothing carrying warning text, so neither can be read from a terminal instead.

---

### What the sources say

**Label for the documentation on its own: not in the sources.**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or the Cycles files says what happens when a file a File Output node is writing cannot be finished. Specifically, none of them says:

- whether the file that failed is left behind part-written, left behind empty, or removed;
- whether the other files being written for that same frame are still written;
- whether the render stops, skips the frame, or carries on to the next frame;
- whether anything is said about it — a message, a warning, an error, or a change in how Blender finishes.

The manual's page for the node describes only what it writes and where. Its whole account of the node's behaviour is:

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

— Blender manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30.

The Python API page for the node describes it in one line and lists only its settings:

> "Write image file to disk"

— Blender Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30.

**A note on the question's wording.** "Cannot be finished" can be read two ways: the write never succeeds at all (the folder is not writable, the path is not valid), or the write begins and then stops part-way (the drive fills, a network folder disappears). I looked for both and found neither described. The sources are silent on both readings, so the answer above does not depend on which reading is meant.

### Where I looked

Searches run over the whole of the 4.5 manual, the whole of the 4.5 Python API documentation, the whole of the 4.5 release notes, and the Cycles files, for these terms (case-insensitive): "unable to write", "cannot write", "can not write", "failed to write", "write permission", "not writable", "read-only", "disk full", "out of disk", "no space", "insufficient space", "free space", "storage", "quota", "truncated", "corrupt", "incomplete", "partially written", "partial file", "aborts", "abort the render", "render fails", "rendering fails", "could not", "couldn't", "cannot save", "can't save", "write error", "writing error", "I/O error", "permission denied", "denied", "errno", "OSError", "IOError", "PermissionError", "no such file", "invalid path", "bad path", "directory does not exist", "non-existent", "nonexistent", "exit code", "return code", "exit status", "errorlevel", "network drive", "network share", "remote drive", "unmounted".

Every hit was either in an unrelated part of the documentation (mesh editing, sculpting, graphics-card troubleshooting, add-on extensions, the video sequencer's proxies) or concerned something other than a File Output node failing to write. Two hits deserve naming because they are close enough to be mistaken for an answer and are not one:

- The whole compositing section of the manual contains the words "fail", "error" or "warning" only twice, in the Checker texture page and the Defocus blur page. Neither concerns writing files.
- The 4.5 bugfixes release note lists "Placeholder output files wrong name", "Reading mmap'ed images from network drives may crash Blender when the file is not valid", and "File output node crash in eevee when in preview mode and without connections" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30). None of these states what happens on a failed write.

Pages read in full and ruled out:

- File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html. Describes the inputs, the Base Path, the File Format label, and that the node has no output sockets. Says nothing about failure.
- The picture that sits on that page, at https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html (the copy names the file `node-types_CompositorNodeOutputFile.webp`). I opened it. It shows the node with a Base Path field reading `/tmp/`, a folder button, and one Image input labelled WebP. Nothing about failure is visible in it. A picture carries no words to quote, so nothing in this answer rests on it; it is recorded here only as somewhere I looked.
- Output panel (render properties) — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html. Covers File Path, File Extensions, Cache Result, File Format, Color, Overwrite, Placeholders, Color Management, Pixel Density, and video encoding. Says nothing about a failed write.
- Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html. Discusses recovering after a computer turns off mid-render by disabling Overwrite, but never says what Blender itself does when a file cannot be written.
- Introduction (render output) — https://docs.blender.org/manual/en/4.5/render/output/introduction.html. Nothing on failure.
- Info Editor — https://docs.blender.org/manual/en/4.5/editors/info_editor.html. Says only, in general, "The Info editor logs the executed operators as well as errors, warnings, and informational messages." It does not say that a File Output write failure produces any such message.
- Network troubleshooting — https://docs.blender.org/manual/en/4.5/troubleshooting/network.html. Concerns only a blocked connection to the extensions website. Not relevant.
- Path templates — https://docs.blender.org/manual/en/4.5/files/file_paths.html. Confirms templates apply to File Output paths, but says nothing about failed writes.
- `bpy.app.handlers` — https://docs.blender.org/api/4.5/bpy.app.handlers.html. The full list of handlers includes `render_write` ("on writing a render frame (directly after the frame is written)") and `save_post_fail` ("on failure to save a blend file (after)"). There is a handler for a blend-file save failing, and none for a frame or output file failing. That is an absence in a complete list, not a statement about behaviour, so I am not treating it as an answer.
- `bpy.types.RenderEngine` — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html. Has `report(type, message)` "Report info, warning or error messages" and `error_set(message)` "Set error message displaying after the render is finished". These belong to a render engine, not to the compositor's File Output node, and the documentation does not connect them to file writing.
- `bpy.types.Node` — https://docs.blender.org/api/4.5/bpy.types.Node.html. Has `warning_propagation`, "The kinds of messages that should be propagated from this node to the parent group node". This shows nodes in general can carry messages, but nothing says a File Output node raises one when a write fails.
- Release notes for the compositor (https://developer.blender.org/docs/release_notes/4.5/compositor/), core (https://developer.blender.org/docs/release_notes/4.5/core/), rendering (https://developer.blender.org/docs/release_notes/4.5/rendering/) and pipeline, assets and I/O (https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/). The only File Output mentions are that its paths now support template expressions, and that its Image Format Settings now has a Media Type. Nothing about failures.
- The Cycles files supplied (`Blender Sources/cycles_4.5`). I searched every Python file in them for "file_output", "OutputFile", "cannot open", "failed to" and "error". The only error messages there concern OSL shader scripts (`osl.py`, and two more in `__init__.py`) and the add-on's own operators (`operators.py`). Nothing in these files mentions the File Output node at all, so nothing in them bears on this question.

*(Changed by the citation check, 2026-08-09: `__init__.py` also carries error reports — both "OSL support disabled in this build" — and was not named. The claim that nothing mentions the File Output node was checked and holds.)*

All of the above read 2026-07-30.

### Not covered

This answer settles only that the supplied sources do not describe the behaviour. It does not tell you what Blender 4.5 actually does — the part-written file may or may not survive, the sibling files may or may not be written, the render may or may not stop, and a message may or may not appear. It also does not cover what happens when the ordinary render output path (the one in Output properties) fails while File Output nodes succeed, or the reverse, because the sources are equally silent there. And it says nothing about which failure causes are treated alike; a full drive and an unwritable folder could well behave differently and the sources would not tell you.

### Outside the interface

**Brought up to date by hand test, 2026-08-09: observed.** Every test above was run with Blender started from a terminal with no window, which is the only way Smoozy will ever run it, so the whole of the answer above is an answer about Blender driven from outside its own interface. What that route gives you: the error naming the path is printed to the terminal every time, on both machines; the process exit status is **0** in every case, so a script watching only the exit status learns nothing; and `bpy.ops.render.render()` returns `{'FINISHED'}` and raises nothing. There is no handler that fires on a failed write and no property on the node recording one, so the printed line is the only signal. Whether the same behaviour holds *inside* Blender's own window was not tested and is not settled here — the rented machine has no display server and the Creator ran nothing this round.

**Added by the second batch of hand tests, 2026-08-09.** One thing that sharpens the exit-status point above. A run that could not read the blend file at all — the same scene, started as a user who cannot enter the folder it sits in — ends with `Error: Cannot read file "…": Permission denied` and **exit status 1**. So the exit status is not simply always 0: it does report a blend file Blender could not open, and it does not report a File Output write that failed. A script watching only the exit status will therefore see the difference between "the job never started" and "the job ran", and will see no difference at all between "every file was written" and "a file could not be written".

The original note from the sources follows.

The sources do not say. Nothing I found describes this behaviour at all, so nothing distinguishes Blender driven by a person from Blender started and driven by something else. The nearest documented fact is general, not about this node: "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors" and "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from" (Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, read 2026-07-30). That tells you where messages appear if there are any; it does not tell you that a failed File Output write produces one. I also searched all three documentation sets for "exit code", "return code", "exit status" and "errorlevel" and found no matches anywhere, so the sources do not say whether a failed write changes what Blender hands back when it finishes.

### Operating system

**Brought up to date by hand test, 2026-08-09: observed on two operating systems, untested on the third.** Every test above ran on Linux (Ubuntu 22.04.5 on the rented machine) and on macOS 26.5.2 on Apple silicon (the Creator's Mac). On everything the question asks the two agreed: which files survive, which do not, that the render carries on, that an error naming the path is printed, and that the exit status is 0. They differed only in wording and in one detail on disk — 4.5 said `(Invalid argument)` on Linux where the Mac said `(Is a directory)`; and when a volume filled, Linux left **zero-byte files** for the outputs that could never be opened where the Mac left **no file at all**. **Windows is untested**, not answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Added by the second batch of hand tests, 2026-08-09.** One part of that last difference turns out to belong to the disk's own file system rather than to the operating system, and the first batch did not record which file system the Mac's disk image carried. Re-run on the Creator's Mac on a 12 MB **HFS+** image and again on a 12 MB **APFS** image, both filled to leave exactly 1500 KB free, the volume-full test left **one** part-written file on HFS+ and **two** on APFS, and `df -k` afterwards read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS even though the writes had failed for want of space. What did hold on both Mac file systems is the thing the line above rests on: no zero-byte file was left for an output that could never be opened, where the rented Linux machine leaves one. So the Linux-against-macOS difference stands; the count of part-written files and the `df` reading are properties of the file system as well.

The original note from the sources follows.

The sources do not say. The record of looking above covers all four sources, and none of them describes this behaviour for any operating system, so there is nothing in them that could differ between Linux, macOS and Windows, and I will not guess. I did not search separately for the operating systems by name, so a page naming one of them in connection with a failed write would have to be found by that route. Note that the causes of such a failure — how a folder's permissions work, how a network folder behaves when it disappears, what happens as a drive fills — are operating-system matters, but the documentation supplied does not address them in this context.

---

## Things a person at a computer with Blender open could settle in a few minutes

This list has been pruned on 2026-08-09. The researcher proposed seven tests. Five of them were run on both machines and their results are written in above: two File Output nodes with one aimed somewhere unwritable; the same across an animation; a write that fails part-way because the drive fills; the same from the command line with the exit status read; and two outputs in one node with one of them bad. What is left is only what is genuinely still open.

1. **A network folder pulled away mid-render.** Point one File Output node at a mounted network folder and another at a local folder. Start an animation render, and disconnect the network while it is running. Note whether Blender stops, hangs, or continues; what appears in the terminal; and what is left at both locations afterwards. **Could not be run on 2026-08-09**, and the reason is equipment: this needs a file server between two machines, and the project has the Creator's Mac and one rented Linux machine reached over the open internet, with nothing set up to serve a share from either to the other. The network cause of a failed write is therefore untested rather than answered.

2. **The same tests on Windows.** **Not run on 2026-08-09**: no Windows machine has run it. Linux and macOS on Apple silicon are both settled above; Windows is untested, not answered.

3. **What a person sees on screen.** Whether the path field of a File Output node with a broken template is shown highlighted in red and what its tooltip says; whether the node shows a warning badge in the compositor after a failed write; and what the Info editor and the status bar hold. **Dropped on 2026-08-09 as needing a window**: the rented Linux machine has no display server, and the Creator ran nothing this round. Before dropping it, the node's own property list was read from a terminal and holds nothing carrying warning text, and 4.5 has no `bpy.context.window_manager.reports`, so there is no way to read any of this without a window.

A file held open and locked by another program, which the second pass of this document also proposed, **could not be run** either: neither macOS nor Linux enforces that kind of lock against an unrelated program.

---

2026-08-09 — Opus 5 (Claude Code) — File output nodes, Blender 4.5

# File output nodes — later questions, second pass (Blender 4.5)

Sources consulted: Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles 4.5 files supplied in this folder. All four were read on 2026-07-30 (the snapshot date given to me), and I worked through them on 2026-08-09.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project. (The one exception worth naming, so it is on the record: I checked that the folder I was told to write into exists, without listing what is in it. I did not open the existing findings document.)

Only question 15 was assigned to me. The rest of the sheet is untouched.

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

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or the supplied Cycles files describes what happens when a File Output node's write does not complete — not for the file that failed, not for the other files being written for the same frame, not for the render as a whole, and not for what gets reported. The documentation describes the node's settings and what it is for; it never describes failure.

The one thing the sources do say about a File Output path going wrong is about a path that cannot be worked out in the first place, which is a different moment: it happens before writing rather than during it. Because it is the closest documented thing, here it is in full, with its limits stated plainly.

### The one documented near-miss: a broken path template

Blender 4.5 added "path templates" — pieces such as `{blend_name}` written into a path and swapped for real values when the path is used. The release notes say where they are allowed:

> "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30.

The manual says the same, and then says what a broken one does:

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

The manual also shows the person-facing signal, and I opened the picture it points to:

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Hovering over the path field will pop up a tooltip that contains a list of the template errors encountered in that path"

The picture `files_file_paths-invalid_path_template_tooltip.png`, on that same manual page, shows a path box filled red containing `//my_render_dir/{blend_name.png`, with a tooltip headed "Output Path" and, in red, "Syntax error(s): - Invalid or incomplete template expression '{blend_name.png'." The panel it sits in is headed "Output". All of that is **documented**, because it is visible in the picture. That the panel is the Output panel of the scene's render settings rather than a File Output node is **inferred** — the picture carries no such label, and a reader has to supply it. On that inferred step, the picture confirms the red-field-and-tooltip behaviour only for the render output path.

*(Changed by the citation check, 2026-08-09: the second half of this reading was labelled documented and is a step the reader supplies, so it is now labelled inferred. The words quoted from the picture were checked against `manual_4.5/_images/files_file_paths-invalid_path_template_tooltip.png` and are all visible in it.)*

Three limits on this, which is why it is not an answer to the question asked:
- It is about a path Blender cannot make sense of, not about a write that starts and cannot finish. Nothing here covers a disk that fills, a folder that is not writable, or a network location that goes away mid-write.
- The sentence about the render failing names the *render output path* as its example. It does not say what a broken template in a File Output node's path does. Extending it to the File Output node would be my step, not the manual's.
- It says nothing about the other files for the same frame.

The Python API release notes add one related fact, again about the render output path only:

> "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/python_api/ — read 2026-07-30.

### Where reports would appear, if there were any

The sources say where errors show up in general, without ever saying that a failed image write produces one:

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/info_editor.html — read 2026-07-30.

> "Informational messages or warnings, such as after saving a file."

(describing the Report Message part of the status bar) Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read 2026-07-30.

I include these only to show I looked; neither mentions image output, File Output nodes, or write failures.

### Not covered

Everything the question actually asks:
- Whether a file that could not be finished is left behind, left partly written, or removed.
- Whether the other outputs of the same node, or other File Output nodes, still write for that frame.
- Whether the render stops, skips the frame, or carries on to the next frame.
- Whether anything is reported at all, in what wording, and where it lands.
- Whether the answer differs by cause (disk full, no permission, folder missing, network location gone, file locked by another program).
- Whether it differs by file format, or between a single-file output and a multi-layer output.

Also not covered: whether the documented path-template failure behaves the same way for a File Output node's path as it does for the render output path.

### Outside the interface

**Brought up to date by hand test, 2026-08-09: observed.** Every test above was run with Blender started from a terminal with no window, which is the only way Smoozy will ever run it, so the whole of the answer above is an answer about Blender driven from outside its own interface. What that route gives you: the error naming the path is printed to the terminal every time, on both machines; the process exit status is **0** in every case, so a script watching only the exit status learns nothing; and `bpy.ops.render.render()` returns `{'FINISHED'}` and raises nothing. There is no handler that fires on a failed write and no property on the node recording one, so the printed line is the only signal. Whether the same behaviour holds *inside* Blender's own window was not tested and is not settled here — the rented machine has no display server and the Creator ran nothing this round.

**Added by the second batch of hand tests, 2026-08-09.** One thing that sharpens the exit-status point above. A run that could not read the blend file at all — the same scene, started as a user who cannot enter the folder it sits in — ends with `Error: Cannot read file "…": Permission denied` and **exit status 1**. So the exit status is not simply always 0: it does report a blend file Blender could not open, and it does not report a File Output write that failed. A script watching only the exit status will therefore see the difference between "the job never started" and "the job ran", and will see no difference at all between "every file was written" and "a file could not be written".

The original note from the sources follows.

The sources do not say. Nothing I found describes write-failure behaviour at all, so there is nothing to hold or not hold when Blender is driven from outside its own window. Two related facts, and they are only that:
- The command line documentation lists `-q, --quiet`, described as "Suppress status printing (warnings & errors are still printed)" — so warnings and errors do reach the console output of a Blender run started from outside, in general. It does not say a File Output write failure is among them.
- It also lists `--debug-exit-on-error`, "Immediately exit when internal errors are detected." It does not say whether a failed file write counts as an internal error.
Source for both: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

The documented path-template failure quoted above is worded around a person seeing a red field and a tooltip, which is an inside-the-window signal; the same page's other sentence ("rendering an animation will fail with an error message") is not tied to the window, but it does not say what that message is or where it goes when nobody is watching.

### Operating system

**Brought up to date by hand test, 2026-08-09: observed on two operating systems, untested on the third.** Every test above ran on Linux (Ubuntu 22.04.5 on the rented machine) and on macOS 26.5.2 on Apple silicon (the Creator's Mac). On everything the question asks the two agreed: which files survive, which do not, that the render carries on, that an error naming the path is printed, and that the exit status is 0. They differed only in wording and in one detail on disk — the bracketed reason was `(Invalid argument)` on Linux and `(Is a directory)` on the Mac; and when a volume filled, Linux left **zero-byte files** for the outputs that could never be opened where the Mac left **no file at all**. **Windows is untested**, not answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Added by the second batch of hand tests, 2026-08-09.** One part of that last difference turns out to belong to the disk's own file system rather than to the operating system, and the first batch did not record which file system the Mac's disk image carried. Re-run on the Creator's Mac on a 12 MB **HFS+** image and again on a 12 MB **APFS** image, both filled to leave exactly 1500 KB free, the volume-full test left **one** part-written file on HFS+ and **two** on APFS, and `df -k` afterwards read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS even though the writes had failed for want of space. What did hold on both Mac file systems is the thing the line above rests on: no zero-byte file was left for an output that could never be opened, where the rented Linux machine leaves one. So the Linux-against-macOS difference stands; the count of part-written files and the `df` reading are properties of the file system as well.

The original note from the sources follows.

The sources do not say. No page I found ties any of this to a particular operating system, and equally, no page states that it is the same everywhere. I found no operating-system note anywhere near the File Output node, the render output settings, or the path templates pages.

---

## Where I looked

I searched the whole of the manual, the whole of the Python API documentation, all of the release notes, and all of the supplied Cycles files at once, rather than opening likely pages, and then opened pages the searches turned up.

**Terms searched across all four sources** (case-insensitive unless noted): "cannot be written", "unable to write", "fails to write", "write error", "failed to write", "cannot write", "cannot save", "cannot open", "could not", "unable to", "disk full", "out of disk", "no space", "full disk", "quota", "storage", "permission denied", "write permission", "writable", "not writable", "unwritable", "read-only", "read only", "partial file", "partially written", "truncated", "incomplete", "corrupt", "empty file", "zero-byte", "leftover", "I/O", "IO error", "network drive", "network share", "unreachable", "disconnect", "will fail", "fail with", "failure", "fails", "abort", "aborts", "aborted", "interrupt", "cancel", "stops rendering", "stop rendering", "rendering will", "render will", "will be aborted", "halts", "skips", "skipped", "silently", "not saved", "wasn't saved", "exit code", "return code", "error message", "reports an error", "report an error", "does not exist", "non-existent", "nonexistent", "create the directory", "missing directory", "Saved:", "placeholder", "template".

The only search hits that bore on the question at all were the path-template ones quoted above. "Failed to write" appears once in the Cycles files, in `osl.py`, and is about compiling a shader script, not about image output — ruled out.

**Pages opened in full and what I ruled out:**
- File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html. Describes the node, its base path, its file format and its inputs. No mention of anything going wrong. This is the obvious page and it is silent.
- File Paths (path templates) — https://docs.blender.org/manual/en/4.5/files/file_paths.html. The one page with a documented failure, quoted above.
- Output properties — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html. Covers file path, Overwrite, Placeholders ("Create empty placeholder frames while rendering"), formats and encoding. Overwrite and Placeholders are about not redoing frames and about a shared network folder; neither says what happens when a write fails, and both are settings of the scene's render output, not of a File Output node. Ruled out.
- Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html. Advises rendering to still images rather than straight to video because "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning" (that sentence is on the Output properties page). Both pages talk about recovering after a stopped render, never about a write that fails. Ruled out.
- Output Introduction — https://docs.blender.org/manual/en/4.5/render/output/introduction.html. Only about a render job being cancelled by the person and resumed. Ruled out.
- Compositor System — https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html. Describes data types, domains, and which output node is chosen. No failure handling. Ruled out.
- Compositor Limitations — https://docs.blender.org/manual/en/4.5/compositing/limitations.html. Only graphics-hardware limits. Ruled out.
- Compositor Sidebar — https://docs.blender.org/manual/en/4.5/compositing/sidebar.html. Backdrop and performance options. Ruled out.
- Info editor — https://docs.blender.org/manual/en/4.5/editors/info_editor.html, and Status Bar — https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html. Where messages appear, quoted above; neither mentions image writing.
- Command Line Arguments — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html. Read the render options and the whole debug and logging section. Quoted the two relevant lines above. Nothing about output failure.
- Troubleshooting section — https://docs.blender.org/manual/en/4.5/troubleshooting/ and its pages (crash, recover, startup, python, network, 3d_view, graphics hardware). All about crashes, startup and drivers. Nothing about output files failing to write. Ruled out.

**API documentation pages opened:**
- https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — lists the node's settings only. No failure behaviour.
- https://docs.blender.org/api/4.5/bpy.ops.render.html — the render operators carry no error description.
- https://docs.blender.org/api/4.5/bpy.app.handlers.html — the render-related hooks are `render_init`, `render_pre`, `render_post`, `render_write` ("on writing a render frame (directly after the frame is written)"), `render_stats`, `render_complete`, `render_cancel`. There is no hook for a write that failed. Note the contrast: for blend-file saving there *is* a failure hook, `save_post_fail`, "on failure to save a blend file (after)". For rendering there is no equivalent. I stop at reporting that the list contains no such hook; drawing any conclusion about what happens on failure from its absence would be my invention, not the source's.
- https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — `frame_path` and `use_placeholder` ("Create empty placeholder files while rendering frames (similar to Unix 'touch')"). Neither covers failure.

**Release notes read:** the compositor, core, python API, rendering, pipeline/assets/IO, and bugfixes pages for 4.5. The compositor notes list added, changed and removed nodes and options, with nothing on failure handling. The bugfixes list contains two entries that mention this area — "Placeholder output files wrong name" and "File output node crash in eevee when in preview mode and without connections" — but neither describes what happens when a write cannot be finished, so neither answers the question.

**Cycles files:** the supplied Cycles folder contains its Python files and a `source` folder holding only `kernel` and `util`. There is no file in it that deals with writing compositor output. I searched its Python files for error reporting and found only shader-compilation and denoising messages, which are unrelated. Ruled out.

**On the picture front:** the File Output node page carries one picture, of the node itself; it shows the node, not any failure. The path-templates page carries the two pictures described above, and I opened them.

---

## Things a person with Blender 4.5 open could settle in a few minutes

This list has been pruned on 2026-08-09. Most of what the researcher proposed was run on both machines and the results are written in above: a folder that cannot be written to; one output of several failing while the rest succeed; a write that fails part-way because the volume fills; whether an animation carries on; the same tests with no window, with the exit status read; two File Output nodes with one failing; Multi-Layer EXR against separate images; a broken path template in a node's own path; and whether a script driving the render can detect any of it. What is left is only what is genuinely still open.

1. **A network folder pulled away mid-render.** Point one File Output node at a mounted network folder and another at a local folder. Start an animation render, and disconnect the network while it is running. Note whether Blender stops, hangs, or continues; what appears in the terminal; and what is left at both locations afterwards. **Could not be run on 2026-08-09**, and the reason is equipment: this needs a file server between two machines, and the project has the Creator's Mac and one rented Linux machine reached over the open internet, with nothing set up to serve a share from either to the other. The network cause of a failed write is therefore untested rather than answered.

2. **A file held open and locked by another program.** **Could not be run on 2026-08-09**: neither macOS nor Linux enforces that kind of lock against an unrelated program, and no Windows machine was tried, where it would be arrangeable.

3. **The same tests on Windows.** **Not run on 2026-08-09**: no Windows machine has run it. Linux and macOS on Apple silicon are both settled above; Windows is untested, not answered.

4. **What a person sees on screen.** Whether the path field of a File Output node with a broken template is shown highlighted in red and what its tooltip says; whether the node shows a warning badge in the compositor after a failed write; and what the Info editor and the status bar hold. **Dropped on 2026-08-09 as needing a window**: the rented Linux machine has no display server, and the Creator ran nothing this round. Before dropping it, the node's own property list was read from a terminal and holds nothing carrying warning text, and 4.5 has no `bpy.context.window_manager.reports`, so there is no way to read any of this without a window.

---

2026-08-09 — Opus 5 (Claude Code) — File output nodes, Blender 4.5

# File output nodes — later questions (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual, Blender 5.0 Python API documentation, Blender 5.0 release notes, and the Cycles add-on files, all as supplied in `Blender Sources/` and all taken on 2026-07-30. I read them on 2026-08-09.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 15 was assigned to me. The rest of the sheet is not answered here.

---

## 15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?

**Label: observed** — settled by hand test on 2026-08-09. The sources remain silent, and the record of that silence is kept below, because it is what sent this question to the hand tests.

### What the hand tests found — Blender 5.0, 2026-08-09

Every test below was run on both of the project's machines, and each is written up as two results.

**The rented Linux machine.** Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. Blender 5.0.1 at `/opt/blender/5.0/blender`.

**The Creator's Mac.** MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.

Every run added `--factory-startup` and pointed `BLENDER_USER_RESOURCES` at a throwaway folder made just before it.

**What was set up.** A scene built by script with one File Output node whose Directory was a writable folder and whose File Name was `shot`, holding three outputs named `aaa`, `bbb` and `ccc`, each fed from the Render Layers node's Image output; PNG, 64 by 64, Cycles on the CPU at one sample. The scene's own render output path was a *different* folder, so the node's files and the scene's own file could be told apart. Rendered with `<blender> --factory-startup -b <file>.blend -f 1`, followed by `echo $?`.

Two things about the node itself had to be established before any of this would run, and they are recorded because the words that ran are what the record has to show. The compositor is reached through `scene.compositing_node_group`, which is empty until a `CompositorNodeTree` is made and assigned to it. And **the node's format starts at Multi-Layer EXR**: `HT_BUILD_DEFAULT_MEDIA_TYPE MULTI_LAYER_IMAGE`, `HT_BUILD_DEFAULT_FILE_FORMAT OPEN_EXR_MULTILAYER`, and `file_format` will not accept `PNG` until `media_type` is set to `IMAGE` first.

**The control run, both machines.** Three files from the node and one from the scene output, a `Saved:` line for each, exit status 0. On the rented Linux machine, word for word from `Compositing` onward:

```
00:00.395  render           | Fra: 1 | Compositing
00:00.541  render           | Saved: '/root/ht-fon/e2-5.0/out/shotccc0001.png'
00:00.546  render           | Saved: '/root/ht-fon/e2-5.0/out/shotaaa0001.png'
00:00.548  render           | Saved: '/root/ht-fon/e2-5.0/out/shotbbb0001.png'
00:00.549  render           | Saved: '/root/ht-fon/e2-5.0/scene/f0001.png'
00:00.549  render           | Time: 00:00.26 (Saving: 00:00.00)
```

The node names its files from the File Name glued to the output's own name, then the frame number: `shotaaa0001.png`. The Mac gave the same names and the same four lines.

**One output of three blocked, by a folder standing where its file goes.** A directory was made at `shotbbb0001.png`, which refuses the write even to `root`. The directory was still an empty directory afterwards, so the block bit. On the rented Linux machine:

```
00:00.503  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/ht-fon/e3-5.0/out/shotbbb0001.png"
00:00.503  reports          | ERROR Render error (Success) cannot save: '/root/ht-fon/e3-5.0/out/shotbbb0001.png'
00:00.530  render           | Saved: '/root/ht-fon/e3-5.0/out/shotccc0001.png'
00:00.536  render           | Saved: '/root/ht-fon/e3-5.0/out/shotaaa0001.png'
00:00.537  render           | Saved: '/root/ht-fon/e3-5.0/scene/f0001.png'
```

On the Creator's Mac the same run gave the same two lines with a different reason in the brackets:

```
00:00.612  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e3-5.0/out/shotbbb0001.png"
00:00.612  reports          | ERROR Render error (Undefined error: 0) cannot save: '<HT>/e3-5.0/out/shotbbb0001.png'
```

**The two machines agreed** on everything the question asks: `aaa` and `ccc` were written at the same byte size as in the control, nothing was left for `bbb`, the scene's own output was written, an error naming the path appeared, the render finished, and `echo $?` gave 0. They disagreed only on the words inside the brackets — `(Success)` on the rented Linux machine, `(Undefined error: 0)` on the Mac.

**Across a five-frame animation.** Same block, rendered with `-a`. The control run with nothing blocked printed 20 `Saved:` lines and left five files each of `aaa` and `ccc`. The blocked run printed 19 — one fewer, the missing one being `bbb` for frame 1 — reached frame 5, and wrote `bbb` normally for frames 2 to 5, because only frame 1's name was occupied. The error appeared once, not once per frame. Exit status 0. Same on both machines.

**A folder that refuses writing, for a user who is not root.** On the rented Linux machine this was run as the user `tester`, because root writes into a read-only folder without complaint; on the Mac as the ordinary user. In both cases `touch` into the folder was refused with `Permission denied` before Blender was run, so the condition really bit. Where the whole node's folder was read-only and a second File Output node pointed at a writable one, on the rented Linux machine:

```
00:00.934  render           | Saved: '<HT>/e6a-5.0/good/shotddd0001.png'
00:00.936  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.0/blocked/shotaaa0001.png"
00:00.936  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.0/blocked/shotaaa0001.png'
00:00.941  render           | Saved: '<HT>/e6a-5.0/good/shoteee0001.png'
00:00.942  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.0/blocked/shotbbb0001.png"
00:00.942  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.0/blocked/shotbbb0001.png'
00:00.943  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.0/blocked/shotccc0001.png"
00:00.943  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.0/blocked/shotccc0001.png'
00:00.944  render           | Saved: '<HT>/e6a-5.0/scene/f0001.png'
```

All three of that node's outputs failed; the second File Output node wrote both of its files; the scene output was written; the blocked folder was empty afterwards; exit status 0. The Mac gave the same, with `(Undefined error: 0)` in the brackets.

**One output of three inside a read-only folder could not be set up in 5.0**, and that is recorded rather than left as a silence: an output's name will not carry a folder in this version (see the naming test below), and an individual output has no folder of its own, so one output of several cannot be aimed at a separate place. Only the whole-node case above could be run here. The same test was run in the other three versions to the same script.

**An output's name carrying a folder.** With the three outputs named `aaa`, `sub/bbb`, `ccc` and no `sub` folder existing beforehand, the slash did not survive being set at all. The build reported the three outputs back as `['aaa', 'sub_bbb', 'ccc']` — Blender turned the slash into an underscore — and no folder was made: the file came out as `shotsub_bbb0001.png` in the node's own folder. Both machines gave the same result. This was run in all four versions to the same script, so this version's behaviour is a measurement rather than an absence.

**Storage running out part-way through the write.** Three outputs, OpenEXR at 32-bit with no compression, 2000 by 2000, aimed at a volume with about 1.5 MB free — on the rented Linux machine the 56 MB filesystem at `/root/small` filled with `dd if=/dev/zero of=/root/small/filler bs=1048576 count=50`, on the Mac a 12 MB disk image filled with `count=9`. The same node writing to an ordinary disk produced complete files of **64,032,676 bytes** on the rented Linux machine and **64,032,782 bytes** on the Mac, and `df` showed the small volume at 0 available and 100% used after each blocked run.

On the rented Linux machine:

```
image.openexr    | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "/root/small/out/shotbbb0001.exr". No space left on device.
render           | Saved: '/root/small/out/shotbbb0001.exr'
image.openexr    | ERROR IMB_exr_begin_write: Cannot open image stream "/root/small/out/shotccc0001.exr". No space left on device.
reports          | ERROR Error writing render result, No space left on device (see console)
reports          | ERROR Render error (No space left on device) cannot save: '/root/small/out/shotccc0001.exr'
image.openexr    | ERROR IMB_exr_begin_write: Cannot open image stream "/root/small/out/shotaaa0001.exr". No space left on device.
reports          | ERROR Error writing render result, No space left on device (see console)
reports          | ERROR Render error (No space left on device) cannot save: '/root/small/out/shotaaa0001.exr'
render           | Saved: '<HT>/e7-5.0/scene/f0001.png'
```

and what was left on the volume:

```
-rw-r--r-- 1 root root       0 Aug  9 14:38 shotaaa0001.exr
-rw-r--r-- 1 root root 4870144 Aug  9 14:38 shotbbb0001.exr
-rw-r--r-- 1 root root       0 Aug  9 14:38 shotccc0001.exr
```

On the Creator's Mac the same test left a single part-written file and nothing else:

```
image.openexr    | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "<HT>/smallmnt/out/shotaaa0001.exr". No space left on device.
render           | Saved: '<HT>/smallmnt/out/shotaaa0001.exr'
image.openexr    | IMB_exr_begin_write: ERROR: No space left on device.
reports          | ERROR Error writing render result, No space left on device (see console)
reports          | ERROR Render error (No space left on device) cannot save: '<HT>/smallmnt/out/shotbbb0001.exr'
```

```
-rw-r--r--@ 1 artist     staff  2813952 Aug  9 16:41 shotaaa0001.exr
```

*(Corrected by the citation check, 2026-08-09: the third line above is written `image.openexr | IMB_exr_begin_write: ERROR: No space left on device.` The repeat on the Creator's Mac gave the same words in this version's own log format, `image.openexr    | ERROR IMB_exr_begin_write: No space left on device.`, which is the arrangement this document uses correctly a few entries further down. The original line is left as written; this note records the difference. Everything else in this entry reproduced word for word.)*

**The two machines agreed** that one file is left **part-written** on disk — 4,870,144 bytes on the rented Linux machine and 2,813,952 bytes on the Mac, against a complete file of 64,032,676 and 64,032,782 bytes respectively — that **Blender prints a `Saved:` line for that part-written file**, that the outputs which could not be started are not written, that the scene's own output on the ordinary disk is written in full, that the render finishes, and that the exit status is 0. They disagreed on two details: the rented Linux machine left **zero-byte files** for the two that could not be opened, where the Mac left **no file at all**; and the Linux message names the path it could not open (`Cannot open image stream "…"`) where the Mac message does not.

**Two File Output nodes, one unable to write at all.** The second node was aimed inside a path that cannot be a folder, because a plain file sat there; that path was still a plain file of zero bytes afterwards. The healthy node wrote all three of its files, the scene output was written, exit status 0, on both machines. On the rented Linux machine:

```
render           | Saved: '<HT>/e8-5.0/out/shotccc0001.png'
image            | ERROR Couldn't create directory for file <HT>/e8-5.0/notadir/inside/shoteee0001.png: Not a directory
reports          | ERROR Render error (Not a directory) cannot save: '<HT>/e8-5.0/notadir/inside/shoteee0001.png'
image            | ERROR Couldn't create directory for file <HT>/e8-5.0/notadir/inside/shotddd0001.png: Not a directory
reports          | ERROR Render error (Not a directory) cannot save: '<HT>/e8-5.0/notadir/inside/shotddd0001.png'
render           | Saved: '<HT>/e8-5.0/out/shotaaa0001.png'
render           | Saved: '<HT>/e8-5.0/out/shotbbb0001.png'
render           | Saved: '<HT>/e8-5.0/scene/f0001.png'
```

The Mac gave the same lines. So one node failing does not stop another node in the same tree.

**Multi-Layer EXR.** With the node's Media Type set to `MULTI_LAYER_IMAGE`, the node wrote **exactly one file** for the frame — `shot0001.exr` — not three, on both machines. So in that mode there are no other files from that node for that frame. With a directory standing at that one name, nothing at all was left for the node, the scene's own output was still written, and the exit status was 0. On the rented Linux machine:

```
image.openexr    | ERROR IMB_exr_begin_write: Is a directory.
reports          | ERROR Error writing render result, Is a directory (see console)
reports          | ERROR Render error (Is a directory) cannot save: '<HT>/e9-5.0/out/shot0001.exr'
render           | Saved: '<HT>/e9-5.0/scene/f0001.png'
```

On the Mac the first of those three lines read `ERROR IMB_exr_begin_write: Is a directory.`; the other two matched.

**A broken path template in the node's own path.** The control, a valid `{blend_name}_`, was substituted for the blend file's name on both machines — from `t.blend` the files came out as `t_aaa0001.png` when the template was in the node's File Name, and `shott_aaa0001.png` when it was in one output's own name — so path templates are live in 5.0 and a broken one failing is not "templates are switched off".

With the unclosed `{blend_name_` in **one output's own name**, that one output was skipped and the node's other two were written, alongside a second File Output node which wrote both of its files, the scene output, and exit status 0. One line appeared:

```
reports          | ERROR Invalid path template in File Output node. Skipping writing file.
```

With the unclosed template in the node's **File Name**, which every one of that node's outputs goes through, all three of that node's outputs were skipped — the same line three times — while the second node still wrote both of its files and the scene output was written. The render was not refused and did not stop; over five frames it ran to the end. Exit status 0. Identical on both machines.

**What a script driving the render is handed.** On the blocked scene, `bpy.ops.render.render(animation=False, write_still=True)` returned **`{'FINISHED'}`** and raised nothing, on both machines, while the same run printed the error naming the blocked file:

```
image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e11b-5.0/out/shotbbb.png"
reports          | ERROR Render error (Success) cannot save: '<HT>/e11b-5.0/out/shotbbb.png'
render           | Saved: '<HT>/e11b-5.0/out/shotccc.png'
render           | Saved: '<HT>/e11b-5.0/out/shotaaa.png'
render           | Saved: '<HT>/e11b-5.0/scene/f.png'
VS_RENDER_BEGIN
VS_RENDER_RETURN {'FINISHED'}
VS_RENDER_END
```

Recorded because it caught the test out once: driven through `bpy.ops.render.render()` rather than `-f 1`, this version names the file `shotbbb.png` with **no frame number**, so the first run of this test blocked a name that was never used and all three files were written. The test was rewritten to render once unblocked through the same route, read the name that route actually gives, and only then place the block; the run above is the corrected one.

**What the node and the handler list carry.** Read off a running Blender on both machines, the File Output node's full property list is `file_output_items`, `active_item_index`, `directory`, `file_name`, `format`, `save_as_render`, plus the general node properties including `warning_propagation`. **There is no property reporting whether a write succeeded or failed, and none naming the last path written.** The handler list holds `composite_cancel`, `composite_post`, `composite_pre`, `load_post_fail`, `render_cancel`, `render_complete`, `render_init`, `render_post`, `render_pre`, `render_stats`, `render_write`, `save_post`, `save_post_fail`, `save_pre`. There is a failure hook for loading a blend file and one for saving one, and **no failure hook for writing a render frame**. That is an absence in a list demonstrably being read, since the same listing returned the settings and returned `save_post_fail`.

`bpy.context.window_manager.reports` does not exist in 5.0 — `PROBE_WM_HAS_REPORTS False` on both machines — so there is no way to read Blender's own report log from a terminal in this version.

### A second batch of hand tests, sent back by the citation check — Blender 5.0, 2026-08-09

The citation check of this document re-ran every test above and sent three of them back — not because an answer was wrong, but because three written recipes had gaps a fresh pair of hands would trip on. All three were run again by me on 2026-08-09, on the machines each names, with `--factory-startup` and a fresh throwaway settings folder every run as before. **Nothing here changes any answer to question 15.** My working folders were `/root/b2-fon` on the rented Linux machine and a throwaway folder in my own scratch space on the Creator's Mac, written `<B2>` below; both were removed afterwards, and the rented machine's shared 56 MB filesystem was not needed by any of the three and was left as found.

**1. The read-only folder test, and where the test tree has to sit.** The record above says this ran as the user `tester` on the rented Linux machine but not where the files were. It matters: `/root` is `drwx------`, so with the tree under `/root` the run as `tester` never reaches the compositor at all —

```
00:00.274  reports          | ERROR Cannot read file "/root/b2-fon/e13trap-5.0/t.blend": Permission denied
```

— with exit status **1**, which is a test that did not run rather than a result. Re-run with the whole tree (blend file, both output folders, scene output folder and throwaway settings folder) at `/tmp/b2-e13-5.0`, which `tester` can read and write, the earlier result comes back unchanged. Both controls held first, before Blender was started: `touch` into the blocked folder was refused (`touch: cannot touch '/tmp/b2-e13-5.0/blocked/canary': Permission denied`) while `touch` into the second node's folder succeeded, so the block bit and the user was not simply unable to write anywhere. Rented Linux machine:

```
00:00.638  render           | Saved: '/tmp/b2-e13-5.0/good/shotddd0001.png'
00:00.639  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.0/blocked/shotaaa0001.png"
00:00.639  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.0/blocked/shotaaa0001.png'
00:00.645  render           | Saved: '/tmp/b2-e13-5.0/good/shoteee0001.png'
00:00.646  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.0/blocked/shotbbb0001.png"
00:00.646  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.0/blocked/shotbbb0001.png'
00:00.646  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.0/blocked/shotccc0001.png"
00:00.646  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.0/blocked/shotccc0001.png'
00:00.647  render           | Saved: '/tmp/b2-e13-5.0/scene/f0001.png'
```

Exit status 0; the blocked folder empty afterwards; the second node's two files written at 15,741 bytes each; the scene's own output written. On the Creator's Mac, with the tree at `<B2>/e13-5.0` and the blocked folder at `chmod 500`, the same run gave the same lines with `(Undefined error: 0)` in the brackets:

```
00:00.451  render           | Saved: '<B2>/e13-5.0/good/shotddd0001.png'
00:00.451  image.write      | ERROR OpenImageIO write failed: Could not open file "<B2>/e13-5.0/blocked/shotaaa0001.png"
00:00.451  reports          | ERROR Render error (Undefined error: 0) cannot save: '<B2>/e13-5.0/blocked/shotaaa0001.png'
00:00.452  render           | Saved: '<B2>/e13-5.0/good/shoteee0001.png'
00:00.452  image.write      | ERROR OpenImageIO write failed: Could not open file "<B2>/e13-5.0/blocked/shotbbb0001.png"
00:00.452  reports          | ERROR Render error (Undefined error: 0) cannot save: '<B2>/e13-5.0/blocked/shotbbb0001.png'
00:00.453  image.write      | ERROR OpenImageIO write failed: Could not open file "<B2>/e13-5.0/blocked/shotccc0001.png"
00:00.453  reports          | ERROR Render error (Undefined error: 0) cannot save: '<B2>/e13-5.0/blocked/shotccc0001.png'
00:00.453  render           | Saved: '<B2>/e13-5.0/scene/f0001.png'
```

Exit status 0, blocked folder empty, both of the second node's files written, scene output written.

**2. What the complete-file byte size actually depends on.** The figure of **64,032,676 bytes** quoted above for a complete OpenEXR file is not a fixed measurement, and this test says what moves it. The same scene was rendered three times on each machine, changing exactly one thing at a time. On the rented Linux machine, with the blend file at `/root/b2-fon/e14-5.0/t.blend` (28 characters) the three complete files came to **64,032,674 bytes** each; with the blend file moved to `…/ttttttttttt.blend` (38 characters, nothing else changed) they came to **64,032,684** — ten characters more in the path, ten bytes more in the file. With the blend path held fixed and the *output folder* lengthened by ten characters instead, the size did not move at all (64,032,675 both times). The blend file's own path is inside each image; `grep -a -o` finds it word for word. On the Creator's Mac the same pattern: a 134-character blend path gave **64,032,780 bytes**, a 144-character one **64,032,790**, and lengthening the output folder changed nothing.

Subtracting the blend path's length from the figure leaves **64,032,646** on both machines. So the roughly 106-byte gap this document records between the rented Linux machine and the Mac is the gap between the two working-folder names, not a difference between the machines. Neither the figure above nor the citation check's repeat of 64,032,673 is being corrected — I was not in the room for either run — but the number as printed cannot be checked by anyone whose folders are named differently, and the point it carries here (a complete file of about 64 MB against roughly 5 MB left part-written) is untouched.

**3. What the disk image's file system changes when the volume fills.** The citation check noted that the record above does not say what file system the Mac's 12 MB disk image carried. Re-run on the Creator's Mac on both, with the file system written down from `hdiutil create … -fs HFS+` / `-fs APFS` and confirmed by `diskutil info` on the mounted volume (`File System Personality: HFS+`, `Type (Bundle): hfs`; and `APFS` / `apfs`), each filled to leave exactly 1500 KB free by `df`, 5.0 printed the same wording on both file systems — and it does name the file and the reason:

```
00:27.191  image.openexr    | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "<B2>/mnt-HFS-5.0/out/shotbbb0001.exr". No space left on device.
00:27.191  render           | Saved: '<B2>/mnt-HFS-5.0/out/shotbbb0001.exr'
00:27.191  image.openexr    | ERROR IMB_exr_begin_write: No space left on device.
00:27.191  reports          | ERROR Error writing render result, No space left on device (see console)
00:27.191  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-HFS-5.0/out/shotccc0001.exr'
00:27.191  image.openexr    | ERROR IMB_exr_begin_write: No space left on device.
00:27.191  reports          | ERROR Error writing render result, No space left on device (see console)
00:27.191  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-HFS-5.0/out/shotaaa0001.exr'
00:27.524  render           | Saved: '<B2>/e12-HFS-5.0/scene/f0001.png'
```

Exit status 0 both times, and the scene's own output written on the ordinary disk both times. **But what is left behind is not the same on the two file systems.** On HFS+ the volume ended with one part-written file, `shotbbb0001.exr`, of **1,536,000 bytes**. On APFS, from the same 1500 KB free, it ended with **two** part-written files — `shotbbb0001.exr` at 434,577 bytes and `shotccc0001.exr` at 86,417 bytes — with a second `IMB_exr_write_channels: Failed to write pixel data…` line to match. And the two file systems do not even report the volume the same way: after the run `df -k` read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS, although the writes had plainly failed for want of space.

So any figure for what a full volume leaves behind has to say which file system it was measured on. This document's "a single part-written file" matches HFS+ and not APFS, and the document does not say which it was.

**Nothing in this batch opens anything on the list of things still needing a person at a computer.** The network folder, the locked file, Windows and the checks needing a window are all untouched by these three tests and stay as they are below.

### What the hand tests did not settle

A network folder taken away part-way through a write could not be run: it needs a file server between the two machines, which the project does not have. A file held open and locked by another program could not be run: neither macOS nor Linux enforces that kind of lock. Nothing was run on Windows: no Windows machine has run it, so that operating system is untested rather than answered. Whether the path field is shown highlighted in red and what its tooltip says, and whether the node shows a warning badge in the compositor, both need eyes on a window and were dropped this round — the rented machine has no display server and the Creator ran nothing; the node property list was checked first and holds nothing carrying warning text, so neither can be read from a terminal instead.

---

### What the sources say

**Label for the documentation on its own: not in the sources.**

Nothing in the Blender 5.0 manual, the Python API documentation, the 5.0 release notes, or the Cycles files says what happens when a File Output node starts writing one of its files for a frame and cannot finish it. The sources describe what the node writes and how the names are built; they say nothing about a write that goes wrong. Specifically, none of them state:

- whether the file that failed is left behind at part length, left behind empty, or removed;
- whether the node's other files for that same frame are still written, or skipped;
- whether the render stops at that frame, carries on to the next frame, or finishes and reports afterwards;
- whether anything is reported at all, where it would appear, or what it would say.

The page that describes the node in full is silent on all of this. Its description of what the node does reads:

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence."

Source: manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30 (snapshot date).

The node's API page is equally silent. Its whole description of the class is:

> "Write image file to disk"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30 (snapshot date).

### The one nearby thing the sources do say, and why it is not an answer

There is exactly one documented case of a render output path going wrong, and it is a different case from the one asked about. It concerns a path that cannot be worked out in the first place, not a write that begins and cannot be completed.

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Note
>
> Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30 (snapshot date).

The same page confirms these templates apply to the File Output node's paths:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

Source: manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30 (snapshot date).

I also looked at the picture on that page showing the tooltip for a broken path: `_images/files_file_paths-invalid_path_template_tooltip.png`, on https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30 (snapshot date). What it carries is a field outlined in red, with a tooltip reading "Syntax error(s):" and beneath it "- Invalid or incomplete template expression '{blend_name.png'." Label for that much: **documented**, since the picture is part of the manual.

Two further steps are mine, and both are **inferred**: that the field is the scene's own Output path rather than one of the node's, and that the warning comes while editing rather than during a render. A picture shows no timing, and neither step is written in words on the page.

Why this does not answer the question: the quoted note is about the render output path in Scene properties, and the "fail" it describes happens because the path could not be turned into a filename at all. The manual does not say the same for the File Output node's own paths — it only says "Depending on the path, template errors may prevent certain actions", and leaves which actions unstated for this node. And in the question's case the path is fine; the writing is what fails. Reading the note across to that case would be my step, not the manual's, so I do not take it.

### One further absence worth naming, since it is where a reader would look next

The Python API lists a handler that runs when a render frame has been written, and a handler that runs when saving a blend-file has failed. I read the handler entries for rendering and for saving; none of those covers a render frame that failed to be written. I did not read every entry on that page, so this is a statement about the entries I read, not about the whole list:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written)"

> "bpy.app.handlers.save_post_fail
>
> on failure to save a blend file (after). Accepts one argument: the file being saved, an empty string for the startup-file."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30 (snapshot date).

I report this as a gap in what I read, not as a finding about behaviour. Not finding such a handler does not tell you what Blender does when a write fails, and I draw no conclusion from it.

### Not covered

This answer settles only that the sources are silent. It does not settle any of the actual behaviour asked about — not the state of the failed file, not the fate of the sibling files for that frame, not whether the render continues, and not what is reported or where. It also does not distinguish between kinds of failure (no space left, no permission to write, a folder that has gone away, a network location that has dropped, a file already open elsewhere); the sources do not mention any of them, so I cannot say whether Blender treats them alike. Nor does it cover the Multi-Layer EXR case, where the node writes one file holding every input rather than several files. Nothing I read describes what happens when that write cannot be finished either, so the question's premise of "one of several files" only fits the separate-images case.

### Outside the interface

**Brought up to date by hand test, 2026-08-09: observed.** Every test above was run with Blender started from a terminal with no window, which is the only way Smoozy will ever run it, so the whole of the answer above is an answer about Blender driven from outside its own interface. What that route gives you: the error naming the path is printed to the terminal every time, on both machines; the process exit status is **0** in every case, so a script watching only the exit status learns nothing; and `bpy.ops.render.render()` returns `{'FINISHED'}` and raises nothing. There is no handler that fires on a failed write and no property on the node recording one, so the printed line is the only signal. Whether the same behaviour holds *inside* Blender's own window was not tested and is not settled here — the rented machine has no display server and the Creator ran nothing this round.

**Added by the second batch of hand tests, 2026-08-09.** One thing that sharpens the exit-status point above. A run that could not read the blend file at all — the same scene, started as a user who cannot enter the folder it sits in — ends with `Error: Cannot read file "…": Permission denied` and **exit status 1**. So the exit status is not simply always 0: it does report a blend file Blender could not open, and it does not report a File Output write that failed. A script watching only the exit status will therefore see the difference between "the job never started" and "the job ran", and will see no difference at all between "every file was written" and "a file could not be written".

The original note from the sources follows.

The sources do not say. Nothing I found describes File Output failure behaviour at all, so nothing describes it either inside Blender's window or when Blender is started and driven from outside. The one adjacent statement — that a broken path template makes rendering an animation "fail with an error message" — does not say where that message goes when there is no window, and the red highlighting and tooltip beside it are described by the manual as things happening "in the UI", so they say nothing about a run with no interface. The command-line documentation lists general switches for messages (`-q, --quiet`: "Suppress status printing (warnings & errors are still printed)"; `--debug-io`: "Enable debug messages for I/O."; `--debug-exit-on-error`: "Immediately exit when internal errors are detected.") but never connects any of them to output files failing to write. Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30 (snapshot date).

### Operating system

**Brought up to date by hand test, 2026-08-09: observed on two operating systems, untested on the third.** Every test above ran on Linux (Ubuntu 22.04.5 on the rented machine) and on macOS 26.5.2 on Apple silicon (the Creator's Mac). On everything the question asks the two agreed: which files survive, which do not, that the render carries on, that an error naming the path is printed, and that the exit status is 0. They differed only in wording and in one detail on disk — the bracketed reason was `(Success)` on Linux and `(Undefined error: 0)` on the Mac; and when a volume filled, Linux left **zero-byte files** for the outputs that could never be opened where the Mac left **no file at all**. **Windows is untested**, not answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Added by the second batch of hand tests, 2026-08-09.** One part of that last difference turns out to belong to the disk's own file system rather than to the operating system, and the first batch did not record which file system the Mac's disk image carried. Re-run on the Creator's Mac on a 12 MB **HFS+** image and again on a 12 MB **APFS** image, both filled to leave exactly 1500 KB free, the volume-full test left **one** part-written file on HFS+ and **two** on APFS, and `df -k` afterwards read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS even though the writes had failed for want of space. What did hold on both Mac file systems is the thing the line above rests on: no zero-byte file was left for an output that could never be opened, where the rented Linux machine leaves one. So the Linux-against-macOS difference stands; the count of part-written files and the `df` reading are properties of the file system as well.

The original note from the sources follows.

The sources do not say. No page I found ties File Output writing, or its failure, to any particular operating system, and no page states that the behaviour is the same on all of them. Since the behaviour itself is undocumented, there is nothing whose portability could be reported.

### Where I looked, so that "not in the sources" can be checked

Everything below was searched across the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once, case-insensitively, unless stated otherwise.

Searches for the failure itself: "cannot be written", "cannot be saved", "cannot be created", "unable to write / save / create / open", "failed to write / save / create / open", "could not write / save / open / create", "not be saved", "saving failed", "write error", "disk full", "out of disk space", "no space".

Searches for what a half-finished file might be called: "truncat", "corrupt", "partial write", "partially written", "partially saved", "incomplete file", "zero-byte", "empty file", "half-written", "rolled back", "atomic", "temporary file", ".tmp", "rename".

Searches for what might happen to the render: "abort", "aborts", "stops the render", "cancels the render", "cancelled the render", "halts", "silently", "skipped", "continues to render", "keeps rendering", "remaining frames", "exit code", "return code", "exit status", "non-zero".

Searches for causes a write could fail on: "permission", "read-only file", "write access", "writable", "access denied", "denied", "quota", "file is locked", "in use by another", "does not exist", "will be created", "automatically created", "non-existent", "invalid path", "bad path", "path error", "disk", "storage space", "free space", "network drive / share / path / location / file system".

Broad sweeps: every occurrence of "error" and of "fail / fails / failed / failure / failing" inside `manual_5.0/render/`, `manual_5.0/compositing/`, `manual_5.0/files/` and `manual_5.0/advanced/`; every occurrence of "file output" across the manual, the API documentation and the release notes.

Pages read rather than only searched:

- File Output node, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read in full. Describes inputs, Directory, File Name, Media Type, Node Format, Images/Layers, Output Paths. No mention of failure. I also opened its picture, `_images/node-types_CompositorNodeOutputFile.webp`. What is visible in it, and so **documented**: a node headed "File Output", a text field reading "/tmp/" with a folder button beside it, a second text field reading "file_name", and one unconnected socket circle on the left edge. Nothing about failure is visible. That the first field is the Directory setting and the second the File Name setting is **inferred** — neither carries a visible label.

*(Changed by the citation check, 2026-08-09: the node was described as "collapsed", which the picture does not show — it is drawn open with both fields visible — and the two fields were named as though the picture labelled them. Corrected against `manual_5.0/_images/node-types_CompositorNodeOutputFile.webp`.)*
- `bpy.types.CompositorNodeOutputFile`, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html — read in full. Lists `directory`, `file_name`, `file_output_items`, `format`, `save_as_render`, `active_item_index`, and inherited members. Nothing about errors, return values or failure.
- Path Templates, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read in full; quoted above. The only page in the manual that ties a render output path to an error.
- Output properties, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read the relevant part. It covers Overwrite ("Overwrite existing files when rendering.") and Placeholders ("Create empty placeholder frames while rendering."). Neither description mentions a write that fails, and I do not say here what either setting is for. Ruled out.
- Rendering Animations, https://docs.blender.org/manual/en/5.0/render/output/animation.html — read in full. Its "Hints" section covers a computer turning off mid-render and recommends image sequences plus disabling Overwrite. It never says what happens to a file that was being written when something went wrong. Ruled out.
- Info Editor, https://docs.blender.org/manual/en/5.0/editors/info_editor.html — read in full, as the place reports would appear. It says only "The Info editor logs the executed operators as well as errors, warnings, and informational messages." It gives no examples and does not mention rendering or file writing. Ruled out as a source for what would be reported.
- Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, and Rendering From The Command Line, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — searched for "error", "fail", "abort", "quit", "exit". Only the general switches quoted above. Neither page mentions output files failing to write, and neither states an exit code for a failed render. Ruled out.
- `bpy.app.handlers`, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read the handler entries for rendering and for saving, not the whole page; quoted above.
- Release notes: the compositor page, https://developer.blender.org/docs/release_notes/5.0/compositor/, has several File Output entries (the redesign, translation clipping, frame-number appending) and none about errors or failures. The Python API page, https://developer.blender.org/docs/release_notes/5.0/python_api/, mentions only the split into `directory` and `file_name`. The pipeline & I/O page, https://developer.blender.org/docs/release_notes/5.0/pipeline_io/, and the rendering page, https://developer.blender.org/docs/release_notes/5.0/rendering/, contain no "file output" or "render output" mentions at all. The bug-fix list, https://developer.blender.org/docs/release_notes/5.0/bugfixes/, has three File Output lines — optional frame numbering, translated images not saved, and OpenEXR images written for render layers that do not render — none of which describes failure handling. All read 2026-07-30 (snapshot date).
- Cycles files: searched `Blender Sources/cycles_5.0` for "file output", "output_file", "cannot write", "error writing", and "write" near "fail". The only hits are at `osl.py` lines 16–18 and at `operators.py` lines 43 and 131. The `operators.py` hits are `output_filepath` settings on Cycles' own operators. None of the hits names the compositor's File Output node. I do not say what any of that code does when it runs — reading a file cannot settle that. Ruled out.

I did not need the web and did not use it.

---

## Things a person with Blender open could settle in a few minutes

This list has been pruned on 2026-08-09. Most of what the researcher proposed was run on both machines and the results are written in above: a folder that cannot be written to; one output of several failing while the rest succeed; a write that fails part-way because the volume fills; whether an animation carries on; the same tests with no window, with the exit status read; two File Output nodes with one failing; Multi-Layer EXR against separate images; a broken path template in a node's own path; and whether a script driving the render can detect any of it. What is left is only what is genuinely still open.

1. **A network folder pulled away mid-render.** Point one File Output node at a mounted network folder and another at a local folder. Start an animation render, and disconnect the network while it is running. Note whether Blender stops, hangs, or continues; what appears in the terminal; and what is left at both locations afterwards. **Could not be run on 2026-08-09**, and the reason is equipment: this needs a file server between two machines, and the project has the Creator's Mac and one rented Linux machine reached over the open internet, with nothing set up to serve a share from either to the other. The network cause of a failed write is therefore untested rather than answered.

2. **A file held open and locked by another program.** **Could not be run on 2026-08-09**: neither macOS nor Linux enforces that kind of lock against an unrelated program, and no Windows machine was tried, where it would be arrangeable.

3. **The same tests on Windows.** **Not run on 2026-08-09**: no Windows machine has run it. Linux and macOS on Apple silicon are both settled above; Windows is untested, not answered.

4. **What a person sees on screen.** Whether the path field of a File Output node with a broken template is shown highlighted in red and what its tooltip says; whether the node shows a warning badge in the compositor after a failed write; and what the Info editor and the status bar hold. **Dropped on 2026-08-09 as needing a window**: the rented Linux machine has no display server, and the Creator ran nothing this round. Before dropping it, the node's own property list was read from a terminal and holds nothing carrying warning text, and 5.0 has no `bpy.context.window_manager.reports`, so there is no way to read any of this without a window.

---

2026-08-09 — Opus 5 (Claude Code) — File output nodes, Blender 5.0

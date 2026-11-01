# File output nodes — later questions (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2 release notes, and the Cycles 5.2 files — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for every citation below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing to declare precisely, so the line above is not doing quiet work: I listed the names of the files in the folder I was told to write into, to confirm the folder existed before writing. I did not open any of them, and nothing in that listing informed any answer below. No other part of the project was listed, searched, or read.

I did not use the web. I did no work through helpers; every search and every reading below is my own.

---

## 15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?

**Label: observed** — settled by hand test on 2026-08-09. The sources remain silent, and the record of that silence is kept below, because it is what sent this question to the hand tests.

### What the hand tests found — Blender 5.2, 2026-08-09

Every test below was run on both of the project's machines, and each is written up as two results.

**The rented Linux machine.** Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. Blender 5.2.0 at `/opt/blender/5.2/blender`.

**The Creator's Mac.** MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics. Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`.

Every run added `--factory-startup` and pointed `BLENDER_USER_RESOURCES` at a throwaway folder made just before it.

**What was set up.** A scene built by script with one File Output node whose Directory was a writable folder and whose File Name was `shot`, holding three outputs named `aaa`, `bbb` and `ccc`, each fed from the Render Layers node's Image output; PNG, 64 by 64, Cycles on the CPU at one sample. The scene's own render output path was a *different* folder, so the node's files and the scene's own file could be told apart. Rendered with `<blender> --factory-startup -b <file>.blend -f 1`, followed by `echo $?`.

Two things about the node itself had to be established before any of this would run, and they are recorded because the words that ran are what the record has to show. The compositor is reached through `scene.compositing_node_group`, which is empty until a `CompositorNodeTree` is made and assigned to it. And **the node's format starts at Multi-Layer EXR**: `HT_BUILD_DEFAULT_MEDIA_TYPE MULTI_LAYER_IMAGE`, `HT_BUILD_DEFAULT_FILE_FORMAT OPEN_EXR_MULTILAYER`, and `file_format` will not accept `PNG` until `media_type` is set to `IMAGE` first.

**The control run, both machines.** Three files from the node and one from the scene output, a `Saved:` line for each, exit status 0. On the rented Linux machine, word for word from `Compositing` onward:

```
00:00.395  render           | Fra: 1 | Compositing
00:00.541  render           | Saved: '/root/ht-fon/e2-5.2/out/shotccc0001.png'
00:00.546  render           | Saved: '/root/ht-fon/e2-5.2/out/shotaaa0001.png'
00:00.548  render           | Saved: '/root/ht-fon/e2-5.2/out/shotbbb0001.png'
00:00.549  render           | Saved: '/root/ht-fon/e2-5.2/scene/f0001.png'
00:00.549  render           | Time: 00:00.26 (Saving: 00:00.00)
```

The node names its files from the File Name glued to the output's own name, then the frame number: `shotaaa0001.png`. The Mac gave the same names and the same four lines.

**One output of three blocked, by a folder standing where its file goes.** A directory was made at `shotbbb0001.png`, which refuses the write even to `root`. The directory was still an empty directory afterwards, so the block bit. On the rented Linux machine:

```
00:00.503  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/ht-fon/e3-5.2/out/shotbbb0001.png"
00:00.503  reports          | ERROR Render error (Success) cannot save: '/root/ht-fon/e3-5.2/out/shotbbb0001.png'
00:00.530  render           | Saved: '/root/ht-fon/e3-5.2/out/shotccc0001.png'
00:00.536  render           | Saved: '/root/ht-fon/e3-5.2/out/shotaaa0001.png'
00:00.537  render           | Saved: '/root/ht-fon/e3-5.2/scene/f0001.png'
```

On the Creator's Mac the same run gave the same two lines with a different reason in the brackets:

```
00:00.612  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e3-5.2/out/shotbbb0001.png"
00:00.612  reports          | ERROR Render error (Undefined error: 0) cannot save: '<HT>/e3-5.2/out/shotbbb0001.png'
```

**The two machines agreed** on everything the question asks: `aaa` and `ccc` were written at the same byte size as in the control, nothing was left for `bbb`, the scene's own output was written, an error naming the path appeared, the render finished, and `echo $?` gave 0. They disagreed only on the words inside the brackets — `(Success)` on the rented Linux machine, `(Undefined error: 0)` on the Mac.

**Across a five-frame animation.** Same block, rendered with `-a`. The control run with nothing blocked printed 20 `Saved:` lines and left five files each of `aaa` and `ccc`. The blocked run printed 19 — one fewer, the missing one being `bbb` for frame 1 — reached frame 5, and wrote `bbb` normally for frames 2 to 5, because only frame 1's name was occupied. The error appeared once, not once per frame. Exit status 0. Same on both machines.

**A folder that refuses writing, for a user who is not root.** On the rented Linux machine this was run as the user `tester`, because root writes into a read-only folder without complaint; on the Mac as the ordinary user. In both cases `touch` into the folder was refused with `Permission denied` before Blender was run, so the condition really bit. Where the whole node's folder was read-only and a second File Output node pointed at a writable one, on the rented Linux machine:

```
00:00.934  render           | Saved: '<HT>/e6a-5.2/good/shotddd0001.png'
00:00.936  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.2/blocked/shotaaa0001.png"
00:00.936  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.2/blocked/shotaaa0001.png'
00:00.941  render           | Saved: '<HT>/e6a-5.2/good/shoteee0001.png'
00:00.942  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.2/blocked/shotbbb0001.png"
00:00.942  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.2/blocked/shotbbb0001.png'
00:00.943  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.2/blocked/shotccc0001.png"
00:00.943  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.2/blocked/shotccc0001.png'
00:00.944  render           | Saved: '<HT>/e6a-5.2/scene/f0001.png'
```

All three of that node's outputs failed; the second File Output node wrote both of its files; the scene output was written; the blocked folder was empty afterwards; exit status 0. The Mac gave the same, with `(Undefined error: 0)` in the brackets.

Where only one output of three sat inside a read-only subfolder — an output named `sub/bbb`, giving the folder `shotsub` — the other two outputs were written, nothing was left in the blocked folder, and the exit status was 0, on both machines.

**An output's name carrying a folder.** With the three outputs named `aaa`, `sub/bbb`, `ccc` and no `sub` folder existing beforehand, the slash was kept, and Blender made the folder and wrote into it. The folder is named from the node's File Name glued to the part before the slash, so `sub/bbb` under a File Name of `shot` produced `Saved: '<HT>/e5-5.2/out/shotsub/bbb0001.png'`. Both machines gave the same result. This was run in all four versions to the same script, so this version's behaviour is a measurement rather than an absence.

**Storage running out part-way through the write.** Three outputs, OpenEXR at 32-bit with no compression, 2000 by 2000, aimed at a volume with about 1.5 MB free — on the rented Linux machine the 56 MB filesystem at `/root/small` filled with `dd if=/dev/zero of=/root/small/filler bs=1048576 count=50`, on the Mac a 12 MB disk image filled with `count=9`. The same node writing to an ordinary disk produced complete files of **64,032,680 bytes** on the rented Linux machine and **64,032,786 bytes** on the Mac, and `df` showed the small volume at 0 available and 100% used after each blocked run.

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
render           | Saved: '<HT>/e7-5.2/scene/f0001.png'
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
image.openexr    | ERROR Unknown error in IMB_exr_begin_write
reports          | ERROR Error writing render result, No space left on device (see console)
reports          | ERROR Render error (No space left on device) cannot save: '<HT>/smallmnt/out/shotbbb0001.exr'
```

```
-rw-r--r--@ 1 artist     staff  2813952 Aug  9 16:41 shotaaa0001.exr
```

*(Disagreement found by the citation check, 2026-08-09, and left unresolved. Repeating this test on the Creator's Mac gave a different **first** line. The run above has `image.openexr | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.` The repeat gave `image.openexr    | ERROR Unknown error in IMB_exr_write_channels`; the same wording came back from the equivalent repeat in 5.1, on both an HFS+ and an APFS disk image. Everything else in this entry reproduced, including `ERROR Unknown error in IMB_exr_begin_write` on the two lines below. Neither run is being picked over the other; the test is on the list to be run again with the disk image's file system written down, which this record does not state. A third run was made on 2026-08-09 and is recorded in full under *A second batch of hand tests* below, on a 12 MB HFS+ disk image and a 12 MB APFS one with the file system written down both times; it gave the same words as the repeat, on both file systems, and all three runs now sit side by side. Still nobody's run is picked over another's.)*

**The two machines agreed** that one file is left **part-written** on disk — 4,870,144 bytes on the rented Linux machine and 2,813,952 bytes on the Mac, against a complete file of 64,032,680 and 64,032,786 bytes respectively — that **Blender prints a `Saved:` line for that part-written file**, that the outputs which could not be started are not written, that the scene's own output on the ordinary disk is written in full, that the render finishes, and that the exit status is 0. They disagreed on two details: the rented Linux machine left **zero-byte files** for the two that could not be opened, where the Mac left **no file at all**; and the rented Linux machine names the path and the reason (`IMB_exr_begin_write: Cannot open image stream "…". No space left on device.`) where the Mac printed only `Unknown error in IMB_exr_begin_write`, though the line after it on the Mac still said `Error writing render result, No space left on device (see console)`.

**Two File Output nodes, one unable to write at all.** The second node was aimed inside a path that cannot be a folder, because a plain file sat there; that path was still a plain file of zero bytes afterwards. The healthy node wrote all three of its files, the scene output was written, exit status 0, on both machines. On the rented Linux machine:

```
render           | Saved: '<HT>/e8-5.2/out/shotccc0001.png'
image            | ERROR Couldn't create directory for file <HT>/e8-5.2/notadir/inside/shoteee0001.png: Not a directory
reports          | ERROR Render error (Not a directory) cannot save: '<HT>/e8-5.2/notadir/inside/shoteee0001.png'
image            | ERROR Couldn't create directory for file <HT>/e8-5.2/notadir/inside/shotddd0001.png: Not a directory
reports          | ERROR Render error (Not a directory) cannot save: '<HT>/e8-5.2/notadir/inside/shotddd0001.png'
render           | Saved: '<HT>/e8-5.2/out/shotaaa0001.png'
render           | Saved: '<HT>/e8-5.2/out/shotbbb0001.png'
render           | Saved: '<HT>/e8-5.2/scene/f0001.png'
```

The Mac gave the same lines. So one node failing does not stop another node in the same tree.

**Multi-Layer EXR.** With the node's Media Type set to `MULTI_LAYER_IMAGE`, the node wrote **exactly one file** for the frame — `shot0001.exr` — not three, on both machines. So in that mode there are no other files from that node for that frame. With a directory standing at that one name, nothing at all was left for the node, the scene's own output was still written, and the exit status was 0. On the rented Linux machine:

```
image.openexr    | ERROR IMB_exr_begin_write: Is a directory.
reports          | ERROR Error writing render result, Is a directory (see console)
reports          | ERROR Render error (Is a directory) cannot save: '<HT>/e9-5.2/out/shot0001.exr'
render           | Saved: '<HT>/e9-5.2/scene/f0001.png'
```

On the Mac the first of those three lines read `ERROR Unknown error in IMB_exr_begin_write`; the other two matched.

**A broken path template in the node's own path.** The control, a valid `{blend_name}_`, was substituted for the blend file's name on both machines — from `t.blend` the files came out as `t_aaa0001.png` when the template was in the node's File Name, and `shott_aaa0001.png` when it was in one output's own name — so path templates are live in 5.2 and a broken one failing is not "templates are switched off".

With the unclosed `{blend_name_` in **one output's own name**, that one output was skipped and the node's other two were written, alongside a second File Output node which wrote both of its files, the scene output, and exit status 0. One line appeared:

```
reports          | ERROR Invalid path template in File Output node. Skipping writing file.
```

With the unclosed template in the node's **File Name**, which every one of that node's outputs goes through, all three of that node's outputs were skipped — the same line three times — while the second node still wrote both of its files and the scene output was written. The render was not refused and did not stop; over five frames it ran to the end. Exit status 0. Identical on both machines.

**What a script driving the render is handed.** On the blocked scene, `bpy.ops.render.render(animation=False, write_still=True)` returned **`{'FINISHED'}`** and raised nothing, on both machines, while the same run printed the error naming the blocked file:

```
image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e11b-5.2/out/shotbbb.png"
reports          | ERROR Render error (Success) cannot save: '<HT>/e11b-5.2/out/shotbbb.png'
render           | Saved: '<HT>/e11b-5.2/out/shotccc.png'
render           | Saved: '<HT>/e11b-5.2/out/shotaaa.png'
render           | Saved: '<HT>/e11b-5.2/scene/f.png'
VS_RENDER_BEGIN
VS_RENDER_RETURN {'FINISHED'}
VS_RENDER_END
```

Recorded because it caught the test out once: driven through `bpy.ops.render.render()` rather than `-f 1`, this version names the file `shotbbb.png` with **no frame number**, so the first run of this test blocked a name that was never used and all three files were written. The test was rewritten to render once unblocked through the same route, read the name that route actually gives, and only then place the block; the run above is the corrected one.

**What the node and the handler list carry.** Read off a running Blender on both machines, the File Output node's full property list is `file_output_items`, `active_item_index`, `directory`, `file_name`, `format`, `save_as_render`, `use_file_extension`, plus the general node properties including `warning_propagation`. **There is no property reporting whether a write succeeded or failed, and none naming the last path written.** The handler list holds `composite_cancel`, `composite_post`, `composite_pre`, `load_post_fail`, `render_cancel`, `render_complete`, `render_init`, `render_post`, `render_pre`, `render_stats`, `render_write`, `save_post`, `save_post_fail`, `save_pre`. There is a failure hook for loading a blend file and one for saving one, and **no failure hook for writing a render frame**. That is an absence in a list demonstrably being read, since the same listing returned the settings and returned `save_post_fail`.

`bpy.context.window_manager.reports` **does exist** in 5.2 — `PROBE_WM_HAS_REPORTS True` on both machines — but with no window it stayed empty, and a control run shows its emptiness carries no information. Asked after a successful save and again after an operator that certainly errored, it was empty both times:

```
VS0_HAS_REPORTS True
VS0_AT_START []
VS0_AFTER_A_SUCCESSFUL_SAVE []
VS0_OPEN_RAISED RuntimeError Error: Cannot read file "/definitely/no/such/file.blend": No such file or directory
VS0_AFTER_A_FAILED_OPEN []
VS0_END
```

So the list being empty after a failed File Output write says nothing about whether a report was made. That is a limit of the test and is recorded as one.

### A second batch of hand tests, sent back by the citation check — Blender 5.2, 2026-08-09

The citation check of this document re-ran every test above and sent three of them back. Two were written-record gaps; **one is a straight disagreement about what the Creator's Mac prints when a volume fills, and it is settled below only in the sense that a third run now sits beside the other two.** All three were run again by me on 2026-08-09, with `--factory-startup` and a fresh throwaway settings folder every run as before. **Nothing here changes any answer to question 15.** My working folders were `/root/b2-fon` on the rented Linux machine and a throwaway folder in my own scratch space on the Creator's Mac, written `<B2>` below; both were removed afterwards, and the rented machine's shared 56 MB filesystem was not needed by any of the three and was left as found.

**1. The disagreement: what the Mac prints when the volume fills, and what file system it was on.** The disputed line is the first `image.openexr` line in the Mac transcript above. I made a 12 MB disk image twice, once on each of two file systems, and **wrote the file system down from both ends**: from the command that made it (`hdiutil create -size 12m -fs HFS+ -volname b2small`, and again `-fs APFS`) and from `diskutil info` on the mounted volume, which reported `File System Personality: HFS+` with `Type (Bundle): hfs`, and `File System Personality: APFS` with `Type (Bundle): apfs`. Each was filled by measurement rather than by a fixed `count`, so that both started the render from exactly the same free space — `dd if=/dev/zero of=<mount>/filler bs=1024 count=$((available - 1500))`, leaving 1500 KB free by `df` on both. Then the same scene as above: one File Output node on the small volume, three outputs, OpenEXR at 32-bit with no compression, 2000 by 2000, the scene's own output on the ordinary disk, rendered with `-f 1`.

The run on **HFS+**:

```
00:28.271  render           | Fra: 1 | Compositing
00:28.408  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:28.408  render           | Saved: '<B2>/mnt-HFS-5.2/out/shotaaa0001.exr'
00:28.409  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:28.409  reports          | ERROR Error writing render result, No space left on device (see console)
00:28.409  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-HFS-5.2/out/shotbbb0001.exr'
00:28.409  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:28.409  reports          | ERROR Error writing render result, No space left on device (see console)
00:28.409  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-HFS-5.2/out/shotccc0001.exr'
00:28.824  render           | Saved: '<B2>/e12-HFS-5.2/scene/f0001.png'
```

and on **APFS**:

```
00:10.830  render           | Fra: 1 | Compositing
00:11.007  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:11.007  render           | Saved: '<B2>/mnt-APFS-5.2/out/shotaaa0001.exr'
00:11.293  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:11.294  render           | Saved: '<B2>/mnt-APFS-5.2/out/shotbbb0001.exr'
00:11.294  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:11.294  reports          | ERROR Error writing render result, No space left on device (see console)
00:11.294  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-APFS-5.2/out/shotccc0001.exr'
00:11.634  render           | Saved: '<B2>/e12-APFS-5.2/scene/f0001.png'
```

Exit status 0 both times, and the scene's own output on the ordinary disk written both times.

**The three runs, side by side, for the line in dispute.** No run is being deleted and none is being picked over the others:

| whose run | what the first `image.openexr` line says |
|---|---|
| the original hand test, 2026-08-09 | `ERROR IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.` |
| the citation check's repeat, 2026-08-09 | `ERROR Unknown error in IMB_exr_write_channels` |
| this third run, 2026-08-09, on an HFS+ image and an APFS image | `ERROR Unknown error in IMB_exr_write_channels` |

**My run matches the citation check's and not the original's.** That is the whole of what I am saying about it: two runs out of three give the shorter wording, and mine gave it on both file systems. I was not in the room for the original run and I am not deciding it.

Two things this run does add. First, **the file system is not the explanation** — the shorter wording came back on HFS+ and on APFS alike. Second, **the control holds**: on the same disk images, on both file systems, in the same sitting, Blender 4.5 and 5.0 did name the reason on that same line (4.5: `OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "…". No space left on device.`; 5.0: `image.openexr | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.`), so the channel being watched was not simply refusing to name reasons that day.

**What the file system does change is what is left behind.** On HFS+ the volume ended with **one** part-written file, `shotaaa0001.exr`, of 1,536,000 bytes. On APFS, from the same 1500 KB free, it ended with **two** — `shotaaa0001.exr` at 434,581 bytes and `shotbbb0001.exr` at 86,421 bytes — with a second `Unknown error in IMB_exr_write_channels` line and a second `Saved:` line to match. The two also report the volume differently: after the run `df -k` read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS, although the writes had plainly failed for want of space. So the record above of "a single part-written file" on the Mac matches HFS+ and not APFS, and it does not say which it was.

**2. The read-only folder test, and where the test tree has to sit.** The record above says this ran as a non-root user on the rented Linux machine but not where the files were. It matters: `/root` is `drwx------`, so with the tree under `/root` the run as `tester` never reaches the compositor at all —

```
00:00.524  reports          | ERROR Cannot read file "/root/b2-fon/e13trap-5.2/t.blend": Permission denied
```

— with exit status **1**, which is a test that did not run rather than a result. Re-run with the whole tree (blend file, both output folders, scene output folder and throwaway settings folder) at `/tmp/b2-e13-5.2`, which `tester` can read and write, the earlier result comes back unchanged. Both controls held first, before Blender was started: `touch` into the blocked folder was refused (`touch: cannot touch '/tmp/b2-e13-5.2/blocked/canary': Permission denied`) while `touch` into the second node's folder succeeded, so the block bit and the user was not simply unable to write anywhere. Rented Linux machine:

```
00:01.532  render           | Saved: '/tmp/b2-e13-5.2/good/shotddd0001.png'
00:01.534  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.2/blocked/shotaaa0001.png"
00:01.534  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.2/blocked/shotaaa0001.png'
00:01.537  render           | Saved: '/tmp/b2-e13-5.2/good/shoteee0001.png'
00:01.538  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.2/blocked/shotbbb0001.png"
00:01.538  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.2/blocked/shotbbb0001.png'
00:01.538  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.2/blocked/shotccc0001.png"
00:01.539  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.2/blocked/shotccc0001.png'
00:01.540  render           | Saved: '/tmp/b2-e13-5.2/scene/f0001.png'
00:01.540  render           | Time: 00:00.93 (Saving: 00:00.00)
```

Exit status 0; the blocked folder empty afterwards; the second node's two files written at 15,610 bytes each; the scene's own output written. On the Creator's Mac, with the tree at `<B2>/e13-5.2` and the blocked folder at `chmod 500`:

```
00:00.678  render           | Saved: '<B2>/e13-5.2/good/shotddd0001.png'
00:00.678  image.write      | ERROR OpenImageIO write failed: Could not open file "<B2>/e13-5.2/blocked/shotaaa0001.png"
00:00.678  reports          | ERROR Render error (Undefined error: 0) cannot save: '<B2>/e13-5.2/blocked/shotaaa0001.png'
00:00.679  render           | Saved: '<B2>/e13-5.2/good/shoteee0001.png'
00:00.680  image.write      | ERROR OpenImageIO write failed: Could not open file "<B2>/e13-5.2/blocked/shotbbb0001.png"
00:00.680  reports          | ERROR Render error (Undefined error: 0) cannot save: '<B2>/e13-5.2/blocked/shotbbb0001.png'
00:00.680  image.write      | ERROR OpenImageIO write failed: Could not open file "<B2>/e13-5.2/blocked/shotccc0001.png"
00:00.680  reports          | ERROR Render error (Undefined error: 0) cannot save: '<B2>/e13-5.2/blocked/shotccc0001.png'
00:00.681  render           | Saved: '<B2>/e13-5.2/scene/f0001.png'
```

Exit status 0, blocked folder empty, both of the second node's files written, scene output written.

**3. What the complete-file byte size actually depends on.** The figure of **64,032,680 bytes** quoted above for a complete OpenEXR file is not a fixed measurement, and this test says what moves it. The same scene was rendered three times on each machine, changing exactly one thing at a time. On the rented Linux machine, with the blend file at `/root/b2-fon/e14-5.2/t.blend` (28 characters) the three complete files came to **64,032,678 bytes** each; with the blend file moved to `…/ttttttttttt.blend` (38 characters, nothing else changed) they came to **64,032,688** — ten characters more in the path, ten bytes more in the file. With the blend path held fixed and the *output folder* lengthened by ten characters instead, the size did not move at all (64,032,679 both times). The blend file's own path is inside each image; `grep -a -o` finds it word for word. On the Creator's Mac the same pattern: a 134-character blend path gave **64,032,784 bytes**, a 144-character one **64,032,794**, and lengthening the output folder changed nothing.

Subtracting the blend path's length from the figure leaves **64,032,650** on both machines. So the roughly 106-byte gap this document records between the rented Linux machine and the Mac is the gap between the two working-folder names, not a difference between the machines. Neither the figure above nor the citation check's repeat of 64,032,677 is being corrected — I was not in the room for either run — but the number as printed cannot be checked by anyone whose folders are named differently, and the point it carries here (a complete file of about 64 MB against roughly 5 MB left part-written) is untouched.

**Nothing in this batch opens anything on the list of things still needing a person at a computer.** The network folder, the locked file, Windows and the checks needing a window are all untouched by these three tests and stay as they are below.

### What the hand tests did not settle

A network folder taken away part-way through a write could not be run: it needs a file server between the two machines, which the project does not have. A file held open and locked by another program could not be run: neither macOS nor Linux enforces that kind of lock. Nothing was run on Windows: no Windows machine has run it, so that operating system is untested rather than answered. Whether the path field is shown highlighted in red and what its tooltip says, and whether the node shows a warning badge in the compositor, both need eyes on a window and were dropped this round — the rented machine has no display server and the Creator ran nothing; the node property list was checked first and holds nothing carrying warning text, so neither can be read from a terminal instead.

---

### What the sources say

**Label for the documentation on its own: not in the sources.**

**Answer.** The sources do not say. Nothing in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or the Cycles 5.2 files describes what happens when a File Output node's write fails part-way — not to the file that failed, not to the other files being written for the same frame, not to the render as a whole, and not to what gets reported. The documentation describes what the node writes and how the names and folders are formed; it does not describe failure at all.

**The question read one way.** I read "cannot be finished" as a write that starts or is about to start and does not complete — a folder that cannot be written to, storage that runs out, a name the file system rejects, a network location that drops. The other reading is a file that is never attempted because something about its settings is wrong before writing begins. That second reading has one partial, and only partial, answer in the sources, given below under "the nearest thing the sources do say". Neither reading is answered for the failure-during-writing case.

**What the sources do say about writing, which is as far as they go.**

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html, read 2026-07-30.

> "Any missing directories are created automatically during output."

— Manual, same page, read 2026-07-30. This is the only statement I found anywhere about the node coping with a condition it meets on disk, and it is about a folder that is absent, not about a write that fails.

**The nearest thing the sources do say, and its limits.** There is one documented way a path can be prevented from working, and it is a fault in the path text itself rather than a fault at write time:

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

— Manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html, read 2026-07-30.

The same page states that File Output node paths are one of the two kinds of path that support these templates:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

— Manual, same page, read 2026-07-30.

This does not answer the question. The worked example is explicitly about the render output path in Scene properties, not about a File Output node; the sentence before it says only that such errors "may prevent certain actions", without saying which ones for a node path. Reading it as "a bad template in a File Output node path fails the render with an error message" would be me supplying the step, and I am not supplying it.

**On "its reports", what exists is the mechanism, not the behaviour.** Blender has a general place where errors and warnings land:

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

— Manual, https://docs.blender.org/manual/en/5.2/editors/info_editor.html, read 2026-07-30.

And in 5.2 that log became readable from Python:

> "Blender reports are now exposed as a read-only list in `WindowManager` ID types. They also now have a session-wide unique identifier integer"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30.

The API documentation describes what one such entry holds: a `message` ("Report message text"), a `type` ("Report type (severity)"), and a `session_uid` ("Unique per-session report identifier") — Python API, https://docs.blender.org/api/5.2/bpy.types.Report.html, read 2026-07-30; the list itself is `reports`, "Collection of reports", on https://docs.blender.org/api/5.2/bpy.types.WindowManager.html, read 2026-07-30.

That tells you reports exist and can be read. It does not say that a failed File Output write produces one, nor what it would say, nor its severity. Concluding that it would is exactly the step I am not taking.

**What is absent that you might expect to be present.** The Python API's `CompositorNodeOutputFile` (https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html) and `NodeCompositorFileOutputItem` (https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html), both read 2026-07-30, list every property of the node and of an individual output. Neither carries any property reporting success, failure, or the last path actually written. There is no handler for a failed write in `bpy.app.handlers` (https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30); the one write-related handler there is described as firing "on writing a render frame (directly after the frame is written)", and that description does not say whether it fires when a write does not complete. This is an absence of features, not a statement about behaviour, and I record it as such.

**Not covered.** This answer settles only that the sources are silent. It does not settle what Blender actually does — the failed file may be left absent, left empty, or left half-written; the other outputs for the frame may all be written, or none of them; the render may stop, or carry on to the next frame; something may or may not be reported. I have no basis in the sources to say which. It also does not settle whether a failure differs between the two Media Types, which is the one place where "several files for the same frame" might change shape. The node's page carries a Media Type setting, but nothing quoted here establishes how many files each Media Type produces; that belongs to a different question and I have not settled it.

**Outside the interface.** **Brought up to date by hand test, 2026-08-09: observed.** Every test above was run with Blender started from a terminal with no window, which is the only way Smoozy will ever run it, so the whole of the answer above is an answer about Blender driven from outside its own interface. What that route gives you: the error naming the path is printed to the terminal every time, on both machines; the process exit status is **0** in every case, so a script watching only the exit status learns nothing; and `bpy.ops.render.render()` returns `{'FINISHED'}` and raises nothing. There is no handler that fires on a failed write and no property on the node recording one, so the printed line is the only signal. Whether the same behaviour holds *inside* Blender's own window was not tested and is not settled here — the rented machine has no display server and the Creator ran nothing this round.

**Added by the second batch of hand tests, 2026-08-09.** One thing that sharpens the exit-status point above. A run that could not read the blend file at all — the same scene, started as a user who cannot enter the folder it sits in — ends with `Error: Cannot read file "…": Permission denied` and **exit status 1**. So the exit status is not simply always 0: it does report a blend file Blender could not open, and it does not report a File Output write that failed. A script watching only the exit status will therefore see the difference between "the job never started" and "the job ran", and will see no difference at all between "every file was written" and "a file could not be written".

The original note from the sources follows.

 The sources do not say. I found nothing anywhere describing failed-write behaviour at all, and therefore nothing describing whether it differs when Blender is started and driven from outside its own window. Specifically: the command-line arguments page (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30) documents `--background`, the `--log` family, and `--debug-exit-on-error` ("Immediately exit when internal errors are detected"), but says nothing tying any of them to output files that cannot be written, and nothing about an exit status for a render that failed to write. Note also that the general reporting mechanism quoted above is described in terms of an editor inside Blender's own window; the sources do not say what becomes of those messages with no window present.

**Operating system.** **Brought up to date by hand test, 2026-08-09: observed on two operating systems, untested on the third.** Every test above ran on Linux (Ubuntu 22.04.5 on the rented machine) and on macOS 26.5.2 on Apple silicon (the Creator's Mac). On everything the question asks the two agreed: which files survive, which do not, that the render carries on, that an error naming the path is printed, and that the exit status is 0. They differed only in wording and in one detail on disk — the bracketed reason was `(Success)` on Linux and `(Undefined error: 0)` on the Mac; and when a volume filled, Linux left **zero-byte files** for the outputs that could never be opened where the Mac left **no file at all**. In this version the Mac also printed `Unknown error in IMB_exr_begin_write` where Linux named the reason. **Windows is untested**, not answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Added by the second batch of hand tests, 2026-08-09.** One part of that last difference turns out to belong to the disk's own file system rather than to the operating system, and the first batch did not record which file system the Mac's disk image carried. Re-run on the Creator's Mac on a 12 MB **HFS+** image and again on a 12 MB **APFS** image, both filled to leave exactly 1500 KB free, the volume-full test left **one** part-written file on HFS+ and **two** on APFS, and `df -k` afterwards read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS even though the writes had failed for want of space. What did hold on both Mac file systems is the thing the line above rests on: no zero-byte file was left for an output that could never be opened, where the rented Linux machine leaves one. So the Linux-against-macOS difference stands; the count of part-written files and the `df` reading are properties of the file system as well.

The original note from the sources follows.

 The sources do not say. There is nothing operating-system-specific about File Output writing anywhere I searched. The only permission-related warning I found in the whole manual is about launching an external text editor on Windows under elevated privileges (https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html, read 2026-07-30), which is unrelated to this node. The sources neither state that this behaviour is the same on every operating system nor state that it differs, so it is unsettled rather than settled-as-uniform.

**Where I looked.**

Searched, across the whole 5.2 manual and the whole 5.2 API documentation at once, for: `unable to open/write/save`, `cannot save`, `cannot write`, `cannot be saved`, `cannot be written`, `failed to save/write/open`, `write error`, `disk full`, `out of disk`, `no space`, `disk space`, `storage`, `quota`, `read-only`, `read only`, `permission`, `locked`, `network drive`, `network share`, `corrupt`, `truncated`, `incomplete`, `partial file`, `zero-byte`, `placeholder`, `abort`, `halt`, `error`, `fail`, `invalid path`, `directory does not exist`, `missing directory`, `exit code`, `return code`, `non-zero`, `stdout`, `standard output`, `console output`, `Saved:`. Also searched for every mention of the phrase "file output" in the manual, and followed each one.

Pages read in full or in the relevant part, and why each was ruled out:

- Manual, File Output Node (https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) — the node's own page, read end to end. Covers what it writes, its inputs, Directory, File Name, Media Type, Node Format, Images/Layers, Override Node Format, Item Format, Output Paths, File Extensions. No mention of failure, retries, partial files, or reports.
- Manual, File Paths (https://docs.blender.org/manual/en/5.2/files/file_paths.html) — read end to end, including its "Errors" section. Quoted above; concerns faults in the path text, and its only worked example of a failed action is the Scene render output path.
- Manual, Output panel (https://docs.blender.org/manual/en/5.2/render/output/properties/output.html) — read for the passage on turning automatic saving off. It says "If no such nodes exist, an error is reported", which is about there being no File Output node at all when one is needed, not about one failing. Also covers Overwrite and Placeholders, which are options of the Scene output path, not of the node.
- Manual, Rendering Animations (https://docs.blender.org/manual/en/5.2/render/output/animation.html) — read for the passages naming File Output nodes. They describe when the node is relied on instead of the Output panel. Nothing on failure.
- Manual, Compositing — Introduction, Compositor System, Limitations, Usage, Sidebar (https://docs.blender.org/manual/en/5.2/compositing/) — searched for `error`, `fail`, `invalid`; no matches in any of them. Limitations covers GPU hardware only.
- Manual, Info Editor (https://docs.blender.org/manual/en/5.2/editors/info_editor.html) and Status Bar (https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html) — read for how errors surface. Generic mechanism only; neither mentions rendering output files.
- Manual, Command Line Arguments (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html) — searched for background mode, logging, and exit behaviour. Nothing links any of it to output-file failure.
- Manual, Troubleshooting section — searched; its contents are graphics-hardware, startup, and file-recovery matters. Nothing on render output writing.
- Python API: `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, `NodeCompositorFileOutputItems`, `bpy.app.handlers`, `bpy.types.Report`, `bpy.types.WindowManager` (all at https://docs.blender.org/api/5.2/) — read for any property, method, or handler concerning failure. None exists; see above.
- Release notes 5.2: searched `compositor`, `core`, `pipeline_io`, `rendering`, `python_api` for `error`, `fail`, `report`, `abort`, `cancel`. The only hit of substance is the Python reports entry quoted above. The compositor page's only File Output entry is the new File Extension checkbox: "File Output node now has a **File Extension** checkbox that is independent of the 'File Extension' checkbox in the scene render settings." (https://developer.blender.org/docs/release_notes/5.2/compositor/, read 2026-07-30.)
- Release notes 5.2, fixed-bug list (https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30) — contains, under "Pipeline & IO", the line "Rendering File Output Anamorph with Mulit-Layer EXR cannot save file". I am ruling this out as an answer: it is a one-line entry naming a fault that was fixed, and it says nothing about what happened to the other files, the render, or the reports when it occurred.
- Cycles 5.2 files — searched the whole folder for `file_output`, `OutputFile`, `compositor`, `save_image`, and image-file writing generally. The only matches are user-interface panel classes for compositor denoising performance in `ui.py`. Cycles' files contain nothing about File Output node writing.

---

## Things a person with Blender 5.2 open could settle

This list has been pruned on 2026-08-09. Most of what the researcher proposed was run on both machines and the results are written in above: a folder that cannot be written to; one output of several failing while the rest succeed; a write that fails part-way because the volume fills; whether an animation carries on; the same tests with no window, with the exit status read; two File Output nodes with one failing; Multi-Layer EXR against separate images; a broken path template in a node's own path; and whether a script driving the render can detect any of it. What is left is only what is genuinely still open.

1. **A network folder pulled away mid-render.** Point one File Output node at a mounted network folder and another at a local folder. Start an animation render, and disconnect the network while it is running. Note whether Blender stops, hangs, or continues; what appears in the terminal; and what is left at both locations afterwards. **Could not be run on 2026-08-09**, and the reason is equipment: this needs a file server between two machines, and the project has the Creator's Mac and one rented Linux machine reached over the open internet, with nothing set up to serve a share from either to the other. The network cause of a failed write is therefore untested rather than answered.

2. **A file held open and locked by another program.** **Could not be run on 2026-08-09**: neither macOS nor Linux enforces that kind of lock against an unrelated program, and no Windows machine was tried, where it would be arrangeable.

3. **The same tests on Windows.** **Not run on 2026-08-09**: no Windows machine has run it. Linux and macOS on Apple silicon are both settled above; Windows is untested, not answered.

4. **What a person sees on screen.** Whether the path field of a File Output node with a broken template is shown highlighted in red and what its tooltip says; whether the node shows a warning badge in the compositor after a failed write; and what the Info editor and the status bar hold. **Dropped on 2026-08-09 as needing a window**: the rented Linux machine has no display server, and the Creator ran nothing this round. Before dropping it, the node's own property list was read from a terminal and holds nothing carrying warning text, and although 5.2 does expose `bpy.context.window_manager.reports`, a control run showed that list stays empty with no window even for an operator that certainly errored, so it cannot stand in for what a window would show.

---

2026-08-09 — Opus 5 (Claude Code) — File output nodes, Blender 5.2

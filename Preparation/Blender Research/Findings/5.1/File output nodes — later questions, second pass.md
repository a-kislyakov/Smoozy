# File output nodes — later questions, second pass (Blender 5.1)

Sources consulted, all read on 2026-07-30: Blender's 5.1 user manual, Blender's 5.1 Python API documentation, the 5.1 release notes, and Cycles' own files as shipped with 5.1.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?

**Label: observed** — settled by hand test on 2026-08-09. The sources remain silent, and the record of that silence is kept below, because it is what sent this question to the hand tests.

### What the hand tests found — Blender 5.1, 2026-08-09

Every test below was run on both of the project's machines, and each is written up as two results.

**The rented Linux machine.** Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. Blender 5.1.2 at `/opt/blender/5.1/blender`.

**The Creator's Mac.** MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.

Every run added `--factory-startup` and pointed `BLENDER_USER_RESOURCES` at a throwaway folder made just before it.

**What was set up.** A scene built by script with one File Output node whose Directory was a writable folder and whose File Name was `shot`, holding three outputs named `aaa`, `bbb` and `ccc`, each fed from the Render Layers node's Image output; PNG, 64 by 64, Cycles on the CPU at one sample. The scene's own render output path was a *different* folder, so the node's files and the scene's own file could be told apart. Rendered with `<blender> --factory-startup -b <file>.blend -f 1`, followed by `echo $?`.

Two things about the node itself had to be established before any of this would run, and they are recorded because the words that ran are what the record has to show. The compositor is reached through `scene.compositing_node_group`, which is empty until a `CompositorNodeTree` is made and assigned to it. And **the node's format starts at Multi-Layer EXR**: `HT_BUILD_DEFAULT_MEDIA_TYPE MULTI_LAYER_IMAGE`, `HT_BUILD_DEFAULT_FILE_FORMAT OPEN_EXR_MULTILAYER`, and `file_format` will not accept `PNG` until `media_type` is set to `IMAGE` first.

**The control run, both machines.** Three files from the node and one from the scene output, a `Saved:` line for each, exit status 0. On the rented Linux machine, word for word from `Compositing` onward:

```
00:00.395  render           | Fra: 1 | Compositing
00:00.541  render           | Saved: '/root/ht-fon/e2-5.1/out/shotccc0001.png'
00:00.546  render           | Saved: '/root/ht-fon/e2-5.1/out/shotaaa0001.png'
00:00.548  render           | Saved: '/root/ht-fon/e2-5.1/out/shotbbb0001.png'
00:00.549  render           | Saved: '/root/ht-fon/e2-5.1/scene/f0001.png'
00:00.549  render           | Time: 00:00.26 (Saving: 00:00.00)
```

The node names its files from the File Name glued to the output's own name, then the frame number: `shotaaa0001.png`. The Mac gave the same names and the same four lines.

**One output of three blocked, by a folder standing where its file goes.** A directory was made at `shotbbb0001.png`, which refuses the write even to `root`. The directory was still an empty directory afterwards, so the block bit. On the rented Linux machine:

```
00:00.503  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/ht-fon/e3-5.1/out/shotbbb0001.png"
00:00.503  reports          | ERROR Render error (Success) cannot save: '/root/ht-fon/e3-5.1/out/shotbbb0001.png'
00:00.530  render           | Saved: '/root/ht-fon/e3-5.1/out/shotccc0001.png'
00:00.536  render           | Saved: '/root/ht-fon/e3-5.1/out/shotaaa0001.png'
00:00.537  render           | Saved: '/root/ht-fon/e3-5.1/scene/f0001.png'
```

On the Creator's Mac the same run gave the same two lines with a different reason in the brackets:

```
00:00.612  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e3-5.1/out/shotbbb0001.png"
00:00.612  reports          | ERROR Render error (Undefined error: 0) cannot save: '<HT>/e3-5.1/out/shotbbb0001.png'
```

**The two machines agreed** on everything the question asks: `aaa` and `ccc` were written at the same byte size as in the control, nothing was left for `bbb`, the scene's own output was written, an error naming the path appeared, the render finished, and `echo $?` gave 0. They disagreed only on the words inside the brackets — `(Success)` on the rented Linux machine, `(Undefined error: 0)` on the Mac.

**Across a five-frame animation.** Same block, rendered with `-a`. The control run with nothing blocked printed 20 `Saved:` lines and left five files each of `aaa` and `ccc`. The blocked run printed 19 — one fewer, the missing one being `bbb` for frame 1 — reached frame 5, and wrote `bbb` normally for frames 2 to 5, because only frame 1's name was occupied. The error appeared once, not once per frame. Exit status 0. Same on both machines.

**A folder that refuses writing, for a user who is not root.** On the rented Linux machine this was run as the user `tester`, because root writes into a read-only folder without complaint; on the Mac as the ordinary user. In both cases `touch` into the folder was refused with `Permission denied` before Blender was run, so the condition really bit. Where the whole node's folder was read-only and a second File Output node pointed at a writable one, on the rented Linux machine:

```
00:00.934  render           | Saved: '<HT>/e6a-5.1/good/shotddd0001.png'
00:00.936  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.1/blocked/shotaaa0001.png"
00:00.936  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.1/blocked/shotaaa0001.png'
00:00.941  render           | Saved: '<HT>/e6a-5.1/good/shoteee0001.png'
00:00.942  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.1/blocked/shotbbb0001.png"
00:00.942  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.1/blocked/shotbbb0001.png'
00:00.943  image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e6a-5.1/blocked/shotccc0001.png"
00:00.943  reports          | ERROR Render error (Success) cannot save: '<HT>/e6a-5.1/blocked/shotccc0001.png'
00:00.944  render           | Saved: '<HT>/e6a-5.1/scene/f0001.png'
```

All three of that node's outputs failed; the second File Output node wrote both of its files; the scene output was written; the blocked folder was empty afterwards; exit status 0. The Mac gave the same, with `(Undefined error: 0)` in the brackets.

Where only one output of three sat inside a read-only subfolder — an output named `sub/bbb`, giving the folder `shotsub` — the other two outputs were written, nothing was left in the blocked folder, and the exit status was 0, on both machines.

**An output's name carrying a folder.** With the three outputs named `aaa`, `sub/bbb`, `ccc` and no `sub` folder existing beforehand, the slash was kept, and Blender made the folder and wrote into it. The folder is named from the node's File Name glued to the part before the slash, so `sub/bbb` under a File Name of `shot` produced `Saved: '<HT>/e5-5.1/out/shotsub/bbb0001.png'`. Both machines gave the same result. This was run in all four versions to the same script, so this version's behaviour is a measurement rather than an absence.

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
render           | Saved: '<HT>/e7-5.1/scene/f0001.png'
```

and what was left on the volume:

```
-rw-r--r-- 1 root root       0 Aug  9 14:38 shotaaa0001.exr
-rw-r--r-- 1 root root 4874240 Aug  9 14:38 shotbbb0001.exr
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

*(Disagreement found by the citation check, 2026-08-09, and left unresolved. Repeating this test on the Creator's Mac gave a different **first** line. The run above has `image.openexr | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.` The repeat gave, twice — once on a 12 MB HFS+ disk image and once on a 12 MB APFS disk image, otherwise to the same recipe — `image.openexr    | ERROR Unknown error in IMB_exr_write_channels`. Everything else in this entry reproduced, including `ERROR Unknown error in IMB_exr_begin_write` on the two lines below. Neither run is being picked over the other; the test is on the list to be run again with the disk image's file system written down, which this record does not state. A third run was made on 2026-08-09 and is recorded in full under *A second batch of hand tests* below, on a 12 MB HFS+ disk image and a 12 MB APFS one with the file system written down both times; it gave the same words as the repeat, on both file systems, and all three runs now sit side by side. Still nobody's run is picked over another's.)*

**The two machines agreed** that one file is left **part-written** on disk — 4,874,240 bytes on the rented Linux machine and 2,813,952 bytes on the Mac, against a complete file of 64,032,676 and 64,032,782 bytes respectively — that **Blender prints a `Saved:` line for that part-written file**, that the outputs which could not be started are not written, that the scene's own output on the ordinary disk is written in full, that the render finishes, and that the exit status is 0. They disagreed on two details: the rented Linux machine left **zero-byte files** for the two that could not be opened, where the Mac left **no file at all**; and the rented Linux machine names the path and the reason (`IMB_exr_begin_write: Cannot open image stream "…". No space left on device.`) where the Mac printed only `Unknown error in IMB_exr_begin_write`, though the line after it on the Mac still said `Error writing render result, No space left on device (see console)`.

**Two File Output nodes, one unable to write at all.** The second node was aimed inside a path that cannot be a folder, because a plain file sat there; that path was still a plain file of zero bytes afterwards. The healthy node wrote all three of its files, the scene output was written, exit status 0, on both machines. On the rented Linux machine:

```
render           | Saved: '<HT>/e8-5.1/out/shotccc0001.png'
image            | ERROR Couldn't create directory for file <HT>/e8-5.1/notadir/inside/shoteee0001.png: Not a directory
reports          | ERROR Render error (Not a directory) cannot save: '<HT>/e8-5.1/notadir/inside/shoteee0001.png'
image            | ERROR Couldn't create directory for file <HT>/e8-5.1/notadir/inside/shotddd0001.png: Not a directory
reports          | ERROR Render error (Not a directory) cannot save: '<HT>/e8-5.1/notadir/inside/shotddd0001.png'
render           | Saved: '<HT>/e8-5.1/out/shotaaa0001.png'
render           | Saved: '<HT>/e8-5.1/out/shotbbb0001.png'
render           | Saved: '<HT>/e8-5.1/scene/f0001.png'
```

The Mac gave the same lines. So one node failing does not stop another node in the same tree.

**Multi-Layer EXR.** With the node's Media Type set to `MULTI_LAYER_IMAGE`, the node wrote **exactly one file** for the frame — `shot0001.exr` — not three, on both machines. So in that mode there are no other files from that node for that frame. With a directory standing at that one name, nothing at all was left for the node, the scene's own output was still written, and the exit status was 0. On the rented Linux machine:

```
image.openexr    | ERROR IMB_exr_begin_write: Is a directory.
reports          | ERROR Error writing render result, Is a directory (see console)
reports          | ERROR Render error (Is a directory) cannot save: '<HT>/e9-5.1/out/shot0001.exr'
render           | Saved: '<HT>/e9-5.1/scene/f0001.png'
```

On the Mac the first of those three lines read `ERROR Unknown error in IMB_exr_begin_write`; the other two matched.

**A broken path template in the node's own path.** The control, a valid `{blend_name}_`, was substituted for the blend file's name on both machines — from `t.blend` the files came out as `t_aaa0001.png` when the template was in the node's File Name, and `shott_aaa0001.png` when it was in one output's own name — so path templates are live in 5.1 and a broken one failing is not "templates are switched off".

With the unclosed `{blend_name_` in **one output's own name**, that one output was skipped and the node's other two were written, alongside a second File Output node which wrote both of its files, the scene output, and exit status 0. One line appeared:

```
reports          | ERROR Invalid path template in File Output node. Skipping writing file.
```

With the unclosed template in the node's **File Name**, which every one of that node's outputs goes through, all three of that node's outputs were skipped — the same line three times — while the second node still wrote both of its files and the scene output was written. The render was not refused and did not stop; over five frames it ran to the end. Exit status 0. Identical on both machines.

**What a script driving the render is handed.** On the blocked scene, `bpy.ops.render.render(animation=False, write_still=True)` returned **`{'FINISHED'}`** and raised nothing, on both machines, while the same run printed the error naming the blocked file:

```
image.write      | ERROR OpenImageIO write failed: Could not open file "<HT>/e11b-5.1/out/shotbbb.png"
reports          | ERROR Render error (Success) cannot save: '<HT>/e11b-5.1/out/shotbbb.png'
render           | Saved: '<HT>/e11b-5.1/out/shotccc.png'
render           | Saved: '<HT>/e11b-5.1/out/shotaaa.png'
render           | Saved: '<HT>/e11b-5.1/scene/f.png'
VS_RENDER_BEGIN
VS_RENDER_RETURN {'FINISHED'}
VS_RENDER_END
```

Recorded because it caught the test out once: driven through `bpy.ops.render.render()` rather than `-f 1`, this version names the file `shotbbb.png` with **no frame number**, so the first run of this test blocked a name that was never used and all three files were written. The test was rewritten to render once unblocked through the same route, read the name that route actually gives, and only then place the block; the run above is the corrected one.

**What the node and the handler list carry.** Read off a running Blender on both machines, the File Output node's full property list is `file_output_items`, `active_item_index`, `directory`, `file_name`, `format`, `save_as_render`, plus the general node properties including `warning_propagation`. **There is no property reporting whether a write succeeded or failed, and none naming the last path written.** The handler list holds `composite_cancel`, `composite_post`, `composite_pre`, `load_post_fail`, `render_cancel`, `render_complete`, `render_init`, `render_post`, `render_pre`, `render_stats`, `render_write`, `save_post`, `save_post_fail`, `save_pre`. There is a failure hook for loading a blend file and one for saving one, and **no failure hook for writing a render frame**. That is an absence in a list demonstrably being read, since the same listing returned the settings and returned `save_post_fail`.

`bpy.context.window_manager.reports` does not exist in 5.1 — `PROBE_WM_HAS_REPORTS False` on both machines — so there is no way to read Blender's own report log from a terminal in this version.

### A second batch of hand tests, sent back by the citation check — Blender 5.1, 2026-08-09

The citation check of this document re-ran every test above and sent three of them back. Two were written-record gaps; **one is a straight disagreement about what the Creator's Mac prints when a volume fills, and it is settled below only in the sense that a third run now sits beside the other two.** All three were run again by me on 2026-08-09, with `--factory-startup` and a fresh throwaway settings folder every run as before. **Nothing here changes any answer to question 15.** My working folders were `/root/b2-fon` on the rented Linux machine and a throwaway folder in my own scratch space on the Creator's Mac, written `<B2>` below; both were removed afterwards, and the rented machine's shared 56 MB filesystem was not needed by any of the three and was left as found.

**1. The disagreement: what the Mac prints when the volume fills, and what file system it was on.** The disputed line is the first `image.openexr` line in the Mac transcript above. I made a 12 MB disk image twice, once on each of two file systems, and **wrote the file system down from both ends**: from the command that made it (`hdiutil create -size 12m -fs HFS+ -volname b2small`, and again `-fs APFS`) and from `diskutil info` on the mounted volume, which reported `File System Personality: HFS+` with `Type (Bundle): hfs`, and `File System Personality: APFS` with `Type (Bundle): apfs`. Each was filled by measurement rather than by a fixed `count`, so that both started the render from exactly the same free space — `dd if=/dev/zero of=<mount>/filler bs=1024 count=$((available - 1500))`, leaving 1500 KB free by `df` on both. Then the same scene as above: one File Output node on the small volume, three outputs, OpenEXR at 32-bit with no compression, 2000 by 2000, the scene's own output on the ordinary disk, rendered with `-f 1`.

The whole run on **HFS+**, in order:

```
00:15.763  render           | Executing compositor
00:15.763  render           | Fra: 1 | Compositing
00:15.793  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:15.794  render           | Saved: '<B2>/mnt-HFS-5.1/out/shotbbb0001.exr'
00:15.794  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:15.794  reports          | ERROR Error writing render result, No space left on device (see console)
00:15.794  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-HFS-5.1/out/shotccc0001.exr'
00:15.794  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:15.794  reports          | ERROR Error writing render result, No space left on device (see console)
00:15.794  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-HFS-5.1/out/shotaaa0001.exr'
00:16.127  render           | Saved: '<B2>/e12-HFS-5.1/scene/f0001.png'
00:16.127  render           | Time: 00:15.69 (Saving: 00:00.33)
```

and on **APFS**, in order:

```
00:11.279  render           | Executing compositor
00:11.279  render           | Fra: 1 | Compositing
00:11.305  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:11.305  render           | Saved: '<B2>/mnt-APFS-5.1/out/shotaaa0001.exr'
00:11.408  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:11.408  render           | Saved: '<B2>/mnt-APFS-5.1/out/shotbbb0001.exr'
00:11.408  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:11.408  reports          | ERROR Error writing render result, No space left on device (see console)
00:11.408  reports          | ERROR Render error (No space left on device) cannot save: '<B2>/mnt-APFS-5.1/out/shotccc0001.exr'
00:11.782  render           | Saved: '<B2>/e12-APFS-5.1/scene/f0001.png'
00:11.782  render           | Time: 00:11.39 (Saving: 00:00.37)
```

Exit status 0 both times, and the scene's own output on the ordinary disk written both times.

**The three runs, side by side, for the line in dispute.** No run is being deleted and none is being picked over the others:

| whose run | what the first `image.openexr` line says |
|---|---|
| the original hand test, 2026-08-09 | `ERROR IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.` |
| the citation check's repeat, 2026-08-09, on an HFS+ image and an APFS image | `ERROR Unknown error in IMB_exr_write_channels` |
| this third run, 2026-08-09, on an HFS+ image and an APFS image | `ERROR Unknown error in IMB_exr_write_channels` |

**My run matches the citation check's and not the original's.** That is the whole of what I am saying about it: two runs out of three, on two file systems each, give the shorter wording. I was not in the room for the original run and I am not deciding it.

Two things this run does add. First, **the file system is not the explanation** — the shorter wording came back on HFS+ and on APFS alike. Second, **the control holds**: on the same disk images, on both file systems, in the same sitting, Blender 4.5 and 5.0 did name the reason on that same line (4.5: `OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "…". No space left on device.`; 5.0: `image.openexr | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.`), so the channel being watched was not simply refusing to name reasons that day.

**What the file system does change is what is left behind.** On HFS+ the volume ended with **one** part-written file, `shotbbb0001.exr`, of 1,536,000 bytes. On APFS, from the same 1500 KB free, it ended with **two** — `shotaaa0001.exr` at 463,249 bytes and `shotbbb0001.exr` at 57,745 bytes — with a second `Unknown error in IMB_exr_write_channels` line and a second `Saved:` line to match. The two also report the volume differently: after the run `df -k` read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS, although the writes had plainly failed for want of space. So the record above of "a single part-written file" on the Mac matches HFS+ and not APFS, and it does not say which it was.

**2. The read-only folder test, and where the test tree has to sit.** The record above says this ran as a non-root user on the rented Linux machine but not where the files were. It matters: `/root` is `drwx------`, so with the tree under `/root` the run as `tester` never reaches the compositor at all —

```
00:00.353  reports          | ERROR Cannot read file "/root/b2-fon/e13trap-5.1/t.blend": Permission denied
```

— with exit status **1**, which is a test that did not run rather than a result. Re-run with the whole tree (blend file, both output folders, scene output folder and throwaway settings folder) at `/tmp/b2-e13-5.1`, which `tester` can read and write, the earlier result comes back unchanged. Both controls held first, before Blender was started: `touch` into the blocked folder was refused (`touch: cannot touch '/tmp/b2-e13-5.1/blocked/canary': Permission denied`) while `touch` into the second node's folder succeeded, so the block bit and the user was not simply unable to write anywhere. Rented Linux machine:

```
00:00.800  render           | Saved: '/tmp/b2-e13-5.1/good/shotddd0001.png'
00:00.801  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.1/blocked/shotaaa0001.png"
00:00.801  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.1/blocked/shotaaa0001.png'
00:00.803  render           | Saved: '/tmp/b2-e13-5.1/good/shoteee0001.png'
00:00.803  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.1/blocked/shotbbb0001.png"
00:00.803  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.1/blocked/shotbbb0001.png'
00:00.804  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/b2-e13-5.1/blocked/shotccc0001.png"
00:00.804  reports          | ERROR Render error (Success) cannot save: '/tmp/b2-e13-5.1/blocked/shotccc0001.png'
00:00.805  render           | Saved: '/tmp/b2-e13-5.1/scene/f0001.png'
```

Exit status 0; the blocked folder empty afterwards; the second node's two files written at 15,841 bytes each; the scene's own output written. On the Creator's Mac, with the tree at `<B2>/e13-5.1` and the blocked folder at `chmod 500`, the same run gave the same lines with `(Undefined error: 0)` in the brackets, exit status 0, the blocked folder empty, both of the second node's files written and the scene output written.

**3. What the complete-file byte size actually depends on.** The figure of **64,032,676 bytes** quoted above for a complete OpenEXR file is not a fixed measurement, and this test says what moves it. The same scene was rendered three times on each machine, changing exactly one thing at a time. On the rented Linux machine, with the blend file at `/root/b2-fon/e14-5.1/t.blend` (28 characters) the three complete files came to **64,032,674 bytes** each; with the blend file moved to `…/ttttttttttt.blend` (38 characters, nothing else changed) they came to **64,032,684** — ten characters more in the path, ten bytes more in the file. With the blend path held fixed and the *output folder* lengthened by ten characters instead, the size did not move at all (64,032,675 both times). The blend file's own path is inside each image; `grep -a -o` finds it word for word. On the Creator's Mac the same pattern: a 134-character blend path gave **64,032,780 bytes**, a 144-character one **64,032,790**, and lengthening the output folder changed nothing.

Subtracting the blend path's length from the figure leaves **64,032,646** on both machines. So the roughly 106-byte gap this document records between the rented Linux machine and the Mac is the gap between the two working-folder names, not a difference between the machines. Neither the figure above nor the citation check's repeat of 64,032,673 is being corrected — I was not in the room for either run — but the number as printed cannot be checked by anyone whose folders are named differently, and the point it carries here (a complete file of about 64 MB against roughly 5 MB left part-written) is untouched.

**Nothing in this batch opens anything on the list of things still needing a person at a computer.** The network folder, the locked file, Windows and the checks needing a window are all untouched by these three tests and stay as they are below.

### What the hand tests did not settle

A network folder taken away part-way through a write could not be run: it needs a file server between the two machines, which the project does not have. A file held open and locked by another program could not be run: neither macOS nor Linux enforces that kind of lock. Nothing was run on Windows: no Windows machine has run it, so that operating system is untested rather than answered. Whether the path field is shown highlighted in red and what its tooltip says, and whether the node shows a warning badge in the compositor, both need eyes on a window and were dropped this round — the rented machine has no display server and the Creator ran nothing; the node property list was checked first and holds nothing carrying warning text, so neither can be read from a terminal instead.

---

### What the sources say

**Label for the documentation on its own: not in the sources.**

**The question was ambiguous, and I answered the wider reading.** "Cannot be finished" could mean either (a) a write that starts and then breaks off partway — the disk fills, a network share drops, the machine loses power — or (b) a write that never gets going, because the folder is not writable, the name is not usable, or the format cannot take the image. I looked for both. The sources cover neither. Nothing below turns on which reading is meant, because the documentation is silent on both.

### What the sources do say, and how far it gets

The manual states what the node writes, and nothing about what happens when a write does not complete:

> "The File Output node writes one or more images to disk during rendering or compositing."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

That page is the whole of the manual's treatment of the node. I read it end to end (about 130 lines). It describes inputs, Directory, File Name, Media Type, Node Format, Images/Layers, Override Node Format, Item Format, and Output Paths. It contains no sentence about errors, failures, missing permissions, full disks, partial files, or what a failed write does to the rest of the render. I also opened the one picture on that page, `_images/node-types_CompositorNodeOutputFile.webp`. Visible in it, and so **documented**: a node headed "File Output", a text field reading `/tmp/` with a folder button, a second text field reading `file_name`, and one empty socket circle. No warning or error indicator is visible in it, and nothing in it bears on failure. That the two fields are the Directory and File Name settings is **inferred**; the picture carries no labels for them.

*(Changed by the citation check, 2026-08-09: the fields were named as though the picture labelled them. Checked against `manual_5.1/_images/node-types_CompositorNodeOutputFile.webp`.)*

One nearby statement removes a single possible cause of failure, and only that one:

> "Any missing directories are created automatically during output."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30. This appears in the Inputs section, under the case where Media Type is Image and an input name contains a slash. So a folder that does not yet exist is not, by itself, a reason a write cannot finish — Blender makes it. **Label for that narrow point: documented.** It says nothing about what happens if creating the directory itself fails.

The one documented failure of any kind in this area is a *path template* error, and the consequence is stated only for the scene's render output path, not for the node's paths:

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

The same page confirms the node's paths are in the group that supports templates:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

So the manual tells us a bad template in the *render output path* fails the animation render with an error message. It does not say what a bad template in a *File Output node* path does — whether it fails the render, skips that one file, or does something else. "Depending on the path" is the manual's own signal that the answer varies and is not stated. I did not extend the render-output-path sentence to the node; doing so would be a guess, not an inference.

Also worth recording, because it is what the question's "reports" would run through:

- The Python API lists a handler that fires *after* a successful frame write, and lists no counterpart for a failed one. The full handler list on that page contains `load_post_fail` and `save_post_fail` — failure handlers for loading and saving blend files — but nothing equivalent for rendering or writing. The write handler is described as: "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered" — API, https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30. The absence of a write-failure handler in that list is a fact about the list, not evidence about what Blender does on failure.
- The general rule for operator reports: "If operator was cancelled but there wasn't any reports from it with {'ERROR'} type, it will just return {'CANCELLED'} without raising any exceptions. However, if there are error reports, a RuntimeError will be raised after the operator finishes execution, including all error report messages, regardless of the return status (even if it was {'FINISHED'})." — API, https://docs.blender.org/api/5.1/bpy.ops.html, read 2026-07-30. This says what happens *if* an error report is produced. Nothing in the sources says whether a File Output write failure produces one, so this rule cannot be applied to answer the question.
- The render operator's own documentation is blank: "Undocumented, consider contributing." — API, https://docs.blender.org/api/5.1/bpy.ops.render.html, read 2026-07-30 (entry for `bpy.ops.render.render`). Its parameters are listed; its behaviour, including on failure, is not.
- Where messages surface generally: "The Info editor logs the executed operators as well as errors, warnings, and informational messages." — Manual, https://docs.blender.org/manual/en/5.1/editors/info_editor.html, read 2026-07-30. And on the status bar: "Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor." — Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html, read 2026-07-30. Neither page ties any specific message to a file-write failure.

One structural point that bears on the phrase "the other files for the frame", stated plainly by the manual:

> "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

> "Image: Saves each input as a separate image file. Each input can use its own image format settings."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30. **Label: documented**, for the setting itself. What that implies for a broken write — that with Multi-Layer EXR there is only one file to break, so there are no "other files" from that node for that frame — is a step past the words, so I do not claim it as a finding beyond noting the setting exists.

### Where I looked, and what I ruled out

Searched the whole of `manual_5.1` and `api_5.1` on disk, and all of `release_notes_5.1`, for these terms (case-insensitive), each as a plain string across every file:

`unable to write`, `cannot write`, `failed to write`, `write error`, `disk full`, `full disk`, `out of disk`, `no space`, `quota`, `storage`, `read-only`, `read only`, `not writable`, `permission`, `cannot be written`, `cannot be saved`, `failed to save`, `could not`, `unsuccessful`, `error`, `fail`, `failure`, `denied`, `abort`, `aborts`, `halts`, `cancel`, `interrupted`, `corrupt`, `partial`, `partial file`, `incomplete`, `incomplete file`, `truncat`, `zero-byte`, `empty file`, `0 bytes`, `stops the render`, `stop rendering`, `continue rendering`, `skipped`, `left behind`, `half-written`, `atomic`, `temporary file`, `.tmp`, `rename`, `flush`, `fsync`, `retry`, `retries`, `roll back`, `rolls back`, `clean up`, `delete the file`, `removes the file`, `exit code`, `return code`, `exit status`, `stderr`, `standard error`, `report`, `Placeholder`, `Overwrite`.

I also ran a combined pattern across the whole manual looking for any sentence pairing a failure word (`cannot`, `can not`, `can't`, `unable`, `fail`, `fails`, `failed`, `failure`, `denied`, `invalid`, `missing`) with a writing word (`write`, `writing`, `save`, `saving`, `output`, `file`, `directory`, `folder`, `path`, `disk`) within forty characters, and the same across the API documentation. The manual returned eleven hits, all about importing other formats, missing linked files, packed data, asset libraries, relative paths in unsaved files, and geometry-node string formatting. The API returned none.

Pages I opened in full and ruled out, with the reason:

- `https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html` — the node's own page. Read whole. No failure behaviour of any kind.
- `https://docs.blender.org/manual/en/5.1/files/file_paths.html` — read whole, including its "Errors" section. Covers template errors only, and only states the consequence for the render output path.
- `https://docs.blender.org/manual/en/5.1/render/output/properties/output.html` — the scene's Output panel. Read the whole first half covering File Path, Saving, Media Type, File Format, Overwrite and Placeholders. Overwrite is "Overwrite existing files when rendering" and Placeholders is "Create empty placeholder frames while rendering", with a hint about using them across several machines on a shared network directory. Both are properties of the scene's render output, and neither this page nor the node's page connects them to the File Output node. Ruled out: it is about a different writer, and says nothing about a write that fails.
- `https://docs.blender.org/manual/en/5.1/render/output/animation.html` — read whole. The nearest it comes is advice about a computer switching off mid-render and restarting with Overwrite disabled. That is about resuming, not about a file that could not be finished, and it concerns the scene's output frames rather than a File Output node.
- `https://docs.blender.org/manual/en/5.1/compositing/limitations.html` — read whole. Covers GPU compositor hardware limits, UI freezes, and the Windows driver timeout. Nothing about writing files.
- `https://docs.blender.org/manual/en/5.1/compositing/introduction.html`, `.../compositing/index.html`, `.../compositing/sidebar.html`, `.../compositing/compositor_system.html` — searched for `file output`, `error`, `fail`, `write`, `disk`, `save`. Only generic mentions of saving an image. Ruled out.
- `https://docs.blender.org/manual/en/5.1/editors/info_editor.html` and `https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html` — read whole. They describe where messages appear, never which messages a failed write produces.
- `https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html` and `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html` — read the render page whole and the relevant argument sections. There is a logging system (`--log`, `--log-level` with levels `fatal`, `error`, `warning`, `info`, `debug`, `trace`, and `--log-file`), and an exit-code option that applies only to Python scripts: "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables." No exit code is documented for a render or a write that fails. Ruled out as an answer, kept as the record of what the command line does document.
- `https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html` and `.../troubleshooting/network.html` — checked. Crashes and blocked extension downloads. Nothing about render output.
- `https://docs.blender.org/manual/en/5.1/files/media/image_formats.html` and `.../files/media/video_formats.html` — searched for failure words. Only colour-management text came back. Ruled out.
- `https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html` — the node "Propagation" setting, which controls whether "informational messages, warnings, and errors" pass to a parent group, is labelled as belonging to Geometry Nodes. Ruled out as not applying to the compositor. The Find Node feature does say nodes "can be searched by name, socket label, warning messages, or certain socket values" (https://docs.blender.org/manual/en/5.1/interface/controls/nodes/selecting.html), which confirms nodes can carry warning messages, but nothing says a File Output node gets one when a write fails.
- API: `https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html` and `https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html` — read whole. Every property is listed with its type and default. There is no property, method, or note relating to write success, failure, or error state.
- API: `https://docs.blender.org/api/5.1/bpy.app.handlers.html` — read the complete list of handlers (forty entries). Described above.
- Release notes: searched all of `release_notes_5.1` for `file output`, and separately for `error`, `fail`, `report`, `disk`, `write`, and `compositor`. The File Output entries are: "The **Image** and **File Output** nodes now read and write custom meta-data." and "The **File Output** node now allows specifying directories in item names." (https://developer.blender.org/docs/release_notes/5.1/compositor/, read 2026-07-30). The bug-fix list mentions the node twice — a vector socket dimensions fix and a link-drag-search fix — neither about writing failures. Nothing in the notes touches error handling on write.
- Cycles' own files: **observed.** I searched every one of the 406 files under `Blender Sources/cycles_5.1` for `file_output`, `OutputFile`, and `compositor`. The only matches were in `ui.py` — the import line `from bl_ui.properties_render import draw_curves_settings, CompositorPerformanceButtonsPanel, CompositorDenoisePerformanceButtonsPanel` and the panel classes `CYCLES_RENDER_PT_performance_compositor` and `CYCLES_RENDER_PT_performance_compositor_denoise_settings`. I also searched the Python files for `self.report` and `error`; every result was about OSL script compilation and denoising, for example in `osl.py`: `report({'ERROR'}, "OSL script compilation failed, see console for errors")`. Cycles' files contain no code that writes File Output node files and no error handling for them. Ruled out as a source for this question.

I did not need the web and did not use it.

**Not covered.** Everything the question asks. The sources do not say what becomes of the file that could not be finished — whether a partial file is left on disk, emptied, or removed. They do not say whether the other files for that same frame are still written, or written and then discarded. They do not say whether the render stops at that frame or carries on to the next. They do not say what message, if any, is produced, where it appears, or whether it is severe enough to make a script calling the render operator raise. They do not say whether the File Output node itself shows a warning. They also do not say whether the answer differs by cause — a full disk, a lost network share, a folder that cannot be written to, a format that cannot hold the image — and they do not say whether Multi-Layer EXR behaves differently from separate images. The one adjacent documented fact I can offer, that missing directories are created automatically, closes off one cause of failure and nothing more.

**Outside the interface.** **Brought up to date by hand test, 2026-08-09: observed.** Every test above was run with Blender started from a terminal with no window, which is the only way Smoozy will ever run it, so the whole of the answer above is an answer about Blender driven from outside its own interface. What that route gives you: the error naming the path is printed to the terminal every time, on both machines; the process exit status is **0** in every case, so a script watching only the exit status learns nothing; and `bpy.ops.render.render()` returns `{'FINISHED'}` and raises nothing. There is no handler that fires on a failed write and no property on the node recording one, so the printed line is the only signal. Whether the same behaviour holds *inside* Blender's own window was not tested and is not settled here — the rented machine has no display server and the Creator ran nothing this round.

**Added by the second batch of hand tests, 2026-08-09.** One thing that sharpens the exit-status point above. A run that could not read the blend file at all — the same scene, started as a user who cannot enter the folder it sits in — ends with `Error: Cannot read file "…": Permission denied` and **exit status 1**. So the exit status is not simply always 0: it does report a blend file Blender could not open, and it does not report a File Output write that failed. A script watching only the exit status will therefore see the difference between "the job never started" and "the job ran", and will see no difference at all between "every file was written" and "a file could not be written".

The original note from the sources follows.

 The sources do not say. Nothing on the File Output node page, the command-line render page, or the command-line arguments page distinguishes a render driven by a person clicking from one started by a command. No exit code is documented for a failed render or a failed write, and the only documented exit-code control applies to Python exceptions in scripts. Since the manual never describes the failure at all, it also never describes how the failure is signalled to something outside Blender's window — which, for this question, is the gap that matters most.

**Operating system.** **Brought up to date by hand test, 2026-08-09: observed on two operating systems, untested on the third.** Every test above ran on Linux (Ubuntu 22.04.5 on the rented machine) and on macOS 26.5.2 on Apple silicon (the Creator's Mac). On everything the question asks the two agreed: which files survive, which do not, that the render carries on, that an error naming the path is printed, and that the exit status is 0. They differed only in wording and in one detail on disk — the bracketed reason was `(Success)` on Linux and `(Undefined error: 0)` on the Mac; and when a volume filled, Linux left **zero-byte files** for the outputs that could never be opened where the Mac left **no file at all**. In this version the Mac also printed `Unknown error in IMB_exr_begin_write` where Linux named the reason. **Windows is untested**, not answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Added by the second batch of hand tests, 2026-08-09.** One part of that last difference turns out to belong to the disk's own file system rather than to the operating system, and the first batch did not record which file system the Mac's disk image carried. Re-run on the Creator's Mac on a 12 MB **HFS+** image and again on a 12 MB **APFS** image, both filled to leave exactly 1500 KB free, the volume-full test left **one** part-written file on HFS+ and **two** on APFS, and `df -k` afterwards read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS even though the writes had failed for want of space. What did hold on both Mac file systems is the thing the line above rests on: no zero-byte file was left for an output that could never be opened, where the rented Linux machine leaves one. So the Linux-against-macOS difference stands; the count of part-written files and the `df` reading are properties of the file system as well.

The original note from the sources follows.

 The sources do not say. The node's page, the file-paths page, and the release notes make no distinction between operating systems for this node, and the only per-system material I found anywhere near the topic was the Windows graphics-driver timeout on the compositing limitations page, which concerns the GPU compositor and not writing files.

---

## Things a person with Blender 5.1 open could settle by hand in a few minutes

This list has been pruned on 2026-08-09. Most of what the researcher proposed was run on both machines and the results are written in above: a folder that cannot be written to; one output of several failing while the rest succeed; a write that fails part-way because the volume fills; whether an animation carries on; the same tests with no window, with the exit status read; two File Output nodes with one failing; Multi-Layer EXR against separate images; a broken path template in a node's own path; and whether a script driving the render can detect any of it. What is left is only what is genuinely still open.

1. **A network folder pulled away mid-render.** Point one File Output node at a mounted network folder and another at a local folder. Start an animation render, and disconnect the network while it is running. Note whether Blender stops, hangs, or continues; what appears in the terminal; and what is left at both locations afterwards. **Could not be run on 2026-08-09**, and the reason is equipment: this needs a file server between two machines, and the project has the Creator's Mac and one rented Linux machine reached over the open internet, with nothing set up to serve a share from either to the other. The network cause of a failed write is therefore untested rather than answered.

2. **A file held open and locked by another program.** **Could not be run on 2026-08-09**: neither macOS nor Linux enforces that kind of lock against an unrelated program, and no Windows machine was tried, where it would be arrangeable.

3. **The same tests on Windows.** **Not run on 2026-08-09**: no Windows machine has run it. Linux and macOS on Apple silicon are both settled above; Windows is untested, not answered.

4. **What a person sees on screen.** Whether the path field of a File Output node with a broken template is shown highlighted in red and what its tooltip says; whether the node shows a warning badge in the compositor after a failed write; and what the Info editor and the status bar hold. **Dropped on 2026-08-09 as needing a window**: the rented Linux machine has no display server, and the Creator ran nothing this round. Before dropping it, the node's own property list was read from a terminal and holds nothing carrying warning text, and 5.1 has no `bpy.context.window_manager.reports`, so there is no way to read any of this without a window.

---

2026-08-09 — Opus 5 (Claude Code) — File output nodes, Blender 5.1

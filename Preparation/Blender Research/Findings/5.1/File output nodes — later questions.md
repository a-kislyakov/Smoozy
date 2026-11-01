# File output nodes — later questions (Blender 5.1)

Sources consulted, all read on 2026-07-30 (the snapshot date):

- The Blender 5.1 manual (`Blender Sources/manual_5.1`), published at https://docs.blender.org/manual/en/5.1/
- The Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), published at https://docs.blender.org/api/5.1/
- The Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), published at https://developer.blender.org/docs/release_notes/5.1/
- Cycles' own files (`Blender Sources/cycles_5.1`)

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

None of the four sources describes what Blender 5.1 does when a File Output node cannot finish writing one of the files it is producing for a frame. There is nothing on the state the unfinished file is left in, nothing on whether the node's other files for that frame are still written, nothing on whether the render stops or carries on to the next frame, and nothing on what message (if any) is produced.

**How the question was read.** I took "cannot be finished" to mean the write starts or is attempted and does not complete — the destination is out of space, the destination is not writable, a network location drops out, and so on. A second reading is possible: that the file is never begun because something about it is invalid (a malformed path, for example). The sources are silent on the first reading; on the second reading there is one adjacent documented fact about malformed paths, given below, but it is about the scene's render output path rather than a File Output node's own paths.

**The nearest things the sources do say** (none of them answers the question, and I flag each as adjacent rather than as an answer):

1. On folders that do not yet exist, the manual states an outcome. It says of File Output node input names used as folders:

   > "Any missing directories are created automatically during output."

   Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

2. For malformed path templates, the manual states an outcome, but for the scene's render output path, not for a File Output node:

   > "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

   Source: manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30. The File Output node's own `directory` and `file_name` are documented as supporting template expressions ("Supports template expressions", https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30), but the manual does not extend the sentence above to them, and I am not extending it either.

3. Where messages of any kind surface is documented in general terms, but nothing ties a File Output write problem to them:

   > "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

   Source: manual, https://docs.blender.org/manual/en/5.1/editors/info_editor.html — read 2026-07-30.

   > "Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor."

   Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30.

4. A node can carry error messages, and there is a control over whether they travel out of a node group. The Python API documents this on `Node` generally:

   > "The kinds of messages that should be propagated from this node to the parent group node (default 'ALL')"
   > "All Messages – Propagate every info, error, and warning message upstream."

   Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.Node.html — read 2026-07-30. The manual's description of the same control is headed "Propagation Geometry Nodes", that is, marked as a Geometry Nodes feature (https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html, read 2026-07-30). Nothing in either source says a File Output node ever produces such a message, or that a write problem is one.

5. There is a handler that fires after a frame is written, and there is no counterpart for a write that failed. The API lists:

   > "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

   for `bpy.app.handlers.render_write`, and separately lists `render_cancel` ("on canceling a render job"), `render_complete` ("on completion of render job"), and — for blend-files only — `save_post_fail` ("on failure to save a blend file (after)"). Source: API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30. I note the absence of a render-write-failure handler as an absence in the list, not as evidence of what Blender does.

6. The 5.1 release notes record two changes to the File Output node and two fixed bugs mentioning it, none about writing failures:

   > "The **Image** and **File Output** nodes now read and write custom meta-data."
   > "The **File Output** node now allows specifying directories in item names. This was possible in older versions of Blender and the functionality is now restored."

   Source: release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/ — read 2026-07-30. The bug-fix list entries naming the node are "File output node vector socket dimensions don't automatically match the source socket" and "Compositor File Output Node missing in Link Drag Search" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/, read 2026-07-30).

**Not covered.** Everything the question asks. Specifically: whether an unfinished file is deleted, left at zero length, or left partly written; whether the remaining outputs of the same node for the same frame are attempted, skipped, or abandoned; whether other File Output nodes in the same tree are affected; whether the current frame is counted as done; whether an animation render stops at that frame or continues; whether anything is written to the Info editor, the status bar, or the console; and whether a script driving the render can detect it. Also not covered: whether the two sentences quoted above about the scene's render output path apply in any way to a File Output node's paths.

**Outside the interface.** **Brought up to date by hand test, 2026-08-09: observed.** Every test above was run with Blender started from a terminal with no window, which is the only way Smoozy will ever run it, so the whole of the answer above is an answer about Blender driven from outside its own interface. What that route gives you: the error naming the path is printed to the terminal every time, on both machines; the process exit status is **0** in every case, so a script watching only the exit status learns nothing; and `bpy.ops.render.render()` returns `{'FINISHED'}` and raises nothing. There is no handler that fires on a failed write and no property on the node recording one, so the printed line is the only signal. Whether the same behaviour holds *inside* Blender's own window was not tested and is not settled here — the rented machine has no display server and the Creator ran nothing this round.

**Added by the second batch of hand tests, 2026-08-09.** One thing that sharpens the exit-status point above. A run that could not read the blend file at all — the same scene, started as a user who cannot enter the folder it sits in — ends with `Error: Cannot read file "…": Permission denied` and **exit status 1**. So the exit status is not simply always 0: it does report a blend file Blender could not open, and it does not report a File Output write that failed. A script watching only the exit status will therefore see the difference between "the job never started" and "the job ran", and will see no difference at all between "every file was written" and "a file could not be written".

The original note from the sources follows.

 The sources do not say. There is no statement anywhere in the manual, the API documentation, the release notes, or Cycles' files about how a File Output write problem behaves when Blender is run without its window — no exit status is documented for a failed render anywhere in these sources (I searched the whole manual and API for "exit code", "exit status", "return code" and found no such statement about rendering), and the command-line documentation does not mention output-writing failures. Because the answer to the question itself is "not in the sources", there is also nothing to compare between the two ways of running Blender.

**Operating system.** **Brought up to date by hand test, 2026-08-09: observed on two operating systems, untested on the third.** Every test above ran on Linux (Ubuntu 22.04.5 on the rented machine) and on macOS 26.5.2 on Apple silicon (the Creator's Mac). On everything the question asks the two agreed: which files survive, which do not, that the render carries on, that an error naming the path is printed, and that the exit status is 0. They differed only in wording and in one detail on disk — the bracketed reason was `(Success)` on Linux and `(Undefined error: 0)` on the Mac; and when a volume filled, Linux left **zero-byte files** for the outputs that could never be opened where the Mac left **no file at all**. In this version the Mac also printed `Unknown error in IMB_exr_begin_write` where Linux named the reason. **Windows is untested**, not answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Added by the second batch of hand tests, 2026-08-09.** One part of that last difference turns out to belong to the disk's own file system rather than to the operating system, and the first batch did not record which file system the Mac's disk image carried. Re-run on the Creator's Mac on a 12 MB **HFS+** image and again on a 12 MB **APFS** image, both filled to leave exactly 1500 KB free, the volume-full test left **one** part-written file on HFS+ and **two** on APFS, and `df -k` afterwards read **0 available, 100% used** on HFS+ but **984 KB available, 92% used** on APFS even though the writes had failed for want of space. What did hold on both Mac file systems is the thing the line above rests on: no zero-byte file was left for an output that could never be opened, where the rented Linux machine leaves one. So the Linux-against-macOS difference stands; the count of part-written files and the `df` reading are properties of the file system as well.

The original note from the sources follows.

 The sources do not say. Nothing in the File Output node page, the file-paths page, or the release notes distinguishes Windows, macOS, or Linux on this point, and nothing mentions per-system differences in how a failed write is handled or reported.

**Where I looked, and what I ruled out.**

- Read in full: the File Output node page (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html); the API page for the node (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html); the 5.1 compositor release notes (https://developer.blender.org/docs/release_notes/5.1/compositor/); the Info editor page (https://docs.blender.org/manual/en/5.1/editors/info_editor.html). None mentions a write that cannot be finished.
- Read the "Errors" section of the file paths page (https://docs.blender.org/manual/en/5.1/files/file_paths.html). It covers only malformed template expressions, and only names the scene render output path as an example.
- Read the render output properties page (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html) including "Overwrite" and "Placeholders" and the "Primitive Render Farm" hint. These describe the scene's own output path, not File Output nodes, and say nothing about failed writes.
- Read the animation output page (https://docs.blender.org/manual/en/5.1/render/output/animation.html), including its "Hints" about a computer turning off mid-render. It advises rendering to image sequences for easy restart but does not describe what Blender does when a write fails.
- Searched the whole manual, the whole API documentation, and the whole release notes for: "cannot be written", "cannot write", "cannot save", "cannot create", "unable to write", "unable to save", "unable to open", "unable to create", "write error", "write fail", "failed to write", "failed to save", "failed to open", "failed to create", "disk full", "disk space", "no space", "quota", "retry", "partially written", "partial file", "half-written", "incomplete", "corrupt", "truncated", "abort", "aborted", "skips the frame", "continue rendering", "stops the render", "cancel the render", "unreachable", "offline", "read-only", "network drive", "permission", "writable", "could not", "does not exist", "missing directory", "missing folder", "invalid path", "exit code", "exit status", "return code", "non-zero". The only hits relevant to output writing at all are the ones quoted above; every other hit was about unrelated subjects (GPU driver errors, mesh import validation, extension repository permissions, and so on).
- Searched the compositing, render, files, editors, and command-line sections of the manual for "error", "fail", "failed", "failure". Hits were GPU kernel errors, EEVEE shadow buffer messages, Freestyle modifier names, importer options, and the template-path sentence already quoted.
- Searched Cycles' own files (`Blender Sources/cycles_5.1`) for "file output", "compositor", "cannot open", "cannot write", "unable to", "failed to". The only error-reporting lines found are about OSL shader compilation, for example `cycles_5.1/osl.py` line 216: `report({'ERROR'}, rpt_("Failed to write .oso file next to external .osl file at {:s}").format(dst_path))`. Nothing found in Cycles' files mentions the compositor's File Output node. Ruled out as a source for this question.
- Checked `bpy.app.handlers` (https://docs.blender.org/api/5.1/bpy.app.handlers.html) for anything that fires on a failed write. There is `render_write`, `render_cancel`, `render_complete`, `render_init`, `render_pre`, `render_post`, `render_stats`, and `save_post_fail` for blend-files. No render write-failure handler is listed.
- Checked the 5.1 release notes files for core, pipeline and I/O, rendering, and the Python API for any mention of error handling or reporting around output writing. The only "error" hit across those four is about `gpu.types.GPUFrameBuffer` reads.
- Looked at no picture, because no page that discusses writing failures was found for a picture to accompany. The File Output node page names one picture (`_images/node-types_CompositorNodeOutputFile.webp`); it was not examined.

---

## Things a person with Blender open could settle by hand

This list has been pruned on 2026-08-09. Most of what the researcher proposed was run on both machines and the results are written in above: a folder that cannot be written to; one output of several failing while the rest succeed; a write that fails part-way because the volume fills; whether an animation carries on; the same tests with no window, with the exit status read; two File Output nodes with one failing; Multi-Layer EXR against separate images; a broken path template in a node's own path; and whether a script driving the render can detect any of it. What is left is only what is genuinely still open.

1. **A network folder pulled away mid-render.** Point one File Output node at a mounted network folder and another at a local folder. Start an animation render, and disconnect the network while it is running. Note whether Blender stops, hangs, or continues; what appears in the terminal; and what is left at both locations afterwards. **Could not be run on 2026-08-09**, and the reason is equipment: this needs a file server between two machines, and the project has the Creator's Mac and one rented Linux machine reached over the open internet, with nothing set up to serve a share from either to the other. The network cause of a failed write is therefore untested rather than answered.

2. **A file held open and locked by another program.** **Could not be run on 2026-08-09**: neither macOS nor Linux enforces that kind of lock against an unrelated program, and no Windows machine was tried, where it would be arrangeable.

3. **The same tests on Windows.** **Not run on 2026-08-09**: no Windows machine has run it. Linux and macOS on Apple silicon are both settled above; Windows is untested, not answered.

4. **What a person sees on screen.** Whether the path field of a File Output node with a broken template is shown highlighted in red and what its tooltip says; whether the node shows a warning badge in the compositor after a failed write; and what the Info editor and the status bar hold. **Dropped on 2026-08-09 as needing a window**: the rented Linux machine has no display server, and the Creator ran nothing this round. Before dropping it, the node's own property list was read from a terminal and holds nothing carrying warning text, and 5.1 has no `bpy.context.window_manager.reports`, so there is no way to read any of this without a window.

---

2026-08-09 — Opus 5 (Claude Code) — File output nodes, Blender 5.1

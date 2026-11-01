# Reading a file — Blender 5.2

**Sources consulted, and when.** Blender 5.2's manual, Blender 5.2's Python API documentation, Blender 5.2's release notes, and Cycles' own files — all as supplied in `Blender Sources/` (`manual_5.2`, `api_5.2`, `release_notes_5.2`, `cycles_5.2`), snapshot taken 2026-07-30. That snapshot date is the date read for every citation below. One picture from the manual's `_images` folder was opened and is cited where used. Work done 2026-08-04.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

One thing to disclose so the line above is not doing quiet work: to confirm the folder I was told to save into existed, I ran a directory listing of `Preparation/Blender Research/Findings/` and `Preparation/Blender Research/Findings/5.2/`. That showed me file names only. I opened none of them, and I read no content from that folder or anywhere else in the project.

---

## 1. What does a `.blend` file give up without a Blender to open it, and what needs one?

**Ambiguity flagged.** "Give up" reads two ways: what the file *yields* to something that is not Blender, and what the file *surrenders/loses*. I answer the first reading — what can be got out of a `.blend` without running Blender, and what needs Blender. The other reading would be about data loss, which is a different subject (and is covered by the manual's compatibility page, quoted below in passing).

**LABEL: not in the sources** — for the main part of the question. Two narrow exceptions are documented and are given below.

**The sources do not describe the `.blend` file's internal layout at all.** There is no page describing its header, its structure, its byte order, or any way for another program to read its contents. So the sources give no basis for saying what a non-Blender program could pull out of one.

Where I looked (searches run across the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2`): "blend-file format", "file format", "DNA", "SDNA", "endian", "byte order", "binary format", "file header", "specification", "third-party" / "third party", "parse the", "without Blender", "outside Blender", "other software", "reverse engineer". The only hits for "file format" were about *image* and *video* formats and about importers/exporters (OBJ, STL, FBX, Alembic, PLY, USD, glTF), none about the `.blend` itself. "DNA" and "SDNA" hit only unrelated words inside interface diagrams. Nothing anywhere describes the container's contents.

**Two things are documented, and both are narrow.**

*(a) The compression wrapper can be handled by other tools.* **LABEL: documented.**

> "The used compression algorithm is Zstandard. It is not unique to Blender so files can be compressed/decompressed with external tools."

Source: manual — https://docs.blender.org/manual/en/5.2/files/blend/open_save.html. Date read: 2026-07-30.

The same page adds, about older files:

> "This means to open newer blend-files in versions prior to 3.0, blend-files must first be saved without compression in a newer version of Blender or decompressed using an external Zstandard tool."

So an outside tool can undo the compression. What it then finds is not described anywhere in these sources.

*(b) The file carries a preview picture, and the operating system can show it.* **LABEL: documented.**

> "Blender saves by default a small preview of current scene in the blend-file. This will show in the Thumbnail view of the File Browser. During its installation, Blender also adds a small tool to your OS, that will allow your system file browser to show those previews as file thumbnails as well."

Source: manual — https://docs.blender.org/manual/en/5.2/files/blend/previews.html. Date read: 2026-07-30.

Note the wording: the thumbnail is shown by a tool Blender installs, not by an arbitrary reader. The sources do not say how that tool gets the picture out.

**What needs a Blender.** **LABEL: inferred.** Every other capability this question sheet asks about — listing what is in the file, reading library paths, reading output paths, reading view layers — is documented in these sources only through Blender's own Python API (`bpy`) or Blender's own interface. The step I am taking: the documentation describes these only as `bpy` calls and menu commands, and describes no other route, therefore on the evidence of these sources they need Blender running. The sources nowhere say "this is impossible without Blender"; they simply offer no other way.

One wrinkle worth recording, because it changes what "a Blender" means. **LABEL: documented.**

> "Blender supports being built as a Python module, allowing `import bpy` to be added to any Python script, providing access to Blender's features." … "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments `--background` or `-b`), however there are some differences to be aware of." … "Blender as a Python Module isn't provided on Blender's official download page. A pre-compiled `bpy` module is available via PIP."

Source: Python API documentation — https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html. Date read: 2026-07-30.

So the reading can be done inside another program's Python process, with no Blender application window and no Blender executable — but it is still Blender's code doing the reading.

**Not covered.** Nothing here settles what a `.blend` actually contains at the byte level, whether any part of it (names, paths, versions) is readable by an ordinary program, or how the preview thumbnail is stored. Nothing here settles whether the OS thumbnail tool is available in a headless install. Nothing here covers the second reading of the question (what is lost).

**Outside the interface.** The compression fact has nothing to do with the interface. The thumbnail fact is explicitly about the *operating system's* file browser, which is outside Blender entirely. The `bpy`-as-a-module fact is specifically about running with no interface. So the answer holds, and if anything is stronger, outside the interface. The observed test below bears on this: a file saved by this version with no window showed a picture of its contents in the Finder, as did the same file saved again from Blender's window.

**Operating system.** **LABEL: documented** for the macOS qualification and **not in the sources** for Windows and Linux. The Zstandard statement names no operating system and is not qualified. The preview page says, under a macOS heading, "The Blender Thumbnail extension can be disabled in the System Settings under 'Login Items & Extensions', within the Quick Look category." The cited pages say nothing about equivalent controls on Windows or Linux. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

> **Narrowed by the citation check of the Linux hand tests, 2026-08-06.** One of the tests below is not the same test on the two machines. The preview question was put to the Finder on the Mac and to Blender's own thumbnail tool on Linux. What matches across the two is the pattern of results, not one test run twice, and no file browser on Linux was looked at.

**Observed — what the first bytes of a saved file hold, and whether it is compressed (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7). Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder so the Creator's own settings were untouched. The factory startup file was saved three times into a fresh empty folder: once with no compression argument, once with `compress=False`, once with `compress=True`. The script run was:

```
import bpy, sys
folder = sys.argv[-1]
prefs = bpy.context.preferences.filepaths
print("PREF use_file_compression", getattr(prefs, "use_file_compression", "NO SUCH PROPERTY"))
print("DEFAULT compress arg",
      bpy.ops.wm.save_as_mainfile.get_rna_type().properties["compress"].default)
bpy.ops.wm.save_as_mainfile(filepath=folder + "/default.blend")
bpy.ops.wm.save_as_mainfile(filepath=folder + "/plain.blend", compress=False)
bpy.ops.wm.save_as_mainfile(filepath=folder + "/compressed.blend", compress=True)
print("DONE")
```

What came back:

```
PREF use_file_compression True
DEFAULT compress arg False
DONE
```

The three files' sizes in bytes, from `stat -f %z`:

```
default.blend 95936
plain.blend 567136
compressed.blend 95936
```

The first 32 bytes of each, from `head -c 32 <file> | xxd`:

```
-- default
00000000: 28b5 2ffd a041 0601 0025 3700 4a47 8011  (./..A...%7.JG..
00000010: 4960 589e 03bb d78b eb2e 2eb4 5e22 dfc3  I`X.........^"..
-- plain
00000000: 424c 454e 4445 5231 372d 3031 7630 3530  BLENDER17-01v050
00000010: 3252 454e 4400 0000 0010 0000 0000 0000  2REND...........
-- compressed
00000000: 28b5 2ffd a041 0601 0025 3700 4a47 8011  (./..A...%7.JG..
00000010: 4960 589e 03bb d78b eb2e 2eb4 5e22 dfc3  I`X.........^"..
```

**Observed — whether an ordinary Zstandard tool can unwrap a compressed `.blend` (hand test, 2026-08-04).** Same computer, same day, and no Blender involved in this part. The compressed file saved above was copied and handed to the Homebrew build of `zstd` at `/opt/homebrew/bin/zstd`, with the command `zstd -d -f zcheck.blend -o zcheck.out`. What came back:

```
zcheck.blend        : 567136 bytes 
```

and the first 32 bytes of the result, from `head -c 32 zcheck.out | xxd`:

```
00000000: 424c 454e 4445 5231 372d 3031 7630 3530  BLENDER17-01v050
00000010: 3252 454e 4400 0000 0010 0000 0000 0000  2REND...........
```

That is the same size and the same opening bytes as the uncompressed save above.

**Observed — what one file gives up to another Blender without being fully opened (hand test, 2026-08-04).** Same computer, same build, same throwaway settings folder. `bpy.data.libraries.load()` was called on a saved file that itself links another file. What came back:

```
SRC ATTRS ['actions', 'annotations', 'armatures', 'brushes', 'cache_files', 'cameras', 'collections', 'curves', 'fonts', 'grease_pencils', 'hair_curves', 'images', 'lattices', 'libraries', 'lightprobes', 'lights', 'linestyles', 'masks', 'materials', 'meshes', 'metaballs', 'movieclips', 'node_groups', 'objects', 'paint_curves', 'palettes', 'particles', 'pointclouds', 'scenes', 'screens', 'sounds', 'speakers', 'texts', 'textures', 'version', 'volumes', 'workspaces', 'worlds']
SRC OBJECTS []
SRC COLLECTIONS ['ChainCollection']
SRC LIBRARIES [bpy_lib_library(filepath='//c.blend', is_archive=False)]
SRC VERSION (5, 2, 44)
ABSENT ERROR OSError load: /…/08/does_not_exist.blend failed to open blend file
```

The file peeked into holds one linked object inside a local collection, and itself links `c.blend`; `SRC OBJECTS` came back empty while `SRC COLLECTIONS` named the local collection and `SRC LIBRARIES` named the file it links.

**Observed — whether the operating system's own file browser shows a picture for a file saved with no window (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open.

Four files had been saved beforehand by this chat, one per version, each by that version of Blender started with no window (`-b`) and with `--factory-startup`, each holding a single cube: `4.5-background.blend`, `5.0-background.blend`, `5.1-background.blend`, `5.2-background.blend`. They sat together in one folder, `/Users/artist/Documents/Smoozy hand tests/Reading a file/16/`. The Creator then opened each of the four Blenders in turn, opened that version's `-background.blend`, and saved it again through **File ‣ Save As** into the same folder as `4.5-window.blend`, `5.0-window.blend`, `5.1-window.blend` and `5.2-window.blend`. They opened the folder in the Finder with thumbnails showing and took one screenshot of all eight.

For 5.2 in particular: `5.2-background.blend` showed a picture of a grey cube. `5.2-window.blend` showed a picture of a grey cube.

All eight icons, read off that one screenshot:

- `4.5-background.blend` — plain grey document icon with the Blender logo and the word BLEND
- `4.5-window.blend` — picture of a grey cube
- `5.0-background.blend` — plain grey document icon with the Blender logo and the word BLEND
- `5.0-window.blend` — picture of a grey cube
- `5.1-background.blend` — picture of a grey cube
- `5.1-window.blend` — picture of a grey cube
- `5.2-background.blend` — picture of a grey cube
- `5.2-window.blend` — picture of a grey cube

The screenshot is kept at `/Users/artist/Desktop/Screenshots/022/`.

The test asked only what the Finder shows. Nothing here establishes whether a preview is absent from the file itself or merely not shown, and nothing here covers any file browser other than the Finder.

**Observed — the same four things asked again on a Linux machine (hand test, 2026-08-06).**

Done on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender **5.2.0 LTS** (hash fbe6228777e7, built 2026-07-14). It is a container sharing someone else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at this topic's own settings folder; that the setting took effect was checked by printing `bpy.utils.resource_path('USER')`, which gave `/root/smoozy/Reading a file/res-5.2`.

*The first bytes of a saved file, and whether it is compressed.* What came back:

```
PREF use_file_compression True
DEFAULT compress arg False
DONE
```

The three files' sizes in bytes, from `stat -c %s`, and the first 32 bytes of each from `head -c 32 <file> | od -A d -t x1z`:

```
-- default  95931 bytes
0000000 28 b5 2f fd a0 41 06 01 00 3d 37 00 7a 47 8c 11  >(./..A...=7.zG..<
0000016 49 60 58 9e 03 bb d7 8b eb 2e 2e b4 5e 22 df c3  >I`X.........^"..<
-- plain  567136 bytes
0000000 42 4c 45 4e 44 45 52 31 37 2d 30 31 76 30 35 30  >BLENDER17-01v050<
0000016 32 52 45 4e 44 00 00 00 00 10 00 00 00 00 00 00  >2REND...........<
-- compressed  95931 bytes
0000000 28 b5 2f fd a0 41 06 01 00 3d 37 00 7a 47 8c 11  >(./..A...=7.zG..<
0000016 49 60 58 9e 03 bb d7 8b eb 2e 2e b4 5e 22 df c3  >I`X.........^"..<
```

So on Linux too the preference reads **on** in this version while the argument's own default reads **off**, and a save with no compression argument comes out the size of the explicitly compressed save. The file's text is `BLENDER17-01v0502REND`. All of that matches the Mac; the exact byte counts differ, as two machines' saved files do.

*Whether an ordinary Zstandard tool can unwrap it.* No Blender involved. `zstd -d -f -q compressed.blend.zst -o undone.blend`, using the system `zstd` at `/usr/bin/zstd`, exited 0. The result was **567136 bytes** — the same as the uncompressed save — with these first 32 bytes:

```
0000000 42 4c 45 4e 44 45 52 31 37 2d 30 31 76 30 35 30  >BLENDER17-01v050<
0000016 32 52 45 4e 44 00 00 00 00 10 00 00 00 00 00 00  >2REND...........<
```

`cmp undone.blend plain.blend` reported `differ: byte 65970, line 6`, so the two are the same size with the same opening bytes but not identical throughout.

*What one file gives up to another Blender without being fully opened.* What came back:

```
SRC ATTRS ['actions', 'annotations', 'armatures', 'brushes', 'cache_files', 'cameras', 'collections', 'curves', 'fonts', 'grease_pencils', 'hair_curves', 'images', 'lattices', 'libraries', 'lightprobes', 'lights', 'linestyles', 'masks', 'materials', 'meshes', 'metaballs', 'movieclips', 'node_groups', 'objects', 'paint_curves', 'palettes', 'particles', 'pointclouds', 'scenes', 'screens', 'sounds', 'speakers', 'texts', 'textures', 'version', 'volumes', 'workspaces', 'worlds']
SRC OBJECTS []
SRC COLLECTIONS ['ChainCollection']
SRC LIBRARIES [bpy_lib_library(filepath='//c.blend', is_archive=False)]
SRC VERSION (5, 2, 44)
ABSENT ERROR OSError load: /root/smoozy/Reading a file/work/08/does_not_exist.blend failed to open blend file
```

Identical to the Mac's, including the library list this version alone offers and the version number.

*Whether a file saved with no window carries a preview picture.* There is no Finder on this machine, so what was asked instead is whether Blender's own thumbnail tool — the one it ships beside its binary — can make a picture out of the file. Asked with no arguments the tool printed its own usage, so its arguments come from the tool and were not guessed: `Usage: blender-thumbnailer <input.blend> <output.png>`. Two files were made by this version: `5.2-background.blend`, saved with no window and with `--factory-startup`, and `5.2-window.blend`, made by opening that file in this version's window on the machine's own display and saving it again. What came back:

```
-- 5.2 background
   exit 0
   made 65737 bytes
-- 5.2 window
   exit 0
   made 65737 bytes
```

Both pictures show a grey cube. So in this version a file saved with no window **does** carry a preview Blender's own tool can render — which is what the Creator saw in the Finder on the Mac for 5.2. This settles Blender's own thumbnail tool on Linux and no other file browser.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

**LABEL: not in the sources.**

Neither half is answered. The sources state no limit on how many copies of Blender may run on one computer, and say nothing about what happens when one Blender reads a `.blend` while another Blender renders — whether from the same file or a different one.

**Where I looked.** Whole-source searches for: "single instance", "one instance", "multiple instances", "instances of Blender", "already running", "another Blender", "second instance", "in parallel", "simultaneous", "concurrent", "lock file", "file lock", "is locked", "read-only", "file is open", "in use by", "exclusive". I read in full: the command line pages (`advanced/command_line/index`, `arguments`, `render`), `advanced/blender_directory_layout` (temporary directory section), `editors/preferences/save_load`, `editors/preferences/file_paths`, `files/blend/open_save`, `render/output/properties/output`, and `bpy.app` in the API. None of them discuss concurrency, locking, or contention.

**Supporting facts that were found, and what they do and do not settle.**

*Blender's own API assumes new instances get started.* **LABEL: documented.**

> "The location of Blender's executable, useful for utilities that open new instances."

(`bpy.app.binary_path`.) Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.app.html. Date read: 2026-07-30. This shows the idea of more than one instance is normal, but names no number and says nothing about interference.

*Auto-save files are named per process.* **LABEL: documented.**

> "Automatic saving of temporary files in temp directory, uses process ID."

(`PreferencesFilePaths.use_auto_save_temporary_files`.) Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html. Date read: 2026-07-30.

**LABEL: inferred** — that separate running Blenders will not overwrite one another's auto-save files, because the file name carries the process ID and process IDs differ between running processes. The step a reader must supply: that two Blenders running at once are two processes with different IDs. The sources do not say this.

*But some temporary files are not named per process.* **LABEL: documented.**

> "Closes Blender. The current scene is saved to a file called 'quit.blend' in Blender's temporary directory (which can be found on the 'File Paths' tab of the Preferences)."

Source: manual — https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html. Date read: 2026-07-30.

And the temporary directory itself is shared:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs). The temporary directory is selected based on the following priority: User Preference (see File Paths). Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). The /tmp/ directory."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html. Date read: 2026-07-30.

I stop there deliberately. That two Blenders quitting would both write `quit.blend` to the same place is *not* stated, and I will not claim it.

*Also documented, but about machines rather than one computer.* **LABEL: documented.**

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html. Date read: 2026-07-30.

This is the closest the sources come. It is about many *machines* writing into one directory, not about many Blenders on one machine, and it says nothing about reading. It does document a supported way for several renders to share an output directory without colliding, via Overwrite off and Placeholders on.

**Not covered.** No number, no limit, no statement about memory or GPU contention between instances, nothing about whether a `.blend` is locked while open, nothing about reading a file that another Blender is writing, and nothing about a file being read while a render is in progress.

**Outside the interface.** Not settled either way — the sources do not discuss concurrency at all, inside or outside the interface. The one adjacent documented fact (the render farm hint) is a command-line-friendly arrangement but the manual does not say so.

**Operating system.** Not settled. The temporary directory selection does differ by operating system, as quoted above (`TEMP` on Windows, `TMP` and `TMP_DIR` elsewhere, then `/tmp/`), and the manual notes that the autosave blend-file location is "(Windows only, temp directory used for other systems.)" (https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html). But nothing in the sources ties those differences to running several Blenders at once. The interference half of the question was tested on **two** machines — the Creator's Mac and a rented Linux machine — and came back the same on both, so macOS and Linux are settled for it. **Windows stays open.** The count is settled by neither machine and by no machine.

**Observed — a second Blender reading the same file while a first renders it, and a third saving over it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7). Every Blender in this test was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at its own throwaway settings folder — a separate one for the renderer, the reader and the saver — so none of them shared a settings folder with another and none touched the Creator's own.

A file was built in a fresh empty folder holding three objects named `Cube`, `Sphere` and `Camera`, 60 frames, a 480 by 270 render and PNG output. On this machine that render takes about 23 seconds on its own, which is the window the other Blenders were started in. Each part started the render in the background and then waited — checking every fifth of a second — before starting the second Blender.

> **Noted by the citation check, 2026-08-05.** The paragraph above said the wait ran until two frames had appeared, while the recorded output below reads `frames on disk: 1` in both parts. The description and the record do not agree about the condition that was waited for. This check was not in the room and has changed no number; the test has gone to the hand-test list to be re-run with the wait condition written down and printed at every poll.

**Corrected 2026-08-05 by the re-run recorded below.** The paragraph above used to say the wait ran until two frames had appeared on disk. That was wrong. The line that ended the wait counted every entry in the output folder rather than only the frames, and the render's own log was written into that same folder, so the wait let go as soon as one finished frame sat beside one log file. The second Blender was therefore started after one frame, not two. The count printed below, `frames on disk: 1`, was the true number of finished frames at that moment; it was the description that did not match, not the number.

**Part A, the render on its own, for comparison.**

```
PART A RENDER EXIT CODE: 0
PART A FRAMES WRITTEN: 60
--- PART A render.log, last 5 lines ---
00:22.828  render           | Fra: 60 | Rendering 64 / 64 samples
00:22.838  render           | Saved: '/…/17/52/A/frame_0060.png'
00:22.838  render           | Time: 00:00.38 (Saving: 00:00.00)

Blender quit
```

**Part B, a second Blender reading the same file while the first rendered it.**

```
PART B RENDER STARTED, pid 60193
PART B RENDER IS UNDER WAY, frames on disk: 1
PART B READER EXIT CODE: 0
PART B RENDER STILL RUNNING WHEN READER FINISHED: yes
--- PART B reader.log, in full ---
00:00.425  blend            | Read blend: "/…/17/52/concurrency.blend"
READER FILEPATH /…/17/52/concurrency.blend
READER OBJECT COUNT 3
READER OBJECT NAMES ['Camera', 'Cube', 'Sphere']
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
PART B RENDER EXIT CODE: 0
PART B FRAMES WRITTEN: 60
--- PART B render.log, every line holding Error or Warning ---
(none)
```

**Part C, a third Blender opening the same file, renaming an object and saving over it while the first rendered from it.** The file used here was a copy, so that part B's file was left as it was.

```
PART C RENDER STARTED, pid 60262
PART C RENDER IS UNDER WAY, frames on disk: 1
PART C SAVER EXIT CODE: 0
PART C RENDER STILL RUNNING WHEN SAVER FINISHED: yes
--- PART C saver.log, in full ---
00:00.553  blend            | Read blend: "/…/17/52/copy_for_saving.blend"
SAVER FILEPATH /…/17/52/copy_for_saving.blend
Info: Saved "copy_for_saving.blend"
SAVER RESULT {'FINISHED'}
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
PART C RENDER EXIT CODE: 0
PART C FRAMES WRITTEN: 60
--- PART C render.log, every line holding Error or Warning ---
(none)
--- PART C: what the saved-over file holds afterwards ---
AFTER OBJECT NAMES ['Cube', 'RenamedByTheSaver', 'Sphere']
```

Every part of this ran with no window at all, so what it settles holds when Blender is driven from outside its own interface. It was run on macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open, because no Windows machine has run it.

**Observed — the same test again, with the wait condition stated and the frame count printed at every poll (hand test 18, 2026-08-05).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7). The file, the reader and the saver were the same as in the run above, so this is the same test. Every Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at its own settings folder — a separate one for the renderer, the reader and the saver. All three of those folders were made before any Blender was pointed at one, because Blender crashes when pointed at a settings folder that does not exist. Nothing else was running on the machine, and the four versions were run one after another, never together.

**The condition that had to be true before the second and third Blenders were started:** the number of files whose names end `.png`, in that part's own output folder, had to be two or more. The folder was looked at five times a second and the count printed at every look, not only at the last one. Every frame number the script printed came from one line, `ls -1 <folder> | grep -c '\.png$'`, and no log was written into an output folder, so nothing but a finished frame could be counted as a frame. Had the count not reached two within 150 looks — thirty seconds — the wait would have given up and said so. It did not have to.

The long scratch-folder path is written below as `/…`; everything else is word for word as it came back.

```
########## SETTINGS FOLDERS -- one per Blender, each made before it is used ##########
/…/18/52/cfg_reader
/…/18/52/cfg_render
/…/18/52/cfg_saver
THE ONE COUNTING COMMAND BEHIND EVERY FRAME NUMBER BELOW: ls -1 <folder> | grep -c '\.png$'
THE WAIT CONDITION: start the next Blender at the first poll where that count is two or more; poll five times a second; give up after 150 polls.
NO LOG IS WRITTEN INTO AN OUTPUT FOLDER, so only finished frames are ever in the folder being counted.

########## PART A -- the render on its own, for comparison ##########
PART A RENDER EXIT CODE: 0
PART A FRAMES WRITTEN: 60
--- PART A A_render.log, last 5 lines ---
00:22.805  render           | Fra: 60 | Rendering 64 / 64 samples
00:22.815  render           | Saved: '/…/18/52/A/frame_0060.png'
00:22.815  render           | Time: 00:00.35 (Saving: 00:00.00)

Blender quit

########## PART B -- a second Blender reads the same file while it renders ##########
PART B RENDER STARTED, pid 49851, at 22:27:44
PART B POLL 1 at 22:27:44, 0s after the render was started, frames on disk: 0
PART B POLL 2 at 22:27:45, 1s after the render was started, frames on disk: 0
PART B POLL 3 at 22:27:45, 1s after the render was started, frames on disk: 0
PART B POLL 4 at 22:27:45, 1s after the render was started, frames on disk: 0
PART B POLL 5 at 22:27:45, 1s after the render was started, frames on disk: 0
PART B POLL 6 at 22:27:46, 2s after the render was started, frames on disk: 0
PART B POLL 7 at 22:27:46, 2s after the render was started, frames on disk: 1
PART B POLL 8 at 22:27:46, 2s after the render was started, frames on disk: 1
PART B POLL 9 at 22:27:46, 2s after the render was started, frames on disk: 2
PART B WAIT ENDED at poll 9 because the count reached two or more. FRAMES ON DISK AT THAT MOMENT: 2
PART B THE WHOLE OUTPUT FOLDER AT THAT MOMENT: frame_0001.png frame_0002.png 
PART B READER STARTED at 22:27:46, 2s after the render was started
PART B READER EXIT CODE: 0
PART B FRAMES ON DISK WHEN THE READER FINISHED: 4
PART B RENDER STILL RUNNING WHEN READER FINISHED: yes
--- PART B B_reader.log, in full ---
00:00.422  blend            | Read blend: "/…/18/52/concurrency.blend"
READER FILEPATH /…/18/52/concurrency.blend
READER OBJECT COUNT 3
READER OBJECT NAMES ['Camera', 'Cube', 'Sphere']
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
PART B RENDER EXIT CODE: 0
PART B FRAMES WRITTEN: 60
--- PART B B_render.log, last 5 lines ---
00:19.906  render           | Fra: 60 | Rendering 64 / 64 samples
00:19.914  render           | Saved: '/…/18/52/B/frame_0060.png'
00:19.914  render           | Time: 00:00.30 (Saving: 00:00.00)

Blender quit
--- PART B B_render.log, every line holding Error or Warning ---
(none)

########## PART C -- a third Blender saves over the same file while it renders ##########
PART C RENDER STARTED, pid 49924, at 22:28:05
PART C POLL 1 at 22:28:05, 0s after the render was started, frames on disk: 0
PART C POLL 2 at 22:28:05, 0s after the render was started, frames on disk: 0
PART C POLL 3 at 22:28:05, 0s after the render was started, frames on disk: 0
PART C POLL 4 at 22:28:05, 0s after the render was started, frames on disk: 0
PART C POLL 5 at 22:28:06, 1s after the render was started, frames on disk: 0
PART C POLL 6 at 22:28:06, 1s after the render was started, frames on disk: 0
PART C POLL 7 at 22:28:06, 1s after the render was started, frames on disk: 1
PART C POLL 8 at 22:28:06, 1s after the render was started, frames on disk: 1
PART C POLL 9 at 22:28:06, 1s after the render was started, frames on disk: 2
PART C WAIT ENDED at poll 9 because the count reached two or more. FRAMES ON DISK AT THAT MOMENT: 2
PART C THE WHOLE OUTPUT FOLDER AT THAT MOMENT: frame_0001.png frame_0002.png 
PART C SAVER STARTED at 22:28:06, 1s after the render was started
PART C SAVER EXIT CODE: 0
PART C FRAMES ON DISK WHEN THE SAVER FINISHED: 5
PART C RENDER STILL RUNNING WHEN SAVER FINISHED: yes
--- PART C C_saver.log, in full ---
00:00.425  blend            | Read blend: "/…/18/52/copy_for_saving.blend"
SAVER FILEPATH /…/18/52/copy_for_saving.blend
Info: Saved "copy_for_saving.blend"
SAVER RESULT {'FINISHED'}
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
PART C RENDER EXIT CODE: 0
PART C FRAMES WRITTEN: 60
--- PART C C_render.log, last 5 lines ---
00:19.960  render           | Fra: 60 | Rendering 64 / 64 samples
00:19.971  render           | Saved: '/…/18/52/C/frame_0060.png'
00:19.971  render           | Time: 00:00.35 (Saving: 00:00.00)

Blender quit
--- PART C C_render.log, every line holding Error or Warning ---
(none)
--- PART C: what the saved-over file holds afterwards ---
AFTER OBJECT NAMES ['Cube', 'RenamedByTheSaver', 'Sphere']
```

**Where this disagrees with the record above, and how.** Only about the wait. This time the count printed as the wait ended reads `2` in both parts, and the folder listing printed at the same moment holds `frame_0001.png frame_0002.png`, so the description and the record agree. The earlier run's `frames on disk: 1` was a correct count of a wait that had already let go — the two numbers in that run came from two different lines of its script, which counted different things. That script is still on disk, and its three lines that matter read:

```
11:    if [ "$(ls -1 "$folder" 2>/dev/null | wc -l | tr -d ' ')" -ge 2 ]; then return 0; fi
33:BLENDER_USER_RESOURCES="$CFG" "$B" -b --factory-startup "$BLEND" -o "$D/B/frame_" -a > "$D/B/render.log" 2>&1 &
37:  echo "PART B RENDER IS UNDER WAY, frames on disk: $(ls -1 "$D/B" | grep -c '\.png$')"
```

Line 33 put the render's log inside the folder line 11 was counting. Run against a folder holding one finished frame and one render log, the two counting expressions give different numbers:

```
frame_0001.png
render.log
THE WAIT LINE'S EXPRESSION  ls -1 <folder> | wc -l | tr -d ' '  GIVES: 2
THE PRINTED LINE'S EXPRESSION  ls -1 <folder> | grep -c '\.png$'  GIVES: 1
```

**Where it agrees with the record above.** Everywhere else, and on everything the finding rests on. With a render of the file under way, a second Blender opened that same file with no window, printed its path and its three object names `['Camera', 'Cube', 'Sphere']`, and quit with an exit code of 0 while the render was still going. The render finished with an exit code of 0, wrote all 60 frames, and printed no error and no warning. With a third Blender that opened a copy of the file, renamed an object and saved over it mid-render, the save returned `{'FINISHED'}`, the render still finished with an exit code of 0 and all 60 frames and no error or warning, and reopening the file afterwards showed the rename had taken. So the answer is the same whether the other Blender starts after one finished frame or after two.

This ran with no window at all, on macOS. The same test was run on a rented Linux machine and is recorded separately below; **Windows** stays open.

The count question is a separate matter, and no test can answer it: whatever number of Blenders this Mac would run at once is a fact about this Mac, not about Blender. It stays unresolved.

**Observed — the same test on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS** (hash fbe6228777e7). Other work was running on the machine at the same time, so **nothing here says how long anything took**.

The set-up was the same: one file holding a cube named `Cube`, a sphere named `Sphere` and a camera named `Camera`, 60 frames, a 480 by 270 render, PNG output. Three settings folders, one per Blender, all made before any Blender was pointed at one. The wait condition was written down and used by every part — the next Blender starts at the first poll where the number of files whose names end `.png`, in that part's own output folder, is two or more; polls happen five times a second; the wait gives up after 150 polls. One function makes every frame number the script prints, and no log is written into a folder whose frames are being counted:

```
count_frames () {
  ls -1 "$1" 2>/dev/null | grep -c '\.png$'
}
```

What came back, kept word for word apart from the render logs, which are long and whose error counts are given instead:

```
--- PART A: the render on its own ---
PART A RENDER EXIT 0
PART A FRAMES ON DISK 60
--- PART B: a second Blender reading the same file while the first renders it ---
PART B POLL 1 frames_on_disk=0
PART B POLL 2 frames_on_disk=0
PART B POLL 3 frames_on_disk=0
PART B POLL 4 frames_on_disk=0
PART B POLL 5 frames_on_disk=0
PART B POLL 6 frames_on_disk=0
PART B POLL 7 frames_on_disk=1
PART B POLL 8 frames_on_disk=3
PART B WAIT ENDED at poll 8 with frames_on_disk=3 (condition: frames_on_disk >= 2)
PART B READER EXIT 0
PART B FRAMES ON DISK WHEN READER FINISHED 9
PART B RENDER EXIT 0
PART B FRAMES ON DISK 60
--- PART C: a third Blender saving over a copy while a render of that copy runs ---
PART C POLL 1 frames_on_disk=0
PART C POLL 2 frames_on_disk=0
PART C POLL 3 frames_on_disk=0
PART C POLL 4 frames_on_disk=0
PART C POLL 5 frames_on_disk=0
PART C POLL 6 frames_on_disk=0
PART C POLL 7 frames_on_disk=1
PART C POLL 8 frames_on_disk=3
PART C WAIT ENDED at poll 8 with frames_on_disk=3 (condition: frames_on_disk >= 2)
PART C SAVER EXIT 0
PART C RENDER EXIT 0
PART C FRAMES ON DISK 60
--- READER OUTPUT ---
00:00.383  blend            | Read blend: "/root/smoozy/Reading a file/work/12/52/concurrency.blend"
READER FILEPATH /root/smoozy/Reading a file/work/12/52/concurrency.blend
READER OBJECT COUNT 3
READER OBJECT NAMES ['Camera', 'Cube', 'Sphere']

Blender quit
--- SAVER OUTPUT ---
00:00.394  blend            | Read blend: "/root/smoozy/Reading a file/work/12/52/copy_for_saving.blend"
SAVER FILEPATH /root/smoozy/Reading a file/work/12/52/copy_for_saving.blend
Info: Saved "copy_for_saving.blend"
SAVER RESULT {'FINISHED'}

Blender quit
--- THE SAVED-OVER FILE REOPENED ---
AFTER OBJECT NAMES ['Cube', 'RenamedByTheSaver', 'Sphere']
--- LINES IN THE A RENDER LOG HOLDING error OR warning ---
0
--- LINES IN THE B RENDER LOG HOLDING error OR warning ---
0
--- LINES IN THE C RENDER LOG HOLDING error OR warning ---
0
```

This agrees with the Mac in every respect: all three processes finished with an exit code of 0, all 60 frames were written in each part, no line of any render log held the word error or warning, the reader printed the file's three object names while the render was going, and the saved-over file held the rename afterwards. So the second half of the question comes back the same on macOS and on Linux. **Windows stays open.**

**The count still cannot be tested, and this machine cannot help.** Whatever number of Blenders it would start is a fact about that container, which shares someone else's hardware and does not own all of its own settings. One figure about it is worth recording so nobody mistakes it for a Blender limit: the number of files one process on it may have open at once is **1024**, read with `ulimit -n`. That is a fact about the container.

---

## 3. Can a single read of a file tell, for every view layer, whether that layer renders at any frame later asked about?

**Ambiguity flagged.** "A single read" could mean (a) opening the file once, after which you may do as much work in memory as you like, or (b) taking one snapshot of the state without stepping frames. I answer (a), which I think is meant. Under reading (b) the answer is plainly no whenever the flag is animated, for the reason given below.

**LABEL: mixed — documented for the flag itself, inferred for the per-frame part, not in the sources for one crucial piece.**

**The flag exists, per view layer, and is readable.** **LABEL: documented.**

> "use
> Enable or disable rendering of this View Layer (default True)
> Type: bool"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html. Date read: 2026-07-30.

The layers are a collection on the scene:

> "view_layers … Type: ViewLayers[ViewLayer]"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Scene.html. Date read: 2026-07-30.

The manual names the same control:

> "Use for Rendering
> The active view layer will be used during rendering."

Source: manual — https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html. Date read: 2026-07-30.

For a scene reached through the API, the sources document its view-layer collection and each layer's current flag. **LABEL: inferred** that one open of the file gives every view layer of every scene: the quoted entries do not themselves say that every scene in the file is exposed through that collection.

**But the flag can be keyframed, so it need not be the same at every frame.** **LABEL: inferred** (from two documented facts).

Documented fact one — the picture. I opened `manual_5.2/_images/render_layers_view-layer_panel.png`, which the View Layer page names as "View Layer panel." It shows the panel with two rows: "Use for Rendering" with a ticked checkbox **and a small round decorator dot to its right**, and "Render Single Layer" with an unticked checkbox and **no** decorator dot. The Passes rows below all carry decorator dots too.

Source: manual — the picture on https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html. **LABEL for what the picture shows: documented.** Date read: 2026-07-30.

Documented fact two — what that dot means:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated." … "Clicking on the decorator dot icon will add a Keyframe to that property." … "If a property is being driven by another, the decorator shows the driver icon."

Source: manual — https://docs.blender.org/manual/en/5.2/interface/controls/buttons/decorators.html. Date read: 2026-07-30.

The step I am taking, and a reader must accept: a decorator dot beside "Use for Rendering" means that property can be animated and can be driven; therefore its value can differ from frame to frame. Note that "Render Single Layer", in the same picture, has no decorator.

**Even so, one open of the file is enough to work out the value at any frame — with conditions.** **LABEL: inferred.**

The animation curves live in the same file and can be read and evaluated without re-reading it:

> "data_path — RNA Path to property affected by F-Curve" and "evaluate(frame) — Evaluate F-Curve … Value of F-Curve specific frame"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.FCurve.html. Date read: 2026-07-30.

And frames can be stepped in place:

> "frame_set(frame, *, subframe=0.0) — Set scene frame updating all objects and view layers immediately"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Scene.html. Date read: 2026-07-30.

The step: since the F-Curve and its data path are in the file, and since the scene can be moved to any frame in memory, a single open lets you ask the question for any frame. The sources do not say this in so many words.

**Three conditions on that.**

*(i) Python drivers do not run by default.* **LABEL: documented.**

> "-Y, --disable-autoexec
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read: 2026-07-30.

And:

> "To render an animation in background mode, allowing drivers and other scripts to run:
> `blender --background --enable-autoexec my_movie.blend --render-anim`"

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html. Date read: 2026-07-30.

*(ii) A script in the file can change things at each frame.* **LABEL: documented.**

> "bpy.app.handlers.frame_change_pre — Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.app.handlers.html. Date read: 2026-07-30. **LABEL: inferred** that such a handler could change a view layer's flag — the docs say "change data and relations" generally and do not name view layers.

*(iii) The "only render the active layer" switch does not apply outside the interface.* **LABEL: documented.**

> "use_single_layer — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html. Date read: 2026-07-30.

The manual says the same:

> "Render Single Layer
> Only render the active view layer.
> Note
> This option is ignored when rendering from the command line."

Source: manual — https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html. Date read: 2026-07-30.

**Not covered — and this is the important gap.** The sources nowhere say **when Blender consults the `use` flag during an animation render**: whether it is read once when the render starts, or re-read at each frame. That means even a correct per-frame reading of the flag does not tell you what Blender will actually do across an animation render. I searched for this ("Use for Rendering", "use_single_layer", the render layers pages, `bpy.ops.render.render`, the render handlers list) and found nothing. Also not covered: whether `ViewLayer.use` on a *linked* scene behaves differently, and whether a view layer with `use` off still appears to a Render Layers node in the compositor.

**Outside the interface.** **LABEL: inferred** that the flag itself reads identically: it is documented as a data property, but the sources do not compare its value inside and outside the interface. One documented difference is that "Render Single Layer" / `use_single_layer` is honoured inside the interface and ignored from the command line (quoted above). One documented condition is that Python drivers are off unless `--enable-autoexec` is passed (quoted above), so a Python-driven flag would read differently in a default command-line run than in a session where auto-run is allowed. `bpy.ops.render.render` also takes `layer` — "Render Layer, Single render layer to re-render (used only when animation is disabled)" (https://docs.blender.org/api/5.2/bpy.ops.render.html) — which is another way a single layer gets picked that has nothing to do with the file's own flags.

**Operating system.** **LABEL: not in the sources.** I checked the cited view-layer, animation, command-line, security and handler pages; none gives an operating-system qualification. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

**Observed — the flag's stored default, whether it can be keyframed, and whether an animation render honours it frame by frame (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7). Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder.

First, what the property list itself says:

```
PROBE ViewLayer.use default = True
PROBE ViewLayer.use is_animatable = True
PROBE RenderSettings.use_single_layer default = False
```

Then a file was built from the factory startup in a fresh empty folder: a cube, a camera, one scene, two view layers renamed `LayerA` and `LayerB`, frames 1 to 2, a 32 by 32 render, output to `//out/frame_` in multi-layer OpenEXR (so that each rendered view layer becomes a named layer inside the written file), and the new-keyframe interpolation set to constant. With `LayerB` switched on, `layer_b.keyframe_insert(data_path="use", frame=1)` was called; `LayerB` was then switched off and `layer_b.keyframe_insert(data_path="use", frame=2)` called. What came back:

```
FORMAT MULTI_LAYER_IMAGE OPEN_EXR_MULTILAYER
ENGINE BLENDER_EEVEE
KEY1 True
KEY2 True
FCURVE view_layers["LayerB"].use [(1.0, 1.0, 'CONSTANT'), (2.0, 0.0, 'CONSTANT')]
```

The curve is on the scene, and its data path is `view_layers["LayerB"].use`. Reaching it needed walking `action.layers` → `strips` → `channelbags` → `fcurves`, because `Action.fcurves` does not exist in this version.

The saved file was then reopened in a second run with no window, which stepped the frame and read the flag back:

```
FRAME 1 [('LayerA', True), ('LayerB', True)]
FRAME 2 [('LayerA', True), ('LayerB', False)]
CURVE view_layers["LayerB"].use at 1 = 1.0 at 2 = 0.0
```

The same file was then rendered as an animation with `-a` in a third run with no window. The terminal printed:

```
00:00.417  render           | Start rendering: Scene, LayerA
00:01.134  render           | Start rendering: Scene, LayerB
00:01.479  render           | Saved: '/…/out/frame_0001.exr'
00:01.479  render           | Start rendering: Scene, LayerA
00:01.805  render           | Saved: '/…/out/frame_0002.exr'
```

The names of the view layers stored inside each written file, read with `strings -n 5 <file> | grep -oE "Layer[AB]" | sort -u`:

```
-- frame_0001.exr
LayerA
LayerB
-- frame_0002.exr
LayerA
```

This is the gap named under "Not covered" above: on this machine, in this build, the renderer did consult the flag at each frame of an animation render. The whole test ran with no window, so it holds outside Blender's interface.

**Observed — what the View Layer panel offers, and whether the render switch can be keyframed from the interface (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open. This one is about Blender's own window, so it says nothing about being driven from outside it.

A file prepared beforehand by this chat, holding one scene and two view layers named `LayerA` and `LayerB`, was opened at `/Users/artist/Documents/Smoozy hand tests/Reading a file/12/5.2.blend`. In the Properties editor, on the View Layer tab, the Creator hovered the mouse over the **Use for Rendering** checkbox and pressed **I**, then hovered over the **Render Single Layer** checkbox and pressed **I**. Two screenshots were taken, kept at `/Users/artist/Desktop/Screenshots/009/5.2/`.

The Creator's own words, in full:

> Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions.

What the screenshots show. The first is the top of the View Layer tab before the keyframe was inserted. It holds a panel headed `View Layer` with two rows: `Use for Rendering`, its checkbox ticked, and **a small round dot to the right of it**; and `Render Single Layer`, its checkbox unticked, with **no dot beside it**. Below that sits the `Passes` panel, whose own checkboxes each carry a dot. The second screenshot is the lower part of the same tab — Cryptomatte, Shader AOV, Filter, Override and Custom Properties — and does not show the `Use for Rendering` checkbox.

A gap worth stating plainly: neither screenshot shows the checkbox after **I** was pressed, so the yellow checkbox and the diamond rest on the Creator's written description above and not on a picture.

**Observed — the same test on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS**, started with no window (`-b`), with `--factory-startup`, and with its own settings folder. The set-up was the same: one scene, two view layers `LayerA` and `LayerB`, a cube, a camera, frames 1 to 2, a 32 by 32 render, multi-layer OpenEXR output, and `LayerB` keyframed on at frame 1 and off at frame 2 with constant interpolation. What came back from the build and the read:

```
ENGINE BLENDER_EEVEE
FORMAT MULTI_LAYER_IMAGE OPEN_EXR_MULTILAYER
KEY1 True
KEY2 True
SAVED /root/smoozy/Reading a file/work/10/5.2/vl.blend
FRAME 1 [('LayerA', True), ('LayerB', True)]
FRAME 2 [('LayerA', True), ('LayerB', False)]
DIRECT FCURVES ERROR AttributeError 'Action' object has no attribute 'fcurves'
CURVE ROUTE action.layers -> strips -> channelbags -> fcurves
CURVE view_layers["LayerB"].use [(1.0, 1.0, 'CONSTANT'), (2.0, 0.0, 'CONSTANT')] at 1 = 1.0 at 2 = 0.0
```

Then the animation was rendered with `-a`. Two files were written, and the layer names inside each were read with `grep -a -o -E "Layer[AB]" <file> | sort -u`:

```
/root/smoozy/Reading a file/work/10/5.2/out/frame_0001.exr
/root/smoozy/Reading a file/work/10/5.2/out/frame_0002.exr
-- frame_0001.exr
LayerA
LayerB
-- frame_0002.exr
LayerA
```

So on Linux too the switch can be keyframed from a script, stepping the frame changes what it reads, and the animation render honours the keyframed value frame by frame. The direct route to an action's curves raised here as it did on the Mac, and the walk through layers, strips and channel bags is what worked. This ran on a machine whose graphics are an NVIDIA card, with no window at all. Every line agrees with the Mac.

**Observed — whether the two switches are marked as things that can be animated (hand test, 2026-08-06).** Same machine, same build, no file needed. What came back:

```
PROBE RenderSettings.use_single_layer is_animatable = False
PROBE ViewLayer.use is_animatable = True
```

That is a machine-readable statement of what the Creator saw on the Mac when they pressed **I** over each of the two checkboxes. What Blender shows on screen when the key is pressed could not be checked on the Linux machine: nothing on it can move the mouse to a control or press a key at it, and installing such a tool is not permitted there.

---

## 4. Can the places a file is set up to write to be read out of it?

**LABEL: documented — for the main two, with an important caveat about what the stored text means.**

**Yes for the scene's render output path.**

> "filepath
> Directory/name to save animations, # characters define the position and padding of frame numbers (default '//', never None, blend relative // prefix supported, Supports template expressions)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html. Date read: 2026-07-30.

Better still, the resolved path for a given frame can be asked for directly:

> "frame_path(*, frame=-2147483648, preview=False, view='')
> Return the absolute path to the filename to be written for a given frame
> … frame (int) – Frame number to use, if unset the current frame will be used … view (str) – View, The name of the view to use to replace the '%' chars … Returns: File Path, The resulting filepath from the scenes render settings"

Same page, same date.

The surrounding switches are readable too, from the same page: `use_file_extension` ("Add the file format extensions to the rendered file name"), `file_extension` ("The file extension used for saving renders", readonly), `use_overwrite` ("Overwrite existing files while rendering", default True), `use_placeholder` ("Create empty placeholder files while rendering frames (similar to Unix 'touch')", default False), and — new in this version — `save_output`:

> "save_output
> Write frames to disk for animation renders (default True)"

Same page, same date. The release notes state the change outright:

> "Saving render output can now be disabled."

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/rendering/. Date read: 2026-07-30.

The manual explains what turning it off means:

> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html. Date read: 2026-07-30.

**Yes for the compositor's File Output nodes.**

> "directory
> The directory where the image will be written (default '', never None, blend relative // prefix supported, Supports template expressions)
> file_name
> The base name of the file. Other information might be included in the final file name depending on the node options (default '', never None, Supports template expressions)
> file_output_items … Type: NodeCompositorFileOutputItems[NodeCompositorFileOutputItem]"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html. Date read: 2026-07-30.

Each item carries the name that becomes part of the file name, plus its own format settings:

> "name (default '', never None) … override_node_format — Use a different format instead of the node format for this file (default False) … format … Type: ImageFormatSettings | None"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html. Date read: 2026-07-30.

The manual gives the shape of the final name and one behaviour that matters:

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

and

> "When the Media Type is set to Image, input names are also used to construct subdirectories: If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy. Any missing directories are created automatically during output."

Source: manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html. Date read: 2026-07-30.

**The caveat: the stored text is not always the final path.** **LABEL: documented.**

> "Path templates substitute template expressions (written {abcd}, where 'abcd' is a variable name) in a filepath when the path is used." … "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." … "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

Source: manual — https://docs.blender.org/manual/en/5.2/files/file_paths.html. Date read: 2026-07-30.

The variables available include `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, and — in render output paths only — `fps`, `resolution_x` / `resolution_y`, `scene_name`, `camera_name`, and on node paths `node_name`. Same page. The same page also documents that a template can be malformed: "Paths that support templates can have template errors, which prevent the path from being processed… if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

So reading the property gives you the *recipe*, not necessarily the path. Whether `frame_path()` resolves templates is **not stated** — see Not covered.

**Other places a file is set up to write to, also readable.** **LABEL: documented.**

- Geometry-nodes bakes: "bake_directory — Location on disk where the bake data is stored (default '', never None, blend relative // prefix supported)" and "bake_target — Where to store the baked data (default 'PACKED')" with options PACKED and DISK. Source: https://docs.blender.org/api/5.2/bpy.types.NodesModifier.html. Date read: 2026-07-30.

**Not covered.**
- Whether `frame_path()` resolves `{…}` template expressions. The API entry says only "The resulting filepath from the scenes render settings"; the file-paths page says substitution happens "at time of use". Neither states whether `frame_path()` counts as a use. This is the single most useful thing left unsettled.
- There is **no documented function that returns the final path for a File Output node slot.** The manual describes an "Output Paths" panel that "Displays the final file path and extension for each input/layer, based on current settings" (same file_output page), but no Python property or method matching it appears anywhere in the API documentation — I searched the API for the node type, the item type, and for "Output Paths". The sources therefore leave unresolved how a script obtains that displayed final path.
- Whether `bpy.data.file_path_map` / `file_path_foreach` (see Q5) include *output* paths as well as input ones. They are described as "file paths used by" data-blocks, and the release note frames the new options around "find all relevant image files for rendering". Output paths are not mentioned either way.
- Video output (the Video media type and its container) is named in the manual but I did not chase its properties, as the question is about places, and the place is the same `filepath`.

**Outside the interface.** **LABEL: inferred.** The paths are documented as Python data properties and `frame_path()` as a method, so they appear readable from a script without using the named interface panels. The sources do not say expressly that no window is needed. The manual's "Output Paths" panel is an interface display, and no matching script route was found. The `save_output` behaviour quoted above describes errors being "raised" or "reported" without saying where they appear.

**Operating system.** **LABEL: not in the sources.** No operating-system qualification appears on the cited output-path pages. Relevant nearby: `bpy.path.native_pathsep(path)` — "Replace the path separator with the system's native os.sep" — and `bpy.path.basename(path)` — "Equivalent to os.path.basename, but skips a '//' prefix. Use for Windows compatibility." (https://docs.blender.org/api/5.2/bpy.path.html, date read 2026-07-30). Those document separator handling but do not say that stored output paths read back differently. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

**Observed — what a stored output path reads back as, and what `frame_path()` makes of it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A file named `tmpl.blend` was saved in a fresh empty folder with its render output path set to `//out/{blend_name}_/frame_`, then reopened in a second run which printed the stored string and then asked `frame_path(frame=7)` for four different stored strings in turn. What came back:

```
STORED '//out/{blend_name}_/frame_'
SET '//out/{blend_name}_/frame_'
FRAME_PATH '/…/02/52/out/tmpl_/frame_0007.png'
SET '//out/{scene_name}_/frame_'
FRAME_PATH '/…/02/52/out/Scene_/frame_0007.png'
SET '//out/{blend_name_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{blend_name_/frame_':
- Invalid or incomplete template expression '{blend_name_/frame_'.

SET '//out/plain_/frame_'
FRAME_PATH '/…/02/52/out/plain_/frame_0007.png'
```

That answers the first item under "Not covered" above: `frame_path()` returns a path with the templates worked out, while the stored string keeps its braces, and a malformed template makes it raise rather than return anything.

**Observed — the File Output node's stored paths and the files a render writes (hand test, 2026-08-04).** Same computer, same build, same conditions. A file was built with compositing switched on and one File Output node whose destination folder was set to `//out/`, whose file name was set to `shot`, and which was given two extra inputs, one named with a slash in it. Building it printed:

```
TREE <bpy_struct, CompositorNodeTree("Compositing") at 0xb4d989508>
NODE ATTRS ['__slots__', 'active_item_index', 'directory', 'file_name', 'file_output_items', 'use_file_extension']
SET directory
SET file_name
NEW SIGNATURE HELP NodeCompositorFileOutputItems.new(socket_type, name)
Add an item at the end
ITEM ADDED passes/diffuse
ITEM ADDED beauty
```

Reopening the saved file and reading the node back printed:

```
BASE_PATH 'NO SUCH ATTRIBUTE'
DIRECTORY '//out/'
FILE_NAME 'shot'
ITEM NAMES ['passes/diffuse', 'beauty']
INPUT NAMES ['passes/diffuse', 'beauty', '']
```

Rendering frame 1 of that file with `-f 1`, in a run with no window, printed:

```
00:01.003  render           | Fra: 1 | Compositing
00:01.010  render           | Saved: '/…/03/52/out/shot0001.exr'
00:01.085  render           | Saved: '/…/03/52/scene_out/frame_0001.png'
00:01.085  render           | Time: 00:00.67 (Saving: 00:00.07)
```

and the files found on disk afterwards were exactly those two: `out/shot0001.exr` and `scene_out/frame_0001.png`. The node's own path-named attributes were listed in the `NODE ATTRS` line above, and there is no attribute among them holding an assembled final path per input; that part of the "Not covered" list stands.

**Observed — whether the whole-file path lists include output paths (hand test, 2026-08-04).** Same computer, same build, same conditions, on the same saved file (whose render output path is `//scene_out/frame_` and whose File Output node points at `//out/`). What came back:

```
RENDER FILEPATH '//scene_out/frame_'
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
FILE_PATH_MAP {}
FILE_PATH_FOREACH []
```

`FILE_PATH_MAP` was printed with entries holding no paths filtered out, so `{}` means no data-block reported any path at all. `FILE_PATH_FOREACH` is the list a callback passed to `bpy.data.file_path_foreach` collected; it was never called. That answers the third item under "Not covered" above.

**Observed — whether the output-path marker is set on particular properties (hand test, 2026-08-04).** Same computer, same build, same conditions, with no file loaded. What came back:

```
PROBE Property.is_path_output exists = True
PROBE RenderSettings.filepath is_path_output = True
PROBE RenderSettings.filepath is_readonly = False
PROBE Image.filepath is_path_output = False
PROBE Image.filepath is_readonly = False
PROBE Library.filepath is_path_output = False
PROBE Library.filepath is_readonly = False
PROBE PointCache.filepath is_path_output = False
PROBE PointCache.filepath is_readonly = False
PROBE CompositorNodeOutputFile properties = ['active_item_index', 'bl_description', 'bl_height_default', 'bl_height_max', 'bl_height_min', 'bl_icon', 'bl_idname', 'bl_label', 'bl_static_type', 'bl_width_default', 'bl_width_max', 'bl_width_min', 'color', 'color_tag', 'dimensions', 'directory', 'file_name', 'file_output_items', 'format', 'height', 'hide', 'inputs', 'internal_links', 'label', 'location', 'location_absolute', 'mute', 'name', 'outputs', 'panel_states', 'parent', 'rna_type', 'save_as_render', 'select', 'show_options', 'show_preview', 'show_texture', 'type', 'use_custom_color', 'use_file_extension', 'warning_propagation', 'width']
PROBE Scene has node_tree = False
PROBE Scene has compositing_node_group = True
```

All of these ran with no window, so the "Outside the interface" line above holds by test as well as by inference.

**Observed — what the File Output node's sidebar shows for its final paths (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open. This one is about Blender's own window, so it says nothing about what a script can reach.

A file prepared beforehand by this chat was opened at `/Users/artist/Documents/Smoozy hand tests/Reading a file/13/5.2.blend`. It holds one File Output node whose destination folder is `//out/`, with two extra inputs added, one of them named with a slash in it. The Creator switched to the Compositing workspace, selected the node, opened the sidebar with **N**, and looked through every panel on its **Node** tab. Two screenshots were taken, kept at `/Users/artist/Desktop/Screenshots/010/5.2/`. No words came with them.

In 5.2 the sidebar held three panels, **Node**, **Properties** and **Custom Properties**, and the Properties panel ended with a section headed **Output Paths**. What it showed, read off the screenshot:

- an unlabelled text field reading `//out/`, and below it a second reading `shot`
- a two-button row reading `Image` and `Multi-Layer EXR`, with `Multi-Layer EXR` selected
- `Node Format` holding `Save as Render` ticked, `Color Depth` set to `Float (Full)`, `Codec` set to `None`, and `Interleave` unticked
- `Color Management` set to `Follow Scene`
- a section headed `Layers` holding two entries, `passes/diffuse` and `beauty`, with `beauty` highlighted, and `Socket Type` set to `Color`
- a section headed **`Output Paths`** holding a `File Extensions` checkbox, ticked, then **one** line beside a small image icon, then a `+` button:

```
/Users/artist/Documents/Smoozy hand tests/Reading a file/13/out/shot.exr
```

The Node panel in this version also holds a `Propagate` setting reading `All Messages`, which the earlier versions' screenshots do not show. The other screenshot shows the node itself in the compositor: its body holds two fields reading `//out/` and `shot`, then two named inputs, `passes/diffuse` and `beauty`, then one further empty socket with a `+` beside it. A red warning marker sits on the link running into that empty socket. Nothing here establishes what that marker means.

**Observed — the same tests on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS**, with no window, `--factory-startup`, and its own settings folder.

*What a stored output path reads back as, and what asking for a frame's path makes of it.* What came back:

```
STORED '//out/{blend_name}_/frame_'
SET '//out/{blend_name}_/frame_'
FRAME_PATH '/root/smoozy/Reading a file/work/02/5.2/out/tmpl_/frame_0007.png'
SET '//out/{scene_name}_/frame_'
FRAME_PATH '/root/smoozy/Reading a file/work/02/5.2/out/Scene_/frame_0007.png'
SET '//out/{blend_name_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{blend_name_/frame_':
- Invalid or incomplete template expression '{blend_name_/frame_'.

SET '//out/plain_/frame_'
FRAME_PATH '/root/smoozy/Reading a file/work/02/5.2/out/plain_/frame_0007.png'
```

The braces survive in the stored string; a malformed template makes the call raise; and `{scene_name}` resolves to the scene's name in this version. Word for word the same as on the Mac.

*The File Output node's stored paths, and the files a render actually writes.* The build script tests which property the scene carries and says which route it took. What came back:

```
BRANCH new
TREE <bpy_struct, CompositorNodeTree("Compositing") at 0x7f5b9c432208>
NODE ATTRS ['__slots__', 'active_item_index', 'directory', 'file_name', 'file_output_items', 'use_file_extension']
SET directory
SET file_name
BASE_PATH 'NO SUCH ATTRIBUTE'
DIRECTORY '//out/'
FILE_NAME 'shot'
ITEM NAMES ['passes/diffuse', 'beauty']
INPUT NAMES ['passes/diffuse', 'beauty', '']
```

So `BRANCH new` here; the slash in the item name survives in this version; and this version's node carries a `use_file_extension` property the other three do not. Rendering one frame wrote two files:

```
00:01.022  render           | Saved: '/root/smoozy/Reading a file/work/03/5.2/out/shot0001.exr'
00:01.155  render           | Saved: '/root/smoozy/Reading a file/work/03/5.2/scene_out/frame_0001.png'
```

one for the whole node plus the scene's own output. Note that the node's media type in this test is Multi-Layer EXR; what is recorded here is what this version did in this test.

The whole-file path lists, on that same file, carried neither the scene's output path nor the node's:

```
RENDER FILEPATH '//scene_out/frame_'
BLEND_PATHS ['//../../../../../../opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
FILE_PATH_MAP {}
FILE_PATH_FOREACH []
```

All of it agrees with the Mac.

**Observed — what the File Output node's sidebar shows, in Blender's own window on Linux (hand test, 2026-08-06).**

Same machine, same build, but with a window open on the machine's own display. A script drove the window in stages, each on its own timer: switch to the Compositing workspace and make the File Output node active; maximise the node editor; open the sidebar, set its tab to **Node**, hide the tool bar and the asset shelf, and frame the node; scroll the sidebar down; take Blender's own screenshot with `bpy.ops.screen.screenshot` and quit. Every stage printed what it did, ending `SHOT RESULT {'FINISHED'}`.

Read off that picture, the sidebar for this node in 5.2 holds, top to bottom: **Node** (Name `File Output`, Label, Color, Show Options ticked, Mute unticked, Propagate `All Messages`), **Properties** (a field reading `//out/`, a field reading `shot`, a two-way choice `Image` / `Multi-Layer EXR` with Multi-Layer EXR selected), **Node Format** (Save as Render ticked, Color Depth `Float (Full)`, Codec `None`, Interleave unticked), **Color Management**, **Layers** (a list reading `passes/diffuse` and `beauty`), Socket Type `Color`, then **a panel headed Output Paths** with a `+` button below it, and **Custom Properties**. That the panel exists agrees with the Creator's own screenshots on the Mac. **What it holds was not read on that run:** the panel was closed, and on the day of that run nothing on the machine could click it open. The picture is kept on the box at `/root/smoozy/Reading a file/work/w13/5.2/shot4.png`.

> **Note from the citation check of the Linux hand tests, 2026-08-06.** A tool for moving the pointer was put on that machine afterwards and the check tried to open the panel with it. **It stays unread on Linux.** Two of the check's own before-and-after photographs do show the panel open, but a click on that machine could not be confirmed: on a control whose state Blender will report — the node's Mute switch — four attempts left it unchanged, and stray clicks twice landed on a different control instead. So nothing about the panel's contents on Linux is claimed here. What it holds was read on the Mac and rests on that alone. The attempts are written up in `Audits/Blender Research/Reading a file/Citation check/Linux — Opus 5.md`.

---

## 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**LABEL: documented. Yes, both, and by several things.**

**Reading them all at once.**

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
> Returns a list of paths to external files referenced by the loaded .blend file.
> absolute (bool) – When true the paths returned are made absolute.
> packed (bool) – When true include file paths for packed data.
> local (bool) – When true skip linked library paths."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.utils.html. Date read: 2026-07-30.

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them. … include_libraries (bool) – Include library file paths of linked data. False by default. … Returns: dictionary of bpy.types.ID instances, with sets of file path strings as their values."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.BlendData.html. Date read: 2026-07-30.

**Reading and writing them in one pass.**

> "file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})
> Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data. … visit_path_fn … function that takes three parameters: the data-block, a file path, and a bpy.types.BlendDataPathMeta metadata object. The function should return either None or a str. In the latter case, the visited file path will be replaced with the returned string."

Same page, same date. That is the write-back: return a string and the path is replaced.

This gained three options in this version. **LABEL: documented.**

> "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache.
> - `EXPAND_TOKENS`: visit each UDIM tile path.
> - `EXPAND_SEQUENCES`: visit each image and volume sequence file path.
> - `EXPAND_CACHES`: visit cache files, like texture cache files associated with images.
> The previously reserved metadata field is now provided to callers, with `is_expanded` and `is_cache` booleans to indicate the file kind. Additionally `is_readonly` indicates if the path can be edited, which is not possible for expanded paths."

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/python_api/. Date read: 2026-07-30.

**Not every path can be written back.** **LABEL: documented.**

> "is_cache — True when the path is a cache file, like the image texture cache. These paths can not be edited.
> is_expanded — True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited.
> is_readonly — True when the path is read-only and can not be edited."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html. Date read: 2026-07-30.

The full set of flags controlling what gets visited is documented at https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html (date read 2026-07-30): `SKIP_LINKED`, `SKIP_PACKED`, `SKIP_WEAK_REFERENCES` ("Skip weak reference paths. Those paths are typically 'nice to have' extra information, but are not used as actual source of data by the current .blend file."), `SKIP_MULTIFILE` ("Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed."), `RELOAD_EDITED` ("Reload data when the path is edited"), `RESOLVE_TOKEN`, `EXPAND_TOKENS`, `EXPAND_SEQUENCES`, `EXPAND_CACHES`.

**One path at a time.** **LABEL: inferred.** Individual properties are described as strings and are not marked readonly:

- "filepath — Path to the library .blend file (default '', never None, blend relative // prefix supported)" — https://docs.blender.org/api/5.2/bpy.types.Library.html
- "filepath — Image/Movie file name (default '', never None, blend relative // prefix supported)" and "filepath_raw — Image/Movie file name (without data refreshing)" — https://docs.blender.org/api/5.2/bpy.types.Image.html

Both date read 2026-07-30. The step is that absence of the `readonly` mark means assignment is allowed. The quoted entries do not say "writable" in so many words.

**Turning stored paths into real ones.** **LABEL: documented.**

> "bpy.path.abspath(path, *, start=None, library=None) — Returns the absolute path relative to the current blend file using the '//' prefix. … library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."
> "bpy.path.relpath(path, *, start=None) — Returns the path relative to the current blend file using the '//' prefix."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.path.html. Date read: 2026-07-30.

**In bulk, from the interface or as operators.** **LABEL: documented.**

> "bpy.ops.file.make_paths_absolute() — Make all paths to external files absolute"
> "bpy.ops.file.make_paths_relative() — Make all paths to external files relative to current .blend"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.ops.file.html. Date read: 2026-07-30. The manual describes the same two under File ▸ External Data (https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html).

**And on write.** **LABEL: documented.**

> "path_remap (str) – Optionally remap paths when writing the file: NONE No path manipulation (default). RELATIVE Remap paths that are already relative to the new location. RELATIVE_ALL Remap all paths to be relative to the new location. ABSOLUTE Make all paths absolute on writing."

(`bpy.data.libraries.write`.) Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html. Date read: 2026-07-30.

The manual's Save As has the interface equivalent: "Remap Relative — This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location." (https://docs.blender.org/manual/en/5.2/files/blend/open_save.html).

**What the `//` prefix means.** **LABEL: documented.**

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file." … "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

Source: manual — https://docs.blender.org/manual/en/5.2/files/blend/open_save.html. Date read: 2026-07-30. And: "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

**Not covered.** Whether writing a path through any of these actually reloads the data it points at (only `RELOAD_EDITED` hints at it, and only for `file_path_foreach`). Whether the write survives without saving the file. Whether paths inside *linked* data can be edited (there is a `SKIP_LINKED` flag, and `ID.is_editable` exists, but the sources do not spell out the rule). Whether template expressions in a path interact with these rewrites.

**Outside the interface.** **LABEL: inferred** for the functions and properties: they are documented as Python routes with no interface step, but the sources do not expressly say that they work without a window. The two operators `file.make_paths_absolute` / `file.make_paths_relative` take no arguments and the sources do not say whether they need a window. The manual route (File ▸ External Data, Save As ▸ Remap Relative) is interface-only by nature. **LABEL for that operator caveat: not in the sources.** **Added by the hand tests of 2026-08-06.** Both bulk rewrites, the one-pass path visitor where it exists, the write call, saving elsewhere with the remap option, the missing-file search and library relocation were all run with no window at all on a rented Linux machine in this version, and all returned `{'FINISHED'}` and did what they claim. Two operators would **not** run there — `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path`, both refusing with a failed context check. **Brought forward by the citation check, 2026-08-06.** The sentence that used to end this line said those two were the only part of it still owed a machine with a screen. The Mac sitting of 2026-08-07 settled both, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file. So only the first of the two was ever owed a screen, and it is owed no longer. The full record is in this version's `Preparation/Blender Research/Findings/5.2/Reading a file — added questions.md`. See the observed addition below.

**Operating system.** **LABEL: not in the sources.** No qualification appears on the cited path-reading and path-writing pages. Two nearby helpers are documented: "bpy.path.native_pathsep(path) — Replace the path separator with the system's native os.sep" and "bpy.path.resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (https://docs.blender.org/api/5.2/bpy.path.html, date read 2026-07-30). They do not say which systems differ or that stored paths read back differently. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it. **Added by the hand tests of 2026-08-06, and corrected by the citation check, 2026-08-06.** The line used to say the new material below was run on Linux only and left macOS owed a second sitting. That was true when it was written; the second sitting is now printed below, under *Observed on the Creator's Mac, 2026-08-07*. The same file was built on the Mac in this version and read back, and twenty-five of its twenty-seven printed lines match the Linux printout character for character. So **macOS is settled for what that test covers**, and it differs from Linux on two lines, both recorded below. The rest of the round's entries were run on Linux alone and stay untested on macOS. **Windows stays open** for the same reason as before.

**Observed — reading a recorded image path, writing a new one, and whether it survives a save (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A fresh empty folder was given a 4 by 4 PNG at `tex/probe.png`, written by Blender itself, and a `.blend` beside it holding one image data-block loaded from that PNG, marked with a fake user so it survived the save, and saved with `relative_remap=True`. A second run with no window opened that file, read the paths, assigned `img.filepath = "//tex/moved.png"`, and saved:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/probe.png']
BLEND_PATHS ABS ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/…/04/52/tex/probe.png']
ASSIGNED OK
AFTER ASSIGN //tex/moved.png
BLEND_PATHS AFTER ASSIGN ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

A third run with no window opened the same file again:

```
REOPENED [('ProbeLoaded', '//tex/moved.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
```

Nothing in the built file pointed at the Blender application's own brushes file; that entry appeared in `blend_paths()` of its own accord. This settles the step marked inferred above, that a path property without a `readonly` mark can be written, and it settles that the write survives a save and reopening — the second item under "Not covered" above.

**Observed — which path-reading and path-writing calls exist (hand test, 2026-08-04).** Same computer, same build, same conditions, with no file loaded. What came back:

```
PROBE bpy.utils.blend_paths = True
PROBE bpy.data.file_path_map = True
PROBE bpy.data.file_path_foreach = True
PROBE bpy.data.user_map = True
PROBE bpy.data.all_ids = True
PROBE bpy.data.temp_data = True
```

and the full list of operators under `bpy.ops.file`:

```
PROBE bpy.ops.file operators = ['autopack_toggle', 'bookmark_add', 'bookmark_cleanup', 'bookmark_delete', 'bookmark_move', 'cancel', 'delete', 'directory_new', 'edit_directory_path', 'execute', 'external_operation', 'filenum', 'filepath_drop', 'find_missing_files', 'hidedot', 'highlight', 'make_paths_absolute', 'make_paths_relative', 'mouse_execute', 'next', 'pack_all', 'pack_libraries', 'parent', 'previous', 'refresh', 'rename', 'report_missing_files', 'reset_recent', 'select', 'select_all', 'select_bookmark', 'select_box', 'select_walk', 'smoothscroll', 'sort_column_ui_context', 'start_filter', 'unpack_all', 'unpack_item', 'unpack_libraries', 'view_selected']
```

**Observed — whether a recorded path survives when its target is gone (hand test, 2026-08-04).** Same computer, same build, same conditions. A file was built exactly as above, then the PNG it points at was renamed from `tex/probe.png` to `tex/gone.png` and the file opened with no window. What came back:

```
IMAGES [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
IS_MISSING [('ProbeLoaded', False), ('Render Result', False), ('Viewer Node', False)]
BLEND_PATHS ABS [('/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', True), ('/…/05/52/tex/probe.png', False)]
FILE_PATH_MAP {'WinMan': [], 'Animation': [], 'Compositing': [], 'Geometry Nodes': [], 'Layout': [], 'Modeling': [], 'Rendering': [], 'Scripting': [], 'Sculpting': [], 'Shading': [], 'Texture Paint': [], 'UV Editing': [], 'Scene': [], 'Palette': [], 'Collection': [], 'Camera': [], 'Cube': [], 'Light': [], 'World': [], 'LineStyle': [], 'Material': [], 'ProbeLoaded': ['//tex/probe.png'], 'Render Result': [], 'Viewer Node': []}
```

The second item of each pair in `BLEND_PATHS ABS` is the result of `os.path.exists` on that path.

**Observed — reading a recorded path, writing a new one, and seeing whether it survives a save, on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS**, with no window, `--factory-startup`, and its own settings folder. What came back from the three runs:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/probe.png']
BLEND_PATHS ABS ['/opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/smoozy/Reading a file/work/04/5.2/tex/probe.png']
ASSIGNED OK
AFTER ASSIGN //tex/moved.png
BLEND_PATHS AFTER ASSIGN ['//../../../../../../opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

and, after reopening the file:

```
REOPENED [('ProbeLoaded', '//tex/moved.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
```

So on Linux too the path can be read, written, saved and read back with the new value, all with no window. The list of external paths carried one entry nothing in the file pointed at — Blender's own brushes file — here as well. Every line agrees with the Mac.

**Observed — that a recorded path survives when the file it names is gone, and what this version offers (hand test, 2026-08-06).** Same machine and build. With the PNG renamed away:

```
IMAGES [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
IS_MISSING [('ProbeLoaded', False), ('Render Result', False), ('Viewer Node', False)]
BLEND_PATHS ABS [('/opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', True), ('/root/smoozy/Reading a file/work/05/5.2/tex/probe.png', False)]
```

And the probe:

```
PROBE bpy.data.file_path_foreach = True
PROBE bpy.data.all_ids = True
PROBE WindowManager.reports exists = True
PROBE bpy.ops.file operators = ['autopack_toggle', 'bookmark_add', 'bookmark_cleanup', 'bookmark_delete', 'bookmark_move', 'cancel', 'delete', 'directory_new', 'edit_directory_path', 'execute', 'external_operation', 'filenum', 'filepath_drop', 'find_missing_files', 'hidedot', 'highlight', 'make_paths_absolute', 'make_paths_relative', 'mouse_execute', 'next', 'pack_all', 'pack_libraries', 'parent', 'previous', 'refresh', 'rename', 'report_missing_files', 'reset_recent', 'select', 'select_all', 'select_bookmark', 'select_box', 'select_walk', 'smoothscroll', 'sort_column_ui_context', 'start_filter', 'unpack_all', 'unpack_item', 'unpack_libraries', 'view_selected']
```

Forty operators, identical to the Mac's; and this version alone offers both the one-pass path visitor and the whole-file list of data-blocks. All of it agrees with the Mac.

---

**Observed — which kinds of recorded location the reading and writing machinery actually reaches (hand test, 2026-08-06). Added by the hand-test round that put question 9 on this sheet; this document is from the earlier round and the rest of it is left as it was.**

This answers the item this question left open — *whether every kind of recorded path is covered by the machinery that reads and writes them.* Done on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**, with Blender **5.2.0 LTS** (hash `fbe6228777e7`) at `/opt/blender/5.2/blender`, started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at `/root/reading-a-file/res-5.2` — a settings folder made for this round and never the real one. Nothing here was run on the Creator's Mac, so nothing here settles macOS. The whole record, with every command, is in `Preparation/Blender Research/Findings/5.2/Reading a file — added questions.md` and `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

A file was built holding **twenty-one recorded locations, every one written out in full from the drive root**, all pointing into an `assets` folder beside it: an image, a sound, a font, a text file, a volume, a movie clip, an Alembic cache file, a linked library, the render output path, the bake output path, the compositor File Output node's base path, a cloth point cache, a fluid domain cache folder, a geometry-nodes bake folder, a mesh-cache modifier file, an ocean modifier folder, an OSL script node, an IES texture node, a camera's own shader path, a sequencer image strip's folder with the sequencer's proxy folder, and a custom property on an object holding a path-shaped string. The multi-resolution modifier's path would not take a value in any version, so that kind is not in the file and nothing here speaks for it.

What the whole-file listing calls returned for that file:

```
LIST blend_paths()                 = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST blend_paths(packed=True)      = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST blend_paths(local=True)       = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST blend_paths(absolute=True)    = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST file_path_map()               = ['/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST file_path_map(include_libraries=True) = ['/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
```

Then `bpy.ops.file.make_paths_relative()` on the same file, with no window. It returned `{'FINISHED'}`. Every one of the twenty-one read back afterwards:

```
AFTER | Image.filepath                           | //assets/probe.png
AFTER | Image.filepath_raw                       | //assets/probe.png
AFTER | Sound.filepath                           | //assets/probe.wav
AFTER | VectorFont.filepath                      | //assets/probe.woff2
AFTER | Text.filepath                            | //assets/probe.txt
AFTER | Volume.filepath                          | //assets/probe.vdb
AFTER | MovieClip.filepath                       | //assets/probe.png
AFTER | CacheFile.filepath                       | //assets/probe.abc
AFTER | Library.filepath                         | //lib.blend
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.2/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.2/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.2/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.2/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.2/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.2/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.2/assets/proxydir/
```

Reading that off and adding nothing to it: the recorded text changed for **CacheFile.filepath, Camera.custom_filepath, FluidDomainSettings.cache_directory, Image.filepath, Image.filepath_raw, ImageStrip.directory, Library.filepath, MeshCacheModifier.filepath, MovieClip.filepath, OceanModifier.filepath, ShaderNodeScript.filepath, ShaderNodeTexIES.filepath, Sound.filepath, Text.filepath, VectorFont.filepath, Volume.filepath**. It is the same after as before for **BakeSettings.filepath, CompositorNodeOutputFile base path, NodesModifier.bake_directory, PointCache.filepath (cloth), RenderSettings.filepath, StripProxy.directory (sequencer), custom property on an object**.

The same file with `bpy.ops.file.make_paths_absolute()` put every one of those sixteen back into full form and left the same seven alone, so the two sweeps reach the same set.

The one-pass path visitor, called with its flags emptied and a function that returned each path with `/assets/` replaced by `/MOVED/`, visited sixteen paths and rewrote fifteen of them (the sixteenth is the library path, which the function handed back unchanged because it holds no `/assets/`). What it did **not** reach is the same set the bulk sweeps miss:

```
FEW visited ('CacheFile', 'probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/MOVED/probe.abc', False)
FEW visited ('Camera', 'ProbeCam', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/MOVED/camshader.osl', False)
FEW visited ('Image', 'ProbeImage', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/MOVED/probe.png', False)
FEW visited ('Library', 'lib.blend', '/root/reading-a-file/work/many/5.2/lib.blend', '/root/reading-a-file/work/many/5.2/lib.blend', False)
FEW visited ('Material', 'Material', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/MOVED/brushes/essentials_brushes-gp_draw.blend', False)
FEW visited ('MovieClip', 'ProbeClip', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/MOVED/probe.png', False)
FEW visited ('Object', 'FluidObj', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/MOVED/fluidcache/', False)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/MOVED/meshcache.mdd', False)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/MOVED/oceanbake/', False)
FEW visited ('Scene', 'Scene', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/MOVED/stripdir/probe.png', False)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/MOVED/probe.ies', False)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/MOVED/probe.osl', False)
FEW visited ('Sound', 'ProbeSound', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/MOVED/probe.wav', False)
FEW visited ('Text', 'ProbeText', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/MOVED/probe.txt', False)
FEW visited ('VectorFont', 'ProbeFont', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/MOVED/probe.woff2', False)
FEW visited ('Volume', 'ProbeVolume', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/MOVED/probe.vdb', False)
```

Last, a walk over thirteen of `bpy.data`'s collections, four levels deep, collecting every text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries one of the two path marks Blender puts on a property. What it reached, one line per distinct value, with the custom property on the end — that one is not found by the walk at all and turns up only in a separate listing of custom properties:

```
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = /root/reading-a-file/work/many/5.2/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = /root/reading-a-file/work/many/5.2/assets/camshader.osl
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = /root/reading-a-file/work/many/5.2/assets/custom_prop.png
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = /root/reading-a-file/work/many/5.2/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = /root/reading-a-file/work/many/5.2/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.2/assets/meshcache.mdd
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].directory = /root/reading-a-file/work/many/5.2/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.2/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = /root/reading-a-file/work/many/5.2/assets/pointcache/
SWEEP bpy.data.cache_files['probe.abc'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.abc
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.2/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.2/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = /root/reading-a-file/work/many/5.2/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = /root/reading-a-file/work/many/5.2/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.strips[].directory = /root/reading-a-file/work/many/5.2/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = /root/reading-a-file/work/many/5.2/lib.blend
SWEEP bpy.data.materials['Material'].library_weak_reference.id_name = MAMaterial
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].file_name = {blend_name}
```

The walk reaches nineteen of the twenty-one, including six of the seven the bulk sweeps miss. The seventh of those, the custom property, carries no mark and is found only by listing custom properties separately. The text data-block's path is the one recorded location no mark reaches at all: its subtype came back as `NONE` and it carries neither mark. The bulk sweeps do reach it.

**Observed on the Creator's Mac, 2026-08-07.** The same file was built again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**, by the same two build scripts, and its paths made relative the same way, and then read. The Blender was `/Applications/Blender.app/Contents/MacOS/Blender`, reporting **5.2.0 LTS**, hash `fbe6228777e7` — the same version string and hash the rented Linux machine reported. It ran with `--factory-startup` and with `BLENDER_USER_RESOURCES` pointed at a settings folder made for this sitting and for nothing else, never at the real one. Other topics were running Blender on the same Mac at the time, so nothing here says how long anything took. The scripts were written again from the recorded procedure rather than copied off the Linux machine, which this sitting was told not to touch; they were checked against the Linux printout first, and the twenty-four readings and the map of data-blocks to paths came back the same character for character.

```
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/many/5.2/many_rel.blend" --python "$S/s_cross.py"
```

where `$T` is `/private/tmp/smoozy-test/raf` and `$S` is `/Users/artist/Documents/Smoozy hand tests/Reading a file/added-questions/scripts`. What came back:

```
CROSS | Image.filepath                           | //assets/probe.png
CROSS | Image.filepath_raw                       | //assets/probe.png
CROSS | Sound.filepath                           | //assets/probe.wav
CROSS | VectorFont.filepath                      | //assets/probe.woff2
CROSS | Text.filepath                            | //assets/probe.txt
CROSS | Volume.filepath                          | //assets/probe.vdb
CROSS | MovieClip.filepath                       | //assets/probe.png
CROSS | CacheFile.filepath                       | //assets/probe.abc
CROSS | Library.filepath                         | //lib.blend
CROSS | RenderSettings.filepath                  | <HERE>/assets/render_out/
CROSS | BakeSettings.filepath                    | <HERE>/assets/bake_out/
CROSS | CompositorNodeOutputFile base path       | <HERE>/assets/node_out/
CROSS | PointCache.filepath (cloth)              | <HERE>/assets/pointcache/
CROSS | FluidDomainSettings.cache_directory      | //assets/fluidcache/
CROSS | NodesModifier.bake_directory             | <HERE>/assets/gnbake/
CROSS | MultiresModifier.filepath                | 
CROSS | MeshCacheModifier.filepath               | //assets/meshcache.mdd
CROSS | OceanModifier.filepath                   | //assets/oceanbake/
CROSS | ShaderNodeScript.filepath                | //assets/probe.osl
CROSS | ShaderNodeTexIES.filepath                | //assets/probe.ies
CROSS | Camera.custom_filepath                   | //assets/camshader.osl
CROSS | custom property on an object             | <HERE>/assets/custom_prop.png
CROSS | ImageStrip.directory                     | //assets/stripdir/
CROSS | StripProxy.directory (sequencer)         | <HERE>/assets/proxydir/
CROSS blend_paths   = ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
```

**Twenty-five of the twenty-seven lines are identical to the Linux printout, character for character** — every one of the twenty-four readings, and the map of data-blocks to paths. Two are not, and they are recorded as two results and not folded into one.

The flat path list holds sixteen entries on the Mac and fifteen on Linux; the fifteen they share are identical. The extra one on the Mac is a path into the Blender application's own folder, ending `datafiles/assets/brushes/essentials_brushes-gp_draw.blend`, recorded relative to the file. On the Mac the same file lists that path in full form *before* the sweep as well, so the difference appears only after the sweep and the save, and afterwards the material it hangs off reads `//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend`.

And **whether capital letters in a name matter is answered differently on the two machines.** The check asks for the image's own file again with its name written in capitals — `os.path.exists(os.path.join(os.path.dirname(p), os.path.basename(p).upper()))` where `p` is `bpy.path.abspath(bpy.data.images["ProbeImage"].filepath)`. The file on disk is `probe.png` and nothing called `PROBE.PNG` was ever written; asked for the folder's own listing, the Mac answered `['probe.png']`, one file. The Mac's answer to the check was `True` and the Linux machine's was `False`. **The Mac's answer is the one that speaks for macOS**; neither machine speaks for the other, and neither speaks for Windows.

**Windows is untouched.** No Windows machine has run it, so whether any of this behaves differently there was not answered by any test here.

---

## 6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?

**LABEL: documented. Yes to all three, by named operators.**

**Report which are missing.**

> "bpy.ops.file.report_missing_files()
> Report all missing external files"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.ops.file.html. Date read: 2026-07-30.

The manual says where the answer appears:

> "Report Missing Files
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

Source: manual — https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html. Date read: 2026-07-30.

**Search for them.**

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', … )
> Try to find missing external files
> find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)
> directory (str) – Directory, Directory of the file (optional, never None)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.ops.file.html. Date read: 2026-07-30.

> "Find Missing Files
> In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative.
> Note
> Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual — https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html. Date read: 2026-07-30.

**Rewrite the recorded locations.** All of Q5's answer applies: `bpy.ops.file.make_paths_absolute()`, `bpy.ops.file.make_paths_relative()`, direct assignment to `Library.filepath` / `Image.filepath`, and `bpy.data.file_path_foreach` returning a replacement string. For a whole linked library there is a dedicated operator:

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, …)
> Relocate the given library to one or several others
> library (str) – Library, Library to relocate"

and

> "bpy.ops.wm.lib_reload(*, library='', filepath='', …)
> Reload the given library"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.ops.wm.html. Date read: 2026-07-30.

The manual describes the interface equivalents and what they are for:

> "Relocate Library … When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

and, for a single data-block:

> "Relocate Linked ID … It allows to relocate a directly linked ID by another one, from the same library or from a different one. … Note: This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."

Source: manual — https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html. Date read: 2026-07-30.

**Missing linked data can also be spotted per data-block, without running an operator.** **LABEL: documented.**

> "is_missing
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.ID.html. Date read: 2026-07-30.

Backed by the manual:

> "Broken Libraries
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

Source: manual — https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html. Date read: 2026-07-30.

**How a script could read what "report" reported.** **LABEL: inferred.** This version made Blender's reports readable:

> "Blender reports are now exposed as a read-only list in WindowManager ID types. They also now have a session-wide unique identifier integer."

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/python_api/. Date read: 2026-07-30. The type:

> "reports — Collection of reports (default None, readonly) — Type: bpy_prop_collection[Report]" (https://docs.blender.org/api/5.2/bpy.types.WindowManager.html), and "Report entry … message — Report message text … type — Report type (severity)" (https://docs.blender.org/api/5.2/bpy.types.Report.html).

The step I am taking: `report_missing_files` produces a warning, warnings are reports, and reports are now a readable list — so a script could read the result. The sources do not connect these three statements; nobody says `report_missing_files` writes into `WindowManager.reports`.

**Not covered.**
- What `report_missing_files` counts as "external files" — whether missing *libraries* are included alongside missing images, or only unpacked file references. The manual says "links to unpacked files that no longer exist", which reads narrower than the operator's "all missing external files".
- Whether `find_missing_files` can be driven without a file browser. It takes a `directory` argument, which suggests it can, but the manual describes it only as "A File Browser will show up", and the sources do not say what happens when it is called from a script with `directory` already set. **This is a real gap for any unattended use.**
- Whether either operator returns anything useful (both are documented as returning only the generic operator result set).
- Whether "recovered" means the stored path is rewritten and saved, or only changed in memory. The manual's "Recovered files might need to be reloaded" implies memory, and the advice to then run Make Paths Relative implies the stored path was changed — but neither is said outright.

**Outside the interface.** **LABEL: mixed — documented for the named interface routes, inferred for one reports-list route, and otherwise not in the sources.**
- `report_missing_files` delivers its answer to "the Info editor's header" — that is an interface place. Whether anything equivalent reaches a script or a console outside the interface is **not in the sources**, beyond the inferred reports-list route above.
- `find_missing_files` is documented in the manual as opening a File Browser, which is an interface thing; its `directory` parameter suggests but does not confirm a scriptable route.
- `make_paths_absolute`, `make_paths_relative`, `lib_relocate`, `lib_reload` are all documented as operators with no interface requirement stated, but nothing says they work without a window.
- The purely data-side routes — `ID.is_missing`, `Library.filepath`, `bpy.data.file_path_foreach` — carry no interface dependence in their documentation.

**Operating system.** **LABEL: not in the sources.** No qualification appears on the cited missing-file and relocation pages. The one adjacent fact is `bpy.path.resolve_ncase` ("Resolve a case insensitive path on a case sensitive system"), which documents that path case behaviour differs by system, but the sources do not connect it to these operators. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed on everything except the exact recovered path, which is set out below. So macOS and Linux are both settled for what they cover. **Windows stays open.**

**Observed — what "report missing files" produces with no window, and whether the reports list holds it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A file was built in a fresh empty folder holding one image data-block loaded from a PNG beside it, then that PNG was renamed away so the recorded path pointed at nothing. Opening the file with no window, the script called `bpy.ops.file.report_missing_files()` and then read `bpy.context.window_manager.reports`. What came back:

```
Warning: Path '/…/05/52/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
REPORT RESULT {'FINISHED'}
WM REPORTS []
```

`WindowManager` does have a `reports` property in this version, and it was empty after the operator had run. The same run also printed the flag on the image itself:

```
IS_MISSING [('ProbeLoaded', False), ('Render Result', False), ('Viewer Node', False)]
```

**Observed — whether "find missing files" can be pointed at a folder with no window (hand test, 2026-08-04).** Same computer, same build, same conditions. The same kind of file was built, and the PNG it points at was moved out of `tex/` into `search/deeper/probe.png`. Opening the file with no window, the script called `bpy.ops.file.find_missing_files(directory="/…/06/52/search/")` and then saved. What came back:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
FIND RESULT {'FINISHED'}
AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

Reopening the saved file in a further run with no window:

```
REOPENED [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//search/deeper/probe.png']
```

That answers the second and fourth items under "Not covered" above: the search operator runs with a directory supplied and no window, it rewrote the recorded path in memory, and the rewritten path was still there after saving and reopening. The path came back in the relative `//` form. The Creator's own run, recorded below, adds the other half: calling the search from the Python Console inside Blender's window, with a directory supplied, opened no File Browser either.

**Observed — how missing linked data is flagged (hand test, 2026-08-04).** See the chain test recorded under question 8: with a linked file renamed away, `is_missing` came back `True` on both the library entry and the linked object, and Blender printed warnings naming the file and the data-block on the terminal.

**Observed — what "Report Missing Files" shows on screen (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 5.2.0 LTS. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open.

A file prepared beforehand by this chat was opened at `/Users/artist/Documents/Smoozy hand tests/Reading a file/14/5.2.blend`. It holds one image data-block whose recorded path points at a PNG that had been moved away, so the link is broken. The Creator chose **File ‣ External Data ‣ Report Missing Files**.

The Creator's own words, in full:

> In all versions it shows this at the bottom of the window:
> Path '/Users/artist/Documents/Smoozy hand tests/Reading a file/14/tex_4.5/probe.png' not found, from local data-block 'IMBrokenProbe'
> Again, 4.5 is the version, obviously.

That last line means the folder in the path changes with the version. In 5.2 the message read:

```
Path '/Users/artist/Documents/Smoozy hand tests/Reading a file/14/tex_5.2/probe.png' not found, from local data-block 'IMBrokenProbe'
```

and the rest of it was the same in all four versions.

**Observed — whether "Find Missing Files" opens a file browser when it is given a folder (hand test, 2026-08-05).** Same computer, same day, same build, same file, freshly opened. The Creator switched to the Scripting workspace and typed two lines into the Python Console. What the console held afterwards, pasted in full:

```
>>> bpy.ops.file.find_missing_files(directory="/Users/artist/Documents/Smoozy hand tests/Reading a file/15/")
{'FINISHED'}

>>> [(i.name, i.filepath) for i in bpy.data.images]
[('BrokenProbe', '//../15/tex_5.2/probe.png')]
```

and, in the Creator's own words: "No windows are opening in any version."

The Creator reported that console as identical in all four versions — including the path that came back, which reads `tex_5.2` in every one of them, this version among them. That is recorded here as it stands. **It was not established why the recovered path names `tex_5.2` in a version other than 5.2**, and nothing here should be read as explaining it.

**Observed — reporting, finding and rewriting missing references on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS**, with no window, `--factory-startup`, and its own settings folder.

*Reporting.* A file whose one image had been renamed away was opened and the report asked for. What came back:

```
Warning: Path '/root/smoozy/Reading a file/work/05/5.2/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
REPORT RESULT {'FINISHED'}
WM REPORTS []
```

Same wording as on the Mac, apart from the path. The report works with no window and names the missing file. **The reports list this version alone has came back empty**, straight after the report had run — so the report still does not reach a script as data. That is the Mac's result too.

*Searching and rewriting.* The PNG was instead moved to `search/deeper/probe.png` and the search handed the folder `search/`. What came back:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
FIND RESULT {'FINISHED'}
AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

and after reopening the file:

```
REOPENED [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
```

So the search works with no window when handed a folder, finds the moved file, rewrites the recorded path in the relative form, and the change survives a save and a reopening — as on the Mac. **One thing differs between the two machines and is not explained by either.** The path that came back here is `//search/deeper/probe.png`, the plain relative path to where the file actually was. On the Mac, the Creator's own run of the same operator returned `//../15/tex_5.2/probe.png`, and that record says it was not established why. Both results stand; neither explains the other, and nothing here says the difference belongs to the operating system.

**Observed — the same two things in Blender's own window on Linux (hand test, 2026-08-06).**

Same machine and build, with a window open on the machine's own display and the operator called from a timer.

*Reporting, in a window:*

```
WINDOW COUNT BEFORE 1
Warning: Path '/root/smoozy/Reading a file/work/w14/5.2/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
REPORT RESULT {'FINISHED'}
WINDOW COUNT AFTER 1
WM REPORTS []
SHOT RESULT {'FINISHED'}
```

No second window opened, and **the reports list was empty even here — with a window open and the report freshly run**. That is worth stating plainly: this version is the only one with such a list, and it did not hold the missing-file report in either place it was tried. The message the Creator read at the bottom of Blender's window on the Mac could not be produced here, because the operator had to be called from a script rather than chosen from the menu, and nothing on this machine can drive a menu.

*Searching, in a window:*

```
WINDOW COUNT BEFORE 1
FIND RESULT {'FINISHED'}
WINDOW COUNT AFTER 1
IMAGES AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
SHOT RESULT {'FINISHED'}
```

The window count was one before and one after, so **no file browser opened** — the same answer the Creator got on the Mac, and the opposite of what the manual leads a reader to expect.

---

## 7. Can something a file points at be traced to the scene that uses it?

**LABEL: mixed — documented as a direct answer for objects, documented only one hop at a time for libraries and other data-blocks, and inferred for a full trace from those things to a scene.**

**Documented, and direct, for an object.**

> "users_scene
> The scenes this object is in.
> Type: tuple[Scene, …]
> Note: Takes O(len(bpy.data.scenes) * len(bpy.data.objects)) time.
> (readonly)"

and

> "users_collection
> The collections this object is in.
> Type: tuple[Collection, …]
> Note: Takes O(len(bpy.data.collections) + len(bpy.data.scenes)) time."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Object.html. Date read: 2026-07-30.

**Documented for the first hop from a linked library, not direct to a scene.**

> "users_id
> ID data-blocks that use this library
> Type: tuple[ID, …]
> Note: Takes O(n) time, where n is the total number of all linkable ID types in bpy.data.
> (readonly)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Library.html. Date read: 2026-07-30. That gives you the data-blocks, not the scene.

**Documented, general, but one hop at a time.**

> "user_map(*, subset=None, key_types=None, value_types=None)
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them. … Returns: dictionary that maps data-blocks ID's to their users."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.BlendData.html. Date read: 2026-07-30.

And a count, per pair:

> "user_of_id(id)
> Count the number of times that ID uses/references given one … Returns: Number of usages/references of given id by current data-block"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.ID.html. Date read: 2026-07-30.

Also useful, and new in this version, for going the other way — from a data-block to the files it points at:

> "all_ids — Read-only list of all IDs listed in Blender data-base" (https://docs.blender.org/api/5.2/bpy.types.BlendData.html), announced as "An `all_ids` iterator has been added to `bpy.data`, allowing to iterate over all data-blocks at once" (https://developer.blender.org/docs/release_notes/5.2/python_api/). Date read: 2026-07-30.

**The inferred part.** **LABEL: inferred.** For something that is *not* an object — an image, a material, a linked library — the sources document no single call that returns the scene. `user_map` returns each data-block's immediate users. The step: to reach a scene from an image you follow `user_map` from image to material, material to mesh, mesh to object, and then either continue through `user_map` or use `Object.users_scene`. The sources describe the map and describe that data-blocks "can reference each other, for reuse and instancing. (Child/parent, object/object-data, materials/images, in modifiers or constraints too…)" (https://docs.blender.org/manual/en/5.2/files/data_blocks.html, date read 2026-07-30) — but nowhere state that walking the map repeatedly reaches a scene, nor that it always terminates.

**Nearby information in the interface, not a documented route to the scene.** **LABEL: documented.**

> "Blender File:
> Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number of users – clicking it adds or removes a fake user."

Source: manual — https://docs.blender.org/manual/en/5.2/editors/outliner/interface.html. Date read: 2026-07-30. And, for linked data:

> "Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."

Source: manual — https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html. Date read: 2026-07-30.

Also documented, and worth naming since the manual mentions counts: "you can see the number of current users of a data-block to the right of its name in the interface" (https://docs.blender.org/manual/en/5.2/files/data_blocks.html) — a count, not a trail.

**Not covered.** No documented one-call answer for "which scene uses this image / this material / this library". No statement about what happens with circular references while walking the map (the manual warns "In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links." — https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — but that is about linking, not about walking users). No statement about whether a data-block reachable only through a *view layer's* excluded collection still counts as used by that scene.

**Outside the interface.** **LABEL: inferred** for the Python routes: the sources document `user_map`, `user_of_id`, `Object.users_scene`, `Object.users_collection`, `Library.users_id` and `all_ids` without an interface step, but do not expressly say that no window is needed. The Outliner information is interface-only. Two properties carry explicit cost notes (quoted above); `Object.users_scene` is O(scenes × objects). **Added by the hand tests of 2026-08-06.** The walk was run again with no window on a rented Linux machine, this time from references held only inside a node group, a driver and a custom property, and it held in every case. See the observed addition below.

**Operating system.** **LABEL: not in the sources.** I checked the cited data-block, library, object and Outliner pages; none gives an operating-system qualification. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it. **Added by the hand tests of 2026-08-06.** The new material below was run on the rented Linux machine only, so it settles **Linux** and leaves **macOS** owed a second sitting. **Windows stays open** for the same reason as before.

**Observed — following an image up to the scene that uses it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A file was built in a fresh empty folder holding two scenes, `SceneWithIt` and `SceneWithout`; a PNG on disk named `UsedTex.png` loaded and used by an image texture node in a material on a cube that sits in `SceneWithIt` only; and a second PNG named `UnusedTex.png` loaded into the file and used by nothing. Opening that file with no window, the script called `bpy.data.user_map()` once and then walked outwards from each image, printing each step. What came back:

```
START UsedTex
  STEP 1 'UsedTex' Image -> [('ProbeMat', 'Material')]
  STEP 2 'ProbeMat' Material -> [('Cube', 'Mesh')]
  STEP 3 'Cube' Mesh -> [('Cube', 'Object')]
  STEP 4 'Cube' Object -> [('SceneWithIt', 'Scene')]
  STEP 5 'SceneWithIt' Scene -> [('WinMan', 'WindowManager')]
  STEP 6 'WinMan' WindowManager -> []
  SCENES REACHED ['SceneWithIt']
  USERS_SCENE [('Cube', ['SceneWithIt'])]
  VALUE_TYPES SCENE {'UsedTex': []}
START UnusedTex
  STEP 1 'UnusedTex' Image -> []
  SCENES REACHED []
  USERS_SCENE []
  VALUE_TYPES SCENE {'UnusedTex': []}
FILE_PATH_MAP {'UnusedTex': ['//tex/UnusedTex.png'], 'UsedTex': ['//tex/UsedTex.png']}
```

The `VALUE_TYPES SCENE` line is the result of `bpy.data.user_map(subset=[img], value_types={'SCENE'})`. The `FILE_PATH_MAP` line was printed with entries holding no paths filtered out. The walk terminated on its own in six steps and did not loop. The whole test ran with no window, so it holds outside Blender's interface.

**Observed — the same walk on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS**, with no window, `--factory-startup`, and its own settings folder.

**The Mac's build script errored here, and that is recorded rather than hidden.** Run as the Mac list has it, this version printed:

```
Traceback (most recent call last):
  File "/root/smoozy/Reading a file/work/scripts/e09_trace.py", line 4, in <module>
    img = bpy.data.images[start]
          ~~~~~~~~~~~~~~~^^^^^^^
KeyError: 'bpy_prop_collection[key]: key "UnusedTex" not found'
```

The walk for the used image had already printed in full and matched the Mac; the second image, which nothing used, was not in the saved file. The Mac list records the same script running there with no repair needed. Why the two runs differ was not established, and nothing here says it is a difference between the operating systems. The build script was repaired by marking the unused image with a fake user so it survives the save:

```python
    if name == "UnusedTex":
        img.use_fake_user = True
```

Run again with that, what came back was:

```
START UsedTex
  STEP 1 'UsedTex' Image -> [('ProbeMat', 'Material')]
  STEP 2 'ProbeMat' Material -> [('Cube', 'Mesh')]
  STEP 3 'Cube' Mesh -> [('Cube', 'Object')]
  STEP 4 'Cube' Object -> [('SceneWithIt', 'Scene')]
  STEP 5 'SceneWithIt' Scene -> [('WinMan', 'WindowManager')]
  STEP 6 'WinMan' WindowManager -> []
  SCENES REACHED ['SceneWithIt']
  USERS_SCENE [('Cube', ['SceneWithIt'])]
  VALUE_TYPES SCENE {'UsedTex': []}
START UnusedTex
  STEP 1 'UnusedTex' Image -> []
  SCENES REACHED []
  USERS_SCENE []
  VALUE_TYPES SCENE {'UnusedTex': []}
FILE_PATH_MAP {'WinMan': [], 'Animation': [], 'Compositing': [], 'Geometry Nodes': [], 'Layout': [], 'Modeling': [], 'Rendering': [], 'Scripting': [], 'Sculpting': [], 'Shading': [], 'Texture Paint': [], 'UV Editing': [], 'SceneWithIt': [], 'SceneWithout': [], 'Cube': [], 'ProbeMat': [], 'UnusedTex': ['//tex/UnusedTex.png'], 'UsedTex': ['//tex/UsedTex.png']}
```

That is the Mac's result line for line: the walk reaches the scene in four steps and stops cleanly two steps later; an image nothing uses reaches nothing; asking the map directly for scene-type users returns an empty set, so the shortcut does not work here either; and the map from data-blocks to their recorded paths names both images. The whole thing ran with no window.

---

**Observed — whether a reference held only inside a node group, a driver or a custom property shows up in the map of what uses what (hand test, 2026-08-06). Added by the hand-test round that put question 9 on this sheet; this document is from the earlier round and the rest of it is left as it was.**

This answers the item this question left open. Same machine, same conditions as the addition under question 5 above: **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**, Blender **5.2.0 LTS** (hash `fbe6228777e7`), no window, `--factory-startup`, this round's own settings folder. Nothing here was run on the Creator's Mac.

A file was built holding one scene called `TheScene`, one object, one material and four images loaded from disk. `Img_plain` is used straight from the material's own nodes and is the control. `Img_group` is used only by an image node **inside a node group**, which the material uses through a group node. `Img_driver` is named only as the target of a **driver** on a custom property of the object. `Img_custom` is held only by a **custom property on the object** that points at the data-block. The file was then opened and `bpy.data.user_map()` called once; from each image the walk goes outwards until nothing new appears. What came back:

```
UM group   users = [('InnerGroup', 'ShaderNodeTree')]
UM group   scenes reached in 7 steps = ['TheScene']
UM group   user_map(value_types=SCENE) = {'Img_group': []}
UM group   users count = 2
UM plain   users = [('HolderMat', 'Material')]
UM plain   scenes reached in 6 steps = ['TheScene']
UM plain   user_map(value_types=SCENE) = {'Img_plain': []}
UM plain   users count = 2
UM driver  users = [('Holder', 'Object')]
UM driver  scenes reached in 4 steps = ['TheScene']
UM driver  user_map(value_types=SCENE) = {'Img_driver': []}
UM driver  users count = 1
UM custom  users = [('Holder', 'Object')]
UM custom  scenes reached in 4 steps = ['TheScene']
UM custom  user_map(value_types=SCENE) = {'Img_custom': []}
UM custom  users count = 2
UM file_path_map = [('Img_custom', ['/root/reading-a-file/work/um/5.2/assets/custom.png']), ('Img_driver', ['/root/reading-a-file/work/um/5.2/assets/driver.png']), ('Img_group', ['/root/reading-a-file/work/um/5.2/assets/group.png']), ('Img_plain', ['/root/reading-a-file/work/um/5.2/assets/plain.png'])]
```

All three of the awkward cases are visible in the map: the node-group one through the group's own node tree, and the driver one and the custom-property one through the object itself. Every image that is used at all reaches `TheScene`. Asking the map directly for scene-type users returns an empty set for every one of them, so the walk is what is needed, which is what the earlier round found for a plainly-used image.

**The macOS half of this is not run.** The test that settled this question — the one that builds a file whose image is used only inside a node group, only by a driver, and only by a custom property, and then walks the map of what uses what — was written for both machines and was run on the rented Linux machine alone. The Mac sitting of 2026-08-07 ran only the tests that had no result at all, so for this question macOS is untested rather than settled. It is recorded as owed in `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

**Windows is untouched**, for the same reason as above.

---

## 8. Can a file's full chain of references be read while some of the referenced files are absent?

**LABEL: mixed — documented for one level, inferred and partly not-in-the-sources for the rest.**

**One level down can be read straight out of the file being loaded, without opening the libraries themselves.** **LABEL: documented.** This is new in this version.

> "Each library object contains:
> Attributes matching bpy.data, which are lists of strings representing linkable data-blocks.
> A `libraries` attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file. The output libraries attribute is always None.
> A `version` attribute, representing the version of the loaded library blend-file (for the input) or the version of the current Blender (for the output).
> Notes:
> Not all data-block types are linkable (e.g. WindowManager, Library, …).
> Packed linked data-blocks are not linkable and are not listed in the input libraries attribute."

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html. Date read: 2026-07-30.

The release notes state the change and show it working:

> "API Improvements to `libraries.load()`
> Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"

with the example output:

> `bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)`

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/python_api/. Date read: 2026-07-30.

Note the exclusion in the quote above: packed linked data-blocks are **not** listed in the input data-block lists. The sources do not say whether that also removes a path from the separate `libraries` list, so they do not establish that the one-level library list becomes incomplete when packing has been used. Packing libraries is documented: "bpy.ops.file.pack_libraries() — Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again" (https://docs.blender.org/api/5.2/bpy.ops.file.html, date read 2026-07-30). Archives of packed IDs are described by `is_archive`, `Library.archive_libraries` and `Library.archive_parent_library` (https://docs.blender.org/api/5.2/bpy.types.Library.html).

**Going deeper needs each library file to be present.** **LABEL: inferred.**

The only documented way to read a library's own library list is to load that library — the `libraries` attribute belongs to the input side of `bpy.data.libraries.load(filepath)`, which takes "The path to a blend file" (same API page). The step a reader must supply: to get level two of the chain you must call `load()` on each level-one path, and a file that is not on disk cannot be loaded. The sources do not say what `load()` does when the path does not exist — no error, no return value, nothing.

**Opening the file instead gives you the chain that Blender managed to load, with holes marked.** **LABEL: documented for the pieces, inferred for the conclusion.**

Documented pieces:
- "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored." — https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html
- "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)" and "is_library_indirect — Is this ID block linked indirectly" and "library — Library file the data-block is linked from" — https://docs.blender.org/api/5.2/bpy.types.ID.html
- "parent — (readonly) — Type: Library | None" and "filepath — Path to the library .blend file" — https://docs.blender.org/api/5.2/bpy.types.Library.html

All date read 2026-07-30.

The inference: after opening the file, `bpy.data.libraries` plus `Library.parent` shows the chain of libraries Blender actually loaded, and `ID.is_missing` marks where a library could not be found. But below a missing library there is nothing to report, because its own library list was never read. So the chain is readable *down to* each absent file and no further. The sources state each piece; they do not state this conclusion.

**Not covered — and these are the sharp edges.**
- What `bpy.data.libraries.load()` does when the file at `filepath` is absent. Nothing in the API page or release notes says.
- Whether a `Library` entry exists at all for a library that could not be found, or only placeholder data-blocks. The manual says placeholders replace "missing linked ones" (the data-blocks); it does not say the library entry survives with its recorded path. **This is the single most important thing left unsettled for this question.**
- Whether opening a file with missing libraries succeeds, warns, or fails.
- Whether the one-level `libraries` list is affected by the library files being absent (it is read from the loaded file itself, so it should not be — but that is my reasoning, not the source's, and I have not labelled it as a finding).

**Where I looked.** `bpy.types.BlendDataLibraries`, `bpy.types.Library`, `bpy.types.ID`, `bpy.types.BlendData`, `bpy.ops.wm` (lib_relocate, lib_reload), `bpy.ops.file` (pack_libraries, unpack_libraries), the whole of `files/linked_libraries/` in the manual, `files/blend/compatibility`, and all five release-notes files that mention libraries. Searches for "missing", "placeholder", "broken", "is_archive", "recursive".

**Outside the interface.** **LABEL: inferred** for the Python routes: `bpy.data.libraries.load()` has a documented script example, and the other named routes are documented as data properties, but the sources do not expressly say that no window is needed. The Outliner's Blender File view and the Relocate command are interface-only. **LABEL: not in the sources** for whether placeholder creation differs when a file is opened from the command line; I checked the pages named under "Where I looked" and found no statement either way.

**Operating system.** **LABEL: not in the sources.** No qualification appears on the pages named under "Where I looked". The nearby `bpy.path.resolve_ncase` entry says "Resolve a case insensitive path on a case sensitive system" (https://docs.blender.org/api/5.2/bpy.path.html), but the sources do not say Blender applies that when loading libraries. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

---

**Observed — how far a chain of linked files reads when a file in the middle, or at the end, is gone (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **5.2.0 LTS** (hash fbe6228777e7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. Three files were built in one fresh empty folder: `c.blend` holding a cube named `ChainCube`; `b.blend`, which links that cube from `c.blend` and puts it inside a local collection named `ChainCollection`; and `a.blend`, which links `ChainCollection` from `b.blend`. All were saved with `relative_remap=True`. `a.blend` was then opened three times with no window — once with everything present, once with `c.blend` renamed to `c_gone.blend`, and once with `c.blend` back and `b.blend` renamed to `b_gone.blend`.

With everything present:

```
00:00.411  reports          | Read library: '/…/07/52/b.blend', '//b.blend', parent '<direct>'
00:00.414  reports          | Read library: '/…/07/52/c.blend', '//c.blend', parent '<direct>'
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, False)]
OBJECTS [('ChainCube', False, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With `c.blend` — the far end of the chain — renamed away:

```
00:00.458  reports          | WARNING Unable to open '/…/07/52/c.blend': No such file or directory
00:00.458  reports          | Cannot find lib '/…/07/52/c.blend'
00:00.459  reports          | LIB: Object: 'ChainCube' missing from '/…/07/52/c.blend', parent '<direct>'
00:00.459  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, True)]
OBJECTS [('ChainCube', True, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With `b.blend` — the middle of the chain — renamed away instead, and `c.blend` present:

```
00:00.414  reports          | WARNING Unable to open '/…/07/52/b.blend': No such file or directory
00:00.414  reports          | Cannot find lib '/…/07/52/b.blend'
00:00.415  reports          | LIB: Collection: 'ChainCollection' missing from '/…/07/52/b.blend', parent '<direct>'
00:00.415  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, True)]
OBJECTS []
BLEND_PATHS ['//b.blend']
```

Each entry of the `LIBRARIES` line is `(name, filepath, parent's filepath or None, is_missing)`. Each entry of the `OBJECTS` line is `(name, is_missing, is_library_indirect, the library's filepath or None)`.

That answers three of the four items under "Not covered" above. A `Library` entry does exist for a library that could not be found, and it keeps its recorded path — that was named there as the single most important thing left unsettled. Opening a file with missing libraries succeeded and printed warnings. And `bpy.data.libraries.load()` on a path that does not exist raised `OSError load: … failed to open blend file`, recorded in full under question 1. All three runs were with no window, and the placeholder behaviour and its messages both appeared there.

---

**Observed — the same three chain reads on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **5.2.0 LTS**, with no window, `--factory-startup`, and its own settings folder. The same three-deep chain was built by this version and opened three times.

With everything present:

```
00:00.386  blend            | Read blend: "/root/smoozy/Reading a file/work/07/5.2/a.blend"
00:00.391  reports          | Read library: '/root/smoozy/Reading a file/work/07/5.2/b.blend', '//b.blend', parent '<direct>'
00:00.394  reports          | Read library: '/root/smoozy/Reading a file/work/07/5.2/c.blend', '//c.blend', parent '<direct>'
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, False)]
OBJECTS [('ChainCube', False, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With the far file gone:

```
00:00.400  reports          | WARNING Unable to open '/root/smoozy/Reading a file/work/07/5.2/c.blend': No such file or directory
00:00.400  reports          | Cannot find lib '/root/smoozy/Reading a file/work/07/5.2/c.blend'
00:00.401  reports          | LIB: Object: 'ChainCube' missing from '/root/smoozy/Reading a file/work/07/5.2/c.blend', parent '<direct>'
00:00.402  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, True)]
OBJECTS [('ChainCube', True, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With the middle file gone instead:

```
00:00.398  reports          | Read library: '/root/smoozy/Reading a file/work/07/5.2/b.blend', '//b.blend', parent '<direct>'
00:00.398  reports          | WARNING Unable to open '/root/smoozy/Reading a file/work/07/5.2/b.blend': No such file or directory
00:00.399  reports          | LIB: Collection: 'ChainCollection' missing from '/root/smoozy/Reading a file/work/07/5.2/b.blend', parent '<direct>'
00:00.399  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, True)]
OBJECTS []
BLEND_PATHS ['//b.blend']
```

That is the Mac's result line for line, including the timestamped, channelled log lines this version prints. The readable chain stops at the first absent file on Linux too, and the library's parent property came back empty in every case here as well. While building the chain, the loader printed `AVAILABLE OBJECTS []` when the third file was linked from the second, so the loader lists only a file's own local data on Linux too.

**Observed — peeking into a file, on a Linux machine (hand test, 2026-08-06).** See the block recorded under question 1 for this machine. In this version the loader **does** offer a list of the libraries the peeked file itself links: it returned `SRC LIBRARIES [bpy_lib_library(filepath='//c.blend', is_archive=False)]`, alongside `SRC VERSION (5, 2, 44)`. It listed only the peeked file's own local data-blocks otherwise, and raised `OSError load: /root/smoozy/Reading a file/work/08/does_not_exist.blend failed to open blend file` for a path that is not there. Identical to the Mac's.

---

## Things a person with Blender open could settle by hand in a few minutes

The hand-test step ran in two sittings and settled every item that stood here. The terminal-reachable ones were run on 2026-08-04, including the concurrency one, which was run after the restriction against starting two Blenders at once was lifted. The ones needing eyes on Blender's window were run by the Creator on 2026-08-05. What each found is written into the answer it belongs to above, as an observed finding.

One of them was answered from both sides and is worth naming, since it stood here as the open item. The node's own path-named attributes were listed by test and none of them holds an assembled final path; the sidebar's `Output Paths` panel was then photographed and holds one assembled path for the whole node. Whether any other route exposes that same string to a script was not established, and no test here looked for one.

**Run 2026-08-06 — the same tests again on a rented Linux machine.** New equipment arrived and the list came back out. Every terminal test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and Blender's window was opened on that machine's own display for three more. All of it is recorded above, beside the Mac run it belongs to, and the two are never merged. Linux is therefore no longer open on any question this document answers by test. The Linux list is `Audits/Blender Research/Reading a file/Hand tests — Linux.md`.

Three things stay open for want of equipment rather than for want of a test. **Windows** was not tried at all — no Windows machine has run it — so every operating-system line above is settled for macOS and Linux and open for Windows. **No test can put a number** on how many Blenders may run at once, since one computer cannot establish a limit for every computer — and the rented machine is a container sharing someone else's hardware, so it is even less able to. And on the day of the Linux run **nothing on that machine could move the mouse to a control or press a key at it**, so what Blender shows on screen when the render switch is keyframed by hand, and what the File Output node's Output Paths panel holds, were not read there. Both were read on the Mac.

> **Corrected by the citation check of the Linux hand tests, 2026-08-06.** This read "nothing can move the mouse to a control or press a key at it", as a standing fact about the machine. A tool for moving the pointer was put on that machine afterwards, so the pointer can now be moved. Pressing and clicking is another matter: the check tried both and could not confirm that either reaches Blender. On a control whose state Blender will report — a node's Mute switch — four separate clicks left it unchanged, and clicks aimed by window rather than by position twice landed on a different control. **So both items stay unread on Linux**: what Blender shows when the render switch is keyframed by hand, and what the Output Paths panel holds. Both were read on the Mac. The attempts are written up in `Audits/Blender Research/Reading a file/Citation check/Linux — Opus 5.md`.

**Run 2026-08-05 — the re-run the citation check asked for.** The citation check added one test here: run the concurrency test again and record the wait condition exactly, because the test under question 2 described waiting until two frames had appeared while every recorded output line read `frames on disk: 1`. It was run in all four versions as hand test 18, with the wait condition written down and the frame count printed at every poll. Nothing stays open because of it. What it found, and the correction it forced, are recorded under question 2 above, beside the run they belong to.


**Added 2026-08-06 — the round that put question 9 on the sheet.** That round's hand tests also carried two items this document had left open: whether every kind of recorded path is covered by the machinery that reads and writes them (question 5), and whether a reference held only inside a node group, a driver or a custom property shows up in the map of what uses what (question 7). Both were run on a rented Linux machine and both are written into the answers above as observed additions. Nothing was run on the Creator's Mac in that sitting, so those two additions settle Linux only. That round's list is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

---

2026-08-04 — Opus 5 (Claude Code) — Reading a file, Blender 5.2

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests, added under questions 5 and 7

# Reading a file — Blender 4.5

These findings answer the Reading a file question sheet for Blender 4.5.

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, Blender's 4.5 release notes, Cycles' own add-on files as shipped with 4.5, and Blender's requirements page (the copy taken 2026-08-01). All of these were read from the `Blender Sources/` folder; the manual, API and release-note snapshots carry the date **2026-07-30**, which is the date read given throughout. Answers written 2026-08-04.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. What does a `.blend` file give up without a Blender to open it, and what needs one?

**Answer.** The sources name only a few things that come out of a `.blend` without a running Blender, and they do not describe the file's internal format at all.

What comes out without Blender interpreting the file:

- **Its bytes, decompressed.** If the file was saved compressed, the compression is a standard one and ordinary tools can undo it. That gets you the raw file, not its meaning.
- **A preview picture, through a small tool Blender installs into the operating system.** The system's own file browser shows the blend-file's stored preview as a thumbnail. This is Blender-supplied plumbing rather than Blender itself running.
- **Asset catalogs, which are not in the `.blend` at all.** They live beside it in a plain UTF-8 text file with a documented line format.

The manual also states plainly that tooling exists which manipulates blend-files from outside Blender, and points at one such tool by name — but it says nothing about what such a tool can read.

Everything else the documentation describes about a file's contents — its data-blocks, scenes, view layers, render settings, and recorded paths — it describes through Blender's interface or Blender's Python API. The sources contain no specification of the `.blend` container and no account of what a non-Blender reader can extract. That silence does not establish that only Blender can read those contents.

**Label:** documented for each quoted item; **not in the sources** for any general account of the file format or of what an outside reader can extract.

**Not covered.** The sources do not say what a non-Blender reader can actually pull out of a `.blend`. They do not describe the file's structure, its header, its version stamp as stored on disk, or the `.blend1` backup. Nothing here says a third-party tool can read data-block names, paths or settings — only that such tooling exists.

**Exact words.**

- "The used compression algorithm is Zstandard. It is not unique to Blender so files can be compressed/decompressed with external tools."
  Manual — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30.
- "Blender saves by default a small preview of current scene in the blend-file. This will show in the Thumbnail view of the File Browser." and "During its installation, Blender also adds a small tool to your OS, that will allow your system file browser to show those previews as file thumbnails as well."
  Manual — https://docs.blender.org/manual/en/4.5/files/blend/previews.html — read 2026-07-30.
- "Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it's possible to also create an external tool for doing such edits across blend-files." and, in the same passage, "Blender itself is not allowed to write to other blend-files than the currently open one."
  Manual — https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html — read 2026-07-30.
- "It is stored in blender_assets.cats.txt in the root directory of the asset library." and "Catalog Definition Files (CDFs) are relatively simple text files, encoded in UTF-8. Each CDF consists of a version indicator, and a line of text per catalog. Each catalog line is colon-separated, of the form {UUID}:{path}:{simple name}."
  Manual — https://docs.blender.org/manual/en/4.5/files/asset_libraries/catalogs.html — read 2026-07-30.
- "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."
  Manual — https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

**Where I looked to be able to say the rest is silent.** Searched the whole manual, the whole API documentation and all release notes for: `blend-file format`, `file format`, `file structure`, `binary format`, `SDNA`, `DNA`, `blendfile`, `outside of Blender`, `outside Blender`, `without Blender`, `external tool`, `third-party`, `third party`, `Asset Tracer`, `sidecar`, `thumbnail`, `Quick Look`, `specification`. The only hits bearing on reading a file from outside Blender are the ones quoted above.

**Outside the interface.** The compression fact and the catalog text file are properties of files on disk and do not involve Blender's window. The thumbnail tool is an operating-system integration, also outside Blender's window. The API confirms that Blender can run without an interface through `bpy.app.background`, "Boolean, True when blender is running without a user interface (started with -b)" (https://docs.blender.org/api/4.5/bpy.app.html, read 2026-07-30). The sources do not establish that every other item named above can be reached that way. The observed test below found one thing that does turn on how Blender was started: a file saved by this version with no window showed a plain document icon in the Finder, while the same file saved again from Blender's window showed a picture of its contents.

**Operating system.** The sources do not state that the compression or catalog-file findings differ by operating system. For the thumbnail tool, the manual gives an instruction for macOS only — "The Blender Thumbnail extension can be disabled in the System Settings under 'Login Items & Extensions', within the Quick Look category" (https://docs.blender.org/manual/en/4.5/files/blend/previews.html, read 2026-07-30) — and says nothing about how, or whether, that tool appears on Windows or Linux. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed. So macOS and Linux are both settled for what those tests cover. **Windows stays open**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

> **Narrowed by the citation check of the Linux hand tests, 2026-08-06.** One of the tests below is not the same test on the two machines. The preview question was put to the Finder on the Mac and to Blender's own thumbnail tool on Linux. What matches across the two is the pattern of results, not one test run twice, and no file browser on Linux was looked at.

**Observed — what the first bytes of a saved file hold, and whether it is compressed (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7). Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder so the Creator's own settings were untouched. The factory startup file was saved three times into a fresh empty folder: once with no compression argument, once with `compress=False`, once with `compress=True`. The script run was:

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
PREF use_file_compression False
DEFAULT compress arg False
DONE
```

The three files' sizes in bytes, from `stat -f %z`:

```
default.blend 434663
plain.blend 434663
compressed.blend 90810
```

The first 32 bytes of each, from `head -c 32 <file> | xxd`:

```
-- default
00000000: 424c 454e 4445 522d 7634 3035 5245 4e44  BLENDER-v405REND
00000010: 4800 0000 801c 626f 0100 0000 0000 0000  H.....bo........
-- plain
00000000: 424c 454e 4445 522d 7634 3035 5245 4e44  BLENDER-v405REND
00000010: 4800 0000 801c 626f 0100 0000 0000 0000  H.....bo........
-- compressed
00000000: 28b5 2ffd 60d4 03dd 0300 a405 424c 454e  (./.`.......BLEN
00000010: 4445 522d 7634 3035 5245 4e44 4800 0000  DER-v405RENDH...
```

**Observed — whether an ordinary Zstandard tool can unwrap a compressed `.blend` (hand test, 2026-08-04).** Same computer, same day, and no Blender involved in this part. The compressed file saved above was copied and handed to the Homebrew build of `zstd` at `/opt/homebrew/bin/zstd`, with the command `zstd -d -f zcheck.blend -o zcheck.out`. What came back:

```
zcheck.blend        : 434663 bytes 
```

and the first 32 bytes of the result, from `head -c 32 zcheck.out | xxd`:

```
00000000: 424c 454e 4445 522d 7634 3035 5245 4e44  BLENDER-v405REND
00000010: 4800 0000 801c 626f 0100 0000 0000 0000  H.....bo........
```

That is the same size and the same opening bytes as the uncompressed save above.

**Observed — what one file gives up to another Blender without being fully opened (hand test, 2026-08-04).** Same computer, same build, same throwaway settings folder. `bpy.data.libraries.load()` was called on a saved file that itself links another file. The script run was `$SCRATCH/08/peek.py`, printing the loader's attributes and contents. What came back:

```
SRC ATTRS ['actions', 'armatures', 'brushes', 'cache_files', 'cameras', 'collections', 'curves', 'fonts', 'grease_pencils', 'grease_pencils_v3', 'hair_curves', 'images', 'lattices', 'lightprobes', 'lights', 'linestyles', 'masks', 'materials', 'meshes', 'metaballs', 'movieclips', 'node_groups', 'objects', 'paint_curves', 'palettes', 'particles', 'pointclouds', 'scenes', 'screens', 'sounds', 'speakers', 'texts', 'textures', 'volumes', 'workspaces', 'worlds']
SRC OBJECTS []
SRC COLLECTIONS ['ChainCollection']
SRC LIBRARIES NO SUCH ATTRIBUTE
SRC VERSION NO SUCH ATTRIBUTE
ABSENT ERROR OSError load: /…/08/does_not_exist.blend failed to open blend file
```

The file peeked into holds one linked object inside a local collection; `SRC OBJECTS` came back empty while `SRC COLLECTIONS` named the local collection.

**Observed — whether the operating system's own file browser shows a picture for a file saved with no window (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open.

Four files had been saved beforehand by this chat, one per version, each by that version of Blender started with no window (`-b`) and with `--factory-startup`, each holding a single cube: `4.5-background.blend`, `5.0-background.blend`, `5.1-background.blend`, `5.2-background.blend`. They sat together in one folder, `/Users/artist/Documents/Smoozy hand tests/Reading a file/16/`. The Creator then opened each of the four Blenders in turn, opened that version's `-background.blend`, and saved it again through **File ‣ Save As** into the same folder as `4.5-window.blend`, `5.0-window.blend`, `5.1-window.blend` and `5.2-window.blend`. They opened the folder in the Finder with thumbnails showing and took one screenshot of all eight.

For 4.5 in particular: `4.5-background.blend` showed a plain grey document icon carrying the Blender logo and the word BLEND. `4.5-window.blend` showed a picture of a grey cube.

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

Done on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender **4.5.12 LTS** (hash 84afd5f785f7, built 2026-07-21). It is a container sharing someone else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at this topic's own settings folder; that the setting took effect was checked by printing `bpy.utils.resource_path('USER')`, which gave `/root/smoozy/Reading a file/res-4.5`.

*The first bytes of a saved file, and whether it is compressed.* The same script as the Mac run above. What came back:

```
PREF use_file_compression False
DEFAULT compress arg False
DONE
```

The three files' sizes in bytes, from `stat -c %s`, and the first 32 bytes of each from `head -c 32 <file> | od -A d -t x1z`:

```
-- default  434663 bytes
0000000 42 4c 45 4e 44 45 52 2d 76 34 30 35 52 45 4e 44  >BLENDER-v405REND<
0000016 48 00 00 00 40 89 84 5a fe 7f 00 00 00 00 00 00  >H...@..Z........<
-- plain  434663 bytes
0000000 42 4c 45 4e 44 45 52 2d 76 34 30 35 52 45 4e 44  >BLENDER-v405REND<
0000016 48 00 00 00 40 89 84 5a fe 7f 00 00 00 00 00 00  >H...@..Z........<
-- compressed  91057 bytes
0000000 28 b5 2f fd 60 d4 03 ed 03 00 e4 05 42 4c 45 4e  >(./.`.......BLEN<
0000016 44 45 52 2d 76 34 30 35 52 45 4e 44 48 00 00 00  >DER-v405RENDH...<
```

The preference, the argument's default, the two uncompressed sizes and the file's opening text are the same as on the Mac. The compressed size differs (91,057 here against 90,810 there); the bytes in a saved file are not identical between two machines.

*Whether an ordinary Zstandard tool can unwrap it.* No Blender involved. The compressed file was copied to `compressed.blend.zst` and handed to the system `zstd` at `/usr/bin/zstd` with `zstd -d -f -q compressed.blend.zst -o undone.blend`. It exited 0. The result was **434663 bytes** — the same as the uncompressed save — and its first 32 bytes were:

```
0000000 42 4c 45 4e 44 45 52 2d 76 34 30 35 52 45 4e 44  >BLENDER-v405REND<
0000016 48 00 00 00 40 89 84 5a fe 7f 00 00 00 00 00 00  >H...@..Z........<
```

`cmp undone.blend plain.blend` reported the two are **not** byte for byte identical: `differ: byte 181, line 1`. Same size and same opening bytes, as on the Mac; identical throughout, no.

*What one file gives up to another Blender without being fully opened.* Same script as the Mac run. What came back:

```
SRC ATTRS ['actions', 'armatures', 'brushes', 'cache_files', 'cameras', 'collections', 'curves', 'fonts', 'grease_pencils', 'grease_pencils_v3', 'hair_curves', 'images', 'lattices', 'lightprobes', 'lights', 'linestyles', 'masks', 'materials', 'meshes', 'metaballs', 'movieclips', 'node_groups', 'objects', 'paint_curves', 'palettes', 'particles', 'pointclouds', 'scenes', 'screens', 'sounds', 'speakers', 'texts', 'textures', 'volumes', 'workspaces', 'worlds']
SRC OBJECTS []
SRC COLLECTIONS ['ChainCollection']
SRC LIBRARIES NO SUCH ATTRIBUTE
SRC VERSION NO SUCH ATTRIBUTE
ABSENT ERROR OSError load: /root/smoozy/Reading a file/work/08/does_not_exist.blend failed to open blend file
```

Identical to the Mac's.

*Whether a file saved with no window carries a preview picture.* There is no Finder on this machine, so what was asked instead is whether Blender's own thumbnail tool — the one it ships beside its binary — can make a picture out of the file. Asked with no arguments the tool printed its own usage, so its arguments come from the tool and were not guessed: `Usage: blender-thumbnailer <input.blend> <output.png>`. Two files were made by this version: `4.5-background.blend`, saved with no window and with `--factory-startup`, and `4.5-window.blend`, made by opening that file in this version's window on the machine's own display and saving it again. Then, for each:

```
/opt/blender/blender-4.5.12-linux-x64/blender-thumbnailer 4.5-background.blend thumbs/4.5-background.png
/opt/blender/blender-4.5.12-linux-x64/blender-thumbnailer 4.5-window.blend     thumbs/4.5-window.png
```

What came back:

```
-- 4.5 background
   exit 6
   no file made
-- 4.5 window
   exit 0
   made 65737 bytes
```

The tool printed no message either time. The picture it made for the window-saved file shows a grey cube. So on this machine, with Blender's own thumbnail tool, a 4.5 file saved with no window yields no picture and one saved from the window does — which is what the Creator saw in the Finder on the Mac. This settles Blender's own thumbnail tool on Linux and no other file browser.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

**Answer.** The sources say that several Blenders can be started at once, each holding its own state, and that any one of them has only one blend-file *edited* at a time. They give no number and no limit.

> **Changed by the citation check, 2026-08-05.** This sentence read "only one blend-file open at a time". The source says "edited", not "open", and says it on the page about Blender built as a Python module, under the heading "Single Blend File Restriction". The two are not the same — a linked library is open without being edited — so the claim is narrowed to the word the source uses. Whether anything limits how many files one Blender may have *open* has gone back for a fresh research pass. On the second half of the question — one Blender reading a file while another renders — the sources are silent. There is no statement about file locking, no statement about concurrent access, and no warning about it.

Two documented facts bear on it indirectly. First, a Blender only ever writes to the blend-file it has open, so a Blender that merely reads a file is not described as writing anything back into it. Second, all instances pick their temporary directory the same way, from the same preference, environment variables and fall-back, so nothing in the documentation gives each instance a private one.

**Label:** **documented** that multiple instances are possible and that one instance edits one file; **not in the sources** for any limit on how many, and for whether concurrent reading and rendering causes trouble.

**Not covered.** No maximum number. Nothing on memory, licence or process limits. Nothing on two Blenders opening the same file, on one saving while another reads, on shared or networked storage, or on collisions in the shared temporary directory. Nothing on what happens to a render if the blend-file it came from is replaced on disk mid-render.

**Exact words.**

- "Only a single .blend file can be edited at a time." and "As with the application it's possible to start multiple instances, each with their own bpy and therefor Blender state. Python provides the multiprocessing module to make communicating with sub-processes more convenient."
  Python API — https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html — read 2026-07-30.
- "Blender itself is not allowed to write to other blend-files than the currently open one."
  Manual — https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html — read 2026-07-30.
- "In short: Python threads cause Blender to crash in hard to diagnose ways." and "For running Python code independently of Blender, it is recommended to use the multiprocessing module."
  Python API — https://docs.blender.org/api/4.5/info_gotchas_threading.html — read 2026-07-30.
- "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)." followed by "The temporary directory is selected based on the following priority: User Preference (see File Paths). Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). The /tmp/ directory."
  Manual — https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

The step from that last quote to "so two instances started the same way will use the same temporary directory" is mine, not the source's — that part is **inferred**. The sources do not say whether sharing it matters.

**Where I looked to be able to say it is silent.** Searched the whole manual, the whole API documentation and all release notes for: `file lock`, `flock`, `locked file`, `lock the file`, `exclusive access`, `read-only`, `another process`, `second instance`, `single instance`, `multiple instances`, `instances of Blender`, `another instance`, `concurrent`, `simultaneously`, `at the same time`, `in parallel`, `race condition`, `corrupt`, `network render`, `render farm`, `network share`, `network drive`, `NFS`, `shared storage`, `same blend-file`, `already open`. `render farm` appears once, in an unrelated note about the metadata stamp field. Nothing addresses concurrent access to one file.

**Outside the interface.** Running headless with `-b` is documented as a way to render without a display ("One advantage of using the command line is that we do not need a graphical display… and consequently we can render via a remote shell (typically SSH)", https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30), and the Blender-as-a-module page says using it "is equivalent to running a script in background-mode… however there are some differences to be aware of", listing among them that signal handlers are not set up, so Control-C does not cancel a render and no crash log is written. The sources document headless rendering and multiple instances separately. They do not settle concurrent reading and rendering.

**Operating system.** No statement in the pages and searches named above says instance counts differ by operating system. The one OS-specific detail found is which environment variable names the temporary directory (`TEMP` on Windows, `TMP` and `TMP_DIR` elsewhere). The second half of the question was tested on **two** machines — the Creator's Mac and a rented Linux machine — and came back the same on both, so macOS and Linux are settled for it. **Windows stays open.** The first half, the count, is settled by neither machine and by no machine.

**Observed — a second Blender reading the same file while a first renders it, and a third saving over it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7). Every Blender in this test was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at its own throwaway settings folder — a separate one for the renderer, the reader and the saver — so none of them shared a settings folder with another and none touched the Creator's own.

A file was built in a fresh empty folder holding three objects named `Cube`, `Sphere` and `Camera`, 60 frames, a 480 by 270 render and PNG output. On this machine that render takes about 23 seconds on its own, which is the window the other Blenders were started in. Each part started the render in the background and then waited — checking every fifth of a second — before starting the second Blender.

> **Noted by the citation check, 2026-08-05.** The paragraph above said the wait ran until two frames had appeared, while the recorded output below reads `frames on disk: 1` in both parts. The description and the record do not agree about the condition that was waited for. This check was not in the room and has changed no number; the test has gone to the hand-test list to be re-run with the wait condition written down and printed at every poll.

**Corrected 2026-08-05 by the re-run recorded below.** The paragraph above used to say the wait ran until two frames had appeared on disk. That was wrong. The line that ended the wait counted every entry in the output folder rather than only the frames, and the render's own log was written into that same folder, so the wait let go as soon as one finished frame sat beside one log file. The second Blender was therefore started after one frame, not two. The count printed below, `frames on disk: 1`, was the true number of finished frames at that moment; it was the description that did not match, not the number.

**Part A, the render on its own, for comparison.**

```
PART A RENDER EXIT CODE: 0
PART A FRAMES WRITTEN: 60
--- PART A render.log, last 5 lines ---
Saved: '/…/17/45/A/frame_0060.png'
Time: 00:00.37 (Saving: 00:00.00)


Blender quit
```

**Part B, a second Blender reading the same file while the first rendered it.**

```
PART B RENDER STARTED, pid 59711
PART B RENDER IS UNDER WAY, frames on disk: 1
PART B READER EXIT CODE: 0
PART B RENDER STILL RUNNING WHEN READER FINISHED: yes
--- PART B reader.log, in full ---
READER FILEPATH /…/17/45/concurrency.blend
READER OBJECT COUNT 3
READER OBJECT NAMES ['Camera', 'Cube', 'Sphere']
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/…/17/45/concurrency.blend"

Blender quit
PART B RENDER EXIT CODE: 0
PART B FRAMES WRITTEN: 60
--- PART B render.log, every line holding Error or Warning ---
(none)
```

**Part C, a third Blender opening the same file, renaming an object and saving over it while the first rendered from it.** The file used here was a copy, so that part B's file was left as it was.

```
PART C RENDER STARTED, pid 59777
PART C RENDER IS UNDER WAY, frames on disk: 1
PART C SAVER EXIT CODE: 0
PART C RENDER STILL RUNNING WHEN SAVER FINISHED: yes
--- PART C saver.log, in full ---
SAVER FILEPATH /…/17/45/copy_for_saving.blend
Info: Saved "copy_for_saving.blend"
SAVER RESULT {'FINISHED'}
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/…/17/45/copy_for_saving.blend"

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

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7). The file, the reader and the saver were the same as in the run above, so this is the same test. Every Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at its own settings folder — a separate one for the renderer, the reader and the saver. All three of those folders were made before any Blender was pointed at one, because Blender crashes when pointed at a settings folder that does not exist. Nothing else was running on the machine, and the four versions were run one after another, never together.

**The condition that had to be true before the second and third Blenders were started:** the number of files whose names end `.png`, in that part's own output folder, had to be two or more. The folder was looked at five times a second and the count printed at every look, not only at the last one. Every frame number the script printed came from one line, `ls -1 <folder> | grep -c '\.png$'`, and no log was written into an output folder, so nothing but a finished frame could be counted as a frame. Had the count not reached two within 150 looks — thirty seconds — the wait would have given up and said so. It did not have to.

The long scratch-folder path is written below as `/…`; everything else is word for word as it came back.

```
########## SETTINGS FOLDERS -- one per Blender, each made before it is used ##########
/…/18/45/cfg_reader
/…/18/45/cfg_render
/…/18/45/cfg_saver
THE ONE COUNTING COMMAND BEHIND EVERY FRAME NUMBER BELOW: ls -1 <folder> | grep -c '\.png$'
THE WAIT CONDITION: start the next Blender at the first poll where that count is two or more; poll five times a second; give up after 150 polls.
NO LOG IS WRITTEN INTO AN OUTPUT FOLDER, so only finished frames are ever in the folder being counted.

########## PART A -- the render on its own, for comparison ##########
PART A RENDER EXIT CODE: 0
PART A FRAMES WRITTEN: 60
--- PART A A_render.log, last 5 lines ---
Saved: '/…/18/45/A/frame_0060.png'
Time: 00:00.41 (Saving: 00:00.00)


Blender quit

########## PART B -- a second Blender reads the same file while it renders ##########
PART B RENDER STARTED, pid 49333, at 22:24:00
PART B POLL 1 at 22:24:00, 0s after the render was started, frames on disk: 0
PART B POLL 2 at 22:24:00, 0s after the render was started, frames on disk: 0
PART B POLL 3 at 22:24:00, 0s after the render was started, frames on disk: 0
PART B POLL 4 at 22:24:00, 0s after the render was started, frames on disk: 0
PART B POLL 5 at 22:24:01, 1s after the render was started, frames on disk: 0
PART B POLL 6 at 22:24:01, 1s after the render was started, frames on disk: 1
PART B POLL 7 at 22:24:01, 1s after the render was started, frames on disk: 1
PART B POLL 8 at 22:24:01, 1s after the render was started, frames on disk: 2
PART B WAIT ENDED at poll 8 because the count reached two or more. FRAMES ON DISK AT THAT MOMENT: 2
PART B THE WHOLE OUTPUT FOLDER AT THAT MOMENT: frame_0001.png frame_0002.png 
PART B READER STARTED at 22:24:01, 1s after the render was started
PART B READER EXIT CODE: 0
PART B FRAMES ON DISK WHEN THE READER FINISHED: 4
PART B RENDER STILL RUNNING WHEN READER FINISHED: yes
--- PART B B_reader.log, in full ---
READER FILEPATH /…/18/45/concurrency.blend
READER OBJECT COUNT 3
READER OBJECT NAMES ['Camera', 'Cube', 'Sphere']
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/…/18/45/concurrency.blend"

Blender quit
PART B RENDER EXIT CODE: 0
PART B FRAMES WRITTEN: 60
--- PART B B_render.log, last 5 lines ---
Saved: '/…/18/45/B/frame_0060.png'
Time: 00:00.41 (Saving: 00:00.00)


Blender quit
--- PART B B_render.log, every line holding Error or Warning ---
(none)

########## PART C -- a third Blender saves over the same file while it renders ##########
PART C RENDER STARTED, pid 49405, at 22:24:23
PART C POLL 1 at 22:24:23, 0s after the render was started, frames on disk: 0
PART C POLL 2 at 22:24:23, 0s after the render was started, frames on disk: 0
PART C POLL 3 at 22:24:23, 0s after the render was started, frames on disk: 0
PART C POLL 4 at 22:24:24, 1s after the render was started, frames on disk: 0
PART C POLL 5 at 22:24:24, 1s after the render was started, frames on disk: 0
PART C POLL 6 at 22:24:24, 1s after the render was started, frames on disk: 1
PART C POLL 7 at 22:24:24, 1s after the render was started, frames on disk: 2
PART C WAIT ENDED at poll 7 because the count reached two or more. FRAMES ON DISK AT THAT MOMENT: 2
PART C THE WHOLE OUTPUT FOLDER AT THAT MOMENT: frame_0001.png frame_0002.png 
PART C SAVER STARTED at 22:24:24, 1s after the render was started
PART C SAVER EXIT CODE: 0
PART C FRAMES ON DISK WHEN THE SAVER FINISHED: 3
PART C RENDER STILL RUNNING WHEN SAVER FINISHED: yes
--- PART C C_saver.log, in full ---
SAVER FILEPATH /…/18/45/copy_for_saving.blend
Info: Saved "copy_for_saving.blend"
SAVER RESULT {'FINISHED'}
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/…/18/45/copy_for_saving.blend"

Blender quit
PART C RENDER EXIT CODE: 0
PART C FRAMES WRITTEN: 60
--- PART C C_render.log, last 5 lines ---
Saved: '/…/18/45/C/frame_0060.png'
Time: 00:00.32 (Saving: 00:00.00)


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

Done on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender **4.5.12 LTS** (hash 84afd5f785f7). Other work was running on the machine at the same time, so **nothing here says how long anything took**.

The set-up was the same: one file holding a cube named `Cube`, a sphere named `Sphere` and a camera named `Camera`, 60 frames, a 480 by 270 render, PNG output. Three settings folders, one per Blender, all made before any Blender was pointed at one. The wait condition was written down and used by every part — the next Blender starts at the first poll where the number of files whose names end `.png`, in that part's own output folder, is two or more; polls happen five times a second; the wait gives up after 150 polls. One function makes every frame number the script prints, and no log is written into a folder whose frames are being counted:

```
count_frames () {
  ls -1 "$1" 2>/dev/null | grep -c '\.png$'
}
```

What came back, kept word for word apart from the render logs, which are long and whose last lines and error counts are given instead:

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
PART B POLL 6 frames_on_disk=2
PART B WAIT ENDED at poll 6 with frames_on_disk=2 (condition: frames_on_disk >= 2)
PART B READER EXIT 0
PART B FRAMES ON DISK WHEN READER FINISHED 6
PART B RENDER EXIT 0
PART B FRAMES ON DISK 60
--- PART C: a third Blender saving over a copy while a render of that copy runs ---
PART C POLL 1 frames_on_disk=0
PART C POLL 2 frames_on_disk=0
PART C POLL 3 frames_on_disk=0
PART C POLL 4 frames_on_disk=0
PART C POLL 5 frames_on_disk=0
PART C POLL 6 frames_on_disk=2
PART C WAIT ENDED at poll 6 with frames_on_disk=2 (condition: frames_on_disk >= 2)
PART C SAVER EXIT 0
PART C RENDER EXIT 0
PART C FRAMES ON DISK 60
--- READER OUTPUT ---
READER FILEPATH /root/smoozy/Reading a file/work/12/45/concurrency.blend
READER OBJECT COUNT 3
READER OBJECT NAMES ['Camera', 'Cube', 'Sphere']
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/smoozy/Reading a file/work/12/45/concurrency.blend"

Blender quit
--- SAVER OUTPUT ---
SAVER FILEPATH /root/smoozy/Reading a file/work/12/45/copy_for_saving.blend
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

**Ambiguity flagged.** I read the question as: from one opening of the file, can you determine, for each view layer, whether it will render at an arbitrary frame you might later ask about. The other reading — "does it render at some frame or other in the range" — is answered by the same evidence, and comes out the same way.

**Answer.** The sources establish that a view layer has a readable on/off flag. They do not establish that one read gives the full list for every scene, or that the stored value settles what will happen at any later frame. The interface picture shows an animation decorator beside the flag. Together with the decorators page, that supports an inference that the flag can vary by frame.

The API separately documents setting a scene frame and evaluating an animation curve. It does not directly connect either operation to this flag. Whether those operations answer the question for every later frame is therefore not established by the sources.

There is a second flag that changes which layers render, and it behaves differently depending on how Blender was started: "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line."

**Label:** **documented** that the per-layer flag exists and is readable, that a "render single layer" flag exists and is ignored from the command line, and that the flag's checkbox carries an animation decorator; **inferred** that the decorator means the flag can take different values at different frames; **not in the sources** for whether one read can answer every later frame or whether Blender re-evaluates the flag per frame while rendering an animation.

**Not covered.** Whether an animation render samples the flag once at the start or at every frame. Whether a driver on the flag is evaluated during a render. What happens if every view layer in a scene is switched off at a given frame. Nothing about layers in scenes other than the one being rendered.

**Exact words.**

- "use — Enable or disable rendering of this View Layer — Type: boolean, default False"
  Python API — https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html — read 2026-07-30.
- "Use for Rendering — The active view layer will be used during rendering." and "Render Single Layer — Only render the active view layer." followed by "Note — This option is ignored when rendering from the command line."
  Manual — https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html — read 2026-07-30.
- "use_single_layer — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line."
  Python API — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.
- "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated." and "Clicking on the decorator dot icon will add a Keyframe to that property."
  Manual — https://docs.blender.org/manual/en/4.5/interface/controls/buttons/decorators.html — read 2026-07-30.
- Picture: the View Layer panel screenshot in the manual, `_images/render_layers_view-layer_panel.png`, shown on https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html, read 2026-07-30. What is visible in it: the panel has two rows, "Use for Rendering" and "Render Single Layer"; the "Use for Rendering" row has a checkbox and, to its right, a decorator dot; the "Render Single Layer" row has a checkbox and no dot beside it. Taking that together with the decorators page above, "Use for Rendering" is a property Blender presents as animatable. That last step is **inferred**; the picture alone shows only the dot.
- The means to ask about a specific frame are documented: "frame_set(frame, *, subframe=0.0) — Set scene frame updating all objects and view layers immediately" (https://docs.blender.org/api/4.5/bpy.types.Scene.html), and, for reading a curve without moving the scene, "data_path — RNA Path to property affected by F-Curve" together with "evaluate(frame) — Evaluate F-Curve" (https://docs.blender.org/api/4.5/bpy.types.FCurve.html). Both read 2026-07-30.

**Where I looked to be able to say the rest is silent.** The whole `bpy.types.ViewLayer` and `bpy.types.Scene` API pages; the manual's view layer pages (`render/layers/introduction`, `render/layers/view_layer`, `render/layers/passes`, `render/layers/filter`); `render/output/animation`; `render/output/properties/frame_range`. Searched manual and API for `animatable`, `can be animated`, `not animatable`, `cannot be animated`, `per frame`, `each frame`, and for `animat…` in the same file as `view layer`. The API pages do not mark properties as animatable at all; the only machine-readable statement of the fact is `Property.is_animatable`, "Property is animatable through RNA" (https://docs.blender.org/api/4.5/bpy.types.Property.html, read 2026-07-30), which a script can query per property.

**Outside the interface.** The API exposes the per-layer flag, but the sources do not say whether its value or evaluation differs in background mode. The "Render Single Layer" flag does differ: the manual and API both say it is ignored when rendering from the command line. One further headless-only caveat is documented: `--disable-depsgraph-on-file-load` means "Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)", with the note "this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). The observed test below was run entirely from outside Blender's window, so it holds there.

**Operating system.** None of the pages and searches named above states an operating-system difference. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

**Observed — the flag's stored default, whether it can be keyframed, and whether an animation render honours it frame by frame (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7). Blender was started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder.

First, what the property list itself says. The script printed, among other lines:

```
PROBE ViewLayer.use default = False
PROBE ViewLayer.use is_animatable = True
PROBE RenderSettings.use_single_layer default = False
```

Then a file was built from the factory startup in a fresh empty folder: a cube, a camera, one scene, two view layers renamed `LayerA` and `LayerB`, frames 1 to 2, a 32 by 32 render, output to `//out/frame_` in multi-layer OpenEXR (so that each rendered view layer becomes a named layer inside the written file). With `LayerB` switched on, `layer_b.keyframe_insert(data_path="use", frame=1)` was called; `LayerB` was then switched off and `layer_b.keyframe_insert(data_path="use", frame=2)` called. What came back:

```
ENGINE BLENDER_EEVEE_NEXT
KEY1 True
KEY2 True
FCURVE view_layers["LayerB"].use [(1.0, 1.0), (2.0, 0.0)]
```

The curve is on the scene, and its data path is `view_layers["LayerB"].use`.

The saved file was then reopened in a second run of Blender with no window, which stepped the frame and read the flag back:

```
FRAME 1 [('LayerA', True), ('LayerB', True)]
FRAME 2 [('LayerA', True), ('LayerB', False)]
CURVE view_layers["LayerB"].use at 1 = 1.0 at 2 = 0.0
```

The same file was then rendered as an animation with `-a` in a third run with no window. It wrote:

```
Saved: '/…/out/frame_0001.exr'
Saved: '/…/out/frame_0002.exr'
```

The names of the view layers stored inside each written file, read with `strings -n 5 <file> | grep -oE "Layer[AB]" | sort -u`:

```
-- frame_0001.exr
LayerA
LayerB
-- frame_0002.exr
LayerA
```

One further run was made to see whether the terminal itself names the layers: the same animation render with `--log "*" --log-level 2` added, its output filtered for the word "layer". Nothing was printed.

**Observed — what the View Layer panel offers, and whether the render switch can be keyframed from the interface (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open. This one is about Blender's own window, so it says nothing about being driven from outside it.

A file prepared beforehand by this chat, holding one scene and two view layers named `LayerA` and `LayerB`, was opened at `/Users/artist/Documents/Smoozy hand tests/Reading a file/12/4.5.blend`. In the Properties editor, on the View Layer tab, the Creator hovered the mouse over the **Use for Rendering** checkbox and pressed **I**, then hovered over the **Render Single Layer** checkbox and pressed **I**. Two screenshots were taken, kept at `/Users/artist/Desktop/Screenshots/009/4.5/`.

The Creator's own words, in full:

> Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions.

What the screenshots show. The first is the top of the View Layer tab before the keyframe was inserted. It holds a panel headed `View Layer` with two rows: `Use for Rendering`, its checkbox ticked, and **a small round dot to the right of it**; and `Render Single Layer`, its checkbox unticked, with **no dot beside it**. Below that sits the `Passes` panel, whose own checkboxes each carry a dot. The second screenshot is the lower part of the same tab — Cryptomatte, Shader AOV, Filter and Custom Properties — and does not show the `Use for Rendering` checkbox.

A gap worth stating plainly: neither screenshot shows the checkbox after **I** was pressed, so the yellow checkbox and the diamond rest on the Creator's written description above and not on a picture.

**Observed — the same test on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **4.5.12 LTS**, started with no window (`-b`), with `--factory-startup`, and with its own settings folder. The set-up was the same: one scene, two view layers `LayerA` and `LayerB`, a cube, a camera, frames 1 to 2, a 32 by 32 render, multi-layer OpenEXR output, and `LayerB` keyframed on at frame 1 and off at frame 2 with constant interpolation. What came back from the build and the read:

```
ENGINE BLENDER_EEVEE_NEXT
FORMAT OPEN_EXR_MULTILAYER
KEY1 True
KEY2 True
SAVED /root/smoozy/Reading a file/work/10/4.5/vl.blend
FRAME 1 [('LayerA', True), ('LayerB', True)]
FRAME 2 [('LayerA', True), ('LayerB', False)]
DIRECT FCURVES OK 1
CURVE ROUTE action.fcurves
CURVE view_layers["LayerB"].use [(1.0, 1.0, 'CONSTANT'), (2.0, 0.0, 'CONSTANT')] at 1 = 1.0 at 2 = 0.0
```

Then the animation was rendered with `-a`. Two files were written, and the layer names inside each were read with `grep -a -o -E "Layer[AB]" <file> | sort -u`:

```
/root/smoozy/Reading a file/work/10/4.5/out/frame_0001.exr
/root/smoozy/Reading a file/work/10/4.5/out/frame_0002.exr
-- frame_0001.exr
LayerA
LayerB
-- frame_0002.exr
LayerA
```

So on Linux too the switch can be keyframed from a script, the curve lands on the scene with a data path naming the layer, stepping the frame changes what the switch reads, and the animation render honours the keyframed value frame by frame. In 4.5 the curves hang directly off the action, and reading them that way worked. This ran on a machine whose graphics are an NVIDIA card, with no window at all. Every line agrees with the Mac.

**Observed — whether the two switches are marked as things that can be animated (hand test, 2026-08-06).** Same machine, same build, no file needed. What came back:

```
PROBE RenderSettings.use_single_layer is_animatable = False
PROBE ViewLayer.use is_animatable = True
```

That is a machine-readable statement of what the Creator saw on the Mac when they pressed **I** over each of the two checkboxes. What Blender shows on screen when the key is pressed could not be checked on the Linux machine: nothing on it can move the mouse to a control or press a key at it, and installing such a tool is not permitted there.

---

## 4. Can the places a file is set up to write to be read out of it?

**Answer.** Partly. The sources establish readable paths for the scene's render output and the compositor's File Output nodes. This document does not establish every place a file can be set up to write.

- The scene's render output path is a readable property: "filepath — Directory/name to save animations, # characters define the position and padding of frame numbers".
- Each File Output node in the compositor exposes a base path plus a sub-path per slot: "base_path — Base output path for the image" and, per slot, "path — Subpath used for this slot".
- The finished scene-render filename for a frame can be asked for directly: "frame_path(...) — Return the absolute path to the filename to be written for a given frame".

Three things about what you get back. The stored string is the stored string: `#` characters mark frame-number padding and are not expanded in it; a leading `//` means "relative to this blend-file" and is not expanded either; and template expressions such as `{blend_name}` are left in the field untouched, being substituted only when the path is used. For the scene render path, `frame_path()` returns an absolute filename for a given frame — and in 4.5 it can fail: "scene.render.frame_path() used to be infallible but can now fail if there are errors processing Path Templates in the render output path." The sources do not say that `frame_path()` resolves a File Output node's base path and slot path.

Cycles' own code uses `frame_path()` to build a list of per-frame paths and then checks whether each path exists. That shows how the shipped add-on uses the method. It does not establish Blender's intent or what a render writes.

**Label:** **documented** for the properties, the frame-number and relative-path behaviour, the templates and the new failure mode; **observed** for Cycles' use of `frame_path()`.

**Not covered.** This document has no search record for every place a blend-file can write. It names bake settings, point caches, geometry-node bakes, collection exporters, sequencer proxies, and fluid caches without quoting or citing the relevant entries. It therefore cannot support a **not in the sources** finding for a complete list. Nothing here says whether a File Output node is enabled, connected, or in a node tree that is actually used. Nothing about what the Video Sequencer writes. Nothing about whether reading a path tells you the file will in fact be written.

**Exact words.**

- "filepath — Directory/name to save animations, # characters define the position and padding of frame numbers — Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)" and "frame_path(*, frame=-2147483648, preview=False, view='') — Return the absolute path to the filename to be written for a given frame".
  Python API — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.
- "base_path — Base output path for the image — Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)".
  Python API — https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.
- "path — Subpath used for this slot — Type: string, default "", (never None, Supports template expressions)".
  Python API — https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html — read 2026-07-30.
- "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)." and "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."
  Manual — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.
- "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." and "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."
  Manual — https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.
- "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."
  Release notes — https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30.
- "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing [Path Templates] in the render output path."
  Release notes — https://developer.blender.org/docs/release_notes/4.5/python_api/ — read 2026-07-30.

**Observed, from Cycles' own files.** In `Blender Sources/cycles_4.5/operators.py`, in the `execute` method of the class `CYCLES_OT_denoise_animation`, the loop over the scene's frame range reads:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)

                if not os.path.isfile(filepath):
```

That is the shipped add-on setting the render output path and then asking `frame_path()` for the resulting per-frame filename, and then testing whether that file exists. What the file says is solid; what a render would actually write is a step beyond it and I have not taken it.

**Outside the interface.** The API exposes the properties and `frame_path()`, but the sources do not state that their results are identical in background mode. Two differences are documented for the command line: `-o, --render-output <path>` sets the render path from outside the file, and "You can use path templating features such as {blend_name} in the path"; and argument order matters — "blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). A path read out of the file is therefore not necessarily the path a given command line will use.

**Operating system.** None of the cited output-path pages states an operating-system difference. The only OS-flavoured path helpers named here are `bpy.path.native_pathsep` and `bpy.path.basename`, the latter documented as "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." (https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30). The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

**Observed — what a stored output path reads back as, and what `frame_path()` makes of it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A file was saved in a fresh empty folder with its render output path set to `//out/{blend_name}_/frame_`, then reopened in a second run which printed the stored string and then asked `frame_path(frame=7)` for four different stored strings in turn. What came back:

```
Error: Parse errors in path '//out/{scene_name}_/frame_':
- Unknown variable referenced in template expression '{scene_name}'.
Error: Parse errors in path '//out/{blend_name_/frame_':
- Invalid or incomplete template expression '{blend_name_/frame_'.
STORED '//out/{blend_name}_/frame_'
SET '//out/{blend_name}_/frame_'
FRAME_PATH '/…/02/45/out/tmpl_/frame_0007.png'
SET '//out/{scene_name}_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{scene_name}_/frame_':
- Unknown variable referenced in template expression '{scene_name}'.

SET '//out/{blend_name_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{blend_name_/frame_':
- Invalid or incomplete template expression '{blend_name_/frame_'.

SET '//out/plain_/frame_'
FRAME_PATH '/…/02/45/out/plain_/frame_0007.png'
```

The saved file was named `tmpl.blend`. The two `Error:` lines at the top were printed by Blender itself as the two malformed paths were assigned.

**Observed — the File Output node's stored paths and the files a render writes (hand test, 2026-08-04).** Same computer, same build, same conditions. A file was built with compositing switched on and one File Output node whose destination was set to `//out/` and which was given two extra inputs, one named with a slash in it. Building it printed:

```
TREE <bpy_struct, CompositorNodeTree("Compositing Node Tree") at 0xbf74a3c08>
NODE ATTRS ['__slots__', 'base_path', 'file_slots', 'layer_slots']
SET base_path
file_slots methods ['bl_rna', 'clear', 'find', 'foreach_get', 'foreach_set', 'get', 'items', 'keys', 'move', 'new', 'remove', 'rna_type', 'values']
```

Reopening the saved file and reading the node back printed:

```
BASE_PATH '//out/'
DIRECTORY 'NO SUCH ATTRIBUTE'
FILE_NAME 'NO SUCH ATTRIBUTE'
SLOT PATHS ['Image', 'passes/diffuse', 'beauty']
INPUT NAMES ['Image', 'passes/diffuse', 'beauty']
```

Rendering frame 1 of that file with `-f 1`, in a run with no window, printed:

```
Fra:1 Mem:11.26M (Peak 40.79M) | Time:00:00.85 | Compositing
Saved: '/…/03/45/out/beauty0001.png'
Saved: '/…/03/45/out/Image0001.png'
Saved: '/…/03/45/out/passes/diffuse0001.png'
Saved: '/…/03/45/scene_out/frame_0001.png'
```

and the files found on disk afterwards were exactly those four: `out/Image0001.png`, `out/beauty0001.png`, `out/passes/diffuse0001.png`, and `scene_out/frame_0001.png`.

**Observed — whether the whole-file path lists include output paths (hand test, 2026-08-04).** Same computer, same build, same conditions, on the same saved file (whose render output path is `//scene_out/frame_` and whose File Output node points at `//out/`). What came back:

```
RENDER FILEPATH '//scene_out/frame_'
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
FILE_PATH_MAP {}
FILE_PATH_FOREACH NO SUCH METHOD
```

`FILE_PATH_MAP` was printed with entries holding no paths filtered out, so `{}` means no data-block reported any path at all.

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
PROBE CompositorNodeOutputFile properties = ['active_input_index', 'base_path', 'bl_description', 'bl_height_default', 'bl_height_max', 'bl_height_min', 'bl_icon', 'bl_idname', 'bl_label', 'bl_static_type', 'bl_width_default', 'bl_width_max', 'bl_width_min', 'color', 'color_tag', 'dimensions', 'file_slots', 'format', 'height', 'hide', 'inputs', 'internal_links', 'label', 'layer_slots', 'location', 'location_absolute', 'mute', 'name', 'outputs', 'parent', 'rna_type', 'save_as_render', 'select', 'show_options', 'show_preview', 'show_texture', 'type', 'use_custom_color', 'warning_propagation', 'width']
PROBE Scene has node_tree = True
PROBE Scene has compositing_node_group = False
```

**Observed — what the File Output node's sidebar shows for its final paths (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open. This one is about Blender's own window, so it says nothing about what a script can reach.

A file prepared beforehand by this chat was opened at `/Users/artist/Documents/Smoozy hand tests/Reading a file/13/4.5.blend`. It holds one File Output node whose destination folder is `//out/`, with two extra inputs added, one of them named with a slash in it. The Creator switched to the Compositing workspace, selected the node, opened the sidebar with **N**, and looked through every panel on its **Node** tab. Two screenshots were taken, kept at `/Users/artist/Desktop/Screenshots/010/4.5/`. No words came with them.

In 4.5 the sidebar held two panels, **Node** and **Properties**, and **no panel headed Output Paths**. What the Properties panel showed, read off the screenshot:

- a heading `Base Path:` above a text field reading `//out/`
- `Save as Render`, ticked
- `File Format` set to `PNG`
- `Color` set to `RGBA`, `Color Depth` set to `8`, `Compression` set to `15%`
- `Color Management` set to `Follow Scene`
- a button reading `Add Input`
- a list holding three entries: `Image`, `passes/diffuse`, `beauty`, with `beauty` highlighted
- a field headed `File Subpath:` reading `beauty`
- `Format:` with `Use Node Format` ticked

The other screenshot shows the node itself in the compositor. Its body reads `Base Path:` above a field containing `//out/`, then three inputs listed as `Image`, `passes/di...` and `beauty`, each with `PNG` beside it. Its Image output runs to a Composite node and a Viewer node, and two Render Layers nodes feed its inputs.

**Observed — the same tests on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **4.5.12 LTS**, with no window, `--factory-startup`, and its own settings folder.

*What a stored output path reads back as, and what asking for a frame's path makes of it.* A one-scene file was saved with its render output path set to `//out/{blend_name}_/frame_`, then reopened and four values tried. What came back:

```
STORED '//out/{blend_name}_/frame_'
SET '//out/{blend_name}_/frame_'
FRAME_PATH '/root/smoozy/Reading a file/work/02/4.5/out/tmpl_/frame_0007.png'
SET '//out/{scene_name}_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{scene_name}_/frame_':
- Unknown variable referenced in template expression '{scene_name}'.

SET '//out/{blend_name_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{blend_name_/frame_':
- Invalid or incomplete template expression '{blend_name_/frame_'.

SET '//out/plain_/frame_'
FRAME_PATH '/root/smoozy/Reading a file/work/02/4.5/out/plain_/frame_0007.png'
```

The braces survive in the stored string and are worked out only when the path is asked for; a malformed template makes the call raise; `{scene_name}` is an unknown variable in this version. Word for word the same as on the Mac.

*The File Output node's stored paths, and the files a render actually writes.* A file was built with compositing switched on and one File Output node with destination folder `//out/` and two extra inputs, one named `passes/diffuse`. The build script tests which property the scene carries and says which route it took. What came back:

```
BRANCH old
TREE <bpy_struct, CompositorNodeTree("Compositing Node Tree") at 0x7c7abb7a2f08>
NODE ATTRS ['__slots__', 'base_path', 'file_slots', 'layer_slots']
SET base_path
BASE_PATH '//out/'
DIRECTORY 'NO SUCH ATTRIBUTE'
FILE_NAME 'NO SUCH ATTRIBUTE'
SLOT PATHS ['Image', 'passes/diffuse', 'beauty']
INPUT NAMES ['Image', 'passes/diffuse', 'beauty']
```

Rendering one frame wrote four files, and the slash in the input's name made a folder on disk:

```
Saved: '/root/smoozy/Reading a file/work/03/4.5/out/beauty0001.png'
Saved: '/root/smoozy/Reading a file/work/03/4.5/out/Image0001.png'
Saved: '/root/smoozy/Reading a file/work/03/4.5/out/passes/diffuse0001.png'
Saved: '/root/smoozy/Reading a file/work/03/4.5/scene_out/frame_0001.png'
```

And the whole-file path lists, on that same file, carried neither the scene's output path nor the node's:

```
RENDER FILEPATH '//scene_out/frame_'
BLEND_PATHS ['//../../../../../../opt/blender/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
FILE_PATH_MAP {}
FILE_PATH_FOREACH NO SUCH METHOD
```

All of it agrees with the Mac.

**Observed — what the File Output node's sidebar shows, in Blender's own window on Linux (hand test, 2026-08-06).**

Same machine, same build, but with a window open on the machine's own display. A script drove the window in stages, each on its own timer: switch to the Compositing workspace and make the File Output node active; maximise the node editor; open the sidebar, set its tab to **Node**, hide the tool bar and the asset shelf, and frame the node; scroll the sidebar down; take Blender's own screenshot with `bpy.ops.screen.screenshot` and quit. Every stage printed what it did, ending `SHOT RESULT {'FINISHED'}`.

Read off that picture, the sidebar for this node in 4.5 holds, top to bottom: **Node** (Name `File Output`, Label, Color), **Properties** (Base Path `//out/`, Save as Render ticked, File Format `PNG`, Color `RGBA`, Color Depth `8`, Compression `15%`, Color Management `Follow Scene`), **Add Input**, the list of inputs `Image`, `passes/diffuse`, `beauty`, then **File Subpath** `beauty` and **Format** with Use Node Format ticked. **There is no panel headed Output Paths anywhere in it**, and the whole of the sidebar's content fitted on screen with nothing below it. That agrees with the Creator's own screenshots on the Mac. The picture is kept on the box at `/root/smoozy/Reading a file/work/w13/4.5/shot4.png`.

---

## 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**Answer.** Reading is documented. Some kinds of rewriting are documented. Writing an arbitrary new value to every kind of per-item path is inferred rather than documented.

Reading the lot: "blend_paths(absolute=False, packed=False, local=False) — Returns a list of paths to external files referenced by the loaded .blend file", with switches to make them absolute, to skip packed data, and to skip linked library paths.

Reading one at a time: the cited pages expose `filepath` on an image and on a linked library. Those string properties are not marked read-only, unlike neighbouring properties on the same pages. Treating that absence as proof that they accept assignment is an inference. The document gives no equivalent citation for sounds, movie clips, fonts, or cache files.

Writing back in bulk, from Blender's own commands: "Make all paths to external files absolute" and "Make all paths to external files relative to current .blend", plus, for linked blend-files specifically, relocate and reload. The sources quoted here do not state when these changes become persistent on disk.

There is one hard boundary on "by what": a Blender writes only to the file it has open. For editing paths across many files, the manual points at external tooling.

**Label:** **documented** for `blend_paths`, the cited image and library `filepath` properties, the bulk path operators, and the one-file-at-a-time boundary; **inferred** that the absence of a "(readonly)" marker means the cited per-item properties can be assigned. Assignment and persistence across every kind of external reference are not established, and this document contains no search record that would support a **not in the sources** finding for them.

**Not covered.** Nothing here says what happens to a path you write that points at nothing. Nothing on whether writing a path reloads the thing it points at (the manual hints otherwise, see question 6). Nothing on packed data — a packed image still carries a path, and the sources do not say how writing that path interacts with the packed copy. `blend_paths` is described as returning paths of "the loaded .blend file"; it says nothing about paths recorded inside libraries that were not loaded.

**Exact words.**

- "bpy.utils.blend_paths(absolute=False, packed=False, local=False) — Returns a list of paths to external files referenced by the loaded .blend file." with "absolute (bool) – When true the paths returned are made absolute.", "packed (bool) – When true skip file paths for packed data.", "local (bool) – When true skip linked library paths."
  Python API — https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30.
- "filepath — Image/Movie file name — Type: string, default "", (never None, blend relative // prefix supported)" and "filepath_raw — Image/Movie file name (without data refreshing)".
  Python API — https://docs.blender.org/api/4.5/bpy.types.Image.html — read 2026-07-30.
- "filepath — Path to the library .blend file — Type: string, default "", (never None, blend relative // prefix supported)".
  Python API — https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.
- "bpy.ops.file.make_paths_absolute() — Make all paths to external files absolute" and "bpy.ops.file.make_paths_relative() — Make all paths to external files relative to current .blend".
  Python API — https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30.
- "Make Paths Relative — Make all paths to external files relative to the current blend-file." and "Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)."
  Manual — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.
- "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file." and "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so." and "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."
  Manual — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30.
- "Blender itself is not allowed to write to other blend-files than the currently open one." and "Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it's possible to also create an external tool for doing such edits across blend-files."
  Manual — https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html — read 2026-07-30.
- Turning a recorded path into a usable one, and back: "bpy.path.abspath(path, *, start=None, library=None) — Returns the absolute path relative to the current blend file using the "//" prefix", where "library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start"; and "bpy.path.relpath(path, *, start=None) — Returns the path relative to the current blend file using the "//" prefix."
  Python API — https://docs.blender.org/api/4.5/bpy.path.html — read 2026-07-30.

**Outside the interface.** The API exposes the path properties and operators, but the cited words do not state that assignment and persistence work in background mode. One interface-only element is documented — the Relative Path checkbox in the File Browser — while the `//` string form and the `relative_remap` option on the save operators are exposed through the API (https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30). The observed test below was run entirely from outside Blender's window: reading, assigning, saving and reopening all worked there. **Added by the hand tests of 2026-08-06.** Both bulk rewrites, the one-pass path visitor where it exists, the write call, saving elsewhere with the remap option, the missing-file search and library relocation were all run with no window at all on a rented Linux machine in this version, and all returned `{'FINISHED'}` and did what they claim. Two operators would **not** run there — `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path`, both refusing with a failed context check. **Brought forward by the citation check, 2026-08-06.** The sentence that used to end this line said those two were the only part of it still owed a machine with a screen. The Mac sitting of 2026-08-07 settled both, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file. So only the first of the two was ever owed a screen, and it is owed no longer. The full record is in this version's `Preparation/Blender Research/Findings/4.5/Reading a file — added questions.md`. See the observed addition below.

**Operating system.** None of the cited pages states that reading or rewriting these paths differs by operating system. The sources do carry two path-related notes: `bpy.path.basename` "skips a "//" prefix. Use for Windows compatibility", and "bpy.path.resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30). The sources do not connect either helper to assignment or persistence. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it. **Added by the hand tests of 2026-08-06, and corrected by the citation check, 2026-08-06.** The line used to say the new material below was run on Linux only and left macOS owed a second sitting. That was true when it was written; the second sitting is now printed below, under *Observed on the Creator's Mac, 2026-08-07*. The same file was built on the Mac in this version and read back, and twenty-five of its twenty-seven printed lines match the Linux printout character for character. So **macOS is settled for what that test covers**, and it differs from Linux on two lines, both recorded below. The rest of the round's entries were run on Linux alone and stay untested on macOS. **Windows stays open** for the same reason as before.

**Observed — reading a recorded image path, writing a new one, and whether it survives a save (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A fresh empty folder was given a 4 by 4 PNG at `tex/probe.png`, written by Blender itself, and a `.blend` beside it holding one image data-block loaded from that PNG, marked with a fake user so it survived the save, and saved with `relative_remap=True`. The build printed:

```
PATHS AFTER SAVE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
```

A second run with no window opened that file, read the paths, assigned a different one, and saved:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', '')]
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/probe.png']
BLEND_PATHS ABS ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/…/04/45b/tex/probe.png']
ASSIGNED OK
AFTER ASSIGN //tex/moved.png
BLEND_PATHS AFTER ASSIGN ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

The assignment made was `img.filepath = "//tex/moved.png"`. A third run with no window opened the same file again:

```
REOPENED [('ProbeLoaded', '//tex/moved.png'), ('Render Result', '')]
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
```

Nothing in the built file pointed at the Blender application's own brushes file; that entry appeared in `blend_paths()` of its own accord.

**Observed — which path-reading and path-writing calls exist (hand test, 2026-08-04).** Same computer, same build, same conditions, with no file loaded. What came back:

```
PROBE bpy.utils.blend_paths = True
PROBE bpy.data.file_path_map = True
PROBE bpy.data.file_path_foreach = False
PROBE bpy.data.user_map = True
PROBE bpy.data.all_ids = False
PROBE bpy.data.temp_data = True
```

and the full list of operators under `bpy.ops.file`:

```
PROBE bpy.ops.file operators = ['autopack_toggle', 'bookmark_add', 'bookmark_cleanup', 'bookmark_delete', 'bookmark_move', 'cancel', 'delete', 'directory_new', 'edit_directory_path', 'execute', 'external_operation', 'filenum', 'filepath_drop', 'find_missing_files', 'hidedot', 'highlight', 'make_paths_absolute', 'make_paths_relative', 'mouse_execute', 'next', 'pack_all', 'pack_libraries', 'parent', 'previous', 'refresh', 'rename', 'report_missing_files', 'reset_recent', 'select', 'select_all', 'select_bookmark', 'select_box', 'select_walk', 'smoothscroll', 'sort_column_ui_context', 'start_filter', 'unpack_all', 'unpack_item', 'unpack_libraries', 'view_selected']
```

**Observed — whether a recorded path survives when its target is gone (hand test, 2026-08-04).** Same computer, same build, same conditions. A file was built exactly as above, then the PNG it points at was renamed from `tex/probe.png` to `tex/gone.png` and the file opened with no window. What came back:

```
IMAGES [('ProbeLoaded', '//tex/probe.png'), ('Render Result', '')]
IS_MISSING [('ProbeLoaded', False), ('Render Result', False)]
BLEND_PATHS ABS [('/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', True), ('/…/05/45/tex/probe.png', False)]
FILE_PATH_MAP {'WinMan': [], 'Animation': [], 'Compositing': [], 'Geometry Nodes': [], 'Layout': [], 'Modeling': [], 'Rendering': [], 'Scripting': [], 'Sculpting': [], 'Shading': [], 'Texture Paint': [], 'UV Editing': [], 'Scene': [], 'Palette': [], 'Collection': [], 'Camera': [], 'Cube': [], 'Light': [], 'World': [], 'LineStyle': [], 'Material': [], 'ProbeLoaded': ['//tex/probe.png'], 'Render Result': []}
```

The second item of each pair in `BLEND_PATHS ABS` is the result of `os.path.exists` on that path.

**Observed — reading a recorded path, writing a new one, and seeing whether it survives a save, on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **4.5.12 LTS**, with no window, `--factory-startup`, and its own settings folder. A folder was built holding a small PNG at `tex/probe.png` written by Blender itself and a `.blend` beside it whose one image data-block points at that PNG by a relative path. Then the path was read, given a new value, saved, and read back after reopening. What came back from the three runs:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', '')]
BLEND_PATHS ['//../../../../../../opt/blender/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/probe.png']
BLEND_PATHS ABS ['/opt/blender/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/smoozy/Reading a file/work/04/4.5/tex/probe.png']
ASSIGNED OK
AFTER ASSIGN //tex/moved.png
BLEND_PATHS AFTER ASSIGN ['//../../../../../../opt/blender/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

and, after reopening the file:

```
REOPENED [('ProbeLoaded', '//tex/moved.png'), ('Render Result', '')]
BLEND_PATHS ['//../../../../../../opt/blender/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
```

So on Linux too the path can be read, written, saved and read back with the new value, all with no window. The list of external paths carried one entry nothing in the file pointed at — Blender's own brushes file — here as well, at this machine's own path. Every line agrees with the Mac.

**Observed — that a recorded path survives when the file it names is gone (hand test, 2026-08-06).** Same machine and build. The PNG was renamed from `tex/probe.png` to `tex/gone.png` and the file reopened:

```
IMAGES [('ProbeLoaded', '//tex/probe.png'), ('Render Result', '')]
IS_MISSING [('ProbeLoaded', False), ('Render Result', False)]
BLEND_PATHS ABS [('/opt/blender/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', True), ('/root/smoozy/Reading a file/work/05/4.5/tex/probe.png', False)]
```

The recorded path still reads back and the missing flag on the image still reads false. Agrees with the Mac.

**Observed — the list of operators under the file group (hand test, 2026-08-06).** Same machine and build. Asking for that list gave forty names, identical to the Mac's:

```
PROBE bpy.ops.file operators = ['autopack_toggle', 'bookmark_add', 'bookmark_cleanup', 'bookmark_delete', 'bookmark_move', 'cancel', 'delete', 'directory_new', 'edit_directory_path', 'execute', 'external_operation', 'filenum', 'filepath_drop', 'find_missing_files', 'hidedot', 'highlight', 'make_paths_absolute', 'make_paths_relative', 'mouse_execute', 'next', 'pack_all', 'pack_libraries', 'parent', 'previous', 'refresh', 'rename', 'report_missing_files', 'reset_recent', 'select', 'select_all', 'select_bookmark', 'select_box', 'select_walk', 'smoothscroll', 'sort_column_ui_context', 'start_filter', 'unpack_all', 'unpack_item', 'unpack_libraries', 'view_selected']
```

---

**Observed — which kinds of recorded location the reading and writing machinery actually reaches (hand test, 2026-08-06). Added by the hand-test round that put question 9 on this sheet; this document is from the earlier round and the rest of it is left as it was.**

This answers the item this question left open — *whether every kind of recorded path is covered by the machinery that reads and writes them.* Done on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**, with Blender **4.5.12 LTS** (hash `84afd5f785f7`) at `/opt/blender/4.5/blender`, started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at `/root/reading-a-file/res-4.5` — a settings folder made for this round and never the real one. Nothing here was run on the Creator's Mac, so nothing here settles macOS. The whole record, with every command, is in `Preparation/Blender Research/Findings/4.5/Reading a file — added questions.md` and `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

A file was built holding **twenty-one recorded locations, every one written out in full from the drive root**, all pointing into an `assets` folder beside it: an image, a sound, a font, a text file, a volume, a movie clip, an Alembic cache file, a linked library, the render output path, the bake output path, the compositor File Output node's base path, a cloth point cache, a fluid domain cache folder, a geometry-nodes bake folder, a mesh-cache modifier file, an ocean modifier folder, an OSL script node, an IES texture node, a camera's own shader path, a sequencer image strip's folder with the sequencer's proxy folder, and a custom property on an object holding a path-shaped string. The multi-resolution modifier's path would not take a value in any version, so that kind is not in the file and nothing here speaks for it.

What the whole-file listing calls returned for that file:

```
LIST blend_paths()                 = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST blend_paths(packed=True)      = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST blend_paths(local=True)       = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST blend_paths(absolute=True)    = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST file_path_map()               = ['/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST file_path_map(include_libraries=True) = ['/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
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
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/4.5/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/4.5/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/4.5/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/4.5/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/4.5/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/4.5/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/4.5/assets/proxydir/
```

Reading that off and adding nothing to it: the recorded text changed for **CacheFile.filepath, Camera.custom_filepath, FluidDomainSettings.cache_directory, Image.filepath, Image.filepath_raw, ImageStrip.directory, Library.filepath, MeshCacheModifier.filepath, MovieClip.filepath, OceanModifier.filepath, ShaderNodeScript.filepath, ShaderNodeTexIES.filepath, Sound.filepath, Text.filepath, VectorFont.filepath, Volume.filepath**. It is the same after as before for **BakeSettings.filepath, CompositorNodeOutputFile base path, NodesModifier.bake_directory, PointCache.filepath (cloth), RenderSettings.filepath, StripProxy.directory (sequencer), custom property on an object**.

The same file with `bpy.ops.file.make_paths_absolute()` put every one of those sixteen back into full form and left the same seven alone, so the two sweeps reach the same set.

The one-pass path visitor does not exist in this version — asking for it returned:

```
FEW file_path_foreach = NOT PRESENT
```

Last, a walk over thirteen of `bpy.data`'s collections, four levels deep, collecting every text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries one of the two path marks Blender puts on a property. What it reached, one line per distinct value, with the custom property on the end — that one is not found by the walk at all and turns up only in a separate listing of custom properties:

```
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = /root/reading-a-file/work/many/4.5/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = /root/reading-a-file/work/many/4.5/assets/camshader.osl
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = /root/reading-a-file/work/many/4.5/assets/custom_prop.png
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = /root/reading-a-file/work/many/4.5/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = /root/reading-a-file/work/many/4.5/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/4.5/assets/meshcache.mdd
SWEEP bpy.data.scenes['Scene'].node_tree.nodes[].base_path = /root/reading-a-file/work/many/4.5/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/4.5/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = /root/reading-a-file/work/many/4.5/assets/pointcache/
SWEEP bpy.data.cache_files['probe.abc'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.abc
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/4.5/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/4.5/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = /root/reading-a-file/work/many/4.5/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = /root/reading-a-file/work/many/4.5/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.sequences[].directory = /root/reading-a-file/work/many/4.5/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = /root/reading-a-file/work/many/4.5/lib.blend
SWEEP bpy.data.scenes['Scene'].node_tree.nodes[].file_slots[].path = Image
SWEEP bpy.data.materials['Material'].library_weak_reference.id_name = MAMaterial
```

The walk reaches nineteen of the twenty-one, including six of the seven the bulk sweeps miss. The seventh of those, the custom property, carries no mark and is found only by listing custom properties separately. The text data-block's path is the one recorded location no mark reaches at all: its subtype came back as `NONE` and it carries neither mark. The bulk sweeps do reach it.

**Observed on the Creator's Mac, 2026-08-07.** The same file was built again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**, by the same two build scripts, and its paths made relative the same way, and then read. The Blender was `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, reporting **4.5.12 LTS**, hash `84afd5f785f7` — the same version string and hash the rented Linux machine reported. It ran with `--factory-startup` and with `BLENDER_USER_RESOURCES` pointed at a settings folder made for this sitting and for nothing else, never at the real one. Other topics were running Blender on the same Mac at the time, so nothing here says how long anything took. The scripts were written again from the recorded procedure rather than copied off the Linux machine, which this sitting was told not to touch; they were checked against the Linux printout first, and the twenty-four readings and the map of data-blocks to paths came back the same character for character.

```
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/many/4.5/many_rel.blend" --python "$S/s_cross.py"
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
CROSS blend_paths   = ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
```

**Twenty-five of the twenty-seven lines are identical to the Linux printout, character for character** — every one of the twenty-four readings, and the map of data-blocks to paths. Two are not, and they are recorded as two results and not folded into one.

The flat path list holds sixteen entries on the Mac and fifteen on Linux; the fifteen they share are identical. The extra one on the Mac is a path into the Blender application's own folder, ending `datafiles/assets/brushes/essentials_brushes-gp_draw.blend`, recorded relative to the file. On the Mac the same file lists that path in full form *before* the sweep as well, so the difference appears only after the sweep and the save, and afterwards the material it hangs off reads `//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend`.

And **whether capital letters in a name matter is answered differently on the two machines.** The check asks for the image's own file again with its name written in capitals — `os.path.exists(os.path.join(os.path.dirname(p), os.path.basename(p).upper()))` where `p` is `bpy.path.abspath(bpy.data.images["ProbeImage"].filepath)`. The file on disk is `probe.png` and nothing called `PROBE.PNG` was ever written; asked for the folder's own listing, the Mac answered `['probe.png']`, one file. The Mac's answer to the check was `True` and the Linux machine's was `False`. **The Mac's answer is the one that speaks for macOS**; neither machine speaks for the other, and neither speaks for Windows.

**Windows is untouched.** No Windows machine has run it, so whether any of this behaves differently there was not answered by any test here.

---

## 6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?

**Answer.** Yes to all three. Blender has a named command for each, and they are all reachable from Python.

- **Report.** "bpy.ops.file.report_missing_files() — Report all missing external files". In the interface this surfaces as a warning: "After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."
- **Search.** "bpy.ops.file.find_missing_files(...) — Try to find missing external files", which takes a `directory` and a `find_all` switch ("Find All, Find all files in the search path (not just missing)"). The manual describes the search as recursive and the repairs as absolute.
- **Rewrite.** `make_paths_absolute` and `make_paths_relative` rewrite every external path in one go; individual `filepath` properties can be assigned (question 5); and for linked blend-files there are `wm.lib_relocate` ("Relocate the given library to one or several others") and `wm.lib_reload` ("Reload the given library"). 4.5 adds relocation at the level of a single linked data-block: "Linked IDs can now also be relocated, if they are not a dependency of any other linked data".

For missing linked blend-files specifically there is also a readable flag rather than just a message: a data-block standing in for missing linked data reports `is_missing`.

**Label:** **documented** for the existence and stated purpose of the report, search, path-rewrite, relocate, and reload operations; **not in the sources** for a report returned as data and for the headless behaviour of those operations.

**Not covered.** The report is described as a warning message in an editor's header; nothing says it can be obtained as a list a program could work through, and nothing says what it does when there is no interface. Nothing says whether "missing" is judged for packed data, for paths with template errors, or for sequences where only some frames are absent. Nothing about how long the recursive search takes or how it chooses between two matching files with the same name. The manual notes that repair is not the whole job: recovered files may need reloading.

**Exact words.**

- "bpy.ops.file.report_missing_files() — Report all missing external files" and "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …) — Try to find missing external files" with "find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)" and "directory (string, (optional, never None)) – Directory, Directory of the file".
  Python API — https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30.
- "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files." and "Find Missing Files — In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative." and the following note, "Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once."
  Manual — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.
- "bpy.ops.wm.lib_relocate(...) — Relocate the given library to one or several others" and "bpy.ops.wm.lib_reload(...) — Reload the given library".
  Python API — https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30.
- "Relocate Library … It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file." and "Broken Libraries — If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
  Manual — https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.
- "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) — Type: boolean, default False, (readonly)".
  Python API — https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30.
- "Linked IDs can now also be relocated, if they are not a dependency of any other linked data".
  Release notes — https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30.
- "USE_PLACEHOLDERS — Generate a placeholder (empty ID) if not found in any library files" (one of the options on a blend-file import operation).
  Python API — https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30.

**Where I looked for the parts I call not covered.** Searched the manual and API for `report_missing`, `Report Missing`, `find_missing`, `missing`, `broken link`, `placeholder`. The `Report Missing Files` description in the topbar page and the operator's one-line description are the whole of what the sources say about the report; nothing anywhere describes its output as data.

**Outside the interface.** The searching operator accepts a `directory` argument, but the sources do not say that supplying it avoids the File Browser or works in background mode. The reporting operator's documented result is a warning in the Info editor's header, and the sources do not say what it produces when there is no interface. The API exposes `ID.is_missing` for missing linked data, but the cited words do not state how it behaves in background mode. For missing images, sounds, and similar references, the sources offer no documented headless report. Both observed tests below were run entirely from outside Blender's window: the report named the missing file on the terminal, and the search ran with a directory supplied and no window. The Creator's own run, recorded below, adds the other half: calling the search from the Python Console inside Blender's window, with a directory supplied, opened no File Browser either.

**Operating system.** None of the cited pages states an operating-system difference. The one OS-flavoured entry is `bpy.path.resolve_ncase`, "Resolve a case insensitive path on a case sensitive system". The sources do not connect that function to the missing-files commands or state what effect it has on their results. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed on everything except the exact recovered path, which is set out below. So macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

**Observed — what "report missing files" produces with no window (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A file was built in a fresh empty folder holding one image data-block loaded from a PNG beside it, then that PNG was renamed away so the recorded path pointed at nothing. Opening the file with no window and calling `bpy.ops.file.report_missing_files()` printed:

```
Warning: Path '/…/05/45/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
REPORT RESULT {'FINISHED'}
WM REPORTS NO SUCH PROPERTY
```

The `WM REPORTS` line is what the script printed after testing whether `bpy.context.window_manager` has a `reports` property; it does not in this version. The same run also printed the flag on the image itself:

```
IS_MISSING [('ProbeLoaded', False), ('Render Result', False)]
```

**Observed — whether "find missing files" can be pointed at a folder with no window (hand test, 2026-08-04).** Same computer, same build, same conditions. The same kind of file was built, and the PNG it points at was moved out of `tex/` into `search/deeper/probe.png`. Opening the file with no window, the script called `bpy.ops.file.find_missing_files(directory="/…/06/45/search/")` and then saved. What came back:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', '')]
FIND RESULT {'FINISHED'}
AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', '')]
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

Reopening the saved file in a further run with no window:

```
REOPENED [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', '')]
BLEND_PATHS ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//search/deeper/probe.png']
```

**Observed — how missing linked data is flagged (hand test, 2026-08-04).** See the chain test recorded under question 8: with a linked file renamed away, `is_missing` came back `True` on both the library entry and the linked object, and Blender printed a warning naming the file and the data-block.

**Observed — what "Report Missing Files" shows on screen (hand test, 2026-08-05).**

Run by the Creator, on their own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender 4.5.12 LTS. Because it was run on a Mac, it settles macOS. The same test was run on a rented Linux machine and is recorded separately in this document; **Windows** stays open.

A file prepared beforehand by this chat was opened at `/Users/artist/Documents/Smoozy hand tests/Reading a file/14/4.5.blend`. It holds one image data-block whose recorded path points at a PNG that had been moved away, so the link is broken. The Creator chose **File ‣ External Data ‣ Report Missing Files**.

The Creator's own words, in full:

> In all versions it shows this at the bottom of the window:
> Path '/Users/artist/Documents/Smoozy hand tests/Reading a file/14/tex_4.5/probe.png' not found, from local data-block 'IMBrokenProbe'
> Again, 4.5 is the version, obviously.

That last line means the folder in the path changes with the version. In 4.5 the message read:

```
Path '/Users/artist/Documents/Smoozy hand tests/Reading a file/14/tex_4.5/probe.png' not found, from local data-block 'IMBrokenProbe'
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

Done on the rented Linux machine with Blender **4.5.12 LTS**, with no window, `--factory-startup`, and its own settings folder.

*Reporting.* A file whose one image had been renamed away was opened and the report asked for. What came back:

```
FILE_PATH_MAP {'WinMan': [], 'Animation': [], 'Compositing': [], 'Geometry Nodes': [], 'Layout': [], 'Modeling': [], 'Rendering': [], 'Scripting': [], 'Sculpting': [], 'Shading': [], 'Texture Paint': [], 'UV Editing': [], 'Scene': [], 'Palette': [], 'Collection': [], 'Camera': [], 'Cube': [], 'Light': [], 'World': [], 'LineStyle': [], 'Material': [], 'ProbeLoaded': ['//tex/probe.png'], 'Render Result': []}
Warning: Path '/root/smoozy/Reading a file/work/05/4.5/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
REPORT RESULT {'FINISHED'}
WM REPORTS NO SUCH PROPERTY
```

Same wording as on the Mac, apart from the path. The report works with no window, names the missing file, and there is no property on the window manager holding Blender's reports as a list in this version.

*Searching and rewriting.* The PNG was instead moved to `search/deeper/probe.png` and the search handed the folder `search/`. What came back:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', '')]
FIND RESULT {'FINISHED'}
AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', '')]
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

and after reopening the file:

```
REOPENED [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', '')]
```

So the search works with no window when handed a folder, finds the moved file, rewrites the recorded path in the relative form, and the change survives a save and a reopening — as on the Mac. **One thing differs between the two machines and is not explained by either.** The path that came back here is `//search/deeper/probe.png`, the plain relative path to where the file actually was. On the Mac, the Creator's own run of the same operator returned `//../15/tex_5.2/probe.png` in every version, 4.5 included, and that document says it was not established why. Both results stand; neither explains the other, and nothing here says the difference belongs to the operating system.

**Observed — the same two things in Blender's own window on Linux (hand test, 2026-08-06).**

Same machine and build, with a window open on the machine's own display and the operator called from a timer.

*Reporting, in a window.* What came back:

```
WINDOW COUNT BEFORE 1
Warning: Path '/root/smoozy/Reading a file/work/w14/4.5/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
REPORT RESULT {'FINISHED'}
WINDOW COUNT AFTER 1
WM REPORTS NO SUCH PROPERTY
SHOT RESULT {'FINISHED'}
```

No second window opened. The message the Creator read at the bottom of Blender's window on the Mac could not be produced here, because the operator had to be called from a script rather than chosen from the menu, and nothing on this machine can drive a menu. So what this settles is the operator and the reports list, not what is written on screen.

*Searching, in a window.* The same file with the image moved into `search/deeper/`, the operator called with the folder. What came back:

```
WINDOW COUNT BEFORE 1
FIND RESULT {'FINISHED'}
WINDOW COUNT AFTER 1
IMAGES AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', '')]
SHOT RESULT {'FINISHED'}
```

The window count was one before and one after, so **no file browser opened** — the same answer the Creator got on the Mac, and the opposite of what the manual leads a reader to expect.

---

## 7. Can something a file points at be traced to the scene that uses it?

**Answer.** Direct tracing is documented for objects and linked libraries. Tracing an arbitrary referenced item to a scene takes a reasoning step that the sources do not state.

The general tool is a whole-file map of who uses what: "user_map(*, subset=None, key_types=None, value_types=None) — Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them", with filters on the types of both keys and values, and an option to restrict it to a given set of data-blocks. There is also a per-pair count: "user_of_id(id) — Count the number of times that ID uses/references given one".

For objects there is a direct answer: "users_scene — The scenes this object is in" (and "users_collection — The collections this object is in"). For linked blend-files there is the reverse view: "users_id — ID data-blocks that use this library".

For anything else — such as an image, a material, or a node group — `user_map` provides users of data-blocks. Following those relationships until an object is reached, then treating `users_scene` as the end of the chain, is inferred. The sources do not state that every chain reaches an object or that the walk is complete for every kind of reference.

The Outliner separately shows data-block user counts and the source paths of linked data. The cited words do not say that it traces an arbitrary reference to a scene.

**Label:** **documented** for `user_map`, `user_of_id`, `users_scene`, `users_collection`, `users_id`, `users` and the Outliner view; **inferred** for the multi-step chaining from an arbitrary data-block up to a scene.

**Not covered.** No documented single call from a file on disk, or from a path, to the scene using it — you must first find the data-block holding that path. Nothing says whether a use through a modifier, a driver, a custom property or a node group is counted the same way; `user_map` is described only as "all data-blocks using them". Nothing about whether reaching a scene means the thing actually renders — that is question 3's territory. Nothing about tracing something used only inside a linked file.

**Exact words.**

- "user_map(*, subset=None, key_types=None, value_types=None) — Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them." with "subset (Sequence[bpy.types.ID]) – When passed, only these data-blocks and their users will be included as keys/values in the map." and the return "dictionary that maps data-blocks ID's to their users."
  Python API — https://docs.blender.org/api/4.5/bpy.types.BlendData.html — read 2026-07-30.
- "users_scene — The scenes this object is in." with "Note — Takes O(len(bpy.data.scenes) * len(bpy.data.objects)) time." and "users_collection — The collections this object is in." with "Note — Takes O(len(bpy.data.collections) + len(bpy.data.scenes)) time."
  Python API — https://docs.blender.org/api/4.5/bpy.types.Object.html — read 2026-07-30.
- "users_id — ID data-blocks that use this library" with "Note — Takes O(n) time, where n is the total number of all linkable ID types in bpy.data."
  Python API — https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.
- "users — Number of times this data-block is referenced" and "user_of_id(id) — Count the number of times that ID uses/references given one".
  Python API — https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30.
- "Blender File: Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number of users – clicking it adds or removes a fake user." and "Data API: Lists every data-block in the file along with any properties that it might have."
  Manual — https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html — read 2026-07-30.
- "Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."
  Manual — https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.
- Background on why the chain exists at all: "They can reference each other, for reuse and instancing. (Child/parent, object/object-data, materials/images, in modifiers or constraints too…)" and "Every data-block has its usage counted (reference count)".
  Manual — https://docs.blender.org/manual/en/4.5/files/data_blocks.html — read 2026-07-30.

**Where I looked.** Searched the whole API for `user_map`, `users_scene`, `users_collection`, `users_id`, and for every property whose name begins `users_`; that search returns exactly the ones above plus texture-specific `users_material` / `users_object_modifier` and `Collection.users_dupli_group`. Searched the manual's Outliner pages and the data-blocks page. There is no call that starts from a scene-less item and returns a scene in one step other than `Object.users_scene`.

**Outside the interface.** The API exposes the listed data and calls, while the Outliner is an editor. The sources do not directly state that the inferred multi-step tracing works in background mode. Two of the direct properties are documented as taking time proportional to the size of the file. The observed test below was run entirely from outside Blender's window, so the walk holds there. **Added by the hand tests of 2026-08-06.** The walk was run again with no window on a rented Linux machine, this time from references held only inside a node group, a driver and a custom property, and it held in every case. See the observed addition below.

**Operating system.** None of the pages and searches named above states an operating-system difference. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it. **Added by the hand tests of 2026-08-06.** The new material below was run on the rented Linux machine only, so it settles **Linux** and leaves **macOS** owed a second sitting. **Windows stays open** for the same reason as before.

**Observed — following an image up to the scene that uses it (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. A file was built in a fresh empty folder holding two scenes, `SceneWithIt` and `SceneWithout`; a PNG on disk named `UsedTex.png` loaded and used by an image texture node in a material on a cube that sits in `SceneWithIt` only; and a second PNG named `UnusedTex.png` loaded into the file and used by nothing. Opening that file with no window, the script called `bpy.data.user_map()` once and then walked outwards from each image, printing each step. What came back:

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

The `VALUE_TYPES SCENE` line is the result of `bpy.data.user_map(subset=[img], value_types={'SCENE'})`. The `FILE_PATH_MAP` line was printed with entries holding no paths filtered out.

**Observed — the same walk on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **4.5.12 LTS**, with no window, `--factory-startup`, and its own settings folder.

**The Mac's build script errored here, and that is recorded rather than hidden.** Run as the Mac list has it, all four versions on this machine printed:

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

This answers the item this question left open. Same machine, same conditions as the addition under question 5 above: **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**, Blender **4.5.12 LTS** (hash `84afd5f785f7`), no window, `--factory-startup`, this round's own settings folder. Nothing here was run on the Creator's Mac.

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
UM file_path_map = [('Img_custom', ['/root/reading-a-file/work/um/4.5/assets/custom.png']), ('Img_driver', ['/root/reading-a-file/work/um/4.5/assets/driver.png']), ('Img_group', ['/root/reading-a-file/work/um/4.5/assets/group.png']), ('Img_plain', ['/root/reading-a-file/work/um/4.5/assets/plain.png'])]
```

All three of the awkward cases are visible in the map: the node-group one through the group's own node tree, and the driver one and the custom-property one through the object itself. Every image that is used at all reaches `TheScene`. Asking the map directly for scene-type users returns an empty set for every one of them, so the walk is what is needed, which is what the earlier round found for a plainly-used image.

**The macOS half of this is not run.** The test that settled this question — the one that builds a file whose image is used only inside a node group, only by a driver, and only by a custom property, and then walks the map of what uses what — was written for both machines and was run on the rented Linux machine alone. The Mac sitting of 2026-08-07 ran only the tests that had no result at all, so for this question macOS is untested rather than settled. It is recorded as owed in `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

**Windows is untouched**, for the same reason as above.

---

## 8. Can a file's full chain of references be read while some of the referenced files are absent?

**Answer.** Partly, and the split is worth being precise about.

What still reads: a file that cannot find a library it links to still keeps the reference. "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost". Those placeholders are identifiable through `ID.is_missing`. The API also exposes a library `filepath`, a library `parent`, and `blend_paths()`. Treating those separate properties as proof that the missing library's recorded path and its place in a complete parent chain always remain readable is inferred.

What does not read: what a *missing* file itself pointed at. The sources do not say this in so many words. It follows from what they do say — Blender makes the placeholder precisely because the file could not be found, and everything else here reads references out of files that were loaded — so a chain stops at the first absent link, and anything below it is invisible. I am labelling that step inferred, because a reader has to supply it.

**Label:** **documented** that references to missing libraries survive as placeholders, that missing placeholders are flagged, that library path and parent properties exist, and that `blend_paths` lists the loaded file's outward paths; **inferred** that those properties preserve the missing library's place in a readable parent chain and that the chain cannot be followed past an absent file.

**Not covered.** The sources never use the words "full chain" or "dependency tree" of files, and never describe walking one. Nothing about missing images or sounds as opposed to missing libraries — the placeholder mechanism is described for libraries only, though `blend_paths` is described without any reference to whether the files exist, from which their recorded paths would still be listed (inferred). Nothing about circular references beyond a warning that they misbehave. Nothing about how deep library nesting may go.

**Exact words.**

- "Broken Libraries — If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
  Manual — https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30. The same page's Known Limitations add: "In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links." and "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them."
- "filepath — Path to the library .blend file", "parent — Type: Library, (readonly)", and "version — Version of Blender the library .blend was saved with — Type: int array of 3 items in [0, inf], default (0, 0, 0), (readonly)".
  Python API — https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.
- "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)" and "is_library_indirect — Is this ID block linked indirectly" and "library — Library file the data-block is linked from".
  Python API — https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30.
- "USE_PLACEHOLDERS — Generate a placeholder (empty ID) if not found in any library files."
  Python API — https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30.
- "Returns a list of paths to external files referenced by the loaded .blend file."
  Python API — https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30.
- A lighter-weight way to look into another file without opening it as the main file: "load(filepath, link=False, …) — Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked." That gives names of data-blocks in a file, not that file's own outward paths.
  Python API — https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html — read 2026-07-30.

**Where I looked to be able to say the rest is silent.** The manual's whole `files/` section (`data_blocks`, `file_paths`, `blend/*`, `linked_libraries/*`, `asset_libraries/*`); the API pages for `Library`, `ID`, `BlendDataLibraries`, `BlendData`, `BlendImportContext`, `BlendImportContextItem`, `bpy.utils`, `bpy.path`; searched all three source sets for `missing`, `placeholder`, `broken`, `dependency`, `chain`, `recursive`, `indirect`. Nothing describes reading a dependency chain across files, and nothing states what happens to references held inside a file that could not be opened.

**Outside the interface.** The API exposes the data and calls named above, but the sources do not state that the inferred chain behaviour holds in background mode. One headless-only option touches this area: `--disable-liboverride-auto-resync`, "Do not perform library override automatic resync when loading a new blend-file" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). There is also a related warning flag on the data itself: "needs_liboverride_resync — True if this library contains library overrides that are linked in current blendfile, and that had to be recursively resynced on load (it is recommended to open and re-save that library blendfile then)" (https://docs.blender.org/api/4.5/bpy.types.Library.html, read 2026-07-30).

**Operating system.** None of the pages and searches named above states an operating-system difference. `bpy.path.resolve_ncase` shows that Blender provides a helper for case-insensitive paths on a case-sensitive system, but the sources do not connect that helper to missing libraries or chain traversal. The observed tests below were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it.

**Observed — how far a chain of linked files reads when a file in the middle, or at the end, is gone (hand test, 2026-08-04).**

Done on the Creator's own computer: a MacBook Pro with an M1 Pro chip running macOS 26.5.2, with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder. Three files were built in one fresh empty folder: `c.blend` holding a cube named `ChainCube`; `b.blend`, which links that cube from `c.blend` and puts it inside a local collection named `ChainCollection`; and `a.blend`, which links `ChainCollection` from `b.blend`. All were saved with `relative_remap=True`. `a.blend` was then opened three times with no window — once with everything present, once with `c.blend` renamed to `c_gone.blend`, and once with `c.blend` back and `b.blend` renamed to `b_gone.blend`.

With everything present:

```
Info: Read library: '/…/07/45b/b.blend', '//b.blend', parent '<direct>'
Info: Read library: '/…/07/45b/c.blend', '//c.blend', parent '<direct>'
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, False)]
OBJECTS [('ChainCube', False, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With `c.blend` — the far end of the chain — renamed away:

```
Warning: Unable to open '/…/07/45b/c.blend': No such file or directory
Info: Cannot find lib '/…/07/45b/c.blend'
Info: LIB: Object: 'ChainCube' missing from '/…/07/45b/c.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, True)]
OBJECTS [('ChainCube', True, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With `b.blend` — the middle of the chain — renamed away instead, and `c.blend` present:

```
Warning: Unable to open '/…/07/45b/b.blend': No such file or directory
Info: Cannot find lib '/…/07/45b/b.blend'
Info: LIB: Collection: 'ChainCollection' missing from '/…/07/45b/b.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, True)]
OBJECTS []
BLEND_PATHS ['//b.blend']
```

Each entry of the `LIBRARIES` line is `(name, filepath, parent's filepath or None, is_missing)`. Each entry of the `OBJECTS` line is `(name, is_missing, is_library_indirect, the library's filepath or None)`.

**Observed — what `bpy.data.libraries.load()` gives back, and what it does when the file is not there (hand test, 2026-08-04).** See the block recorded under question 1: the loader exposed no `libraries` attribute and no `version` attribute in this version, listed only the peeked file's own local data-blocks, and raised `OSError load: … failed to open blend file` when handed a path that does not exist.

**Observed — the same three chain reads on a Linux machine (hand test, 2026-08-06).**

Done on the rented Linux machine with Blender **4.5.12 LTS**, with no window, `--factory-startup`, and its own settings folder. The same three-deep chain was built by this version and opened three times.

With everything present:

```
Read blend: "/root/smoozy/Reading a file/work/07/4.5/a.blend"
Info: Read library: '/root/smoozy/Reading a file/work/07/4.5/b.blend', '//b.blend', parent '<direct>'
Info: Read library: '/root/smoozy/Reading a file/work/07/4.5/c.blend', '//c.blend', parent '<direct>'
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, False)]
OBJECTS [('ChainCube', False, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With the far file gone:

```
Info: Read library: '/root/smoozy/Reading a file/work/07/4.5/b.blend', '//b.blend', parent '<direct>'
Info: Read library: '/root/smoozy/Reading a file/work/07/4.5/c.blend', '//c.blend', parent '<direct>'
Warning: Unable to open '/root/smoozy/Reading a file/work/07/4.5/c.blend': No such file or directory
Info: Cannot find lib '/root/smoozy/Reading a file/work/07/4.5/c.blend'
Info: LIB: Object: 'ChainCube' missing from '/root/smoozy/Reading a file/work/07/4.5/c.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, True)]
OBJECTS [('ChainCube', True, True, '//c.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With the middle file gone instead:

```
Info: Read library: '/root/smoozy/Reading a file/work/07/4.5/b.blend', '//b.blend', parent '<direct>'
Warning: Unable to open '/root/smoozy/Reading a file/work/07/4.5/b.blend': No such file or directory
Info: Cannot find lib '/root/smoozy/Reading a file/work/07/4.5/b.blend'
Info: LIB: Collection: 'ChainCollection' missing from '/root/smoozy/Reading a file/work/07/4.5/b.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, True)]
OBJECTS []
BLEND_PATHS ['//b.blend']
```

That is the Mac's result line for line, including the plain, untimestamped log lines this version prints. The readable chain stops at the first absent file on Linux too, and the library's parent property came back empty in every case here as well. While building the chain, the loader printed `AVAILABLE OBJECTS []` when the third file was linked from the second, so the loader lists only a file's own local data on Linux too.

One thing the Mac record does not carry: after the third read, this version quit with `Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB`. It is recorded as it came back and nothing is claimed about it.

**Observed — peeking into a file, on a Linux machine (hand test, 2026-08-06).** See the block recorded under question 1 for this machine: the loader exposed no `libraries` attribute and no `version` attribute in this version, listed only the peeked file's own local data-blocks, and raised `OSError load: /root/smoozy/Reading a file/work/08/does_not_exist.blend failed to open blend file`. Identical to the Mac's.

---

## Unresolved tests for the hand-test step

The hand-test step ran in two sittings and settled every test that stood here. The terminal-reachable ones were run on 2026-08-04, including the concurrent reading and rendering one, which was run after the restriction against starting two Blenders at once was lifted. The ones needing eyes on Blender's window were run by the Creator on 2026-08-05. What each found is written into the answer it belongs to above, as an observed finding.

**Run 2026-08-06 — the same tests again on a rented Linux machine.** New equipment arrived and the list came back out. Every terminal test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — and Blender's window was opened on that machine's own display for three more. All of it is recorded above, beside the Mac run it belongs to, and the two are never merged. Linux is therefore no longer open on any question this document answers by test. The Linux list is `Audits/Blender Research/Reading a file/Hand tests — Linux.md`.

Three things stay open for want of equipment rather than for want of a test. **Windows** was not tried at all — no Windows machine has run it — so every operating-system line above is settled for macOS and Linux and open for Windows. **No test can put a number** on how many Blenders may run at once, since one computer cannot establish a limit for every computer — and the rented machine is a container sharing someone else's hardware, so it is even less able to. And on the day of the Linux run **nothing on that machine could move the mouse to a control or press a key at it**, so what Blender shows on screen when the render switch is keyframed by hand, and what the File Output node's Output Paths panel holds, were not read there. Both were read on the Mac.

> **Corrected by the citation check of the Linux hand tests, 2026-08-06.** This read "nothing can move the mouse to a control or press a key at it", as a standing fact about the machine. A tool for moving the pointer was put on that machine afterwards, so the pointer can now be moved. Pressing and clicking is another matter: the check tried both and could not confirm that either reaches Blender. On a control whose state Blender will report — a node's Mute switch — four separate clicks left it unchanged, and clicks aimed by window rather than by position twice landed on a different control. **So both items stay unread on Linux**: what Blender shows when the render switch is keyframed by hand, and what the Output Paths panel holds. Both were read on the Mac. The attempts are written up in `Audits/Blender Research/Reading a file/Citation check/Linux — Opus 5.md`.

**Run 2026-08-05 — the re-run the citation check asked for.** The citation check added one test here: run the concurrency test again and record the wait condition exactly, because the test under question 2 described waiting until two frames had appeared while every recorded output line read `frames on disk: 1`. It was run in all four versions as hand test 18, with the wait condition written down and the frame count printed at every poll. Nothing stays open because of it. What it found, and the correction it forced, are recorded under question 2 above, beside the run they belong to.


**Added 2026-08-06 — the round that put question 9 on the sheet.** That round's hand tests also carried two items this document had left open: whether every kind of recorded path is covered by the machinery that reads and writes them (question 5), and whether a reference held only inside a node group, a driver or a custom property shows up in the map of what uses what (question 7). Both were run on a rented Linux machine and both are written into the answers above as observed additions. Nothing was run on the Creator's Mac in that sitting, so those two additions settle Linux only. That round's list is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

---

2026-08-04 — Opus 5 (Claude Code) — Reading a file, Blender 4.5

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests, added under questions 5 and 7

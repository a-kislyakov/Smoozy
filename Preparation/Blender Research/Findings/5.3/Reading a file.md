# Reading a file — findings for Blender 5.3

- Sources consulted: Blender 5.3 manual (https://docs.blender.org/manual/en/5.3/), Blender 5.3 Python API documentation (https://docs.blender.org/api/5.3/), and Blender 5.3 release notes (https://developer.blender.org/docs/release_notes/5.3/). All three were taken on 2026-10-11. Cycles' own files for 5.3 were taken on 2026-10-16. Dates read below follow those dates.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Disclosure, for completeness. I opened nothing else. But the tool I run in showed me some text on its own when the session started: the project's `CLAUDE.md` (a short signpost telling agents to read a planning file, and telling Blender researchers to ignore it), a one-line-per-entry index of the user's saved notes, and a list of changed file names in the project. I did not open any of these and did not follow the signpost. None of that text says anything about how Blender behaves on the topics below. Inside `Blender Sources/` I also read `About this folder.md`, which is a description of the folder; I did not follow its link out of the folder.

Nothing was run for this research. Every finding is from reading. Labels "observed" below mean "read directly in Cycles' own files", as the brief defines it.

---

## 1. What does a `.blend` file give up without a Blender to open it, and what needs one?

**Reading of the question.** This is ambiguous. Reading A: what can be got from a `.blend` file when no Blender program is used at all. Reading B: what Blender can get from a `.blend` file without opening it as the current file. I answer both. Reading A seems to be the main one.

**Label:** documented for each item quoted below. **Inferred** that anything beyond preview and decompression needs Blender, and that Python calls run outside the interface. **Not in the sources** for a full account of what a program other than Blender can read from the file. The sources never describe the inside of a `.blend` file.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Reading A — no Blender program used.**

The sources name three things.

1. A preview picture. Blender stores a small preview in the file. A tool that Blender installs lets the operating system's file browser show it.
   - "Blender saves by default a small preview of current scene in the blend-file. This will show in the Thumbnail view of the File Browser. During its installation, Blender also adds a small tool to your OS, that will allow your system file browser to show those previews as file thumbnails as well."
   - Manual, https://docs.blender.org/manual/en/5.3/files/blend/previews.html, read 2026-10-11.
   - Saving previews can be switched off: "You can disable writing any previews when saving a blend-file using the Save Preview Images setting". Same page.
2. Compression. A compressed `.blend` file can be decompressed by tools other than Blender.
   - "The used compression algorithm is Zstandard. It is not unique to Blender so files can be compressed/decompressed with external tools."
   - Manual, https://docs.blender.org/manual/en/5.3/files/blend/open_save.html, read 2026-10-11.
   - What the decompressed file then gives up is not stated.
3. Outside tooling exists. The manual says other tools can change `.blend` files, and names one of them.
   - "Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it’s possible to also create an external tool for doing such edits across blend-files."
   - Manual, https://docs.blender.org/manual/en/5.3/files/asset_libraries/introduction.html, read 2026-10-11.
   - A preference mentions the same tool by its short name: "Forces all linked data to be considered as directly linked. Workaround for current issues/limitations in BAT (Blender studio pipeline tool)". API, https://docs.blender.org/api/5.3/bpy.types.PreferencesExperimental.html (`use_all_linked_data_direct`), read 2026-10-11.
   - The sources do not say what this tool can read or write.

**What needs a Blender (reading A).** Every way of reading what is inside a file that the sources describe goes through Blender: the Outliner, Python's `bpy.data`, or Blender built as a Python module.
- "You can easily inspect the contents of your file by using the Outliner editor, which displays all of the data in your blend-file." Manual, https://docs.blender.org/manual/en/5.3/files/introduction.html, read 2026-10-11.
- `BlendData`: "Main data structure representing a .blend file and all its data-blocks". API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html, read 2026-10-11.
- Blender as a Python module is still Blender: "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b)". API, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.
- **Inferred:** the sources describe no way other than these to read scenes, settings or recorded paths. So, as far as the sources go, anything beyond the preview and decompression needs Blender. This is inferred from silence. The sources do not say it outright.

**Reading B — Blender reading a file it has not opened.**
- `bpy.data.libraries.load()` reads another file without making it the open file. It gives the names of the data-blocks in it, the other libraries it uses, and its version: "Attributes matching bpy.data, which are lists of strings representing linkable data-blocks. A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). … A version attribute, representing the version of the loaded library blend-file". API, https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html, read 2026-10-11.
- Its limit: "this API operates on reading and writing ID data-blocks and isn’t a complete substitute for loading .blend files". API, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.
- A file's preview can be loaded: `ImagePreviewCollection.load(name, filepath, file_type, …)` with `file_type` one of "'IMAGE', 'MOVIE', 'BLEND', 'FONT', 'OBJECT_IO'". API, https://docs.blender.org/api/5.3/bpy.utils.previews.html, read 2026-10-11.

**Not covered:** the layout of a `.blend` file; whether a program other than Blender can read settings, scenes or recorded paths; what Blender Asset Tracer can read; whether the preview tool needs Blender to be running.

**Where I looked (for the parts not in the sources):** searched the whole 5.3 manual, API documentation and release notes for: thumbnailer, file header, blend-file header, file format version, without opening, external tool, outside of Blender, SDNA, DNA, endian, file-block, blend-file structure, blend file format, Zstandard, zstd, gzip, Asset Tracer, BAT. Read the pages on previews, opening and saving, compatibility, the files introduction, asset library introduction, the System preferences (file association), and the API pages on Blender as a Python module, `BlendDataLibraries`, and `bpy.utils.previews`. No page describes the inside of the file.

**Outside the interface:** `libraries.load()` and `bpy.utils.previews` are Python. The sources show Python scripts running without a window ("blender myscene.blend --background --python myscript.py", API, https://docs.blender.org/api/5.3/info_tips_and_tricks.html). That they work there is **inferred**; the pages do not say so. The operating system preview tool works outside Blender's window by its nature (documented above).

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: saving with and without compression, `bpy.data.libraries.load()` on a present and an absent file, and Blender's own preview loader (`bpy.utils.previews`) all ran (tests 2, 3 and 4). A file saved with no window carried a 128 by 128 preview of the scene; one saved from Blender's window carried a 256 by 256 one; one saved with previews switched off carried none (test 3). An ordinary Zstandard tool unwrapped a compressed file with no Blender involved (test 2).

**Operating system:** the preview tool differs by system. macOS: "The Blender Thumbnail extension can be disabled in the System Settings under “Login Items & Extensions”, within the Quick Look category" (previews page above). Linux: "The thumbnailer is installed so blend-file thumbnails will be shown in file managers (For All Users only)", and "Make this installation your default Blender (MS-Windows & Linux only)" (Manual, https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11). The sources say nothing system-specific about decompression or reading.

**Brought up to date by the hand tests of 2026-10-17.** Tests 2 to 4 settle macOS on Apple silicon for what they cover. What the Finder shows was not reached: macOS's Quick Look, asked from the terminal, never answered for any file. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 2 — what the first bytes of a saved file hold, whether it is compressed, and whether an ordinary Zstandard tool can unwrap it.** With no window, the factory startup file was saved three times into an empty folder by `bpy.ops.wm.save_as_mainfile`: once with no compression argument (`default.blend`), once with `compress=False` (`plain.blend`), once with `compress=True` (`compressed.blend`). Before saving, the script printed the preference `bpy.context.preferences.filepaths.use_file_compression` and the default of the save command's own `compress` setting. What came back:

```
PREF use_file_compression True
DEFAULT compress arg False
Info: Saved as "default.blend"
Info: Saved as "plain.blend"
Info: Saved as "compressed.blend"
DONE
```

Each file's size from `stat -f %z`, and its first 32 bytes from `head -c 32 <file> | xxd`:

```
-- default 95760 bytes
00000000: 28b5 2ffd a041 0601 00b5 3700 4447 424c  (./..A....7.DGBL
00000010: 454e 4445 5231 372d 3031 7630 3530 3352  ENDER17-01v0503R
-- plain 571884 bytes
00000000: 424c 454e 4445 5231 372d 3031 7630 3530  BLENDER17-01v050
00000010: 3352 454e 4400 0000 0010 0000 0000 0000  3REND...........
-- compressed 95760 bytes
00000000: 28b5 2ffd a041 0601 00b5 3700 4447 424c  (./..A....7.DGBL
00000010: 454e 4445 5231 372d 3031 7630 3530 3352  ENDER17-01v0503R
```

Then, with no Blender involved, a copy of `compressed.blend` was handed to the Homebrew build of Zstandard (`/opt/homebrew/bin/zstd`, which reports itself `v1.5.7`) as `zstd -d -f zcheck.blend -o zcheck.out`, and the result compared with `plain.blend` by `cmp`. What came back:

```
…/zcheck.blend: 571884 bytes 
zstd exit 0
undone 571884 bytes
00000000: 424c 454e 4445 5231 372d 3031 7630 3530  BLENDER17-01v050
00000010: 3352 454e 4400 0000 0010 0000 0000 0000  3REND...........
…/zcheck.out …/plain.blend differ: char 65970, line 6
```

So the unwrapped file has the plain save's size and opening bytes, and differs from it at byte 65970.

**Test 3 — whether a file saved with no window carries a preview, against one saved from a window and one saved with previews switched off.** With no window, the factory startup file (one grey cube) was saved as `5.3-background.blend`; the preference `file_preview_type` was then set to `'NONE'` and the file saved again as `5.3-nopreview.blend`. Next, `5.3-background.blend` was opened in Blender's window (started from the terminal with `--no-window-focus -p 0 0 1280 800`), and three seconds later a timer saved it as `5.3-window.blend` and quit. Then, with no window, each file's preview was asked for through Blender's own preview loader — `bpy.utils.previews.new().load(name, path, 'BLEND')` — printing the preview's size, the number of colour values it holds, and how many of them are not zero. What came back:

```
PREF file_preview_type = AUTO
PREF use_save_preview_images = NO SUCH PROPERTY
Info: Saved as "5.3-background.blend"
SAVE background {'FINISHED'}
PREF after switching off: file_preview_type = NONE | use_save_preview_images = NO SUCH PROPERTY
Info: Saved as "5.3-nopreview.blend"
SAVE nopreview {'FINISHED'}
```
```
WINDOW background = False
Info: Saved as "5.3-window.blend"
WINDOW save {'FINISHED'}
00:03.726  blend            | Saved session recovery to "…/tmp/quit.blend"
```
```
PREVIEW 5.3-background size=(128, 128) values=65536 nonzero=7186
PREVIEW 5.3-window size=(256, 256) values=262144 nonzero=28262
PREVIEW 5.3-nopreview size=(0, 0) values=0 nonzero=0
```

The two previews that had size were then written out as pictures by the same loader and looked at: both show a grey cube on an empty background, 128 by 128 for the file saved with no window and 256 by 256 for the file saved from the window.

The same three files were also handed to macOS's own thumbnail route from the terminal, `qlmanage -t -s 256 -o <folder> <file>`. It never answered: the first call made no picture in about three minutes and was stopped by hand, and so was the second. Asked again with a twenty-second limit, it made no picture and was still running for `5.3-window.blend` and equally for a file saved by Blender 5.2.2 on the same Mac. So what the Finder would show was not reached from the terminal, for any file, and nothing here speaks for it. `pluginkit` named the thumbnail tool registered on this Mac as Blender 5.2.2's (`/Applications/Blender.app/Contents/PlugIns/blender-thumbnailer.appex`), not 5.3's.

**Test 4, first part — what one file gives up to another Blender without being opened.** Three files were built in an empty folder, all saved with `relative_remap=True`: `c.blend` holding a mesh object `ChainCube`; `b.blend`, which links `ChainCube` from `c.blend` and puts it in a local collection `ChainCollection`; and `a.blend`, which links `ChainCollection` from `b.blend`. While building, the loader printed what each source offered: `AVAILABLE OBJECTS in c ['ChainCube']`, then `AVAILABLE OBJECTS in b []` and `AVAILABLE COLLECTIONS in b ['ChainCollection']`. Then a fresh Blender with no window called `bpy.data.libraries.load()` on `b.blend` and printed what the source side offers, and called it again on a path where no file is. What came back:

```
SRC ATTRS ['actions', 'annotations', 'armatures', 'brushes', 'cache_files', 'cameras', 'collections', 'curves', 'fonts', 'grease_pencils', 'hair_curves', 'images', 'lattices', 'libraries', 'lightprobes', 'lights', 'linestyles', 'masks', 'materials', 'meshes', 'metaballs', 'movieclips', 'node_groups', 'objects', 'paint_curves', 'palettes', 'particles', 'pointclouds', 'scenes', 'screens', 'sounds', 'speakers', 'texts', 'textures', 'version', 'volumes', 'workspaces', 'worlds']
SRC OBJECTS []
SRC COLLECTIONS ['ChainCollection']
SRC LIBRARIES [bpy_lib_library(filepath='//c.blend', is_archive=False)]
SRC VERSION (5, 3, 29)
ABSENT ERROR OSError load: /…/e04/does_not_exist.blend failed to open blend file
```

The object `b.blend` links from `c.blend` is not listed among its objects; the collection it holds is, and so is the file it links. What happens when the same chain is opened with files missing is under question 8.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

**Label:** documented that more than one can run at once. **Inferred** that auto-save files are told apart per running Blender. **Not in the sources:** a number, and whether one reading a file while another renders causes trouble.

**Several can run.**
- "As with the application it’s possible to start multiple instances, each with their own bpy and therefore Blender state. Python provides the multiprocessing module to make communicating with sub-processes more convenient." API, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.
- "Nodes can be copied and pasted between Blender instances." Manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/editing.html, read 2026-10-11.
- Each one holds one file at a time: "Only a single .blend file can be edited at a time." API, info_advanced_blender_as_bpy page above.

**No number is given.** The sources speak only of resources.
- "Rendering takes all available CPU time; you should render overnight, when the computer is not needed, or set Blender to a low priority while rendering, and work on other things (be careful with the RAM space!)." Manual, https://docs.blender.org/manual/en/5.3/render/output/animation.html, read 2026-10-11.
- Threads can be capped: "-t, --threads <threads> Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.

**Shared places on disk.** These are facts the sources give. What they mean for two Blenders running together is not stated.
- The temporary directory: "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)." Manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.
- Auto-save: "Automatic saving of temporary files in temp directory, uses process ID." API, https://docs.blender.org/api/5.3/bpy.types.PreferencesFilePaths.html (`use_auto_save_temporary_files`), read 2026-10-11. **Inferred:** auto-save files are told apart per running Blender. The sources do not say this about any other temporary file.

**Several Blenders writing to one output folder** is documented, but for separate computers, not one:
- "An easy way to get multiple machines to share the rendering workload is to: Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11.

**Reading while another renders: not in the sources.** Nothing says whether one Blender opening or reading a `.blend` file disturbs another Blender rendering the same or another file, whether files are locked, or whether the temporary directory can clash.

**Not tested for 5.3, and why.** The earlier versions' test of this — a second Blender reading the file while a first renders it, and a third saving over it — needs two Blenders running at once. The hand-test sitting of 2026-10-17 was allowed one of its own Blenders at a time on the Creator's Mac, which has 16 GB of memory, so the test was written out in full (`Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, under *Cannot be run*) and not run. The same holds for auto-save files, `quit.blend` and the preferences file when two Blenders run. A count of how many Blenders may run cannot be settled by a test on one computer.

**Not covered:** a limit on the number of instances; GPU sharing between instances; file locking; clashes in the temporary directory other than auto-save.

**Where I looked:** searched the 5.3 manual, API documentation and release notes for: multiple instances, another instance, instances of Blender, Blender instances, single instance, several instances, second instance, other instance, same time, simultaneous, concurrent, at once, file lock, lock file, lockfile, locked file, in use by, being written, being rendered, while rendering, during render, process ID, pid, per process, subprocess, separate process, multiple processes, render farm, multiple machines, network render, placeholder, network file system, shared directory. Read the GPU rendering questions page (https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html), the threading notes (https://docs.blender.org/api/5.3/info_gotchas_threading.html, which are about threads inside one Blender), the macOS launch page and the command-line pages. None of them addresses this.

**Outside the interface:** the "multiple instances" statement sits on the page about Blender driven as a Python module, and says it holds "As with the application". The render farm hint is written for the interface's Output panel; the matching settings (`use_overwrite`, `use_placeholder`) are on `RenderSettings` in the API. The sources say nothing else here.

**Brought up to date by the hand tests of 2026-10-17.** Nothing here was tested for 5.3; see *Not tested for 5.3, and why* above.

**Operating system:** the sources do not say. The temporary directory location differs by system: "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). The /tmp/ directory." (directory layout page above).

**Brought up to date by the hand tests of 2026-10-17.** Nothing here was tested for 5.3, on any system.

---

## 3. Can a single read of a file tell, for every view layer, whether that layer renders at any frame later asked about?

**Label:** documented that each view layer stores an on/off setting for rendering, and that "Render Single Layer" is ignored on the command line. **Not in the sources:** whether that setting can change from frame to frame. So whether one read settles every frame is **not settled** by the sources. Parts below are **inferred** and say so.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**What is stored per view layer.**
- `ViewLayer.use`: "Enable or disable rendering of this View Layer (default True)". API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html, read 2026-10-11.
- In the interface: "Use for Rendering — The active view layer will be used during rendering." Manual, https://docs.blender.org/manual/en/5.3/render/layers/view_layer.html, read 2026-10-11.

**The scene-wide "single layer" switch.**
- `RenderSettings.use_single_layer`: "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)". API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- Manual: "Render Single Layer — Only render the active view layer. Note — This option is ignored when rendering from the command line." View layer page above.

**Whether the setting can change over frames: not in the text.** No page says whether "Use for Rendering" can be keyframed or driven.
- One picture bears on it, weakly. The View Layer panel picture `_images/render_layers_view-layer_panel.png` (on the view layer page above) shows "Use for Rendering" (ticked) and "Render Single Layer" with **no** small dot to their right. The pass checkboxes below them (Combined, Z, Mist and others) **do** have a dot to their right. **Documented** (what the picture shows).
- The manual says what that dot means: "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated." Manual, https://docs.blender.org/manual/en/5.3/interface/controls/buttons/decorators.html, read 2026-10-11.
- **Inferred, and weak:** in that picture "Use for Rendering" is not offered for animation from the panel. Three gaps: the wording is "may appear", so a missing dot does not prove a setting cannot be animated; the page does not say which Blender version the picture comes from; and animation set up other ways (Python, drivers) is not addressed.
- A running Blender can be asked directly. `Property.is_animatable`: "Property is animatable through RNA (default False, readonly)". API, https://docs.blender.org/api/5.3/bpy.types.Property.html, read 2026-10-11. I did not run this.

**If a value is animated, a single read at one frame does not give other frames. Inferred.** Means to get a value at another frame exist:
- `Scene.frame_set(frame, *, subframe=0.0)`: "Set scene frame updating all objects and view layers immediately". API, https://docs.blender.org/api/5.3/bpy.types.Scene.html, read 2026-10-11.
- `FCurve.evaluate(frame)`: "Evaluate F-Curve … frame (float) – Frame, Evaluate F-Curve at given frame". API, https://docs.blender.org/api/5.3/bpy.types.FCurve.html, read 2026-10-11.

**Other things the sources tie to which view layers render.** These go beyond the per-layer setting, so "every view layer" is wider than one scene's list.
- The compositor's Render Layers node "renders a View Layer from a scene", with properties "Scene — The scene for which to render a view layer" and "View Layer — The view layer to render." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html, read 2026-10-11.
- Release note: "The compositor now renders scenes referenced inside node groups. (blender/blender@a939b4a6a2)". Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.
- A render engine is told when all view layers of a frame are done: `render_frame_finish()` "Perform finishing operations after all view layers in a frame were rendered". API, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html, read 2026-10-11.
- The sources do not say whether a Render Layers node renders a view layer whose "Use for Rendering" is off.

**Not covered:** whether `ViewLayer.use` can be keyframed or driven in 5.3; how a compositor Render Layers node interacts with a layer that is switched off; whether view layers of other scenes, reached through the compositor, are included.

**Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; recorded at the end of this answer).** Two of these items were tested. Keyframing: test 6 keyframed `LayerB.use` from a script and an animation render followed the keyframed value at each frame; test 7 pressed I over the checkbox in the window and a curve appeared. A Render Layers node aimed at a switched-off layer: test 8 rendered it and the node's picture was empty, against blue with the layer switched on. Drivers, and view layers of other scenes reached through the compositor, were not tested.

**Where I looked:** searched the 5.3 manual, API documentation and release notes for: use for rendering, render single layer, use_single_layer, single layer, animatable, not animatable, can be animated, keyframed, decorator, "view layer" next to animat / keyfram / frame, all view layers, every view layer, each view layer. Read the view layer pages (introduction, view layer), the `ViewLayer` and `RenderSettings` API pages, the decorators page, the Render Layers node page, and Cycles' `engine.py`, `__init__.py` and `ui.py` (no code in them decides which view layers render).

**Outside the interface:** "Render Single Layer" is ignored when rendering from the command line (documented, quoted above). Reading `ViewLayer.use` from Python without a window is **inferred** to work; the sources do not say.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window on the Creator's Mac: the switch's default and its *can be animated* mark were read (test 1); the switch was keyframed, read back at each frame, and followed frame by frame by an animation render started from the command line (test 6); and a Render Layers node aimed at a switched-off layer gave an empty picture in a command-line render (test 8). Only test 7 needed the window, and it is about what the panel shows.

**Operating system:** the sources do not say.

**Brought up to date by the hand tests of 2026-10-17.** Tests 1, 6, 7 and 8 below were run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro), so they settle macOS on Apple silicon for what they cover. No other machine was available for 5.3: Linux and Windows are untested for this version.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 1 — whether the two switches are marked as things that can be animated.** With no file opened, Blender's own description of each property was read — `bpy.types.ViewLayer.bl_rna.properties['use']` and `bpy.types.RenderSettings.bl_rna.properties['use_single_layer']` — and each one's `default` and `is_animatable` printed. What came back:

```
PROBE ViewLayer.use default = True | is_animatable = True
PROBE RenderSettings.use_single_layer default = False | is_animatable = False
```

**Test 6 — whether the switch can be keyframed, whether it reads differently at different frames, and whether an animation render follows it frame by frame.** With no window, a file was built from the factory startup: its view layer renamed `LayerA` and a second one, `LayerB`, added; frames 1 to 2; a 32 by 32 render; output `//out/frame_` as multi-layer OpenEXR (so each rendered view layer becomes a named layer inside the written file); new keyframes set to constant. `LayerB.use` was set on and `LayerB.keyframe_insert(data_path="use", frame=1)` called, then set off and `keyframe_insert(..., frame=2)` called. The script first tried the file format the way earlier Blenders take it, `image_settings.file_format = 'OPEN_EXR_MULTILAYER'`, and that run stopped with this error, recorded as it came:

```
TypeError: bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

The script was changed to set `image_settings.media_type = 'MULTI_LAYER_IMAGE'` first, and run again. What came back from building:

```
FILE FORMATS OFFERED ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
FORMAT MULTI_LAYER_IMAGE OPEN_EXR_MULTILAYER
ENGINE BLENDER_EEVEE
KEY1 True
KEY2 True
DIRECT FCURVES ERROR AttributeError 'Action' object has no attribute 'fcurves'
FCURVE view_layers["LayerB"].use [(1.0, 1.0, 'CONSTANT'), (2.0, 0.0, 'CONSTANT')]
Info: Saved as "vl.blend"
SAVE {'FINISHED'}
```

The curve was reached by walking the scene's action through `layers` › `strips` › `channelbags` › `fcurves`. A second run with no window reopened the file, stepped the frame with `scene.frame_set()` and read each view layer's switch, then evaluated the curve:

```
FRAME 1 [('LayerA', True), ('LayerB', True)]
FRAME 2 [('LayerA', True), ('LayerB', False)]
CURVE view_layers["LayerB"].use at 1 = 1.0 at 2 = 0.0
```

A third run with no window rendered the file as an animation with `-a`; the terminal lines naming view layers or saved files:

```
00:00.330  render           | Start rendering: Scene, LayerA
00:04.061  render           | Start rendering: Scene, LayerB
00:04.520  render           | Saved: '/…/e06/out/frame_0001.exr'
00:04.520  render           | Start rendering: Scene, LayerA
00:04.978  render           | Saved: '/…/e06/out/frame_0002.exr'
```

The view-layer names stored inside each written file, read with `strings -n 5 <file> | grep -oE "Layer[AB]" | sort -u`:

```
-- frame_0001.exr
LayerA
LayerB
-- frame_0002.exr
LayerA
```

**Test 7 — what the View Layer tab offers, and what pressing I over each of its two switches does, in Blender's window.** A file was saved from the factory startup with its view layer renamed `LayerA` and a second one, `LayerB`, added. It was opened in Blender's window, started from the terminal with `--no-window-focus --enable-event-simulate -p 0 0 1280 800`; the last setting lets a script move the pointer and press keys inside that one window. A script turned the window's biggest area into the Properties editor on its View Layer tab and photographed the area with Blender's own `bpy.ops.screen.screenshot_area`. The run printed `VL window 1280 x 800 | area at 4 159 size 2096 1388` and `VL properties context = VIEW_LAYER | view layer = LayerA`.

What that picture shows, read off it. A panel headed **View Layer** with two rows: **Use for Rendering**, its checkbox ticked, with a small round dot at the far right of its row; and **Render Single Layer**, its checkbox unticked, with no dot anywhere on its row. Below it, the **Passes** panel's checkboxes (Combined, Depth, Mist, Normal, Position, Vector, Grease Pencil, Denoising Data, and the Light group) have no dot at the right of their rows; a crop of the right-hand edge of the picture was looked at to be sure.

In two further runs, the script moved the pointer to the middle of the *Use for Rendering* checkbox, as read off that picture and turned into window coordinates (891, 1262), and pressed and released **I** there; then did the same over the *Render Single Layer* checkbox (891, 1217). After each press it printed every curve on the scene's animation and Blender's list of reports; the last run also photographed the whole window three seconds after the second press. What came back, the same in both runs:

```
VL pressed I at 891 1262
VL curves on the scene = ['view_layers["LayerA"].use']
VL reports = []
VL pressed I at 891 1217
VL curves on the scene = ['view_layers["LayerA"].use']
VL reports = []
```

What the pictures show. Each area picture shows the state from before the step just taken, so they are read one step late: the picture taken after the first press still shows the area as a 3D Viewport, from before the area had redrawn as the Properties editor. The picture taken after the second press, and the whole-window picture after it, show the *Use for Rendering* checkbox drawn in a yellow-green colour with a diamond in place of the dot at the far right of its row, and *Render Single Layer* unchanged — unticked, ordinary colour, no dot or diamond. The status bar along the bottom of the whole-window picture reads only `Pan View`, `Context Menu` and `5.3.0 b`; no message appears there three seconds after the second press, and nothing was printed on the terminal. Whether a message showed and went in those three seconds is not established by this test.

**Test 8 — whether a compositor Render Layers node renders a view layer whose *Use for Rendering* is off.** With no window, a file was built: Cycles on the processor, 8 by 8, one sample, Standard view transform, a camera looking straight down; a red emissive plane in collection `ColA` and a blue emissive plane in `ColB`, at the same place; view layer `LayerA` with `ColB` excluded, and view layer `LayerB` with `ColA` excluded. The scene was given a compositor node group whose only output is a Render Layers node with its scene set to this scene and its layer set to `LayerB`, wired to the group's output; compositing on. The file was saved twice: with `LayerB.use` off (`use_off.blend`) and with it on as the control (`use_on.blend`). What building printed:

```
…/e08_rl_build.py:22: DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0
  sc.compositing_node_group = tree
BRANCH node group
NODE layer = LayerB | outputs = ['Image', 'Alpha']
Info: Saved as "use_off.blend"
SAVE off {'FINISHED'} | LayerB.use = False
Info: Saved as "use_on.blend"
SAVE on {'FINISHED'} | LayerB.use = True
```

Each file was rendered with no window, one frame, by `-o <folder>/<name>_#### -f 1`; the terminal lines naming view layers or saved files:

```
== use_off
00:00.334  render           | Start rendering: Scene, LayerA
00:00.497  render           | Saved: '/…/e08/use_off_0001.png'
== use_on
00:00.337  render           | Start rendering: Scene, LayerA
00:00.427  render           | Start rendering: Scene, LayerB
00:00.458  render           | Saved: '/…/e08/use_on_0001.png'
```

The centre pixel of each written picture, read back by Blender as red, green, blue, alpha:

```
PIXEL use_off_0001.png [0.0, 0.0, 0.0, 0.0]
PIXEL use_on_0001.png [0.0, 0.0, 1.0, 1.0]
```

With `LayerB` switched off, the render named only `LayerA` and the picture the node fed out was empty; with it switched on, the render named both and the picture was blue.

---

## 4. Can the places a file is set up to write to be read out of it?

**Label:** documented that the file stores these places as settings that Blender's Python API exposes. Reading them is done through Blender. **Inferred** that paths with project variables cannot be resolved from the file alone, that Python calls work without a window, and that stored absolute paths are machine-tied. **Not in the sources:** reading them without Blender. The list below is what I found; the sources give no complete list.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Settings that hold a place to write to** (each from its API page, read 2026-10-11):

| Where | Setting | API wording |
|---|---|---|
| Render output | `RenderSettings.filepath` | "Directory/name to save animations, # characters define the position and padding of frame numbers (default “//”, never None, blend relative // prefix supported, Supports template expressions)" — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html |
| Render output on/off | `RenderSettings.save_output` | "Write frames to disk for animation renders (default True)" — same page |
| Compositor File Output node | `CompositorNodeOutputFile.directory`, `.file_name` | "The directory where the image will be written (… blend relative // prefix supported, Supports template expressions)"; "The base name of the file. Other information might be included in the final file name depending on the node options" — https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html |
| Geometry Nodes bakes | `NodesModifier.bake_directory`, `NodesModifierBake.directory` | "Location on disk where the bake data is stored (… Supports template expressions)" — https://docs.blender.org/api/5.3/bpy.types.NodesModifier.html and https://docs.blender.org/api/5.3/bpy.types.NodesModifierBake.html |
| Physics point cache | `PointCache.filepath` | "Cache file path" — https://docs.blender.org/api/5.3/bpy.types.PointCache.html |
| Fluid cache | `FluidDomainSettings.cache_directory` | "Directory that contains fluid cache files" — https://docs.blender.org/api/5.3/bpy.types.FluidDomainSettings.html |
| Dynamic Paint | `DynamicPaintSurface.image_output_path` | "Directory to save the textures" — https://docs.blender.org/api/5.3/bpy.types.DynamicPaintSurface.html |
| Ocean modifier | `OceanModifier.filepath` | "Path to a folder to store external baked images" — https://docs.blender.org/api/5.3/bpy.types.OceanModifier.html |
| Texture baking | `BakeSettings.filepath` | "Image filepath to use when saving externally" — https://docs.blender.org/api/5.3/bpy.types.BakeSettings.html |
| Collection exporters | `CollectionExport.filepath` | "The file path used for exporting" — https://docs.blender.org/api/5.3/bpy.types.CollectionExport.html |
| Sequencer and clip proxies | `SequenceEditor.proxy_dir`, `StripProxy.directory`, `MovieClipProxy.directory` | "Location to store the proxy files" (the last two) — https://docs.blender.org/api/5.3/bpy.types.StripProxy.html, https://docs.blender.org/api/5.3/bpy.types.MovieClipProxy.html |

The final file name for a render frame can be asked for: `RenderSettings.frame_path(*, frame=…, preview=False, view='')`: "Return the absolute path to the filename to be written for a given frame". Same RenderSettings page.

**What a read of the stored setting does not give:**
- Templates stay unresolved in the stored path. "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." Manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html, read 2026-10-11. Same page: "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node." The 5.3 release notes add "Bake path template support (blender/blender!164240)" (https://developer.blender.org/docs/release_notes/5.3/geometry_nodes/) and "`{project_root}` and custom project variables for path templates" (https://developer.blender.org/docs/release_notes/5.3/core/), both read 2026-10-11. **Inferred:** a path using project variables cannot be fully resolved from the file alone, since a project "is identified by a `.blender_project` directory inside the root directory" (core release notes).
- Some places are worked out from the file's own location, not stored. Point cache on disk: "Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file." Manual, https://docs.blender.org/manual/en/5.3/physics/baking.html, read 2026-10-11.
- Some places come from preferences, not the file. Cycles' texture cache: "By default, files are stored in the blender_tx/ folder next to the image file. This can be changed with the Texture Cache file path preference." Manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html, read 2026-10-11. The render cache: "Render Cache — The location where cached render images are stored." in the File Paths preferences. Manual, https://docs.blender.org/manual/en/5.3/editors/preferences/file_paths.html, read 2026-10-11.
- Saving can also write images back to where they came from: "When saving a blend-file, Blender may also save modified external images depending on the Save Modified Images preference." Open/save page above.

**Not covered:** whether `frame_path()` resolves templates; whether any of the listing tools in question 9 include these write places; places that add-ons write to; a complete list.

**Brought up to date by the hand tests of 2026-10-17 (recorded at the end of this answer).** `frame_path()` returned the path with `{blend_name}` and `{scene_name}` worked out and raised an error on a broken template (test 9). The whole-file listing means — `blend_paths()`, `file_path_map()`, `file_path_foreach()` — carried neither the render output path nor a File Output folder (test 10), nor the bake output path, a cloth point cache or a geometry-nodes bake folder (test 20, under question 9). The File Output node's sidebar shows one assembled path under *Output Paths* (test 11).

**Where I looked:** listed every API property described as "blend relative // prefix supported" and read each description. Searched the manual for: blendcache, bake directory, bake path, next to the blend, next to the image, path templates, project_root, blender_project. Read the File Paths, Output properties, File Output node, Baking, Geometry Nodes modifier, Cycles performance and File Paths preferences pages.

**Outside the interface:** reading these through Python without a window is **inferred** to work; the sources do not say. The command line can replace the render output at run time without changing what the file stores: "-o, --render-output <path> Set the render path and file name." Order matters: "…will not render to /tmp because loading the blend-file overwrites the render output that was set." Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11. A bad template stops an animation render: "if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors." File Paths page above.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: every write place listed here was read back by name, `frame_path()` ran, and a render wrote where the scene and the File Output node point (tests 9, 10 and 20). `-o` given after the file name replaced the render output path for that run, and lasted only if the file was then saved (test 24). Test 11 is the window's side.

**Operating system:** the sources do not say. Absolute paths are defined as "full path from the system’s root" (https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html), so a stored absolute path is tied to one machine's folder layout. That last step is **inferred**.

**Brought up to date by the hand tests of 2026-10-17.** Tests 9, 10, 11, 20, 24 and 28 settle macOS on Apple silicon for what they cover. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 1 — the output mark, the File Output node's properties, and where the scene keeps its compositor.** With no file opened, Blender's own description of six path settings was read and four marks printed for each; then the full list of the File Output node's properties; then whether the scene type has properties named `node_tree`, `compositing_node_group` and `use_nodes`. What came back:

```
PROBE Property.is_path_output exists = True
PROBE RenderSettings.filepath subtype=FILE_PATH is_path_output=True is_path_supports_blend_relative=True is_path_supports_templates=True is_readonly=False
PROBE Image.filepath subtype=FILE_PATH is_path_output=False is_path_supports_blend_relative=True is_path_supports_templates=False is_readonly=False
PROBE Library.filepath subtype=FILE_PATH is_path_output=False is_path_supports_blend_relative=True is_path_supports_templates=False is_readonly=False
PROBE PointCache.filepath subtype=DIR_PATH is_path_output=False is_path_supports_blend_relative=True is_path_supports_templates=False is_readonly=False
PROBE Text.filepath subtype=NONE is_path_output=False is_path_supports_blend_relative=False is_path_supports_templates=False is_readonly=False
PROBE Camera.custom_filepath subtype=FILE_PATH is_path_output=False is_path_supports_blend_relative=False is_path_supports_templates=False is_readonly=False
PROBE CompositorNodeOutputFile properties = ['active_item_index', 'bl_description', 'bl_height_default', 'bl_height_max', 'bl_height_min', 'bl_icon', 'bl_idname', 'bl_label', 'bl_static_type', 'bl_width_default', 'bl_width_max', 'bl_width_min', 'color', 'color_tag', 'dimensions', 'directory', 'file_name', 'file_output_items', 'format', 'height', 'hide', 'inputs', 'internal_links', 'label', 'location', 'location_absolute', 'mute', 'name', 'outputs', 'panel_states', 'parent', 'rna_type', 'save_as_render', 'select', 'show_options', 'show_preview', 'show_texture', 'type', 'use_custom_color', 'use_file_extension', 'warning_propagation', 'width']
PROBE Scene has node_tree = False
PROBE Scene has compositing_node_group = True
PROBE Scene has use_nodes = True
```

No property of the File Output node in that list holds an assembled final path per input.

**Test 9 — what a stored output path with a template reads back as, and what `frame_path()` makes of it.** With no window, `tmpl.blend` was saved in an empty folder with its render output path set to `//out/{blend_name}_/frame_`. A second run with no window reopened it, printed the stored string, then set the stored string to each of four values in turn and asked `scene.render.frame_path(frame=7)` each time. The folder path is shortened to `/…/e09/`. What came back:

```
STORED '//out/{blend_name}_/frame_'
SET '//out/{blend_name}_/frame_'
FRAME_PATH '/…/e09/out/tmpl_/frame_0007.png'
SET '//out/{scene_name}_/frame_'
FRAME_PATH '/…/e09/out/Scene_/frame_0007.png'
SET '//out/{blend_name_/frame_'
FRAME_PATH ERROR RuntimeError Error: Parse errors in path '//out/{blend_name_/frame_':
- Invalid or incomplete template expression '{blend_name_/frame_'.
SET '//out/plain_/frame_'
FRAME_PATH '/…/e09/out/plain_/frame_0007.png'
```

**Test 10 — the File Output node's stored paths, the files a render writes, and whether the whole-file path lists carry output paths.** With no window, a file was built from the factory startup at 64 by 36: the scene's own output path set to `//scene_out/frame_`; a compositor node group given to the scene with a Render Layers node wired to the group's output; and one File Output node with its folder set to `//out/`, its file name set to `shot`, and two extra inputs added by `file_output_items.new('RGBA', name)`, named `passes/diffuse` and `beauty`, each wired from the Render Layers node's picture. The script listed the node's attributes that a plain node does not have. What building printed:

```
…/e10_fo_build.py:11: DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0
  sc.compositing_node_group = tree
BRANCH new
TREE <bpy_struct, CompositorNodeTree("Compositing") at 0xb7813f608>
NODE ATTRS ['active_item_index', 'bl_description', 'bl_height_default', 'bl_height_max', 'bl_height_min', 'bl_icon', 'bl_idname', 'bl_label', 'bl_static_type', 'bl_system_properties_get', 'bl_width_default', 'bl_width_max', 'bl_width_min', 'color', 'color_tag', 'debug_zone_body_lazy_function_graph', 'debug_zone_lazy_function_graph', 'dimensions', 'directory', 'draw_buttons', 'draw_buttons_ext', 'file_name', 'file_output_items', 'format', 'height', 'hide', 'input_template', 'inputs', 'internal_links', 'label', 'location', 'location_absolute', 'mute', 'name', 'output_template', 'outputs', 'panel_states', 'parent', 'poll_instance', 'rna_type', 'save_as_render', 'select', 'show_options', 'show_preview', 'show_texture', 'socket_value_update', 'type', 'update', 'use_custom_color', 'use_file_extension', 'warning_propagation', 'width']
SET directory
SET file_name
NEW SIGNATURE HELP NodeCompositorFileOutputItems.new(socket_type, name)
Add an item at the end
ITEM ADDED passes/diffuse
ITEM ADDED beauty
Info: Saved as "fo.blend"
SAVE {'FINISHED'}
```

A second run with no window reopened the file and read the node back, then asked the three whole-file listing means — `bpy.utils.blend_paths()`, `bpy.data.file_path_map()` (entries holding no path left out) and the paths `bpy.data.file_path_foreach()` hands its function with its default switches:

```
BASE_PATH 'NO SUCH ATTRIBUTE'
DIRECTORY '//out/'
FILE_NAME 'shot'
ITEM NAMES ['passes/diffuse', 'beauty']
INPUT NAMES ['passes/diffuse', 'beauty', '']
RENDER FILEPATH '//scene_out/frame_'
BLEND_PATHS ['//../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
FILE_PATH_MAP {}
FILE_PATH_FOREACH []
```

(That run, too, printed the same deprecation warning for `Scene.compositing_node_group`.) The one entry in the flat list is a file inside the Blender application; nothing in the built file was pointed at it. Rendering frame 1 with no window (`-f 1`) printed:

```
00:01.030  render           | Saved: '/…/e10/out/shot0001.exr'
00:01.094  render           | Saved: '/…/e10/scene_out/frame_0001.png'
```

and the files found in the folder afterwards, other than the blend-files, were exactly `out/shot0001.exr` and `scene_out/frame_0001.png`.

**Test 11 — what the File Output node's sidebar shows, in Blender's window.** Test 10's `fo.blend` was opened in Blender's window, started from the terminal with `--no-window-focus --enable-event-simulate -p 0 0 1600 1000`; the last setting lets a script move the pointer, click and press keys inside that one window. A script switched to the Compositing workspace, made the File Output node the only selected and the active node, hid the node editor's asset shelf, opened its sidebar, and framed the node. It then clicked the sidebar's **Node** tab with a simulated click, clicked the *Node* panel's header and the *Node Format* header to fold them, and clicked the *Output Paths* header to open it, photographing the area with Blender's own `bpy.ops.screen.screenshot_area` after each step, each time just after forcing a redraw with `bpy.ops.wm.redraw_timer(type='DRAW_WIN_SWAP', iterations=1)`. Every click point was read off the previous run's picture. It took nine runs to arrive at that sequence: setting the sidebar's tab from the script printed success, but after a redraw the sidebar showed its **Group** tab again; changing the interface scale, or making the node editor full-screen, did the same; and simulated mouse-wheel turns and `bpy.ops.view2d.scroll_down` did not move the sidebar. The last run printed:

```
FOW view_selected {'FINISHED'}
FOW area at 4 104 size 3016 815 | sidebar at 2459 104 size 561 763
FOW clicked at 2997 714
FOW sidebar tab now = Node
FOW screenshot_area tab.png {'FINISHED'}
FOW sidebar scroll_down {'FINISHED'}
FOW sidebar scroll_down {'FINISHED'}
FOW sidebar scroll_down {'FINISHED'}
FOW screenshot_area scrolled.png {'FINISHED'}
FOW clicked Node panel header at 2546 837
FOW clicked Node Format header at 2615 269
FOW screenshot_area folded.png {'FINISHED'}
FOW clicked Output Paths header at 2615 233
FOW screenshot_area output_paths.png {'FINISHED'}
FOW screenshot {'FINISHED'}
```

What the pictures show, read off them. While the script had not yet chosen a tab, the sidebar's tabs read **Group, Tool, View, Options**; once the node was active they read **Group, Node, Tool, View, Options**. On the **Node** tab, top to bottom: a panel **Node** (Name `File Output`, Label empty, Color, Show Options ticked, Mute unticked, Propagate `All Messages`); a panel **Properties** holding a field reading `//out/`, a field reading `shot`, a two-way choice `Image` / `Multi-Layer EXR` with `Multi-Layer EXR` chosen, a section **Node Format** (Save as Render ticked, Color Depth `Float (Half)` / `Float (Full)` with `Float (Full)` chosen, Codec `None`), a section **Layers** listing `passes/diffuse` and `beauty` with Socket Type `Color`, and a section **Output Paths**, closed until clicked; then **Custom Properties**. Opened, **Output Paths** holds a `File Extensions` checkbox, ticked, and below it **one** line beside a small picture icon, reading:

```
/private/tmp/claud…k/e10/out/shot.exr
```

The middle of the path is cut by Blender itself with `…` to fit the sidebar's width. The picture ends just below that line, so whether anything follows it is not shown.

**Test 20 — the many-paths file.** Under question 9, test 20: on a file whose render output path, bake output path, File Output folder, cloth point cache and geometry-nodes bake folder were all set, none of those five appeared in `bpy.utils.blend_paths()`, `bpy.data.file_path_map()` or `bpy.data.file_path_foreach()`; all five were found by walking Blender's own property marks, and read back by name.

**Test 24 — the render-output argument.** Under question 9, test 24: a file opened with `-o //FROM_CLI/` after its name held `//FROM_CLI/` as its render output path in that run, and still held it when opened again after being saved in that run; a file opened with `-o //NOSAVE_CLI/` and not saved held that path in that run only, and its stored path read as before in the next run.

**Test 28 — a template in an output path, through both sweeps.** Under question 9, test 28: a render output path and a File Output folder holding `{blend_name}` were left unchanged by Make Paths Relative and Make Paths Absolute and appeared in no listing means, while `frame_path(frame=1)` returned the path with the template worked out (`out_tmpl_0001.png`) before and after each sweep.

---

## 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**Label:** documented that a running Blender can read and rewrite them, through the interface and through Python. **Not in the sources:** which outside tool, if any, can do the same. **Inferred:** a change becomes part of the file on disk only when the file is saved.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Read out, by Blender's Python.**
- All at once, as a list: `bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)`: "Returns a list of paths to external files referenced by the loaded .blend file." API, https://docs.blender.org/api/5.3/bpy.utils.html, read 2026-10-11.
- Grouped by data-block: `BlendData.file_path_map(*, subset=None, key_types=None, include_libraries=False)`: "Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them." API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html, read 2026-10-11.
- One at a time: each data-block's own path setting, for example `Image.filepath`, `Library.filepath`, `Sound.filepath`, `VectorFont.filepath`, `Volume.filepath`, `MovieClip.filepath`, `CacheFile.filepath` (each API page, read 2026-10-11).
- Paths inside linked data are relative to their library: "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID." API, https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html, read 2026-10-11.

**Read out, in the interface.**
- Linked files: "They’re also listed in the Outliner’s Blender File Display Mode, along with the path of the blend-file they originate from." Manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html, read 2026-10-11.
- An image's path: "Path — Path to the linked file." Manual, https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html, read 2026-10-11.

**Written back, by Blender's Python.**
- `BlendData.file_path_foreach(visit_path_fn, …)`: "The function should return either None or a str. In the latter case, the visited file path will be replaced with the returned string." BlendData page above.
- Some paths refuse it. `BlendDataPathMeta`: "is_cache — … These paths can not be edited."; "is_expanded — True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited."; "is_readonly — True when the path is read-only and can not be edited." API, https://docs.blender.org/api/5.3/bpy.types.BlendDataPathMeta.html, read 2026-10-11.
- Data can be reloaded on edit: "RELOAD_EDITED: … Reload data when the path is edited." API, https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html, read 2026-10-11.
- Setting a data-block's own path. For images there are two settings: `filepath` "Image/Movie file name" and `filepath_raw` "Image/Movie file name (without data refreshing)". API, https://docs.blender.org/api/5.3/bpy.types.Image.html, read 2026-10-11.
- Library paths carry a warning: "WARNING: Typically, this value should be considered read-only, changing this path without proper processing afterwards can leave Blender data in an invalid state". API, https://docs.blender.org/api/5.3/bpy.types.Library.html, read 2026-10-11.
- Operators that rewrite many paths at once: `bpy.ops.file.make_paths_relative()`, `make_paths_absolute()`, `find_missing_files()`; `bpy.ops.wm.lib_relocate()`; `save_as_mainfile(relative_remap=True)`. Details are in questions 6 and 9.

**Written back, in the interface.** File ‣ External Data ‣ Make Paths Relative / Make Paths Absolute / Find Missing Files; Outliner ‣ Relocate for linked files; Save As with "Remap Relative". Sources in questions 6 and 9.

**Outside Blender.** The manual names "Blender Asset Tracer" as "tooling that can manipulate blend-files outside of Blender itself" (question 1). The sources do not say whether it reads or rewrites recorded locations.

**Paths stored inside a linked library file.** "Blender itself is not allowed to write to other blend-files than the currently open one." Manual, https://docs.blender.org/manual/en/5.3/files/asset_libraries/introduction.html, read 2026-10-11. **Inferred:** to rewrite a path stored inside a library file, that library file must itself be opened in Blender and saved. The manual says this rule only in a section about assets. It also documents writing to other files in some cases (for example `bpy.data.libraries.write()`, which writes chosen data-blocks to a new file), so the reach of the rule is not fully clear.

**Saving.** **Inferred:** rewriting a path changes Blender's copy in memory; it reaches the file on disk only when saved. The sources do not say this for paths in so many words. They say it for packing: "Actual packing will happen on the next save of the blend-file." Manual, https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html, read 2026-10-11.

**Not covered:** what tools other than Blender can do; whether every kind of stored path is reachable by `file_path_foreach`; what "proper processing" after changing `Library.filepath` means.

**Brought up to date by the hand tests of 2026-10-17 (recorded under this answer and under question 9).** What `file_path_foreach` reaches was tested against twenty-three kinds of recorded location: it visited sixteen paths and missed the render output, bake output, File Output folder, cloth point cache, geometry-nodes bake folder, sequencer proxy folder and a custom property (tests 20 and 23). Assigning `Library.filepath` changed the recorded path at once but brought the new file's data in only after `Library.reload()` (test 31). What tools other than Blender can do was not tested.

**Outside the interface:** all the Python means above are functions or operators. Running them without a window is **inferred** to work; the sources do not say so for these calls.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: reading and assigning an image's path and saving it (test 12); every listing means, both sweeps, the one-pass visitor, the write call, Save As with Remap Relative, a library's own path with `reload()`, and the relocate command (tests 20 to 24 and 26 to 32) all ran. `image.replace` and `sequencer.change_path` ran with no window once their editor and context were supplied (test 33). A rewrite that was not saved was gone in the next run (test 24).

**Operating system:** the sources do not say how paths written on one operating system behave on another. Related documented facts: "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user’s file system cannot be ignored." (encoding gotchas page above). `bpy.path.native_pathsep(path)`: "Replace the path separator with the system’s native os.sep." `bpy.path.basename`: "Equivalent to os.path.basename, but skips a “//” prefix. Use for Windows compatibility." API, https://docs.blender.org/api/5.3/bpy.path.html, read 2026-10-11.

**Brought up to date by the hand tests of 2026-10-17.** These tests settle macOS on Apple silicon for what they cover; on this Mac, the image's own file was found when its name was asked for in capitals (test 21). Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 1 — which path-reading and path-writing calls exist.** With no file opened, the script asked whether each of six whole-file calls exists, called `bpy.utils.blend_paths(True)` to see whether it takes a plain argument, and printed the full list of commands under `bpy.ops.file`. What came back:

```
PROBE has bpy.utils.blend_paths = True
PROBE has bpy.data.file_path_map = True
PROBE has bpy.data.file_path_foreach = True
PROBE has bpy.data.user_map = True
PROBE has bpy.data.all_ids = True
PROBE has bpy.data.temp_data = True
PROBE blend_paths positional = ERROR TypeError blend_paths() takes no positional arguments
PROBE bpy.types.BlendDataPathMeta exists = True
PROBE WindowManager.reports exists = True
PROBE bpy.ops.file operators = ['autopack_toggle', 'bookmark_add', 'bookmark_cleanup', 'bookmark_delete', 'bookmark_move', 'cancel', 'delete', 'directory_new', 'edit_directory_path', 'execute', 'external_operation', 'filenum', 'filepath_drop', 'find_missing_files', 'hidedot', 'highlight', 'make_paths_absolute', 'make_paths_relative', 'mouse_execute', 'next', 'pack_all', 'pack_libraries', 'parent', 'previous', 'refresh', 'rename', 'report_missing_files', 'reset_recent', 'select', 'select_all', 'select_bookmark', 'select_box', 'select_walk', 'smoothscroll', 'sort_column_ui_context', 'start_filter', 'unpack_all', 'unpack_item', 'unpack_libraries', 'view_selected']
```

**Test 12 — reading a recorded image path, writing a new one, and whether it survives a save.** With no window, a file was first saved as `img.blend` in an empty folder; a 4 by 4 PNG was written by Blender itself at `tex/probe.png` beside it; an image data-block was loaded from it with `bpy.data.images.load(<full path>)`, renamed `ProbeLoaded` and given a fake user so it survives the save; and the file saved again with `relative_remap=True`. What the build printed after that save (the folder is shortened to `/…/e12/`):

```
SAVE {'FINISHED'}
BUILT [('ProbeLoaded', '/…/e12/tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
```

So the path, given in full form when the image was loaded, stayed in full form through a save with `relative_remap=True`. A second run with no window opened the file, read the paths, assigned `img.filepath = "//tex/moved.png"`, and saved:

```
BEFORE [('ProbeLoaded', '/…/e12/tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/…/e12/tex/probe.png']
BLEND_PATHS ABS ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/…/e12/tex/probe.png']
ASSIGNED OK
AFTER ASSIGN //tex/moved.png
BLEND_PATHS AFTER ASSIGN ['//../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
```

A third run with no window opened the file again:

```
REOPENED [('ProbeLoaded', '//tex/moved.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex/moved.png']
```

Nothing in the built file was pointed at the Blender application's own brushes file; that entry appeared in `blend_paths()` of its own accord.

**Test 13 — whether recorded paths survive when the files they name are gone.** Built as test 12 but with two images, `ProbeLoaded` from `tex/probe.png` and `SecondLoaded` from `tex/second.png` (both paths in full form); then both PNGs renamed away. The file was opened with no window and the script printed each image's path, each one's `is_missing`, each listed path with whether `os.path.exists` finds it, and the whole map of data-blocks to paths. What came back:

```
IMAGES [('ProbeLoaded', '/…/e13/tex/probe.png'), ('Render Result', ''), ('SecondLoaded', '/…/e13/tex/second.png'), ('Viewer Node', '')]
IS_MISSING [('ProbeLoaded', False), ('Render Result', False), ('SecondLoaded', False), ('Viewer Node', False)]
BLEND_PATHS ABS [('/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', True), ('/…/e13/tex/probe.png', False), ('/…/e13/tex/second.png', False)]
FILE_PATH_MAP {'WinMan': [], 'Animation': [], 'Compositing': [], 'Geometry Nodes': [], 'Layout': [], 'Modeling': [], 'Rendering': [], 'Scripting': [], 'Sculpting': [], 'Shading': [], 'Texture Paint': [], 'UV Editing': [], 'Scene': [], 'Palette': [], 'Collection': [], 'Camera': [], 'Cube': [], 'Light': [], 'World': [], 'LineStyle': [], 'Material': [], 'ProbeLoaded': ['/…/e13/tex/probe.png'], 'Render Result': [], 'SecondLoaded': ['/…/e13/tex/second.png'], 'Viewer Node': []}
```

What the missing-file report did with the same file is under question 6, test 13.

**Tests 20 to 25 — many kinds of recorded location at once.** A file holding twenty-three kinds of recorded location was put through every reading and rewriting means: read by name, listed, swept relative and absolute, rewritten by the one-pass visitor, rewritten with no save, and looked at in the Outliner's Data API view. These are recorded under question 9, tests 20 to 25.

**Test 24 — whether a rewrite reaches the file on disk without a save.** Under question 9, test 24: after `bpy.ops.file.make_paths_relative()` in a run that quit without saving, the image's path read `//../e20/assets/probe.png` in that run and was back in full form in the next run.

**Tests 31 and 32 — a library's own path, and paths inside linked data.** Under question 9: assigning a linked library's `filepath` changed the path the objects report, but the objects stayed those of the old file until the library's `reload()` was called, after which the new file's objects were there; the relocate command did it in one step and wrote the path in `//` form (test 31). A path stored inside a library, `//libtex.png`, made real with `bpy.path.abspath()` alone pointed beside the open file, where there is no such file; with `library=` set to the library it pointed beside the library, where the file is (test 32).

**Test 51 — whether writing a path reloads what it points at.** Recorded under question 11: after an image's pixels had been read, assigning `filepath` dropped its loaded data and the next read gave the new file's picture; assigning `filepath_raw`, or rewriting the path with `bpy.data.file_path_foreach()` and its default switches, kept the old picture; `file_path_foreach()` with `RELOAD_EDITED` gave the new picture.

---

## 6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?

**Label:** documented that all three exist as menu items and as operators. **Inferred** that passing directory from Python avoids the file browser. **Not in the sources:** whether libraries are covered by report or search, where reports go without a window, and whether missing files are listed or only warned about.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Report.**
- "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor’s header. If no warning is shown, there are no missing external files." Manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11.
- Operator: `bpy.ops.file.report_missing_files()` "Report all missing external files". API, https://docs.blender.org/api/5.3/bpy.ops.file.html, read 2026-10-11.

**Search and rewrite.**
- "Find Missing Files — In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative." Topbar page above.
- "Note — Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once." Same page.
- Operator: `bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)` "Try to find missing external files"; "find_all (bool) – Find All, Find all files in the search path (not just missing)"; "directory (str) – Directory, Directory of the file". API, bpy.ops.file page above.

**Rewrite all paths' form.**
- "Make Paths Relative — Make all paths to external files relative to the current blend-file." "Make Paths Absolute — Make all paths to external files absolute (= full path from the system’s root)." Topbar page above. Operators `bpy.ops.file.make_paths_relative()`, `bpy.ops.file.make_paths_absolute()`, bpy.ops.file page above.

**Missing linked files (libraries).**
- "Broken Libraries — If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored." Manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html, read 2026-10-11.
- Relocate: "It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed)". Same page. Operator `bpy.ops.wm.lib_relocate(…)` "Relocate the given library to one or several others". API, https://docs.blender.org/api/5.3/bpy.ops.wm.html, read 2026-10-11.
- Each placeholder can be recognised: `ID.is_missing` "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)". API, https://docs.blender.org/api/5.3/bpy.types.ID.html, read 2026-10-11.
- The sources do not say whether Report Missing Files and Find Missing Files also cover missing library files. The library pages describe Relocate, not a search.

**Missing media in the Video Sequencer** is also shown: "Strips with missing content will be displayed as red in the timeline." Manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/preview/sidebar.html, read 2026-10-11.

**Not covered:** where the report text goes when there is no window; whether the report lists each missing file or only warns; whether library files are included in report or search; how a match is chosen (question 10).

**Brought up to date by the hand tests of 2026-10-17 (recorded at the end of this answer).** With no window, Report Missing Files printed one `Warning:` line per missing file on the terminal, each naming the full path and the data-block, and the window manager's list of reports stayed empty (test 13). It named a missing linked blend-file as well, and Find Missing Files rewrote that library's path (test 37). In a window, the message appears in the status bar when the command is chosen from the File menu, and did not when it was called from a script (test 15). How a match is chosen is under question 10.

**Outside the interface:** the operators are callable from Python. What happens to a warning report when there is no Info editor is **not in the sources**. For errors only: "if there are error reports, a RuntimeError will be raised after the operator finishes execution, including all error report messages". API, https://docs.blender.org/api/5.3/bpy.ops.html, read 2026-10-11. Find Missing Files opens a File Browser in the interface. From Python the `directory` setting exists; that it can be passed instead of using the browser is **inferred**.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: both commands ran; the report went to the terminal and not to the reports list (test 13); `find_missing_files(directory=…)` searched the folder given and opened nothing (tests 14, 35 to 38). Called from a script in a window, the search opened no second window (test 15).

**Operating system:** the sources do not say.

**Brought up to date by the hand tests of 2026-10-17.** These tests settle macOS on Apple silicon for what they cover. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 4 — how missing linked data is flagged.** See question 8, test 4: with a linked file renamed away and the file opened with no window, the library entry and the linked object or collection each came back with `is_missing` `True`, and Blender printed warnings naming the missing file and the missing data-block on the terminal.

**Test 13 — what Report Missing Files produces with no window, for two missing files.** A file holding two images whose PNGs had been renamed away (question 5, test 13) was opened with no window; the script called `bpy.ops.file.report_missing_files()` and then read `bpy.context.window_manager.reports`. What came back on the terminal:

```
Warning: Path '/…/e13/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
Warning: Path '/…/e13/tex/second.png' not found, from local data-block 'IMSecondLoaded'
REPORT RESULT {'FINISHED'}
WM REPORTS []
```

One line per missing file, each naming the full path and the data-block; the window manager's list of reports was empty straight after. The same run printed `is_missing` as `False` for both images.

**Test 14 — whether Find Missing Files can be handed a folder with no window, and what it writes back.** Built as question 5, test 12 (one image `ProbeLoaded` loaded from `tex/probe.png`), in two arms. In the first the image's path was left in full form, as loaded. In the second a further run rewrote it with `bpy.path.relpath()` and saved, and printed `MADE RELATIVE [('ProbeLoaded', '//tex/probe.png')]`. In both, the PNG was then moved to `search/deeper/probe.png`, the file opened with no window, `bpy.ops.file.find_missing_files(directory=<the search/ folder>)` called, and the file saved; then opened again in a further run. The folders are shortened to `/…/e14/` and `/…/e14rel/`.

Path in full form to start with:

```
BEFORE [('ProbeLoaded', '/…/e14/tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
FIND RESULT {'FINISHED'}
AFTER [('ProbeLoaded', '/…/e14/search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
REOPENED [('ProbeLoaded', '/…/e14/search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/…/e14/search/deeper/probe.png']
```

Path in `//` form to start with:

```
BEFORE [('ProbeLoaded', '//tex/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
FIND RESULT {'FINISHED'}
AFTER [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
Info: Saved "img.blend"
SAVE RESULT {'FINISHED'}
REOPENED [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
BLEND_PATHS ['//../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//search/deeper/probe.png']
```

In both arms the command ran with no window, rewrote the path to where the file now is, and the rewrite was still there after saving and reopening. The form written back matched the form recorded before.

**Test 15 — Report Missing Files and Find Missing Files in Blender's window.** Built as test 14's second arm: one image `ProbeLoaded` recorded as `//tex/probe.png`, its PNG then moved to `search/deeper/probe.png`. The file was opened in Blender's window, started from the terminal with `--no-window-focus -p 0 0 1280 800`.

*Both commands called from a script running inside the window.* Three seconds after opening, a timer called `bpy.ops.file.report_missing_files()` with the window in its context, counted the windows before and after, and read the window manager's list of reports; then the whole window was photographed with `bpy.ops.screen.screenshot` just after a forced redraw (`bpy.ops.wm.redraw_timer(type='DRAW_WIN_SWAP', iterations=1)`). Then `bpy.ops.file.find_missing_files(directory=<the search/ folder>)` was called the same way, with the windows counted before and after, and the window photographed again. What came back:

```
WIN count before report 1
Warning: Path '/…/e15b/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
WIN report result {'FINISHED'}
WIN count after report 1
WIN reports []
WIN screenshot 1 {'FINISHED'}
WIN count before find 1
WIN find result {'FINISHED'}
WIN count after find 1
WIN images after [('ProbeLoaded', '//search/deeper/probe.png'), ('Render Result', ''), ('Viewer Node', '')]
WIN screenshot 2 {'FINISHED'}
WIN count at the end 1
```

In both pictures the status bar along the bottom reads only `Select`, `Rotate View`, `Options` and `5.3.0 b`: the warning printed on the terminal did not appear in the window. (A first run of the same thing, without the forced redraw, gave the same lines and the same status bar.) No second window opened for the search.

*Report Missing Files chosen from the File menu.* The same file was opened again in a window started with `--enable-event-simulate`, which lets a script move the pointer and click inside that window. The script clicked **File**, moved the pointer down the menu in steps to **External Data**, across to its submenu, and clicked **Report Missing Files**; each point was read off the previous run's picture (the File menu's last items read *External Data*, *Clean Up*, *Defaults*, *Quit*, and the External Data submenu's last two read *Report Missing Files* and *Find Missing Files…*). The terminal printed, at the click:

```
00:23.116  operator         | WARNING Path '/…/e15c/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
```

and the whole-window picture taken just after, following a forced redraw, shows a yellow warning triangle in the status bar followed by:

```
Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Reading a file/work/e15c/tex/probe.png' not found, from local data-block 'IMProbeLoaded'
```

**Tests 35 and 36 — the search, with no window, against fifteen arrangements of the search folder.** Recorded under question 10. In every arm `bpy.ops.file.find_missing_files(directory=…)` ran with no window and returned `{'FINISHED'}`; where it found nothing it printed `Warning: Could not find 'tex.png' in '<folder>'` and left the path as it was.

**Test 37 — six kinds of file gone at once, a missing linked blend-file among them: what opening, Report Missing Files and Find Missing Files do with no window, and what reopening the result shows.** In two arms, each in its own folder (`<WORK>/e37/find`, `<WORK>/e37/control`), with no window: `orig/lib.blend` was saved holding collection `LibCol` with a mesh object `LibCube`; an Alembic archive `orig/probe.abc` was exported from the factory startup scene with `bpy.ops.wm.alembic_export(…, selected=False, start=1, end=1)`; then `main.blend`, from an empty start and saved in the arm's folder, was given an 8 by 8 PNG written by Blender at `orig/image.png`, a tenth of a second of silence written by Python's `wave` module at `orig/sound.wav`, a copy of `/System/Library/Fonts/Supplemental/Arial.ttf` at `orig/font.ttf`, and a single-frame image sequence `orig/clip_0001.png`, each loaded as an image, sound, font and movie clip with a fake user; the archive brought in by `bpy.ops.wm.alembic_import(…, as_background_job=False, always_add_cache_reader=True)` with the imported objects deleted (`bpy.data.cache_files.load` does not exist: `AttributeError bpy_prop_collection: attribute "load" not found`); and `LibCol` linked from `orig/lib.blend` and put in the scene. Saved with `relative_remap=True`. Then all six files were moved: in the `find` arm to `search/found/`, in the `control` arm to `away/`, leaving `search/` empty.

A second run with no window opened `main.blend`, printed the six recorded locations and the library's missing flags, called `bpy.ops.file.report_missing_files()`, then `bpy.ops.file.find_missing_files(directory=<the search/ folder>)`, printed the six again and the flags again, and saved as `after.blend`. A third run opened `after.blend` and printed the library and its data-blocks. `find` arm:

```
00:00.320  reports          | Read library: '<WORK>/e37/find/orig/lib.blend', '//orig/lib.blend', parent '<direct>'
00:00.320  reports          | WARNING Unable to open '<WORK>/e37/find/orig/lib.blend': No such file or directory
00:00.320  reports          | Cannot find lib '<WORK>/e37/find/orig/lib.blend'
00:00.320  reports          | LIB: Collection: 'LibCol' missing from '<WORK>/e37/find/orig/lib.blend', parent '<direct>'
Could not open as Ogawa file from provided streams.
Unable to open <WORK>/e37/find/orig/probe.abc
00:00.321  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
HT| before | image = <WORK>/e37/find/orig/image.png
HT| before | sound = <WORK>/e37/find/orig/sound.wav
HT| before | font = ['<WORK>/e37/find/orig/font.ttf']
HT| before | movie clip = <WORK>/e37/find/orig/clip_0001.png
HT| before | linked blend = //orig/lib.blend
HT| before | alembic = //orig/probe.abc
HT| library is_missing = True | linked ids missing = [('LibCol', True)]
Warning: Path '<WORK>/e37/find/orig/probe.abc' not found, from local data-block 'CFprobe.abc'
Warning: Path '<WORK>/e37/find/orig/clip_0001.png' not found, from local data-block 'MCclip_0001.png'
Warning: Path '<WORK>/e37/find/orig/image.png' not found, from local data-block 'IMimage.png'
Warning: Path '<WORK>/e37/find/orig/sound.wav' not found, from local data-block 'SOsound.wav'
Warning: Path '<WORK>/e37/find/orig/font.ttf' not found, from local data-block 'VFArial Regular'
Warning: Path '<WORK>/e37/find/orig/lib.blend' not found, from local data-block 'LIlib.blend'
HT| report returned = {'FINISHED'}
HT| find returned = {'FINISHED'}
HT| after | image = <WORK>/e37/find/search/found/image.png
HT| after | sound = <WORK>/e37/find/search/found/sound.wav
HT| after | font = ['<WORK>/e37/find/search/found/font.ttf']
HT| after | movie clip = <WORK>/e37/find/search/found/clip_0001.png
HT| after | linked blend = //search/found/lib.blend
HT| after | alembic = //search/found/probe.abc
HT| library is_missing after find = True | linked ids missing = [('LibCol', True)]
Info: Saved as "after.blend"
HT| saved = {'FINISHED'}
```
```
00:00.332  reports          | Read library: '<WORK>/e37/find/search/found/lib.blend', '//search/found/lib.blend', parent '<direct>'
HT| reopened library = //search/found/lib.blend | is_missing = False
HT| reopened linked ids missing = [('LibCol', False), ('LibCube', False), ('LibMesh', False)]
HT| reopened objects = [('LibCube', False)]
```

`control` arm (nothing in `search/`) — the opening lines and the six `Warning: Path …` lines were the same as above with `control` in the folder name, then:

```
HT| report returned = {'FINISHED'}
Warning: Could not find 'probe.abc' in '<WORK>/e37/control/search'
Warning: Could not find 'clip_0001.png' in '<WORK>/e37/control/search'
Warning: Could not find 'image.png' in '<WORK>/e37/control/search'
Warning: Could not find 'sound.wav' in '<WORK>/e37/control/search'
Warning: Could not find 'font.ttf' in '<WORK>/e37/control/search'
Warning: Could not find 'lib.blend' in '<WORK>/e37/control/search'
HT| find returned = {'FINISHED'}
HT| after | image = <WORK>/e37/control/orig/image.png
HT| after | sound = <WORK>/e37/control/orig/sound.wav
HT| after | font = ['<WORK>/e37/control/orig/font.ttf']
HT| after | movie clip = <WORK>/e37/control/orig/clip_0001.png
HT| after | linked blend = //orig/lib.blend
HT| after | alembic = //orig/probe.abc
HT| library is_missing after find = True | linked ids missing = [('LibCol', True)]
HT| saved = {'FINISHED'}
HT| reopened library = //orig/lib.blend | is_missing = True
HT| reopened linked ids missing = [('LibCol', True)]
HT| reopened objects = []
```

(Reopening the control's `after.blend` printed the same four library warnings and the two Alembic lines as the first opening.) Report Missing Files named all six, the linked blend-file included, one line each. Find Missing Files rewrote all six in the `find` arm, each in the form it had before (four in full form, the linked blend-file and the archive in `//` form). In that same run the library and its collection still read `is_missing = True` after the rewrite; after saving and reopening, the library read `False` and its collection, object and mesh were there.

---

## 7. Can something a file points at be traced to the scene that uses it?

**Label:** documented that the needed building blocks exist in Blender's Python API. **Inferred** that putting them together traces a referenced file to a scene, and that Python functions work without a window. **Not in the sources:** whether Outliner can trace a file to a scene, whether maps tie to what renders, non-data-block references, and OS differences.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Building blocks.**
- From data-block to the file paths it uses: `BlendData.file_path_map()` "Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them." API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html, read 2026-10-11.
- From data-block to its users: `BlendData.user_map(*, subset=None, key_types=None, value_types=None)` "Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them." Same page.
- From a data-block downward, through everything it reaches: `bpy_extras.id_map_utils.get_id_reference_map()` "Return a dictionary of direct data-block references for every data-block in the blend file." and `get_all_referenced_ids(id, ref_map)` "Return a set of IDs directly or indirectly referenced by id." API, https://docs.blender.org/api/5.3/bpy_extras.id_map_utils.html, read 2026-10-11.
- For a linked file: `Library.users_id` "ID data-blocks that use this library". API, https://docs.blender.org/api/5.3/bpy.types.Library.html, read 2026-10-11.
- Pairwise check: `ID.user_of_id(id)` "Count the number of times that ID uses/references given one". API, https://docs.blender.org/api/5.3/bpy.types.ID.html, read 2026-10-11.

**The trace (inferred).** Two routes. (a) Start from a scene. Take everything it reaches with `get_all_referenced_ids`, then ask `file_path_map(subset=…)` for those data-blocks' paths. (b) Start from a path. Find the data-block holding it in `file_path_map`, then climb `user_map` until a scene is reached. Scenes are data-blocks, so they can appear in both maps. The pages do not say this in so many words.

**In the interface.** The Outliner's "Blender File" mode "Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number of users", and its "Scenes" mode "Shows the view layers, collections, and objects across all scenes." Manual, https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html, read 2026-10-11. The manual does not describe tracing a file to its scene in the Outliner.

**"Uses" is ambiguous.** The functions above follow references between data-blocks. Whether a reference leads to the thing appearing in a render (for example, through a collection excluded from a view layer, or a disabled modifier) is a different question. The sources do not tie these maps to what renders.

**Not covered:** references that are not data-block to data-block (for example, a compositor node naming another scene's view layer) and whether the maps include them; what a "user" counts in edge cases.

**Brought up to date by the hand tests of 2026-10-17 (recorded at the end of this answer).** Three edge cases were tested: a node group, a driver's target and a custom property each appear in the map as a user of the image (test 17); an image named only as a driver's target and carrying no fake user was not in the file after saving (test 17's first build); and an image used only in a collection excluded from its scene's view layer is still connected to that scene by the map, while the exclusion shows only in the view layer's own settings and in `visible_get()` (test 18). A compositor node naming another scene's view layer was not tested.

**Outside the interface:** these are Python functions. Running them without a window is **inferred** to work. The Outliner is interface-only.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window on the Creator's Mac: `bpy.data.user_map()`, the walk outwards from an image, `Object.users_scene`, `visible_get()` and `bpy.data.file_path_map()` all ran (tests 16, 17 and 18). The walk reached the scene that holds the object from a plainly used image, from images used only inside a node group, by a driver, or by a custom property, and from an image used only in a collection excluded from its scene's view layer; asking the map directly for scene-type users returned an empty set each time. Test 19 is the window's side: the Outliner's Scenes view, filtered by name, showed a linked object under the one scene that holds it.

**Operating system:** the sources do not say.

**Brought up to date by the hand tests of 2026-10-17.** Tests 16 to 19 below were run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro), so they settle macOS on Apple silicon for what they cover. No other machine was available for 5.3: Linux and Windows are untested for this version.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 16 — following an image up to the scene that uses it.** With no window, a file was built from an empty start holding two scenes, `SceneWithIt` and `SceneWithout`; a PNG `tex/UsedTex.png` loaded and used by an image texture node in material `ProbeMat` on a mesh object `Cube` that sits in `SceneWithIt` only; and a second PNG `tex/UnusedTex.png` loaded, used by nothing, and given a fake user so it survives the save. Saved with `relative_remap=True`. A second run with no window opened it, called `bpy.data.user_map()` once, and walked outwards from each image — at each step printing every data-block in the current set and its users, then moving to the users not seen before — until nothing new appeared. It also printed `users_scene` of each object whose material uses the image, `bpy.data.user_map(subset=[img], value_types={'SCENE'})`, and the map of data-blocks to paths (entries holding no path left out). What came back:

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

**Test 17 — whether a reference held only inside a node group, a driver or a custom property shows up in the map of what uses what.** With no window, a file was built from an empty start: one scene renamed `TheScene`; one mesh object `Holder` with material `HolderMat`; four images loaded from `assets/` — `Img_plain` used by an image node in the material's own node tree; `Img_group` used only by an image node inside node group `InnerGroup`, which the material uses through a group node; `Img_driver` named only as the target (`id_type='IMAGE'`, data path `size[0]`) of a driver on a custom property `drv` of the object; `Img_custom` held only by a custom property `img_ref` on the object that points at the data-block. Saved. A second run with no window opened it, called `bpy.data.user_map()` once, and printed for each image its users, whether the walk outwards reaches `TheScene` and in how many steps, the map asked directly for scene-type users, and the image's user count; then the map of data-blocks to paths.

The first build gave the images no fake user. Opened again, the walk stopped at the third image with this error, recorded as it came:

```
UM group   users = [('InnerGroup', 'ShaderNodeTree')]
UM group   scenes reached in 7 steps = ['TheScene']
UM group   user_map(value_types=SCENE) = {'Img_group': []}
UM group   users count = 1
UM plain   users = [('HolderMat', 'Material')]
UM plain   scenes reached in 6 steps = ['TheScene']
UM plain   user_map(value_types=SCENE) = {'Img_plain': []}
UM plain   users count = 1
Traceback (most recent call last):
  File "/…/e17_walk.py", line 8, in <module>
    img = bpy.data.images["Img_" + k]
          ~~~~~~~~~~~~~~~^^^^^^^^^^^^
KeyError: 'bpy_prop_collection[key]: key "Img_driver" not found'
```

So an image named only as a driver's target, with no fake user, was not in the saved file. The build was repaired by giving every image a fake user, and run again. What came back (the folder shortened to `/…/e17/`):

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
UM file_path_map = [('Img_custom', ['/…/e17/assets/custom.png']), ('Img_driver', ['/…/e17/assets/driver.png']), ('Img_group', ['/…/e17/assets/group.png']), ('Img_plain', ['/…/e17/assets/plain.png'])]
```

Each user count above includes the fake user.

**Test 18 — an image used only by an object in a collection that is excluded from its scene's view layer.** With no window, a file was built from an empty start: scene `SceneA` holding collection `ColA` with object `ObjA`, whose material `MatA` uses image `ImgA` in an image node; and scene `SceneB` holding collection `ColB` with object `ObjB`, whose material `MatB` uses image `ImgB`. `ColB` was then excluded from `SceneB`'s view layer by setting its layer collection's `exclude` to `True`. The first build stopped at that line, because the new scene's view layer had no layer collections yet:

```
Traceback (most recent call last):
  File "/…/e18_build.py", line 23, in <module>
    vlb.layer_collection.children["ColB"].exclude = True
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AttributeError: 'NoneType' object has no attribute 'children'
```

The build was repaired by calling the view layer's own `update()` first, and printed:

```
BUILT SceneB layer_collection before update = None
BUILT SceneB layer_collection after update = <bpy_struct, LayerCollection("Scene Collection") at 0xbd6735ab8>
BUILT SceneB ColB exclude = True
Info: Saved as "excl.blend"
SAVE {'FINISHED'}
```

(It also printed two `DeprecationWarning` lines saying `'Material.use_nodes' is expected to be removed in Blender 6.0`.) A second run with no window opened the file, walked outwards from each image as in test 16, then printed, for each scene's view layer, each top collection with its `exclude` setting and each object's `visible_get(view_layer=…)`; each scene's `collection.all_objects`; and each object's `users_scene`. What came back:

```
START ImgA
  STEP 1 'ImgA' Image -> [('MatA', 'Material')]
  STEP 2 'MatA' Material -> [('MeshA', 'Mesh')]
  STEP 3 'MeshA' Mesh -> [('ObjA', 'Object')]
  STEP 4 'ObjA' Object -> [('ColA', 'Collection'), ('SceneA', 'Scene')]
  STEP 5 'ColA' Collection -> [('SceneA', 'Scene')]
  STEP 6 'SceneA' Scene -> [('WinMan', 'WindowManager')]
  STEP 7 'WinMan' WindowManager -> []
  SCENES REACHED ['SceneA']
START ImgB
  STEP 1 'ImgB' Image -> [('MatB', 'Material')]
  STEP 2 'MatB' Material -> [('MeshB', 'Mesh')]
  STEP 3 'MeshB' Mesh -> [('ObjB', 'Object')]
  STEP 4 'ObjB' Object -> [('ColB', 'Collection')]
  STEP 5 'ColB' Collection -> [('SceneB', 'Scene')]
  STEP 6 'SceneB' Scene -> []
  SCENES REACHED ['SceneB']
LAYER SceneA ViewLayer [('ColA', False)]
  VISIBLE_GET ObjA in SceneA/ViewLayer = True
  VISIBLE_GET ObjB in SceneA/ViewLayer = False
ALL_OBJECTS SceneA ['ObjA']
LAYER SceneB ViewLayer [('ColB', True)]
  VISIBLE_GET ObjA in SceneB/ViewLayer = False
  VISIBLE_GET ObjB in SceneB/ViewLayer = False
ALL_OBJECTS SceneB ['ObjB']
USERS_SCENE ObjA ['SceneA']
USERS_SCENE ObjB ['SceneB']
```

The walk reached `SceneB` from `ImgB` through `ColB`. In the map, `ObjA` is used by its collection and by `SceneA` directly; `ObjB` only by its collection. The exclusion shows in `SceneB`'s view layer (`('ColB', True)`) and in `visible_get()`, which reads `False` for `ObjB` there; the map of what uses what carries neither.

**Test 19 — the Outliner's Scenes view, filtered to a linked object, in Blender's window.** With no window, `lib.blend` was saved holding one mesh object `LinkedCube`; then `main.blend` was saved holding scene `Scene_1`, into whose own collection `LinkedCube` was linked from `lib.blend`, and scene `Scene_2`, holding only a local mesh object `LocalThing`. `main.blend` was opened in Blender's window (`--no-window-focus -p 0 0 1280 800`). Before the window first drew, a script turned its biggest area into the Outliner, set its display mode to `'SCENES'` and typed `LinkedCube` into its search filter; later it opened the tree three levels, forced a redraw, and photographed the area with Blender's own screenshot command. What it printed:

```
OUT display_mode = SCENES | filter_text = 'LinkedCube'
OUT show_one_level {'FINISHED'}
OUT show_one_level {'FINISHED'}
OUT show_one_level {'FINISHED'}
OUT screenshot_area {'FINISHED'}
OUT LinkedCube users_scene = ['Scene_1']
```

What the picture shows, read off it: the search field reads `LinkedCube`; the tree holds **`Scene_1`** › **Objects** › **`LinkedCube`** (drawn with a chain icon beside the object icon), and under it **`LinkedCubeMesh`** (also with a chain icon), the last two rows highlighted green. **`Scene_2` does not appear in the filtered tree.**

---

## 8. Can a file's full chain of references be read while some of the referenced files are absent?

**Reading of the question.** Ambiguous. Reading A: the chain of linked `.blend` files (a file links a library, which links another, and so on). Reading B: every external file referenced anywhere along that chain, images included. I answer A mainly, and note B.

**Label:** documented that a file whose library is missing still opens, keeps placeholders for the missing data, and that each file's own library list can be read. **Inferred** that the part of the chain recorded only inside an absent file cannot be read. **Not in the sources:** whether the open file itself records links that lie deeper than an absent library.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**What survives an absent library.**
- "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost". Manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html, read 2026-10-11.
- `ID.is_missing` marks those placeholders. `ID.is_library_indirect` "Is this ID block linked indirectly". API, https://docs.blender.org/api/5.3/bpy.types.ID.html, read 2026-10-11.
- `Library.parent` exists ("(readonly) Type: Library | None") but has no description. API, https://docs.blender.org/api/5.3/bpy.types.Library.html, read 2026-10-11.

**Reading the chain one file at a time.**
- `bpy.data.libraries.load(filepath)` gives, for that file, "A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file." With a limit: "Packed linked data-blocks are not linkable and are not listed in the input libraries attribute." API, https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html, read 2026-10-11.
- **Inferred:** repeating this on each listed file walks the chain. A file that is absent cannot be loaded, so the libraries it alone lists cannot be found this way. The page does not say what `load()` does with a missing file.
- The library paths of the open file can be listed without the files being present. `file_path_map(include_libraries=True)` includes "library file paths of linked data". BlendData page, https://docs.blender.org/api/5.3/bpy.types.BlendData.html, read 2026-10-11. Report Missing Files exists precisely to list paths whose files are gone (question 6). That the listing does not need the files present is **inferred** from that.

**What the open file records about deeper links.**
- Release note on packed linked data, 5.3: "code writing linked data now only writes directly linked packed IDs … The indirectly linked ones are exclusively used by regular linked data from other libraries, so writing them in the current blend-file is useless and only wastes space (and adds clutter)." Release notes, https://developer.blender.org/docs/release_notes/5.3/core/, read 2026-10-11. This is about packed data only.
- A preference hints that outside tools have trouble with indirect links: "Forces all linked data to be considered as directly linked. Workaround for current issues/limitations in BAT (Blender studio pipeline tool)". API, https://docs.blender.org/api/5.3/bpy.types.PreferencesExperimental.html, read 2026-10-11. The issue itself is not described.
- Whether an ordinary (not packed) `.blend` file records anything about libraries linked by its libraries is **not in the sources**.

**Reading B.** Paths to images and other files inside an absent library cannot be read from that library (**inferred**, same reason). Whether placeholders keep any of those paths is **not in the sources**.

**Not covered:** what `libraries.load()` does when its file is absent; what `Library.parent` holds; whether a library linked only by an absent library shows up in `bpy.data.libraries` of the open file.

**Brought up to date by the hand tests of 2026-10-17 (recorded at the end of this answer and under question 1).** `libraries.load()` on an absent file raised `OSError … failed to open blend file`. `Library.parent` read `None` for every library, including one reached only through another library. A library linked only by an absent library did not appear in `bpy.data.libraries` at all (test 4), nor in the Outliner's Blender File view (test 5).

**Where I looked:** searched the 5.3 manual, API documentation and release notes for: missing, placeholder, broken, indirect, indirectly, is_missing, is_library_indirect, parent, libraries attribute, recursive, Asset Tracer, BAT. Read the Link & Append, Library Overrides, Packed Data pages, and the API pages for `Library`, `ID`, `BlendData`, `BlendDataLibraries`, `LibraryWeakReference`, `BlendImportContextLibrary`.

**Outside the interface:** all means above are Python. Running them without a window is **inferred**. Placeholders are created "while loading a blend-file", which also happens on the command line (**inferred**; not stated separately).

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: opening a file whose library is missing succeeded, printed warnings naming the missing file and data-block, and kept a library entry with its recorded path and `is_missing` set (test 4). Test 5 is the window's side.

**Operating system:** the sources do not say.

**Brought up to date by the hand tests of 2026-10-17.** Tests 4 and 5 settle macOS on Apple silicon for what they cover. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 4 — how far a chain of linked files reads when a file at the end, or in the middle, is gone.** The three files described under question 1, test 4: `c.blend` holds `ChainCube`; `b.blend` links it into its local collection `ChainCollection`; `a.blend` links `ChainCollection` from `b.blend`; all saved relative. `a.blend` was opened three times with no window — with everything present; with `c.blend` renamed to `c_gone.blend`; and with `c.blend` back and `b.blend` renamed to `b_gone.blend` — and each time the script printed, for each library, `(name, filepath, the parent's filepath or None, is_missing)`; the list `[(l.filepath, l.parent) for l in bpy.data.libraries]`; for each object `(name, is_missing, is_library_indirect, the library's filepath or None)`; the same for each collection without the indirect flag; and `bpy.utils.blend_paths()`. The folder path is shortened to `/…/e04/`.

With everything present:

```
00:00.339  blend            | Read blend: "/…/e04/a.blend"
00:00.342  reports          | Read library: '/…/e04/b.blend', '//b.blend', parent '<direct>'
00:00.345  reports          | Read library: '/…/e04/c.blend', '//c.blend', parent '<direct>'
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, False)]
PARENTS [('//b.blend', None), ('//c.blend', None)]
OBJECTS [('ChainCube', False, True, '//c.blend')]
COLLECTIONS [('ChainCollection', False, '//b.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With `c.blend` — the far end — renamed away:

```
00:00.327  blend            | Read blend: "/…/e04/a.blend"
00:00.331  reports          | Read library: '/…/e04/b.blend', '//b.blend', parent '<direct>'
00:00.333  reports          | Read library: '/…/e04/c.blend', '//c.blend', parent '<direct>'
00:00.333  reports          | WARNING Unable to open '/…/e04/c.blend': No such file or directory
00:00.333  reports          | Cannot find lib '/…/e04/c.blend'
00:00.334  reports          | LIB: Object: 'ChainCube' missing from '/…/e04/c.blend', parent '<direct>'
00:00.334  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, False), ('c.blend', '//c.blend', None, True)]
PARENTS [('//b.blend', None), ('//c.blend', None)]
OBJECTS [('ChainCube', True, True, '//c.blend')]
COLLECTIONS [('ChainCollection', False, '//b.blend')]
BLEND_PATHS ['//b.blend', '//c.blend']
```

With `b.blend` — the middle — renamed away instead, and `c.blend` present:

```
00:00.335  blend            | Read blend: "/…/e04/a.blend"
00:00.338  reports          | Read library: '/…/e04/b.blend', '//b.blend', parent '<direct>'
00:00.338  reports          | WARNING Unable to open '/…/e04/b.blend': No such file or directory
00:00.338  reports          | Cannot find lib '/…/e04/b.blend'
00:00.339  reports          | LIB: Collection: 'ChainCollection' missing from '/…/e04/b.blend', parent '<direct>'
00:00.339  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
LIBRARIES [('b.blend', '//b.blend', None, True)]
PARENTS [('//b.blend', None)]
OBJECTS []
COLLECTIONS [('ChainCollection', True, '//b.blend')]
BLEND_PATHS ['//b.blend']
```

Each run's Blender also printed its version line, `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`, among these lines; it is left out above. In every run `Library.parent` came back `None`. With the middle file gone, nothing about `c.blend` was listed anywhere. What `bpy.data.libraries.load()` offers for a present file, and its error for an absent one, are recorded under question 1, test 4.

**Test 5 — the same chain in the Outliner's Blender File view, in Blender's window.** `a.blend` from test 4, with `b.blend` still renamed away, was opened in Blender's window (started from the terminal with `--no-window-focus -p 0 0 1280 800`). A script turned the window's biggest area into the Outliner, set its display mode to `'LIBRARIES'` (the Blender File view), opened its tree three levels with `bpy.ops.outliner.show_one_level(open=True)`, and photographed the area with Blender's own `bpy.ops.screen.screenshot_area`. What the terminal printed while opening, and from the script:

```
00:00.538  blend.readfile   | Read library: '/…/e04/b.blend', '//b.blend', parent '<direct>'
00:00.538  reports          | WARNING Unable to open '/…/e04/b.blend': No such file or directory
00:00.538  blend.readfile   | Cannot find lib '/…/e04/b.blend'
00:00.539  blend.readfile   | LIB: Collection: 'ChainCollection' missing from '/…/e04/b.blend', parent '<direct>'
00:00.543  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
OUTL display_mode = LIBRARIES
OUTL show_one_level {'FINISHED'}
OUTL show_one_level {'FINISHED'}
OUTL show_one_level {'FINISHED'}
OUTL screenshot_area {'FINISHED'}
OUTL libraries [('b.blend', '//b.blend', True)]
```

The first attempt at this photograph showed Blender's first-start *Quick Setup* screen over a 3D Viewport, because the throwaway settings folder held no saved preferences; a preferences file was saved into that folder with no window (`bpy.ops.wm.save_userpref()`) and the run repeated. What the repeated picture shows, read off it: a tree headed **Current File**; under it **Libraries**, holding one entry, `//b.blend`, with a warning triangle drawn at the far left of its row; then **Scenes** › `Scene` › **View Layers** › `ViewLayer`, and **Scene Collection** › `ChainCollection`, drawn with the linked-data icon beside the collection icon, and a user count of 1 at the right; then **Objects**, **Screens** and **Workspaces**. **No entry for `c.blend` appears anywhere in the picture.** A tooltip reading *Jump Time by Delta* is drawn at the bottom edge, where the mouse pointer happened to rest; nothing was clicked.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**Label:** documented that each means below exists, with the reach its page states. The pages mostly describe reach in general words ("external files", "all paths"). **Inferred** for `file_path_foreach` default flag behavior, Remap Relative leaving absolute paths untouched, unpack rewriting recorded paths, and Python functions working without a window. **Not in the sources:** an exact, kind-by-kind list for any of them, and whether any of them includes output places such as the render output path (question 4).

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

All pages read 2026-10-11 unless marked.

1. **`bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)`** — lists only.
   - Reach: "paths to external files referenced by the loaded .blend file". Options: "absolute — When true the paths returned are made absolute."; "packed — When true include file paths for packed data."; "local — When true skip linked library paths."
   - https://docs.blender.org/api/5.3/bpy.utils.html
2. **`BlendData.file_path_map(*, subset=None, key_types=None, include_libraries=False)`** — lists, grouped by data-block.
   - Reach: "all file paths used by" every data-block. "include_libraries (bool) – Include library file paths of linked data. False by default." "key_types … filter the keys mapped by ID types."
   - https://docs.blender.org/api/5.3/bpy.types.BlendData.html
3. **`BlendData.file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})`** — lists and rewrites ("the visited file path will be replaced with the returned string").
   - The flag list is the most detailed statement of kinds in the sources (https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html):
     - "SKIP_LINKED … Skip paths of linked IDs."
     - "SKIP_PACKED … Skip paths when their matching data is packed."
     - "SKIP_WEAK_REFERENCES … Skip weak reference paths. Those paths are typically ‘nice to have’ extra information, but are not used as actual source of data by the current .blend file."
     - "SKIP_MULTIFILE … Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches."
     - "RESOLVE_TOKEN … Currently only used for UDIM tiles."
     - "EXPAND_TOKENS … Currently only used for UDIM tiles."
     - "EXPAND_SEQUENCES … Expand image and volume sequences, invoking the callback once per file on disk."
     - "EXPAND_CACHES … Visit cache files, like texture cache files associated with images."
   - **Inferred** from these flags: by default it visits linked data-blocks' paths, multi-file paths (sequence strip images, point caches) and UDIM paths, and skips packed and weak-reference paths. Cache, expanded and read-only paths are visited but "can not be edited" (`BlendDataPathMeta`, https://docs.blender.org/api/5.3/bpy.types.BlendDataPathMeta.html).
   - A "weak reference" is described as "Read-only external reference to a linked data-block and its library file". https://docs.blender.org/api/5.3/bpy.types.LibraryWeakReference.html
4. **Make Paths Relative / Make Paths Absolute** (File ‣ External Data; `bpy.ops.file.make_paths_relative()` / `make_paths_absolute()`) — rewrite.
   - Reach: "Make all paths to external files relative to the current blend-file." / "absolute (= full path from the system’s root)."
   - https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html ; https://docs.blender.org/api/5.3/bpy.ops.file.html
5. **Report Missing Files** (`bpy.ops.file.report_missing_files()`) — lists missing ones.
   - Reach: "links to unpacked files that no longer exist". Topbar page.
6. **Find Missing Files** (`bpy.ops.file.find_missing_files()`) — searches a folder and rewrites.
   - Reach: "Every missing file found in the search will be recovered", written "as absolute paths". With `find_all`: "Find all files in the search path (not just missing)". Topbar page; bpy.ops.file page.
7. **Save / Save As with Remap Relative** (`bpy.ops.wm.save_as_mainfile(relative_remap=True)`, also on `save_mainfile`) — rewrites while saving to another folder.
   - Reach: "This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location." https://docs.blender.org/manual/en/5.3/files/blend/open_save.html
   - API: "Remap relative paths when saving to a different directory". https://docs.blender.org/api/5.3/bpy.ops.wm.html
   - Absolute paths are not mentioned. **Inferred:** they are not changed.
8. **`bpy.data.libraries.write(filepath, datablocks, *, path_remap='NONE', …)`** — rewrites paths in a new file it writes.
   - Reach: "NONE No path manipulation (default). RELATIVE Remap paths that are already relative to the new location. RELATIVE_ALL Remap all paths to be relative to the new location. ABSOLUTE Make all paths absolute on writing."
   - https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html
9. **`bpy.data.libraries.load(filepath)`, input `libraries`** — lists, for a file that is not open.
   - Reach: library files only. "a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive)"; "Packed linked data-blocks are not linkable and are not listed in the input libraries attribute." Same page.
10. **Relocate a linked file** (Outliner context menu ‣ Relocate; `bpy.ops.wm.lib_relocate()`; `bpy.ops.outliner.lib_operation(type='RELOCATE')` "Select a new path for this library, and reload all its data") — rewrites one library's location.
    - https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html ; https://docs.blender.org/api/5.3/bpy.ops.wm.html ; https://docs.blender.org/api/5.3/bpy.ops.outliner.html
    - A per-data-block form exists: `bpy.ops.wm.id_linked_relocate()`, "Currently only designed as an internal operator, not directly exposed to the user". bpy.ops.wm page.
11. **Each data-block's own path setting** (for example `Image.filepath`, `Library.filepath`, `Sound.filepath`, `PointCache.filepath`) — read and write one location each, in Python or in the path field shown in the interface.
    - `Library.filepath` warns against changing it (quoted in question 5). `Image.filepath_raw` changes the path "without data refreshing".
12. **Outliner, Blender File mode** — lists linked files "along with the path of the blend-file they originate from". Link & Append page.
13. **Unpack Resources** (`bpy.ops.file.unpack_all(method=…)`) — writes packed files out to disk.
    - "Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ‘’textures’’ for instance)." https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html
    - **Inferred:** this changes where those data-blocks point. The page does not say outright that the recorded path is rewritten.
14. **Outside Blender: Blender Asset Tracer.** Named as "tooling that can manipulate blend-files outside of Blender itself". https://docs.blender.org/manual/en/5.3/files/asset_libraries/introduction.html
    - Its reach is **not in the sources**.

**Not covered:** whether output places (render output, File Output node, bake folders) are reached by numbers 1–7; whether Report and Find Missing Files cover library files; whether Make Paths Relative changes linked libraries' paths, cache paths or weak references; what Blender Asset Tracer reaches.

**Brought up to date by the hand tests of 2026-10-17 (recorded at the end of this answer).** Output places: none of the listing means 1 to 3 carried the render output, bake output, File Output folder, cloth point cache or geometry-nodes bake folder, and Make Paths Relative and Absolute left all five unchanged (tests 10, 20 to 22, 28). Report and Find Missing Files covered a missing linked library (test 37). Make Paths Relative changed the linked library's path to `//lib.blend` (test 21); the factory material's weak-reference path was in `//` form before and after it, and Make Paths Absolute turned it into full form (test 28). No cache path was in the file. Blender Asset Tracer was not tested.

**Where I looked:** searched the 5.3 manual, API documentation and release notes for: blend_paths, file_path_map, file_path_foreach, make_paths, relative_remap, path_remap, remap, relocate, external data, missing, weak reference, Asset Tracer, unpack. Read every page cited above in full.

**Outside the interface:** numbers 1, 2, 3, 8, 9 and 11 are Python and need no window (**inferred** for running without a window). Numbers 4–7, 10 and 13 are operators that can be called from Python (documented on the API operator pages); how those designed around a file browser behave without a window is **not in the sources**. Number 12 is interface-only.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: every means numbered here that has a Python form ran — the listings, both sweeps, Report and Find Missing Files, Save As with Remap Relative, the write call, the loader, Relocate, each path setting, and Unpack (tests 1, 10, 13, 14, 20 to 24, 26 to 33, 37, 38). `image.replace` and `sequencer.change_path` needed their editor and context supplied, with or without a window (tests 33 and 34). The Outliner's views are the window's side (tests 5, 19, 25).

**Operating system:** the sources do not say whether any means behaves differently by system. See question 5 for the path helpers that mention Windows.

**Brought up to date by the hand tests of 2026-10-17.** These tests settle macOS on Apple silicon for what they cover. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 1 — the one-pass visitor's switches and the write call's remap choices.** With no file opened (the factory startup file in memory), `bpy.data.file_path_foreach` was called once per switch with a function that returns nothing, each switch on its own; then once with its switches emptied and a function that records the types of the three things it is handed; then `bpy.data.libraries.write()` was called with no data-blocks and each `path_remap` choice, plus the value `False`. What came back:

```
PROBE file_path_foreach flag SKIP_LINKED = ACCEPTED
PROBE file_path_foreach flag SKIP_PACKED = ACCEPTED
PROBE file_path_foreach flag SKIP_WEAK_REFERENCES = ACCEPTED
PROBE file_path_foreach flag SKIP_MULTIFILE = ACCEPTED
PROBE file_path_foreach flag RELOAD_EDITED = ACCEPTED
PROBE file_path_foreach flag RESOLVE_TOKEN = ACCEPTED
PROBE file_path_foreach flag EXPAND_TOKENS = ACCEPTED
PROBE file_path_foreach flag EXPAND_SEQUENCES = ACCEPTED
PROBE file_path_foreach flag EXPAND_CACHES = ACCEPTED
PROBE callback arg types = [('Material', 'str', 'BlendDataPathMeta')]
PROBE path_remap 'NONE' = ACCEPTED
PROBE path_remap 'RELATIVE' = ACCEPTED
PROBE path_remap 'RELATIVE_ALL' = ACCEPTED
PROBE path_remap 'ABSOLUTE' = ACCEPTED
PROBE path_remap False = REJECTED ERROR ValueError expected a string, got bool
```

The one call with its switches emptied was handed a material from the factory startup file, a path as text, and a `BlendDataPathMeta`. Which calls and commands exist is recorded under question 5.

**Test 10 — whether the whole-file listing means carry the render output path or a File Output node's folder.** On a file whose render output path is `//scene_out/frame_` and whose one File Output node writes to `//out/`, `bpy.utils.blend_paths()` listed only a file inside the Blender application, `bpy.data.file_path_map()` gave no data-block any path (`{}`), and `bpy.data.file_path_foreach()` with its default switches visited nothing (`[]`). The full record is under question 4, test 10; tests 20 to 23 below put many more kinds of path, outputs among them, through the same means.

**Test 20 — the many-paths file: every kind read by name, every listing means, and a walk over the property marks.** With no window, two files were built in an empty folder (written `<D>` below). First `lib.blend`, holding collection `LibColl` with a mesh object `LibCube` (mesh `LibMesh`). Then `many.blend`, from the factory startup file, saved once in the folder and then given these recorded locations, each written in full form from the drive root and pointing into `assets/` beside it unless said otherwise: an image `ProbeImage` loaded from a 4 by 4 PNG (`probe.png`); a sound `ProbeSound` from a tenth of a second of silence written by Python's `wave` module (`probe.wav`); a font `ProbeFont` loaded from a copy of `/System/Library/Fonts/Supplemental/Arial.ttf` named `probe.woff2`; a text `ProbeText` loaded from `probe.txt`; a volume `ProbeVolume` with its path set to `probe.vdb` (no such file made); a movie clip `ProbeClip` loaded from `probe.png`; an Alembic cache file from `probe.abc`, exported from the scene with `bpy.ops.wm.alembic_export(…, selected=False, start=1, end=1)` and brought in by `bpy.ops.wm.alembic_import(…, as_background_job=False, always_add_cache_reader=True)` (the objects it imported were deleted; `bpy.data.cache_files.load` was tried first and the build printed `BUILD cache_files.load AttributeError bpy_prop_collection: attribute "load" not found`); `LibColl` linked from `lib.blend` and put in the scene; the render output path `render_out/`; the bake output path `bake_out/`; a compositor node group `ProbeComp` given to the scene, holding a File Output node with folder `node_out/` and file name `{blend_name}`; a cloth modifier's point-cache path `pointcache/` on object `ClothObj`; a fluid domain's cache folder `fluidcache/` on `FluidObj`; a geometry-nodes modifier's bake folder `gnbake/` on `GNObj`; on object `ModObj`, a multi-resolution modifier's path (set to `multires.btx`), a mesh-cache modifier's file `meshcache.mdd`, an ocean modifier's folder `oceanbake/`, and a custom property `probe_path` holding the text `<D>/assets/custom_prop.png`; material `ProbeMat` with an OSL script node in External mode at `probe.osl` and an IES node in External mode at `probe.ies` (no such file made); camera data `ProbeCam` of type `'CUSTOM'` with its own shader path `camshader.osl`; and a sequencer image strip from `stripdir/probe.png`, with the sequencer's proxy storage set to `'PROJECT'` and its proxy folder to `proxydir/`. Every data-block that nothing else uses was given a fake user. Saved with `relative_remap=False`. The build printed, among other lines:

```
BUILD cache file route = alembic_import, imported objects removed
BUILD cloth point cache filepath reads '<D>/assets/pointcache/'
BUILD multires filepath reads ''
BUILD files on disk ['assets/camshader.osl', 'assets/probe.abc', 'assets/probe.osl', 'assets/probe.png', 'assets/probe.txt', 'assets/probe.wav', 'assets/probe.woff2', 'assets/stripdir/probe.png', 'lib.blend', 'many.blend', 'many.blend1']
Alembic import //assets/probe.abc took 1.91 ms
```

So the multi-resolution modifier's path took no value and stayed empty, and the Alembic cache file's path came in as `//assets/probe.abc` rather than in full form. A second run with no window opened `many.blend`. It read every kind by name (`READ`); called `bpy.utils.blend_paths()` with each of its switches, `bpy.data.file_path_map()` with and without `include_libraries=True` (all paths together, then per data-block), and `bpy.data.file_path_foreach()` with its default switches and with its switches emptied, printing what each visit was handed (`LIST`); and walked thirteen of `bpy.data`'s collections up to four levels deep through their properties, collecting every non-empty text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries `is_path_output` or `is_path_supports_blend_relative`, then listed every object's custom properties separately (`SWEEP`). `<APP>` stands for `/Applications/Blender 5.3b.app/Contents/Resources`. Everything it printed:

```
00:00.332  reports          | Read library: '<D>/lib.blend', '<D>/lib.blend', parent '<direct>'
00:00.338  object.modifier  | WARNING Object: "ModObj", Modifier: "MeshCache", No such file or directory
00:00.338  object.modifier  | WARNING Object: "GNObj", Modifier: "GN", Node group must have a group output node
00:00.339  physics.fluid    | ERROR Fluid initialization failed. Should not happen!
READ | Image.filepath                           | <D>/assets/probe.png
READ | Image.filepath_raw                       | <D>/assets/probe.png
READ | Sound.filepath                           | <D>/assets/probe.wav
READ | VectorFont.filepath                      | <D>/assets/probe.woff2
READ | Text.filepath                            | <D>/assets/probe.txt
READ | Volume.filepath                          | <D>/assets/probe.vdb
READ | MovieClip.filepath                       | <D>/assets/probe.png
READ | CacheFile.filepath                       | //assets/probe.abc
READ | Library.filepath                         | <D>/lib.blend
READ | RenderSettings.filepath                  | <D>/assets/render_out/
READ | BakeSettings.filepath                    | <D>/assets/bake_out/
READ | CompositorNodeOutputFile base path       | <D>/assets/node_out/
READ | PointCache.filepath (cloth)              | <D>/assets/pointcache/
READ | FluidDomainSettings.cache_directory      | <D>/assets/fluidcache/
READ | NodesModifier.bake_directory             | <D>/assets/gnbake/
READ | MultiresModifier.filepath                | 
READ | MeshCacheModifier.filepath               | <D>/assets/meshcache.mdd
READ | OceanModifier.filepath                   | <D>/assets/oceanbake/
READ | ShaderNodeScript.filepath                | <D>/assets/probe.osl
READ | ShaderNodeTexIES.filepath                | <D>/assets/probe.ies
READ | Camera.custom_filepath                   | <D>/assets/camshader.osl
READ | custom property on an object             | <D>/assets/custom_prop.png
READ | ImageStrip.directory                     | <D>/assets/stripdir/
READ | SequenceEditor.proxy_dir                 | <D>/assets/proxydir/
LIST blend_paths()                 = ['<D>/assets/stripdir/probe.png', '<D>/assets/fluidcache/', '<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/', '<D>/assets/camshader.osl', '<D>/assets/probe.vdb', '//assets/probe.abc', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<D>/assets/probe.osl', '<D>/assets/probe.ies', '<D>/assets/probe.png', '<D>/assets/probe.png', '<D>/assets/probe.wav', '<D>/assets/probe.txt', '<D>/assets/probe.woff2', '<D>/lib.blend']
LIST blend_paths(packed=True)      = ['<D>/assets/stripdir/probe.png', '<D>/assets/fluidcache/', '<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/', '<D>/assets/camshader.osl', '<D>/assets/probe.vdb', '//assets/probe.abc', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<D>/assets/probe.osl', '<D>/assets/probe.ies', '<D>/assets/probe.png', '<D>/assets/probe.png', '<D>/assets/probe.wav', '<D>/assets/probe.txt', '<D>/assets/probe.woff2', '<D>/lib.blend']
LIST blend_paths(local=True)       = ['<D>/assets/stripdir/probe.png', '<D>/assets/fluidcache/', '<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/', '<D>/assets/camshader.osl', '<D>/assets/probe.vdb', '//assets/probe.abc', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<D>/assets/probe.osl', '<D>/assets/probe.ies', '<D>/assets/probe.png', '<D>/assets/probe.png', '<D>/assets/probe.wav', '<D>/assets/probe.txt', '<D>/assets/probe.woff2', '<D>/lib.blend']
LIST blend_paths(absolute=True)    = ['<D>/assets/stripdir/probe.png', '<D>/assets/fluidcache/', '<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/', '<D>/assets/camshader.osl', '<D>/assets/probe.vdb', '<D>/assets/probe.abc', '<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<D>/assets/probe.osl', '<D>/assets/probe.ies', '<D>/assets/probe.png', '<D>/assets/probe.png', '<D>/assets/probe.wav', '<D>/assets/probe.txt', '<D>/assets/probe.woff2', '<D>/lib.blend']
LIST file_path_map()               = ['//assets/probe.abc', '<D>/assets/camshader.osl', '<D>/assets/fluidcache/', '<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/', '<D>/assets/probe.ies', '<D>/assets/probe.osl', '<D>/assets/probe.png', '<D>/assets/probe.txt', '<D>/assets/probe.vdb', '<D>/assets/probe.wav', '<D>/assets/probe.woff2', '<D>/assets/stripdir/probe.png', '<D>/lib.blend']
LIST file_path_map(include_libraries=True) = ['//assets/probe.abc', '<D>/assets/camshader.osl', '<D>/assets/fluidcache/', '<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/', '<D>/assets/probe.ies', '<D>/assets/probe.osl', '<D>/assets/probe.png', '<D>/assets/probe.txt', '<D>/assets/probe.vdb', '<D>/assets/probe.wav', '<D>/assets/probe.woff2', '<D>/assets/stripdir/probe.png', '<D>/lib.blend']
LIST file_path_map by data-block   = [('FluidObj', ['<D>/assets/fluidcache/']), ('LibColl', ['<D>/lib.blend']), ('LibCube', ['<D>/lib.blend']), ('LibMesh', ['<D>/lib.blend']), ('ModObj', ['<D>/assets/meshcache.mdd', '<D>/assets/oceanbake/']), ('ProbeCam', ['<D>/assets/camshader.osl']), ('ProbeClip', ['<D>/assets/probe.png']), ('ProbeFont', ['<D>/assets/probe.woff2']), ('ProbeImage', ['<D>/assets/probe.png']), ('ProbeMat', ['<D>/assets/probe.ies', '<D>/assets/probe.osl']), ('ProbeSound', ['<D>/assets/probe.wav']), ('ProbeText', ['<D>/assets/probe.txt']), ('ProbeVolume', ['<D>/assets/probe.vdb']), ('Scene', ['<D>/assets/stripdir/probe.png']), ('lib.blend', ['<D>/lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
LIST foreach[default] CacheFile 'probe.abc' //assets/probe.abc ro=False cache=False exp=False
LIST foreach[default] Camera 'ProbeCam' <D>/assets/camshader.osl ro=False cache=False exp=False
LIST foreach[default] Image 'ProbeImage' <D>/assets/probe.png ro=False cache=False exp=False
LIST foreach[default] Library 'lib.blend' <D>/lib.blend ro=False cache=False exp=False
LIST foreach[default] MovieClip 'ProbeClip' <D>/assets/probe.png ro=False cache=False exp=False
LIST foreach[default] Object 'FluidObj' <D>/assets/fluidcache/ ro=False cache=False exp=False
LIST foreach[default] Object 'ModObj' <D>/assets/meshcache.mdd ro=False cache=False exp=False
LIST foreach[default] Object 'ModObj' <D>/assets/oceanbake/ ro=False cache=False exp=False
LIST foreach[default] Scene 'Scene' <D>/assets/stripdir/probe.png ro=False cache=False exp=False
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' <D>/assets/probe.ies ro=False cache=False exp=False
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' <D>/assets/probe.osl ro=False cache=False exp=False
LIST foreach[default] Sound 'ProbeSound' <D>/assets/probe.wav ro=False cache=False exp=False
LIST foreach[default] Text 'ProbeText' <D>/assets/probe.txt ro=False cache=False exp=False
LIST foreach[default] VectorFont 'ProbeFont' <D>/assets/probe.woff2 ro=False cache=False exp=False
LIST foreach[default] Volume 'ProbeVolume' <D>/assets/probe.vdb ro=False cache=False exp=False
LIST foreach[default] COUNT 15
LIST foreach[flags=set()] CacheFile 'probe.abc' //assets/probe.abc ro=False cache=False exp=False
LIST foreach[flags=set()] Camera 'ProbeCam' <D>/assets/camshader.osl ro=False cache=False exp=False
LIST foreach[flags=set()] Image 'ProbeImage' <D>/assets/probe.png ro=False cache=False exp=False
LIST foreach[flags=set()] Library 'lib.blend' <D>/lib.blend ro=False cache=False exp=False
LIST foreach[flags=set()] Material 'Material' //../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend ro=False cache=False exp=False
LIST foreach[flags=set()] MovieClip 'ProbeClip' <D>/assets/probe.png ro=False cache=False exp=False
LIST foreach[flags=set()] Object 'FluidObj' <D>/assets/fluidcache/ ro=False cache=False exp=False
LIST foreach[flags=set()] Object 'ModObj' <D>/assets/meshcache.mdd ro=False cache=False exp=False
LIST foreach[flags=set()] Object 'ModObj' <D>/assets/oceanbake/ ro=False cache=False exp=False
LIST foreach[flags=set()] Scene 'Scene' <D>/assets/stripdir/probe.png ro=False cache=False exp=False
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' <D>/assets/probe.ies ro=False cache=False exp=False
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' <D>/assets/probe.osl ro=False cache=False exp=False
LIST foreach[flags=set()] Sound 'ProbeSound' <D>/assets/probe.wav ro=False cache=False exp=False
LIST foreach[flags=set()] Text 'ProbeText' <D>/assets/probe.txt ro=False cache=False exp=False
LIST foreach[flags=set()] VectorFont 'ProbeFont' <D>/assets/probe.woff2 ro=False cache=False exp=False
LIST foreach[flags=set()] Volume 'ProbeVolume' <D>/assets/probe.vdb ro=False cache=False exp=False
LIST foreach[flags=set()] COUNT 16
SWEEP COUNT 28
SWEEP distinct values 22
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = //../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.cache_files['probe.abc'].filepath = //assets/probe.abc
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = <D>/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = <D>/assets/camshader.osl
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = <D>/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = <D>/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = <D>/assets/meshcache.mdd
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].inputs[].node.directory = <D>/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = <D>/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = <D>/assets/pointcache/
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = <D>/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = <D>/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = <D>/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = <D>/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = <D>/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = <D>/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = <D>/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = <D>/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.strips[].directory = <D>/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = <D>/lib.blend
SWEEP bpy.data.materials['Material'].library_weak_reference.id_name = MAMaterial
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].inputs[].node.file_name = {blend_name}
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = <D>/assets/custom_prop.png
```

The walk over the marks reached every recorded location in the file but two: the multi-resolution modifier's, which is empty, and the text's, whose path carries neither a path subtype nor a path mark (test 1 printed `PROBE Text.filepath subtype=NONE …` with both marks `False`); the custom property turned up only in the separate listing of custom properties.

**Test 21 — what Make Paths Relative reaches, and the same file read back.** Test 20's `many.blend` was opened with no window and `bpy.ops.file.make_paths_relative()` called; every kind was read again by name and the file saved as `many_rel.blend` beside it. What came back (`<D>` is the folder):

```
Info: Total files 16 | Changed 14 | Failed 0
OP make_paths_relative = {'FINISHED'}
AFTER | Image.filepath                           | //assets/probe.png
AFTER | Image.filepath_raw                       | //assets/probe.png
AFTER | Sound.filepath                           | //assets/probe.wav
AFTER | VectorFont.filepath                      | //assets/probe.woff2
AFTER | Text.filepath                            | //assets/probe.txt
AFTER | Volume.filepath                          | //assets/probe.vdb
AFTER | MovieClip.filepath                       | //assets/probe.png
AFTER | CacheFile.filepath                       | //assets/probe.abc
AFTER | Library.filepath                         | //lib.blend
AFTER | RenderSettings.filepath                  | <D>/assets/render_out/
AFTER | BakeSettings.filepath                    | <D>/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | <D>/assets/node_out/
AFTER | PointCache.filepath (cloth)              | <D>/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | <D>/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | <D>/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | SequenceEditor.proxy_dir                 | <D>/assets/proxydir/
Info: Saved as "many_rel.blend"
SAVE many_rel {'FINISHED'}
```

Read off without adding to it: the recorded text changed to `//` form for Image, Sound, VectorFont, Text, Volume, MovieClip, Library, the fluid cache folder, the mesh-cache file, the ocean folder, the OSL script node, the IES node, the camera's shader path and the image strip's folder (the cache file's path was `//` form already). It stayed in full form for the render output path, the bake output path, the File Output folder, the cloth point cache, the geometry-nodes bake folder, the sequencer's proxy folder and the custom property.

A further run with no window opened `many_rel.blend` and read every kind again with this folder written as `<HERE>`, then the flat list, the map of data-blocks to paths, the separator Python uses, and whether the image's own file is found when its name is asked for in capitals — `os.path.exists()` on `<HERE>/assets/PROBE.PNG`, where the file on disk is `probe.png`, followed by the folder's own listing of every name that matches `probe.png` in any spelling. `<APP>` stands for `/Applications/Blender 5.3b.app/Contents/Resources`. What came back:

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
CROSS | SequenceEditor.proxy_dir                 | <HERE>/assets/proxydir/
CROSS blend_paths   = ['//assets/stripdir/probe.png', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/camshader.osl', '//assets/probe.vdb', '//assets/probe.abc', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/probe.osl', '//assets/probe.ies', '//assets/probe.png', '//assets/probe.png', '//assets/probe.wav', '//assets/probe.txt', '//assets/probe.woff2', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
CASE asked for <HERE>/assets/PROBE.PNG
CASE real name ['probe.png']
```

**Test 22 — what Make Paths Absolute reaches.** Test 21's `many_rel.blend` was opened with no window and `bpy.ops.file.make_paths_absolute()` called; every kind was read again. What came back:

```
Info: Total files 16 | Changed 16 | Failed 0
OP make_paths_absolute = {'FINISHED'}
AFTER | Image.filepath                           | <D>/assets/probe.png
AFTER | Image.filepath_raw                       | <D>/assets/probe.png
AFTER | Sound.filepath                           | <D>/assets/probe.wav
AFTER | VectorFont.filepath                      | <D>/assets/probe.woff2
AFTER | Text.filepath                            | <D>/assets/probe.txt
AFTER | Volume.filepath                          | <D>/assets/probe.vdb
AFTER | MovieClip.filepath                       | <D>/assets/probe.png
AFTER | CacheFile.filepath                       | <D>/assets/probe.abc
AFTER | Library.filepath                         | <D>/lib.blend
AFTER | RenderSettings.filepath                  | <D>/assets/render_out/
AFTER | BakeSettings.filepath                    | <D>/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | <D>/assets/node_out/
AFTER | PointCache.filepath (cloth)              | <D>/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | <D>/assets/fluidcache/
AFTER | NodesModifier.bake_directory             | <D>/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | <D>/assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | <D>/assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | <D>/assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | <D>/assets/probe.ies
AFTER | Camera.custom_filepath                   | <D>/assets/camshader.osl
AFTER | custom property on an object             | <D>/assets/custom_prop.png
AFTER | ImageStrip.directory                     | <D>/assets/stripdir/
AFTER | SequenceEditor.proxy_dir                 | <D>/assets/proxydir/
```

The kinds that test 21 put in `//` form all came back in full form; the seven test 21 left alone had stayed in full form throughout.

**Test 23 — the one-pass path visitor, and whether returning a string rewrites.** Test 20's `many.blend` was opened with no window and `bpy.data.file_path_foreach()` called with its switches emptied (`flags=set()`) and a function that returns each path with `/assets/` replaced by `/MOVED/`, recording what it was handed; then every kind was read again. What came back:

```
FEW returned = None
FEW visited ('CacheFile', 'probe.abc', '//assets/probe.abc', '//MOVED/probe.abc', False)
FEW visited ('Camera', 'ProbeCam', '<D>/assets/camshader.osl', '<D>/MOVED/camshader.osl', False)
FEW visited ('Image', 'ProbeImage', '<D>/assets/probe.png', '<D>/MOVED/probe.png', False)
FEW visited ('Library', 'lib.blend', '<D>/lib.blend', '<D>/lib.blend', False)
FEW visited ('Material', 'Material', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../..<APP>/5.3/datafiles/MOVED/brushes/essentials_brushes-gp_draw.blend', False)
FEW visited ('MovieClip', 'ProbeClip', '<D>/assets/probe.png', '<D>/MOVED/probe.png', False)
FEW visited ('Object', 'FluidObj', '<D>/assets/fluidcache/', '<D>/MOVED/fluidcache/', False)
FEW visited ('Object', 'ModObj', '<D>/assets/meshcache.mdd', '<D>/MOVED/meshcache.mdd', False)
FEW visited ('Object', 'ModObj', '<D>/assets/oceanbake/', '<D>/MOVED/oceanbake/', False)
FEW visited ('Scene', 'Scene', '<D>/assets/stripdir/probe.png', '<D>/MOVED/stripdir/probe.png', False)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '<D>/assets/probe.ies', '<D>/MOVED/probe.ies', False)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '<D>/assets/probe.osl', '<D>/MOVED/probe.osl', False)
FEW visited ('Sound', 'ProbeSound', '<D>/assets/probe.wav', '<D>/MOVED/probe.wav', False)
FEW visited ('Text', 'ProbeText', '<D>/assets/probe.txt', '<D>/MOVED/probe.txt', False)
FEW visited ('VectorFont', 'ProbeFont', '<D>/assets/probe.woff2', '<D>/MOVED/probe.woff2', False)
FEW visited ('Volume', 'ProbeVolume', '<D>/assets/probe.vdb', '<D>/MOVED/probe.vdb', False)
FEW AFTER | Image.filepath                           | <D>/MOVED/probe.png
FEW AFTER | Image.filepath_raw                       | <D>/MOVED/probe.png
FEW AFTER | Sound.filepath                           | <D>/MOVED/probe.wav
FEW AFTER | VectorFont.filepath                      | <D>/MOVED/probe.woff2
FEW AFTER | Text.filepath                            | <D>/MOVED/probe.txt
FEW AFTER | Volume.filepath                          | <D>/MOVED/probe.vdb
FEW AFTER | MovieClip.filepath                       | <D>/MOVED/probe.png
FEW AFTER | CacheFile.filepath                       | //MOVED/probe.abc
FEW AFTER | Library.filepath                         | <D>/lib.blend
FEW AFTER | RenderSettings.filepath                  | <D>/assets/render_out/
FEW AFTER | BakeSettings.filepath                    | <D>/assets/bake_out/
FEW AFTER | CompositorNodeOutputFile base path       | <D>/assets/node_out/
FEW AFTER | PointCache.filepath (cloth)              | <D>/assets/pointcache/
FEW AFTER | FluidDomainSettings.cache_directory      | <D>/MOVED/fluidcache/
FEW AFTER | NodesModifier.bake_directory             | <D>/assets/gnbake/
FEW AFTER | MultiresModifier.filepath                | 
FEW AFTER | MeshCacheModifier.filepath               | <D>/MOVED/meshcache.mdd
FEW AFTER | OceanModifier.filepath                   | <D>/MOVED/oceanbake/
FEW AFTER | ShaderNodeScript.filepath                | <D>/MOVED/probe.osl
FEW AFTER | ShaderNodeTexIES.filepath                | <D>/MOVED/probe.ies
FEW AFTER | Camera.custom_filepath                   | <D>/MOVED/camshader.osl
FEW AFTER | custom property on an object             | <D>/assets/custom_prop.png
FEW AFTER | ImageStrip.directory                     | <D>/MOVED/stripdir/
FEW AFTER | SequenceEditor.proxy_dir                 | <D>/assets/proxydir/
```

It visited sixteen paths and rewrote each it was handed a changed string for; the library path holds no `/assets/` and came back unchanged. The kinds it did not visit are the same seven the two sweeps left alone.

**Test 24 — the render-output argument with a save and without one, and a bulk rewrite that is never saved.** Two copies of test 20's `many.blend` were made in a second folder, `cli.blend` and `nosave.blend` (both still pointing into test 20's folder, written `<WORK>/e20` below). Every run had no window and printed the scene's render output path and the image's path as that run held them.

`cli.blend` was opened with `-o //FROM_CLI/` placed after the file name, and saved; then opened again with no argument:

```
CLI render.filepath in this run = //FROM_CLI/
CLI image in this run = <WORK>/e20/assets/probe.png
Info: Saved "cli.blend"
CLI save = {'FINISHED'}
CLI render.filepath in this run = //FROM_CLI/
CLI image in this run = <WORK>/e20/assets/probe.png
```

`nosave.blend` was then opened four times in a row: run 1 called `bpy.ops.file.make_paths_relative()` and quit without saving; run 2 opened it with nothing given; run 3 opened it with `-o //NOSAVE_CLI/` and saved nothing; run 4 opened it with nothing given:

```
RUN1 render.filepath in this run = <WORK>/e20/assets/render_out/
RUN1 image in this run = <WORK>/e20/assets/probe.png
RUN1 op make_paths_relative = {'FINISHED'}
RUN1 after, not saved, image = //../e20/assets/probe.png | render = <WORK>/e20/assets/render_out/
RUN2 render.filepath in this run = <WORK>/e20/assets/render_out/
RUN2 image in this run = <WORK>/e20/assets/probe.png
RUN3 render.filepath in this run = //NOSAVE_CLI/
RUN3 image in this run = <WORK>/e20/assets/probe.png
RUN4 render.filepath in this run = <WORK>/e20/assets/render_out/
RUN4 image in this run = <WORK>/e20/assets/probe.png
```

**Test 25 — the Outliner's Data API view, in Blender's window.** Test 21's `many_rel.blend` was opened in Blender's window (`--no-window-focus -p 0 0 1600 1000`). A script turned the window's biggest area into the Outliner in its Data API view, set the interface scale to 0.5, opened the tree two levels and then two more with `bpy.ops.outliner.show_one_level(open=True)`, and photographed the area after a forced redraw each time. It then assigned `bpy.data.images["ProbeImage"].filepath = "//assets/CHANGED.png"` while that view was on screen and saved the file as `outl.blend` in a second folder; a further run with no window opened `outl.blend` and read the path back. What came back:

```
OUTL display_mode  = DATA_API | ui_scale = 0.5
OUTL area          = 2477 x 1503
OUTL opened 2 more levels
OUTL screenshot_area two_levels {'FINISHED'} 138047
OUTL opened 2 more levels
OUTL screenshot_area four_levels {'FINISHED'} 146583
OUTL before = //assets/probe.png
OUTL assignment raised nothing
OUTL after  = //assets/CHANGED.png
OUTL saved  = {'FINISHED'}
OUTL reopened = //../e20/assets/CHANGED.png
```

(The reopened path carries `../e20/` because `outl.blend` was saved one folder over, and saving remaps relative paths by default.)

What the pictures show, read off them. The first shows **Blend-File Data** opened one level: a first column of names, a second of values. `Filename` has a greyed text field holding the file's own path, cut by Blender to `/private/tm…y_rel.blend`; `File Has Unsaved Changes`, `File is Saved` and `Use Auto-Pack` have tick-boxes; each kind of data-block (Cameras, Scenes, Objects, … Volumes) shows an item count; then `Color Space` (a greyed field reading `Blend-File Color Space`), `All Data-Blocks` (`61 items`) and `Project` (a greyed field reading `Blender Project`). In the second, opened further, every kind lists its data-blocks by name — under **Images**: `ProbeImage`, `Render Result`, `Viewer Node`; under **Libraries**: `lib.blend` with a chain icon; `LibCube`, `LibMesh` and `LibColl` with chain icons — but the value column is drawn empty on every row except `Version`'s `X`, `Y` and `Z`, which read `5`, `3` and `29`.

The image's own rows were then tried for, in a window started with `--enable-event-simulate`: a script clicked the expander beside `ProbeImage` with a simulated click, at points read off whole-window pictures. Three runs were made. In the first, the tree had not yet opened to the depth the click was aimed at, and the click landed on `ClothObj`, which became highlighted. In the third, a first click on an empty part of the Outliner and then a click on `ProbeImage`'s expander left the picture as before, with `ProbeImage` unopened. So the image data-block's own `Filepath` row was not brought into a picture.

**Test 26 — the packed switch, and which listing means show a packed image's path.** With no window, starting from the factory startup file saved as `packed.blend` in an empty folder (`<WORK>/e26`), two 4 by 4 PNGs were written by Blender to `assets/` and loaded as `PackedImage` and `PlainImage` (fake users), the first was packed with `Image.pack()`, and the file saved. Then each listing means was asked. `<APP>` stands for `/Applications/Blender 5.3b.app/Contents/Resources`. What came back:

```
PACK saved = {'FINISHED'}
PACK PackedImage.packed_file is None = False
PACK PackedImage.filepath = <WORK>/e26/assets/packed.png
PACK PlainImage.filepath  = <WORK>/e26/assets/plain.png
PACK blend_paths()            = ['//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<WORK>/e26/assets/plain.png']
PACK blend_paths(packed=True) = ['//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<WORK>/e26/assets/packed.png', '<WORK>/e26/assets/plain.png']
PACK file_path_map()          = [('PlainImage', ['<WORK>/e26/assets/plain.png'])]
PACK foreach[default] = ["Image 'PlainImage' <WORK>/e26/assets/plain.png"]
PACK foreach[flags=set()] = ["Image 'PackedImage' <WORK>/e26/assets/packed.png", "Image 'PlainImage' <WORK>/e26/assets/plain.png", "Material 'Dots Stroke' //../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' //../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
PACK foreach[SKIP_PACKED] = ["Image 'PlainImage' <WORK>/e26/assets/plain.png", "Material 'Dots Stroke' //../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' //../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
```

**Test 27 — whether unpacking rewrites the recorded location.** Two copies of test 26's `packed.blend` were made, `u1.blend` and `u2.blend`. Each was opened with no window and `bpy.ops.file.unpack_all(method=…)` called — `'USE_LOCAL'` on the first, `'USE_ORIGINAL'` on the second — printing the packed image's path and packed state before and after, and every file in the folder afterwards (`<W>` is the folder). What came back:

```
UNPACK before, filepath = <WORK>/e26/assets/packed.png | packed = True
Info: Saved packed file to: <WORK>/e26/textures/packed.png
UNPACK op(USE_LOCAL) = {'FINISHED'}
UNPACK after,  filepath = //textures/packed.png | packed = False
UNPACK on disk: <W>/packed.blend
UNPACK on disk: <W>/packed.blend1
UNPACK on disk: <W>/u1.blend
UNPACK on disk: <W>/u2.blend
UNPACK on disk: <W>/assets/packed.png
UNPACK on disk: <W>/assets/plain.png
UNPACK on disk: <W>/textures/packed.png
```
```
UNPACK before, filepath = <WORK>/e26/assets/packed.png | packed = True
Info: Use existing file (instead of packed): <WORK>/e26/assets/packed.png
UNPACK op(USE_ORIGINAL) = {'FINISHED'}
UNPACK after,  filepath = <WORK>/e26/assets/packed.png | packed = False
```

(The second run's folder listing is the same seven files; `textures/packed.png` there is the one the first run wrote.)

**Test 28 — a path holding a template, through the listing means and both sweeps.** With no window, a file was saved as `tmpl.blend` in an empty folder (`<WORK>/e28`) with its render output path set to `<WORK>/e28/assets/out_{blend_name}_####.png`, a compositor node group holding a File Output node whose folder is `<WORK>/e28/assets/node_{blend_name}/`, and one image loaded from `assets/probe.png` — all three in full form. The script then printed the three paths, `blend_paths()`, the paths in `file_path_map()`, the paths `file_path_foreach()` visits, and `frame_path(frame=1)`; called `make_paths_relative()` and printed the same; then `make_paths_absolute()` and printed the same. What came back:

```
TMPL save = {'FINISHED'}
TMPL OPENED   render.filepath = <WORK>/e28/assets/out_{blend_name}_####.png
TMPL OPENED   node base path  = <WORK>/e28/assets/node_{blend_name}/
TMPL OPENED   image           = <WORK>/e28/assets/probe.png
TMPL OPENED   blend_paths     = ['//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<WORK>/e28/assets/probe.png']
TMPL OPENED   file_path_map   = [['<WORK>/e28/assets/probe.png']]
TMPL OPENED   foreach         = ['<WORK>/e28/assets/probe.png']
TMPL OPENED   frame_path(1)   = <WORK>/e28/assets/out_tmpl_0001.png
Info: Total files 3 | Changed 1 | Failed 0
TMPL op make_paths_relative = {'FINISHED'}
TMPL REL      render.filepath = <WORK>/e28/assets/out_{blend_name}_####.png
TMPL REL      node base path  = <WORK>/e28/assets/node_{blend_name}/
TMPL REL      image           = //assets/probe.png
TMPL REL      blend_paths     = ['//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../..<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/probe.png']
TMPL REL      file_path_map   = [['//assets/probe.png']]
TMPL REL      foreach         = ['//assets/probe.png']
TMPL REL      frame_path(1)   = <WORK>/e28/assets/out_tmpl_0001.png
Info: Total files 3 | Changed 3 | Failed 0
TMPL op make_paths_absolute = {'FINISHED'}
TMPL ABS      render.filepath = <WORK>/e28/assets/out_{blend_name}_####.png
TMPL ABS      node base path  = <WORK>/e28/assets/node_{blend_name}/
TMPL ABS      image           = <WORK>/e28/assets/probe.png
TMPL ABS      blend_paths     = ['<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<APP>/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '<WORK>/e28/assets/probe.png']
TMPL ABS      file_path_map   = [['<WORK>/e28/assets/probe.png']]
TMPL ABS      foreach         = ['<WORK>/e28/assets/probe.png']
TMPL ABS      frame_path(1)   = <WORK>/e28/assets/out_tmpl_0001.png
```

(Building also printed `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0`.)

**Test 29 — the write call's four remap choices, and writing onto a file that already exists.** With no window, starting from the factory startup file saved as `src.blend` in an empty folder (`<WORK>/e29`), two 4 by 4 PNGs were written to `assets/`; `RelImage` was loaded from one and its path set to `//assets/rel.png`, `AbsImage` loaded from the other and left in full form. `bpy.data.libraries.write(<out>/<choice>.blend, {RelImage, AbsImage}, path_remap=<choice>)` was called once per choice into `out/`, one folder below. Then a 24-byte text file named `existing.blend` was made and the write call aimed at it. What came back:

```
WRITE source [('RelImage', '//assets/rel.png'), ('AbsImage', '<WORK>/e29/assets/abs.png')]
WRITE NONE written
WRITE RELATIVE written
WRITE RELATIVE_ALL written
WRITE ABSOLUTE written
WRITE existing file size = 24
WRITE onto existing file = no error raised
WRITE existing file size after = 138865
WRITE existing file first bytes after = b'BLENDER17-01'
```

Each written file was then opened with no window, and each image printed as its stored text, what `bpy.path.abspath()` makes of it, and whether a file is there:

```
00:00.319  reports          | WARNING Library file, loading empty scene
WREAD NONE.blend = [('AbsImage', ('<WORK>/e29/assets/abs.png', '<WORK>/e29/assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '<WORK>/e29/out/assets/rel.png', False))]
00:00.327  reports          | WARNING Library file, loading empty scene
WREAD RELATIVE.blend = [('AbsImage', ('<WORK>/e29/assets/abs.png', '<WORK>/e29/assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '<WORK>/e29/assets/rel.png', True))]
00:00.331  reports          | WARNING Library file, loading empty scene
WREAD RELATIVE_ALL.blend = [('AbsImage', ('//../assets/abs.png', '<WORK>/e29/assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '<WORK>/e29/assets/rel.png', True))]
00:00.331  reports          | WARNING Library file, loading empty scene
WREAD ABSOLUTE.blend = [('AbsImage', ('<WORK>/e29/assets/abs.png', '<WORK>/e29/assets/abs.png', True)), ('RelImage', ('<WORK>/e29/assets/rel.png', '<WORK>/e29/assets/rel.png', True))]
```

**Test 30 — what Save As with Remap Relative moves.** With no window, a file was saved as `orig.blend` in a folder (`<WORK>/e30`) holding `RelImage`, its path set to `//assets/rel.png`, and `AbsImage`, left in full form (both present, fake users), with the render output path set to `//renders/`. Then `bpy.ops.wm.save_as_mainfile(filepath=<WORK>/e30/elsewhere/moved.blend, relative_remap=True)` was called. The first run of the script stopped before anything else, because it saved into a folder it had not made: `RuntimeError: Error: Cannot open file <WORK>/e30/orig.blend@ for writing: No such file or directory`. With the folder made first, what came back:

```
SAVEAS save1 = {'FINISHED'}
SAVEAS OPENED   RelImage = //assets/rel.png
SAVEAS OPENED   AbsImage = <WORK>/e30/assets/abs.png
SAVEAS OPENED   render   = //renders/
Info: Saved as "moved.blend"
SAVEAS op = {'FINISHED'}
SAVEAS AFTER    RelImage = //../assets/rel.png
SAVEAS AFTER    AbsImage = <WORK>/e30/assets/abs.png
SAVEAS AFTER    render   = //renders/
SAVEAS new file location = <WORK>/e30/elsewhere/moved.blend
```

**Test 31 — writing a library's own path, against the relocate command.** With no window, three files were made in one folder (`<WORK>/e31`) from an empty start: `B.blend` holding collection `LinkColl` with a mesh object `CubeFromB`; `C.blend` holding collection `LinkColl` with `CubeFromC`; `A.blend` linking `LinkColl` from `B.blend` by its full path and putting it in its scene. `A.blend` was then opened twice with no window. In the first run the script assigned the library's `filepath` the full path of `C.blend`, printed, called the library's `reload()`, and printed again. In the second it called `bpy.ops.wm.lib_relocate(library=<the library's name>, directory=<the folder>/, filename="C.blend")`. Each print lists every object with its library's path. What came back:

```
RELOC OPENED   library = <WORK>/e31/B.blend
RELOC OPENED   objects = [('CubeFromB', '<WORK>/e31/B.blend')]
RELOC ASSIGNED library = <WORK>/e31/C.blend
RELOC ASSIGNED objects = [('CubeFromB', '<WORK>/e31/C.blend')]
RELOC reload() = None
RELOC RELOADED library = <WORK>/e31/C.blend
RELOC RELOADED objects = [('CubeFromC', '<WORK>/e31/C.blend')]
RELOC OPENED   library = <WORK>/e31/B.blend
RELOC OPENED   objects = [('CubeFromB', '<WORK>/e31/B.blend')]
RELOC op lib_relocate = {'FINISHED'}
RELOC RELOCATED library = //C.blend
RELOC RELOCATED objects = [('CubeFromC', '//C.blend')]
```

**Test 32 — turning a stored path into a real one, with and without the library it came from.** With no window: `libdir/lib.blend` was saved holding image `LibTex`, loaded from `libdir/libtex.png` and its path set to `//libtex.png`; then `abs.blend`, one folder up (`<WORK>/e32b`), was saved holding its own image `Own`, its path set to `//assets/own.png`, and linking `LibTex` from `lib.blend`. The first build linked `LibTex` and gave it nothing that uses it, and when `abs.blend` was opened the linked image was not there — the read stopped with `IndexError: list index out of range` at the line looking for an image with a library, after printing the four `AP own` lines. The build was repaired by having a material with a fake user use `LibTex` in an image node, and run again into a fresh folder. `abs.blend` was then opened with no window and `bpy.path.abspath()` asked of each stored path, the linked one with and without `library=` set to its library. What came back:

```
AP own stored          = //assets/own.png
AP own abspath()       = <WORK>/e32b/assets/own.png
AP own relpath(abs)    = //assets/own.png
AP own file exists     = True
AP linked name         = LibTex library = <WORK>/e32b/libdir/lib.blend
AP linked stored       = //libtex.png
AP linked abspath()    = <WORK>/e32b/libtex.png
AP linked abspath(lib) = <WORK>/e32b/libdir/libtex.png
AP linked exists plain = False
AP linked exists lib   = True
```

**Tests 33 and 34 — replacing the file an image or a sequencer strip is loaded from, with no window and in a window.** With no window, two 4 by 4 PNGs were written by Blender at `first/a.png` and `second/b.png` in an empty folder, and `repl.blend` saved there holding image `ProbeImage` loaded from `first/a.png` (fake user) and a Video Sequencer image strip loaded from `first/a.png`, made the active strip. The build printed each command's settings:

```
REPL change_path properties = ['filepath', 'directory', 'files', 'hide_props_region', 'check_existing', 'filter_blender', 'filter_backup', 'filter_image', 'filter_movie', 'filter_python', 'filter_font', 'filter_sound', 'filter_text', 'filter_archive', 'filter_btx', 'filter_alembic', 'filter_usd', 'filter_obj', 'filter_volume', 'filter_folder', 'filter_blenlib', 'filemode', 'relative_path', 'display_type', 'sort_method', 'use_sequence_detection', 'use_placeholders']
REPL image.replace properties = ['filepath', 'hide_props_region', 'check_existing', 'filter_blender', 'filter_backup', 'filter_image', 'filter_movie', 'filter_python', 'filter_font', 'filter_sound', 'filter_text', 'filter_archive', 'filter_btx', 'filter_alembic', 'filter_usd', 'filter_obj', 'filter_volume', 'filter_folder', 'filter_blenlib', 'filemode', 'relative_path', 'show_multiview', 'use_multiview', 'display_type', 'sort_method']
```

The file was then opened once with no window (test 33) and once, built again in its own folder, in Blender's window with the calls made from a timer (test 34). Each run called `bpy.ops.image.replace(filepath=<…>/second/b.png, relative_path=False)` with nothing supplied, then again with the biggest area turned into an Image editor showing the image and that area, its main region and the image supplied in the context; then `bpy.ops.sequencer.change_path(directory=<…>/second/, files=[{"name": "b.png"}])` with nothing supplied, then again with that area turned into a Sequencer editor and the scene supplied as both `scene` and `sequencer_scene`. With no window (test 33):

```
REPLBG background = True | version = 5.3.0 Beta | windows = 1
REPLBG before image = <WORK>/e33/first/a.png
REPLBG op image.replace (no override) = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPLBG after image, first call = <WORK>/e33/first/a.png
REPLBG op image.replace (Image editor supplied) = {'FINISHED'}
REPLBG after image, second call = <WORK>/e33/second/b.png
REPLBG before strip = <WORK>/e33/first/ | ['a.png']
REPLBG op sequencer.change_path (no override) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPLBG after strip, first call = <WORK>/e33/first/ | ['a.png']
REPLBG context has sequencer_scene = <bpy_struct, Scene("Scene") at 0xad1c16020>
REPLBG op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPLBG after strip, second call = //second/ | ['b.png']
```

In a window (test 34):

```
REPL background = False | version = 5.3.0 Beta | windows = 1
REPL before image = <WORK>/e34/first/a.png
REPL op image.replace (no override) = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPL after image, first call = <WORK>/e34/first/a.png
REPL op image.replace (Image editor supplied) = {'FINISHED'}
REPL after image, second call = <WORK>/e34/second/b.png
REPL before strip = <WORK>/e34/first/ | ['a.png']
REPL op sequencer.change_path (no override) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after strip, first call = <WORK>/e34/first/ | ['a.png']
REPL context has sequencer_scene = <bpy_struct, Scene("Scene") at 0xa60eda020>
REPL op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPL after strip, second call = //second/ | ['b.png']
```

The two runs answered alike: each command refused with nothing supplied and rewrote the recorded location once its editor and context were supplied, with no window as well as with one. The strip's new folder was written in `//` form; `change_path` was given no `relative_path` setting.

**Test 37 — which kinds Report Missing Files and Find Missing Files reach.** Recorded under question 6: with an image, a sound, a font, a movie clip, a linked blend-file and an Alembic archive all missing, Report Missing Files printed one line for each of the six, and Find Missing Files handed a folder holding all six rewrote all six recorded locations, the linked blend-file's included.

**Test 38 — Find All.** Recorded under question 10: with Find All on, the search moved recorded locations whose files were present to same-named files in the folder searched; with it off, only the missing one moved.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Label:** documented for the location written back (absolute) and for where the search looks. **Not in the sources:** what makes a file a match, and which one is chosen when there are several.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Where the search looks.** "Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories." Manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11.

**What is written back.** "Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative." Same page.

**Scope option.** `find_all (bool) – Find All, Find all files in the search path (not just missing)`. API, https://docs.blender.org/api/5.3/bpy.ops.file.html, read 2026-10-11. The page does not say what "find" means for a file that is not missing.

**Match rule and choice between several: not in the sources.** Nothing says whether a match is by file name alone, by name and size, or by anything else, nor how a winner is picked.

**Settled for 5.3 on the Creator's Mac by the hand tests of 2026-10-17 (observed; recorded at the end of this answer).** A file was taken as a match only when its name was the same, letter case included; a same-named file eight folders down was found. Where two matched, the larger file was taken from either folder and at either depth; where they were the same size, the one in the folder named `aaa` was taken over `zzz`, and the files' dates made no difference. No line was printed about there having been more than one (tests 35 and 36).

**Not covered:** case-sensitivity of matching; handling of image sequences and UDIM sets; whether library files are searched.

**Brought up to date by the hand tests of 2026-10-17.** Letter case: `TEX.PNG` was not taken for `tex.png` on the Mac, whose file system does not treat case as significant (test 35). Library files are searched: a missing linked blend-file was found and its path rewritten (test 37). Image sequences and UDIM sets were not tested.

**Where I looked:** searched the 5.3 manual, API documentation and release notes for: find missing, find_missing_files, find_all, Find All, recursive, recursively, same name, same file name, matching, by name, by file name, file size, largest, newest, most recent. Read the Topbar page, the `bpy.ops.file` page, the Link & Append page and the Packed Data page. Only the topbar passage and the operator's parameter list describe this search.

**Outside the interface:** the operator exists for Python with a `directory` setting (documented). Behaviour without a window is **not in the sources**.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: the command ran when handed a folder, in every arrangement (tests 14, 35 to 39). The location written back kept the form recorded before — `//` for a `//` path, full form for a full-form path — not always full form (tests 14, 35, 37). Undo of a recovery could not be run with no window (test 39).

**Operating system:** the sources do not say. Whether matching ignores letter case on some systems is not stated.

**Brought up to date by the hand tests of 2026-10-17.** These tests settle macOS on Apple silicon for what they cover. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 14 — what location is written back.** With one missing image and its file moved to a sub-folder of the folder searched, `bpy.ops.file.find_missing_files(directory=…)` with no window wrote back `/…/e14/search/deeper/probe.png` when the path recorded before was in full form (`/…/e14/tex/probe.png`), and `//search/deeper/probe.png` when the path recorded before was `//tex/probe.png`. The full record is under question 6, test 14.

**Test 35 — what the search accepts as a match.** One arrangement per arm, each in its own fresh folder. With no window, `tex.blend` was saved holding image `Tex`, loaded from an 8 by 8 red PNG written by Blender at `orig/tex.png`, its path set to `//orig/tex.png`, with a fake user; `orig/tex.png` was then deleted, and the folder `search/` arranged for the arm, every file in it printed with its size and date. A second run with no window opened `tex.blend` and called `bpy.ops.file.find_missing_files(directory=<the search/ folder>)`, then printed the path, whether it begins `//`, whether the image's data was loaded at that moment (`has_data`), whether the file now named is there, and the image's first pixel (reading it loads the file). The arms: `same` — a copy at `s_same/deep/tex.png`; `ext` — only `tex.jpg` (a PNG renamed); `other` — only `other.png`, the same picture; `empty` — nothing; `upper` — only `TEX.PNG` (a PNG renamed); `upper_control` — `tex.png`; `deep8` — `tex.png` eight folders down. What came back (lines starting `HT|`, and every warning; `<WORK>` is the work folder):

```
== same
HT| same | candidate search/s_same/deep/tex.png size=623 mtime=2026-10-17 09:41:26
HT| filepath before = '//orig/tex.png'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//search/s_same/deep/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = True
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
== ext
HT| ext | candidate search/tex.jpg size=623 mtime=2026-10-17 09:41:28
HT| filepath before = '//orig/tex.png'
Warning: Could not find 'tex.png' in '<WORK>/e35/ext/search'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//orig/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = False
HT| pixel after = []
== other
HT| other | candidate search/other.png size=623 mtime=2026-10-17 09:41:30
HT| filepath before = '//orig/tex.png'
Warning: Could not find 'tex.png' in '<WORK>/e35/other/search'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//orig/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = False
HT| pixel after = []
== empty
HT| filepath before = '//orig/tex.png'
Warning: Could not find 'tex.png' in '<WORK>/e35/empty/search'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//orig/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = False
HT| pixel after = []
== upper
HT| upper | candidate search/TEX.PNG size=623 mtime=2026-10-17 09:41:33
HT| filepath before = '//orig/tex.png'
Warning: Could not find 'tex.png' in '<WORK>/e35/upper/search'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//orig/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = False
HT| pixel after = []
== upper_control
HT| upper_control | candidate search/tex.png size=623 mtime=2026-10-17 09:41:35
HT| filepath before = '//orig/tex.png'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//search/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = True
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
== deep8
HT| deep8 | candidate search/1/2/3/4/5/6/7/8/tex.png size=623 mtime=2026-10-17 09:41:36
HT| filepath before = '//orig/tex.png'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//search/1/2/3/4/5/6/7/8/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = True
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
```

The Mac's file system does not treat capital letters in a file name as significant; `TEX.PNG` was still not taken.

**Test 36 — which candidate is chosen when several match.** Built as test 35, with two candidates per arm, told apart by colour; the PNGs are written by Blender, 8 by 8 unless said. Arms: `size_zzz` — red in `aaa/`, blue 64 by 64 in `zzz/`; `size_aaa` — the two swapped; `deep_bigger` — red at the top of `search/`, green 16 by 16 three folders down in `x/y/z/`; `top_bigger` — those swapped; `tie` — red in `aaa/`, blue in `zzz/`; `tie_swapped` — blue in `aaa/`, red in `zzz/`; `date_zzz_newer` — as `tie`, with `aaa/`'s file dated 2020-01-01 by `os.utime`; `date_aaa_newer` — as `tie`, with `zzz/`'s file dated 2020-01-01. What came back:

```
== size_zzz
HT| size_zzz | candidate search/aaa/tex.png size=623 mtime=2026-10-17 09:41:45
HT| size_zzz | candidate search/zzz/tex.png size=739 mtime=2026-10-17 09:41:45
HT| filepath after = '//search/zzz/tex.png'
HT| pixel after = [0.0, 0.0, 1.0, 1.0]
== size_aaa
HT| size_aaa | candidate search/aaa/tex.png size=739 mtime=2026-10-17 09:41:46
HT| size_aaa | candidate search/zzz/tex.png size=623 mtime=2026-10-17 09:41:46
HT| filepath after = '//search/aaa/tex.png'
HT| pixel after = [0.0, 0.0, 1.0, 1.0]
== deep_bigger
HT| deep_bigger | candidate search/tex.png size=623 mtime=2026-10-17 09:41:48
HT| deep_bigger | candidate search/x/y/z/tex.png size=634 mtime=2026-10-17 09:41:48
HT| filepath after = '//search/x/y/z/tex.png'
HT| pixel after = [0.0, 1.0, 0.0, 1.0]
== top_bigger
HT| top_bigger | candidate search/tex.png size=634 mtime=2026-10-17 09:41:50
HT| top_bigger | candidate search/x/y/z/tex.png size=623 mtime=2026-10-17 09:41:50
HT| filepath after = '//search/tex.png'
HT| pixel after = [0.0, 1.0, 0.0, 1.0]
== tie
HT| tie | candidate search/aaa/tex.png size=623 mtime=2026-10-17 09:41:51
HT| tie | candidate search/zzz/tex.png size=623 mtime=2026-10-17 09:41:51
HT| filepath after = '//search/aaa/tex.png'
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
== tie_swapped
HT| tie_swapped | candidate search/aaa/tex.png size=623 mtime=2026-10-17 09:41:53
HT| tie_swapped | candidate search/zzz/tex.png size=623 mtime=2026-10-17 09:41:53
HT| filepath after = '//search/aaa/tex.png'
HT| pixel after = [0.0, 0.0, 1.0, 1.0]
== date_zzz_newer
HT| date_zzz_newer | candidate search/aaa/tex.png size=623 mtime=2020-01-01 12:00:00
HT| date_zzz_newer | candidate search/zzz/tex.png size=623 mtime=2026-10-17 09:41:55
HT| filepath after = '//search/aaa/tex.png'
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
== date_aaa_newer
HT| date_aaa_newer | candidate search/aaa/tex.png size=623 mtime=2026-10-17 09:41:56
HT| date_aaa_newer | candidate search/zzz/tex.png size=623 mtime=2020-01-01 12:00:00
HT| filepath after = '//search/aaa/tex.png'
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
```

No line about there having been more than one candidate appeared in any arm; the warning that does appear when nothing is found is shown in test 35.

**Test 37 — six kinds of file found by one search.** Recorded under question 6: one call of `bpy.ops.file.find_missing_files(directory=…)` with no window rewrote the recorded locations of a missing image, sound, font, movie clip, linked blend-file and Alembic archive to `search/found/…`, each in the form it had before; with the search folder empty, each printed its own `Warning: Could not find '<name>' in '<folder>'` and none moved.

**Test 38 — Find All, and a packed image whose file has moved.** Four arms, each from an empty start in its own folder with no window, each printing every image's path, whether its file is there, and whether it is packed, before and after one call of `bpy.ops.file.find_missing_files`. `two_folders`: `FromOne` loaded from `one/same.png` and `FromTwo` from `two/same.png`, both present, the search handed `one/` with `find_all=True`. `keep_off` and `keep_on`: `GoodImg` loaded from `keep/good.png` (present) and `BrokenImg` from `orig/broken.png`, which was then deleted; `search/` holds a `good.png` and a `broken.png`; the search handed `search/` with `find_all` off, and on. `packed`: `PackedImg` loaded from `orig/p.png` and packed, the file then moved to `search/found/p.png`, the search handed `search/`. What came back:

```
== two_folders
FINDALL before FromOne = <WORK>/e38/two_folders/one/same.png | exists True | packed False
FINDALL before FromTwo = <WORK>/e38/two_folders/two/same.png | exists True | packed False
FINDALL op = {'FINISHED'}
FINDALL after FromOne = <WORK>/e38/two_folders/one/same.png | exists True | packed False
FINDALL after FromTwo = <WORK>/e38/two_folders/one/same.png | exists True | packed False
== keep_off
FINDALL before BrokenImg = <WORK>/e38/keep_off/orig/broken.png | exists False | packed False
FINDALL before GoodImg = <WORK>/e38/keep_off/keep/good.png | exists True | packed False
FINDALL op (find_all=False) = {'FINISHED'}
FINDALL after BrokenImg = <WORK>/e38/keep_off/search/broken.png | exists True | packed False
FINDALL after GoodImg = <WORK>/e38/keep_off/keep/good.png | exists True | packed False
== keep_on
FINDALL before BrokenImg = <WORK>/e38/keep_on/orig/broken.png | exists False | packed False
FINDALL before GoodImg = <WORK>/e38/keep_on/keep/good.png | exists True | packed False
FINDALL op (find_all=True) = {'FINISHED'}
FINDALL after BrokenImg = <WORK>/e38/keep_on/search/broken.png | exists True | packed False
FINDALL after GoodImg = <WORK>/e38/keep_on/search/good.png | exists True | packed False
== packed
FINDALL before PackedImg = <WORK>/e38/packed/orig/p.png | exists False | packed True
FINDALL op = {'FINISHED'}
FINDALL after PackedImg = <WORK>/e38/packed/search/found/p.png | exists True | packed True
```

**Test 39 — whether a recovery can be undone with no window.** Test 35's `same` arrangement was built again in a fresh folder and repaired the same way with no window; then `bpy.ops.ed.undo_push(message="after find")` and `bpy.ops.ed.undo()` were called, the second with nothing supplied and then with a 3D Viewport area and its main region supplied, printing the image's path after each. What came back:

```
HT| filepath before = '//orig/tex.png'
HT| operator returned = "{'FINISHED'}"
HT| filepath after = '//search/s_same/deep/tex.png'
HT| starts with // = True
HT| has_data = False
HT| file there = True
HT| pixel after = [1.0, 0.0, 0.0, 1.0]
HT| undo_push = {'FINISHED'}
HT| undo (no override) = ERROR RuntimeError Operator bpy.ops.ed.undo.poll() failed, context is incorrect
HT| filepath after undo (no override) = '//search/s_same/deep/tex.png'
HT| undo (3D Viewport area supplied) = ERROR RuntimeError Operator bpy.ops.ed.undo.poll() failed, context is incorrect
HT| filepath after undo (3D Viewport area supplied) = '//search/s_same/deep/tex.png'
```

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**Label:** documented, per kind of file, that Blender offers a reload and in some places says the change is not picked up until then. Documented for text files that Blender notices the change, and for Cycles' texture cache that it regenerates. **Observed** in Cycles' own files for the texture cache UI controls and CLI command registration. **Inferred** that inside a running Blender an item must be reloaded or the file reopened before a render uses the changed file. **Not in the sources:** a general rule that covers every kind of file, and what happens on its own, without a reload, for most kinds.

**Observed — added by the hand tests of 2026-10-17.** This answer now also carries results of tests run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) with Blender 5.3.0 Beta; they are recorded at the end of the answer, under *Observed — hand tests on the Creator's Mac, 2026-10-17*.

**Images.**
- "Reload — Reload the image from the file on drive." Manual, https://docs.blender.org/manual/en/5.3/editors/image/editing.html, read 2026-10-11.
- "Reload — Reloads the file. Useful when it has been reworked in an external application." Manual, https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html, read 2026-10-11.
- Painting in an outside program, step list: "Back in Blender, you reload the image in the Image Editor." Manual, https://docs.blender.org/manual/en/5.3/sculpt_paint/texture_paint/introduction.html, read 2026-10-11.
- Python: `Image.reload()` "Reload the image from its source path" (https://docs.blender.org/api/5.3/bpy.types.Image.html); `bpy.ops.image.reload()` "Reload current image from disk" (https://docs.blender.org/api/5.3/bpy.ops.image.html). Both read 2026-10-11.

**Linked `.blend` files (libraries).**
- "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded." Manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html, read 2026-10-11.
- "you can right-click a linked blend-file and choose Reload to immediately update the current blend-file with the latest version of the linked data-blocks, without having to reopen the file." Same page.
- Python: `Library.reload()` "Reload this library and all its linked data-blocks" (https://docs.blender.org/api/5.3/bpy.types.Library.html); `bpy.ops.wm.lib_reload(…)` "Reload the given library" (https://docs.blender.org/api/5.3/bpy.ops.wm.html). Both read 2026-10-11.

**Video Sequencer media.**
- "Movie strips reference the original media file rather than embedding it in the blend-file, allowing changes made in external editing or encoding software to be reflected by simply reloading the file." Manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/movie.html, read 2026-10-11. Sound strips: same wording on https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/sound.html.
- "Refresh All Ctrl-E — Reloads external files and refreshes the current frame preview. This is useful when you modified an external file or made a change in a scene that Blender didn’t detect." Manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/sequencer/navigating.html, read 2026-10-11.
- Python: `bpy.ops.sequencer.reload(*, adjust_length=False)` "Reload strips in the sequencer". API, https://docs.blender.org/api/5.3/bpy.ops.sequencer.html, read 2026-10-11.

**Movie clips.** "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender." Manual, https://docs.blender.org/manual/en/5.3/movie_clip/tracking/clip/editing/clip.html, read 2026-10-11.

**Cache files (archives such as Alembic).** `bpy.ops.cachefile.reload()` "Update objects paths list with new data from the archive". API, https://docs.blender.org/api/5.3/bpy.ops.cachefile.html, read 2026-10-11. The Alembic importer's option reads: "Add cache modifiers and constraints to imported objects even if they are not animated so that they can be updated when reloading the Alembic archive". API, https://docs.blender.org/api/5.3/bpy.ops.wm.html (`alembic_import`, `always_add_cache_reader`), read 2026-10-11. The sources do not say whether other archive types behave the same.

**Text files — Blender notices.**
- "Resolve Conflict — Resolves modified file conflicts when an external text file is updated from another program." and "Ignore — Hides the warning message until the external text file is modified externally again." Manual, https://docs.blender.org/manual/en/5.3/editors/text_editor.html, read 2026-10-11.
- `Text.is_modified`: "Text file on disk is different than the one in memory". API, https://docs.blender.org/api/5.3/bpy.types.Text.html, read 2026-10-11.

**Cycles texture cache (`.tx` files).**
- "Auto Generate — Automatically generate and update .tx files for images when rendering. Files are regenerated when the source image is modified." Manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html, read 2026-10-11.
- `RenderSettings.use_auto_generate_texture_cache`: "Automatically create tx files from image files when rendering, if the files do not exist or are outdated. … (default False)". `use_texture_cache` "(default True)". API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- Cycles' own files, **observed**: `ui.py`, class `CYCLES_RENDER_PT_performance_texture_cache` (lines 908–932), draws `col.prop(rd, "use_auto_generate_texture_cache", text="Auto Generate")` and the buttons `sub.operator("render.generate_texture_cache", text="Generate")` and `sub.operator("render.clear_texture_cache", text="Clear")`. `maketx.py`, lines 14–19: "Generate Cycles texture cache (.tx) files. … When no file path is given, a blend file must be loaded and tx files are generated for all images used by shader node trees." Read 2026-10-16.
- **Not in the sources:** what Cycles uses when the texture cache is on, Auto Generate is off (its default), and the source image has changed since its `.tx` file was made.

**Packed data does not follow the disk.** "After running this operator and saving the blend-file, the external files will no longer be used – any changes in them will no longer be reflected in the blend-file". Manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11.

**Reloading everything at once / the `.blend` itself.**
- "you can save the blend-file and reload it again, so that all external files are reloaded at once." Topbar page above.
- "Revert — Reopens the current file to its last saved version." Topbar page above. `bpy.ops.wm.revert_mainfile()` "Reload the saved file". API, bpy.ops.wm page above.

**What has to happen before a later render uses the changed file — inferred from the above.** Inside a running Blender: reload the item (image, library, strip, clip, cache), or reopen the file. Several pages pair the change with a reload, which implies it is not picked up without one; only the sequencer page says outright that Blender may not "detect" a change. A Blender started after the change reads the files it loads at that time; the sources do not say this in so many words.

**Not covered:** whether Blender notices changed images, libraries or caches on its own (it is documented only for text files); what "Persistent Data" does with a changed file between renders ("Keep render data in memory after rendering for faster re-renders and animation renders", performance page above; no mention of changed files); whether "replaced" (a different file put at the same path) behaves differently from "changed"; whether a frame-by-frame animation render re-reads images each frame.

**Brought up to date by the hand tests of 2026-10-17 (recorded at the end of this answer).** Within one running Blender, with nothing done: a changed image, image-sequence frame and external OSL script reached the next render; a replaced linked blend-file, a sequencer strip's file, a movie clip and an Alembic archive did not; a changed text file was flagged by `Text.is_modified` (tests 40, 42 to 47, 50). With Persistent Data on, a changed image did not reach the next render even after `Image.reload()` (test 41). An image replaced during an animation render showed the old picture up to a frame and the new one after it, with no frame mixed (test 52). A freshly started Blender showed the changed file in tests 40, 42, 46 (with the cache reader), 49 (after the timing repair) and 50; in test 46 without a cache reader it still showed the old archive's size, and in test 49's first run, where the image changed within the same second as its `.tx` was made, it still showed the old picture. Whether "replaced" differs from "changed" was not separated: every file was replaced by copying over it.

**Where I looked:** searched the 5.3 manual, API documentation and release notes for: reload, refresh, revert, modified, is_modified, modified externally, changed on disk, edited outside, outside of Blender, didn’t detect, detect, texture cache, tx, outdated, persistent data, auto refresh. Read the image, text editor, sequencer, movie clip, Link & Append, Packed Data, topbar and Cycles performance pages, the API pages cited above, and Cycles' `ui.py`, `maketx.py`, `engine.py`, `__init__.py`.

**Outside the interface:** every reload above has a Python form (cited). The texture cache can be built from the command line: "blender scene.blend --command maketx" (performance page above; also Cycles `__init__.py` line 158, `cli_commands.append(bpy.utils.register_cli_command("maketx", maketx_command))`, **observed**, read 2026-10-16). The text-file warning is an interface message; whether anything signals a changed text file without a window, other than `Text.is_modified`, is **not in the sources**.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window: every test for this question ran with no window. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` ran as they are. The command forms for images, movie clips, cache files, text, the sequencer and OSL script updates refused with nothing supplied and finished once their editor and context were supplied (tests 40, 43, 45 to 47, 50). `bpy.ops.wm.lib_reload()` refused with `Error: Not a library` (test 42).

**Operating system:** the sources do not say.

**Brought up to date by the hand tests of 2026-10-17.** These tests settle macOS on Apple silicon for what they cover. Linux and Windows are untested for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Done on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16) at `/Applications/Blender 5.3b.app`. Every run was started from the terminal with `--factory-startup`, with Blender's temporary folder (`TMPDIR`) and its settings folder (`BLENDER_USER_RESOURCES`) pointed at a throwaway work folder, and with no window (`-b`) unless the entry says a window was opened. Only one of these Blenders ran at a time. The Creator's own Blender 5.2 window was open on the same Mac throughout and was not touched. Each test carries its number in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md`, where its script is kept word for word; what is needed to repeat it is written here. Long paths are shortened to `/…` where marked; everything else is as the machine printed it.

**Test 1 — the settings that bear on a changed file, and what the library reload command takes.** With no file opened, Blender's own description of five settings was read and each default printed; then whether `bpy.ops.render` holds the two texture-cache commands; then the list of settings `bpy.ops.wm.lib_reload` takes, with Blender's own description of each. What came back:

```
PROBE RenderSettings.use_texture_cache default = True
PROBE RenderSettings.use_auto_generate_texture_cache default = False
PROBE RenderSettings.use_persistent_data default = False
PROBE ShaderNodeScript.use_auto_update default = False
PROBE ImageUser.use_auto_refresh default = False
PROBE bpy.ops.render has generate_texture_cache = True | clear_texture_cache = True
PROBE wm.lib_reload properties = [('library', 'Library to reload'), ('filepath', 'Path to file'), ('directory', 'Directory of the file'), ('filename', 'Name of the file'), ('hide_props_region', 'Collapse the region displaying the operator settings'), ('check_existing', 'Check and warn on overwriting existing files'), ('filter_blender', ''), ('filter_backup', ''), ('filter_image', ''), ('filter_movie', ''), ('filter_python', ''), ('filter_font', ''), ('filter_sound', ''), ('filter_text', ''), ('filter_archive', ''), ('filter_btx', ''), ('filter_alembic', ''), ('filter_usd', ''), ('filter_obj', ''), ('filter_volume', ''), ('filter_folder', ''), ('filter_blenlib', ''), ('filemode', 'The setting for the file browser mode to load a .blend file, a library or a special file'), ('relative_path', 'Select the file relative to the blend file'), ('display_type', ''), ('sort_method', '')]
```

**How tests 40 to 53 were set up.** Unless the test says otherwise: one plane, 2 across, lit by an emission shader whose colour comes from one image file, `tex.png`, through an image texture node; an orthographic camera looking straight down at it so the plane fills the frame; Cycles on the processor, 8 by 8 pixels, one sample, denoising and adaptive sampling off, Standard view transform, PNG output. Each render was written to a file and its centre pixel read back by Blender as red, green, blue, alpha, so red reads `[1.0, 0.0, 0.0, 1.0]` and blue `[0.0, 0.0, 1.0, 1.0]`. The red and blue 8 by 8 PNGs were written by Blender. Files were replaced by the operating system with Python's `shutil.copyfile`, never by Blender, and every replacement was followed by two and a half seconds before anything else was done. Every run had no window.

**Test 40 — an image changed on disk under a running Blender, the two reload routes, and a freshly started Blender.** One Blender built the scene with `tex.png` a copy of red, saved it as `scene.blend`, and rendered; then blue was copied over `tex.png` and it rendered again with nothing done; then `Image.reload()` was called; red copied back; `bpy.ops.image.reload()` called with nothing supplied; rendered; blue copied over again; `bpy.ops.image.reload()` called with the biggest area turned into an Image editor and that area and its main region supplied, then again with the image put in that editor and supplied too; rendered; and red copied back. Then two fresh Blenders opened `scene.blend` and rendered — the first with red on disk, the second after blue had been copied over `tex.png` from the terminal. What came back:

```
HT| 1 render, red on disk = [1.0, 0.0, 0.0, 1.0]
HT| 2 render after blue was copied over it, nothing done = [0.0, 0.0, 1.0, 1.0]
HT| Image.reload() returned None
HT| bpy.ops.image.reload() with no Image editor = {'CANCELLED'}
HT| 3 render after red was copied back = [1.0, 0.0, 0.0, 1.0]
HT| bpy.ops.image.reload() with an Image editor, no image in it = {'CANCELLED'}
HT| bpy.ops.image.reload() with the image in an Image editor = {'FINISHED'}
HT| 4 render after blue was copied over it again = [0.0, 0.0, 1.0, 1.0]
HT| left on disk for the fresh run: red
HT| fresh Blender, render = [1.0, 0.0, 0.0, 1.0]
HT| fresh Blender, render = [0.0, 0.0, 1.0, 1.0]
```

**Test 41 — the same with Persistent Data off and on.** Built as test 40, once with `scene.render.use_persistent_data` off and once on: render; blue copied over `tex.png`; render; `Image.reload()`; render. What came back:

```
HT| off | use_persistent_data = False
HT| off | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| off | 2 render after blue was copied over it = [0.0, 0.0, 1.0, 1.0]
HT| off | Image.reload() returned None
HT| off | 3 render after Image.reload() = [0.0, 0.0, 1.0, 1.0]
HT| on | use_persistent_data = True
HT| on | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| on | 2 render after blue was copied over it = [1.0, 0.0, 0.0, 1.0]
HT| on | Image.reload() returned None
HT| on | 3 render after Image.reload() = [1.0, 0.0, 0.0, 1.0]
```

**Test 42 — a linked blend-file replaced on disk.** From an empty start, `lib_red.blend` and `lib_blue.blend` were saved, each holding collection `LibCol` with one plane whose emission colour is set in the material itself, red or blue; `lib.blend` was made a copy of the red one; `main.blend` was saved with the camera and Cycles settings above, linking `LibCol` from `lib.blend` (`//lib.blend`) into its scene. One Blender opened `main.blend` and rendered; blue's file was copied over `lib.blend`; rendered with nothing else done; called the library's `reload()`; rendered; red copied back over `lib.blend`; then called `bpy.ops.wm.lib_reload(library=<the library's name>)` and rendered, and `bpy.ops.wm.lib_reload(library=<the name>, filepath=<the library's recorded path>)` and rendered (test 1 records that the command takes `library`, `filepath`, `directory`, `filename` and file-browser settings); blue was copied over `lib.blend` again; a fresh Blender opened `main.blend` and rendered. What came back:

```
HT| 1 render = [1.0, 0.0, 0.0, 1.0]
HT| 2 render after the library was replaced, nothing else done = [1.0, 0.0, 0.0, 1.0]
HT| Library.reload() returned None
HT| 3 render after Library.reload() = [0.0, 0.0, 1.0, 1.0]
Error: Not a library
HT| bpy.ops.wm.lib_reload(library name) = ERROR RuntimeError Error: Not a library
HT| render after that call, red now on disk = [0.0, 0.0, 1.0, 1.0]
Error: Not a library
HT| bpy.ops.wm.lib_reload(library name and its path) = ERROR RuntimeError Error: Not a library
HT| render after that call, red now on disk = [0.0, 0.0, 1.0, 1.0]
HT| left on disk for the fresh run: blue
HT| fresh Blender, render = [0.0, 0.0, 1.0, 1.0]
```

The second `lib_reload` call also printed `RuntimeWarning: WM_OT_lib_reload.filepath: does not support blend relative "//" prefix`; the library's recorded path is `//lib.blend`. A repeat of the same run gave the same lines.

**Test 43 — a Video Sequencer strip's file replaced.** A scene whose picture comes from one sequencer image strip loaded from `tex.png` (red at first), with the sequencer used for rendering, 8 by 8. Render; blue copied over `tex.png`; render with nothing else done; then `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()`, each with nothing supplied, then each with the biggest area turned into a Sequencer editor and that area, its main region, and the scene as both `scene` and `sequencer_scene` supplied; render. What came back:

```
HT| 1 render = [1.0, 0.0, 0.0, 1.0]
HT| 2 render after blue was copied over it, nothing else done = [1.0, 0.0, 0.0, 1.0]
HT| bpy.ops.sequencer.reload() with no editor = ERROR RuntimeError Operator bpy.ops.sequencer.reload.poll() failed, context is incorrect
HT| bpy.ops.sequencer.refresh_all() with no editor = ERROR RuntimeError Operator bpy.ops.sequencer.refresh_all.poll() failed, context is incorrect
HT| bpy.ops.sequencer.reload() with a Sequencer editor supplied = {'FINISHED'}
HT| bpy.ops.sequencer.refresh_all() with a Sequencer editor supplied = {'FINISHED'}
HT| 3 render after those calls = [0.0, 0.0, 1.0, 1.0]
```

**Test 44 — one frame of an image sequence overwritten, Auto Refresh off and on.** The image is a two-frame sequence, `f_0001.png` (red) and `f_0002.png` (green): loaded from the first, its source set to `'SEQUENCE'`, and the texture node's image user set to 2 frames from frame 1, with `use_auto_refresh` off in one run and on in the other. Frame 1 rendered, frame 2 rendered, white copied over `f_0001.png`, frame 1 rendered again. What came back:

```
HT| off | use_auto_refresh = False
HT| off | frame 1 = [1.0, 0.0, 0.0, 1.0]
HT| off | frame 2 = [0.0, 1.0, 0.0, 1.0]
HT| off | frame 1 after f_0001.png was overwritten with white = [1.0, 1.0, 1.0, 1.0]
HT| on | use_auto_refresh = True
HT| on | frame 1 = [1.0, 0.0, 0.0, 1.0]
HT| on | frame 2 = [0.0, 1.0, 0.0, 1.0]
HT| on | frame 1 after f_0001.png was overwritten with white = [1.0, 1.0, 1.0, 1.0]
```

**Test 45 — a movie clip's file replaced.** A movie clip loaded with `bpy.data.movieclips.load()` from an 8 by 8 PNG, `clip.png`; a 16 by 16 PNG copied over it; the clip's size read; then `bpy.ops.clip.reload()` with nothing supplied, and with the biggest area turned into a Clip editor showing the clip and that area, its main region and the clip supplied. What came back:

```
HT| clip.size at load = (8, 8)
HT| clip.size after the file was replaced, nothing done = (8, 8)
HT| bpy.ops.clip.reload() with no Clip editor = {'CANCELLED'}
HT| clip.size = (8, 8)
HT| bpy.ops.clip.reload() with the clip in a Clip editor = {'FINISHED'}
HT| clip.size = (16, 16)
```

**Test 46 — an Alembic archive replaced.** From an empty start, a cube 2 across and then a cube 4 across were each made with `bpy.ops.mesh.primitive_cube_add(size=…)` and exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` as `small.abc` and `big.abc`. In each of two arms, `small.abc` was copied to `arch.abc` and imported into an empty file with `bpy.ops.wm.alembic_import(filepath=…, as_background_job=False)` — in the `defaults` arm with nothing else, in the `reader` arm with `always_add_cache_reader=True` — and the file saved; then `big.abc` was copied over `arch.abc`, and the evaluated object measured (its smallest and largest `x` in world space, and its dimensions) after each of: nothing done; `bpy.ops.cachefile.reload()` with nothing supplied; the same with the cache file supplied as `edit_cachefile`; the cache file's path written back to itself; the frame stepped to 5 and back to 1. Then a fresh Blender opened each arm's saved file and measured; in the `reader` arm it also imported the replacement archive on its own and measured that. What came back:

```
HT| export small {'FINISHED'}
HT| size of small.abc 3120
HT| export big {'FINISHED'}
HT| size of big.abc 3120
HT| defaults | import = {'FINISHED'}
HT| defaults | Cube modifiers = [] constraints = []
HT| defaults 1 at import | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| defaults 2 after the archive was replaced, nothing done | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| defaults | cachefile.reload() with no context = {'CANCELLED'}
HT| defaults 3 | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| defaults | cachefile.reload() with edit_cachefile supplied = {'FINISHED'}
HT| defaults 4 | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| defaults 5 after the recorded path was written back to itself | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| defaults 6 after the frame was stepped away and back | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| fresh fresh_defaults | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| reader | import = {'FINISHED'}
HT| reader | Cube modifiers = ['MESH_SEQUENCE_CACHE'] constraints = ['TRANSFORM_CACHE']
HT| reader 1 at import | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| reader 2 after the archive was replaced, nothing done | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| reader | cachefile.reload() with no context = {'CANCELLED'}
HT| reader 3 | Cube x from -1.000 to 1.000 | dimensions (2.0, 2.0, 2.0)
HT| reader | cachefile.reload() with edit_cachefile supplied = {'FINISHED'}
HT| reader 4 | Cube x from -2.000 to 2.000 | dimensions (4.0, 4.0, 4.0)
HT| reader 5 after the recorded path was written back to itself | Cube x from -2.000 to 2.000 | dimensions (4.0, 4.0, 4.0)
HT| reader 6 after the frame was stepped away and back | Cube x from -2.000 to 2.000 | dimensions (4.0, 4.0, 4.0)
HT| fresh fresh_reader | Cube x from -2.000 to 2.000 | dimensions (4.0, 4.0, 4.0)
HT| the replacement archive imported on its own | Cube x from -2.000 to 2.000 | dimensions (4.0, 4.0, 4.0)
HT| the replacement archive imported on its own | ReplacementImportedAlone x from -2.000 to 2.000 | dimensions (4.0, 4.0, 4.0)
```

The two archives are the same size on disk, which is why the object was measured. With the importer's defaults the object carries no cache modifier or constraint, and nothing — not even a fresh Blender — showed it at the new size.

**Test 47 — a text file changed on disk.** A text was loaded with `bpy.data.texts.load()` from `t.txt` holding `one`. Straight after, the file was rewritten by Python to `two`, and two and a half seconds later `Text.is_modified` and the text Blender held were read. Then `bpy.ops.text.reload()` was called with nothing supplied, and again with the biggest area turned into a Text editor showing the text and that area, its main region and the text supplied; the flag and the text read; then the file rewritten to `three`, two and a half seconds, the flag and text read, `bpy.ops.text.resolve_conflict(resolution='RELOAD')` called in the same Text editor, and the flag and text read again. What came back:

```
HT| is_modified at load = False | text = 'one\n'
HT| is_modified after the file was rewritten = False | text = 'one\n'
HT| bpy.ops.text.reload() with no Text editor = ERROR RuntimeError Operator bpy.ops.text.reload.poll() failed, context is incorrect
HT| bpy.ops.text.reload() with a Text editor = {'FINISHED'}
HT| after reload: is_modified = False | text = 'two\n'
HT| after a second rewrite: is_modified = True | text = 'two\n'
HT| bpy.ops.text.resolve_conflict(resolution='RELOAD') = {'FINISHED'}
HT| after resolve: is_modified = False | text = 'three\n'
```

The first rewrite was made within a fraction of a second of loading and was not flagged; the second was made more than two and a half seconds after the reload and was flagged.

**Test 48 — a packed image whose file on disk is changed.** Built as test 40 in three arms: `unpacked`; `packed` — `Image.pack()` called and the blend-file saved; `packed_reopened` — packed, saved, and opened again with `bpy.ops.wm.open_mainfile()`. Each: render; blue copied over `tex.png`; render; `Image.reload()`; render; whether the image is still packed. What came back:

```
HT| unpacked | test PNG bytes = 623
HT| unpacked | packed = False
HT| unpacked | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| unpacked | 2 render after blue was copied over the file = [0.0, 0.0, 1.0, 1.0]
HT| unpacked | Image.reload() returned None
HT| unpacked | 3 render after Image.reload() = [0.0, 0.0, 1.0, 1.0]
HT| unpacked | still packed = False
HT| packed | test PNG bytes = 623
HT| packed | packed = True
HT| packed | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| packed | 2 render after blue was copied over the file = [1.0, 0.0, 0.0, 1.0]
HT| packed | Image.reload() returned None
HT| packed | 3 render after Image.reload() = [0.0, 0.0, 1.0, 1.0]
HT| packed | still packed = True
HT| packed_reopened | test PNG bytes = 623
HT| packed_reopened | packed = True
HT| packed_reopened | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| packed_reopened | 2 render after blue was copied over the file = [1.0, 0.0, 0.0, 1.0]
HT| packed_reopened | Image.reload() returned None
HT| packed_reopened | 3 render after Image.reload() = [0.0, 0.0, 1.0, 1.0]
HT| packed_reopened | still packed = True
```

**Test 49 — Cycles' texture cache when the source image changes.** Built as test 40, with `scene.render.use_texture_cache` on, in three arms: `auto_on` — `use_auto_generate_texture_cache` on throughout; `auto_off_later` — on for the first render, switched off before the image changes; `auto_off` — off from the start. Each: render; list the files in `blender_tx/` beside the image with their sizes and times; overwrite `tex.png` with blue; render; list again. Then, with blue on disk, a file was saved with the same scene and the arm's settings, and a freshly started Blender opened it, rendered, and listed `blender_tx/` again.

The first run of this test wrote blue over `tex.png` straight after the first render. Because an 8 by 8 render takes well under a second, the replacement landed in the same second as the red file had been written. In that run the `.tx` file's recorded time came back as a whole second (`mtime_ns=1792219629000000000`, for example), and a `stat` of `tex.png` afterwards showed the same whole second. What it gave:

```
HT| auto_on | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| auto_on | after render 1 | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1638 mtime_ns=1792219629000000000
HT| auto_on | 2 render after blue was copied over the image = [1.0, 0.0, 0.0, 1.0]
HT| auto_on | after render 2 | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1638 mtime_ns=1792219629000000000
HT| auto_on fresh | render = [1.0, 0.0, 0.0, 1.0]
HT| auto_off_later | 2 render after blue was copied over the image = [1.0, 0.0, 0.0, 1.0]
HT| auto_off_later fresh | render = [1.0, 0.0, 0.0, 1.0]
HT| auto_off | 2 render after blue was copied over the image = [0.0, 0.0, 1.0, 1.0]
HT| auto_off | after render 2 | blender_tx/ does not exist
HT| auto_off fresh | render = [0.0, 0.0, 1.0, 1.0]
```

So with a `.tx` made in the same second as the change, both the running Blender and a fresh one went on rendering the old picture. The test was then repaired by waiting two and a half seconds after the first render before replacing the file, and every arm run again in fresh folders. What came back:

```
HT| auto_on | use_texture_cache = True | use_auto_generate_texture_cache = True
HT| auto_on | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| auto_on | after render 1 | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1640 mtime_ns=1792219720000000000
HT| auto_on | tex.png mtime_ns before the swap = 1792219720797141709
HT| auto_on | tex.png mtime_ns after the swap  = 1792219723318556266
HT| auto_on | 2 render after blue was copied over the image = [0.0, 0.0, 1.0, 1.0]
HT| auto_on | after render 2 | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1640 mtime_ns=1792219723000000000
HT| auto_on fresh | use_texture_cache = True | use_auto_generate_texture_cache = True
HT| auto_on fresh | render = [0.0, 0.0, 1.0, 1.0]
HT| auto_on fresh | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1640 mtime_ns=1792219723000000000
HT| auto_off_later | use_texture_cache = True | use_auto_generate_texture_cache = True
HT| auto_off_later | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| auto_off_later | after render 1 | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1646 mtime_ns=1792219728000000000
HT| auto_off_later | use_auto_generate_texture_cache now = False
HT| auto_off_later | tex.png mtime_ns before the swap = 1792219728332237249
HT| auto_off_later | tex.png mtime_ns after the swap  = 1792219730851297757
HT| auto_off_later | 2 render after blue was copied over the image = [0.0, 0.0, 1.0, 1.0]
HT| auto_off_later | after render 2 | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1646 mtime_ns=1792219728000000000
HT| auto_off_later fresh | use_texture_cache = True | use_auto_generate_texture_cache = False
HT| auto_off_later fresh | render = [0.0, 0.0, 1.0, 1.0]
HT| auto_off_later fresh | blender_tx/tex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx size=1646 mtime_ns=1792219728000000000
HT| auto_off | use_texture_cache = True | use_auto_generate_texture_cache = False
HT| auto_off | 1 render = [1.0, 0.0, 0.0, 1.0]
HT| auto_off | after render 1 | blender_tx/ does not exist
HT| auto_off | tex.png mtime_ns before the swap = 1792219735863204001
HT| auto_off | tex.png mtime_ns after the swap  = 1792219738378719423
HT| auto_off | 2 render after blue was copied over the image = [0.0, 0.0, 1.0, 1.0]
HT| auto_off | after render 2 | blender_tx/ does not exist
HT| auto_off fresh | use_texture_cache = True | use_auto_generate_texture_cache = False
HT| auto_off fresh | render = [0.0, 0.0, 1.0, 1.0]
HT| auto_off fresh | blender_tx/ does not exist
```

With Auto Generate on, a `.tx` file appeared after the first render, its recorded time moved on after the source changed, and the second render showed the new colour. With Auto Generate switched off and the out-of-date `.tx` left in place, its time did not move, and the second render, and a fresh Blender's render, still showed the new colour. With Auto Generate off from the start, no `blender_tx/` folder was made.

**Test 50 — an external OSL script rewritten.** Built as test 40 but with Open Shading Language switched on (`scene.cycles.shading_system = True`) and the plane's emission colour coming from an external Script node (`mode = 'EXTERNAL'`) reading `t.osl`, which holds `shader t(output color C = 0) { C = color(1, 0, 0); }`; its output `C` wired to the emission colour; saved as `osl.blend`. First, two files were written a hundredth of a second apart and the gap between their recorded times printed. Then: render; the `.osl` rewritten on disk by Python to emit `color(0, 0, 1)`; render with nothing else done; the `.oso` files in the folder listed after each render; `bpy.ops.node.shader_script_update()` with nothing supplied, and with the biggest area turned into a Shader node editor showing the material's tree and the area, its main region, the Script node and the editor supplied. Then a fresh Blender opened `osl.blend` and rendered, and another did so after the `.oso` had been moved away.

The first run rewrote the `.osl` straight after the first render. Its record:

```
HT| clock: two writes 0.01 s apart got recorded times 11932925 ns apart
HT| ShaderNodeScript.use_auto_update = False
HT| script node outputs = ['C']
HT| 1 render, red script = [1.0, 0.0, 0.0, 1.0]
HT| .oso files after render 1 = [('t.oso', 649, 1792219789972153000)]
HT| .osl recorded time moved from 1792219789932643982 to 1792219790089020504
HT| 2 render after the script was rewritten to blue, nothing else done = [0.0, 0.0, 1.0, 1.0]
HT| .oso files after render 2 = [('t.oso', 644, 1792219790092827785)]
HT| bpy.ops.node.shader_script_update() with no editor = ERROR RuntimeError Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect
Info: OSL shader compilation succeeded
HT| bpy.ops.node.shader_script_update() with a Shader node editor = {'FINISHED'}
HT| fresh Blender, render = [0.0, 0.0, 1.0, 1.0]
HT| fresh Blender, render = [0.0, 0.0, 1.0, 1.0]
```

(the last line is the fresh Blender with the `.oso` moved away). In that run the `.oso` was written at `…789.97` seconds and the `.osl` rewritten at `…790.09`: a fraction of a second apart, but across a whole-second boundary. Because test 49 showed a result turning on whether a change falls in the same whole second, the test was repaired to wait two and a half seconds after the first render before rewriting the script, and run again in a fresh folder. What came back:

```
HT| clock: two writes 0.01 s apart got recorded times 11805550 ns apart
HT| ShaderNodeScript.use_auto_update = False
HT| script node outputs = ['C']
HT| save {'FINISHED'}
HT| 1 render, red script = [1.0, 0.0, 0.0, 1.0]
HT| .oso files after render 1 = [('t.oso', 651, 1792219814647302000)]
HT| .osl recorded time moved from 1792219814623083429 to 1792219817209088896
HT| 2 render after the script was rewritten to blue, nothing else done = [0.0, 0.0, 1.0, 1.0]
HT| .oso files after render 2 = [('t.oso', 647, 1792219817222151942)]
HT| bpy.ops.node.shader_script_update() with no editor = ERROR RuntimeError Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect
Info: OSL shader compilation succeeded
HT| bpy.ops.node.shader_script_update() with a Shader node editor = {'FINISHED'}
HT| fresh Blender, render = [0.0, 0.0, 1.0, 1.0]
HT| fresh Blender, render = [0.0, 0.0, 1.0, 1.0]
```

After the second fresh render, with the old `.oso` moved away, a new `t.oso` was in the folder. In both runs the rewritten script reached the very next render with nothing else done, and the compiled `.oso` beside it was rewritten during that render.

**Test 51 — an image's path written four ways, and whether its picture follows.** With no window, `red.png` and `blue.png` (4 by 4) were written by Blender, and four images loaded from `red.png` (`check_existing=False`), each having its pixels read once so its data is loaded. Then each was pointed at `blue.png` a different way: `Img_filepath` by assigning `filepath`; `Img_filepath_raw` by assigning `filepath_raw`; `Img_foreach` by `bpy.data.file_path_foreach()` with its default switches and a function returning the blue path for that image only; `Img_foreach_reload` the same with `flags={'RELOAD_EDITED'}`. Straight afterwards, with nothing else done, each image's `has_data` and first pixel were read. What came back:

```
HT| filepath        loaded, has_data = True, first pixel = [1.0, 0.0, 0.0, 1.0]
HT| filepath_raw    loaded, has_data = True, first pixel = [1.0, 0.0, 0.0, 1.0]
HT| foreach         loaded, has_data = True, first pixel = [1.0, 0.0, 0.0, 1.0]
HT| foreach_reload  loaded, has_data = True, first pixel = [1.0, 0.0, 0.0, 1.0]
HT| foreach (flags as default) returned None
HT| foreach (flags={'RELOAD_EDITED'}) returned None
HT| filepath        path now blue.png | has_data = False | first pixel = [0.0, 0.0, 1.0, 1.0]
HT| filepath_raw    path now blue.png | has_data = True | first pixel = [1.0, 0.0, 0.0, 1.0]
HT| foreach         path now blue.png | has_data = True | first pixel = [1.0, 0.0, 0.0, 1.0]
HT| foreach_reload  path now blue.png | has_data = False | first pixel = [0.0, 0.0, 1.0, 1.0]
```

**Test 53 — Revert after the blend-file is replaced on disk.** From an empty start, `scene.blend` was saved holding one mesh object `CUBE_A`, and `other.blend` holding `CUBE_B`. `scene.blend` was opened with no window; `other.blend` was copied over it by Python, two and a half seconds were waited, the objects read, and `bpy.ops.wm.revert_mainfile()` called. As a control, both files were built again and `scene.blend` opened and reverted with nothing replaced. What came back:

```
HT| replace | opened, objects = ['CUBE_A']
HT| replace | other.blend copied over scene.blend; objects held = ['CUBE_A']
HT| replace | revert_mainfile = {'FINISHED'}
HT| replace | after revert, objects = ['CUBE_B']
HT| control | opened, objects = ['CUBE_A']
HT| control | revert_mainfile = {'FINISHED'}
HT| control | after revert, objects = ['CUBE_A']
```

**Test 52 — an image replaced while an animation render is running.** Test 40's scene as a twelve-frame animation, 64 by 64 at 256 samples, frames written to `//frames/f_`, saved as `anim.blend` with red on disk. A timing render with nothing replaced, run under `/usr/bin/time -p`, took `real 1.32` seconds and wrote its frames between `00:00.385` and `00:00.883` on Blender's own clock. Then, in one command, the animation was started with `-a` in the background, the shell paused a fixed time, copied blue over `tex.png`, and waited for that render to finish; afterwards the centre pixel of each frame was read.

With a pause of 0.65 seconds the copy landed before the first frame (the shell printed `swapped`, then `render exit 0`), and all twelve frames read `[0.0, 0.0, 1.0, 1.0]`. With red copied back and a pause of 1.05 seconds:

```
swapped
render exit 0
00:00.389  render           | Saved: '<WORK>/e52/frames/f_0001.png'
00:00.434  render           | Saved: '<WORK>/e52/frames/f_0002.png'
00:00.479  render           | Saved: '<WORK>/e52/frames/f_0003.png'
00:00.524  render           | Saved: '<WORK>/e52/frames/f_0004.png'
00:00.570  render           | Saved: '<WORK>/e52/frames/f_0005.png'
00:00.621  render           | Saved: '<WORK>/e52/frames/f_0006.png'
00:00.670  render           | Saved: '<WORK>/e52/frames/f_0007.png'
00:00.716  render           | Saved: '<WORK>/e52/frames/f_0008.png'
00:00.762  render           | Saved: '<WORK>/e52/frames/f_0009.png'
00:00.808  render           | Saved: '<WORK>/e52/frames/f_0010.png'
00:00.853  render           | Saved: '<WORK>/e52/frames/f_0011.png'
00:00.899  render           | Saved: '<WORK>/e52/frames/f_0012.png'
HT| f_0001.png [1.0, 0.0, 0.0, 1.0]
HT| f_0002.png [1.0, 0.0, 0.0, 1.0]
HT| f_0003.png [1.0, 0.0, 0.0, 1.0]
HT| f_0004.png [1.0, 0.0, 0.0, 1.0]
HT| f_0005.png [1.0, 0.0, 0.0, 1.0]
HT| f_0006.png [1.0, 0.0, 0.0, 1.0]
HT| f_0007.png [1.0, 0.0, 0.0, 1.0]
HT| f_0008.png [0.0, 0.0, 1.0, 1.0]
HT| f_0009.png [0.0, 0.0, 1.0, 1.0]
HT| f_0010.png [0.0, 0.0, 1.0, 1.0]
HT| f_0011.png [0.0, 0.0, 1.0, 1.0]
HT| f_0012.png [0.0, 0.0, 1.0, 1.0]
```

Every pixel of every frame was then read and the distinct colours in each frame listed: frames 1 to 7 held only shades of red (fourteen distinct values, from `(0.898, 0.106, 0.106, 1.0)` at the plane's softened edge to `(1.0, 0.0, 0.0, 1.0)`), and frames 8 to 12 only the matching shades of blue. No frame held both.

---

## Things a person could settle by hand

**Pruned by the hand tests of 2026-10-17.** The researcher's list held ten items. Items 1, 2 and 4 to 9 were run on the Creator's Mac on 2026-10-17 and their results are written into the answers above, under *Observed — hand tests on the Creator's Mac, 2026-10-17*: item 1 as tests 1 and 7, item 2 as test 8, item 4 as test 36, item 5 as tests 10, 20 to 22 and 28, item 6 as test 13, item 7 as tests 4 and 5, item 8 as tests 40, 41 and 49, item 9 as test 9. Item 10 was run in part, as test 3. The numbers below are the researcher's own. What is still open:

3. **Question 2 — reading while rendering.** Start two Blender 5.3 copies. In the first, open a file and start Render ‣ Render Animation. While it runs, in the second, open the same file, then save it under a new name. Note any error, warning or change in the first copy's render. Repeat with the second copy only opening the file. *Not run on 2026-10-17: it needs two Blenders at once, and that sitting was allowed one at a time on the Creator's Mac (16 GB of memory). It is written out, ready to run with no window, in `Audits/Blender Research/Reading a file/Hand tests — 5.3.md` under* Cannot be run.
10. **Question 1 — preview without Blender running.** Save a file with Save Preview Images on. Quit Blender. In the system file browser (Finder, Explorer, a Linux file manager), switch to icon view. Note whether the scene preview shows as the file's icon. *Run in part on 2026-10-17 (test 3, under question 1): Blender's own preview loader found a scene preview in a file saved with no window and in one saved from the window, and none in one saved with previews off. What the Finder shows was not reached: macOS's Quick Look, asked from the terminal, never answered, and the Creator's own Blender was open throughout, so "Blender not running" could not be arranged. Explorer and Linux file managers need machines this sitting did not have.*

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Reading a file, Blender 5.3

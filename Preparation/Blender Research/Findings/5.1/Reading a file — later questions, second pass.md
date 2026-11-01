# Reading a file — later questions, second pass (Blender 5.1)

Sources consulted, all as taken on **2026-07-30**: the Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes, and the Cycles files supplied for 5.1. The snapshot date, 2026-07-30, is the date read on every citation below. *(Citation check, 2026-08-09: this document carried "date read 2026-08-09" on every citation — the day the researcher worked, not the day the copy was taken. Levelled to the snapshot date, which is what the other seven documents in this round use. The dates on the hand tests, which really were run on 2026-08-09, are untouched.)*

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Questions answered here: 10 and 11 only.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Reading taken.** I read "a search for a missing reference" as the search Blender itself performs to repair broken links to external files — the command the manual calls **Find Missing Files** (`bpy.ops.file.find_missing_files`). That is the only searching repair mechanism in the sources. The other repair mechanism, **Relocate**, is not a search: the person picks the replacement file by hand. I cover it briefly at the end because the question could conceivably be read as covering it, but I do not think that is the reading meant.

The question has three parts and they do not all have the same answer, so I label each part separately.

### Part A — What makes a file a possible match

**LABEL: not in the sources.**

The manual describes what the search covers, but never states the test a candidate file must pass. The full description is:

> "In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — date read 2026-07-30.

"Every missing file found in the search will be recovered" tells you the search is recursive and that found files get repaired. It does not say what "found" means — whether a candidate has to have the same file name as the missing one, the same name ignoring letter case, the same extension, the same size, or anything else. Nothing anywhere in the three documentation sets states the rule.

The one adjacent thing the sources do state is an option on the operator:

> "find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.file.html — date read 2026-07-30.

That tells you the operator by default confines itself to references that are missing, and can be told to act on all references in the search path instead. It still says nothing about the matching test. The manual does not document this option at all — the "Find Missing Files" text quoted above mentions no options.

One documented fact bears on matching but does not settle it: Blender ships a helper for case-insensitive path resolution,

> "Resolve a case insensitive path on a case sensitive system,
> returning a string with the path if found else return the original path."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.path.html (`bpy.path.resolve_ncase`) — date read 2026-07-30. Nothing says Find Missing Files uses it, so I draw no conclusion from it.

### Part B — Which one is chosen when several are possible

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or the Cycles files addresses more than one candidate. There is no statement about a first match, a nearest match, a shallowest directory, alphabetical order, a warning, a prompt, or any tie-break at all. The manual's wording ("Every missing file found in the search will be recovered") does not distinguish one candidate from several.

### Part C — What location is written back

**LABEL: documented.**

> "Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — date read 2026-07-30.

So the repaired reference is stored as a full path from the system's root, not as a path relative to the blend-file. The manual defines the two forms on the same page:

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

Same page and date.

And the manual's separate description of relative paths:

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Source: manual, https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — date read 2026-07-30.

### The other, hand-picked repair route (for completeness, not a search)

For a linked blend-file, the repair is a manual choice rather than a search:

> "When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

and

> "If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored."

Source: manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — date read 2026-07-30.

The matching operator is documented as "Relocate the given library to one or several others" (`bpy.ops.wm.lib_relocate`), with a `relative_path` parameter; there is also `bpy.ops.outliner.lib_relocate()`, "Relocate the library under cursor". Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.wm.html and https://docs.blender.org/api/5.1/bpy.ops.outliner.html — date read 2026-07-30. The API documentation does not say what `relative_path` does to the stored location beyond its name; I do not read anything further into it.

**Not covered.** I cannot say what makes a file a candidate, nor what happens with two or more candidates — including whether Blender reports the ambiguity, silently takes one, or skips. I also cannot say whether the search looks only for the file's own name or also considers the directory structure below it; whether `find_all` changes the matching rule as well as the scope; whether the recursion has a depth limit; whether the search follows symbolic links; what happens to references inside linked libraries as opposed to the current file; or which kinds of reference the search covers at all (images, videos, sounds, fonts, caches, linked blend-files). Part C is settled only for what this one command writes; other commands write other forms.

### Settled by hand test — observed, 2026-08-09

Eight tests were built for this question and run in **Blender 5.1.2** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 1 to 8.

**What was done, and to what.** A blend-file holding one image texture whose file was then deleted, reopened, and repaired by calling `bpy.ops.file.find_missing_files(directory=<folder>)` from a script. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, and no window was open at any point.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on all eight tests.**

**The command runs with no window at all.** Both machines printed `HT| s_same | operator returned = "{'FINISHED'}"`, and the recorded location changed.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension — `tex.jpg` for a missing `tex.png` — was not, and neither was a file of a different name holding the same picture. Both printed the same line as a folder holding nothing at all: `Warning: Could not find 'tex.png' in '<folder>'`.

**Letter case has to match, on both machines.** With only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it — `HT| s_upper | filepath after = '//orig/tex.png'` — while the control run with `tex.png` on disk recovered it on both. The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run. Wherever the candidates differed in size, the recorded location ended up pointing at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them: `HT| sA | filepath after = '//sA/zzz/tex.png'` against `HT| sB | filepath after = '//sB/aaa/tex.png'`, the larger file winning both times. A file three folders down beat one at the top of the search folder on a single byte, and a 3981-byte file beat a 260-byte one from either position. Where two candidates were exactly the same size, `aaa` was taken rather than `zzz`.

> **The byte sizes measured again.** **Observed**, 2026-08-09, Blender 5.1.2. Entry 28 of the hand-test list ran the tie-break arrangement once more and recorded the exact size of every candidate. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), this version wrote **261 bytes** for the red and blue 8 by 8 pictures, **260** for the green and white, **274** for the 16 by 16 one and **3981** for the 64 by 64 one. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), this version wrote the same four figures. Across the four versions: 4.5.12 LTS and 5.0.1 both wrote 188, 187, 201 and 3908; 5.1.2 and 5.2.0 LTS both wrote 261, 260, 274 and 3981. The choice came out the same at both sets of sizes and on both machines — the larger candidate won from either folder, and where the two candidates were byte-for-byte the same size `aaa` was taken. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Nothing is said about there having been more than one.** No line about ambiguity appeared in any arrangement. The control is what makes that worth recording: with nothing to find, the same terminal carried `Warning: Could not find 'tex.png' in '<folder>'`, so this is a silence about ambiguity and not an unwatched channel.

**The location written back began `//`, not at the system root.** On both machines the recovered location came back as `//s_same/deep/tex.png`, with `HT| s_same | starts with // = True`. That is not what the manual sentence quoted above describes. What is recorded here is what appeared; nothing in the test separates the command's own behaviour from a preference that might turn the path relative afterwards.

**The recovery rewrites the location without loading the data.** Straight after a successful recovery, `HT| s_same | has_data = False`. Reading the pixels afterwards returned the new file's colour, so the file is read when it is next wanted rather than at the moment of the repair.

**How deep it searches.** A file eight folders below the chosen one was found, on both machines.

**Which kinds of recorded location it reaches: all six that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic archive had all six rewritten by a single run, on both machines — the linked blend-file included, although the manual gives libraries a separate repair tool of their own.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the sound, the font, the movie clip, the linked blend-file and the Alembic archive were made. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again, with the making of all six written out.** **Observed**, 2026-08-09, Blender 5.1.2. Entry 26 of the hand-test list rebuilt this test with every step named: an 8 by 8 red PNG written by Blender itself; a tenth of a second of silence written by Python's own `wave` module at one channel, two bytes a sample, 8000 samples a second; a font copied from whichever the machine has; a single-frame image sequence `clip_0001.png`; a blend-file holding one collection named `LibCol` with a default cube in it, linked and instanced; and an archive exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` and everything else left at the operator's own defaults. All six files were then moved into one search folder and the command was called once. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), the font taken was `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`, and all six recorded locations were rewritten to `//found/…`, every one of them beginning `//`. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), the font taken was `/System/Library/Fonts/Supplemental/Arial.ttf`, and the same six locations were rewritten, every one beginning `//`. The control was run on each machine as well: with the search folder empty, none of the six moved and the terminal carried six separate `Warning: Could not find '<name>' in '<folder>'` lines, one per kind. **This matches the result recorded above.** *(Note added by the hand tests, second batch, 2026-08-09.)*

**`find_all` rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a working reference moved too, from `//keep/good.png` to `//search/good.png`.

**A packed resource's recorded location is rewritten as well, and it stays packed.** `HT| after | p.png = ('//found/p.png', True)`.

**Whether a recovery can be undone is not settled.** With no window, `bpy.ops.ed.undo()` refused — `Operator bpy.ops.ed.undo.poll() failed, context is incorrect` — even after `bpy.ops.ed.undo_push()` had returned `{'FINISHED'}`, and again with a 3D Viewport context built by hand. Neither machine can answer the version of that test which needs a window: the rented Linux machine has no display server and the Creator ran nothing this round.

**Outside the interface.** **Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved. Everything above about matching, the tie-break and the form of the written-back location was established that way, so all of it holds outside the interface. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. So the adjacent fact about `bpy.path.resolve_ncase` does not describe this command. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked, for question 10

Searched the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` (case-insensitive) for: `missing files`, `missing file`, `find_missing`, `find missing`, `report_missing`, `missing` (every hit in the manual read or dismissed by subject — the surviving hits were the topbar page, the VSE "Show Missing Media" option, the VSE image-sequence placeholders passage, and the linked-libraries placeholder passage), `broken link`, `recovered`, `Find All`, `search path`, `recursiv`, `cannot be found`, `could not be found`, `cannot find`, `not found`, `unable to find`, `does not exist`, `external files`, `same name`, `file name matches`, `matching name`, `ambiguous`, `first match`, `alphabetical order`, `duplicate file`, `more than one file`, `several matches`, `candidate`, `relative paths`, `modification time`, `timestamp`, `mtime`.

Pages opened in full or in part: the manual's Top Bar page (the whole File and External Data sections), Save & Load / Open & Save (`files/blend/open_save`), File Paths (`files/file_paths` — turned out to be about path templates, ruled out), Packed Data (`files/blend/packed_data`), Link & Append (`files/linked_libraries/link_append`), the File Browser page (`editors/file_browser` — its "Recursions" setting is about listing files in the browser, ruled out as unrelated), and the Preferences pages for File Paths and Save & Load. In the API documentation: the whole of `bpy.ops.file`, the relevant parts of `bpy.ops.wm` and `bpy.ops.outliner`, `bpy.path`, `bpy.utils`.

Ruled out and why: the release notes for 5.1 — the only hits touching this area are two bug-fix list entries, "Crash on relocate linked file" and "External Multires .btx filepath not cleared when re-packing, reports as missing even when empty" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/, date read 2026-07-30); neither describes matching or choosing. The Pipeline & I/O and Core release-note pages (https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ and .../core/) were read in full and contain nothing on missing files or path repair. The Cycles files were searched for `missing` and `reload` and `filepath`; the supplied Cycles tree contains only `kernel` and `util` source plus the add-on's Python (`__init__.py`, `engine.py`, `properties.py`, `ui.py`, `operators.py`, `osl.py`, `presets.py`, `version_update.py`, `camera.py`). There is no image-loading or path-resolution code in it. The only related thing there is a constant for how an unavailable image is shaded — `#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)` in `cycles_5.1/source/util/types_image.h`, line 13 — which is about drawing, not about finding files. I also checked the manual's `_images` folder for a screenshot of the External Data menu or the Find Missing Files file browser. **Documented:** `_images/interface_window-system_topbar_menus.png`, on https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30. Visible in it: just the top bar strip with the Blender icon and the File, Edit, Render, Window and Help menu names — no submenu. So it settles nothing here. *(Citation check, 2026-08-09: labelled **documented** and put into the Method's form for a picture. I opened the same file and it shows exactly that.)*

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**LABEL: documented, for the general shape and for the specific reload commands. Not in the sources, for whether Blender ever picks up a change by itself in a render.**

The sources are consistent on the general shape: Blender holds what it has read in memory, does not generally notice that the file behind it has changed, and provides an explicit reload command per kind of data. The clearest single statement is in the Video Sequencer:

> "Refresh All Ctrl-E
>
> Reloads external files and refreshes the current frame preview.
> This is useful when you modified an external file or made a change in a scene that Blender
> didn't detect."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html — date read 2026-07-30. (The same wording appears on the Video Sequencer preview header page, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/header.html.)

That data read from disk is kept in memory rather than re-read is stated outright for images:

> "The Image data-block is a shallow wrapper around image or video file(s)
> (on disk, as packed data, or generated).
>
> All actual data like the pixel buffer, size, resolution etc. is
> cached in an imbuf.types.ImBuf image buffer (or several buffers
> in some cases, like UDIM textures, multi-views, animations…)."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Image.html — date read 2026-07-30.

### The explicit reload commands, by kind of data

**Images.**

> "Reload
>
> Reloads the file. Useful when it has been reworked in an external application."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/image/image_settings.html — date read 2026-07-30.

> "Reload
> …
> Menu:
>
> Image ‣ Reload
>
> Shortcut:
>
> Alt-R
>
> Reload the image from the file on drive."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/image/editing.html — date read 2026-07-30.

In Python: `bpy.ops.image.reload()`, "Reload current image from disk" (https://docs.blender.org/api/5.1/bpy.ops.image.html) and the data-block method `Image.reload()`, "Reload the image from its source path" (https://docs.blender.org/api/5.1/bpy.types.Image.html) — date read 2026-07-30.

**Linked blend-files (libraries).**

> "When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file."

Source: manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — date read 2026-07-30.

In Python: `bpy.ops.wm.lib_reload(...)`, "Reload the given library" (https://docs.blender.org/api/5.1/bpy.ops.wm.html), and the data-block method `Library.reload()`, "Reload this library and all its linked data-blocks" (https://docs.blender.org/api/5.1/bpy.types.Library.html) — date read 2026-07-30.

**Text files.** This is the one case where the sources describe Blender noticing the change itself:

> "Resolve Conflict
>
> Resolves modified file conflicts when an external text file is updated from another program.
>
> Reload from Disk
>
> Opens the file from drive again, overriding any local changes.
>
> Make Text Internal
>
> Converts the external text data-block into an internal one.
>
> Ignore
>
> Hides the warning message until the external text file is modified externally again."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/text_editor.html — date read 2026-07-30. The same page also documents "Reload Alt-R — Reopens (reloads) the current buffer (all non-saved modifications are lost)." In Python: `bpy.ops.text.reload()`, "Reload active text data-block from its file", and `bpy.ops.text.resolve_conflict(*, resolution='IGNORE')` (https://docs.blender.org/api/5.1/bpy.ops.text.html), date read 2026-07-30.

**Video Sequencer strips.**

> "Reload Strips Alt-R
>
> Reloads the strips from their external saved location.
>
> Reload Strips and Adjust Length Shift-Alt-R
>
> Reloads the strips from their external saved location and re-adjusts the strip duration."

Source: manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/editing.html — date read 2026-07-30. In Python: `bpy.ops.sequencer.reload(*, adjust_length=False)`, "Reload strips in the sequencer", and `bpy.ops.sequencer.refresh_all()` (https://docs.blender.org/api/5.1/bpy.ops.sequencer.html), date read 2026-07-30.

**Movie clips, caches, OSL scripts.** `bpy.ops.clip.reload()`, "Reload clip"; `bpy.ops.cachefile.reload()`, "Update objects paths list with new data from the archive" (https://docs.blender.org/api/5.1/bpy.ops.clip.html and https://docs.blender.org/api/5.1/bpy.ops.cachefile.html). For an OSL shader script node the manual says "Script Node Update — Reloads the text file data-block, creating new inputs and outputs if needed" (https://docs.blender.org/manual/en/5.1/render/shader_nodes/utilities/script.html), with `bpy.ops.node.shader_script_update()`, "Update shader script node with new sockets and options from the script" (https://docs.blender.org/api/5.1/bpy.ops.node.html). All date read 2026-07-30.

**Everything at once.** The manual's advice after repairing broken links is the blunt version:

> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — date read 2026-07-30.

Reopening the blend-file itself is documented as "Revert — Reopens the current file to its last saved version" (same page), and in Python as `bpy.ops.wm.revert_mainfile(*, use_scripts=False)`, "Reload the saved file" (https://docs.blender.org/api/5.1/bpy.ops.wm.html), date read 2026-07-30.

### A file that appears rather than changes

The Video Sequencer documents the case where a file that was absent when the sequence was read is later written to disk:

> "When the missing frames are rendered out or placed into the folder with the rest of the image sequence,
> you can refresh
> the Sequencer and the placeholder frames will be filled in."

Source: manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/image.html — date read 2026-07-30. Again the fix is an explicit refresh, not automatic.

### Two documented cases where a change on disk is not picked up at all

**Packed data.** If external data has been packed into the blend-file, the file on disk stops mattering:

> "Pack Resources
>
> Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — date read 2026-07-30. Unpacking a file with "Use <original file path>" is documented the same way: "If the original file path still exists, mark it as unpacked. Note that it won't be automatically reloaded from the drive." (https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html, same date.)

**Asset libraries** are the one thing in the sources described as keeping itself current with files changing on disk:

> "Loading an asset library for the first time may take a while, but the next time it is loaded should be
> significantly faster. Blender generates an index of all assets contained inside an asset library,
> and keeps it up-to-date as files are modified within it. The indices are stored in the Local Cache Directory."

Source: manual, https://docs.blender.org/manual/en/5.1/files/asset_libraries/introduction.html — date read 2026-07-30. Note this is about the index of what assets exist, not about data already read into a scene.

### What has to happen before a later render uses the changed file

On the strength of the above: for anything already read, one of the reload commands for that kind of data has to be run, or the blend-file has to be reopened (Revert, or opening it again). The sources never describe a render itself re-reading a changed external file.

Two things I checked that bear on a render specifically and do **not** settle it:

- Cycles' Persistent Data: "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender." (manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html); the property is `use_persistent_data`, "Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)" (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html). Both date read 2026-07-30. Whether this changes anything about re-reading a changed external file is **not in the sources**.
- The image "Auto Refresh" option, `ImageUser.use_auto_refresh`, "Always refresh image on frame changes (default False)" (https://docs.blender.org/api/5.1/bpy.types.ImageUser.html, date read 2026-07-30). The manual describes it for the Image Texture node as "Update the video texture in the 3D Viewport when moving through the timeline" (https://docs.blender.org/manual/en/5.1/render/shader_nodes/textures/image.html) and in the Image editor as "Play the video in the Image Editor when the scene animation is playing" (https://docs.blender.org/manual/en/5.1/editors/image/image_settings.html). That second page — the Image editor's Image Settings page, not the node page — introduces its own block of settings with "The options below are for preview purposes only; they don't affect the 3D Viewport or the render." (https://docs.blender.org/manual/en/5.1/editors/image/image_settings.html — read 2026-07-30), so that caveat covers the Image editor's settings and says nothing about the Image Texture node's Auto Refresh. *(Citation check, 2026-08-09: that sentence was attributed here to "the first of those pages", which is the Image Texture node page. It is not on that page; it is on the Image editor's Image Settings page. Address corrected, and the sentence rewritten so it no longer says the caveat covers the node page's setting.)* The word "refresh" is never defined as re-reading from disk, and the setting is about moving between frames of a video or sequence, not about a file being replaced. I draw no conclusion from it.

**Not covered.** The sources do not say what happens at the moment the file changes — whether a partly written or truncated file is detected, whether Blender holds an open handle that would block replacement or deletion on some systems, or whether replacing a file underneath a running Blender can crash it or produce a corrupt frame. Nothing states whether Blender ever checks a modification time. Nothing states what a reload does if the file is now a different size, format or colour depth, or if it has been deleted. Nothing describes what a render started while a reload is in progress does. The behaviour of a fresh Blender process is nowhere stated in these terms either; the sources describe opening a blend-file but never say in so many words that opening it reads the current contents of its external references from disk.

### Settled by hand test — observed, 2026-08-09

Fifteen tests were built for this question and run in **Blender 5.1.2** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 9 to 23.

**What was done, and to what.** A scene whose only visible thing is a plane lit by an emission shader taking its colour from one image file, rendered by Cycles on the processor at eight pixels square with one sample and a Standard view transform, so a render can be read as a single pixel: red comes back as `[0.996, 0.0, 0.0, 1.0]` and blue as `[0.0, 0.0, 0.996, 1.0]`. Files were replaced by the operating system with `shutil.copyfile`, never by Blender, and every replacement was followed by two and a half clear seconds so that a modification time recorded only to the second is plainly newer. Pictures were compared as pictures and never as files. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, with no window open.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on every one of the fifteen tests.**

**An image changed on disk reaches the next render inside a running Blender, with nothing done.** Render one was `[0.996, 0.0, 0.0, 1.0]`; the file was overwritten with a blue one; render two, with nothing touched in between, was `[0.0, 0.0, 0.996, 1.0]`. `Image.reload()` ran without complaint and returned `None`; `bpy.ops.image.reload()` returned `{'CANCELLED'}` with no Image editor open and `{'FINISHED'}` with one built by hand *(Citation check, 2026-08-09: an Image editor built by hand is not enough on its own — on the repeat it returned `{'CANCELLED'}` until the image was also put into that editor, and only then `{'FINISHED'}`. Step added by this check.)* — and a further swap was picked up at the next render even in the run where the operator had done nothing.

**A freshly started Blender reads the file as it stands.** A second Blender process opening the same blend-file after the swap rendered `[0.0, 0.0, 0.996, 1.0]` where the first had rendered `[0.996, 0.0, 0.0, 1.0]`.

**Persistent Data stops the change being picked up.** With `use_persistent_data` off, the second render showed the new picture. With it on, the second render showed the old one, and it still showed the old one after `Image.reload()`. The setting was present with a default of `False`.

**A linked blend-file behaves the other way round from an image.** With `lib.blend` replaced on disk under a running Blender, the next render was unchanged — `HT| 2 render after the library was replaced, nothing else done = [0.996, 0.0, 0.0, 1.0]`. `Library.reload()` brought the new library in, and a freshly started Blender also rendered it. `bpy.ops.wm.lib_reload()` failed both ways it was tried, printing `Error: Not a library`; that operator's argument was not worked out and this test does not settle how to call it.

**A Video Sequencer strip does not pick the change up in this version.** Render one was `[1.0, 0.0, 0.0, 1.0]`, the file was replaced, and render two with nothing else done was still `[1.0, 0.0, 0.0, 1.0]`. Both sequencer commands refused: `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()` each raised `Operator ... poll() failed, context is incorrect`, so with no window open there was nothing left to try and the old picture went on being rendered.

**Auto Refresh makes no difference to a changed frame file.** An image sequence was rendered at frame 1 (red) and frame 2 (green) to show the sequence was being followed, then `f_0001.png` was overwritten with white. Frame 1 rendered white at once — with `use_auto_refresh` off and with it on, identically.

**A movie clip does not pick the change up, and its reload does re-read the file.** `clip.size` stayed `(8, 8)` after the file was replaced with a 16 by 16 one. `bpy.ops.clip.reload()` returned `{'CANCELLED'}` with no Clip editor; with one built by hand it returned `{'FINISHED'}` and `clip.size` became `(16, 16)`.

**An Alembic archive did not pick the change up at any point.** A cache file replaced with one holding a cube of twice the size measured -1 to 1 throughout: after the swap, after `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context, after the same command returned `{'FINISHED'}` with an `edit_cachefile` context built by hand, after the recorded location was written back to itself, and after the frame was stepped away and back.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the Alembic archive was made, nor how it was attached to the object. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again with the set-up written out, and it came out differently.** **Observed**, 2026-08-09, Blender 5.1.2. Entry 27 of the hand-test list names both missing steps. Two archives were exported, from a cube of size 2 and a cube of size 4, with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)`; the smaller was brought in with `bpy.ops.wm.alembic_import(filepath=…, as_background_job=False, always_add_cache_reader=True)`, which gives the object a Mesh Sequence Cache modifier and a Transform Cache constraint, both printed back from the file. **The importer's own defaults were run as an arm of their own**, on both machines and in all four versions. The object came back with `Cube modifiers = []` and `Cube constraints = []` — no cache reader at all — and measured -1 to 1 at every one of the six points, which is the reading above; but a freshly started Blender opening that file after the swap also measured -1 to 1, on both machines, and a fresh Blender cannot be showing a stale archive. In that same fresh process the replacement archive imported on its own measured -2 to 2, so the archive was fine and the arrangement was watching an object that nothing tied to it. With the cache reader attached, on the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): after the swap with nothing done, -1 to 1; `bpy.ops.cachefile.reload()` with no context `{'CANCELLED'}`, -1 to 1; the same command with an `edit_cachefile` context built by hand `{'FINISHED'}`, and the evaluated mesh then measured **-2 to 2**, dimensions `(4.0, 4.0, 4.0)`; it stayed at -2 to 2 after the recorded location was written back to itself and after the frame was stepped away and back. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same readings at all six points. The archives were 3115 bytes each on the Linux machine and 3090 bytes each on the Mac — the same size as one another, which is why the mesh had to be measured. The controls held on both machines: a freshly started Blender opening the same blend-file after the swap measured -2 to 2, and the replacement archive imported on its own measured -2 to 2, so it really did hold a cube of twice the size. The result above is left exactly as it stands. Both runs stand and neither has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Blender does notice a changed external text file, and the conflict can be resolved with no window.** `Text.is_modified` was `False` while the two copies agreed and `True` once the file on disk differed. The text Blender held did not change until a reload was run. `bpy.ops.text.reload()` refused with no Text editor and returned `{'FINISHED'}` with one built by hand; so did `bpy.ops.text.resolve_conflict(resolution='RELOAD')`, which brought the new text in.

**A packed image ignores the file on disk.** With the image packed, the render went on showing the old picture after the file was replaced, and `Image.reload()` — which raised no error — did not bring the disk file in either. The same swap with the image left unpacked did reach the render, which is what shows the swap bit.

> **Unsettled — the citation check's repeat disagreed.** Repeating this on both machines and in all four versions, with the image packed and the blend-file saved and reopened: render one red `[0.659, 0.012, 0.02, 1.0]`, render two after the swap still red, and then, after `Image.reload()`, render three **blue** `[0.004, 0.004, 0.506, 1.0]` — with the image still packed. The control with the image left unpacked went red then blue on both runs. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time.** **Observed**, 2026-08-09, Blender 5.1.2. Entry 24 of the hand-test list ran three arms: the image left unpacked, the image packed with the blend-file saved but *not* reopened, and the image packed with the blend-file saved *and* reopened — the one difference the two earlier records name. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): the unpacked control rendered `[0.996, 0.0, 0.0, 1.0]`, then `[0.0, 0.0, 0.996, 1.0]` after the swap, so the swap bit; packed and not reopened rendered red, red, then **`[0.0, 0.0, 0.996, 1.0]`** after `Image.reload()`, with `packed_file is not None` still `True`; packed and reopened gave the same three renders and stayed packed. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three arms and the same nine renders. The test PNG was 261 bytes in this version. `Image.reload()` returned `None` and raised nothing every time. **This third run matches the citation check's and not the result recorded above.** Whether the blend-file was reopened after packing made no difference on either machine, so that is not what separates the two earlier runs, and this run offers nothing in its place. All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Cycles' texture cache does not exist in this version.** `RenderSettings.use_texture_cache` and `RenderSettings.use_auto_generate_texture_cache` were both absent, and `generate_texture_cache` and `clear_texture_cache` were both absent from the real list of `bpy.ops.render` operators.

**A changed external OSL script reaches the next render with nothing done.** `ShaderNodeScript.use_auto_update` was present with a default of `False`. The first render with the script emitting red was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue and the very next render was `[0.0, 0.0, 0.996, 1.0]`. `bpy.ops.node.shader_script_update()` refused every time it was called, with and without a Shader node editor built by hand, so no update command ever succeeded — and the change reached the render anyway.

> **Unsettled — the citation check's repeat disagreed.** Repeating this — a Script node in External mode emitting red, the `.osl` on disk rewritten to emit blue, the very next render with nothing else done — gave: Creator's Mac 4.5 blue, 5.0 blue, 5.1 red, 5.2 red; rented Linux machine 4.5 red, 5.0 red, 5.1 red, 5.2 blue. The two machines did not agree with each other. The other two parts of this entry did repeat exactly: `use_auto_update` present with a default of `False`, and `bpy.ops.node.shader_script_update()` refusing in every arrangement. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time, with the compiled file watched and the clock measured.** **Observed**, 2026-08-09, Blender 5.1.2. Entry 25 of the hand-test list. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): render 1 with the red `.osl` was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue; render 2, with nothing else done, was **`[0.0, 0.0, 0.996, 1.0]`**; and the compiled `t.oso` sitting beside the `.osl` went from 485 bytes to 496 during that very render, its modification time moving on with it. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three renders, and `t.oso` went from 775 bytes to 848 during render 2. On both machines `bpy.ops.node.shader_script_update()` raised `Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`, with and without a Shader node editor built by hand, so no update command ever succeeded. **The clock was measured rather than assumed.** On the Linux machine two writes a hundredth of a second apart were given recorded times 10,999,745 nanoseconds apart, and the `.osl`'s own recorded time moved from `1786297291622310753` to `1786297295167228584` at the rewrite. On the Mac the same two writes were 13,211,235 nanoseconds apart, and the `.osl`'s recorded time moved from `1786297410208915581` to `1786297412998347965`. Both machines record a modification time to the nanosecond, and nothing in this run was hidden inside one tick of it. Two further controls held on both machines: a genuinely separate Blender process opening the saved blend-file rendered blue, and rendered blue again after every `.oso` had been deleted, so blue was reachable. **This third run matches the result recorded above and not the citation check's.** All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**`bpy.data.file_path_foreach` exists in this version.** `HT| bpy.data.file_path_foreach present = True`. Every call made to it errored, because the keyword this round used was wrong: `TypeError("file_path_foreach() got an unexpected keyword argument 'flag'. Did you mean 'flags'?")`. The error is recorded rather than repaired: this settles that the function is there, and names the spelling a later test should use, and does not settle what `RELOAD_EDITED` does.

**A file replaced while a render is already running takes effect from a later frame onward.** A twelve-frame animation had its image replaced part-way through: the frames up to a point held the old picture and every frame after it held the new one. No frame came out part old and part new. Which frame the change lands on moves from run to run with the timing.

**Revert takes the blend-file as it stands on disk.** With `scene.blend` opened, then replaced on disk by a different file, Blender still held the old contents until `bpy.ops.wm.revert_mainfile()` was run; it returned `{'FINISHED'}` with no window open and afterwards Blender held the replaced file's object, `['CUBE_B']`. A control Revert with nothing replaced gave `['CUBE_A']` back.

**Outside the interface.** **Observed**, 2026-08-09. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. Three refused in every arrangement tried: `bpy.ops.sequencer.reload()`, `bpy.ops.sequencer.refresh_all()` and `bpy.ops.node.shader_script_update()`. What a render re-reads is now answered by observation rather than left open: for an image, an image sequence and an external OSL script the file on disk is read at the next render; for a linked blend-file, a movie clip, an Alembic archive, a Video Sequencer strip and a packed image it is not. The one thing that plainly still needs a window is the warning banner in the Text editor; the flag behind it, `Text.is_modified`, is readable from a script and does become `True`. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.

### Where I looked, for question 11

Searched the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` (case-insensitive) for: `reload` (then opened every manual file that matched), `refresh`, `Refresh All`, `refresh_all`, `Auto Refresh`, `auto_refresh`, `revert`, `re-read`, `reread`, `read again`, `modified externally`, `changed externally`, `external change`, `has been modified`, `modified outside`, `detect changes`, `up to date`, `up-to-date`, `latest version`, `modification time`, `timestamp`, `mtime`, `file watcher`, `watch the file`, `file lock`, `locks the file`, `file handle`, `keeps the file open`, `held open`, `while Blender is running`, `while it is open`, `already open in Blender`, `kept in memory`, `held in memory`, `memory copy`, `on demand`, `lazily`, `lazy load`, `only loaded when`, `cache`, `persistent data`, `use_persistent_data`, `external files`.

Pages opened: the manual's Top Bar page (File and External Data sections in full), Image editor Image Settings and Editing pages, Text Editor page, Link & Append page, Packed Data page, Video Sequencer navigating / preview header / Cache sidebar pages, Video Editing montage editing and image-strip pages, Asset Libraries introduction, Cycles Performance render settings, Preferences ▸ System (Memory & Limits section in full), Command Line Arguments and Rendering From The Command Line. In the API documentation: `bpy.types.Image` (in full at the head and around `reload`), `bpy.types.Library`, `bpy.types.ImageUser`, `bpy.types.RenderSettings` (around `use_persistent_data`), `bpy.ops.image`, `bpy.ops.text`, `bpy.ops.sequencer`, `bpy.ops.clip`, `bpy.ops.cachefile`, `bpy.ops.node` (around `shader_script_update`), `bpy.ops.wm` (around `lib_reload`, `lib_relocate`, `revert_mainfile`), `bpy.ops.file`, `bpy.utils`, `bpy.path`.

Ruled out and why: Preferences ▸ System "Memory & Limits" was read in full because it looked likely; the only related item is "Texture Time Out — Time since last access of a GL texture in seconds, after which it is freed. Set this to 0 to keep textures allocated." (https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, date read 2026-07-30), which is about graphics-card textures being freed for inactivity, not about files changing, so I did not use it. The Video Sequencer Cache page describes caching of frames in memory ("Caches raw image data immediately after it is read from disk", https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/sidebar/cache.html), which is preview caching inside the Sequencer and does not speak to a render, so I did not build an answer on it. The 5.1 release notes were read: Core and Pipeline & I/O in full (https://developer.blender.org/docs/release_notes/5.1/core/ and .../pipeline_io/), and all pages searched for `reload`, `missing`, `relocate` and `filepath`; nothing in them concerns reloading changed external files. The Cycles files were searched for `reload`, `missing`, `filepath` and `cache`; the only `reload` hits are the add-on re-importing its own Python modules in `cycles_5.1/__init__.py`, lines 18–32, which is about the add-on's code, not about scene data, so I ruled it out. The supplied Cycles tree has no scene-synchronisation or image-manager code (only `kernel` and `util`), so it cannot speak to this question.

---

## Things a person with Blender 5.1 open could settle by hand in a few minutes

**All twelve items that stood here have been run**, on 2026-08-09, in Blender 5.1.2 on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, when a file changes on disk, or when a render begins. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open, and not a matter for the machine.** Two things a fresh look at the sources has to settle before another test is worth writing: what `RELOAD_EDITED` does to `bpy.data.file_path_foreach`, whose real keyword this round did not have; and how `bpy.ops.wm.lib_reload()` is meant to be called, which failed with `Error: Not a library` both ways it was tried.

---

9 August 2026 — Opus 5 (Claude Code) — Reading a file, Blender 5.1

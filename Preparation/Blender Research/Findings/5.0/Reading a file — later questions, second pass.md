# Reading a file — later questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual, Blender 5.0 Python API documentation, Blender 5.0 release notes, and Cycles' own files, all as supplied in `Blender Sources/` (snapshot taken 2026-07-30). That snapshot date is my date read for everything cited below. Questions answered: 10 and 11 only.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One disclosure, so the line above is exact: I ran a directory listing of `Preparation/Blender Research/Findings/5.0/` to confirm the folder I was told to save into exists. I saw a list of file names and opened none of them, including the findings document I was told not to open. I opened no other file outside the question sheet and `Blender Sources/`.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**LABEL: mixed.** *What location is written back* — **documented**. *What makes a file a possible match* and *which one is chosen when several match* — **not in the sources**.

### What the sources do say

There is exactly one such search in the sources: the **Find Missing Files** command. The manual describes it in full, and this is the whole of what it says:

> "Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

> "Note
>
> Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html (in the File ‣ External Data section). Read 2026-07-30.

The same command in the Python API documentation:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, check_existing=False, filter_blender=False, filter_backup=False, filter_image=False, filter_movie=False, filter_python=False, filter_font=False, filter_sound=False, filter_text=False, filter_archive=False, filter_btx=False, filter_alembic=False, filter_usd=False, filter_obj=False, filter_volume=False, filter_folder=False, filter_blenlib=False, filemode=9, display_type='DEFAULT', sort_method='')
>
> Try to find missing external files"

> "find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)"

> "directory (string, (optional, never None)) – Directory, Directory of the file"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.file.html. Read 2026-07-30.

The companion command that only reports, without changing anything:

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.file.html. Read 2026-07-30. The manual's wording for the same command:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html. Read 2026-07-30.

### Taking the three parts of the question in turn

**What location is written back — documented.** An absolute path. The manual says it outright: "Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative." So the recorded location becomes a full path, and turning it back into a path relative to the blend-file is a separate, manual step.

**What makes a file a possible match — not in the sources.** The manual states only where the search looks (the chosen directory, and every directory inside it, however deep) and that "Every missing file found in the search will be recovered." It never states the test a file has to pass to count as "found". Nothing in the manual, the API documentation, the release notes or Cycles' files says the test is the file name, or the file name plus extension, or the tail of the recorded path, or size, or anything else. I looked hard for such a statement and there is none — see the record of looking below.

One thing that is *not* an answer to this part, in case it looks like one: the `find_all` option. Its description, "Find all files in the search path (not just missing)", is about which of the blend-file's recorded locations get considered — by default only the ones that are broken, and with the option on, all of them. It says nothing about what makes a candidate file on disk a match. That reading of `find_all` is mine, from the words "not just missing", and is **inferred**, not documented.

The file-type filter arguments (`filter_image`, `filter_movie`, and the rest) are the standard File Browser display filters shared by every operator that opens a File Browser — they appear identically on unrelated operators on the same page. Reading them as the match criterion would be over-reading, and I do not.

**Which one is chosen when several match — not in the sources.** The manual does not raise the possibility of more than one match at all. There is no statement of a precedence rule (shallowest first, alphabetical, first encountered, newest), no statement that the user is asked, and no statement that ambiguity is reported. The word "recursively" tells us the search descends into subdirectories, so several files with the same name can plainly fall inside one search; the sources simply do not say what happens then.

### Where I looked, and what I ruled out

Searches were run over the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` at once, case-insensitively unless noted.

Terms searched: `find_missing_files`, `Find Missing Files`, `find_all`, `Find All`, `report_missing_files`, `Report Missing Files`, `missing`, `missing file`, `missing external`, `broken link`, `not found`, `recovered`, `candidate`, `same name`, `matching name`, `first match`, `ambiguous match`, `recursively`, `subdirector`, `search path`, `search directory`, `search folder`, `locate the file`, `relink`, `repath`, `re-path`, `remap` (and `remap` next to `path`).

Pages opened and read in full or in the relevant part:
- Topbar (the File menu, including all of External Data) — https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html. This is the only page in the whole manual that mentions Find Missing Files.
- `bpy.ops.file` operator list, whole page — https://docs.blender.org/api/5.0/bpy.ops.file.html. I read the full list of the 40-odd `bpy.ops.file.*` operators to be sure no second search-type operator exists. The path-related ones are `find_missing_files`, `report_missing_files`, `make_paths_absolute`, `make_paths_relative`, and the pack/unpack family; none of the others touch recorded locations.
- File Paths — https://docs.blender.org/manual/en/5.0/files/file_paths.html. Ruled out: it is entirely about path template expressions such as `{blend_name}`, and says nothing about searching for missing files.
- Packed Data — https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html. Ruled out for this question: it covers packing and unpacking, not searching.
- Linking and Appending, including Reload Library and Relocate Library — https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html. Relocate is a manual replacement of one library file by another that the user picks; it is not a search, so it is not an answer to this question.
- Opening and Saving (the Recent Files "Items Not Found" entry) — https://docs.blender.org/manual/en/5.0/files/blend/open_save.html. Ruled out: that entry removes dead items from the recent-files list; it has nothing to do with references inside a blend-file.
- File Browser editor page — https://docs.blender.org/manual/en/5.0/editors/file_browser.html. Ruled out: the word "missing" does not appear on it.
- `bpy.ops.wm` (checked `lib_reload` and `lib_relocate`) — https://docs.blender.org/api/5.0/bpy.ops.wm.html. `lib_relocate` is "Relocate the given library to one or several others"; again a chosen replacement, not a search.
- Release notes for 5.0: I searched every subject file for `missing`, `reload`, `relocate`, `library`, `path`. The only hits were unrelated bug-fix entries and the new path template variables. Nothing about Find Missing Files.
- Cycles' own files: searched for `missing` and `find`. The only hits are `find_node_input` and `find_existing_device_entry`, both unrelated. Cycles carries no missing-file search, so it is ruled out for this question.
- Pictures — **documented**: `_images/interface_window-system_topbar_menus.png`, on https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30. Visible in it: the top bar strip itself — the Blender icon and the words File, Edit, Render, Window, Help. No menu contents are visible, so it settles nothing here. *(Citation check, 2026-08-09: labelled **documented** and put into the Method's form for a picture — which picture, the address of the page it sits on, and what is visible in it. I opened the same file and it shows exactly that. The three auditors in this round each labelled picture-findings differently; this is the Method's form.)* I also searched the whole `_images` folder for file names containing "missing", "find" or "external_data"; there is no screenshot of the Find Missing Files File Browser or of its options.

**Not covered.** I cannot tell you the matching test, the tie-break when several files match, or whether Blender reports an ambiguity — the sources are silent on all three. I also cannot tell you whether "recovered" means only the recorded location is rewritten or the data is re-read as well; the manual's note that "Recovered files might need to be reloaded" points towards the former but does not state it, so I do not claim it. Nor do the sources say whether the search covers every kind of recorded location (images, videos, sounds, fonts, cache files, linked blend-files) or only some.

### Settled by hand test — observed, 2026-08-09

Eight tests were built for this question and run in **Blender 5.0.1** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 1 to 8.

**What was done, and to what.** A blend-file holding one image texture whose file was then deleted, reopened, and repaired by calling `bpy.ops.file.find_missing_files(directory=<folder>)` from a script. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, and no window was open at any point.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on all eight tests.**

**The command runs with no window at all.** Both machines printed `HT| s_same | operator returned = "{'FINISHED'}"`, and the recorded location changed.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension — `tex.jpg` for a missing `tex.png` — was not, and neither was a file of a different name holding the same picture. Both printed the same line as a folder holding nothing at all: `Warning: Could not find 'tex.png' in '<folder>'`.

**Letter case has to match, on both machines.** With only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it — `HT| s_upper | filepath after = '//orig/tex.png'` — while the control run with `tex.png` on disk recovered it on both. The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run. Wherever the candidates differed in size, the recorded location ended up pointing at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them: `HT| sA | filepath after = '//sA/zzz/tex.png'` against `HT| sB | filepath after = '//sB/aaa/tex.png'`, the larger file winning both times. A file three folders down beat one at the top of the search folder on a single byte, and a 3981-byte file beat a 260-byte one from either position. Where two candidates were exactly the same size, `aaa` was taken rather than `zzz`.

> **Unsettled — the two passes for this version disagree on the byte figures.** The first pass of 5.0 reports the same hand-test entries as "the 188-byte file winning both times over the 187-byte one" and "a 3908-byte file beat a 187-byte one"; this pass reports 3981 and 260. The citation check's own repeat, on both machines, has Blender 5.0 writing 187- and 188-byte PNGs for picture content that 5.1 and 5.2 write at 260 and 261 bytes — that is, the figures in this pass are the shape of the 5.1/5.2 ones. But the repeat used its own pictures, not these, so it settles nothing on its own. Neither pass has been preferred; the tie-break itself (largest file wins, `aaa` on an exact tie) repeated in full and is not in doubt. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **The byte sizes measured again.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 28 of the hand-test list ran the tie-break arrangement once more and recorded the exact size of every candidate. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), this version wrote **188 bytes** for the red and blue 8 by 8 pictures, **187** for the green and white, **201** for the 16 by 16 one and **3908** for the 64 by 64 one. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), this version wrote the same four figures. Across the four versions: 4.5.12 LTS and 5.0.1 both wrote 188, 187, 201 and 3908; 5.1.2 and 5.2.0 LTS both wrote 261, 260, 274 and 3981. The choice came out the same at both sets of sizes and on both machines — the larger candidate won from either folder, and where the two candidates were byte-for-byte the same size `aaa` was taken. That bears on the disagreement between the two passes for this version: 188, 187 and 3908 is the shape of the figures the first pass carries, and the second pass's 3981 and 260 are what Blender 5.1.2 and 5.2.0 LTS wrote in this run. This is a measurement of this run's own pictures, so it says nothing about what either earlier run put on disk, and neither pass has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Nothing is said about there having been more than one.** No line about ambiguity appeared in any arrangement. The control is what makes that worth recording: with nothing to find, the same terminal carried `Warning: Could not find 'tex.png' in '<folder>'`, so this is a silence about ambiguity and not an unwatched channel.

**The location written back began `//`, not at the system root.** On both machines the recovered location came back as `//s_same/deep/tex.png`, with `HT| s_same | starts with // = True`. That is not what the manual sentence quoted above describes. What is recorded here is what appeared; nothing in the test separates the command's own behaviour from a preference that might turn the path relative afterwards.

**The recovery rewrites the location without loading the data.** Straight after a successful recovery, `HT| s_same | has_data = False`. Reading the pixels afterwards returned the new file's colour, so the file is read when it is next wanted rather than at the moment of the repair.

**How deep it searches.** A file eight folders below the chosen one was found, on both machines.

**Which kinds of recorded location it reaches: all six that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic archive had all six rewritten by a single run, on both machines — the linked blend-file included, although the manual gives libraries a separate repair tool of their own.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the sound, the font, the movie clip, the linked blend-file and the Alembic archive were made. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again, with the making of all six written out.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 26 of the hand-test list rebuilt this test with every step named: an 8 by 8 red PNG written by Blender itself; a tenth of a second of silence written by Python's own `wave` module at one channel, two bytes a sample, 8000 samples a second; a font copied from whichever the machine has; a single-frame image sequence `clip_0001.png`; a blend-file holding one collection named `LibCol` with a default cube in it, linked and instanced; and an archive exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` and everything else left at the operator's own defaults. All six files were then moved into one search folder and the command was called once. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), the font taken was `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`, and all six recorded locations were rewritten to `//found/…`, every one of them beginning `//`. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), the font taken was `/System/Library/Fonts/Supplemental/Arial.ttf`, and the same six locations were rewritten, every one beginning `//`. The control was run on each machine as well: with the search folder empty, none of the six moved and the terminal carried six separate `Warning: Could not find '<name>' in '<folder>'` lines, one per kind. **This matches the result recorded above.** *(Note added by the hand tests, second batch, 2026-08-09.)*

**`find_all` rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a working reference moved too, from `//keep/good.png` to `//search/good.png`.

**A packed resource's recorded location is rewritten as well, and it stays packed.** `HT| after | p.png = ('//found/p.png', True)`.

**Whether a recovery can be undone is not settled.** With no window, `bpy.ops.ed.undo()` refused — `Operator bpy.ops.ed.undo.poll() failed, context is incorrect` — even after `bpy.ops.ed.undo_push()` had returned `{'FINISHED'}`, and again with a 3D Viewport context built by hand. Neither machine can answer the version of that test which needs a window: the rented Linux machine has no display server and the Creator ran nothing this round.

**Outside the interface.** **Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved. Everything above about matching, the tie-break and the form of the written-back location was established that way, so all of it holds outside the interface. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. So the adjacent fact about `bpy.path.resolve_ncase` does not describe this command. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**The question is ambiguous and I flag it.** "A file Blender has already read" can mean (a) an external file the open blend-file points at — an image, a video, a sound, a linked blend-file, a cache file — or (b) the `.blend` file itself. I answer reading (a), which I take to be the one meant, and cover (b) briefly at the end.

**LABEL: mixed.** *What you must do to pick up a changed file* — **documented**, for each kind of file. *Whether Blender notices the change by itself, and whether starting a render re-reads anything* — **not in the sources**, except for one kind of file (text) where it is documented that Blender does notice.

### What happens: the sources never describe an automatic re-read

I could not find any statement, anywhere in the 5.0 manual, API documentation or release notes, that Blender watches files on disk or re-reads a changed one on its own. What I found instead is a long series of explicit commands whose whole purpose is to re-read a file, and one flat statement that re-reading does not happen by itself:

> "Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked.
> Note that it won't be automatically reloaded from the drive.
> (differs) or (identical) show difference status between the packed version
> and the one on-drive."

Source: manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html. Read 2026-07-30. That is said about one particular operation (unpacking), not as a general rule, so I quote it for what it is.

The manual's own instructions for the everyday case — you edited a texture in another program — are a straight instruction to reload by hand:

> "Using an External Image Editor
>
> If you use an external program to edit your UV texture, you must:
>
> Run that paint program (GIMP, Krita, etc.).
>
> Load the image or create a new one.
>
> Change the image.
>
> And re-save it within that program.
>
> Back in Blender, you reload the image in the Image Editor."

Source: manual, https://docs.blender.org/manual/en/5.0/sculpt_paint/texture_paint/introduction.html. Read 2026-07-30.

And the Image panel's Reload button:

> "Reload
>
> Reloads the file. Useful when it has been reworked in an external application."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html. Read 2026-07-30.

That data sits in memory once read is documented in passing:

> "has_data
>
> True if the image data is loaded into memory"

> "buffers_free()
>
> Free the image buffers from memory"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Image.html. Read 2026-07-30.

> "Caches raw image data immediately after it is read from disk."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/sidebar/cache.html (Cache ‣ Raw). Read 2026-07-30. This is the Video Sequencer's own cache, and applies there.

### The one kind of file where Blender does notice the change

Text data-blocks. Here a change made by another program is detected and surfaced:

> "is_modified
>
> Text file on disk is different than the one in memory"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Text.html. Read 2026-07-30.

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

Source: manual, https://docs.blender.org/manual/en/5.0/editors/text_editor.html. Read 2026-07-30. The matching operator:

> "bpy.ops.text.resolve_conflict(*, resolution='IGNORE')
>
> When external text is out of sync, resolve the conflict"
>
> "resolution (enum in ['IGNORE', 'RELOAD', 'SAVE', 'MAKE_INTERNAL'], (optional)) – Resolution, How to solve conflict due to differences in internal and external text"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.text.html. Read 2026-07-30.

Note what this means for the general case: text is singled out as having a conflict-detection mechanism. No equivalent property or command exists for images, videos, sounds or libraries anywhere I searched.

### What has to happen: the documented re-read commands, by kind of file

**Images** —
> "Reload the image from the file on drive." (Menu: Image ‣ Reload, shortcut Alt-R) — manual, https://docs.blender.org/manual/en/5.0/editors/image/editing.html
> "bpy.ops.image.reload()  Reload current image from disk" — https://docs.blender.org/api/5.0/bpy.ops.image.html
> "reload()  Reload the image from its source path" — https://docs.blender.org/api/5.0/bpy.types.Image.html

**Movie clips** —
> "bpy.ops.clip.reload()  Reload clip" — https://docs.blender.org/api/5.0/bpy.ops.clip.html

**Video Sequencer strips** —
> "Reload Strips Alt-R
>
> Reloads the strips from their external saved location.
>
> Reload Strips and Adjust Length Shift-Alt-R
>
> Reloads the strips from their external saved location and re-adjusts the strip duration."
> — manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html
> "Reloads external files and refreshes the current frame preview." (Refresh All, Ctrl-E) — manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/navigating.html
> "bpy.ops.sequencer.reload(*, adjust_length=False)  Reload strips in the sequencer" — https://docs.blender.org/api/5.0/bpy.ops.sequencer.html

**Cache files (Alembic, USD)** —
> "bpy.ops.cachefile.reload()  Update objects paths list with new data from the archive" — https://docs.blender.org/api/5.0/bpy.ops.cachefile.html

**Text** —
> "bpy.ops.text.reload()  Reload active text data-block from its file" — https://docs.blender.org/api/5.0/bpy.ops.text.html

**Linked blend-files (libraries)** —
> "When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file."
> — manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html
> "bpy.ops.wm.lib_reload(...)  Reload the given library" — https://docs.blender.org/api/5.0/bpy.ops.wm.html

**Everything at once** — the manual gives a blanket route, in the note attached to Find Missing Files:
> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."
> — manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html

Reopening the current file has its own command:
> "Revert
>
> Reopens the current file to its last saved version."
> — manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html
> "bpy.ops.wm.revert_mainfile(*, use_scripts=False)  Reload the saved file" — https://docs.blender.org/api/5.0/bpy.ops.wm.html

All read 2026-07-30.

### Two cases where the changed file is deliberately ignored

**Packed resources.** Once external files are packed and the blend-file saved, changes on disk stop mattering:
> "Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them."
Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html. Read 2026-07-30. (The phrase "will no longer be reflected" suggests that when files are *not* packed, changes are reflected once re-read; that reading is mine, and is **inferred**, not documented.)

**Packed linked data**, new in 5.0:
> "Packed IDs remain considered as linked data (i.e. they cannot be edited), but they are stored in the current blendfile. This means that they:
>
> * Are not lost in case the library data becomes unavailable.
> * Are not changed in case the library data is updated."
Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/. Read 2026-07-30.

### One case where something *is* redone automatically, on opening

> "Overrides are automatically resynced if needed on blend-files opening. However,
> it may be needed to resynced them manually sometimes"
Source: manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/library_overrides.html. Read 2026-07-30. This is about library overrides catching up with a changed library structure, and it is tied to opening the file, not to rendering.

### Does starting a render re-read anything?

**Not in the sources.** No page states that beginning a render re-reads external files, and none states that it does not. The one render setting that touches on data being kept between renders is:

> "Persistent Data
>
> Keep render data in memory after rendering for faster re-renders and animation renders
> at the cost of extra memory usage while performing other tasks in Blender."
Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html. Read 2026-07-30. In the API: "use_persistent_data — Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage. Type: boolean, default False" (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30). Whether this setting also prevents a changed file from being picked up is **not stated**; I do not claim it.

### Reading (b): the `.blend` file itself replaced on disk while open

The sources describe no detection of this either. The commands that read the file again are Revert ("Reopens the current file to its last saved version") and simply opening it again ("bpy.ops.wm.open_mainfile(...) Open a Blender file", https://docs.blender.org/api/5.0/bpy.ops.wm.html). Rendering from the command line is documented as being handed a blend-file path each time:
> "-b file.blend ... Path to the blend-file to render."
> "-a ... Render the whole animation using all the settings saved in the blend-file."
Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html. Read 2026-07-30. From that it follows that a command-line render started after the file changed is given the file as it stands then — the step being that a freshly started Blender has nothing in memory to be stale. That step is mine, so this is **inferred**, not documented.

### Where I looked, and what I ruled out

Searches over the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0`, case-insensitively.

Terms searched: `reload`, `re-load`, `Reload`, `revert`, `refresh`, `Auto Refresh`, `auto_refresh`, `modified externally`, `externally modified`, `changed on disk`, `on disk`, `external application`, `external program`, `out of date`, `outdated`, `up to date`, `up-to-date`, `is_modified`, `is_dirty`, `is_in_memory`, `is_missing`, `has_data`, `buffers_free`, `cache`, `cache limit`, `memory cache limit`, `image cache`, `persistent data`, `use_persistent_data`, `file watcher`, `watches`, `monitors the`, `automatically detect`, `detect changes`, `resolve_conflict`, `open_mainfile`, `revert_mainfile`, `lib_reload`, `lib_relocate`, `pack`, `unpack`.

Pages opened and read:
- Image editor: Editing (Image menu) and Image Settings sidebar — https://docs.blender.org/manual/en/5.0/editors/image/editing.html and https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html
- Text Editor — https://docs.blender.org/manual/en/5.0/editors/text_editor.html
- Texture Paint introduction — https://docs.blender.org/manual/en/5.0/sculpt_paint/texture_paint/introduction.html
- Topbar / File menu, including External Data and Revert — https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html
- Packed Data — https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html
- Linking and Appending (Reload Library, Relocate Library) — https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html
- Library Overrides (Resyncing Overrides) — https://docs.blender.org/manual/en/5.0/files/linked_libraries/library_overrides.html
- Video Sequencer: Navigating (Refresh All), Preview header, Editing (Reload Strips), Cache sidebar — https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/navigating.html, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/header.html, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/sidebar/cache.html
- Cycles Performance settings (Persistent Data) — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html
- Rendering From The Command Line — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html
- API: `bpy.types.Image`, `bpy.types.Text`, `bpy.types.ID`, `bpy.types.RenderSettings`, `bpy.ops.image`, `bpy.ops.text`, `bpy.ops.clip`, `bpy.ops.cachefile`, `bpy.ops.sequencer`, `bpy.ops.wm`, `bpy.ops` (calling operators), `info_gotchas_file_paths_and_encoding`
- Release notes: every 5.0 subject file searched for `reload`, `missing`, `library`, `path`. Only relevant hit is the Packed Data section of https://developer.blender.org/docs/release_notes/5.0/core/; the two `reload` hits in `bugfixes.md` are unrelated fixes.

Ruled out and why:
- **Auto Refresh** (on image sequences and videos): "Always refresh image on frame changes" (https://docs.blender.org/api/5.0/bpy.ops.image.html), "Play the video in the Image Editor when the scene animation is playing" (image settings page), "Update the video texture in the 3D Viewport when moving through the timeline" (https://docs.blender.org/manual/en/5.0/render/shader_nodes/textures/image.html). This is about following the timeline to a different frame of a sequence, not about a file's contents changing on disk. Not an answer here.
- **Cycles' own files**: searched for `reload`; the only hits are in `cycles_5.0/__init__.py` lines 18–32, which reload the add-on's own Python modules during development. Nothing about re-reading scene files or textures. Cycles is ruled out for this question.
- **Recent Files ‣ Remove ‣ Items Not Found** (https://docs.blender.org/manual/en/5.0/files/blend/open_save.html): about the recent-files list, not about loaded data.
- **Reload Scripts** (https://docs.blender.org/manual/en/5.0/advanced/operators.html): reloads Blender's scripts, not scene files.
- **Pictures**: I found no screenshot in `_images` showing a reload prompt, an out-of-date warning, or the text-conflict banner. Searching the image file names for "reload", "missing" and "conflict" returned nothing.

**Not covered.** My answer does not settle what actually happens in memory at the moment a file changes underneath a running Blender — whether the old contents stay, whether the change is noticed later, or whether an already-open handle to a replaced file behaves differently from one to an overwritten file. Nor does it settle whether any of the reload commands are enough on their own for a subsequent render, or whether something further (saving and reopening the blend-file) is needed; the manual offers the save-and-reopen route as an alternative, not as a requirement. It also does not settle what happens to a file that is changed *while* a render is running.

### Settled by hand test — observed, 2026-08-09

Fifteen tests were built for this question and run in **Blender 5.0.1** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 9 to 23.

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

> **Run again with the set-up written out, and it came out differently.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 27 of the hand-test list names both missing steps. Two archives were exported, from a cube of size 2 and a cube of size 4, with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)`; the smaller was brought in with `bpy.ops.wm.alembic_import(filepath=…, as_background_job=False, always_add_cache_reader=True)`, which gives the object a Mesh Sequence Cache modifier and a Transform Cache constraint, both printed back from the file. **The importer's own defaults were run as an arm of their own**, on both machines and in all four versions. The object came back with `Cube modifiers = []` and `Cube constraints = []` — no cache reader at all — and measured -1 to 1 at every one of the six points, which is the reading above; but a freshly started Blender opening that file after the swap also measured -1 to 1, on both machines, and a fresh Blender cannot be showing a stale archive. In that same fresh process the replacement archive imported on its own measured -2 to 2, so the archive was fine and the arrangement was watching an object that nothing tied to it. With the cache reader attached, on the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): after the swap with nothing done, -1 to 1; `bpy.ops.cachefile.reload()` with no context `{'CANCELLED'}`, -1 to 1; the same command with an `edit_cachefile` context built by hand `{'FINISHED'}`, and the evaluated mesh then measured **-2 to 2**, dimensions `(4.0, 4.0, 4.0)`; it stayed at -2 to 2 after the recorded location was written back to itself and after the frame was stepped away and back. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same readings at all six points. The archives were 3115 bytes each on the Linux machine and 3115 bytes each on the Mac — the same size as one another, which is why the mesh had to be measured. The controls held on both machines: a freshly started Blender opening the same blend-file after the swap measured -2 to 2, and the replacement archive imported on its own measured -2 to 2, so it really did hold a cube of twice the size. The result above is left exactly as it stands. Both runs stand and neither has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Blender does notice a changed external text file, and the conflict can be resolved with no window.** `Text.is_modified` was `False` while the two copies agreed and `True` once the file on disk differed. The text Blender held did not change until a reload was run. `bpy.ops.text.reload()` refused with no Text editor and returned `{'FINISHED'}` with one built by hand; so did `bpy.ops.text.resolve_conflict(resolution='RELOAD')`, which brought the new text in.

**A packed image ignores the file on disk.** With the image packed, the render went on showing the old picture after the file was replaced, and `Image.reload()` — which raised no error — did not bring the disk file in either. The same swap with the image left unpacked did reach the render, which is what shows the swap bit.

> **Unsettled — the citation check's repeat disagreed.** Repeating this on both machines and in all four versions, with the image packed and the blend-file saved and reopened: render one red `[0.659, 0.012, 0.02, 1.0]`, render two after the swap still red, and then, after `Image.reload()`, render three **blue** `[0.004, 0.004, 0.506, 1.0]` — with the image still packed. The control with the image left unpacked went red then blue on both runs. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 24 of the hand-test list ran three arms: the image left unpacked, the image packed with the blend-file saved but *not* reopened, and the image packed with the blend-file saved *and* reopened — the one difference the two earlier records name. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): the unpacked control rendered `[0.996, 0.0, 0.0, 1.0]`, then `[0.0, 0.0, 0.996, 1.0]` after the swap, so the swap bit; packed and not reopened rendered red, red, then **`[0.0, 0.0, 0.996, 1.0]`** after `Image.reload()`, with `packed_file is not None` still `True`; packed and reopened gave the same three renders and stayed packed. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three arms and the same nine renders. The test PNG was 188 bytes in this version. `Image.reload()` returned `None` and raised nothing every time. **This third run matches the citation check's and not the result recorded above.** Whether the blend-file was reopened after packing made no difference on either machine, so that is not what separates the two earlier runs, and this run offers nothing in its place. All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Cycles' texture cache does not exist in this version.** `RenderSettings.use_texture_cache` and `RenderSettings.use_auto_generate_texture_cache` were both absent, and `generate_texture_cache` and `clear_texture_cache` were both absent from the real list of `bpy.ops.render` operators.

**A changed external OSL script reaches the next render with nothing done.** `ShaderNodeScript.use_auto_update` was present with a default of `False`. The first render with the script emitting red was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue and the very next render was `[0.0, 0.0, 0.996, 1.0]`. `bpy.ops.node.shader_script_update()` refused every time it was called, with and without a Shader node editor built by hand, so no update command ever succeeded — and the change reached the render anyway.

> **Unsettled — the citation check's repeat disagreed.** Repeating this — a Script node in External mode emitting red, the `.osl` on disk rewritten to emit blue, the very next render with nothing else done — gave: Creator's Mac 4.5 blue, 5.0 blue, 5.1 red, 5.2 red; rented Linux machine 4.5 red, 5.0 red, 5.1 red, 5.2 blue. The two machines did not agree with each other. The other two parts of this entry did repeat exactly: `use_auto_update` present with a default of `False`, and `bpy.ops.node.shader_script_update()` refusing in every arrangement. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time, with the compiled file watched and the clock measured.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 25 of the hand-test list. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): render 1 with the red `.osl` was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue; render 2, with nothing else done, was **`[0.0, 0.0, 0.996, 1.0]`**; and the compiled `t.oso` sitting beside the `.osl` went from 485 bytes to 496 during that very render, its modification time moving on with it. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three renders, and `t.oso` went from 775 bytes to 848 during render 2. On both machines `bpy.ops.node.shader_script_update()` raised `Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`, with and without a Shader node editor built by hand, so no update command ever succeeded. **The clock was measured rather than assumed.** On the Linux machine two writes a hundredth of a second apart were given recorded times 10,999,745 nanoseconds apart, and the `.osl`'s own recorded time moved from `1786297266857884778` to `1786297269670819576` at the rewrite. On the Mac the same two writes were 16,358,938 nanoseconds apart, and the `.osl`'s recorded time moved from `1786297384636876828` to `1786297387696806745`. Both machines record a modification time to the nanosecond, and nothing in this run was hidden inside one tick of it. Two further controls held on both machines: a genuinely separate Blender process opening the saved blend-file rendered blue, and rendered blue again after every `.oso` had been deleted, so blue was reachable. **This third run matches the result recorded above and not the citation check's.** All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**`bpy.data.file_path_foreach` exists in this version.** `HT| bpy.data.file_path_foreach present = True`. Every call made to it errored, because the keyword this round used was wrong: `TypeError("'flag' is an invalid keyword argument for file_path_foreach()")`. The error is recorded rather than repaired: this settles that the function is there and does not settle what `RELOAD_EDITED` does.

**A file replaced while a render is already running takes effect from a later frame onward.** A twelve-frame animation had its image replaced part-way through: the frames up to a point held the old picture and every frame after it held the new one. No frame came out part old and part new. Which frame the change lands on moves from run to run with the timing.

**Revert takes the blend-file as it stands on disk.** With `scene.blend` opened, then replaced on disk by a different file, Blender still held the old contents until `bpy.ops.wm.revert_mainfile()` was run; it returned `{'FINISHED'}` with no window open and afterwards Blender held the replaced file's object, `['CUBE_B']`. A control Revert with nothing replaced gave `['CUBE_A']` back.

**Outside the interface.** **Observed**, 2026-08-09. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. Three refused in every arrangement tried: `bpy.ops.sequencer.reload()`, `bpy.ops.sequencer.refresh_all()` and `bpy.ops.node.shader_script_update()`. What a render re-reads is now answered by observation rather than left open: for an image, an image sequence and an external OSL script the file on disk is read at the next render; for a linked blend-file, a movie clip, an Alembic archive, a Video Sequencer strip and a packed image it is not. The one thing that plainly still needs a window is the warning banner in the Text editor; the flag behind it, `Text.is_modified`, is readable from a script and does become `True`. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

**All sixteen items that stood here have been run**, on 2026-08-09, in Blender 5.0.1 on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, when a file changes on disk, or when a render begins. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open, and not a matter for the machine.** Two things a fresh look at the sources has to settle before another test is worth writing: what `RELOAD_EDITED` does to `bpy.data.file_path_foreach`, whose real keyword this round did not have; and how `bpy.ops.wm.lib_reload()` is meant to be called, which failed with `Error: Not a library` both ways it was tried.

---

2026-08-09 — Opus 5 (Claude Code) — Reading a file, Blender 5.0

# Reading a file — later questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (`manual_4.5`), Blender 4.5 Python API documentation (`api_4.5`), Blender 4.5 release notes (`release_notes_4.5`), and Cycles' own files (`cycles_4.5`) — all as supplied inside `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the "date read" on every citation below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One disclosure, so the line above is exact: to confirm the folder this file had to be written into existed, I ran a directory listing of `Preparation/Blender Research/Findings/4.5/` and saw the names of files already in it. I opened none of them and read no content from any of them. Nothing else outside `Blender Sources/` and the question sheet was listed, searched, or opened.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Reading taken.** The question is ambiguous about which "search" it means. I answer it about Blender's own **Find Missing Files** command, which is the one thing in the sources that searches for missing references. The other reading would be the Python helper `bpy_extras.image_utils.load_image()`, which importers use to hunt for an image file; I cover what the sources say about that at the end of this answer, because it is the only place the sources describe matching rules at all.

**Label: split.**
- What location is written back — **documented**.
- What makes a file a possible match — **not in the sources**.
- Which one is chosen when several match — **not in the sources**.

### What the sources do say

The command exists and searches recursively from a directory you pick, and every missing file it finds is fixed:

> "Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual. https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

So the location written back is an **absolute path**. That is stated outright.

The same page describes the companion command that only reports:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

Source: manual. https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

The Python side gives the command's signature. One option bears on the scope of the search, though not on how a file is matched:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, check_existing=False, filter_blender=False, filter_backup=False, filter_image=False, filter_movie=False, … sort_method='')
>
> Try to find missing external files
>
> Parameters:
>
> find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)
>
> directory (string, (optional, never None)) – Directory, Directory of the file"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30.

There is also a companion:

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30.

### What the sources do not say

Nowhere do the sources state **what makes a candidate file a match** — not filename, not extension, not size, not letter case, not anything else. The manual says only "Every missing file found in the search", which names no rule. `find_all` distinguishes "missing" from "not missing" but says nothing about how a file on disk is paired with a broken reference.

Nowhere do the sources state **which file wins when two or more could match** — no mention of shallowest folder, alphabetical order, first found, newest, or a warning to the user. There is no documented tie-break at all, and no documented way to see which candidate was picked.

### Where I looked, to be able to say that

- Searched the whole of `manual_4.5` and `api_4.5` for: `find missing files`, `find_missing_files`, `report_missing_files`, `missing`, `broken link`, `relink`, `re-link`, `recovered`, `recovery`, `search path`, `recursively`, `same name`, `file name`, `basename`, `first match`, `ambiguous`, `duplicate name`, `multiple matches`, `more than one`, `candidate`, `matching file`, `matches the`, `priority`, `shallowest`, `deepest`, `alphabetical`.
- In the whole 4.5 manual, only one page contains the phrase "missing files" at all: the topbar page quoted above. The other manual pages containing the word "missing" are unrelated (add-on tutorial, FBX/glTF import notes, EEVEE and Cycles node limitations, NLA, video sequencer sidebar, startup troubleshooting, data transfer, and so on) and none discusses finding a missing reference.
- Read in full the two manual pages that could plausibly hold the rule: the External Data section of https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html and https://docs.blender.org/manual/en/4.5/files/file_paths.html (which turns out to be entirely about path templates such as `{blend_name}`, not about finding files).
- Read the linked-library page https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, which covers **Relocate** — a manual, user-picks-the-file repair, not a search. It says a relocate "allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed)". No searching, so no match rule.
- Searched `release_notes_4.5` for `missing`, `reload`, `relocate`. The only hits are bug-fix list entries and a core note about relocating linked IDs; none states a matching rule.
- Searched Cycles' own Python files (`cycles_4.5/*.py`) for `missing`, `reload`, `filepath`. Nothing in them searches for a missing file. The `filepath` hits sit in Cycles' denoising operator; no file and place is recorded for that here, so it stands as a record of the search and not as a finding of its own.

### The other reading of the question

If "a search for a missing reference" means the importer helper rather than the user command, the sources do describe its options, though still not a tie-break:

> "bpy_extras.image_utils.load_image(imagepath, dirname='', place_holder=False, recursive=False, ncase_cmp=True, convert_callback=None, verbose=False, relpath=None, check_existing=False, force_reload=False)
>
> Return an image from the file path with options to search multiple paths
> and return a placeholder if its not found.
>
> Parameters:
>
> filepath (str) – The image filename
> If a path precedes it, this will be searched as well.
>
> dirname (str) – is the directory where the image may be located - any file at
> the end will be ignored.
>
> place_holder (bool) – if True a new place holder image will be created.
> this is useful so later you can relink the image to its original data.
>
> recursive (bool) – If True, directories will be recursively searched.
> Be careful with this if you have files in your root directory because
> it may take a long time.
>
> ncase_cmp (bool) – on non windows systems, find the correct case for the file.
>
> … relpath (str | None) – If not None, make the file relative to this path."

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy_extras.image_utils.html — read 2026-07-30.

**Documented**, from the words above: the helper takes an image filename, can be given a directory to look in, can search directories inside it, carries an option described as "on non windows systems, find the correct case for the file", and carries a `relpath` option described as making the file relative to a given path.

**Inferred**, and the step is mine: that the filename is therefore what a candidate file is matched on, and that letter case is treated loosely on non-Windows systems. The words say what each option is for; they set out no matching rule, so a reader has to supply one.

It still does not say which file is chosen when several match. And this is a helper for scripts and importers, not the Find Missing Files command; nothing in the sources says the two share behaviour.

### Settled by hand test — observed, 2026-08-09

Eight tests were built for this question and run in **Blender 4.5.12 LTS** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 1 to 8.

**What was done, and to what.** A blend-file holding one image texture whose file was then deleted, reopened, and repaired by calling `bpy.ops.file.find_missing_files(directory=<folder>)` from a script. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, and no window was open at any point.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on all eight tests.**

**The command runs with no window at all.** Both machines printed `HT| s_same | operator returned = "{'FINISHED'}"`, and the recorded location changed.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension — `tex.jpg` for a missing `tex.png` — was not, and neither was a file of a different name holding the same picture. Both printed the same line as a folder holding nothing at all: `Warning: Could not find 'tex.png' in '<folder>'`.

**Letter case has to match, on both machines.** With only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it — `HT| s_upper | filepath after = '//orig/tex.png'` — while the control run with `tex.png` on disk recovered it on both. The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run. Wherever the candidates differed in size, the recorded location ended up pointing at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them: `HT| sA | filepath after = '//sA/zzz/tex.png'` against `HT| sB | filepath after = '//sB/aaa/tex.png'`, the 188-byte file winning both times over the 187-byte one. A file three folders down beat one at the top of the search folder on a single byte, and a 3908-byte file beat a 187-byte one from either position. Where two candidates were exactly the same size, `aaa` was taken rather than `zzz`.

> **The byte sizes measured again.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 28 of the hand-test list ran the tie-break arrangement once more and recorded the exact size of every candidate. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), this version wrote **188 bytes** for the red and blue 8 by 8 pictures, **187** for the green and white, **201** for the 16 by 16 one and **3908** for the 64 by 64 one. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), this version wrote the same four figures. Across the four versions: 4.5.12 LTS and 5.0.1 both wrote 188, 187, 201 and 3908; 5.1.2 and 5.2.0 LTS both wrote 261, 260, 274 and 3981. The choice came out the same at both sets of sizes and on both machines — the larger candidate won from either folder, and where the two candidates were byte-for-byte the same size `aaa` was taken. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Nothing is said about there having been more than one.** No line about ambiguity appeared in any arrangement. The control is what makes that worth recording: with nothing to find, the same terminal carried `Warning: Could not find 'tex.png' in '<folder>'`, so this is a silence about ambiguity and not an unwatched channel.

**The location written back began `//`, not at the system root.** On both machines the recovered location came back as `//s_same/deep/tex.png`, with `HT| s_same | starts with // = True`. That is not what the manual sentence quoted above describes. What is recorded here is what appeared; nothing in the test separates the command's own behaviour from a preference that might turn the path relative afterwards.

**The recovery rewrites the location without loading the data.** Straight after a successful recovery, `HT| s_same | has_data = False`. Reading the pixels afterwards returned the new file's colour, so the file is read when it is next wanted rather than at the moment of the repair.

**How deep it searches.** A file eight folders below the chosen one was found, on both machines.

**Which kinds of recorded location it reaches: all six that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic archive had all six rewritten by a single run, on both machines — the linked blend-file included, although the manual gives libraries a separate repair tool of their own.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the sound, the font, the movie clip, the linked blend-file and the Alembic archive were made. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again, with the making of all six written out.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 26 of the hand-test list rebuilt this test with every step named: an 8 by 8 red PNG written by Blender itself; a tenth of a second of silence written by Python's own `wave` module at one channel, two bytes a sample, 8000 samples a second; a font copied from whichever the machine has; a single-frame image sequence `clip_0001.png`; a blend-file holding one collection named `LibCol` with a default cube in it, linked and instanced; and an archive exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` and everything else left at the operator's own defaults. All six files were then moved into one search folder and the command was called once. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), the font taken was `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`, and all six recorded locations were rewritten to `//found/…`, every one of them beginning `//`. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), the font taken was `/System/Library/Fonts/Supplemental/Arial.ttf`, and the same six locations were rewritten, every one beginning `//`. The control was run on each machine as well: with the search folder empty, none of the six moved and the terminal carried six separate `Warning: Could not find '<name>' in '<folder>'` lines, one per kind. **This matches the result recorded above.** *(Note added by the hand tests, second batch, 2026-08-09.)*

**`find_all` rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a working reference moved too, from `//keep/good.png` to `//search/good.png`.

**A packed resource's recorded location is rewritten as well, and it stays packed.** `HT| after | p.png = ('//found/p.png', True)`.

**Whether a recovery can be undone is not settled.** With no window, `bpy.ops.ed.undo()` refused — `Operator bpy.ops.ed.undo.poll() failed, context is incorrect` — even after `bpy.ops.ed.undo_push()` had returned `{'FINISHED'}`, and again with a 3D Viewport context built by hand. Neither machine can answer the version of that test which needs a window: the rented Linux machine has no display server and the Creator ran nothing this round.

**Not covered.** After the hand tests, this answer still does not settle: whether the command reports how many references it fixed; whether a recovery can be undone by a person pressing Ctrl-Z in a Blender that has a window; what happens on Windows, which no Windows machine has run; and why the recovered location comes back relative when the manual says absolute.

**Outside the interface.** **Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved. Everything above about matching, the tie-break and the form of the written-back location was established that way, so all of it holds outside the interface. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. On the documentation itself the position is unchanged and the limit on it stands: none of the terms listed under *Where I looked* is an operating-system word, so the silence recorded there is a by-product of the searches made for the question itself. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**Label: split.**
- That Blender provides explicit, per-kind reload commands which re-read from disk — **documented**.
- That Blender generally notices a changed file by itself and re-reads it — **not in the sources**. Two documented cases come close, and only one of them is a plain exception: with an external text data-block Blender notices the change and warns, and the user then chooses what happens; with an OSL script file there is an option, off by default, described as updating the shader when the file changes.
- That packed data stops using the file on disk — **documented**.
- That a render started afterwards uses the new contents — **inferred**, step written out below.

### What the sources say happens

The sources describe no general watching of external files. What they describe is a set of explicit reload commands, one per kind of external data, plus two narrow cases where Blender does notice on its own.

**Images.** The Image editor has a Reload command:

> "Image ‣ Reload
>
> Shortcut:
>
> Alt-R
>
> Reload the image from the file on drive."

Source: manual. https://docs.blender.org/manual/en/4.5/editors/image/editing.html — read 2026-07-30.

And in the image's own settings:

> "Reload
>
> Reloads the file. Useful when it has been reworked in an external application."

Source: manual. https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html — read 2026-07-30.

From Python, both an operator and a method on the image itself:

> "bpy.ops.image.reload()
>
> Reload current image from disk"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.image.html — read 2026-07-30.

> "reload()
>
> Reload the image from its source path"

Source: Python API documentation, on the Image type. https://docs.blender.org/api/4.5/bpy.types.Image.html — read 2026-07-30.

**Linked blend-files.** Reloading a library pulls in the newer version without reopening:

> "When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file."

Source: manual. https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

The same in Python, as a method and as operators:

> "reload()
>
> Reload this library and all its linked data-blocks"

Source: Python API documentation, on the Library type. https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.

> "bpy.ops.wm.lib_reload(*, library='', filepath='', … sort_method='')
>
> Reload the given library
>
> Parameters:
>
> library (string, (optional, never None)) – Library, Library to reload"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30.

> "RELOCATE
> Relocate – Select a new path for this library, and reload all its data.
>
> RELOAD
> Reload – Reload all data from this library."

Source: Python API documentation, on `bpy.ops.outliner.lib_operation`. https://docs.blender.org/api/4.5/bpy.ops.outliner.html — read 2026-07-30.

If the file has been **replaced** somewhere else rather than changed in place, the documented repair is Relocate:

> "It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

Source: manual. https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

**Other kinds of external data** each have their own reload in Python:

> "bpy.ops.clip.reload()
>
> Reload clip"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.clip.html — read 2026-07-30.

> "bpy.ops.cachefile.reload()
>
> Update objects paths list with new data from the archive"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.cachefile.html — read 2026-07-30.

> "bpy.ops.sequencer.reload(*, adjust_length=False)
>
> Reload strips in the sequencer"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.sequencer.html — read 2026-07-30.

> "bpy.ops.text.reload()
>
> Reload active text data-block from its file"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.text.html — read 2026-07-30.

**The whole blend-file** can be re-read:

> "bpy.ops.wm.revert_mainfile(*, use_scripts=False)
>
> Reload the saved file"

Source: Python API documentation. https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30.

And the manual, writing about files just recovered by Find Missing Files, says that reloading the blend-file reloads every external file in one go:

> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual. https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

### The two cases where Blender notices by itself

**External text data-blocks.** Blender detects the change and raises a conflict for the user to resolve:

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

Source: manual. https://docs.blender.org/manual/en/4.5/editors/text_editor.html — read 2026-07-30.

Note what this says: Blender warns, and the user then chooses. It does not reload on its own.

**OSL script files**, with an option that is off by default:

> "use_auto_update
>
> Automatically update the shader when the .osl file changes (external scripts only)
>
> Type:
>
> boolean, default False"

Source: Python API documentation, on the Script shader node. https://docs.blender.org/api/4.5/bpy.types.ShaderNodeScript.html — read 2026-07-30.

### When the change on disk will not be seen at all

If the data was packed into the blend-file, the file on disk stops being used:

> "Pack Resources
>
> Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them."

Source: manual. https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

And unpacking back onto an existing path does not re-read it:

> "Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked.
> Note that it won't be automatically reloaded from the drive.
> (differs) or (identical) show difference status between the packed version
> and the one on-drive."

Source: manual. https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — read 2026-07-30.

### What has to happen before a later render uses the changed file

The sources do not contain a sentence about renders and changed files. What they contain is the reload commands above, each described as re-reading from the file on disk. So:

- **Documented:** for an image, for a linked blend-file and for a text, the reload is described in so many words as re-reading the file — "Reload the image from the file on drive", "the latest version of the linked data-blocks", "Reload active text data-block from its file". Saving and reopening the blend-file is described as reloading all external files at once, in a sentence about files just recovered by Find Missing Files.
- **Not carried by the quotes:** that the cache reload re-reads a changed file from disk. All the API gives is "Update objects paths list with new data from the archive", which speaks of a list of paths, not of contents. Reading it as re-reading the file would be a step nobody wrote down. *(Citation check, 2026-08-09: this bullet used to put the clip and sequencer reloads in the same boat, saying "All the sources give is 'Reload clip', 'Reload strips in the sequencer'…". That is not what the sources give. The 4.5 manual ties both to a file changed outside Blender — "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender." (manual, https://docs.blender.org/manual/en/4.5/movie_clip/tracking/clip/editing/clip.html — read 2026-07-30) and "Reloads the strips from their external saved location." (manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/editing.html — read 2026-07-30). Both are now quoted below and the bullet is narrowed to the cache reload, which is the only one of the three the sources really leave short.)*

  > "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender."

  Source: manual. https://docs.blender.org/manual/en/4.5/movie_clip/tracking/clip/editing/clip.html — read 2026-07-30.

  > "Reload Strips Alt-R
  >
  > Reloads the strips from their external saved location.
  >
  > Reload Strips and Adjust Length Shift-Alt-R
  >
  > Reloads the strips from their external saved location and re-adjusts the strip duration."

  Source: manual. https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/editing.html — read 2026-07-30.
- **Inferred:** that a render started after such a reload therefore uses the new contents. The step a reader has to supply: the sources say the reload replaces the data-block's contents with what is on disk, and separately that renders use the scene's data-blocks; joining those two gives the conclusion. No source states it.
- **Inferred:** that a Blender process started fresh to render a blend-file reads whatever versions of the external files are on disk at that moment. The step: the manual says reloading the blend-file causes "all external files" to be "reloaded at once", and a fresh open is a load of the blend-file. No source states this about a newly started process.
- **Not in the sources:** whether anything at all is re-read automatically at render time without a reload, for images, linked blend-files, caches, movies or fonts.

### Where I looked, for the "not in the sources" parts

- Searched the whole of `manual_4.5` and `api_4.5` for: `reload`, `Reload`, `relocate`, `revert`, `has been modified`, `was modified`, `modified on`, `modified outside`, `changed outside`, `changed on disk`, `up to date`, `up-to-date`, `stale`, `watch`, `file change`, `external editor`, `resolve conflict`, `Live Edit`, `auto refresh`, `auto_refresh`, `force_reload`.
- Read the seventeen manual pages that mention "Reload" and kept the ones about external data: the Image editor's editing and settings pages, the linked-libraries page, and the Text editor page. The rest (video sequencer navigation and preview header, Node Wrangler, OSL script node, import-images-as-planes, movie clip pages, video editing, the manual's own contribution guides, `advanced/operators`) either repeat these or concern something else.
- Checked the two "Auto Refresh" settings, on the image settings page and on the Image Texture node page, in case they meant re-reading a changed file. They do not: they are about video playback — "Play the video in the Image Editor when the scene animation is playing" and "Update the video texture in the 3D Viewport when moving through the timeline" (https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html and https://docs.blender.org/manual/en/4.5/render/shader_nodes/textures/image.html, read 2026-07-30).
- Searched `release_notes_4.5` for `reload`, `relocate`, `missing`. Hits are bug-fix entries and one core note; nothing changes or states the behaviour asked about here.
- Searched Cycles' own Python files (`cycles_4.5/*.py`) for `reload`, `missing`, `filepath`. The only `reload` occurrences are the add-on reloading its own Python modules (`cycles_4.5/__init__.py`, lines 18–32, under the comment `# Support 'reload' case.`). Cycles' files say nothing about re-reading changed external assets.

### Settled by hand test — observed, 2026-08-09

Fifteen tests were built for this question and run in **Blender 4.5.12 LTS** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 9 to 23.

**What was done, and to what.** A scene whose only visible thing is a plane lit by an emission shader taking its colour from one image file, rendered by Cycles on the processor at eight pixels square with one sample and a Standard view transform, so a render can be read as a single pixel: red comes back as `[0.996, 0.0, 0.0, 1.0]` and blue as `[0.0, 0.0, 0.996, 1.0]`. Files were replaced by the operating system with `shutil.copyfile`, never by Blender, and every replacement was followed by two and a half clear seconds so that a modification time recorded only to the second is plainly newer. Pictures were compared as pictures and never as files. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, with no window open.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on every one of the fifteen tests.**

**An image changed on disk reaches the next render inside a running Blender, with nothing done.** Render one was `[0.996, 0.0, 0.0, 1.0]`; the file was overwritten with a blue one; render two, with nothing touched in between, was `[0.0, 0.0, 0.996, 1.0]`. A second swap to green was picked up the same way at the next render, in a run where `bpy.ops.image.reload()` had just returned `{'CANCELLED'}` and so had done nothing. `Image.reload()` ran without complaint and returned `None`. The operator form refused with no Image editor open and returned `{'FINISHED'}` with one built by hand. *(Citation check, 2026-08-09: an Image editor built by hand is not enough on its own — on the repeat it returned `{'CANCELLED'}` until the image was also put into that editor, and only then `{'FINISHED'}`. Step added by this check.)*

**A freshly started Blender reads the file as it stands.** A second Blender process opening the same blend-file after the swap rendered `[0.0, 0.0, 0.996, 1.0]` where the first had rendered `[0.996, 0.0, 0.0, 1.0]`.

**Persistent Data stops the change being picked up.** With `use_persistent_data` off, the second render showed the new picture. With it on, the second render showed the old one, and it still showed the old one after `Image.reload()`. The setting was present with a default of `False`.

**A linked blend-file behaves the other way round from an image.** With `lib.blend` replaced on disk under a running Blender, the next render was unchanged — `HT| 2 render after the library was replaced, nothing else done = [0.996, 0.0, 0.0, 1.0]`. `Library.reload()` brought the new library in. A freshly started Blender also rendered the new library. `bpy.ops.wm.lib_reload()` failed both ways it was tried, printing `Error: Not a library`; that operator's argument was not worked out and this test does not settle how to call it.

**A Video Sequencer strip picked the change up on its own in this version.** Render one was `[1.0, 0.0, 0.0, 1.0]`, the file was replaced, and render two with nothing else done was `[0.0, 0.0, 1.0, 1.0]`. Both `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()` returned `{'FINISHED'}` with no window open.

**Auto Refresh makes no difference to a changed frame file.** An image sequence was rendered at frame 1 (red) and frame 2 (green) to show the sequence was being followed, then `f_0001.png` was overwritten with white. Frame 1 rendered white at once — with `use_auto_refresh` off and with it on, identically.

**A movie clip does not pick the change up.** `clip.size` stayed `(8, 8)` after the file was replaced with a 16 by 16 one. `bpy.ops.clip.reload()` returned `{'CANCELLED'}` with no Clip editor and changed nothing; with a Clip editor built by hand out of the areas a windowless Blender still keeps it returned `{'FINISHED'}` and `clip.size` became `(16, 16)`. So that command does re-read the file.

**An Alembic archive did not pick the change up at any point.** A cache file replaced with one holding a cube of twice the size measured -1 to 1 throughout: after the swap, after `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context, after the same command returned `{'FINISHED'}` with an `edit_cachefile` context built by hand, after the recorded location was written back to itself, and after the frame was stepped away and back.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the Alembic archive was made, nor how it was attached to the object. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again with the set-up written out, and it came out differently.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 27 of the hand-test list names both missing steps. Two archives were exported, from a cube of size 2 and a cube of size 4, with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)`; the smaller was brought in with `bpy.ops.wm.alembic_import(filepath=…, as_background_job=False, always_add_cache_reader=True)`, which gives the object a Mesh Sequence Cache modifier and a Transform Cache constraint, both printed back from the file. **The importer's own defaults were run as an arm of their own**, on both machines and in all four versions. The object came back with `Cube modifiers = []` and `Cube constraints = []` — no cache reader at all — and measured -1 to 1 at every one of the six points, which is the reading above; but a freshly started Blender opening that file after the swap also measured -1 to 1, on both machines, and a fresh Blender cannot be showing a stale archive. In that same fresh process the replacement archive imported on its own measured -2 to 2, so the archive was fine and the arrangement was watching an object that nothing tied to it. With the cache reader attached, on the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): after the swap with nothing done, -1 to 1; `bpy.ops.cachefile.reload()` with no context `{'CANCELLED'}`, -1 to 1; the same command with an `edit_cachefile` context built by hand `{'FINISHED'}`, and the evaluated mesh then measured **-2 to 2**, dimensions `(4.0, 4.0, 4.0)`; it stayed at -2 to 2 after the recorded location was written back to itself and after the frame was stepped away and back. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same readings at all six points. The archives were 3120 bytes each on the Linux machine and 3095 bytes each on the Mac — the same size as one another, which is why the mesh had to be measured. The controls held on both machines: a freshly started Blender opening the same blend-file after the swap measured -2 to 2, and the replacement archive imported on its own measured -2 to 2, so it really did hold a cube of twice the size. The result above is left exactly as it stands. Both runs stand and neither has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Blender does notice a changed external text file, and the conflict can be resolved with no window.** `Text.is_modified` was `False` while the two copies agreed and `True` once the file on disk differed. The text Blender held did not change until a reload was run. `bpy.ops.text.reload()` refused with no Text editor and returned `{'FINISHED'}` with one built by hand, bringing the new text in; so did `bpy.ops.text.resolve_conflict(resolution='RELOAD')`. This corrects the *inferred* step under *Outside the interface* below, which took a Blender driven from outside to have nobody to resolve the conflict.

**A packed image ignores the file on disk.** With the image packed, the render went on showing the old picture after the file was replaced, and `Image.reload()` — which raised no error — did not bring the disk file in either. The same swap with the image left unpacked did reach the render, which is what shows the swap bit.

> **Unsettled — the citation check's repeat disagreed.** Repeating this on both machines and in all four versions, with the image packed and the blend-file saved and reopened: render one red `[0.659, 0.012, 0.02, 1.0]`, render two after the swap still red, and then, after `Image.reload()`, render three **blue** `[0.004, 0.004, 0.506, 1.0]` — with the image still packed. The control with the image left unpacked went red then blue on both runs. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 24 of the hand-test list ran three arms: the image left unpacked, the image packed with the blend-file saved but *not* reopened, and the image packed with the blend-file saved *and* reopened — the one difference the two earlier records name. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): the unpacked control rendered `[0.996, 0.0, 0.0, 1.0]`, then `[0.0, 0.0, 0.996, 1.0]` after the swap, so the swap bit; packed and not reopened rendered red, red, then **`[0.0, 0.0, 0.996, 1.0]`** after `Image.reload()`, with `packed_file is not None` still `True`; packed and reopened gave the same three renders and stayed packed. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three arms and the same nine renders. The test PNG was 188 bytes in this version. `Image.reload()` returned `None` and raised nothing every time. **This third run matches the citation check's and not the result recorded above.** Whether the blend-file was reopened after packing made no difference on either machine, so that is not what separates the two earlier runs, and this run offers nothing in its place. All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Cycles' texture cache does not exist in this version.** `RenderSettings.use_texture_cache` and `RenderSettings.use_auto_generate_texture_cache` were both absent, and `generate_texture_cache` and `clear_texture_cache` were both absent from the real list of `bpy.ops.render` operators.

**A changed external OSL script reaches the next render with nothing done.** `ShaderNodeScript.use_auto_update` was present with a default of `False`. The first render with the script emitting red was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue and the very next render was `[0.0, 0.0, 0.996, 1.0]`. `bpy.ops.node.shader_script_update()` refused every time it was called, with and without a Shader node editor built by hand, so no update command ever succeeded — and the change reached the render anyway.

> **Unsettled — the citation check's repeat disagreed.** Repeating this — a Script node in External mode emitting red, the `.osl` on disk rewritten to emit blue, the very next render with nothing else done — gave: Creator's Mac 4.5 blue, 5.0 blue, 5.1 red, 5.2 red; rented Linux machine 4.5 red, 5.0 red, 5.1 red, 5.2 blue. The two machines did not agree with each other. The other two parts of this entry did repeat exactly: `use_auto_update` present with a default of `False`, and `bpy.ops.node.shader_script_update()` refusing in every arrangement. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time, with the compiled file watched and the clock measured.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 25 of the hand-test list. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): render 1 with the red `.osl` was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue; render 2, with nothing else done, was **`[0.0, 0.0, 0.996, 1.0]`**; and the compiled `t.oso` sitting beside the `.osl` went from 488 bytes to 499 during that very render, its modification time moving on with it. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three renders, and `t.oso` went from 779 bytes to 852 during render 2. On both machines `bpy.ops.node.shader_script_update()` raised `Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`, with and without a Shader node editor built by hand, so no update command ever succeeded. **The clock was measured rather than assumed.** On the Linux machine two writes a hundredth of a second apart were given recorded times 9,999,768 nanoseconds apart, and the `.osl`'s own recorded time moved from `1786297240470496423` to `1786297243204433052` at the rewrite. On the Mac the same two writes were 10,516,663 nanoseconds apart, and the `.osl`'s recorded time moved from `1786297358068659182` to `1786297360874140959`. Both machines record a modification time to the nanosecond, and nothing in this run was hidden inside one tick of it. Two further controls held on both machines: a genuinely separate Blender process opening the saved blend-file rendered blue, and rendered blue again after every `.oso` had been deleted, so blue was reachable. **This third run matches the result recorded above and not the citation check's.** All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**`bpy.data.file_path_foreach` does not exist in this version.** `HT| bpy.data.file_path_foreach present = False`.

**A file replaced while a render is already running takes effect from a later frame onward.** A twelve-frame animation had its image replaced part-way through: the frames up to a point held the old picture and every frame after it held the new one. No frame came out part old and part new. Which frame the change lands on moves from run to run with the timing.

**Revert takes the blend-file as it stands on disk.** With `scene.blend` opened, then replaced on disk by a different file, Blender still held the old contents until `bpy.ops.wm.revert_mainfile()` was run; it returned `{'FINISHED'}` with no window open and afterwards Blender held the replaced file's object, `['CUBE_B']`. A control Revert with nothing replaced gave `['CUBE_A']` back.

**Not covered.** After the hand tests, this answer still does not settle: what happens when the replacement has a different size, format or colour depth from the file it replaces, except for the movie clip in the test above; what happens if the file is deleted rather than changed; whether a warning banner is drawn in the Text editor, which needs a window; and what happens on Windows, which no Windows machine has run.

**Outside the interface.** **Observed**, 2026-08-09, and it holds further than the sources let me say. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. The data-block methods `Image.reload()` and `Library.reload()`, and `bpy.ops.wm.revert_mainfile()`, all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()` needed no such help in this version and returned `{'FINISHED'}` outright. **The inferred step recorded here before — that a Blender driven from outside has nobody to resolve a text-file conflict — is wrong:** `bpy.ops.text.resolve_conflict(resolution='RELOAD')` ran and did the job with no window. `bpy.ops.node.shader_script_update()` is the one command that refused in every arrangement tried. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. On the documentation itself the position is unchanged and the limit on it stands: none of the terms listed under *Where I looked* is an operating-system word. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

**All nine items that stood here have been run**, on 2026-08-09, in Blender 4.5.12 LTS on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery.** Break an image path, run Find Missing Files, then press Ctrl-Z in the 3D Viewport and read the image's path again. *The other way:* the path stays as the search wrote it. With no window `bpy.ops.ed.undo()` refuses outright, so this half is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates. The terminal says nothing about ambiguity, and a control run shows the terminal does carry a message when there is one; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above; whether the banner appears is not.

**Not run at all.** Anything about Windows — no Windows machine has run it. Two things stay untested there: whether the search matches a differently-cased file name on a file system that preserves capital letters without treating them as significant, and whether a file can be replaced on disk while Blender holds it open.

**Wants a fresh look at the sources before it can be tested.** What `RELOAD_EDITED` does to `bpy.data.file_path_foreach`. The function is absent in this version, so there is nothing here to test; the question belongs to the versions that have it.

---

I did the whole of this work myself. No helper agents were started, so there is nobody else's reading to vouch for.

2026-08-09 — Opus 5 (Claude Code) — Reading a file, Blender 4.5

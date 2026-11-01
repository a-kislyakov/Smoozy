# Reading a file — later questions, second pass (Blender 4.5)

Sources consulted, all inside `Blender Sources/`, snapshot taken 2026-07-30, read by me on 2026-07-30 (the snapshot date is my date read):
the Blender 4.5 manual (`manual_4.5`), the Blender 4.5 Python API documentation (`api_4.5`), the Blender 4.5 release notes (`release_notes_4.5`), and Cycles' own files (`cycles_4.5`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 10 and 11 only.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Reading taken.** I read "a search for a missing reference" as Blender's *Find Missing Files* command (`File ‣ External Data ‣ Find Missing Files`, `bpy.ops.file.find_missing_files`), which is the only searching-for-missing-references feature I found in the sources. If the question instead meant some search Blender does silently while opening a file, see the "Not covered" note below — the sources describe no such search.

**LABEL: split.**
- What makes a file a possible match — **not in the sources**.
- Which one is chosen when more than one turns up — **not in the sources**.
- What location is written back — **documented**.

### What the sources do say

The manual describes the whole command in five sentences:

> "Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

So, documented:

- The search area is a directory the user picks, plus every directory inside it, to any depth ("recursively in all contained directories").
- **The location written back is an absolute path.** "Those recoveries will be done as absolute paths". To end up with relative paths instead, a separate command has to be run afterwards: "Make Paths Relative", described on the same page as "Make all paths to external files relative to the current blend-file."

The API documentation adds one option that the manual does not mention:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, …)
>
> Try to find missing external files"

and

> "find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)"

Source: Python API, https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30.

That is the complete text the sources carry about this command.

### What the sources do not say

Nothing in the manual, the API documentation, the release notes, or Cycles' files states:

- **What makes a file a possible match.** The manual says only "Every missing file found in the search will be recovered". It never says what "found" means — whether the file name must match exactly, whether the extension must match, whether case matters, whether the size or type is checked. No matching rule is written down anywhere.
- **Which file is chosen when more than one could match.** The sources never raise the possibility of two candidates, and give no rule for picking between them — not "first found", not "shallowest", not "alphabetical", not "the user is asked". The word "recursively" tells you the search covers subdirectories but not in what order they are walked, so even a "first one wins" rule would have no defined meaning from these sources alone.
- Whether anything is reported when a search finds nothing, or when it finds several.
- Which kinds of reference the search covers (images, sounds, fonts, caches, linked blend-files, and so on). The manual's wording is "external files" and "broken links" throughout, with no list.

The `find_all` option's wording — "Find all files in the search path (not just missing)" — tells you the search can be widened beyond files currently marked missing, but it still does not say what counts as a match.

**Not covered.** This answer does not settle the matching rule, the tie-break, case handling, extension handling, or the order directories are walked. It does not settle whether Find Missing Files touches linked blend-file paths as well as image and media paths. It does not settle what happens when the search finds nothing. It does not settle whether packed files are affected.

### Where I looked, and what I ruled out

Searches run with `grep`, case-insensitive, over `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` together, ignoring the generated index pages:

`find_missing`, `missing files`, `missing` (every hit in the whole manual read), `Find All`, `not just missing`, `broken link`, `broken path`, `recursively`, `recover`, `search path`, `same name`, `first match`, `ambiguous`, `relocate`, `remap`, `candidate`, `more than one file`, `multiple matches`, `several files`, `which file`, `conflict`, `case sensitiv`, `case-sensitiv`, `case insensitiv`.

The search on `missing` across the whole manual returned 42 hits. Exactly two pages concern missing external references: the topbar page quoted above, and https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html . Every other hit was unrelated (missing shadow updates in EEVEE, missing caustics in Cycles, missing NLA strips, missing data layers in the Data Transfer modifier, missing features in the FBX and glTF exporters, and so on).

Pages I opened and ruled out:

- https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — covers *Relocate Library* ("a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file."), *Relocate Linked ID*, and *Broken Libraries* ("If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones."). Ruled out because relocating is the user naming one replacement file by hand; it is not a search, so there is never more than one possible file.
- https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — the unpack options do describe a choice between a local path and the original path ("Use <original file path> (differs)|(identical)", "Overwrite <original file path>", "Create <original file path>"). Ruled out: unpacking writes files out of the blend-file, it does not search for a missing one.
- https://docs.blender.org/manual/en/4.5/files/file_paths.html — read in full. It is entirely about the new path template expressions like `{blend_name}`; nothing about searching for files.
- https://docs.blender.org/api/4.5/bpy.path.html — I listed every function on the page (`abspath`, `basename`, `clean_name`, `display_name`, `display_name_to_filepath`, `display_name_from_filepath`, `ensure_ext`, `is_subdir`, `module_names`, `native_pathsep`, `reduce_dirs`, `relpath`, `resolve_ncase`). None describes a missing-file search. `resolve_ncase` is the closest thing to a matching rule anywhere in the sources — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." — but nothing states that Find Missing Files uses it, so I am not claiming it does.
- https://docs.blender.org/api/4.5/bpy.ops.file.html — I listed every operator on the page. Besides `find_missing_files` the only related ones are `report_missing_files()` ("Report all missing external files"), `make_paths_absolute()` and `make_paths_relative()`. None adds a matching rule.
- Release notes for 4.5: I searched all nineteen subject files for `path`, `missing`, `reload` and `librar`. https://developer.blender.org/docs/release_notes/4.5/core/ mentions linking ("Linked IDs can now also be relocated, if they are not a dependency of any other linked data") and path templates; https://developer.blender.org/docs/release_notes/4.5/python_api/ mentions render path templates and a new warning on `StringProperty` path subtypes. Nothing about finding missing files.
- Cycles' own files: searched `cycles_4.5` for `missing`, `filepath`, `reload` and `cache`. The only path handling is in `osl.py` and `operators.py`, both about OSL scripts and denoising output paths. Cycles has no missing-file search of its own, so it cannot answer this question.

### Settled by hand test — observed, 2026-08-09

Eight tests were built for this question and run in **Blender 4.5.12 LTS** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 1 to 8.

**What was done, and to what.** A blend-file holding one image texture whose file was then deleted, reopened, and repaired by calling `bpy.ops.file.find_missing_files(directory=<folder>)` from a script. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, and no window was open at any point.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on all eight tests.**

**The command runs with no window at all.** Both machines printed `HT| s_same | operator returned = "{'FINISHED'}"`, and the recorded location changed.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension — `tex.jpg` for a missing `tex.png` — was not, and neither was a file of a different name holding the same picture. Both printed the same line as a folder holding nothing at all: `Warning: Could not find 'tex.png' in '<folder>'`.

**Letter case has to match, on both machines.** With only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it — `HT| s_upper | filepath after = '//orig/tex.png'` — while the control run with `tex.png` on disk recovered it on both. The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant. So the note above about `bpy.path.resolve_ncase` does not describe what this command does.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run. Wherever the candidates differed in size, the recorded location ended up pointing at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them: `HT| sA | filepath after = '//sA/zzz/tex.png'` against `HT| sB | filepath after = '//sB/aaa/tex.png'`, the 188-byte file winning both times over the 187-byte one. A file three folders down beat one at the top of the search folder on a single byte, and a 3908-byte file beat a 187-byte one from either position. Where two candidates were exactly the same size, `aaa` was taken rather than `zzz`.

> **The byte sizes measured again.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 28 of the hand-test list ran the tie-break arrangement once more and recorded the exact size of every candidate. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), this version wrote **188 bytes** for the red and blue 8 by 8 pictures, **187** for the green and white, **201** for the 16 by 16 one and **3908** for the 64 by 64 one. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), this version wrote the same four figures. Across the four versions: 4.5.12 LTS and 5.0.1 both wrote 188, 187, 201 and 3908; 5.1.2 and 5.2.0 LTS both wrote 261, 260, 274 and 3981. The choice came out the same at both sets of sizes and on both machines — the larger candidate won from either folder, and where the two candidates were byte-for-byte the same size `aaa` was taken. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Nothing is said about there having been more than one.** No line about ambiguity appeared in any arrangement. The control is what makes that worth recording: with nothing to find, the same terminal carried `Warning: Could not find 'tex.png' in '<folder>'`, so this is a silence about ambiguity and not an unwatched channel.

**The location written back began `//`, not at the system root.** On both machines the recovered location came back as `//s_same/deep/tex.png`, with `HT| s_same | starts with // = True`. That is not what the manual sentence quoted above describes. What is recorded here is what appeared; nothing in the test separates the command's own behaviour from a preference that might turn the path relative afterwards.

**The recovery rewrites the location without loading the data.** Straight after a successful recovery, `HT| s_same | has_data = False`. Reading the pixels afterwards returned the new file's colour. So "recovered" means the recorded location is rewritten and the file is read when it is next wanted — which the manual's note that recovered files "might need to be reloaded" sits beside without stating.

**How deep it searches.** A file eight folders below the chosen one was found, on both machines.

**Which kinds of recorded location it reaches: all six that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic archive had all six rewritten by a single run, on both machines — the linked blend-file included, although the manual gives libraries a separate repair tool of their own.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the sound, the font, the movie clip, the linked blend-file and the Alembic archive were made. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again, with the making of all six written out.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 26 of the hand-test list rebuilt this test with every step named: an 8 by 8 red PNG written by Blender itself; a tenth of a second of silence written by Python's own `wave` module at one channel, two bytes a sample, 8000 samples a second; a font copied from whichever the machine has; a single-frame image sequence `clip_0001.png`; a blend-file holding one collection named `LibCol` with a default cube in it, linked and instanced; and an archive exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` and everything else left at the operator's own defaults. All six files were then moved into one search folder and the command was called once. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), the font taken was `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`, and all six recorded locations were rewritten to `//found/…`, every one of them beginning `//`. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), the font taken was `/System/Library/Fonts/Supplemental/Arial.ttf`, and the same six locations were rewritten, every one beginning `//`. The control was run on each machine as well: with the search folder empty, none of the six moved and the terminal carried six separate `Warning: Could not find '<name>' in '<folder>'` lines, one per kind. **This matches the result recorded above.** *(Note added by the hand tests, second batch, 2026-08-09.)*

**`find_all` rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a working reference moved too, from `//keep/good.png` to `//search/good.png`.

**A packed resource's recorded location is rewritten as well, and it stays packed.** `HT| after | p.png = ('//found/p.png', True)`.

**Whether a recovery can be undone is not settled.** With no window, `bpy.ops.ed.undo()` refused — `Operator bpy.ops.ed.undo.poll() failed, context is incorrect` — even after `bpy.ops.ed.undo_push()` had returned `{'FINISHED'}`, and again with a 3D Viewport context built by hand.

**Outside the interface.** **Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved. Everything above about matching, the tie-break and the form of the written-back location was established that way, so all of it holds outside the interface. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. The adjacent documented fact above — that Blender ships `bpy.path.resolve_ncase` for case-insensitive resolution — remains a note about a separate helper, and the tests show it does not describe this command. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**Reading taken.** I read "a file Blender has already read" as any file Blender loaded and is holding — an image, a video, a linked blend-file, a text, a cache archive. The other reading is the open `.blend` itself; I cover that separately at the end. The question is ambiguous on that point and I have answered both.

**LABEL: documented**, for what has to happen (a reload, per kind of file, or a reload of the whole blend-file). **Not in the sources**, for what Blender does on its own when the file changes underneath it — no source states that Blender notices, warns, or re-reads by itself.

### What the sources say has to happen

Every kind of external file Blender reads has its own explicit reload command, and the manual describes several of them as being for exactly this situation.

**Images.**

> "Reload
>
> Reloads the file. Useful when it has been reworked in an external application."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html — read 2026-07-30.

> "Reload the image from the file on drive."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/image/editing.html — read 2026-07-30 (this is the `Image ‣ Reload` entry, shortcut Alt-R).

In Python: "bpy.ops.image.reload()" / "Reload current image from disk" (https://docs.blender.org/api/4.5/bpy.ops.image.html), and on the data itself "reload()" / "Reload the image from its source path" (https://docs.blender.org/api/4.5/bpy.types.Image.html). Both read 2026-07-30.

**Movie clips.**

> "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender."

Source: manual, https://docs.blender.org/manual/en/4.5/movie_clip/tracking/clip/editing/clip.html — read 2026-07-30. In Python: "bpy.ops.clip.reload()" / "Reload clip" (https://docs.blender.org/api/4.5/bpy.ops.clip.html).

**Video sequencer strips.**

> "Reload Strips Alt-R
>
> Reloads the strips from their external saved location.
>
> Reload Strips and Adjust Length Shift-Alt-R
>
> Reloads the strips from their external saved location and re-adjusts the strip duration."

Source: manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/editing.html — read 2026-07-30. In Python: "bpy.ops.sequencer.reload(*, adjust_length=False)" / "Reload strips in the sequencer" (https://docs.blender.org/api/4.5/bpy.ops.sequencer.html).

The sequencer page also shows the case of files that were absent and later appear on disk:

> "When the missing frames are rendered or placed in the same folder,
> you can refresh
> the Sequencer and get the missing frames in the strip."

Source: manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/image.html — read 2026-07-30.

**Linked blend-files (libraries).**

> "Link creates a reference to data in a source file such that changes made there will be
> reflected in the current file the next time it is reloaded."

and

> "When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file."

Source: manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

In Python: "bpy.ops.wm.lib_reload(…)" / "Reload the given library" (https://docs.blender.org/api/4.5/bpy.ops.wm.html), and the outliner operation "RELOAD / Reload – Reload all data from this library." (https://docs.blender.org/api/4.5/bpy.ops.outliner.html). Both read 2026-07-30.

**Cache archives (Alembic, USD).** "bpy.ops.cachefile.reload()" / "Update objects paths list with new data from the archive" (https://docs.blender.org/api/4.5/bpy.ops.cachefile.html, read 2026-07-30). The Alembic import page adds: "Add cache modifiers and constraints to imported objects even if they are not animated so that they can be updated when reloading the Alembic archive." (https://docs.blender.org/manual/en/4.5/files/import_export/alembic.html, read 2026-07-30).

**Texts.** This is the one place the sources say outright that Blender can tell the disk copy has diverged, and offer a choice:

> "is_modified
>
> Text file on disk is different than the one in memory"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.Text.html — read 2026-07-30.

> "bpy.ops.text.resolve_conflict(*, resolution='IGNORE')
>
> When external text is out of sync, resolve the conflict"
>
> "resolution (enum in ['IGNORE', 'RELOAD', 'SAVE', 'MAKE_INTERNAL'], (optional)) – Resolution, How to solve conflict due to differences in internal and external text"

Source: Python API, https://docs.blender.org/api/4.5/bpy.ops.text.html — read 2026-07-30.

Note the default resolution is `IGNORE`, and that the four choices are offered at all: this is a divergence the user is asked to settle, not one Blender settles on its own.

**Packed files.** A file that was packed into the blend-file and then marked unpacked is explicitly stated not to refresh by itself:

> "Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked.
> Note that it won't be automatically reloaded from the drive.
> (differs) or (identical) show difference status between the packed version
> and the one on-drive."

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — read 2026-07-30. The same page's "Remove Pack" option: "Just mark the file as unpacked, without actually writing it or reloading it from the drive."

**The blanket route — reload everything at once.** The topbar page, describing what to do after Find Missing Files has repaired paths, gives the general remedy:

> "Note
>
> Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

### What the sources do not say

- **No source says Blender watches the disk.** There is no statement anywhere that Blender detects an image, video, library or cache file changing on disk, no statement that it warns, and no statement that it re-reads it at the start of a render. The existence of a manual Reload for every kind of file, plus the wording "Useful when it has been reworked in an external application" and "Is mainly useful when the clip gets edited outside of Blender", is what the sources give. Drawing "therefore Blender never re-reads by itself" from that would be a step the sources do not license, so I am not making it — I am reporting that the sources describe only manual reloads.
- **Auto Refresh is not a disk watch.** It is worth stating plainly because the name invites the wrong reading. The API describes it as "use_auto_refresh (boolean, (optional)) – Auto Refresh, Always refresh image on frame changes" (https://docs.blender.org/api/4.5/bpy.ops.image.html). The manual describes it for the Image editor as "Play the video in the Image Editor when the scene animation is playing." (https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html) and for the Image Texture node as "Update the video texture in the 3D Viewport when moving through the timeline." (https://docs.blender.org/manual/en/4.5/render/shader_nodes/textures/image.html). All three tie it to the frame changing, not to the file changing. Both read 2026-07-30.
- **Nothing about a render already running.** No source says what happens if a file is changed or replaced while a render is under way.
- **Nothing about a fresh Blender.** No source states whether starting a new Blender to render picks up the changed file. (Reading a `.blend` from scratch is the ordinary open case, but the sources do not spell that out in these terms, so I will not claim it.)
- **Nothing about partly-written files, network shares, or file locking.**

### The other reading — the open `.blend` itself changing on disk

The only related thing the sources carry is the manual Revert command:

> "Revert
>
> Reopens the current file to its last saved version."

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30. In Python: "bpy.ops.wm.revert_mainfile(*, use_scripts=False)" / "Reload the saved file" (https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30).

No source says Blender notices that the open `.blend` was changed or replaced by something else, and no source says it warns before saving over such a file. I searched for this specifically (terms below) and found nothing.

### One thing I could check in Cycles' own files — OSL scripts

**LABEL for this part: observed.**

Cycles is supplied here as its own files, and one of them shows how a script text whose disk copy has changed is handled.

In `Blender Sources/cycles_4.5/osl.py`, in the function `update_internal_script`, line 211:

```
    if script.is_in_memory or script.is_dirty or script.is_modified or not os.path.exists(osl_path):
```

and the two branches that follow, lines 212–222:

```
        # write text datablock contents to temporary file
        osl_file = tempfile.NamedTemporaryFile(mode='w', suffix=".osl", delete=False)
        osl_file.write(script.as_string())
        …
    else:
        # compile text datablock from disk directly
        ok, oso_path = osl_compile(osl_path, report)
```

Read from the file 2026-07-30.

What the file says, plainly: when the text Blender holds differs from the file on disk (`is_modified`, which the API documents as "Text file on disk is different than the one in memory"), Cycles compiles **the copy Blender holds**, not the file on disk. Only when they agree does it compile the file on disk directly.

When this code runs is also visible. In `Blender Sources/cycles_4.5/__init__.py`, lines 107–111:

```
    def update_script_node(self, node):
        if engine.with_osl():
            from . import osl
            osl.update_script_node(node, self.report)
```

This is a render-engine callback, and the manual names the matching command: "Script Node Update — Reloads the text file data-block, creating new inputs and outputs if needed." (manual, https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html, read 2026-07-30).

**The line I am not crossing:** the files show that this compile path exists and which copy it prefers. They do not show that it runs at render time, and I did not run a render. Whether a render triggers this is **not in the sources**.

For the *External* mode of the same node the manual says: "Used to specify a .osl file from a drive, and this will then be automatically compiled into a .oso file in the same directory. It is also possible to specify a path to a .oso file, which will then be used directly, with compilation done manually by the user." (same page). The sources do not say when that automatic compilation is re-run if the `.osl` on disk changes.

**Not covered.** This answer does not settle what Blender does on its own when a file changes on disk — only that the sources describe manual reloads and describe no automatic one. It does not settle whether a render re-reads anything. It does not settle behaviour during a render in progress. It does not settle whether a reload of one image affects other data-blocks pointing at the same file. It does not settle what happens when the replacement file has different dimensions, length, or format.

### Where I looked, and what I ruled out

Searches run with `grep`, case-insensitive, over `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5`, ignoring the generated index pages:

`reload`, `Reload`, `refresh`, `auto_refresh`, `Auto Refresh`, `is_modified`, `is_dirty`, `resolve_conflict`, `revert`, `revert_mainfile`, `outside of Blender`, `external application`, `changed on disk`, `modified since`, `out of sync`, `up to date`, `up-to-date`, `stale`, `timestamp`, `mtime`, `file changed`, `newer version`, `latest version`, `cache`, `missing`.

Terms that returned nothing at all anywhere in the sources: `modified since`, `mtime`, `file changed`, `changed on disk`, `multiple matches`, `more than one file`. The absence of `modified since` and `mtime` is the main basis for saying the sources describe no disk-watching.

Pages opened and used: the topbar page, the Link/Append page, the Packed Data page, the Image editor Editing and Image Settings pages, the Image Texture node page, the Script node page, the movie clip Clip page, the sequencer Editing and Image Strip pages, the Alembic page, and the API pages for `bpy.ops.image`, `bpy.ops.clip`, `bpy.ops.sequencer`, `bpy.ops.text`, `bpy.ops.cachefile`, `bpy.ops.wm`, `bpy.ops.outliner`, `bpy.types.Text`, `bpy.types.Image`, `bpy.path`.

Ruled out:

- https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html — read in full. Covers relative-path handling in Python and string encoding only; nothing about files changing on disk.
- https://docs.blender.org/manual/en/4.5/files/file_paths.html — read in full. Path templates only.
- Release notes for 4.5, all nineteen subject files searched for `reload`, `path`, `missing`, `librar`. Only https://developer.blender.org/docs/release_notes/4.5/core/ and https://developer.blender.org/docs/release_notes/4.5/python_api/ had hits, both about linked-ID relocation and path templates. Nothing about reloading changed files.
- `Blender Sources/cycles_4.5` — searched for `reload`, `cache`, `filepath`, `missing`. The only relevant material is `osl.py` (used above) and `operators.py` (denoising output paths, not related).

### Settled by hand test — observed, 2026-08-09

Fifteen tests were built for this question and run in **Blender 4.5.12 LTS** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 9 to 23.

**What was done, and to what.** A scene whose only visible thing is a plane lit by an emission shader taking its colour from one image file, rendered by Cycles on the processor at eight pixels square with one sample and a Standard view transform, so a render can be read as a single pixel: red comes back as `[0.996, 0.0, 0.0, 1.0]` and blue as `[0.0, 0.0, 0.996, 1.0]`. Files were replaced by the operating system with `shutil.copyfile`, never by Blender, and every replacement was followed by two and a half clear seconds so that a modification time recorded only to the second is plainly newer. Pictures were compared as pictures and never as files. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, with no window open.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on every one of the fifteen tests.**

**An image changed on disk reaches the next render inside a running Blender, with nothing done.** Render one was `[0.996, 0.0, 0.0, 1.0]`; the file was overwritten with a blue one; render two, with nothing touched in between, was `[0.0, 0.0, 0.996, 1.0]`. `Image.reload()` ran without complaint and returned `None`; `bpy.ops.image.reload()` returned `{'CANCELLED'}` with no Image editor open and `{'FINISHED'}` with one built by hand *(Citation check, 2026-08-09: an Image editor built by hand is not enough on its own — on the repeat it returned `{'CANCELLED'}` until the image was also put into that editor, and only then `{'FINISHED'}`. Step added by this check.)* — and a further swap was picked up at the next render even in the run where the operator had done nothing.

**A freshly started Blender reads the file as it stands.** A second Blender process opening the same blend-file after the swap rendered `[0.0, 0.0, 0.996, 1.0]` where the first had rendered `[0.996, 0.0, 0.0, 1.0]`.

**Persistent Data stops the change being picked up.** With `use_persistent_data` off, the second render showed the new picture. With it on, the second render showed the old one, and it still showed the old one after `Image.reload()`. The setting was present with a default of `False`.

**A linked blend-file behaves the other way round from an image.** With `lib.blend` replaced on disk under a running Blender, the next render was unchanged. `Library.reload()` brought the new library in, and a freshly started Blender also rendered it. `bpy.ops.wm.lib_reload()` failed both ways it was tried, printing `Error: Not a library`; that operator's argument was not worked out and this test does not settle how to call it.

**A Video Sequencer strip picked the change up on its own in this version.** Render one was `[1.0, 0.0, 0.0, 1.0]`, the file was replaced, and render two with nothing else done was `[0.0, 0.0, 1.0, 1.0]`. Both `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()` returned `{'FINISHED'}` with no window open.

**Auto Refresh makes no difference to a changed frame file.** An image sequence was rendered at frame 1 (red) and frame 2 (green) to show the sequence was being followed, then `f_0001.png` was overwritten with white. Frame 1 rendered white at once — with `use_auto_refresh` off and with it on, identically.

**A movie clip does not pick the change up, and its reload does re-read the file.** `clip.size` stayed `(8, 8)` after the file was replaced with a 16 by 16 one. `bpy.ops.clip.reload()` returned `{'CANCELLED'}` with no Clip editor; with one built by hand it returned `{'FINISHED'}` and `clip.size` became `(16, 16)`.

**An Alembic archive did not pick the change up at any point.** A cache file replaced with one holding a cube of twice the size measured -1 to 1 throughout: after the swap, after `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context, after the same command returned `{'FINISHED'}` with an `edit_cachefile` context built by hand, after the recorded location was written back to itself, and after the frame was stepped away and back.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the Alembic archive was made, nor how it was attached to the object. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again with the set-up written out, and it came out differently.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 27 of the hand-test list names both missing steps. Two archives were exported, from a cube of size 2 and a cube of size 4, with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)`; the smaller was brought in with `bpy.ops.wm.alembic_import(filepath=…, as_background_job=False, always_add_cache_reader=True)`, which gives the object a Mesh Sequence Cache modifier and a Transform Cache constraint, both printed back from the file. **The importer's own defaults were run as an arm of their own**, on both machines and in all four versions. The object came back with `Cube modifiers = []` and `Cube constraints = []` — no cache reader at all — and measured -1 to 1 at every one of the six points, which is the reading above; but a freshly started Blender opening that file after the swap also measured -1 to 1, on both machines, and a fresh Blender cannot be showing a stale archive. In that same fresh process the replacement archive imported on its own measured -2 to 2, so the archive was fine and the arrangement was watching an object that nothing tied to it. With the cache reader attached, on the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): after the swap with nothing done, -1 to 1; `bpy.ops.cachefile.reload()` with no context `{'CANCELLED'}`, -1 to 1; the same command with an `edit_cachefile` context built by hand `{'FINISHED'}`, and the evaluated mesh then measured **-2 to 2**, dimensions `(4.0, 4.0, 4.0)`; it stayed at -2 to 2 after the recorded location was written back to itself and after the frame was stepped away and back. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same readings at all six points. The archives were 3120 bytes each on the Linux machine and 3095 bytes each on the Mac — the same size as one another, which is why the mesh had to be measured. The controls held on both machines: a freshly started Blender opening the same blend-file after the swap measured -2 to 2, and the replacement archive imported on its own measured -2 to 2, so it really did hold a cube of twice the size. The result above is left exactly as it stands. Both runs stand and neither has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Blender does notice a changed external text file, and the conflict can be resolved with no window.** `Text.is_modified` was `False` while the two copies agreed and `True` once the file on disk differed. The text Blender held did not change until a reload was run. `bpy.ops.text.reload()` refused with no Text editor and returned `{'FINISHED'}` with one built by hand; so did `bpy.ops.text.resolve_conflict(resolution='RELOAD')`, which brought the new text in.

**A packed image ignores the file on disk.** With the image packed, the render went on showing the old picture after the file was replaced, and `Image.reload()` — which raised no error — did not bring the disk file in either. The same swap with the image left unpacked did reach the render, which is what shows the swap bit.

> **Unsettled — the citation check's repeat disagreed.** Repeating this on both machines and in all four versions, with the image packed and the blend-file saved and reopened: render one red `[0.659, 0.012, 0.02, 1.0]`, render two after the swap still red, and then, after `Image.reload()`, render three **blue** `[0.004, 0.004, 0.506, 1.0]` — with the image still packed. The control with the image left unpacked went red then blue on both runs. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 24 of the hand-test list ran three arms: the image left unpacked, the image packed with the blend-file saved but *not* reopened, and the image packed with the blend-file saved *and* reopened — the one difference the two earlier records name. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): the unpacked control rendered `[0.996, 0.0, 0.0, 1.0]`, then `[0.0, 0.0, 0.996, 1.0]` after the swap, so the swap bit; packed and not reopened rendered red, red, then **`[0.0, 0.0, 0.996, 1.0]`** after `Image.reload()`, with `packed_file is not None` still `True`; packed and reopened gave the same three renders and stayed packed. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three arms and the same nine renders. The test PNG was 188 bytes in this version. `Image.reload()` returned `None` and raised nothing every time. **This third run matches the citation check's and not the result recorded above.** Whether the blend-file was reopened after packing made no difference on either machine, so that is not what separates the two earlier runs, and this run offers nothing in its place. All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Cycles' texture cache does not exist in this version.** `RenderSettings.use_texture_cache` and `RenderSettings.use_auto_generate_texture_cache` were both absent, and `generate_texture_cache` and `clear_texture_cache` were both absent from the real list of `bpy.ops.render` operators.

**A changed external OSL script reaches the next render with nothing done.** This bears directly on the *observed* reading of `cycles_4.5/osl.py` above. `ShaderNodeScript.use_auto_update` was present with a default of `False`. The first render with the script emitting red was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue and the very next render was `[0.0, 0.0, 0.996, 1.0]`. `bpy.ops.node.shader_script_update()` refused every time it was called, with and without a Shader node editor built by hand, so no update command ever succeeded — and the change reached the render anyway.

> **Unsettled — the citation check's repeat disagreed.** Repeating this — a Script node in External mode emitting red, the `.osl` on disk rewritten to emit blue, the very next render with nothing else done — gave: Creator's Mac 4.5 blue, 5.0 blue, 5.1 red, 5.2 red; rented Linux machine 4.5 red, 5.0 red, 5.1 red, 5.2 blue. The two machines did not agree with each other. The other two parts of this entry did repeat exactly: `use_auto_update` present with a default of `False`, and `bpy.ops.node.shader_script_update()` refusing in every arrangement. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time, with the compiled file watched and the clock measured.** **Observed**, 2026-08-09, Blender 4.5.12 LTS. Entry 25 of the hand-test list. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): render 1 with the red `.osl` was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue; render 2, with nothing else done, was **`[0.0, 0.0, 0.996, 1.0]`**; and the compiled `t.oso` sitting beside the `.osl` went from 488 bytes to 499 during that very render, its modification time moving on with it. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three renders, and `t.oso` went from 779 bytes to 852 during render 2. On both machines `bpy.ops.node.shader_script_update()` raised `Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`, with and without a Shader node editor built by hand, so no update command ever succeeded. **The clock was measured rather than assumed.** On the Linux machine two writes a hundredth of a second apart were given recorded times 9,999,768 nanoseconds apart, and the `.osl`'s own recorded time moved from `1786297240470496423` to `1786297243204433052` at the rewrite. On the Mac the same two writes were 10,516,663 nanoseconds apart, and the `.osl`'s recorded time moved from `1786297358068659182` to `1786297360874140959`. Both machines record a modification time to the nanosecond, and nothing in this run was hidden inside one tick of it. Two further controls held on both machines: a genuinely separate Blender process opening the saved blend-file rendered blue, and rendered blue again after every `.oso` had been deleted, so blue was reachable. **This third run matches the result recorded above and not the citation check's.** All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**`bpy.data.file_path_foreach` does not exist in this version.** `HT| bpy.data.file_path_foreach present = False`.

**A file replaced while a render is already running takes effect from a later frame onward.** A twelve-frame animation had its image replaced part-way through: the frames up to a point held the old picture and every frame after it held the new one. No frame came out part old and part new.

**Revert takes the blend-file as it stands on disk.** With `scene.blend` opened, then replaced on disk by a different file, Blender still held the old contents until `bpy.ops.wm.revert_mainfile()` was run; it returned `{'FINISHED'}` with no window open and afterwards Blender held the replaced file's object, `['CUBE_B']`. A control Revert with nothing replaced gave `['CUBE_A']` back. So the reading the sources were silent on — the open `.blend` itself changing underneath Blender — is settled for what Revert then gives, though not for whether Blender notices on its own.

**Outside the interface.** **Observed**, 2026-08-09, and it reaches further than the sources let me say. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()` needed no such help in this version. The gap named above — whether Blender re-reads external files at render time when nobody is there to press Reload — is now answered: for an image, an image sequence and an external OSL script it does; for a linked blend-file, a movie clip, an Alembic archive and a packed image it does not. `bpy.ops.node.shader_script_update()` is the one command that refused in every arrangement tried. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. The two adjacent documented per-system facts named above — the OSL shader search path and `bpy.path.resolve_ncase` — are still about something else. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

**All sixteen items that stood here have been run**, on 2026-08-09, in Blender 4.5.12 LTS on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, or when a file changes on disk. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor, and whether any warning appears on trying to save over a blend-file that was replaced on disk. That `Text.is_modified` becomes `True`, and that Revert then gives the file as it stands on disk, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open on the sources rather than the machine.** How `bpy.ops.wm.lib_reload()` is meant to be called: it failed with `Error: Not a library` on both attempts, and `Library.reload()` did the job instead, so the behaviour question is answered and the operator's argument is not.

---

2026-08-09 — Opus 5 (Claude Code) — Reading a file, Blender 4.5

# Reading a file — later questions (Blender 5.1)

Sources consulted, all as the snapshot taken on **2026-07-30**:

- Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), pages at `https://docs.blender.org/manual/en/5.1/`
- Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), pages at `https://docs.blender.org/api/5.1/`
- Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), pages at `https://developer.blender.org/docs/release_notes/5.1/`
- Cycles' own files (`Blender Sources/cycles_5.1`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 10 and 11 only.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Short answer.** The search itself is documented. What it counts as a match, and what it does when several files could match, is not. The location written back is documented: an absolute path.

### What the sources do say

There is one such search in the manual: **Find Missing Files**, in the topbar under File ‣ External Data.

> Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative.

- Source: manual
- Address: https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
- Date read: 2026-07-30

The same page, just above, describes the companion command that only reports:

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files.

- Source: manual
- Address: https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
- Date read: 2026-07-30

The Python side names one option the manual does not mention:

> `bpy.ops.file.find_missing_files(*, find_all=False, directory='', ...)`
>
> Try to find missing external files
>
> `find_all` (bool) – Find All, Find all files in the search path (not just missing) (optional)
>
> `directory` (str) – Directory, Directory of the file (optional, never None)

and, on the same page:

> `bpy.ops.file.report_missing_files()`
>
> Report all missing external files

- Source: Python API documentation
- Address: https://docs.blender.org/api/5.1/bpy.ops.file.html
- Date read: 2026-07-30

**Label for this part: documented.**

From this, the three parts of the question come out as follows.

**What location is written back — documented.** An absolute path. "Those recoveries will be done as absolute paths". If relative paths are wanted, a separate command has to be run afterwards: "Make all paths to external files relative to the current blend-file." (same manual page, Make Paths Relative).

**What makes a file a possible match — not in the sources.** No page in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, or Cycles' own files states the rule. The manual says only that "Every missing file found in the search will be recovered", and never defines "found". It does not say whether the match is on file name alone, on file name plus extension, on file size or contents, or on anything else. It does not say whether letter case matters.

**Which one is chosen when more than one could match — not in the sources.** Nothing states an order of preference, a tie-break, a first-wins or last-wins rule, or a warning or prompt when the search finds several candidates. The word "recursively" tells you sub-directories are searched, but not what happens when two of them hold a file that could serve.

**Label for these two parts: not in the sources.**

**A note on the `find_all` option.** Its description — "Find all files in the search path (not just missing)" — is the whole of what the sources say about it. What Blender does with non-missing files it finds, and whether that can rewrite a path that was not broken, is not stated anywhere I searched. I am deliberately not reading anything further into that one line.

### Where I looked (required for the "not in the sources" parts)

Searches were run over the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` at once, case-insensitively, for these terms:

- `find missing` — two hits only: the topbar manual page, and `bpy.ops.file.txt` in the API documentation. Both are quoted in full above.
- `missing` — across the whole manual this returns 40-odd hits. The only ones about broken external references are the two topbar entries above, the "Broken Libraries" paragraph in Link/Append, and `ID.is_missing` in the API. None describes a matching rule. The rest are about unrelated things (missing caustics, missing NLA, missing shadow updates, missing media in the sequencer, missing data layers).
- `search path` — hits are about Python script search paths and the OSL shader search path, not about this search.
- `recursively` — the only relevant hit is the Find Missing Files paragraph itself.
- `broken link` / `broken links` — the Find Missing Files paragraph, and the Relocate Library paragraph in Link/Append.
- `recovered` / `will be recovered` — only the two Find Missing Files sentences.
- `first match`, `ambiguous`, `same file name`, `same name`, `file name matches`, `matching file`, `file with the same`, `base name`, `filename`, `duplicate name`, `search directory`, `sub-directories`, `subdirectories` — no hit anywhere describes how a missing external file is matched to a candidate on disk. The `same name` and `file with the same` hits are about materials on import, USD texture copying, and Save As overwriting.
- `no longer exist`, `cannot find`, `could not be found`, `not found` — the only relevant hits are the Report Missing Files sentence, the Broken Libraries paragraph, and `ID.is_missing`.

Pages I opened and ruled out: the manual's File Paths page (https://docs.blender.org/manual/en/5.1/files/file_paths.html) is entirely about path templates such as `{blend_name}`, not about resolving broken paths. The manual's File Browser page describes recursion levels for browsing, not for this search. The whole `files/` section of the manual was listed and the only relevant pages are Link/Append and Packed Data. The 5.1 release notes contain nothing about missing-file search; the only path-related entries are bug fixes ("Crash on relocate linked file"; "External Multires .btx filepath not cleared when re-packing, reports as missing even when empty"; "Bpy.data.libraries.write with path_remap='RELATIVE_ALL' does not recalculate existing relative paths"), none of which describes matching. Cycles' own files handle paths only for OSL scripts (`cycles_5.1/osl.py`) and for its denoise operator's input and output paths (`cycles_5.1/operators.py`); neither searches for missing files.

The manual page that carries this material has three pictures. **Documented:** `_images/interface_window-system_topbar_menus.png`, on https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30. Visible in it: the top bar strip with the Blender icon and the File, Edit, Render, Window and Help menu names. The other two are of the Workspaces and Scene/View Layer selectors. None shows the External Data menu or the Find Missing Files operator panel, so no picture settles this. *(Citation check, 2026-08-09: labelled **documented** and put into the Method's form for a picture — which picture, the address of the page it sits on, and what is visible in it. I opened the same file and it shows exactly that.)*

**Not covered.** This answer does not settle: the matching rule; the behaviour with several candidates; whether the search reports what it changed; whether it can rewrite a path that was not broken (the `find_all` option); whether packed resources are skipped; whether library (linked blend-file) paths are covered by the same search or only image-style resources; and whether the search can undo a wrong recovery.

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

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**The question is ambiguous and I have to say so.** "A file Blender has already read" can mean two things: (a) an external file the open blend-file points at — an image, a video, a movie clip, a linked blend-file, a cache archive, a text or shader script; or (b) the blend-file itself. Given the questions around it on the sheet, all of which are about the things a blend-file points at, I have answered reading (a) as the main one, and covered (b) briefly at the end.

**Labels are given part by part below: documented where a manual or API quote carries the claim, observed for what Cycles' own files say, inferred where I had to supply a step and said so, and not in the sources for what a render itself does.**

### One page, about the Video Sequencer, says a change on disk can go undetected

The clearest statement that a change on disk can go unnoticed is in the Video Sequencer's View menu:

> Refresh All Ctrl-E
>
> Reloads external files and refreshes the current frame preview.
> This is useful when you modified an external file or made a change in a scene that Blender
> didn't detect.

- Source: manual
- Address: https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html
- Date read: 2026-07-30

The same wording appears on the Sequencer preview's header page (https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/header.html, read 2026-07-30).

### There is a separate reload command per kind of referenced file

Each of these is a manual entry with a Python equivalent.

**Images.**

> Image ‣ Reload
> Shortcut: Alt-R
> Reload the image from the file on drive.

- Source: manual — https://docs.blender.org/manual/en/5.1/editors/image/editing.html — read 2026-07-30

> Reload
>
> Reloads the file. Useful when it has been reworked in an external application.

- Source: manual — https://docs.blender.org/manual/en/5.1/editors/image/image_settings.html — read 2026-07-30

> `reload()`
>
> Reload the image from its source path

- Source: Python API documentation — https://docs.blender.org/api/5.1/bpy.types.Image.html — read 2026-07-30

> `bpy.ops.image.reload()`
>
> Reload current image from disk

- Source: Python API documentation — https://docs.blender.org/api/5.1/bpy.ops.image.html — read 2026-07-30

**Movie clips.**

> Reload Clip
>
> Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender.

- Source: manual — https://docs.blender.org/manual/en/5.1/movie_clip/tracking/clip/editing/clip.html — read 2026-07-30

The Python equivalent is `bpy.ops.clip.reload()`, "Reload clip" (https://docs.blender.org/api/5.1/bpy.ops.clip.html, read 2026-07-30).

**Sequencer strips.**

> Reload Strips Alt-R
>
> Reloads the strips from their external saved location.
>
> Reload Strips and Adjust Length Shift-Alt-R
>
> Reloads the strips from their external saved location and re-adjusts the strip duration.

- Source: manual — https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/editing.html — read 2026-07-30

The Python equivalent is `bpy.ops.sequencer.reload(*, adjust_length=False)`, "Reload strips in the sequencer" (https://docs.blender.org/api/5.1/bpy.ops.sequencer.html, read 2026-07-30).

**Linked blend-files (libraries).**

> When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file.

- Source: manual — https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30

The Python equivalent is `bpy.ops.wm.lib_reload(*, library='', ...)`, "Reload the given library", with `library` described as "Library, Library to reload" (https://docs.blender.org/api/5.1/bpy.ops.wm.html, read 2026-07-30).

**Cache archives (Alembic, USD).**

> `bpy.ops.cachefile.reload()`
>
> Update objects paths list with new data from the archive

- Source: Python API documentation — https://docs.blender.org/api/5.1/bpy.ops.cachefile.html — read 2026-07-30

**Text data-blocks — a documented case where Blender itself raises the change.**

> Resolve Conflict
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
> Hides the warning message until the external text file is modified externally again.

- Source: manual — https://docs.blender.org/manual/en/5.1/editors/text_editor.html — read 2026-07-30

**External OSL scripts used by Cycles.** The manual says the compile step is a command:

> Script Node Update
>
> Reloads the text file data-block, creating new inputs and outputs if needed.

and, for the External mode:

> Used to specify a .osl file from a drive,
> and this will then be automatically compiled into a .oso file in the same directory.

- Source: manual — https://docs.blender.org/manual/en/5.1/render/shader_nodes/utilities/script.html — read 2026-07-30

Cycles' own files show where that compile happens. In `Blender Sources/cycles_5.1/osl.py`, the function `update_external_script` (lines 192–226) is called only from `update_script_node` (line 280) and `update_custom_camera_shader` (line 336):

> ```
> def update_external_script(report, filepath, library):
>     """compile and update OSL script"""
> ```
> (`cycles_5.1/osl.py`, line 192)

> ```
>     elif script_ext == ".osl":
>         # compile .osl file
>         ok, oso_path = osl_compile(script_path, report)
> ```
> (`cycles_5.1/osl.py`, lines 205–207)

> ```
>     if script_ext == ".oso":
>         # it's a .oso file, no need to compile
>         ok, oso_path = True, script_path
> ```
> (`cycles_5.1/osl.py`, lines 202–204)

- Source: Cycles' own files — file `cycles_5.1/osl.py`, lines 192–226
- Date read: 2026-07-30
- Label: **observed** (this is what the file says)

The step beyond that — that an edit to a `.osl` file on disk therefore does not reach a render until `update_script_node` is run — is **inferred**, not observed. The step I took: the file shows compilation happening inside these named functions, and I recorded no other place in `cycles_5.1/osl.py` where it happens — that one file is the whole of what I quoted here — and the API documentation names the operator that reaches them, `bpy.ops.node.shader_script_update()`, "Update shader script node with new sockets and options from the script" (https://docs.blender.org/api/5.1/bpy.ops.node.html, read 2026-07-30). I have not seen anything stating what the renderer does when nobody runs it, and Cycles' compiled part is not among the files I was given.

### Reloading the whole blend-file does all of them at once

> Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once.

- Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30

And, for linked data specifically:

> Link creates a reference to data in a source file such that changes made there will be
> reflected in the current file the next time it is reloaded.

- Source: manual — https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30

A related automatic step is documented for library overrides. The manual ties it to a blend-file being **opened**, not to the reload command above — treating those two as the same event is the step I set out, and label inferred, under *Outside the interface* below.

> The relationships between linked data-blocks can change, resulting in outdated overrides.
> When this happens, overrides need to be resynced to match the new structure of their hierarchy.
> Overrides are automatically resynced if needed on blend-files opening.

- Source: manual — https://docs.blender.org/manual/en/5.1/files/linked_libraries/library_overrides.html — read 2026-07-30

The same page adds that where overrides come from an external library this work is repeated every load:

> However, this is a costly process that needs to be fully redone every time the working
> file is loaded, since Blender cannot edit/modify the external library directly.

- Same page, same date.

That automatic resync can be switched off from the command line:

> `--disable-liboverride-auto-resync`
>
> Do not perform library override automatic resync when loading a new blend-file.

- Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30

### If the resource is packed, changing the file on disk does nothing at all

> Pack Resources
>
> Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them.

- Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30

The same page states the general arrangement: "External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path."

Unpacking has its own wrinkle, and it is the only place I found where Blender is documented to compare a packed copy against the copy on disk:

> Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked.
> Note that it won't be automatically reloaded from the drive.
> (differs) or (identical) show difference status between the packed version
> and the one on-drive.

- Source: manual — https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html — read 2026-07-30

### What the sources do not say

I could not find any statement about:

- whether a render re-reads external files from disk at the moment it renders, or uses whatever was read earlier;
- what happens if an external file is changed or replaced while a render is already running;
- whether Blender checks modification times or sizes for anything other than external text data-blocks;
- what happens when the replacement differs in resolution, format, channel layout or length from the file that was read;
- what happens when the file is deleted rather than changed (only the missing-at-load case is covered, for libraries: "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones", https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html, read 2026-07-30).

Searches run over the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for: `reload`, `refresh`, `auto refresh`, `use_auto_refresh`, `modified externally`, `changed on disk`, `on disk`, `out of date`, `into memory`, `kept in memory`, `loaded in memory`, `unable to open`, `cannot find`, `could not be found`, `no longer exist`. The `auto refresh` hits are all about stepping through frames of a video or image sequence — "Always refresh image on frame changes" (https://docs.blender.org/api/5.1/bpy.types.ImageUser.html, read 2026-07-30), "Update the video texture in the 3D Viewport when moving through the timeline" (https://docs.blender.org/manual/en/5.1/render/shader_nodes/textures/image.html, read 2026-07-30) — not about a file being changed underneath Blender, so I have not treated them as answering this question. Nothing turned up on memory or caching of read files. The manual's command-line pages (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html and the Render page beside it, read 2026-07-30) say nothing about reloading external files or about missing ones; the only relevant argument on them is `--disable-liboverride-auto-resync`, quoted above. The 5.1 release notes contain no entry on this subject.

### The other reading: the blend-file itself changed on disk

The manual documents a command to take the file on disk again:

> Revert
>
> Reopens the current file to its last saved version.

- Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30

> `bpy.ops.wm.revert_mainfile(*, use_scripts=False)`
>
> Reload the saved file

- Source: Python API documentation — https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30

Note the wording: "its last saved version". The sources do not say what happens when the version on disk was written by something other than this Blender, and they do not say whether Blender notices at all that the blend-file it has open has changed underneath it. That part is **not in the sources**.

**Not covered.** This answer does not settle what a render does — only what commands exist and what they are said to do. It does not settle whether any of these reloads is needed for a render as opposed to a viewport, because no source I found distinguishes the two. It does not settle timing, mid-render replacement, or what a partially written file does.

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

---

## Things a person with Blender open could settle by hand

**All fourteen items that stood here have been run**, on 2026-08-09, in Blender 5.1.2 on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, when a file changes on disk, or when a render begins. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open, and not a matter for the machine.** Two things a fresh look at the sources has to settle before another test is worth writing: what `RELOAD_EDITED` does to `bpy.data.file_path_foreach`, whose real keyword this round did not have; and how `bpy.ops.wm.lib_reload()` is meant to be called, which failed with `Error: Not a library` both ways it was tried.

---

9 August 2026 — Opus 5 (Claude Code) — Reading a file, Blender 5.1

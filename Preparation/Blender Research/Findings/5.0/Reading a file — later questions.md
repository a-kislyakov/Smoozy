# Reading a file — later questions (Blender 5.0)

**Sources consulted, and when.** The Blender 5.0 manual, the Blender 5.0 Python API documentation, the Blender 5.0 release notes, and Cycles' own files, all as supplied in `Blender Sources/`. Snapshot date, and therefore my date read for everything below: **2026-07-30**.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project — with one exception I must declare: I ran a directory listing of `Preparation/Blender Research/Findings/` to confirm that the folder I was told to write into existed. That listing returned only four folder names (`4.5`, `5.0`, `5.1`, `5.2`). I opened no file there and read no content of this project. Nothing in that listing informed any answer below.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Reading taken.** I read "a search for a missing reference" as Blender's **Find Missing Files** command (`File ▸ External Data ▸ Find Missing Files`, `bpy.ops.file.find_missing_files`), which is the only search-for-a-missing-file feature I found in the 5.0 sources. The question is ambiguous: another reading would be **Relocate Library**, where a broken link to another blend-file is repaired. The sources describe Relocate as picking one specific replacement file by hand, and describe no search there, so nothing in them shows Relocate producing "more than one possible file" — that last step is mine, not the sources'. I say what the sources hold on it at the end of this answer anyway.

The question has three parts and the sources answer only one of them.

### (a) What makes a file a possible match — **not in the sources**

The manual describes where the search happens, but never what property of a found file makes it count as the missing one. No statement about matching by file name, by extension, by size, by anything.

What the manual does say about the search:

> "In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered."

Source: manual — <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>. Read 2026-07-30.

So: the area searched is one chosen directory plus every directory inside it, to any depth. "Every missing file found" is as close as the manual comes to a matching rule, and it states no rule.

The API documentation adds one option and no rule:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)"
>
> "Try to find missing external files"
>
> "find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.file.html>. Read 2026-07-30.

The quoted words say there is an option to find all files in the search path rather than only the missing ones. What the command then does with those extra files is not stated, and neither is what makes a file on disk answer to a given reference.

### (b) Which one is chosen when several match — **not in the sources**

Nothing in the manual, the API documentation, the release notes, or Cycles' files states a tie-break. There is no documented rule of "first found", "shallowest directory", "alphabetical", "newest", or "asks the user". The manual does not even acknowledge that more than one candidate can exist.

### (c) What location is written back — **documented**

> "Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual — <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>. Read 2026-07-30.

So the location written back into the file is an **absolute** path. Turning it back into a relative one is a separate, manual step (`Make Paths Relative`, described on the same page as "Make all paths to external files relative to the current blend-file.").

### On the other reading (Relocate Library)

**Documented** that Relocate replaces a linked blend-file with one the user selects. That it involves no search is **inferred** — the sources describe no search there, and the step from "not described" to "not done" is mine:

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

Source: manual — <https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html>. Read 2026-07-30.

The matching operation described in the API is likewise a single chosen path, not a search:

> "RELOCATE
> Relocate – Select a new path for this library, and reload all its data."

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.outliner.html>. Read 2026-07-30.

### Where I looked (required, because two parts of this answer are "not in the sources")

- Searched the whole of `manual_5.0` and `api_5.0` for: `missing`, `find missing files`, `recover`, `broken`, `relink`, `repath`, `re-link`, `search path`, `recursively`, `same name`, `first match`, `ambiguous`, `duplicate file`, `more than one`, `priority`, `alphabetical`, `shortest`, `conflict`, `checksum`.
- Searched all of `Blender Sources/` for `find missing files`: the only hits in 5.0 are the single topbar manual page quoted above.
- Read in full: the External Data section of <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>; <https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html>; <https://docs.blender.org/manual/en/5.0/files/file_paths.html>; the `file` and `outliner` operator pages of the API.
- Searched the 5.0 release notes (all subject files) for `missing`, `relocate`, `filepath`, `file path`, `reload`. Nothing about the matching rule. The only near hits are unrelated bug-fix list entries and one new API entry (`bpy.data.file_path_foreach`) on <https://developer.blender.org/docs/release_notes/5.0/python_api/>.
- Searched `cycles_5.0` for `missing`, `cache`, `persistent`. Cycles' files carry render settings; they contain nothing about finding missing external files.
- The topbar manual page has three pictures (`interface_window-system_topbar_menus.png`, `…_workspaces.png`, `…_scenes-layers.png`). None of them is the External Data menu, so no picture could settle the matching rule. I did not find a screenshot of the Find Missing Files file browser anywhere in `_images`.

**Not covered.** The command's remaining arguments, cut off as "…" in the quoted signature above, are not reported here, and one of them could bear on what counts as a match; whether matching is by file name at all; whether extension or case matters; what happens with two candidates of the same name in different subfolders; whether the user is prompted, warned, or told anything when a search is ambiguous; whether a search that finds nothing leaves the old path untouched or clears it; whether `find_all=True` rewrites paths that were not broken; whether packed files are skipped. None of this is in the 5.0 sources.

### Settled by hand test — observed, 2026-08-09

Eight tests were built for this question and run in **Blender 5.0.1** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 1 to 8.

**What was done, and to what.** A blend-file holding one image texture whose file was then deleted, reopened, and repaired by calling `bpy.ops.file.find_missing_files(directory=<folder>)` from a script. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, and no window was open at any point.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on all eight tests.**

**The command runs with no window at all.** Both machines printed `HT| s_same | operator returned = "{'FINISHED'}"`, and the recorded location changed.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension — `tex.jpg` for a missing `tex.png` — was not, and neither was a file of a different name holding the same picture. Both printed the same line as a folder holding nothing at all: `Warning: Could not find 'tex.png' in '<folder>'`.

**Letter case has to match, on both machines.** With only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it — `HT| s_upper | filepath after = '//orig/tex.png'` — while the control run with `tex.png` on disk recovered it on both. The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run. Wherever the candidates differed in size, the recorded location ended up pointing at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them: `HT| sA | filepath after = '//sA/zzz/tex.png'` against `HT| sB | filepath after = '//sB/aaa/tex.png'`, the 188-byte file winning both times over the 187-byte one. A file three folders down beat one at the top of the search folder on a single byte, and a 3908-byte file beat a 187-byte one from either position. Where two candidates were exactly the same size, `aaa` was taken rather than `zzz`.

> **The byte sizes measured again.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 28 of the hand-test list ran the tie-break arrangement once more and recorded the exact size of every candidate. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), this version wrote **188 bytes** for the red and blue 8 by 8 pictures, **187** for the green and white, **201** for the 16 by 16 one and **3908** for the 64 by 64 one. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), this version wrote the same four figures. Across the four versions: 4.5.12 LTS and 5.0.1 both wrote 188, 187, 201 and 3908; 5.1.2 and 5.2.0 LTS both wrote 261, 260, 274 and 3981. The choice came out the same at both sets of sizes and on both machines — the larger candidate won from either folder, and where the two candidates were byte-for-byte the same size `aaa` was taken. That bears on the disagreement between the two passes for this version: 188, 187 and 3908 is the shape of the figures the first pass carries, and the second pass's 3981 and 260 are what Blender 5.1.2 and 5.2.0 LTS wrote in this run. This is a measurement of this run's own pictures, so it says nothing about what either earlier run put on disk, and neither pass has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Nothing is said about there having been more than one.** No line about ambiguity appeared in any arrangement. The control is what makes that worth recording: with nothing to find, the same terminal carried `Warning: Could not find 'tex.png' in '<folder>'`, so this is a silence about ambiguity and not an unwatched channel.

**The location written back began `//`, not at the system root.** On both machines the recovered location came back as `//s_same/deep/tex.png`, with `HT| s_same | starts with // = True`. That is not what the manual sentence quoted above describes. What is recorded here is what appeared; nothing in the test separates the command's own behaviour from a preference that might turn the path relative afterwards.

**The recovery rewrites the location without loading the data.** Straight after a successful recovery, `HT| s_same | has_data = False`. Reading the pixels afterwards returned the new file's colour, so the file is read when it is next wanted rather than at the moment of the repair.

**How deep it searches.** A file eight folders below the chosen one was found, on both machines.

**Which kinds of recorded location it reaches: all six that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic archive had all six rewritten by a single run, on both machines — the linked blend-file included, although the manual gives libraries a separate repair tool of their own.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the sound, the font, the movie clip, the linked blend-file and the Alembic archive were made. Written out as a test for somebody's hands.

> **Run again, with the making of all six written out.** **Observed**, 2026-08-09, Blender 5.0.1. Entry 26 of the hand-test list rebuilt this test with every step named: an 8 by 8 red PNG written by Blender itself; a tenth of a second of silence written by Python's own `wave` module at one channel, two bytes a sample, 8000 samples a second; a font copied from whichever the machine has; a single-frame image sequence `clip_0001.png`; a blend-file holding one collection named `LibCol` with a default cube in it, linked and instanced; and an archive exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` and everything else left at the operator's own defaults. All six files were then moved into one search folder and the command was called once. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), the font taken was `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`, and all six recorded locations were rewritten to `//found/…`, every one of them beginning `//`. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), the font taken was `/System/Library/Fonts/Supplemental/Arial.ttf`, and the same six locations were rewritten, every one beginning `//`. The control was run on each machine as well: with the search folder empty, none of the six moved and the terminal carried six separate `Warning: Could not find '<name>' in '<folder>'` lines, one per kind. **This matches the result recorded above.** *(Note added by the hand tests, second batch, 2026-08-09.)*
>
> A further sentence stood at the end of this entry, saying that "the search does write absolute-form locations for every kind, and it writes them in the same relative form for all six". It contradicts itself, and it contradicts the observed entry below, which records the recovered location beginning `//`. Cut.
>
> *(Both notes added by the citation check, 2026-08-09.)*

**`find_all` rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a working reference moved too, from `//keep/good.png` to `//search/good.png`.

**A packed resource's recorded location is rewritten as well, and it stays packed.** `HT| after | p.png = ('//found/p.png', True)`.

**Whether a recovery can be undone is not settled.** With no window, `bpy.ops.ed.undo()` refused — `Operator bpy.ops.ed.undo.poll() failed, context is incorrect` — even after `bpy.ops.ed.undo_push()` had returned `{'FINISHED'}`, and again with a 3D Viewport context built by hand.

**Outside the interface.** **Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved, so the general warning about an operator's poll failing does not bite here. Everything above about matching, the tie-break and the form of the written-back location was established that way. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. So the adjacent fact about `bpy.path.resolve_ncase` does not describe this command. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

Short form: the 5.0 sources never state that Blender notices an external file changing on disk — except for text files, where they state it plainly. For everything else, what the sources document is a set of **explicit reload commands**, each described as the thing you use when the file was changed outside Blender. The reading I take from that — and it is my step, not a statement in the sources — is that reloading is a deliberate act rather than something that happens by itself.

### Blender does not automatically pick the change up — **inferred, plus one documented exception**

**Not in the sources (automatic detection, for images, movies, clips, caches, libraries).** No page in the 5.0 manual or API documentation says Blender re-checks an external file's contents, date, or size once it has been read. I searched for it specifically (see "where I looked").

**Documented (text files only).** For `Text` data-blocks Blender does detect on-disk change:

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

Source: manual — <https://docs.blender.org/manual/en/5.0/editors/text_editor.html>. Read 2026-07-30.

> "is_modified
>
> Text file on disk is different than the one in memory"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.types.Text.html>. Read 2026-07-30.

> "bpy.ops.text.resolve_conflict(*, resolution='IGNORE')
>
> When external text is out of sync, resolve the conflict
>
> resolution (enum in ['IGNORE', 'RELOAD', 'SAVE', 'MAKE_INTERNAL'], (optional)) – Resolution, How to solve conflict due to differences in internal and external text"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.text.html>. Read 2026-07-30.

**Documented (two loading commands carry settings about a path that is already loaded).** Each of the two below offers a setting that hands back the image already held for that path, and a separate setting that forces a fresh read. The quoted words do not say what either does when the settings are left alone, and they say nothing about any other route into Blender:

> "check_existing (bool) – If true, returns already loaded image data-block if possible (based on file path).
>
> force_reload (bool) – If true, force reloading of image (only useful when check_existing is also enabled)."

Source: Python API — <https://docs.blender.org/api/5.0/bpy_extras.image_utils.html>. Read 2026-07-30.

> "force_reload (boolean, (optional)) – Force Reload, Force reload the image if it is already opened elsewhere in Blender"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.image.html>. Read 2026-07-30.

### What has to happen — **documented, per kind of file**

Several kinds of external data have a reload command of their own. For images and for movie clips the manual ties that command to the file having been changed outside Blender. For the rest, the quoted words only say that the command reloads.

**Images**

> "Reload the image from the file on drive."

Source: manual — <https://docs.blender.org/manual/en/5.0/editors/image/editing.html>. Read 2026-07-30.

> "Reload
>
> Reloads the file. Useful when it has been reworked in an external application."

Source: manual — <https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html>. Read 2026-07-30.

> "bpy.ops.image.reload()
>
> Reload current image from disk"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.image.html>. Read 2026-07-30.

> "reload()
>
> Reload the image from its source path"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.types.Image.html>. Read 2026-07-30.

**Movie clips**

> "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender."

Source: manual — <https://docs.blender.org/manual/en/5.0/movie_clip/tracking/clip/editing/clip.html>. Read 2026-07-30.

**Video-sequencer strips**

> "Reload Strips Alt-R
>
> Reloads the strips from their external saved location.
>
> Reload Strips and Adjust Length Shift-Alt-R
>
> Reloads the strips from their external saved location and re-adjusts the strip duration."

Source: manual — <https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html>. Read 2026-07-30.

**Cache files (e.g. Alembic archives)**

> "bpy.ops.cachefile.reload()
>
> Update objects paths list with new data from the archive"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.cachefile.html>. Read 2026-07-30.

**Linked blend-files (libraries)**

> "Link creates a reference to data in a source file such that changes made there will be
> reflected in the current file the next time it is reloaded."

and

> "When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file."

Source: manual — <https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html>. Read 2026-07-30.

> "reload()
>
> Reload this library and all its linked data-blocks"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.types.Library.html>. Read 2026-07-30.

**The blend-file itself**

> "Revert
>
> Reopens the current file to its last saved version."

Source: manual — <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>. Read 2026-07-30.

> "bpy.ops.wm.revert_mainfile(*, use_scripts=False)
>
> Reload the saved file"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.ops.wm.html>. Read 2026-07-30.

**Everything at once.** The manual gives reopening the blend-file as the blanket way to refresh all external data:

> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual — <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>. Read 2026-07-30.

### Two cases where a change on disk is not picked up

**Packed data.** Once resources are packed into the blend-file, the external files are out of the picture:

> "Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them."

Source: manual — <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>. Read 2026-07-30.

**Unpacking by marking as unpacked.** Even after telling Blender to use the file on disk again, the quoted words say it is not reloaded on its own:

> "Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked.
> Note that it won't be automatically reloaded from the drive."

Source: manual — <https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html>. Read 2026-07-30.

### One render setting that bears on "a later render"

> "Persistent Data
>
> Keep render data in memory after rendering for faster re-renders and animation renders
> at the cost of extra memory usage while performing other tasks in Blender."

Source: manual — <https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html>. Read 2026-07-30.

> "use_persistent_data
>
> Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage"

Source: Python API — <https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html>. Read 2026-07-30.

**Inferred**, and the step is: the setting is documented as keeping render data in memory after a render so that later renders reuse it. If data prepared from a since-changed file is among what is kept, a later render in that same session would use the kept copy. The sources do **not** say what happens to persistent render data when the underlying data-block is reloaded, so this is a caution, not a finding.

### Answering the question directly

**Label: mixed.**
- That Blender offers an explicit reload for images, movie clips, sequencer strips, cache files, linked blend-files and the blend-file itself — **documented** (quotes above). That these exist for the case of the file being changed outside Blender is **documented** for images and movie clips only; for the rest it is my reading of what a reload is for, so **inferred**.
- That a text file changed on disk is detected and reported — **documented**.
- That a changed image / clip / strip / cache / library is *not* detected automatically — **inferred**. The step: the sources describe reloading as a command the user issues, describe it as "useful when it has been reworked in an external application", and nowhere describe any automatic check; and I searched for such a description and found none. A reader has to supply "no documented automatic check means no automatic check", so this is inference, not documentation.
- That after such a reload a later render uses the new content — **inferred**. The step: the reload commands are documented as replacing the data-block's contents from disk, and a render uses the data-blocks in the file; the sources do not join those two statements themselves.
- What has to happen before a later render uses the changed file — **inferred** from the above: either reload the specific data-block (Image ▸ Reload, Reload Clip, Reload Strips, `cachefile.reload`, library Reload), or reopen the blend-file (Revert, or `wm.revert_mainfile`, or opening it afresh), which the manual states reloads all external files at once. Nothing weaker is documented as sufficient. If the data is packed, no change on disk will ever be used until it is unpacked and reloaded.

**Not covered.** Whether a *new render job started while the Blender session is still open* re-reads anything on its own; whether Cycles or EEVEE hold their own copy of image data across renders beyond the Persistent Data setting; whether Auto Refresh re-reads from disk or only re-picks the frame — the sources describe it in terms of frames and playback, not disk. The two descriptions that rests on are: "Always refresh image on frame changes" (Python API, https://docs.blender.org/api/5.0/bpy.ops.image.html and https://docs.blender.org/api/5.0/bpy.types.ImageUser.html — read 2026-07-30) and "Update the video texture in the 3D Viewport when moving through the timeline" (manual, https://docs.blender.org/manual/en/5.0/render/shader_nodes/textures/image.html — read 2026-07-30). *(Citation check, 2026-08-09: those two descriptions were written down here with no address beside them and the document asked a checker to find their page. Found, at the addresses now given, and written in.)* Also not covered: whether any kind of external file beyond the six listed above has a reload of its own; whether replacing a file with one of different size, length or format is handled the same as an in-place edit; what happens if the file is deleted rather than changed; whether the sequencer/clip memory cache (governed by the "Memory Cache Limit" preference, <https://docs.blender.org/manual/en/5.0/editors/preferences/system.html>) keeps serving old frames after a reload.

### Settled by hand test — observed, 2026-08-09

Fifteen tests were built for this question and run in **Blender 5.0.1** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 9 to 23.

**What was done, and to what.** A scene whose only visible thing is a plane lit by an emission shader taking its colour from one image file, rendered by Cycles on the processor at eight pixels square with one sample and a Standard view transform, so a render can be read as a single pixel: red comes back as `[0.996, 0.0, 0.0, 1.0]` and blue as `[0.0, 0.0, 0.996, 1.0]`. Files were replaced by the operating system with `shutil.copyfile`, never by Blender, and every replacement was followed by two and a half clear seconds so that a modification time recorded only to the second is plainly newer. Pictures were compared as pictures and never as files. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, with no window open.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on every one of the fifteen tests.**

**An image changed on disk reaches the next render inside a running Blender, with nothing done.** Render one was `[0.996, 0.0, 0.0, 1.0]`; the file was overwritten with a blue one; render two, with nothing touched in between, was `[0.0, 0.0, 0.996, 1.0]`. So the inference recorded above — that a changed image is not detected automatically — does not hold for what a render then shows. `Image.reload()` ran without complaint and returned `None`; `bpy.ops.image.reload()` returned `{'CANCELLED'}` with no Image editor open and `{'FINISHED'}` with one built by hand *(Citation check, 2026-08-09: an Image editor built by hand is not enough on its own — on the repeat it returned `{'CANCELLED'}` until the image was also put into that editor, and only then `{'FINISHED'}`. Step added by this check.)*.

**A freshly started Blender reads the file as it stands.** A second Blender process opening the same blend-file after the swap rendered `[0.0, 0.0, 0.996, 1.0]` where the first had rendered `[0.996, 0.0, 0.0, 1.0]`.

**Persistent Data stops the change being picked up.** This settles the caution raised above. With `use_persistent_data` off, the second render showed the new picture. With it on, the second render showed the old one, and it still showed the old one after `Image.reload()`. The setting was present with a default of `False`.

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

**A file replaced while a render is already running takes effect from a later frame onward.** A twelve-frame animation had its image replaced part-way through: the frames up to a point held the old picture and every frame after it held the new one. No frame came out part old and part new.

**Revert takes the blend-file as it stands on disk.** With `scene.blend` opened, then replaced on disk by a different file, Blender still held the old contents until `bpy.ops.wm.revert_mainfile()` was run; it returned `{'FINISHED'}` with no window open and afterwards Blender held the replaced file's object, `['CUBE_B']`. A control Revert with nothing replaced gave `['CUBE_A']` back.

**Outside the interface.** **Observed**, 2026-08-09. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. Two refused in every arrangement tried: `bpy.ops.sequencer.reload()` and `bpy.ops.sequencer.refresh_all()`, and `bpy.ops.node.shader_script_update()`. The question the manual's command-line pages left open — when external files are read by a render — is answered by observation: for an image, an image sequence and an external OSL script the file on disk is read at the next render; for a linked blend-file, a movie clip, an Alembic archive and a packed image it is not. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.

### Where I looked

- Searched the whole of `manual_5.0` and `api_5.0` for: `reload`, `refresh`, `auto refresh`, `outside of blender`, `outside blender`, `external application`, `externally`, `edited outside`, `on disk`, `on drive`, `is_modified`, `is_dirty`, `timestamp`, `modification time`, `mtime`, `file watcher`, `monitor`, `checksum`, `persistent data`, `cache limit`. No page describes Blender polling or watching an external file, for any data type other than text.
- Read in full or in the relevant part: <https://docs.blender.org/manual/en/5.0/editors/image/editing.html>, <https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html>, <https://docs.blender.org/manual/en/5.0/movie_clip/tracking/clip/editing/clip.html>, <https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html>, <https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html>, <https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html>, <https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html>, <https://docs.blender.org/manual/en/5.0/editors/text_editor.html>, <https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html>, <https://docs.blender.org/manual/en/5.0/editors/preferences/system.html>, <https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html>.
- API pages read: `bpy.ops.image`, `bpy.ops.clip`, `bpy.ops.sequencer`, `bpy.ops.cachefile`, `bpy.ops.text`, `bpy.ops.wm`, `bpy.ops.outliner`, `bpy.types.Image`, `bpy.types.Text`, `bpy.types.Library`, `bpy.types.RenderSettings`, `bpy_extras.image_utils`, `info_gotchas_operators`, `info_gotchas_file_paths_and_encoding`.
- Searched the 5.0 release notes for `reload`, `filepath`, `file path`, `library`, `link`. Nothing changes reload behaviour; the only related new item is `bpy.data.file_path_foreach()` on <https://developer.blender.org/docs/release_notes/5.0/python_api/>, which is about visiting and rewriting paths, not about re-reading files.
- Searched `cycles_5.0` for `reload`, `cache`, `persistent`. The only cache mentions are unrelated (tile-to-disk memory saving, and the GPU kernel cache in `properties.py`). Cycles' files hold no reload or file-change setting.

---

## Things a person with Blender 5.0 open could settle in a few minutes

**All fifteen items that stood here have been run**, on 2026-08-09, in Blender 5.0.1 on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, when a file changes on disk, or when a render begins. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open, and not a matter for the machine.** Two things a fresh look at the sources has to settle before another test is worth writing: what `RELOAD_EDITED` does to `bpy.data.file_path_foreach`, whose real keyword this round did not have; and how `bpy.ops.wm.lib_reload()` is meant to be called, which failed with `Error: Not a library` both ways it was tried.

---

2026-08-09 — Opus 5 (Claude Code) — Reading a file, Blender 5.0

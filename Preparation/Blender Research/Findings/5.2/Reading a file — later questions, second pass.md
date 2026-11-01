# Reading a file — later questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2 release notes, and Cycles' own files for 5.2 — all as the plain-file copies inside `Blender Sources/`, taken on 2026-07-30. That snapshot date is my date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure, so the line above is exact. To confirm my output folder existed before writing, I listed the directory `Preparation/Blender Research/Findings/5.2/`. That showed me the file names in it, including the name of the earlier findings document I was told not to open. I did not open that document or any other file there, and a list of file names carries no statement about Blender. Nothing else in `Preparation/`, `Audits/`, `App/`, `README.md` or `CLAUDE.md` was opened, listed or searched. No helper agents were used; I did all the searching and reading myself.

Questions answered: 10 and 11 only.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**Reading taken.** I read this as being about Blender's *Find Missing Files* search — the one that takes a directory and hunts in it for files a blend-file refers to but cannot find. That is the only "search for a missing reference" in the sources. Ambiguity flagged: the question could also be read as being about the automatic search Blender does when it opens a file and a referenced file is not where the recorded location says. I found no such automatic search described anywhere in the sources, so that second reading has no answer either; I say what I looked for at the end of this answer.

**Label: split.** *Documented* for what location is written back. *Not in the sources* for what makes a file a possible match, and for which one is chosen when there is more than one.

### What is documented

The whole of what the manual says about this search is:

> "Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

— Blender manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30.

So, on the third part of the question — **what location is written back** — the answer is documented and plain: an **absolute** path. The manual says so outright, and adds that turning those back into relative paths is a separate, manual step (*Make Paths Relative*, on the same menu).

On the **scope of the search**, also documented: it is a directory you pick, searched recursively through every directory inside it.

The API documentation adds one setting the manual does not mention:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, check_existing=False, filter_blender=False, …)
>
> Try to find missing external files"

and, for that first setting:

> "find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.ops.file.html, read 2026-07-30.

That tells us the default behaviour only touches references that are missing, and that turning *Find All* on widens it to references that are not missing as well. It says nothing about how any file is matched.

The neighbouring menu entry, for completeness on what counts as "missing":

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

— same manual page and date. So "missing" means an unpacked external file whose recorded location does not exist.

### What is not in the sources

- **What makes a file a possible match.** Nothing in the manual, the API documentation, the release notes or Cycles' files states the test a found file has to pass. File name? Name and extension? Size? Nothing is said. The manual's phrase is only "Every missing file found in the search will be recovered", which assumes a notion of "found" it never defines.
- **Which one is chosen when more than one matches.** Nothing at all. No mention of a first match, a shallowest match, a preference for one directory over another, a warning, or a report of an ambiguity. The sources do not acknowledge that the situation can arise.

### Where I looked, and what I ruled out

Searches run across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` (2,360 manual pages, 2,191 API pages), case-insensitive:

- `missing file`, `find missing`, `find_missing`, `report missing`, `report_missing` — only three places in all four sources: the manual's topbar page (quoted above), `bpy.ops.file.find_missing_files` and `bpy.ops.file.report_missing_files` in the API (quoted above), plus the API index pages that merely list those two names.
- `missing` on its own — around 60 files. I opened the ones that could bear on file references: `files/linked_libraries/link_append`, `files/linked_libraries/library_overrides`, `bpy.types.ID`, `bpy.types.Library`, `release_notes_5.2/bugfixes.md`. None describes a search or a match rule. `bpy.types.ID.is_missing` is only a flag: "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)".
- `broken link` — three files. Two are the topbar page and `link_append`; the third is the API's contributing page, irrelevant.
- `relocate` — `link_append`, `linked_libraries/index`, `library_overrides`, `files/blend/open_save`, `bpy.ops.wm`, `bpy.ops.outliner`. These describe *Relocate Library* and *Relocate Linked ID*, where **you** pick the replacement file by hand; there is no search and so no matching rule. I ruled them out as answering a different question.
- `recover`, `recovered` — `recovered` appears only on the topbar page, in the sentence quoted. `recover` otherwise leads to crash recovery and auto-save, which is unrelated.
- Terms for the matching itself: `same name`, `file name`, `basename`, `first match`, `ambiguous`, `candidate`, `duplicate file`, `not found`, `cannot be found`, `could not be found`, `no longer exist`, `placeholder`, `search path`, `recursively`, `case-sensitive`, `case sensitive`. The only hit in a file-path context is `recursively` on the topbar page, already quoted. `basename` in the API is `bpy.path.basename` (a string helper) and a bone property — neither is about this search. `case sensitive` in a path context gives only `bpy.path.resolve_ncase`, a separate Python helper described as "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30) — that is a function a script may call itself, not a statement about what Find Missing Files does, so I did not use it as an answer.
- Release notes for 5.2: I grepped all 21 subject files for `missing`, `reload`, `relocate` and `path`. Nothing about missing-file searching, matching or path recovery. The only path-related 5.2 change is a new path iterator in the Python API and the Cycles texture cache, neither of which is this.
- Cycles' own files: grepped the whole of `cycles_5.2` for `missing`, `find`, `search path`. Cycles has no missing-file search of its own. Ruled out.
- The manual's pictures: the topbar page carries three images (`interface_window-system_topbar_menus.png`, `…_workspaces.png`, `…_scenes-layers.png`). None of them is a picture of the Find Missing Files browser or its options, so no picture could settle this.

**Not covered.** This answer does not settle what the match test actually is, nor what happens on a tie — it establishes only that the sources are silent on both. It also does not settle whether Find Missing Files reaches missing *linked blend-file libraries* as well as things like images and sounds: the manual speaks of "external files" and "broken links" and never enumerates the kinds, while it gives libraries their own separate repair tool (*Relocate Library*). And it says nothing about what happens to a reference the search does not find.

### Settled by hand test — observed, 2026-08-09

Eight tests were built for this question and run in **Blender 5.2.0 LTS** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 1 to 8.

**What was done, and to what.** A blend-file holding one image texture whose file was then deleted, reopened, and repaired by calling `bpy.ops.file.find_missing_files(directory=<folder>)` from a script. Every run had a fresh working folder, a fresh `BLENDER_USER_RESOURCES` folder and `--factory-startup`, and no window was open at any point.

**On what kind of computer.** **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2 — and **the rented Linux machine** — Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, no display server at all. **The two machines agreed on all eight tests.**

**The command runs with no window at all.** Both machines printed `HT| s_same | operator returned = "{'FINISHED'}"`, and the recorded location changed.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension — `tex.jpg` for a missing `tex.png` — was not, and neither was a file of a different name holding the same picture. Both printed the same line as a folder holding nothing at all: `Warning: Could not find 'tex.png' in '<folder>'`.

**Letter case has to match, on both machines.** With only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it — `HT| s_upper | filepath after = '//orig/tex.png'` — while the control run with `tex.png` on disk recovered it on both. The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run. Wherever the candidates differed in size, the recorded location ended up pointing at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them: `HT| sA | filepath after = '//sA/zzz/tex.png'` against `HT| sB | filepath after = '//sB/aaa/tex.png'`, the larger file winning both times. A file three folders down beat one at the top of the search folder on a single byte, and a 3981-byte file beat a 260-byte one from either position. Where two candidates were exactly the same size, `aaa` was taken rather than `zzz`.

> **The byte sizes measured again.** **Observed**, 2026-08-09, Blender 5.2.0 LTS. Entry 28 of the hand-test list ran the tie-break arrangement once more and recorded the exact size of every candidate. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), this version wrote **261 bytes** for the red and blue 8 by 8 pictures, **260** for the green and white, **274** for the 16 by 16 one and **3981** for the 64 by 64 one. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), this version wrote the same four figures. Across the four versions: 4.5.12 LTS and 5.0.1 both wrote 188, 187, 201 and 3908; 5.1.2 and 5.2.0 LTS both wrote 261, 260, 274 and 3981. The choice came out the same at both sets of sizes and on both machines — the larger candidate won from either folder, and where the two candidates were byte-for-byte the same size `aaa` was taken. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Nothing is said about there having been more than one.** No line about ambiguity appeared in any arrangement. The control is what makes that worth recording: with nothing to find, the same terminal carried `Warning: Could not find 'tex.png' in '<folder>'`, so this is a silence about ambiguity and not an unwatched channel.

**The location written back began `//`, not at the system root.** On both machines the recovered location came back as `//s_same/deep/tex.png`, with `HT| s_same | starts with // = True`. That is not what the manual sentence quoted above describes. What is recorded here is what appeared; nothing in the test separates the command's own behaviour from a preference that might turn the path relative afterwards.

**The recovery rewrites the location without loading the data.** Straight after a successful recovery, `HT| s_same | has_data = False`. Reading the pixels afterwards returned the new file's colour, so the file is read when it is next wanted rather than at the moment of the repair.

**How deep it searches.** A file eight folders below the chosen one was found, on both machines.

**Which kinds of recorded location it reaches: all six that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic archive had all six rewritten by a single run, on both machines — the linked blend-file included, although the manual gives libraries a separate repair tool of their own.

> **Not repeatable from this record.** The citation check could not follow this without inventing steps: the record does not say how the sound, the font, the movie clip, the linked blend-file and the Alembic archive were made. Written out as a test for somebody's hands. *(Note added by the citation check, 2026-08-09.)*

> **Run again, with the making of all six written out.** **Observed**, 2026-08-09, Blender 5.2.0 LTS. Entry 26 of the hand-test list rebuilt this test with every step named: an 8 by 8 red PNG written by Blender itself; a tenth of a second of silence written by Python's own `wave` module at one channel, two bytes a sample, 8000 samples a second; a font copied from whichever the machine has; a single-frame image sequence `clip_0001.png`; a blend-file holding one collection named `LibCol` with a default cube in it, linked and instanced; and an archive exported with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)` and everything else left at the operator's own defaults. All six files were then moved into one search folder and the command was called once. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all), the font taken was `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`, and all six recorded locations were rewritten to `//found/…`, every one of them beginning `//`. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2), the font taken was `/System/Library/Fonts/Supplemental/Arial.ttf`, and the same six locations were rewritten, every one beginning `//`. The control was run on each machine as well: with the search folder empty, none of the six moved and the terminal carried six separate `Warning: Could not find '<name>' in '<folder>'` lines, one per kind. **This matches the result recorded above.** *(Note added by the hand tests, second batch, 2026-08-09.)*

**`find_all` rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a working reference moved too, from `//keep/good.png` to `//search/good.png`.

**A packed resource's recorded location is rewritten as well, and it stays packed.** `HT| after | p.png = ('//found/p.png', True)`.

**Whether a recovery can be undone is not settled.** With no window, `bpy.ops.ed.undo()` refused — `Operator bpy.ops.ed.undo.poll() failed, context is incorrect` — even after `bpy.ops.ed.undo_push()` had returned `{'FINISHED'}`, and again with a 3D Viewport context built by hand. Neither machine can answer the version of that test which needs a window: the rented Linux machine has no display server and the Creator ran nothing this round.

**Outside the interface.** **Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved. Everything above about matching, the tie-break and the form of the written-back location was established that way, so all of it holds outside the interface. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. So the adjacent fact about `bpy.path.resolve_ncase` does not describe this command. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**Label: documented**, with two clearly-labelled inferences marked in place, and one part **not in the sources**.

**Reading taken.** "A file Blender has already read" = an external file a loaded blend-file points at — an image, a movie, a sound, a movie clip, a linked blend-file library, a text block, a geometry cache — and also the blend-file itself. I answer for all of those, because the sources treat them separately and the answer differs.

### The short of it

Blender holds what it has read and does **not** pick up a change on disk on its own. Something has to tell it to read the file again: a *Reload* on the data concerned, or reopening the blend-file. The one documented exception found in 5.2 is Cycles' texture cache, which is explicitly said to notice when the source image was modified and rebuild itself.

### What the sources say, source by source

**That Blender does not notice by itself.** The Video Sequencer's *Refresh All* is described as being for exactly this situation:

> "Refresh All Ctrl-E
>
> Reloads external files and refreshes the current frame preview.
> This is useful when you modified an external file or made a change in a scene that Blender
> didn't detect."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/navigating.html, read 2026-07-30. The same wording appears at https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/header.html. The Python name is `bpy.ops.sequencer.refresh_all()`, "Refresh the sequencer editor" (https://docs.blender.org/api/5.2/bpy.ops.sequencer.html).

**Why it does not notice, for images.** The API describes an image data-block as a wrapper around a cached copy:

> "The Image data-block is a shallow wrapper around image or video file(s)
> (on disk, as packed data, or generated).
>
> All actual data like the pixel buffer, size, resolution etc. is
> cached in an imbuf.types.ImBuf image buffer (or several buffers
> in some cases, like UDIM textures, multi-views, animations…)."

— https://docs.blender.org/api/5.2/bpy.types.Image.html, read 2026-07-30. The same page carries `has_data`, "True if the image data is loaded into memory (default False, readonly)".

*Inferred* (step written out): because the pixel data lives in a cached buffer rather than being read from the file each time it is used, a change to the file on disk cannot show up until that buffer is replaced — which is what *Reload* does. The sources say the data is cached and they say *Reload* exists and is for externally-reworked files; they do not join those two statements themselves, so I do.

**Reload, for each kind of thing.**

- *Images.* "Reload the image from the file on drive." — https://docs.blender.org/manual/en/5.2/editors/image/editing.html (Image ‣ Reload, shortcut Alt-R). And on the image's own panel: "Reloads the file. Useful when it has been reworked in an external application." — https://docs.blender.org/manual/en/5.2/editors/image/image_settings.html. In Python: `reload()`, "Reload the image from its source path" — https://docs.blender.org/api/5.2/bpy.types.Image.html. There is also the operator `bpy.ops.image.reload()`, "Reload current image from disk" — https://docs.blender.org/api/5.2/bpy.ops.image.html.
- *Movie clips.* "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender." — https://docs.blender.org/manual/en/5.2/movie_clip/tracking/clip/editing/clip.html. In Python: `bpy.ops.clip.reload()`, "Reload clip".
- *Sequencer strips.* "Reload Strips Alt-R / Reloads the strips from their external saved location." and "Reload Strips and Adjust Length Shift-Alt-R / Reloads the strips from their external saved location and re-adjusts the strip duration." — https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/editing.html. In Python: `bpy.ops.sequencer.reload(*, adjust_length=False)`, "Reload strips in the sequencer". The manual states the design reason plainly for both media kinds: "Movie strips reference the original media file rather than embedding it in the blend-file, allowing changes made in external editing or encoding software to be reflected by simply reloading the file." (https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/movie.html) and the same sentence for sound, "allowing changes made in external audio editing software to be reflected by simply reloading the file." (https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/sound.html).
- *Linked blend-file libraries.* "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded." And, without waiting for that: "When the Outliner is in the Blender File Display Mode, you can right-click a linked blend-file and choose Reload to immediately update the current blend-file with the latest version of the linked data-blocks, without having to reopen the file." — https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html. In Python: `Library.reload()`, "Reload this library and all its linked data-blocks" (https://docs.blender.org/api/5.2/bpy.types.Library.html), and the operator `bpy.ops.wm.lib_reload(library='', …)`, "Reload the given library" (https://docs.blender.org/api/5.2/bpy.ops.wm.html).
- *Geometry caches (Alembic, USD).* `bpy.ops.cachefile.reload()`, "Update objects paths list with new data from the archive" — https://docs.blender.org/api/5.2/bpy.ops.cachefile.html. The importer has a matching option: "Always Add Cache Reader / Add cache modifiers and constraints to imported objects even if they are not animated so that they can be updated when reloading the Alembic archive." — https://docs.blender.org/manual/en/5.2/files/import_export/alembic.html.
- *Text blocks.* `bpy.ops.text.reload()`, "Reload active text data-block from its file". Text is the one place the sources describe Blender actually **noticing** a mismatch: `bpy.ops.text.resolve_conflict(*, resolution='IGNORE')`, "When external text is out of sync, resolve the conflict", with "resolution (Literal['IGNORE', 'RELOAD', 'SAVE', 'MAKE_INTERNAL']) – Resolution, How to solve conflict due to differences in internal and external text" — https://docs.blender.org/api/5.2/bpy.ops.text.html. The manual's Text Editor page lists "Reload from Disk" and "Reload Alt-R / Reopens (reloads) the current buffer (all non-saved modifications are lost)." — https://docs.blender.org/manual/en/5.2/editors/text_editor.html.

**Reload everything at once, or reopen.** The manual's own advice, given in the context of files recovered by *Find Missing Files*:

> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

— https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30. Reopening the blend-file itself is *Revert*: "Reopens the current file to its last saved version." (same page); in Python, `bpy.ops.wm.revert_mainfile(*, use_scripts=False)`, "Reload the saved file" (https://docs.blender.org/api/5.2/bpy.ops.wm.html).

**One thing that does happen automatically on reopening: library override resync.**

> "The relationships between linked data-blocks can change, resulting in outdated overrides.
> When this happens, overrides need to be resynced to match the new structure of their hierarchy.
> Overrides are automatically resynced if needed on blend-files opening."

and

> "Blender is also able to resync library overrides from external libraries, that are then linked into a
> working file. However, this is a costly process that needs to be fully redone every time the working
> file is loaded, since Blender cannot edit/modify the external library directly."

— https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html, read 2026-07-30. So where a *changed* library has been overridden, opening the blend-file again does more than reread it: it also repairs the overrides against the library's new structure. That behaviour can be switched off from the command line: "--disable-liboverride-auto-resync / Do not perform library override automatic resync when loading a new blend-file." — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html.

**The documented exception: Cycles' texture cache (new in 5.2).** Here the sources state outright that a change on disk *is* noticed, and acted on at render time.

> "Auto Generate
>
> Automatically generate and update .tx files for images when rendering.
> Files are regenerated when the source image is modified."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html, read 2026-07-30.

The API states the same with the trigger spelled out:

> "use_auto_generate_texture_cache
>
> Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences (default False)"

— https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30. (The companion setting `use_texture_cache` is "Load texture tiles at appropriate resolution on demand to reduce memory usage… (default True)".)

And the 5.2 release notes, which are an explicit statement of change:

> "To get started, enable Performance > Texture Cache and Auto Generate in the render properties. For every image, a corresponding tx file will be generated in the `blender_tx/` folder next to the image file. This tx file optimized for loading just the tiles and resolutions needed for rendering. It is automatically updated when the image file is modified."

— Blender 5.2 release notes, https://developer.blender.org/docs/release_notes/5.2/cycles/, read 2026-07-30.

*Inferred* (step written out): the wording is about the `.tx` cache file being rebuilt from a modified source image, not about a Blender session that already has that image loaded in memory. So this documents that a stale `.tx` will not be used against a changed source; it does not say that an image already read into a running Blender is re-read. Those are two different caches and I keep them apart.

Cycles' own files, **observed**, show the same job can be done ahead of a render from the command line, without a person at the interface. In `Blender Sources/cycles_5.2/maketx.py`, the argument parser's description reads:

> ```
>             "Generate Cycles texture cache (.tx) files.\n"
>             "\n"
>             "When a file path is given, generate a tx file for that image.\n"
>             "When no file path is given, a blend file must be loaded and tx\n"
>             "files are generated for all images used by shader node trees."
> ```

— `Blender Sources/cycles_5.2/maketx.py`, in `argparse_create()`, lines 15–19; read 2026-07-30. The same file's `maketx_blend()` ends by calling `bpy.ops.render.generate_texture_cache()` (line 105). The API documents that operator as "Generate Cycles texture cache files for all images used in shader nodes" and offers `bpy.ops.render.clear_texture_cache()`, "Delete Cycles texture cache files from disk" (https://docs.blender.org/api/5.2/bpy.ops.render.html). The manual gives the invocations: `blender scene.blend --command maketx` and `blender --command maketx image.png --colorspace sRGB` (https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html).

**If the file was packed, changes on disk stop mattering at all.**

> "Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them."

— https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30. And when unpacking, choosing to keep the on-disk copy: "If the original file path still exists, mark it as unpacked. Note that it won't be automatically reloaded from the drive. (differs) or (identical) show difference status between the packed version and the one on-drive." — https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html.

### Where I looked, and what I ruled out

- Whole-source greps (manual, API, release notes, Cycles) for: `reload`, `refresh`, `revert`, `resync`, `timestamp`, `modification time`, `mtime`, `modified on disk`, `changed on disk`, `up to date`, `up-to-date`, `out of date`, `outdated`, `watch`, `detect`, `cache`, `packed`.
- `reload` across the manual gave about 50 hits; I opened every one that concerned external data and quoted the relevant ones above. The rest are about script reloading (`Reload Scripts`, `bpy.ops.script.reload()`), UI translation reloading, and unused-data purging — ruled out as not about a referenced file changing.
- `mtime`, `changed on disk`, `modified on disk` — **no hits anywhere** in the manual, API or release notes. `modification time` appears only as a File Browser sort option (`FILE_SORT_TIME`, "Modified Date – Sort files by modification time"), which is about listing files, not about noticing a change; ruled out.
- `outdated` — the useful hits are `RenderSettings.use_auto_generate_texture_cache` (quoted) and the library-override resync page (quoted). `editors/image/navigating` uses "outdated" only about editors not redrawing ("Update Automatically / … the other editors may display outdated information until they're manually refreshed"), which is a screen-redraw matter, not a disk matter; ruled out.
- `Auto Refresh` — three manual pages. All three define it as reacting to the **frame** changing, not to the file changing: "Play the video in the Image Editor when the scene animation is playing", "Update the video texture in the 3D Viewport when moving through the timeline", "Automatically updates images in the viewport when the frame changes"; and the API's `ImageUser.use_auto_refresh` is "Always refresh image on frame changes (default False)". Ruled out as a false lead — it is not a file-change watcher.
- The Asset Browser is a genuine place where the sources say Blender does track file changes: "Blender generates an index of all assets contained inside an asset library, and keeps it up-to-date as files are modified within it" (https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html). I ruled it out of the answer because it concerns an index used for browsing assets, and the sources do not connect it to what a render reads.
- 5.2 release notes, all 21 subject files, grepped for `reload`, `missing`, `path`, `cache`. The only relevant entry is the Cycles texture cache, quoted. `pipeline_io.md` has "Fix crash reloading scripts", which is about scripts; `bugfixes.md` has "Crash when freeing ImBuf cache of an image sequence with a missing image" and "Crash unloading blend file with missing linked sound" — crash fixes, not statements about reload behaviour; ruled out.
- Cycles' own files: grepped all 420 files for `mtime`, `modified`, `st_mtime`, `getmtime`. The only live hit is in `osl.py` line 241, a check on a script data-block (`script.is_in_memory or script.is_dirty or script.is_modified or not os.path.exists(osl_path)`) — that is about an OSL script text block, not about image files, so I did not build the answer on it. Everything else is licence text and comments about modified algorithms.
- I found nothing anywhere stating that starting a render re-reads external files from disk.

**Not covered.** Three real gaps.

1. **Whether a render itself refreshes anything.** No source says a render re-reads external files. So I cannot tell you whether a render of an already-loaded scene would pick up a changed image without a reload — I can only tell you that reload exists and is documented as the way to bring an externally-changed file in. The one place a render is documented to check the disk is the Cycles `.tx` cache.
2. **Replacement versus modification.** The sources speak of a file being "modified" or "reworked in an external application". They never distinguish a file overwritten in place from a file deleted and a new one written at the same location, and they never say what happens if the replacement has a different size, format or colour depth. Nor do they say what happens if the file is deleted outright while Blender holds it.
3. **A change made mid-render.** Nothing in the sources addresses a file changing while a render is already running.

### Settled by hand test — observed, 2026-08-09

Fifteen tests were built for this question and run in **Blender 5.2.0 LTS** on both of the project's machines. They are set out in full, with every command and every line the machines printed, in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`, entries 9 to 23.

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

> **Run again with the set-up written out, and it came out differently.** **Observed**, 2026-08-09, Blender 5.2.0 LTS. Entry 27 of the hand-test list names both missing steps. Two archives were exported, from a cube of size 2 and a cube of size 4, with `bpy.ops.wm.alembic_export(filepath=…, selected=False, start=1, end=1)`; the smaller was brought in with `bpy.ops.wm.alembic_import(filepath=…, as_background_job=False, always_add_cache_reader=True)`, which gives the object a Mesh Sequence Cache modifier and a Transform Cache constraint, both printed back from the file. **The importer's own defaults were run as an arm of their own**, on both machines and in all four versions. The object came back with `Cube modifiers = []` and `Cube constraints = []` — no cache reader at all — and measured -1 to 1 at every one of the six points, which is the reading above; but a freshly started Blender opening that file after the swap also measured -1 to 1, on both machines, and a fresh Blender cannot be showing a stale archive. In that same fresh process the replacement archive imported on its own measured -2 to 2, so the archive was fine and the arrangement was watching an object that nothing tied to it. With the cache reader attached, on the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): after the swap with nothing done, -1 to 1; `bpy.ops.cachefile.reload()` with no context `{'CANCELLED'}`, -1 to 1; the same command with an `edit_cachefile` context built by hand `{'FINISHED'}`, and the evaluated mesh then measured **-2 to 2**, dimensions `(4.0, 4.0, 4.0)`; it stayed at -2 to 2 after the recorded location was written back to itself and after the frame was stepped away and back. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same readings at all six points. The archives were 3094 bytes each on the Linux machine and 3119 bytes each on the Mac — the same size as one another, which is why the mesh had to be measured. The controls held on both machines: a freshly started Blender opening the same blend-file after the swap measured -2 to 2, and the replacement archive imported on its own measured -2 to 2, so it really did hold a cube of twice the size. The result above is left exactly as it stands. Both runs stand and neither has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Blender does notice a changed external text file, and the conflict can be resolved with no window.** `Text.is_modified` was `False` while the two copies agreed and `True` once the file on disk differed. The text Blender held did not change until a reload was run. `bpy.ops.text.reload()` refused with no Text editor and returned `{'FINISHED'}` with one built by hand; so did `bpy.ops.text.resolve_conflict(resolution='RELOAD')`, which brought the new text in.

**A packed image ignores the file on disk.** With the image packed, the render went on showing the old picture after the file was replaced, and `Image.reload()` — which raised no error — did not bring the disk file in either. The same swap with the image left unpacked did reach the render, which is what shows the swap bit.

> **Unsettled — the citation check's repeat disagreed.** Repeating this on both machines and in all four versions, with the image packed and the blend-file saved and reopened: render one red `[0.659, 0.012, 0.02, 1.0]`, render two after the swap still red, and then, after `Image.reload()`, render three **blue** `[0.004, 0.004, 0.506, 1.0]` — with the image still packed. The control with the image left unpacked went red then blue on both runs. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time.** **Observed**, 2026-08-09, Blender 5.2.0 LTS. Entry 24 of the hand-test list ran three arms: the image left unpacked, the image packed with the blend-file saved but *not* reopened, and the image packed with the blend-file saved *and* reopened — the one difference the two earlier records name. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): the unpacked control rendered `[0.996, 0.0, 0.0, 1.0]`, then `[0.0, 0.0, 0.996, 1.0]` after the swap, so the swap bit; packed and not reopened rendered red, red, then **`[0.0, 0.0, 0.996, 1.0]`** after `Image.reload()`, with `packed_file is not None` still `True`; packed and reopened gave the same three renders and stayed packed. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three arms and the same nine renders. The test PNG was 261 bytes in this version. `Image.reload()` returned `None` and raised nothing every time. **This third run matches the citation check's and not the result recorded above.** Whether the blend-file was reopened after packing made no difference on either machine, so that is not what separates the two earlier runs, and this run offers nothing in its place. All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**Cycles' texture cache exists in this version, and it is rebuilt when the source image changes.** `RenderSettings.use_texture_cache` was present with a default of `True` and `RenderSettings.use_auto_generate_texture_cache` with a default of `False`, and `generate_texture_cache` and `clear_texture_cache` were both in the real list of `bpy.ops.render` operators. With both settings on, a `.tx` file appeared in a `blender_tx/` folder next to the image after the first render. After the source image was overwritten, the second render's picture showed the new colour and the `.tx` file's modification time had moved on — on the rented Linux machine its size changed from 1597 to 1594 bytes as well; on the Creator's Mac the size stayed 1692 and only the time moved. That is the narrow claim the sources make, confirmed: the cache file is rebuilt from a modified source image at render time.

**A changed external OSL script reaches the next render with nothing done.** `ShaderNodeScript.use_auto_update` was present with a default of `False`. The first render with the script emitting red was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue and the very next render was `[0.0, 0.0, 0.996, 1.0]`. `bpy.ops.node.shader_script_update()` refused every time it was called, with and without a Shader node editor built by hand, so no update command ever succeeded — and the change reached the render anyway.

> **Unsettled — the citation check's repeat disagreed.** Repeating this — a Script node in External mode emitting red, the `.osl` on disk rewritten to emit blue, the very next render with nothing else done — gave: Creator's Mac 4.5 blue, 5.0 blue, 5.1 red, 5.2 red; rented Linux machine 4.5 red, 5.0 red, 5.1 red, 5.2 blue. The two machines did not agree with each other. The other two parts of this entry did repeat exactly: `use_auto_update` present with a default of `False`, and `bpy.ops.node.shader_script_update()` refusing in every arrangement. The result above is left exactly as it stands and neither run has been preferred. Back with the hand tests. *(Note added by the citation check, 2026-08-09.)*

> **Run a third time, with the compiled file watched and the clock measured.** **Observed**, 2026-08-09, Blender 5.2.0 LTS. Entry 25 of the hand-test list. On the rented Linux machine (Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, no display server at all): render 1 with the red `.osl` was `[0.996, 0.0, 0.0, 1.0]`; the `.osl` on disk was rewritten to emit blue; render 2, with nothing else done, was **`[0.0, 0.0, 0.996, 1.0]`**; and the compiled `t.oso` sitting beside the `.osl` went from 485 bytes to 496 during that very render, its modification time moving on with it. On the Creator's Mac (MacBook Pro, M1 Pro, macOS 26.5.2): the same three renders, and `t.oso` went from 769 bytes to 842 during render 2. On both machines `bpy.ops.node.shader_script_update()` raised `Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`, with and without a Shader node editor built by hand, so no update command ever succeeded. **The clock was measured rather than assumed.** On the Linux machine two writes a hundredth of a second apart were given recorded times 13,999,676 nanoseconds apart, and the `.osl`'s own recorded time moved from `1786297320049651823` to `1786297323076581661` at the rewrite. On the Mac the same two writes were 10,883,709 nanoseconds apart, and the `.osl`'s recorded time moved from `1786297442957588759` to `1786297446071435297`. Both machines record a modification time to the nanosecond, and nothing in this run was hidden inside one tick of it. Two further controls held on both machines: a genuinely separate Blender process opening the saved blend-file rendered blue, and rendered blue again after every `.oso` had been deleted, so blue was reachable. **This third run matches the result recorded above and not the citation check's.** All three runs stand and none has been preferred. *(Note added by the hand tests, second batch, 2026-08-09.)*

**`bpy.data.file_path_foreach` exists in this version.** `HT| bpy.data.file_path_foreach present = True`. Every call made to it errored, because the keyword this round used was wrong: `TypeError("file_path_foreach() got an unexpected keyword argument 'flag'. Did you mean 'flags'?")`. The error is recorded rather than repaired: this settles that the function is there, and names the spelling a later test should use, and does not settle what `RELOAD_EDITED` does.

**A file replaced while a render is already running takes effect from a later frame onward.** A twelve-frame animation had its image replaced part-way through: the frames up to a point held the old picture and every frame after it held the new one. No frame came out part old and part new. Which frame the change lands on moves from run to run with the timing.

**Revert takes the blend-file as it stands on disk.** With `scene.blend` opened, then replaced on disk by a different file, Blender still held the old contents until `bpy.ops.wm.revert_mainfile()` was run; it returned `{'FINISHED'}` with no window open and afterwards Blender held the replaced file's object, `['CUBE_B']`. A control Revert with nothing replaced gave `['CUBE_A']` back.

**Outside the interface.** **Observed**, 2026-08-09. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. Three refused in every arrangement tried: `bpy.ops.sequencer.reload()`, `bpy.ops.sequencer.refresh_all()` and `bpy.ops.node.shader_script_update()`. What a render re-reads is now answered by observation rather than left open: for an image, an image sequence and an external OSL script the file on disk is read at the next render; for a linked blend-file, a movie clip, an Alembic archive, a Video Sequencer strip and a packed image it is not. The one thing that plainly still needs a window is the warning banner in the Text editor; the flag behind it, `Text.is_modified`, is readable from a script and does become `True`. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

**Operating system.** **Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

**All sixteen items that stood here have been run**, on 2026-08-09, in Blender 5.2.0 LTS on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, when a file changes on disk, or when a render begins. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open, and not a matter for the machine.** Two things a fresh look at the sources has to settle before another test is worth writing: what `RELOAD_EDITED` does to `bpy.data.file_path_foreach`, whose real keyword this round did not have; and how `bpy.ops.wm.lib_reload()` is meant to be called, which failed with `Error: Not a library` both ways it was tried.

---

2026-08-09 — Opus 5 (Claude Code) — Reading a file, Blender 5.2

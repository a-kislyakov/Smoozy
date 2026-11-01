# Reading a file — later questions (Blender 5.2)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- Blender 5.2 manual
- Blender 5.2 Python API documentation
- Blender 5.2 release notes
- Cycles' own files as shipped with Blender 5.2

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: **10** and **11**. The rest of the sheet was left alone.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

The question has three parts and they do not all land the same way. I answer them separately.

### 10a. What makes a file a possible match

**Label: not in the sources.**

The manual describes the search itself but never states the rule by which a file found in the search is judged to correspond to a particular missing reference.

What the manual does say about the search:

> "In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered."

- Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (section *External Data* ▸ *Find Missing Files*)
- Date read: 2026-07-30

So it is documented that the area searched is one chosen directory and, recursively, the directories inside it. The quoted words do not say how deep that goes. It is not documented what makes a candidate file inside that area count as the missing file.

One related documented detail, from the Python API entry for the same command. The command has an option called *Find All*:

> "find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)"

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.file.html (`bpy.ops.file.find_missing_files`)
- Date read: 2026-07-30

That tells you the search normally confines itself to references that are currently missing, and that it can be widened to every reference. It still does not say what counts as a match.

**A step I can take, clearly marked.** **Label: inferred.** The wording "every missing file found in the search" treats a missing reference as something that can be *found* in a directory tree that is not the one recorded in the file. The recorded path has two parts, the directory and the file name; the directory is by definition the part that is now wrong. So the file name is the only recorded thing left that a candidate could be compared against. The step the reader has to supply is that Blender compares names rather than, say, file contents or size — the sources never say so. Treat this as a hypothesis, not a finding.

Also worth recording, and worth not over-reading: the same Python API entry lists a long set of file-type filter options (`filter_image`, `filter_movie`, `filter_sound`, `filter_font`, `filter_blender`, and so on), each shown as `False` by default. Their exact words are not quoted in this document, so treat that as a pointer for a checker rather than a quoted finding. Nothing in the sources says what part, if any, those options play in the recursive search, so I draw nothing from them.

### 10b. Which one is chosen when several are possible

**Label: not in the sources.**

Nothing in the manual, the Python API documentation, the release notes or Cycles' own files describes what happens when more than one candidate file is found — no first-wins rule, no shallowest-path rule, no warning, no prompt. The manual sentence "Every missing file found in the search will be recovered" is written as if the outcome were never in doubt, and the question of two candidates is simply not raised.

### 10c. What location is written back

**Label: documented.**

> "Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

- Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (section *External Data* ▸ *Find Missing Files*)
- Date read: 2026-07-30

So the location written into the file is an absolute path — a full path from the system's root. The manual describes the companion command that converts them afterwards:

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file."

and

> "Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

- Source: manual, same page and section
- Date read: 2026-07-30

The manual's general description of relative paths, on a different page, confirms what the relative form means:

> "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

- Source: manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html (section *Relative Paths*)
- Date read: 2026-07-30

### Where I looked (required for the "not in the sources" parts)

Searches were run across the whole of the 5.2 manual, the whole of the 5.2 Python API documentation, and the whole of the 5.2 release notes, for these terms: `find missing files`, `find_missing_files`, `missing file`, `missing files`, `missing external`, `broken link`, `file not found`, `relocate`, `find all`, `find_all`, `search path`, `same file name`, `identical name`, `first match`, `first file`, `ambiguous`, `more than one file`, `several files`, `multiple matches`, `matching file`, `file name match`, `case-insensitive`, `case sensitive`, `blend_paths`.

Only four files anywhere mention the command at all: the manual's topbar page, the Python API pages `bpy.ops.file` and its two index pages. All of them were read in full at the relevant place. I also read the manual's *Linked Libraries* pages (link/append, library overrides, index) and the *Files* section pages (`file_paths`, `open_save`, `packed_data`, `data_blocks`), and the Python API entries for every operator in `bpy.ops.file` — none of them touch on the match rule.

I checked the manual's picture folder for a screenshot of the Find Missing Files browser (searching the `_images` folder for names containing `missing` and `find`); there is none, and the topbar page carries no picture at that point. So no picture could settle it either.

Cycles' own files were searched too, and nothing in them mentions this command or the search it performs.

### Not covered

This answer does not settle: what Blender compares to decide a file is the one being looked for; what happens with two or more candidates; whether the search follows shortcuts or links between directories; whether the recursion has a depth limit; whether it distinguishes upper and lower case in names; whether a partly successful run reports which references it could not recover; or whether packed data is skipped. All of these are simply absent from the sources.

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

### Outside the interface

**Observed**, 2026-08-09. The command runs with no window at all. Called as `bpy.ops.file.find_missing_files(directory=<folder>)` from a script in a Blender started with `-b` and `--factory-startup`, it returned `{'FINISHED'}` and rewrote the recorded location, on the Creator's Mac and on the rented Linux machine, which has no display server whatever. The directory is given as an argument and no File Browser is involved. Everything above about matching, the tie-break and the form of the written-back location was established that way, so all of it holds outside the interface. What is *not* settled outside the interface is undo: `bpy.ops.ed.undo()` refuses in a Blender with no window. Two more tests were run this way in a second batch on 2026-08-09 — entry 26 of the hand-test list, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. Both ran with no window on both machines and neither needed one, so nothing here is narrowed.

### Operating system

**Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the eight tests was run on both machines and **the two agreed on all eight**, including the one place a difference was most expected: with only `TEX.PNG` on disk and `tex.png` recorded, neither machine recovered it, even though the Creator's Mac has a file system that does not treat capital letters in a file name as significant and the rented Linux machine has one that does. So the adjacent fact about `bpy.path.resolve_ncase` does not describe this command. Windows was not tried — no Windows machine has run it — so that operating system stays untested rather than answered. A second batch was run on 2026-08-09: entry 26, the six kinds of recorded location, and entry 28, the byte sizes behind the tie-break. **The two machines agreed on both** — the same six rewrites, all beginning `//`, and the same byte size for every picture. Windows is still untested and was still not tried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

The sheet's question is ambiguous in one way, so I say plainly which reading I took. I read "a file Blender has already read" as an external file the blend-file points at — an image, a movie, a sound, a linked blend-file, a text file. The other reading is the blend-file itself being changed on disk under a running Blender; I say what the sources hold on that at the end of this answer.

### What happens: the sources describe reloading as something that is asked for

**Label: inferred**, for the general claim that nothing happens on its own to image, movie, sound or library data.

Two different things stand behind that claim. **Documented**: for particular cases the sources say outright that no reload happens by itself, and those quotes follow. **Not in the sources**: no page I opened describes Blender noticing a changed external file of those kinds on its own. The step a reader has to supply is that those particular statements, plus the existence of a set of commands whose purpose is to re-read from disk, cover every case of those kinds. The sources never say that, so this is a reading rather than a reading-off.

The plainest of the documented statements, from the Video Sequencer:

> "Refresh All Ctrl-E
>
> Reloads external files and refreshes the current frame preview.
> This is useful when you modified an external file or made a change in a scene that Blender
> didn't detect."

- Source: manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/navigating.html
- Date read: 2026-07-30

And from unpacking, where Blender knows perfectly well that the file on disk differs from what it holds:

> "Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked.
> Note that it won't be automatically reloaded from the drive.
> (differs) or (identical) show difference status between the packed version
> and the one on-drive."

- Source: manual, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html (section *Selective Unpacking* ▸ *Options*)
- Date read: 2026-07-30

The per-kind reload commands, each documented:

**Images.**

> "Reload the image from the file on drive."

- Source: manual, https://docs.blender.org/manual/en/5.2/editors/image/editing.html (Image ▸ Reload, shortcut Alt-R)
- Date read: 2026-07-30

> "Reloads the file. Useful when it has been reworked in an external application."

- Source: manual, https://docs.blender.org/manual/en/5.2/editors/image/image_settings.html
- Date read: 2026-07-30

The manual's texture painting page walks the same loop as a matter of course: edit in another program, "re-save it within that program", then "Back in Blender, you reload the image in the Image Editor."

- Source: manual, https://docs.blender.org/manual/en/5.2/sculpt_paint/texture_paint/introduction.html
- Date read: 2026-07-30

**Movie clips.**

> "Force reload the currently loaded movie clip. Is mainly useful when the clip gets edited outside of Blender."

- Source: manual, https://docs.blender.org/manual/en/5.2/movie_clip/tracking/clip/editing/clip.html (*Reload Clip*)
- Date read: 2026-07-30

**Video Sequencer strips.**

> "Reload Strips Alt-R
>
> Reloads the strips from their external saved location."

and

> "Reload Strips and Adjust Length Shift-Alt-R
>
> Reloads the strips from their external saved location and re-adjusts the strip duration."

- Source: manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/editing.html (*Inputs*)
- Date read: 2026-07-30

The manual states the same expectation for the media themselves:

> "Movie strips reference the original media file rather than embedding it in the
> blend-file, allowing changes made in external editing or encoding software to
> be reflected by simply reloading the file."

- Source: manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/movie.html
- Date read: 2026-07-30

The sound strips page carries a sentence to the same effect:

> "Sound strips reference the original media rather than embedding it in the
> blend-file, allowing changes made in external audio editing software to be
> reflected by simply reloading the file."

- Source: manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/sound.html
- Date read: 2026-07-30

*(Citation check, 2026-08-09: this document named the page but said its exact words were not quoted and that nothing here rested on it. Found at that address and written in.)*

**Linked blend-files.**

> "Link creates a reference to data in a source file such that changes made there will be
> reflected in the current file the next time it is reloaded."

and

> "When the Outliner is in the Blender File Display Mode,
> you can right-click a linked blend-file and choose Reload to immediately update
> the current blend-file with the latest version of the linked data-blocks,
> without having to reopen the file."

- Source: manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (*Link*, and *Reload Library*)
- Date read: 2026-07-30

If the linked file was not merely changed but moved or renamed, the manual points at a different command:

> "It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

- Source: manual, same page (*Relocate Library*)
- Date read: 2026-07-30

**Text files** are the one kind where the sources say Blender does notice:

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

- Source: manual, https://docs.blender.org/manual/en/5.2/editors/text_editor.html
- Date read: 2026-07-30

**Label for the text case: documented.** A warning message is tied to an external text file being changed by another program, and the person then chooses what to do about it. This is the only kind of external file where the sources describe a warning being shown to the person. One other documented case sits below — the Cycles texture cache — and that one is about a file being rebuilt, not about a person being told.

### What has to happen before a later render uses the changed file

**Label: documented, for the reload route.** The manual states outright that reloading the blend-file reloads all external files at once:

> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."

- Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (note under *Find Missing Files*)
- Date read: 2026-07-30

Reopening the current file is a command in its own right:

> "Revert
>
> Reopens the current file to its last saved version."

- Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (File menu)
- Date read: 2026-07-30

So, by the sources: either the affected data-block is reloaded individually with the command for its kind, or the whole blend-file is loaded again, which reloads every external file.

**Label: inferred**, for a render started in a freshly launched Blender. The manual says loading the blend-file again reloads all external files. The step the reader must supply is that a new Blender process opening that blend-file is a load in the same sense as the reload the manual describes. The sources never state this. If it holds, a render started by launching Blender afresh reads whatever is on disk at that moment, and nothing more need be done.

**Label: not in the sources**, for whether starting a render inside an already-running Blender re-reads external files. I found no statement anywhere that rendering re-reads image, movie, sound or library files from disk, and none that it does not.

### Two documented cases that behave differently

**Library overrides.** If the linked file changed in a way that rearranged its data:

> "The relationships between linked data-blocks can change, resulting in outdated overrides.
> When this happens, overrides need to be resynced to match the new structure of their hierarchy.
> Overrides are automatically resynced if needed on blend-files opening. However,
> it may be needed to resynced them manually sometimes"

- Source: manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html (*Resyncing Overrides*)
- Date read: 2026-07-30

Blender also offers a command-line switch to turn that off: "--disable-liboverride-auto-resync — Do not perform library override automatic resync when loading a new blend-file."

- Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (*Other Options*)
- Date read: 2026-07-30

**Cycles' texture cache.** This is the one place I found where the sources say something is rebuilt by itself when a file on disk changes. Blender 5.2 keeps `.tx` copies of image textures for Cycles, and:

> "Auto Generate
>
> Automatically generate and update .tx files for images when rendering.
> Files are regenerated when the source image is modified."

- Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html (*Memory* ▸ *Texture Cache*)
- Date read: 2026-07-30

The release notes say the same:

> "For every image, a corresponding tx file will be generated in the `blender_tx/` folder next to the image file. This tx file optimized for loading just the tiles and resolutions needed for rendering. It is automatically updated when the image file is modified."

- Source: release notes, https://developer.blender.org/docs/release_notes/5.2/cycles/ (*Texture Cache*)
- Date read: 2026-07-30

and the Python API states the condition it checks:

> "Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences (default False)"

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (`use_auto_generate_texture_cache`)
- Date read: 2026-07-30

Note the default is off. Read this narrowly: what is documented is that the `.tx` cache file is rebuilt from a modified source image at render time. Whether that also means the render's *result* reflects the modified image inside an already-running Blender that has the old image in memory is a further step the sources do not take, and I do not take it either.

**Label: observed**, for the settings that exist. Cycles' own files carry the texture-cache options, with their kinds and defaults, for example:

> `debug_use_texture_cache_eviction: BoolProperty(`
> `    name="Cache Eviction",`
> `    description="Evict unused tiles from the texture cache to free up memory",`
> `    default=True)`

- File: `cycles_5.2/properties.py`, lines 1129–1132 (within the Cycles scene settings class)
- Date read: 2026-07-30

and Cycles' own files carry a command whose own description says it generates those cache files:

> `"Generate Cycles texture cache (.tx) files.\n"`
> `"\n"`
> `"When a file path is given, generate a tx file for that image.\n"`
> `"When no file path is given, a blend file must be loaded and tx\n"`
> `"files are generated for all images used by shader node trees."`

- File: `cycles_5.2/maketx.py`, in `argparse_create()`, the parser description
- Date read: 2026-07-30

Read that narrowly. It establishes that the command sits in Cycles' files and what its own description says, not what running it does. The manual page cited just above also shows it being called from the command line, in these two forms:

> `blender scene.blend --command maketx`

and

> `blender --command maketx image.png --colorspace sRGB`

- Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html (*Memory* ▸ *Texture Cache*)
- Date read: 2026-07-30

*(Citation check, 2026-08-09: these two forms were named but not quoted, and were left as a pointer for a checker. Found on that page and written in.)*

### Packing, which removes the question

**Label: documented.** If the external files were packed into the blend-file, changes to them on disk stop mattering:

> "Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used – any changes in them will no longer be
> reflected in the blend-file, and you are free to move or delete them."

- Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (*External Data* ▸ *Pack Resources*)
- Date read: 2026-07-30

### The other reading of the question

If "a file Blender has already read" means the blend-file itself being changed or replaced on disk while Blender holds it open, the sources say nothing about detection at all. The only thing documented is the deliberate act of reopening it: "Revert — Reopens the current file to its last saved version" (cited above). **Label for that reading: not in the sources.**

### Not covered

This answer does not settle: whether a render started inside a running Blender re-reads any external file; whether image data already loaded is held in memory across a render and for how long; whether reloading one image also reloads other data derived from it; whether a file replaced while a render is already under way is read partly old and partly new; whether the automatic detection for text files extends to any other kind of file; what happens if the changed file is unreadable or half-written when the reload happens; and, for the texture cache, what Blender does if the `.tx` file cannot be written.

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

### Outside the interface

**Observed**, 2026-08-09. Everything above was run in a Blender started with `-b` and `--factory-startup` on a machine with no display server whatever. `Image.reload()`, `Library.reload()` and `bpy.ops.wm.revert_mainfile()` all ran with no window. The operator forms `bpy.ops.image.reload()`, `bpy.ops.clip.reload()`, `bpy.ops.cachefile.reload()`, `bpy.ops.text.reload()` and `bpy.ops.text.resolve_conflict()` all refused with no editor of the right kind and all succeeded when one was built by hand out of the areas a windowless Blender still keeps — `['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']`, whose type can be changed from a script. Three refused in every arrangement tried: `bpy.ops.sequencer.reload()`, `bpy.ops.sequencer.refresh_all()` and `bpy.ops.node.shader_script_update()`. What a render re-reads is now answered by observation rather than left open: for an image, an image sequence and an external OSL script the file on disk is read at the next render; for a linked blend-file, a movie clip, an Alembic archive, a Video Sequencer strip and a packed image it is not. The one thing that plainly still needs a window is the warning banner in the Text editor; the flag behind it, `Text.is_modified`, is readable from a script and does become `True`. Three more tests were run this way in a second batch on 2026-08-09 — entries 24, 25 and 27 of the hand-test list — and all three ran with no window on both machines. Entry 27 sharpens one line above: `bpy.ops.cachefile.reload()` returned `{'CANCELLED'}` with no context and `{'FINISHED'}` with an `edit_cachefile` context built by hand, and where the object carried a Mesh Sequence Cache modifier it then re-read the archive. `bpy.ops.node.shader_script_update()` refused again in every arrangement of entry 25, which keeps it the one command that has never succeeded here.

### Operating system

**Observed**, 2026-08-09, for the parts the tests reached, and **not in the sources** otherwise. Every one of the fifteen tests was run on both machines and **the two agreed on all fifteen** — the same renders, the same operator return values, the same errors word for word. Windows was not tried — no Windows machine has run it — so what happens there, and in particular whether a file can be replaced at all while Blender holds it open, stays untested rather than answered. A second batch was run on 2026-08-09: entries 24, 25 and 27. **The two machines agreed on all three** — the same renders, the same measurements, the same operator return values and the same errors word for word. Two figures do differ between the machines and neither changed an answer: the compiled `.oso`'s byte size, and an Alembic archive's byte size. Windows is still untested and was still not tried.
---

## Things a person with Blender open could settle by hand

**All ten items that stood here have been run**, on 2026-08-09, in Blender 5.2.0 LTS on the Creator's Mac and on the rented Linux machine. Their results are written into the two answers above as observed findings, and the tests themselves — with every command and every line the machines printed — are in `Audits/Blender Research/Reading a file/Hand tests — later questions.md`. The list has been pruned to what is genuinely still open. A second batch of five tests was run on 2026-08-09 — the ones the citation check sent back, entries 24 to 28 of the same list — and it took nothing further off this list: every item left here needs either a window on screen, a Windows computer, or a fresh look at the sources, and none of the five reached any of those.

**Needs a person at a Blender with a window on screen.** Neither machine can do these: the rented Linux machine has no display server and the Creator ran nothing this round.

1. **Whether undo reverses a recovery by Find Missing Files.** With no window `bpy.ops.ed.undo()` refuses outright, even after `bpy.ops.ed.undo_push()` succeeds, so this is untested rather than answered.
2. **Whether anything appears in the Info editor or the status bar** when a search finds several candidates, when a file changes on disk, or when a render begins. The terminal was watched, with a control showing it does carry a message when there is one, and it said nothing about ambiguity; what a panel shows is a separate question.
3. **Whether the changed-text warning banner is drawn** in the Text editor. That `Text.is_modified` becomes `True`, and that the conflict can be resolved from a script with no window, are both settled above.

**Not run at all.** Anything about Windows — no Windows machine has run it. In particular, whether the search matches a differently-cased file name on a Windows file system, and whether a file can be replaced on disk while Blender holds it open.

**Still open, and not a matter for the machine.** Two things a fresh look at the sources has to settle before another test is worth writing: what `RELOAD_EDITED` does to `bpy.data.file_path_foreach`, whose real keyword this round did not have; and how `bpy.ops.wm.lib_reload()` is meant to be called, which failed with `Error: Not a library` both ways it was tried.

---

2026-08-09 — Opus 5 (Claude Code) — Reading a file, Blender 5.2

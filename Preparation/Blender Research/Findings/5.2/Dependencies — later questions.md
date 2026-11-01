# Dependencies — later questions (Blender 5.2)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- The Blender 5.2 manual (`https://docs.blender.org/manual/en/5.2/`), including its images.
- The Blender 5.2 Python API documentation (`https://docs.blender.org/api/5.2/`).
- The Blender 5.2 release notes (`https://developer.blender.org/docs/release_notes/5.2/`).
- Cycles' own files as supplied for 5.2.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did no work with helpers. Everything below I read myself.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**LABEL: not in the sources.**

The 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes and the Cycles files supplied for 5.2 nowhere state the rule Blender uses to decide that two recorded library locations mean one library or two. Nothing says whether the recorded text is compared as written, or whether it is first turned into a full path from the disk root; nothing says how a location written relative to the file is compared against the same file written out in full; nothing says whether capital and small letters are taken into account, nor what happens when two different recorded locations reach the same file on disk by different routes. I searched for this and found no statement either way — the record of that search is below.

What the sources **do** say, and which bears on the question without settling it:

**A library is a data-block, and data-block names are unique per type per file.**

> "A data-block’s name is unique within its namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."

and, in the table of data-block types on the same page:

> "Library ✗ ✓ References to an external blend-file. Access from the Outliner’s Blender File view."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/data_blocks.html`. Read 2026-07-30.

So two library data-blocks in one file cannot carry the same name. But the sources never say what a library data-block's *name* is, and in particular never say that the name is the recorded location. Without that, this quote cannot be turned into an answer.

**A library data-block records the location as a separate property, and that property may be written relative.**

> "filepath
>
> Path to the library .blend file (default “”, never None, blend relative // prefix supported)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.Library.html`. Read 2026-07-30.

**The same library file can be recorded either relative or absolute, at the user's choice.**

> "Relative Path
>
> Reference the external blend-file using a relative path rather than an absolute one."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`. Read 2026-07-30.

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`. Read 2026-07-30.

Taking these two together, one and the same library file can end up recorded as two different pieces of text. That step is mine — neither page says it in so many words. The sources do not then say what Blender makes of two such records.

**Blender's own wording assumes an identity of libraries without defining it.** The manual describes relocating a linked data-block as picking a replacement "from the same library or from a different one":

> "A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`. Read 2026-07-30.

The Python API does the same, when it describes appending:

> "APPEND_RECURSIVE
>
> Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html`. Read 2026-07-30.

In both cases "the same library" is used as a settled idea. Neither page says how sameness is worked out.

**Libraries are addressed by a single piece of text in the operators that act on one.** Both the reload and the relocate operator take the library as a string:

> "bpy.ops.wm.lib_reload(*, library='', filepath='', directory='', filename='', … )
>
> Reload the given library
>
> Parameters:
>
> library (str) – Library, Library to reload (optional, never None)"

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', … )
>
> Relocate the given library to one or several others
>
> Parameters:
>
> library (str) – Library, Library to relocate (optional, never None)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Read 2026-07-30.

This shows a library is picked out by one string, but the pages do not say what that string is, nor how it is matched.

**A library's location is reported as the text that was written, which may be relative or absolute.** The 5.2 release notes show what a file's own list of libraries looks like when read out:

> "Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"

with the worked example:

> "bpy_lib.libraries(filepath='//../../../assets/scripts/update_bake_path.blend', is_archive=False)"

Source: release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`. Read 2026-07-30.

The matching API page adds that this text is not resolved for you:

> "A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`. Read 2026-07-30.

So what is reported is the text in whichever form it was written, and the `filepath` entry quoted above says the stored form may carry the relative `//` prefix. Again, no rule is given for comparing two of them.

**Where I looked, and what I ruled out.**

Pages read in full or in the relevant part:

- Manual: `files/data_blocks.html` (whole page, including the "Name & Rename" section and the data-block type table), `files/linked_libraries/link_append.html` (whole page: Link, Append, Reload Library, Relocate Library, Relocate Linked ID, Make Local, Known Limitations), `files/linked_libraries/index.html`, `files/blend/open_save.html` (the "Relative Paths" section and Save As options), `files/blend/packed_data.html` (the "Pack Linked Libraries" and "Unpack Linked Libraries" sections), `files/file_paths.html` (whole page — it turned out to be about path templates, not about linking), `editors/outliner/interface.html` (the Blender File display mode and the filter options), `glossary/index.html` (searched for a Library entry; the only library entries there are about asset libraries, which are a different thing — a registered folder of assets).
- Python API: `bpy.types.Library.html`, `bpy.types.LibraryWeakReference.html`, `bpy.types.BlendDataLibraries.html`, `bpy.types.BlendData.html` (the `libraries` and `filepath` entries), `bpy.types.ID.html` (the `name`, `name_full`, `library`, `is_library_indirect`, `is_missing` entries), `bpy.types.BlendImportContext.html`, `bpy.types.BlendImportContextItem.html`, `bpy.types.BlendImportContextLibrary.html`, `bpy.types.BlendImportContextLibraries.html`, `bpy.ops.wm.html` (the `link`, `append`, `lib_reload`, `lib_relocate` entries), `bpy.ops.outliner.html` (the `lib_relocate` and `lib_operation` entries), `bpy.path.html` (the `abspath` entry, which has a `library` argument).
- Release notes: every 5.2 subject file was searched for the word "librar"; the only hits were the Python API note quoted above, the core note about a new library-override delete operation, the asset-library notes, and bug-fix lines. None touches library identity.
- Cycles' own files for 5.2: searched for any mention of `.blend`. That search returns ten hits: variables named `blend` in shader kernel code (`source/kernel/svm/fresnel.h` line 38, `source/kernel/svm/image.h` line 108, `source/kernel/svm/mix.h` line 35), `context.blend_data` in `__init__.py` (lines 99, 102, 103), and four web addresses in comments. None is about a blend-file that Blender links from. **I did not search Cycles' 5.2 files for "library", so this record does not establish that Cycles holds nothing on the subject — only that a search for `.blend` found nothing.** *(Changed by the citation check, 2026-08-09: this used to say the only hits were two shader-kernel lines and that "Cycles has no part in this". The `.blend` search returns ten hits, not two, and it is not the search that would find library handling — the second pass for this version searched for "library" and did find `version_update.py` walking `bpy.data.libraries`. The claim has been withdrawn and replaced by what the search actually establishes.)*

Words and phrases I searched for across the manual, the API documentation and the release notes, all with no relevant result: "same library", "already linked", "duplicate library", "two libraries", "separate libraries", "distinct library", "existing library", "already loaded", "one library", "per library", "single library", "shared library", "loaded twice", "same blend-file", "same blendfile", "same source file", "resolves to the same", "identical", "canonical", "normalize", "case sensitive", "case-insensitive", "symbolic link", "symlink", "hard link", "path comparison", "compare path", "de-duplicate", "deduplicate", "name of the library", "library name".

Pictures: the only picture bearing on this subject on the pages I read is the Blender File view of the Outliner on `https://docs.blender.org/manual/en/5.2/files/data_blocks.html`, shown as `[image: _images/files_data-blocks_outliner-blender-file-view.png]`. I opened it, read 2026-07-30. **Documented, from a picture.** Visible in it: the display-mode menu (Scenes, View Layer, Sequence, Blender File highlighted, Data API, Orphan Data) and a single "Current File" tree with its data-block categories (Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds). No linked library row appears anywhere in it. **Inferred**, from what the picture does not show: nothing about how libraries are listed or told apart can be read off it. I did not read anything further into it. *(Changed by the citation check, 2026-08-09: the two labels the Method asks for have been added; the finding already flagged the second part as its own step, and that flag is now a label.)*

**Ambiguity in the question.** I read "two recorded library locations" as two location strings recorded in blend-files — either two entries in one file, or one entry in each of two files that are being brought together. A second reading is possible: two locations recorded for the *same* library entry over time, as when a library is relocated. The sources are silent on both readings, and my search above covers both, so the choice does not change the answer.

**Not covered.** This answer does not settle any of the following, because the sources do not address them: whether comparison is on the text as recorded or on the full path worked out from it; whether the relative form `//assets/x.blend` and the absolute form of the same file are treated as one library; whether letter case is taken into account, and whether that varies by operating system; what happens when two recorded locations reach the same file by different routes, such as through a link or an alias, or through two different mount points naming the same shared folder; whether the answer differs at load time, at link time, at reload, and at relocate; and whether a file packed into an "archive" library counts as the same library as the original it was packed from.

**Outside the interface.** The sources do not say. I found no statement about library identity at all, in the interface or outside it, so I have nothing that would hold or fail when Blender is started and driven by something other than a person clicking. Note that the one piece of hard information nearby — that a library is picked out by a single string in `bpy.ops.wm.lib_reload` and `bpy.ops.wm.lib_relocate` — is written for exactly that outside-the-interface case, but it still does not say how the string is matched.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.

**Operating system.** The sources do not say. The manual and the API documentation give no operating-system-specific wording anywhere near libraries or their recorded locations, and I found nothing in the 5.2 release notes on it either. In particular, nothing states whether letter case matters in a recorded library location, nor whether that differs between operating systems.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 5.2.0 LTS, at `/opt/blender/5.2/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 5.2.0 LTS, at `/Applications/Blender.app/Contents/MacOS/Blender`.

**What was done, in every test.** Blender was started with `--background --factory-startup`, pointed at a throwaway settings folder made immediately before the run, in a working folder made immediately before the run. Blend-files were built by script. Every link was made twice over in two separate runs — once with `bpy.ops.wm.link`, the operator the interface's File ▸ Link runs, and once with `bpy.data.libraries.load`, a plain Python call. **The two gave the same result in every test below**, on both computers. Neither call's arguments were guessed; they were read off the running Blender first.

**What was read, after every step.**

```python
print("HT|%s|library_count|%d" % (tag, len(bpy.data.libraries)))
for i, lib in enumerate(bpy.data.libraries):
    print("HT|%s|library|%d|name=%r|filepath=%r" % (tag, i, lib.name, lib.filepath))
for ob in bpy.data.objects:
    print("HT|%s|object|name=%r|name_full=%r|from=%r" % (
        tag, ob.name, ob.name_full, ob.library.filepath if ob.library else None))
```

That is the same set of library entries the Outliner's Blender File view lists in a window, read from a terminal instead. It is a different test from counting rows on a screen, and no screen was used in any of this.

**Every test also saved the file and opened it again in a fresh Blender**, and read the same lines again, so that a decision made when linking and a decision made when reading a file back could be told apart.

**The same fifteen tests were run in 4.5.12 LTS, 5.0.1 and 5.1.2 as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-5.2-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-5.2-ops/libB.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same counts, `1, 1, 2, 2`. The two agreed. Every test below rests on this: the number can come out either way.

### 2. The same file recorded relative, then recorded absolute

*Done to:* `lib.blend` with objects `A1` and `A2`, and `host.blend` beside it. *Done:* link `A1` with the relative option on, then `A2` from the same file with it off.

The rented Linux machine printed:

```
HT|after-relative|library_count|1
HT|after-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|after-absolute|library_count|1
HT|after-absolute|library|0|name='lib.blend'|filepath='//lib.blend'
HT|after-fresh-reopen|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='//lib.blend'
```

The Creator's Mac printed the same counts and the same `'//lib.blend'`. The two agreed.

### 3. The same file written with a step that goes nowhere

*Done:* link three objects from one file through `…/lib.blend`, `…/./lib.blend` and `…/sub/../lib.blend`.

The rented Linux machine printed:

```
HT|paths|plain='/root/ht-deps/e3-5.2-data/lib.blend'|dotted='/root/ht-deps/e3-5.2-data/./lib.blend'|updown='/root/ht-deps/e3-5.2-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-5.2-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-5.2-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-5.2-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-5.2-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-5.2-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-5.2-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808618 /root/ht-deps/e5-5.2-data/lib.blend` and `1808618 /root/ht-deps/e5-5.2-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-5.2-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-5.2-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-5.2-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-5.2-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-5.2-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.2-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-5.2-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.2-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-5.2-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-5.2-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.2-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.2-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-5.2-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-5.2-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-5.2-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-5.2-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-5.2-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-5.2-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-5.2-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-5.2-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-5.2-data/lib.blend'
HT|after-fresh-reopen|library_count|2
```

`lib.blend` was already present after the first link, so the second half means something. The Creator's Mac printed the same. The two agreed.

### 10. Two libraries whose recorded text would have been the same

*Done to:* two folders side by side, each with its own `x.blend`; in the first, `mid.blend`, which links its neighbour with the relative option on; `host.blend` in the second folder. *Done:* link the second folder's `x.blend` with the relative option on, so `host.blend` also records `//x.blend`, then link the collection from `mid.blend`.

The set-up's own control printed, when `mid.blend` was built, `HT|built-mid.blend|library|0|name='x.blend'|filepath='//x.blend'`. The rented Linux machine then printed:

```
HT|after-own-relative|library_count|1
HT|after-mid|library_count|3
HT|after-mid|library|1|name='x.blend'|filepath='//x.blend'
HT|after-mid|library|2|name='x.blend.001'|filepath='//../one/x.blend'
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-5.2-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-5.2-data/one/mid.blend'|abspath='/root/ht-deps/e10-5.2-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-5.2-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-5.2-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place the versions differ

*Done:* ask whether the three archive properties exist, then link with packing asked for.

The rented Linux machine, Blender 5.2.0 LTS, printed:

```
HT|prop|is_archive|present=True
HT|prop|archive_parent_library|present=True
HT|prop|archive_libraries|present=True
HT|pack|returned normally
HT|after-pack|library_count|2
HT|after-pack|library|0|name='lib.blend'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'
HT|after-pack|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'
HT|after-pack|object|name='A1'|name_full='A1 [lib.blend.001]'|from='/root/ht-deps/e12-5.2-data/lib.blend'
HT|archive|name='lib.blend'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'|is_archive=False|archive_parent=None|archives=['/root/ht-deps/e12-5.2-data/lib.blend']
HT|archive|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'|is_archive=True|archive_parent='/root/ht-deps/e12-5.2-data/lib.blend'|archives=[]
```

So two library entries, holding **the same recorded text**, one with `is_archive=False` and one with `is_archive=True` whose archive parent is the first. The Creator's Mac printed the same two entries with the same two flags. The two agreed.

The same test in Blender 4.5.12 LTS printed `present=False` for all three properties and refused the call with `HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")`, leaving no library at all. 5.0.1, 5.1.2 and 5.2.0 LTS all behaved as printed above.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-5.2-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-5.2-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-5.2-data/b.blend'
HT|after-b|library|2|name='lib.blend'|filepath='//lib.blend'
HT|after-fresh-reopen|library_count|3
```

`lib.blend` is listed once. The Creator's Mac printed the same. The two agreed.

### What the tests do not settle

- What happens when a library really is relocated onto a file another library already records. See test 8: the call never moved the library it named, so no collision arose.
- Anything about Windows. No Windows machine has run it, so a drive letter against a network name, back slashes against forward slashes, a directory junction, and the Windows side of the capitals test all stay untested rather than answered.
- Two different network mounts of one share. This needs a second computer serving a share, which the project does not have. The second mount of one folder on the rented Linux machine, in test 5, is a different condition.
- What any of this looks like in a window: how many rows the Outliner's Blender File view shows, what the Status Bar's memory figure does, and what the Info editor says after a relocate. The rented Linux machine has no display server, and no window was opened on either machine.
- The other reading of the question, about asset libraries registered in the Preferences. No test above touches it.

---

## Still open for a person with Blender 5.2 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 5.2

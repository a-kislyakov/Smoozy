# Dependencies — later questions (Blender 5.1)

Sources consulted, all inside `Blender Sources/`, all read on 2026-07-30 (the snapshot date given to me): the Blender 5.1 manual (`manual_5.1`, including one picture from its `_images` folder), the Blender 5.1 Python API documentation (`api_5.1`), the Blender 5.1 release notes (`release_notes_5.1`), and Cycles' own files (`cycles_5.1`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Label: not in the sources.**

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, or Cycles' own files states the rule by which Blender decides that two recorded library locations mean one library or two. The sources describe what a library is, what its recorded location looks like, and what you can do to a library (reload, relocate, pack, make local). They never describe comparing one recorded location against another.

**The question was ambiguous, and I answered one reading.** I took "library" to mean a linked blend-file — the thing the manual calls a linked library and the API calls `bpy.types.Library`. The other reading is the asset libraries a person registers in Preferences, which are folders rather than files. The sources are silent on the identity rule for that reading too; the one thing they do say about it is that an asset library's name need not be unique and is for the user's own reference, quoted below.

### What the sources do say, and where it stops

Everything in this section is **documented** unless a paragraph carries its own label.

**A linked library records the file it points at in a single path, and that path may start with the relative marker `//`.**

> "External .blend file from which data is linked"

> "filepath
>
> Path to the library .blend file (default “”, never None, blend relative // prefix supported)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Library.html — read 2026-07-30.

That tells us the recorded location is a single text path that can start with `//`. It does not say whether `//assets/props.blend` and the full path it expands to are compared as text or as resolved files.

**Which library a data-block came from is part of that data-block's identity.**

> "name
>
> Unique data-block ID name (within a same type and library) (default “”, never None)"

> "name_full
>
> Unique data-block ID name, including library one if any (default “”, readonly, never None)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ID.html — read 2026-07-30.

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."

> "However, it is possible to have one local and several linked Objects sharing the same name."

Source: manual, https://docs.blender.org/manual/en/5.1/files/data_blocks.html — read 2026-07-30.

The same page lists Library as a data-block type in its own right:

> "Library	✗	✓	References to an external blend-file. Access from the Outliner’s Blender File view."

Source: manual, https://docs.blender.org/manual/en/5.1/files/data_blocks.html — read 2026-07-30.

So a library is itself a data-block, and the quoted words make the library a data-block belongs to part of what keeps its name unique. But the sources stop exactly one step short of the question: they never say what makes two recorded locations resolve to one library data-block rather than two.

**Relocating replaces a library's file; nothing is said about what happens if the new file is already linked.**

> "When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

> "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

Source: manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

The operator that does this takes the library as a plain string parameter labelled "Library", separate from the path arguments beside it, and says nothing about matching locations. What that string is, the page does not say:

*(Changed by the citation check, 2026-08-09: this used to read "takes the library as a plain name string, which is a name rather than a location". The page says only that the parameter is a string called "Library"; calling it a name, and setting a name against a location, is a step the page does not carry.)*

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, …)"

> "Relocate the given library to one or several others"

> "library (str) – Library, Library to relocate (optional, never None)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30.

**The nearest the sources come is appending with data reuse, where Blender keeps a record of where appended data came from — and even there no comparison rule is given.**

> "Append (Reuse Data)
>
> Specific to the Asset Browser.
>
> The first time an asset is used, it will be appended, including its dependencies, just like described previously. However, Blender will keep track of where it originated, and the next time the asset is used, as much data as possible will be reused."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html — read 2026-07-30.

What is kept track of is described here:

> "Read-only external reference to a linked data-block and its library file"

> "filepath
>
> Path to the library .blend file (default “”, never None, blend relative // prefix supported)"

> "id_name
>
> Full ID name in the library .blend file (including the two leading ‘id type’ chars) (default “”, never None)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.LibraryWeakReference.html — read 2026-07-30.

So the recorded origin of appended data is a file path plus a data-block name. This is about appended copies, not about linked libraries, and the sources still do not say how one recorded path is judged equal to another.

**For the other reading of the question — asset libraries registered in Preferences — the sources say the name need not be unique and is for the user's own reference.**

> "name
>
> Identifier (not necessarily unique) for the asset library (default “”, never None)"

> "path
>
> Path to a directory with .blend files to use as an asset library (default “”, never None)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.UserAssetLibrary.html — read 2026-07-30.

> "Asset Libraries
>
> Name and on-drive directory paths of asset libraries. To make Blender aware of an asset library, add it to this list. The name is for your reference only, and will appear in asset library selectors. The path should point to the location of the asset library."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html — read 2026-07-30.

That says the name need not be unique and is for the user's own reference, but it does not say that two entries pointing at the same folder are treated as one, or as two.

### One related thing the sources touch on, and which limits the question

The helper that turns a recorded location into a full one takes the library the location came from, and that library's own path is then used as the starting point:

> "bpy.path.abspath(path, *, start=None, library=None)"

> "library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.path.html — read 2026-07-30.

**Label for this paragraph alone: inferred.** The step: the documentation says a library's own path replaces the starting point when a path from that library is made absolute; a starting point is what a `//` path is measured from; therefore two identical relative texts recorded in two different libraries can name two different files on disk. The documentation does not spell that consequence out, and it does not say what the starting point is when no library is given.

### What Cycles' own files show

**Label for this paragraph alone: observed.** Cycles' add-on asks a data-block for its library, and tests that against a collection of libraries:

```
    # Map of versions used by libraries.
    library_versions = {}
    library_versions[bpy.data.version] = [None]
    for library in bpy.data.libraries:
        library_versions.setdefault(library.version, []).append(library)

    # Do versioning per library, since they might have different versions.
```

and, a few lines further:

```
        for scene in bpy.data.scenes:
            if scene.library not in libraries:
                continue
```

File: `cycles_5.1/version_update.py` — the first block is lines 97–103 and the second is lines 110–112 (the part that follows the line `if not bpy.data.is_saved: return`). Read 2026-07-30.

*(Changed by the citation check, 2026-08-09: the address used to read "lines 97–111". The quoted words are all in the file, but the second block ends at line 112, where the `continue` sits.)*

This is exactly what the file says, no more. It shows that a data-block's library is asked for as a library data-block and tested against other library data-blocks. It does **not** show how two recorded locations become one such data-block in the first place, and I did not run anything.

### Not covered

This answer does not settle any of the following, because the sources are silent on all of them:

- Whether linking twice from the same file, once by a relative location and once by an absolute one, gives one library or two.
- Whether capital and small letters in a recorded location affect the comparison, and whether that differs by operating system.
- Whether a shortcut or link on disk that points at the same file (a symbolic link, a mounted share reached by two names, a drive letter versus a full server name) counts as the same library.
- Whether trailing separators, `..` segments, or other differently written forms of the same location are reduced to a common form before comparison.
- What happens when a library is relocated onto a file that is already linked as another library — whether the two merge.
- Whether the answer differs for the archive libraries the API names (`Library.is_archive`, `Library.archive_parent_library`, `Library.archive_libraries`, which exist in the API but carry no text about identity).

### Outside the interface

The sources do not say. I found no statement anywhere that library identity, path resolution, or linking behaves differently when Blender is started and driven by something other than a person clicking. The pieces above are Python API facts (`bpy.types.Library`, `bpy.data.libraries`, `bpy.path.abspath`, `bpy.ops.wm.lib_relocate`) and the API documentation attaches no interface-only condition to any of them; but since the identity rule itself is not documented at all, nothing can be said about whether it holds outside the interface.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.

### Operating system

The sources do not say. I searched for statements tying path comparison to an operating system and found none. The nearest thing in the sources is a note that a path helper exists for Windows compatibility — "Equivalent to os.path.basename, but skips a “//” prefix. Use for Windows compatibility." (Python API documentation, https://docs.blender.org/api/5.1/bpy.path.html, read 2026-07-30) — which is about the `//` prefix, not about deciding whether two locations are the same.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked

Manual pages read in full or in the relevant part:

- https://docs.blender.org/manual/en/5.1/files/linked_libraries/index.html
- https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html (Link, Append, Reload Library, Relocate Library, Broken Libraries, Relocate Linked ID, Make Local, Known Limitations)
- https://docs.blender.org/manual/en/5.1/files/linked_libraries/library_overrides.html (searched for path, library file and same-library wording)
- https://docs.blender.org/manual/en/5.1/files/data_blocks.html (Data-Block Types table, Name & Rename, Sharing)
- https://docs.blender.org/manual/en/5.1/files/blend/open_save.html (Remap Relative, Relative Paths)
- https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html (Pack/Unpack Linked Libraries)
- https://docs.blender.org/manual/en/5.1/files/file_paths.html (path templates, including `blend_name_lib` and `blend_dir_lib`)
- https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html (Asset Libraries, Import Method, Relative Path)
- https://docs.blender.org/manual/en/5.1/editors/outliner/interface.html (Blender File display mode)
- https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html (External Data: Make Paths Relative/Absolute, Report Missing Files, Find Missing Files)
- https://docs.blender.org/manual/en/5.1/scene_layout/object/selecting.html (Select ‣ Linked ‣ Library)
- https://docs.blender.org/manual/en/5.1/files/asset_libraries/introduction.html
- https://docs.blender.org/manual/en/5.1/glossary/index.html (entries mentioning library)
- https://docs.blender.org/manual/en/5.1/files/blend/rename.html (checked in case naming rules covered libraries; it covers bones, nodes, objects and strips only)

Picture opened: `_images/files_data-blocks_outliner-blender-file-view.png`, the Blender File view of the Outliner shown on https://docs.blender.org/manual/en/5.1/files/data_blocks.html (read 2026-07-30). **Documented, from a picture.** Visible in it: the Outliner's display-mode menu (Scenes, View Layer, Sequence, Blender File highlighted, Data API, Orphan Data), and below it a single top-level entry, "Current File", with data-block types listed under it (Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds). No linked library row appears in it. **Inferred** from that absence: it settles nothing about how libraries are listed or distinguished. *(Changed by the citation check, 2026-08-09: labels added and the display-mode menu named, as the Method now asks for pictures.)* No other picture in the manual's `_images` folder has "librar" in its name apart from the asset-library preferences and pose-library ones, which I checked by listing.

Python API pages read:

- https://docs.blender.org/api/5.1/bpy.types.Library.html
- https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html (`load`, `write`, `remove`)
- https://docs.blender.org/api/5.1/bpy.types.LibraryWeakReference.html
- https://docs.blender.org/api/5.1/bpy.types.BlendImportContextLibrary.html and https://docs.blender.org/api/5.1/bpy.types.BlendImportContextLibraries.html
- https://docs.blender.org/api/5.1/bpy.types.ID.html (`name`, `name_full`, `library`, `is_missing`, `session_uid`)
- https://docs.blender.org/api/5.1/bpy.types.BlendData.html (`libraries`)
- https://docs.blender.org/api/5.1/bpy.path.html (`abspath`, `basename`)
- https://docs.blender.org/api/5.1/bpy.ops.wm.html (`link`, `append`, `lib_reload`, `lib_relocate`)
- https://docs.blender.org/api/5.1/bpy.ops.outliner.html (`lib_relocate`)
- https://docs.blender.org/api/5.1/bpy.types.UserAssetLibrary.html

Release notes: I searched every 5.1 release-note file for "librar" and read https://developer.blender.org/docs/release_notes/5.1/core/, https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ and https://developer.blender.org/docs/release_notes/5.1/python_api/ in full. The only hits are asset-library preferences changes, crash fixes involving libraries, and a note that third-party libraries were upgraded to VFX platform 2026 (a different sense of the word). Nothing about library identity.

Cycles' own files: I searched all of `cycles_5.1` for `libraries` and `.library`. The only hits are `version_update.py` (quoted above) and `osl.py`, where a library is passed to `bpy.path.abspath` to resolve a script path. Nothing about identity.

Search terms I used across the manual and the API documentation, all of which came back empty or irrelevant: "same library", "different library", "two libraries", "duplicate library", "distinct library", "already linked", "existing library", "new library", "per library", "same path", "same file", "identical path", "path comparison", "compare path", "canonical", "normalized path", "resolve path", "absolute path" (in company with "library" or "link"), "symlink", "symbolic link", "hard link", "case-insensitive", "case sensitive", "trailing slash". The one hit for "duplicate libraries in your search path" is on https://docs.blender.org/manual/en/5.1/troubleshooting/python.html and concerns duplicate system code libraries loaded by Python, not linked blend-files; I ruled it out on reading it.

---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 5.1.2, at `/opt/blender/5.1/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 5.1.2, at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.

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

**The same fifteen tests were run in 4.5.12 LTS, 5.0.1 and 5.2.0 LTS as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-5.1-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-5.1-ops/libB.blend'
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
HT|paths|plain='/root/ht-deps/e3-5.1-data/lib.blend'|dotted='/root/ht-deps/e3-5.1-data/./lib.blend'|updown='/root/ht-deps/e3-5.1-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-5.1-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-5.1-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-5.1-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-5.1-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-5.1-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-5.1-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808523 /root/ht-deps/e5-5.1-data/lib.blend` and `1808523 /root/ht-deps/e5-5.1-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-5.1-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-5.1-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-5.1-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-5.1-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-5.1-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.1-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-5.1-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.1-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-5.1-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-5.1-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.1-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.1-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-5.1-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-5.1-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-5.1-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-5.1-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-5.1-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-5.1-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-5.1-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-5.1-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-5.1-data/lib.blend'
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
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-5.1-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-5.1-data/one/mid.blend'|abspath='/root/ht-deps/e10-5.1-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-5.1-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-5.1-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place the versions differ

*Done:* ask whether the three archive properties exist, then link with packing asked for.

The rented Linux machine, Blender 5.1.2, printed:

```
HT|prop|is_archive|present=True
HT|prop|archive_parent_library|present=True
HT|prop|archive_libraries|present=True
HT|pack|returned normally
HT|after-pack|library_count|2
HT|after-pack|library|0|name='lib.blend'|filepath='/root/ht-deps/e12-5.1-data/lib.blend'
HT|after-pack|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.1-data/lib.blend'
HT|after-pack|object|name='A1'|name_full='A1 [lib.blend.001]'|from='/root/ht-deps/e12-5.1-data/lib.blend'
HT|archive|name='lib.blend'|filepath='/root/ht-deps/e12-5.1-data/lib.blend'|is_archive=False|archive_parent=None|archives=['/root/ht-deps/e12-5.1-data/lib.blend']
HT|archive|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.1-data/lib.blend'|is_archive=True|archive_parent='/root/ht-deps/e12-5.1-data/lib.blend'|archives=[]
```

So two library entries, holding **the same recorded text**, one with `is_archive=False` and one with `is_archive=True` whose archive parent is the first. The Creator's Mac printed the same two entries with the same two flags. The two agreed.

The same test in Blender 4.5.12 LTS printed `present=False` for all three properties and refused the call with `HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")`, leaving no library at all. 5.0.1, 5.1.2 and 5.2.0 LTS all behaved as printed above.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-5.1-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-5.1-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-5.1-data/b.blend'
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

## Still open for a person with Blender 5.1 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 5.1

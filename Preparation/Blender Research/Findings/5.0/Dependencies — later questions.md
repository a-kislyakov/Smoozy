# Dependencies — later questions (Blender 5.0)

Sources consulted, all inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date given to me):

- The Blender 5.0 manual (`manual_5.0`), pages under `https://docs.blender.org/manual/en/5.0/`
- The Blender 5.0 Python API documentation (`api_5.0`), pages under `https://docs.blender.org/api/5.0/`
- The Blender 5.0 release notes (`release_notes_5.0`), pages under `https://developer.blender.org/docs/release_notes/5.0/`
- Cycles' own files (`cycles_5.0`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Reading of the question.** I answer it as: inside one `.blend` file, a linked library is recorded as a location (a file path). Given two such recorded locations, what decides whether Blender ends up with one library or with two? The question is ambiguous — "library" is also the name Blender gives to a folder registered in the Preferences as an *asset library*, which is a different thing with its own recorded location. I answer the first reading, because the question sheet is about what a `.blend` file points at outside itself, and the asset library folder is recorded in the Preferences rather than in the file. What I found for the second reading is noted at the end of this answer.

### Label: **not in the sources**

The sources nowhere state the rule. I could not find any sentence, in the manual, in the Python API documentation, in the release notes or in Cycles' own files, that says how two recorded library locations are compared — not whether the comparison is on the stored text or on the resolved location, not whether capital and small letters matter, not whether a shortcut (symbolic link) is followed first, not whether `//lib.blend` and the same file written out in full count as one library or two.

What the sources *do* settle is the surrounding shape of the thing, and one place where a recorded location is used as a key. Those pieces are below, each with its own label. None of them adds up to the rule, and I have not joined them into one.

#### What is documented around it

**(a) A library is recorded as an external `.blend` file, and it carries one recorded path.** — **documented**

> "External .blend file from which data is linked"

> "filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Library.html`. Read 2026-07-30.

The file's set of libraries is a plain collection:

> "Collection of libraries"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html`. Read 2026-07-30.

And in the import machinery, the set of source libraries is described as being the blendfile paths:

> "Collection of source libraries, i.e. blendfile paths"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.BlendImportContextLibraries.html`. Read 2026-07-30.

So "two libraries" means two entries in that collection, each holding one recorded path. The documentation does not say what stops two entries holding the same path, nor what counts as "the same" path.

**(b) A recorded library location is used as a lookup key.** — **documented**

> "Library object name lookup using a pair,
> where the second argument is the library path matching bpy.types.Library.filepath."

and the example line that follows it:

> `obj = bpy.data.objects["my_obj", "//my_lib.blend"]`

Source: Python API documentation, `https://docs.blender.org/api/5.0/info_gotchas_internal_data_and_python_objects.html`, section "Library Collisions". Read 2026-07-30.

**Follow-on, labelled inferred.** The step: the quoted text says the second argument must *match* `bpy.types.Library.filepath`, and the example supplies the stored form `//my_lib.blend`, relative prefix and all — not a resolved location on disk. From that I take it that this particular lookup matches on the recorded string as stored. What I do *not* take from it, and what the source does not say, is that Blender uses this same comparison when deciding whether a newly recorded location is an existing library or a new one. A reader has to supply that, so I have not claimed it.

**(c) A library is a namespace for the names inside it.** — **documented**

> "Unique data-block ID name (within a same type and library)"

> "Unique data-block ID name, including library one if any"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.ID.html` (`ID.name` and `ID.name_full`). Read 2026-07-30.

This says what is at stake in the question — two libraries mean two separate name spaces, so the same name can appear in both — but it does not say how the two are told apart.

**(d) The same recorded text does not always mean the same file, because relative paths are resolved against whoever recorded them.** — **inferred**, from the two documented quotes below

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Source: manual, `https://docs.blender.org/manual/en/5.0/files/blend/open_save.html`, section "Relative Paths". Read 2026-07-30.

> "library (bpy.types.Library) – The library this path is from. This is only included for
> convenience, when the library is not None its path replaces start."

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.path.html` (`bpy.path.abspath`). Read 2026-07-30.

**Inferred, step written out.** The `abspath` entry says that when a path belongs to a library, that library's own path stands in for the starting point of the resolution. Joining that with the manual's statement that a relative path is evaluated against the directory of the referencing file, it follows that one and the same recorded text (say `//textures/x.png`, or `//lib.blend`) resolves to different files depending on which file recorded it. The sources do not go on to say whether library comparison happens before or after this resolution, which is exactly the missing piece of the question.

**(e) More than one library data-block can be tied to one source library — this is new in 5.0.** — **documented**

> "archive_libraries
>
> Archive libraries of packed IDs, generated (and owned) by this source library
>
> Type:
>
> bpy_prop_collection of Library, (readonly)"

> "is_archive
>
> This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library."

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Library.html`. Read 2026-07-30.

The release notes describe the same thing and say why there can be several:

> "In case there are several versions of a same ID (e.g. linked at different moments from a same library, which has been modified in-between), there will be several packed IDs."

> "Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
>
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'."

Source: release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`, section "Packed Data". Read 2026-07-30.

So in 5.0 a single source library can have several further library data-blocks hanging off it — its archives — and those are separate libraries with separate name spaces. What the quoted words do not say is what location an archive library itself records, so they do not establish that two libraries here hold the same recorded location. **Not in the sources**: what an archive library's own `filepath` holds. And nothing here bears on the ordinary, non-archive case the question asks about.

**(f) Pointing an existing library at a different file is a supported operation, but the collision case is not described.** — **documented** for the operation, **not in the sources** for what it decides

> "a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file."

Source: manual, `https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html`, section "Relocate Library". Read 2026-07-30.

> "Relocate the given library to one or several others"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.wm.html` (`bpy.ops.wm.lib_relocate`). Read 2026-07-30.

Nothing in either page says what happens if a library is relocated onto a location that another library in the same file already records — whether the two then become one library or stay two.

**(g) Blender knows that paths can be written in ways that differ only by capitalisation, and that shortcuts exist — but says nothing about library comparison.** — **documented**

> "Resolve a case insensitive path on a case sensitive system,
> returning a string with the path if found else return the original path."

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.path.html` (`bpy.path.resolve_ncase`). Read 2026-07-30.

> "File open dialog resolves symlinks to their final location"

Source: release notes, `https://developer.blender.org/docs/release_notes/5.0/bugfixes/`, "Core" list. Read 2026-07-30. Note what this one says and does not say: it is about the file open dialog, which is a thing a person clicks in, not about how a recorded library location is compared with another.

#### Where I looked

Whole-of-source text searches across `manual_5.0`, `api_5.0` and `release_notes_5.0`, for: `same library`, `two libraries`, `different librar`, `separate librar`, `duplicate librar`, `distinct librar`, `already linked`, `existing library`, `library name`, `name of the library`, `library path`, `library file path`, `libname`, `same path`, `identical path`, `unique path`, `same file`, `canonical`, `normaliz`, `symlink`, `symbolic link`, `case sensitive`, `case-sensitive`, `merge`, `de-duplicat`, `dedup`, `reuse`, `re-use`, `single copy`, `one copy`, `namespace`, and `unique` cross-checked against `librar`.

Pages read in full or in the relevant part, and what each ruled out:

- Manual, `https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html` — the whole Link, Append, Reload, Relocate, Relocate Linked ID, Make Local and Known Limitations material. This is the page where the rule would most naturally live. It never mentions comparing library locations.
- Manual, `https://docs.blender.org/manual/en/5.0/files/linked_libraries/index.html` — the section's table of contents; the only other page in it is Library Overrides, which is about editing linked data, not about identifying libraries.
- Manual, `https://docs.blender.org/manual/en/5.0/files/blend/open_save.html` — the "Relative Paths" material, quoted above. Explains how paths are stored, not how they are compared.
- Manual, `https://docs.blender.org/manual/en/5.0/files/file_paths.html` — path templates only. Nothing on libraries beyond the `blend_name_lib` and `blend_dir_lib` variables.
- Manual, `https://docs.blender.org/manual/en/5.0/files/data_blocks.html` — the Library row of the data-block table, the naming/uniqueness material and the Sharing material. No comparison rule. The one picture on this page, `_images/files_data-blocks_outliner-blender-file-view.png`, read 2026-07-30: **documented, from a picture.** Visible in it: the Outliner's display-mode menu (Scenes, View Layer, Sequence, Blender File highlighted, Data API, Orphan Data), and below it a Blender File listing whose only top-level row reads "Current File", with the data-block types under it (Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds). No linked library row appears in it. **Inferred** from that absence: it settles nothing about how libraries are listed or labelled. *(Added by the citation check, 2026-08-09: this document read the page in full but did not open its picture, which the other three versions did; the Method treats a picture as a document, so it is recorded here.)*
- Manual, `https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html` — packing and unpacking, including "Pack Linked Libraries" and "Unpack Linked Libraries". Nothing on library identity.
- Manual, `https://docs.blender.org/manual/en/5.0/editors/outliner/interface.html` — the Blender File display mode, which is where libraries are listed. Two lines, no rule. I also checked this page and the rest of the Outliner pages for pictures that might show the list; the only images on them are of view modes, mode icons and restriction toggles, none of a library list, so there was no picture to read here.
- Python API, `https://docs.blender.org/api/5.0/bpy.types.Library.html` — every property of the type. There is no identity or key property beyond `filepath`.
- Python API, `https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html` — `load`, `remove`, `tag`, and the worked examples. `load` takes a `filepath` and says nothing about matching it against libraries already present.
- Python API, `https://docs.blender.org/api/5.0/bpy.types.LibraryWeakReference.html` — the closest thing to a documented identity key anywhere in the sources, but it is for *appended* data, not for libraries: "Read-only external reference to a linked data-block and its library file", holding a `filepath` and an `id_name`. It is described as being "used to re-use already appended data instead of appending new copies" (`https://docs.blender.org/api/5.0/bpy.types.ID.html`). It does not describe how libraries themselves are compared.
- Python API, `https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html`, `…BlendImportContextLibrary.html`, `…BlendImportContextLibraries.html`, `…BlendImportContextItem.html` — the import context exposed to the pre/post import handlers. It lists source libraries by `filepath` and reports per-item outcomes such as "An existing matching local ID has been re-used", but "matching" is never defined and applies to data-blocks, not libraries.
- Python API, `https://docs.blender.org/api/5.0/bpy.path.html` — the whole module.
- Python API, `https://docs.blender.org/api/5.0/bpy.ops.wm.html` — `lib_reload` and `lib_relocate` in full.
- Python API, `https://docs.blender.org/api/5.0/bpy.types.BlendData.html` — `file_path_map` and `foreach_path`, which walk the file paths used by data-blocks and have an `include_libraries` option. They gather paths; they do not compare them.
- Release notes, `https://developer.blender.org/docs/release_notes/5.0/core/` in full, plus a search for "librar" across every 5.0 release-note page. The library-related entries are the Packed Data ones quoted above, an asset note pointing back at them, and bug-fix list lines. None states a comparison rule.
- Cycles' own files — **observed**, by reading the files themselves. The only places that touch libraries at all are `cycles_5.0/version_update.py` and `cycles_5.0/osl.py`. `version_update.py`, lines 97–111, holds the line `if scene.library not in libraries:`, which tests a library data-block against a list rather than testing one recorded path against another. `osl.py` holds `bpy.path.abspath(filepath, library=library)` at lines 197 and 237. What these files show is that no comparison of recorded library locations is written in either of them. What a render then does with a library is a step beyond reading them, and I have not taken it.

### Not covered

This answer does not settle any of the following, because the sources are silent on all of them:

- Whether the comparison is made on the recorded text or on the resolved location.
- Whether `//lib.blend` and the same file written out in full, in one file, give one library or two.
- Whether trailing separators, `..` segments, mixed slash directions, drive letters against network-style locations, or differing capitalisation make two recorded locations one library or two.
- Whether shortcuts (symbolic links) are followed before the comparison. The one release-note line on symlinks is about the file open dialog only.
- What happens when a library is relocated onto a location another library already records.
- Whether the answer differs between linking during a session and reading a saved file back in.
- Whether the answer differs for archive libraries beyond the one documented fact that they are separate library data-blocks owned by an archive parent.

### Outside the interface

**Not in the sources.** Since the sources do not state the rule at all, they also do not state whether it holds when Blender is driven from outside its own window. What I can say is narrower and is documented: the pieces involved are all data, not interface — `bpy.data.libraries` is a plain collection of `Library` data-blocks each holding a `filepath`, and `bpy.data.libraries.load(filepath, …)` is a Python call, both quoted above from `https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html` and `https://docs.blender.org/api/5.0/bpy.types.Library.html` (read 2026-07-30). One thing named in this answer is, by contrast, described only as something done by clicking: the one symlink statement is about the file open dialog. Relocate is described both ways — as an Outliner context-menu item in the manual and as `bpy.ops.wm.lib_relocate` in the Python API, both quoted in (f). So the only part of the picture known to be tied to the interface is the open dialog's symlink resolution; the rest is documented as data or as a Python call. I cannot say more without guessing.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.

### Operating system

**Not in the sources.** No page states the rule, and none states an operating-system difference in it. Two documented facts sit next to the question without answering it: `bpy.path.resolve_ncase` exists to "Resolve a case insensitive path on a case sensitive system" (`https://docs.blender.org/api/5.0/bpy.path.html`), which shows Blender treats systems as differing on capitalisation in path handling generally; and `bpy.path.native_pathsep` exists to "Replace the path separator with the systems native os.sep" (same page), which shows separators are treated as system-specific. Neither is said to apply to comparing library locations. Read 2026-07-30.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### On the other reading of the question

If "recorded library location" meant an *asset library* — a folder registered in the Preferences — then one thing is documented and worth recording, though it does not answer the question either:

> "An asset library is a directory on your drive that is registered in the Preferences as an asset library.
> Registering it means that you give the library a name (like "Sprite Fright")
> and the location on drive (like /home/sybren/projects/sprite-fright/assets)."

Source: manual, `https://docs.blender.org/manual/en/5.0/files/asset_libraries/introduction.html`. Read 2026-07-30.

> "name
>
> Identifier (not necessarily unique) for the asset library"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.UserAssetLibrary.html`. Read 2026-07-30.

**Documented**: an asset library entry is a name plus a directory, and the name is explicitly not required to be unique. **Not searched** — not *not in the sources*: what happens when two entries name the same directory, or when one entry's directory sits inside another's. The search terms and pages listed under *Where I looked* above were all aimed at linked libraries, so this second reading has no record of looking behind it and is not answered here. The catalogs page mentions in passing that the on-disk layout is taken at face value, "(ignoring advanced things like symbolic links)" (`https://docs.blender.org/manual/en/5.0/files/asset_libraries/catalogs.html`, read 2026-07-30), which is the nearest the manual comes to the subject and is about catalogs, not about entry identity.

---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 5.0.1, at `/opt/blender/5.0/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 5.0.1, at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.

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

**The same fifteen tests were run in 4.5.12 LTS, 5.1.2 and 5.2.0 LTS as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-5.0-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-5.0-ops/libB.blend'
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
HT|paths|plain='/root/ht-deps/e3-5.0-data/lib.blend'|dotted='/root/ht-deps/e3-5.0-data/./lib.blend'|updown='/root/ht-deps/e3-5.0-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-5.0-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-5.0-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-5.0-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-5.0-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-5.0-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-5.0-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808491 /root/ht-deps/e5-5.0-data/lib.blend` and `1808491 /root/ht-deps/e5-5.0-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-5.0-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-5.0-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-5.0-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-5.0-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-5.0-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.0-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-5.0-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.0-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-5.0-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-5.0-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.0-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.0-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-5.0-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-5.0-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-5.0-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-5.0-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-5.0-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-5.0-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-5.0-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-5.0-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-5.0-data/lib.blend'
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
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-5.0-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-5.0-data/one/mid.blend'|abspath='/root/ht-deps/e10-5.0-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-5.0-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-5.0-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place the versions differ

*Done:* ask whether the three archive properties exist, then link with packing asked for.

The rented Linux machine, Blender 5.0.1, printed:

```
HT|prop|is_archive|present=True
HT|prop|archive_parent_library|present=True
HT|prop|archive_libraries|present=True
HT|pack|returned normally
HT|after-pack|library_count|2
HT|after-pack|library|0|name='lib.blend'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'
HT|after-pack|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'
HT|after-pack|object|name='A1'|name_full='A1 [lib.blend.001]'|from='/root/ht-deps/e12-5.0-data/lib.blend'
HT|archive|name='lib.blend'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'|is_archive=False|archive_parent=None|archives=['/root/ht-deps/e12-5.0-data/lib.blend']
HT|archive|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'|is_archive=True|archive_parent='/root/ht-deps/e12-5.0-data/lib.blend'|archives=[]
```

So two library entries, holding **the same recorded text**, one with `is_archive=False` and one with `is_archive=True` whose archive parent is the first. The Creator's Mac printed the same two entries with the same two flags. The two agreed.

The same test in Blender 4.5.12 LTS printed `present=False` for all three properties and refused the call with `HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")`, leaving no library at all. 5.0.1, 5.1.2 and 5.2.0 LTS all behaved as printed above.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-5.0-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-5.0-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-5.0-data/b.blend'
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

## Still open for a person with Blender 5.0 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 5.0

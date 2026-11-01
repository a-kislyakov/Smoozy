# Dependencies — later questions (Blender 4.5)

Sources consulted, all on **2026-07-30** (the snapshot date of the copies I was given):

- Blender 4.5 manual (`manual_4.5`, including its `_images` folder)
- Blender 4.5 Python API documentation (`api_4.5`)
- Blender 4.5 release notes (`release_notes_4.5`)
- Cycles' own files (`cycles_4.5`)

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I did not use any helper agents. Everything below I read myself.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Label: not in the sources.**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes or Cycles' own files states the rule by which Blender decides that two recorded library locations point at one library rather than two. There is no statement anywhere in these sources about whether the two recorded strings are compared as-is, or first turned into one standard form, or turned into a real file on disk and compared that way. Capital-versus-small letters, a shortcut (symbolic link) standing in for the real file, a location written the short way against the same location written the long way, a drive letter against a network name — none of these are addressed for libraries.

### A note on the reading of the question

I took "library" to mean a linked blend-file — the thing Blender calls a Library data-block, which is what the rest of this sheet is about. The other possible reading is the Asset Library, which is a folder listed in Preferences rather than something recorded in the file. The question says "recorded", which points at the first reading, so that is the one I answered.

**The second reading is not settled here.** The searches recorded below were aimed at libraries in the first sense, and no Asset Library page or Asset Library search term is among them — so this document does not establish that the sources are silent about Asset Library identity, only that it did not look. The one incidental remark met along the way is in the 4.5 release notes' list of fixed bugs: "Mac version blender crashes when Asset Libraries path contains BACK SLASH "\"" (release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30). That is a crash fix, not a rule about identity.

### What the sources do say nearby

None of the following answers the question. I list it because it is the closest the sources come, and because each piece constrains what an answer could look like.

One thing to hold while reading it: the quotes are what the sources say, and the sentences between them are sometimes a step across from a quote rather than a repeat of it. Those steps are **inferred** and are marked as such where they occur.

**A recorded location is a plain string that may be written relative.** From the API documentation:

*(Changed by the citation check, 2026-08-09: this used to open "A library is a data-block, and a recorded location is…". The two quotes below carry the string half only; that a library is a data-block comes from the manual quote further down this answer, not from these words.)*

> "External .blend file from which data is linked"

> "filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Library.html, read 2026-07-30.

**Whether that string is written relative or absolute is a choice made at the moment of linking.** The Link operator carries an option:

> "relative_path (boolean, (optional)) – Relative Path, Select the file relative to the blend file"

Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30 (under `bpy.ops.wm.link`).

The manual describes the same choice in words:

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

Manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30.

So the same file on disk can be recorded in more than one shape — **inferred**, the step being that an option offered at link time can be taken either way for the same file. The quotes say the option exists; they do not say this. The sources do not say what Blender makes of it either.

**Data-block names are unique per type per file, and Library is one of the types.** From the manual:

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."

and, from the table of data-block types on the same page:

> "Library
>
> ✗
>
> ✓
>
> References to an external blend-file. Access from the Outliner's Blender File view."

Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30.

Two Library data-blocks inside one file therefore cannot carry the same name — **inferred**, the step being to put the general rule about namespaces together with the table row showing that Library is one of the data-block types. Neither quote says it in so many words. And neither says what a Library data-block's name is, nor that the name is the recorded location. So it cannot be used to settle the question.

**When Python looks a linked data-block up, it is given the library by its recorded location.** From the API documentation's page of pitfalls, under the heading "Library Collisions":

> "# Library object name lookup using a pair,
> # where the second argument is the library path matching bpy.types.Library.filepath.
> obj = bpy.data.objects["my_obj", "//my_lib.blend"]"

Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html, read 2026-07-30.

The example is written with the relative form, `//my_lib.blend`, and the comment says the argument matches `bpy.types.Library.filepath` — that is, the location as recorded. What "matching" means is not defined: the page does not say whether the comparison is letter-for-letter, nor what happens if the caller supplies the same file written a different way. That is exactly the gap the question asks about.

The same page also warns that names can collide once library data is linked in, and advises against referring to data by name at all:

*(Changed by the citation check, 2026-08-09: this used to read "states that a linked data-block's identity is not its name alone". The page does not say that; it says collisions can occur and that referencing by name is best avoided.)*

> "Blender keeps data names unique (bpy.types.ID.name) so you can't name two objects, meshes, scenes, etc., the same by accident. However, when linking in library data from another blend-file naming collisions can occur, so it's best to avoid referencing data by name at all."

Same page and date. And the API documentation for data-blocks in general:

> "name_full
>
> Unique data-block ID name, including library one is any"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ID.html, read 2026-07-30.

Taken together these point at the library forming part of a linked data-block's full identity — **inferred**, and on soft ground: the words quoted from the `name_full` entry ("including library one is any") do not parse as they stand, so reading them as *the library's name is included in the full name* is a repair as much as a reading. They still do not say what makes two recorded locations one library.

**There is a separate, documented mechanism that carries a recorded location, but it is for appended data, not for libraries.** The API documentation describes a weak reference carrying exactly two things — a location and a name:

> "Read-only external reference to a linked data-block and its library file"

> "filepath
>
> Path to the library .blend file"

> "id_name
>
> Full ID name in the library .blend file (including the two leading 'id type' chars)"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html, read 2026-07-30.

Its purpose, from the data-block page:

> "Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ID.html, read 2026-07-30.

And an option worded to the same effect — that it is the one driving the mechanism above is **inferred**, since neither entry names the other:

> "do_reuse_local_id (boolean, (optional)) – Re-Use Local Data, Try to re-use previously matching appended data-blocks instead of appending a new copy"

Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30 (under `bpy.ops.wm.link`).

This is about deciding whether a thing you are appending is the thing you already appended. It is not about deciding whether two libraries are one library, and I am not carrying it across. Again, "matching" is not defined.

**Nothing in Cycles' own files bears on this.** Searching those files for "librar" turns up two places. The first is version handling, and these are the lines:

> `    # Map of versions used by libraries.`
> `    library_versions = {}`
> `    library_versions[bpy.data.version] = [None]`
> `    for library in bpy.data.libraries:`
> `        library_versions.setdefault(library.version, []).append(library)`

`cycles_4.5/version_update.py`, lines 97–101, read 2026-07-30. Read as text, these lines name a library's `version` and nothing else — no recorded location appears in them, and no comparison of one location with another. What they amount to in a running Blender is a step beyond reading them, and nothing of the kind is claimed here.

The second place is `cycles_4.5/osl.py`, where five lines mention a library and two of them hand one to `bpy.path.abspath` — lines 169 and 209. Those lines are not quoted in this document, so nothing is claimed from them either way.

*(Changed by the citation check, 2026-08-09: this used to say "four lines … their place in the file is not recorded". Five lines of that file mention a library, two of them use `bpy.path.abspath` with a `library=` argument, and their line numbers are now given.)*

### Not covered

This answer does not settle any of the following, because the sources are silent on all of them: whether two recorded locations are compared letter-for-letter or after being reduced to some standard form; whether a location is turned into a real file on disk before comparing; whether the relative form and the absolute form of the same file count as one library; whether difference in capital and small letters matters, and whether that differs by operating system; what happens with a shortcut (symbolic link), a drive letter against a network name, or the same folder reached two ways; whether two libraries found to point at the same file are ever merged into one; and what the Relocate Library and Reload Library commands do about identity when the new location happens to equal an existing one. It also does not settle what a Library data-block's name is, which may or may not be the mechanism at work.

It does settle that the question is not answerable from the sources I was given for 4.5 — the manual, the Python API documentation, the release notes and Cycles' own files. It must be established by trying it.

### Where I looked

- Read in full: the manual's Link & Append page (https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html), covering Link, Append, Reload Library, Relocate Library, Relocate Linked ID, Make Local and Known Limitations; the linked-libraries index; the Data-Blocks page (https://docs.blender.org/manual/en/4.5/files/data_blocks.html), including the "Name & Rename" section; the Open & Save page's "Relative Paths" section (https://docs.blender.org/manual/en/4.5/files/blend/open_save.html); the File Paths page (https://docs.blender.org/manual/en/4.5/files/file_paths.html); the Top Bar's External Data section, covering Make Paths Relative, Make Paths Absolute, Report Missing Files and Find Missing Files (https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html); the Outliner's Display Mode list (https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html).
- Read in full in the API documentation: `bpy.types.Library`, `bpy.types.BlendDataLibraries`, `bpy.types.LibraryWeakReference`, `bpy.types.BlendImportContextLibrary`, `bpy.types.BlendImportContextLibraries`, `bpy.path`, the "Library Collisions" section of the internal-data pitfalls page, and the `bpy.ops.wm.link`, `bpy.ops.wm.lib_reload` and `bpy.ops.wm.lib_relocate` entries.
- Read in full: the release notes for the core (https://developer.blender.org/docs/release_notes/4.5/core/), whose only linking entry is "Linked IDs can now also be relocated, if they are not a dependency of any other linked data", and the pipeline and I/O notes (https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/), which are entirely about USD, glTF and FBX. Searched the whole release-notes folder for "librar".
- Looked at the one picture the sources offer here, `_images/files_data-blocks_outliner-blender-file-view.png`, shown on the Data-Blocks page (https://docs.blender.org/manual/en/4.5/files/data_blocks.html), read 2026-07-30. **Documented, from a picture.** Visible in it: the Outliner's display-mode menu, offering Scenes, View Layer, Sequence, Blender File (highlighted), Data API and Orphan Data; and below it a Blender File listing whose only top-level row reads "Current File", with the data-block types under it (Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds). No linked library row appears anywhere in it. **Inferred** from that absence: the picture settles nothing about how libraries are listed or named. *(Changed by the citation check, 2026-08-09: the picture-finding now names the page, says what is visible, and carries the two labels the Method asks for.)*
- Searched the whole manual, the whole API documentation and the whole release notes for, in each case finding nothing that bears on library identity: "same library", "same path", "same file", "same blend", "already linked", "already loaded", "duplicate librar", "two librar", "separate librar", "distinct librar", "library identity", "identify the library", "library path", "libpath", "library file path", "matching library", "canonical", "normaliz", "identical", "unique", "deduplicat", "merge", "consolidated", "loaded once", "load once", "only once", "single instance", "one entry", "one library", "per library", "twice", "linked twice", "symlink", "symbolic link", "case sensitiv", "case-sensitiv", "resolved to", "compares", "compared", "equality", "equal paths", "lookup", "look up", "relative path", "absolute path".
- Searched Cycles' own files for "librar". The hits are the version-handling lines quoted above, four lines in `cycles_4.5/osl.py` where a data-block's library is named alongside `bpy.path.abspath` — not quoted here, and their place in the file not recorded — and a handful of web addresses inside research citations in the kernel source that merely contain the word.

### Outside the interface

The sources do not say. Nothing I found describes library identity at all, so there is nothing to hold or fail to hold when Blender is driven from outside its own window. I will note only what the material itself is: the pair-lookup form `bpy.data.objects["my_obj", "//my_lib.blend"]` is part of the Python API rather than a menu command, and the manual's Relocate and Reload commands are described as Outliner context-menu items. Whether the underlying decision differs between the two is not stated anywhere.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.


### Operating system

The sources do not say. There is no statement about library identity on any operating system, and therefore none about differences between them. Two nearby remarks show only that Blender's documentation is aware that filesystems differ — the Python utility `bpy.path.resolve_ncase` is described as "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30), and `bpy.path.basename` is described as "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." (same page and date). Neither is about libraries, and neither says how Blender compares two recorded library locations.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 4.5.12 LTS, at `/opt/blender/4.5/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 4.5.12 LTS, at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.

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

**The same fifteen tests were run in 5.0.1, 5.1.2 and 5.2.0 LTS as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-4.5-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-4.5-ops/libB.blend'
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
HT|paths|plain='/root/ht-deps/e3-4.5-data/lib.blend'|dotted='/root/ht-deps/e3-4.5-data/./lib.blend'|updown='/root/ht-deps/e3-4.5-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-4.5-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-4.5-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-4.5-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-4.5-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-4.5-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-4.5-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808457 /root/ht-deps/e5-4.5-data/lib.blend` and `1808457 /root/ht-deps/e5-4.5-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-4.5-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-4.5-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-4.5-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-4.5-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-4.5-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-4.5-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-4.5-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-4.5-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-4.5-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-4.5-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-4.5-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-4.5-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-4.5-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-4.5-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-4.5-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-4.5-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-4.5-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-4.5-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-4.5-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-4.5-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-4.5-data/lib.blend'
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
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-4.5-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-4.5-data/one/mid.blend'|abspath='/root/ht-deps/e10-4.5-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-4.5-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-4.5-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place 4.5.12 LTS differs from the other three versions

*Done:* ask whether the three archive properties exist, then link with packing asked for.

On both computers, Blender 4.5.12 LTS printed:

```
HT|prop|is_archive|present=False
HT|prop|archive_parent_library|present=False
HT|prop|archive_libraries|present=False
HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")
HT|after-pack|library_count|0
```

The same test in 5.0.1, 5.1.2 and 5.2.0 LTS printed `present=True` for all three properties, and the packing call was accepted and produced two library entries holding the same recorded text, one of them an archive of the other. None of that happens in 4.5.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-4.5-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-4.5-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-4.5-data/b.blend'
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

## Still open for a person with Blender 4.5 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 4.5

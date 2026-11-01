# Dependencies — later questions, third pass (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own files — all as supplied inside `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is my date read for everything cited below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering question 19 only, as instructed.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**A note on the question first.** "Library" has two possible meanings in Blender's documentation: (a) a linked `.blend` file whose path is recorded inside another `.blend` file, and (b) an "asset library", a folder registered in Preferences. I read the question as (a), because the sheet is about what a `.blend` file points at outside itself, and a `.blend` file records linked-`.blend` locations but does not record Preferences entries. I say what the sources give for reading (b) at the end, so the other reading is not lost.

**LABEL: partly documented, partly inferred, and on the central point — not in the sources.**

### What the sources do say

A library is a data-block, and data-block names are unique inside a blend-file:

> "Library
>
> ✗
>
> ✓
>
> References to an external blend-file.
> Access from the Outliner's Blender File view."

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."

Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html — read 2026-07-30.

A library data-block carries one recorded path:

> "filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Python API, https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.

The library is part of what makes a linked data-block's identity, and the thing that stands for the library in that identity is the recorded path string:

> "name
>
> Unique data-block ID name (within a same type and library)"

> "name_full
>
> Unique data-block ID name, including library one is any"

Python API, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30. (The wording "including library one is any" is the documentation's own; it appears to be a typo for "if any".)

The clearest statement of what stands in for "which library" is in the API's gotchas page, under the heading "Library Collisions":

> "Blender keeps data names unique (bpy.types.ID.name) so you can't name two objects,
> meshes, scenes, etc., the same by accident.
> However, when linking in library data from another blend-file naming collisions can occur,
> so it's best to avoid referencing data by name at all."

> "# Library object name lookup using a pair,
> # where the second argument is the library path matching bpy.types.Library.filepath.
> obj = bpy.data.objects["my_obj", "//my_lib.blend"]"

> "# Local object name look up using a pair,
> # where the second argument excludes library data from being returned.
> obj = bpy.data.objects["my_obj", None]"

Python API, https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html — read 2026-07-30.

**Inferred from those quotes:** when Blender has to tell one library apart from another, the thing it goes on is the recorded path string held in `Library.filepath`. The step I took: the documentation gives no other property of a library that could serve as an identifier (the only other stored fields on the type are `is_editable`, `needs_liboverride_resync`, `packed_file`, `parent` and `version`, and none is described as identifying), and it gives the path string — in the example, in its stored relative `//` form — as the second half of the key that distinguishes an object in one library from an identically named object in another. So: the recorded location *is* what names the library. That is an inference, not a statement in the source; the source shows the path used as a key, it does not say "two libraries are the same when their paths are the same."

### What the sources do not say — the actual question

Nothing I could find states **how two recorded locations are compared**. Specifically, none of these is answered anywhere in the manual, the API documentation, the release notes or Cycles' files:

- Whether the comparison is made on the string as recorded, or on the location after it has been worked out. So whether `//assets/char.blend` and `/home/me/proj/assets/char.blend`, pointing at the same file on disk, become one library or two, is not stated.
- Whether the comparison ignores or respects the difference between capital and small letters, or forward and backward slashes, or `.` and `..` steps inside a path.
- Whether a symbolic link, a shortcut, a mounted-twice folder, or the same server folder reached by two different names counts as one file.
- Whether a library reached directly and the same library reached indirectly (through another library) end up as one library data-block or two. The type has a `parent` field — "parent / Type: Library, (readonly)" (https://docs.blender.org/api/4.5/bpy.types.Library.html) — showing libraries can stand in a chain, but nothing says what happens when the same file appears at two places in that chain.
- What happens if two recorded locations that are different strings are found to be the same file, or two identical strings are found to be different files.

The documentation comes close in several places without ever settling it. The clearest near-misses:

> "APPEND_RECURSIVE
> Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

Python API, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30. It uses "the same initial library" as if the meaning were obvious, and never defines it.

> "source_libraries
>
> List of libraries to search and import that ID from. The ID will be imported from the first file in that list that contains it"

Python API, https://docs.blender.org/api/4.5/bpy.types.BlendImportContextItem.html — read 2026-07-30. This settles the order of searching several files, not whether two of them could be the same library.

> "library (string, (optional, never None)) – Library, Library to relocate"

and

> "Relocate the given library to one or several others"

Python API, https://docs.blender.org/api/4.5/bpy.ops.wm.html (operator `bpy.ops.wm.lib_relocate`) — read 2026-07-30. A library is addressed here by a plain string, which fits the inference above, but the page does not say what that string is matched against or how.

> "Relative Paths
>
> Many blend-files reference external images or other linked blend-files.
> A path tells Blender where to look for these files."

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30. This tells you both forms exist and how a relative one is meant, and says nothing about two forms of the same location meeting each other.

> "library (bpy.types.Library) – The library this path is from. This is only included for
> convenience, when the library is not None its path replaces start."

Python API, https://docs.blender.org/api/4.5/bpy.path.html (`bpy.path.abspath`) — read 2026-07-30. This documents that a path recorded *inside* a library is worked out against that library's own path. It is about paths held by a library, not about telling libraries apart.

### The other reading of "library" (asset libraries in Preferences)

For completeness, and only because the word is ambiguous:

> "An asset library is a directory on your drive that is registered in the Preferences as an asset library.
> Registering it means that you give the library a name (like "Sprite Fright")"

Manual, https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html — read 2026-07-30.

> "name
>
> Identifier (not necessarily unique) for the asset library"

> "path
>
> Path to a directory with .blend files to use as an asset library"

Python API, https://docs.blender.org/api/4.5/bpy.types.UserAssetLibrary.html — read 2026-07-30.

**Documented** that the name of a registered asset library is explicitly "not necessarily unique", so the name alone cannot be what tells two of them apart. What *does* tell two registered asset libraries apart, and what happens if two entries name the same folder, is **not in the sources**.

### Not covered

My answer does not settle the question that was asked. It establishes what the recorded location is and that it acts as the key that names a library, but not the rule by which two recorded locations are judged the same or different. It says nothing about what happens at load time when a file records the same library twice in two spellings, nor about the direct-versus-indirect case, nor about links and mounts. It also does not cover libraries whose contents have been packed into the file (`Library.packed_file` exists, and the manual documents "Pack Linked Libraries", but nothing says whether a packed library is matched by path at all).

### Outside the interface

The facts I did find are properties of the stored data and of the Python API — `Library.filepath`, `ID.name`, the `(name, library path)` lookup pair, the uniqueness of data-block names within a blend-file. Nothing in those pages ties them to a window being open, and the lookup shown is plain Python. So on the documented parts, I have no reason from the sources to think they differ when Blender is driven from outside. But the sources do not state this either way, and they do not say whether library matching behaves differently in background mode. What the sources *do* show is that some library handling is described only as an interface action — "Relocate the library under cursor" (`bpy.ops.outliner.lib_relocate`, https://docs.blender.org/api/4.5/bpy.ops.outliner.html) and the Outliner's Blender File view context menu (https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html) — while the equivalent `bpy.ops.wm.lib_relocate` and `bpy.ops.wm.lib_reload` take the library as a string parameter. One command-line switch touching linked data exists: "--disable-liboverride-auto-resync / Do not perform library override automatic resync when loading a new blend-file" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html), which concerns overrides, not library identity. On the central question — the comparison rule — the sources are silent inside the interface too, so there is nothing to differ.

### Operating system

The sources do not say. I found no statement anywhere that library matching is case-sensitive, or that it treats slashes, drive letters or network paths differently on one operating system than another. The nearest things in the sources are not about library identity: the 4.5 release notes list a fixed bug, "Mac version blender crashes when Asset Libraries path contains BACK SLASH "\"" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/), which is about asset library paths on macOS; and the OBJ and FBX exporter pages mention relative paths being used "in every case (except when on a different drive on Windows)" (https://docs.blender.org/manual/en/4.5/files/import_export/obj.html), which is about writing export paths, not about libraries. Both read 2026-07-30.

---

## Record of where I looked

Everything below is inside `Blender Sources/`, in the 4.5 folders only.

**Whole-tree searches** (manual, API documentation and release notes together, case-insensitive): `same library`, `two libraries`, `duplicate librar`, `unique librar`, `identified by`, `canonical`, `normalize`, `normalise`, `distinct`, `separate librar`, `single library`, `one library`, `two different`, `merge`, `deduplicat`, `twice`, `already linked`, `already exist`, `existing librar`, `reuse`, `same file`, `same blend`, `same path`, `same filepath`, `same library file`, `different librar`, `library path`, `case sensitive`, `case-sensitive`, `symlink`, `symbolic link`, `shortcut`, `backslash`, `drive letter`, `UNC`, `trailing slash`, `identity`, `instance of the library`, `per-library`, `relative path`, `librar` (as a stem, across the release notes and across several manual folders).

**Pages opened in full or in part, and what each ruled out:**

- Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — the main Link/Append page, including Reload Library, Relocate Library, Relocate Linked ID, Make Local and Known Limitations. Describes linking, relocating and broken libraries; never says how two recorded locations are compared.
- Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/index.html — contents only; the remaining page under it is Library Overrides, which is about editing linked data, not about identifying libraries.
- Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html — read whole. Gives the naming namespace rule and the table entry for the Library type. Nothing on path comparison.
- Manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — Relative Paths and Remap Relative sections. Nothing on comparison.
- Manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read whole. Turned out to be entirely about path templates such as `{blend_name}`; ruled out.
- Manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — Pack/Unpack Linked Libraries. Says libraries can be packed; nothing on identity.
- Manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — the File ‣ External Data menu (Make Paths Relative/Absolute, Report Missing Files, Find Missing Files). Nothing on identity.
- Manual, https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html — the Blender File display mode description. Nothing on identity.
- Manual, https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html — asset libraries, for the second reading of the word.
- Manual, https://docs.blender.org/manual/en/4.5/glossary/index.html — searched for a definition of "Library"; the only library entries are about asset libraries. Ruled out.
- Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — searched for library-related switches; only the override auto-resync one and unrelated debug switches.
- Manual, https://docs.blender.org/manual/en/4.5/physics/baking.html — reached through the phrase "library path"; it is the physics cache "Use Library Path" option, unrelated to library identity. Ruled out.
- Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/python.html — reached through "duplicate libraries"; it is about Python DLLs, not blend libraries. Ruled out.
- Python API, https://docs.blender.org/api/4.5/bpy.types.Library.html — read whole. The complete list of what a library stores.
- Python API, https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html — read whole. A weak reference is "Path to the library .blend file" plus "Full ID name in the library .blend file"; this is for re-using already appended data, not for deciding library sameness.
- Python API, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html — read whole (`load`, `write`, `remove`). Nothing on identity.
- Python API, https://docs.blender.org/api/4.5/bpy.types.ID.html — the `name`, `name_full`, `library` and `library_weak_reference` entries.
- Python API, https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html — the "Library Collisions" section. The single most relevant page found.
- Python API, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html, .../bpy.types.BlendImportContextItem.html, .../bpy.types.BlendImportContextLibrary.html, .../bpy.types.BlendImportContextLibraries.html — the import-handler data added for library operations. Mention "the same initial library" and a list of source libraries, but define neither.
- Python API, https://docs.blender.org/api/4.5/bpy.ops.wm.html — `lib_reload` and `lib_relocate`.
- Python API, https://docs.blender.org/api/4.5/bpy.ops.outliner.html — the library operations in the Outliner context menu.
- Python API, https://docs.blender.org/api/4.5/bpy.path.html — `abspath` and its `library` argument.
- Python API, https://docs.blender.org/api/4.5/bpy.types.UserAssetLibrary.html and .../bpy.types.AssetLibraryReference.html — for the asset-library reading.
- Python API, https://docs.blender.org/api/4.5/bpy.data.html — the top of the data-access module. Nothing on library keys.
- Release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read whole. The only linking item is "Linked IDs can now also be relocated, if they are not a dependency of any other linked data"; nothing on library identity.
- Release notes, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read the first sections; USD, glTF and FBX import/export only. Ruled out.
- Release notes, https://developer.blender.org/docs/release_notes/4.5/python_api/ — searched for "librar" and "link"; no hits. Ruled out.
- Release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — searched for "librar"; the hits are override, asset-system and `bpy.data.libraries.load` crash fixes, none about identity.

**Picture opened:** `_images/files_data-blocks_outliner-blender-file-view.png`, the Blender File view of the Outliner, referenced from https://docs.blender.org/manual/en/4.5/files/data_blocks.html. It shows the display-mode menu and a "Current File" tree with data-block categories (Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds). No library entry is visible in it, so it does not show how libraries are listed or labelled. Ruled out.

**Cycles' own files:** searched the whole of `cycles_4.5` for "librar". The only real hits are `version_update.py` (which groups `bpy.data.libraries` by their `version` field to run per-library versioning) and `osl.py` (which calls `bpy.path.abspath(filepath, library=library)` to resolve an OSL script path recorded in a linked file). Neither compares two library locations or decides sameness. Every other hit is the word "library" inside a web address in a source comment. Ruled out.

---

## Things a person with Blender open could settle in a few minutes

1. **Does the same file recorded two ways become one library or two?** Save `lib.blend` containing an object named `A` and another named `B`, in a folder next to a new file `main.blend`. In `main.blend`, use File ‣ Link with the Relative Path box ticked and bring in `A`. Then use File ‣ Link again with the Relative Path box unticked and bring in `B` from the same `lib.blend`. Open the Outliner, set its display mode to Blender File, and count how many entries for `lib.blend` appear. Then in the Python console run `for L in bpy.data.libraries: print(repr(L.name), repr(L.filepath))` and record every line printed. One entry means the two spellings were merged; two entries means they were not.

2. **Does letter case decide it?** Repeat step 1 on the same computer, but for the second link type the path by hand into the File Browser's path field with the folder or file name in different capitalisation (for example `LIB.blend` on a system that accepts it). Print the same list. Do this once on Windows or macOS and once on Linux, and compare — this is the point where the answer is most likely to differ by operating system.

3. **Does a link reached two ways stay one library?** Make `middle.blend` link a collection from `lib.blend`. Then in `main.blend`, link that collection from `middle.blend` (so `lib.blend` arrives indirectly), and additionally link an object directly from `lib.blend`. Print `for L in bpy.data.libraries: print(repr(L.filepath), L.parent)` and see whether `lib.blend` appears once or twice, and what `parent` holds for each.

4. **Does a symbolic link count as the same file?** Make a symbolic link `lib_alias.blend` pointing at `lib.blend`. Link `A` through `lib.blend` and `B` through `lib_alias.blend` into the same `main.blend`. Print the library list as above. Repeat on each operating system, since links are made differently on each.

5. **Does any of this change without a window?** Save the file from step 1, then run Blender from a command line with `--background` and a short Python script that opens it and prints the same library list. Compare the printed lines with what the Outliner showed. This checks whether the interface and the background run agree.

6. **What does the relocate operation do to two entries?** If step 1 produces two entries for the same file, right-click one in the Outliner's Blender File view, choose Relocate, and pick the very same `lib.blend` again. Print the library list afterwards and see whether the two entries have collapsed into one.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 4.5

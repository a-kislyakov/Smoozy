Sources consulted:
- Blender 5.3 Manual (`manual_5.3`), read on 2026-10-11
- Blender 5.3 Python API Documentation (`api_5.3`), read on 2026-10-11
- Blender 5.3 Release Notes (`release_notes_5.3`), read on 2026-10-11
- Cycles' own files for Blender 5.3 (`cycles_5.3`), read on 2026-10-16

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

### 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**documented** (with file-saving persistence inferred — note added by citation check 2026-10-17)

**Not covered**: Does not settle whether external command-line tools or third-party binary utilities exist that can read or rewrite recorded paths directly in a `.blend` file on disk without starting Blender. Does not settle whether directly modifying `Library.filepath` in Python without invoking library relocate operators handles complex recursive dependency trees and library override resyncing without data corruption (the API explicitly warns that changing this path without proper processing afterwards can leave data in an invalid state).

Yes. The locations recorded in a `.blend` file for the things it points at—specifically external linked library `.blend` files and unpacked external resource files (images, sounds, movie clips, cache files, fonts, volumes, and texts)—are stored as file paths (either absolute, or relative to the referencing `.blend` file using the `//` prefix). These locations can be read out and written back through several mechanisms:

#### By What Locations Can Be Read Out

1. **Python API (`bpy.data`)**:
   - **Linked Libraries**: Each library entry in `bpy.data.libraries` exposes the `filepath` property (`bpy.types.Library.filepath`), which contains the path to the external linked `.blend` file.
   - **External Resource Data-Blocks**: Data-blocks referencing external files expose their recorded path via a `filepath` string property: `bpy.data.images[...].filepath`, `bpy.data.sounds[...].filepath`, `bpy.data.movieclips[...].filepath`, `bpy.data.cache_files[...].filepath`, `bpy.data.volumes[...].filepath`, `bpy.data.fonts[...].filepath`, and `bpy.data.texts[...].filepath`.
   - **Library Inspection Context Manager**: `bpy.data.libraries.load(filepath)` provides a context manager whose input library object exposes a `libraries` attribute containing a list of named tuples `(filepath, is_archive)` of all other libraries referenced by the loaded `.blend` file.
2. **Graphical User Interface**:
   - **The Outliner**:
     - In **Blender File** display mode, the Outliner lists all linked libraries and unpacked external data-blocks.
     - In **Data API** display mode, it lists every data-block and its full property tree, including `filepath`.
   - **Editor Panels & Properties**:
     - The Image Editor, Shader Editor (Image Texture node), Movie Clip Editor, Font settings in Text objects, Cache File modifiers/constraints, and Sound properties display text fields with the recorded file path of each referenced asset.

#### By What Locations Can Be Written Back

1. **Operators and Menus**:
   - **Batch Relative/Absolute Conversion**:
     - `bpy.ops.file.make_paths_relative()` (Menu: `File ‣ External Data ‣ Make Paths Relative`): Rewrites all paths to external files so they are relative to the current `.blend` file.
     - `bpy.ops.file.make_paths_absolute()` (Menu: `File ‣ External Data ‣ Make Paths Absolute`): Rewrites all paths to external files to absolute paths starting from the system root.
   - **Search and Recovery**:
     - `bpy.ops.file.find_missing_files(...)` (Menu: `File ‣ External Data ‣ Find Missing Files`): Recursively searches a designated folder and rewrites missing references to absolute paths where matches are found.
   - **Relocating Libraries and Linked IDs**:
     - `bpy.ops.outliner.lib_relocate()` / `bpy.ops.wm.lib_relocate(...)` (Outliner: `Context menu ‣ Relocate` on a library): Replaces a linked library `.blend` file with a new file path and reloads its data.
     - `bpy.ops.outliner.id_linked_relocate()` / `bpy.ops.wm.id_linked_relocate(...)` (Outliner: `Context menu ‣ ID Data ‣ Relocate` on a linked ID): Replaces a linked data-block with another from the same or a different library.
   - **Save As Remapping**:
     - `bpy.ops.wm.save_as_mainfile(relative_remap=True)` (Menu: `File ‣ Save As...` with `Remap Relative` enabled): Remaps relative paths when saving the file to a different directory.
2. **Python API Property Assignment**:
   - Setting the `filepath` string attribute directly on external data-blocks (e.g. `image.filepath = "/path/to/new.png"`), followed by reloading (`image.reload()`).
   - Setting `library.filepath` on a `Library` data-block (e.g. `library.filepath = "//other.blend"`), followed by `library.reload()`.
   - `bpy.data.libraries.write(filepath, datablocks, path_remap=...)`: Writes specified data-blocks to a new `.blend` file with path remapping options (`NONE`, `RELATIVE`, `RELATIVE_ALL`, `ABSOLUTE`).
   - Path helper functions in `bpy.path` (`abspath`, `relpath`, `clean_name`, `resolve_ncase`).
   - Saving the `.blend` file to disk (`File ‣ Save` or `bpy.ops.wm.save_mainfile()`) commits the modified paths in memory back to the file on disk. *(Inferred: documented sources describe save operations, but do not state path persistence in memory-to-disk terms — note added by citation check 2026-10-17)*

#### Citations and Exact Quotes

- From manual page `https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html` (read 2026-10-11):
  - "External Data: External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path."
  - "Make Paths Relative: Make all paths to external files relative to the current blend-file."
  - "Make Paths Absolute: Make all paths to external files absolute (= full path from the system’s root)."
  - "Find Missing Files: In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."
- From manual page `https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html` (read 2026-10-11):
  - "When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."
  - "Relocate Linked ID ... A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one."
- From manual page `https://docs.blender.org/manual/en/5.3/files/blend/open_save.html` (read 2026-10-11):
  - "Remap Relative: This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."
  - "Relative Paths: Many blend-files reference external images or other linked blend-files. A path tells Blender where to look for these files. ... Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.types.Library.html` (read 2026-10-11):
  - "filepath: Path to the library .blend file. WARNING: Typically, this value should be considered read-only, changing this path without proper processing afterwards can leave Blender data in an invalid state (default “”, never None, blend relative // prefix supported)"
  - "reload(): Reload this library and all its linked data-blocks"
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html` (read 2026-10-11):
  - "load(filepath...): Each library object contains: ... A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive)."
  - "write(filepath, datablocks, *, path_remap='NONE', fake_user=False, compress=False): Optionally remap paths when writing the file: NONE No path manipulation (default). RELATIVE Remap paths that are already relative to the new location. RELATIVE_ALL Remap all paths to be relative to the new location. ABSOLUTE Make all paths absolute on writing."
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.ops.file.html` (read 2026-10-11):
  - "bpy.ops.file.make_paths_absolute(): Make all paths to external files absolute"
  - "bpy.ops.file.make_paths_relative(): Make all paths to external files relative to current .blend"
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.ops.wm.html` (read 2026-10-11):
  - "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, ...): Relocate the given library to one or several others"
  - "bpy.ops.wm.save_as_mainfile(*, filepath='', ... relative_remap=True, ...): Save the current file in the desired location"

#### Record of Where Looked

- Searched `Blender Sources/manual_5.3/`:
  - Terms: `external data`, `relative paths`, `absolute paths`, `relocate`, `remap`, `filepath`, `blendfile`.
  - Pages opened and examined:
    - `manual_5.3/interface/window_system/topbar.txt`: Identified "External Data" submenus, `Make Paths Relative`, `Make Paths Absolute`, `Report Missing Files`, `Find Missing Files`.
    - `manual_5.3/files/linked_libraries/link_append.txt`: Identified Outliner library and linked ID relocation options and broken library handling.
    - `manual_5.3/files/blend/open_save.txt`: Examined `Remap Relative` functionality and relative path prefix `//`.
    - `manual_5.3/files/blend/packed_data.txt`: Examined options for unpacking resources to local vs original paths.
    - `manual_5.3/files/data_blocks.txt`: Verified data-block types that support external file references and linking.
    - `manual_5.3/advanced/command_line/arguments.txt`: Checked command-line options; confirmed there are no standalone CLI arguments for directly inspecting or modifying `.blend` file paths without executing Python scripts.
- Searched `Blender Sources/api_5.3/`:
  - Terms: `bpy.data.libraries`, `Library.filepath`, `make_paths_relative`, `make_paths_absolute`, `relocate`, `bpy.path`, `write`, `load`.
  - Pages opened and examined:
    - `api_5.3/bpy.types.Library.txt`: Verified `filepath` property, warning notice, and `reload()`.
    - `api_5.3/bpy.types.BlendDataLibraries.txt`: Verified `load()` and `write()` methods and path remapping options.
    - `api_5.3/bpy.ops.file.txt`: Checked definitions and signatures of `make_paths_relative`, `make_paths_absolute`, and `find_missing_files`.
    - `api_5.3/bpy.ops.wm.txt`: Checked `lib_relocate` and `save_as_mainfile(relative_remap=True)`.
    - `api_5.3/bpy.ops.outliner.txt`: Checked `lib_relocate` and `id_linked_relocate`.
    - `api_5.3/bpy.path.txt`: Reviewed utility functions for path conversion (`abspath`, `relpath`, `clean_name`, `resolve_ncase`).
- Searched `Blender Sources/release_notes_5.3/`:
  - Terms: `library`, `external`, `filepath`, `relocate`. Found asset library and catalog updates; ruled out new path modification operators in 5.3.
- Searched `Blender Sources/cycles_5.3/`:
  - Terms: `filepath`, `missing`. Ruled out Cycles source files as having any role in reading or rewriting `.blend` file locations.

**Outside the interface**:
Holds. When Blender is executed in background mode without a window (`blender --background <file.blend>`), Python scripts can read and write all `filepath` properties on data-blocks in `bpy.data`, execute operators such as `bpy.ops.file.make_paths_relative()`, `bpy.ops.file.make_paths_absolute()`, `bpy.ops.file.find_missing_files(directory=...)`, and `bpy.ops.wm.lib_relocate(...)`, and commit the changes to disk via `bpy.ops.wm.save_mainfile()`.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window on the Creator's Mac, and recorded in full under questions 5 and 9 of `Preparation/Blender Research/Findings/5.3/Reading a file.md`: assigning an image's `filepath` and saving held after reopening (test 12); `make_paths_relative()`, `make_paths_absolute()`, `find_missing_files(directory=…)`, `lib_relocate(…)` and `save_mainfile()` all ran (tests 14, 21, 22, 31). Assigning `Library.filepath` alone changed the path the objects report, and the new file's data came in only after `Library.reload()` (test 31). A rewrite not followed by a save was gone in the next run (test 24).

**Operating system**:
Holds across all operating systems Blender runs on (Linux, macOS, Windows). Relative paths use `//` uniformly across platforms. The Python API module `bpy.path` provides cross-platform helpers such as `bpy.path.resolve_ncase` (to resolve case-insensitive paths on case-sensitive filesystems like Linux) and `bpy.path.native_pathsep` to format separators according to host OS conventions.

**Brought up to date by the hand tests of 2026-10-17.** The tests above were run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and settle macOS on Apple silicon for what they cover. No Linux or Windows machine ran them for 5.3, so for those two systems the line above has no test behind it.

---

### 6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?

**documented**

**Not covered**: Does not settle whether `Report Missing Files` outputs an itemized list of individual missing file paths to the system console/terminal in addition to displaying a summary warning in the Info editor header. Does not settle what rule `Find Missing Files` uses to break ties when multiple candidate files sharing the exact same filename exist across different subdirectories of the search tree.

Yes. Blender provides specific built-in operators and interface commands to perform all three tasks: reporting missing references, searching for them recursively across directories, and rewriting the recorded locations to the recovered files:

1. **Reporting which references are missing**:
   - **Report Missing Files** (`bpy.ops.file.report_missing_files()` / Menu: `File ‣ External Data ‣ Report Missing Files`):
     Checks for broken links to unpacked external files. If missing files are detected, a warning message appears in the Info editor’s header; if no warning appears, no unpacked external files are missing.
   - **Placeholder Data-Blocks for Broken Libraries**:
     When loading a `.blend` file that points to a missing linked library, Blender creates placeholder data-blocks to preserve references, and sets `ID.is_missing = True` (`bpy.types.ID.is_missing`) on each placeholder data-block.
   - **Scripted Inspection**:
     Via Python in `bpy.data`, a script can iterate through data-blocks (`bpy.data.libraries`, `images`, `sounds`, etc.) and test each recorded path against the filesystem using `bpy.path.abspath(item.filepath)` to produce an explicit list of missing files.

2. **Searching for missing references**:
   - **Find Missing Files** (`bpy.ops.file.find_missing_files(...)` / Menu: `File ‣ External Data ‣ Find Missing Files`):
     Prompts for a directory via the File Browser (or takes a `directory` parameter when invoked via Python), and performs a recursive search through all contained subdirectories.
     It includes options to search all files (`find_all=True`) or only missing files, and allows filtering by file type (`filter_blender`, `filter_image`, `filter_sound`, `filter_movie`, `filter_font`, `filter_volume`, `filter_alembic`, `filter_usd`, etc.).

3. **Rewriting recorded locations**:
   - **Automatic Path Updates via Find Missing Files**:
     When `Find Missing Files` locates a missing file, it recovers the reference and automatically rewrites the recorded location to an absolute path pointing to the newly found file. If relative paths are desired afterwards, `Make Paths Relative` can be executed.
   - **Relocating Missing Libraries**:
     For missing linked libraries, the user or script can invoke `bpy.ops.outliner.lib_relocate()` or `bpy.ops.wm.lib_relocate(library=..., filepath=...)` to specify the new file path, which updates the recorded library path and restores the placeholder data-blocks.
   - **Persisting Changes**:
     Saving the blend file (`File ‣ Save` or `bpy.ops.wm.save_mainfile()`) writes the rewritten locations permanently into the file on disk.

#### Citations and Exact Quotes

- From manual page `https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html` (read 2026-10-11):
  - "Report Missing Files: This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor’s header. If no warning is shown, there are no missing external files."
  - "Find Missing Files: In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."
  - "Note: Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once."
- From manual page `https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html` (read 2026-10-11):
  - "Broken Libraries: If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
  - "Relocate Library: When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.ops.file.html` (read 2026-10-11):
  - "bpy.ops.file.report_missing_files(): Report all missing external files"
  - "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, check_existing=False, filter_blender=False, filter_backup=False, filter_image=False, filter_movie=False, filter_python=False, filter_font=False, filter_sound=False, filter_text=False, filter_archive=False, filter_btx=False, filter_alembic=False, filter_usd=False, filter_obj=False, filter_volume=False, filter_folder=False, filter_blenlib=False, filemode=9, display_type='DEFAULT', sort_method=''): Try to find missing external files"
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.types.ID.html` (read 2026-10-11):
  - "is_missing: This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

#### Record of Where Looked

- Searched `Blender Sources/manual_5.3/`:
  - Terms: `report missing`, `find missing`, `missing files`, `missing`, `broken links`, `placeholder`.
  - Pages opened and examined:
    - `manual_5.3/interface/window_system/topbar.txt`: Found full descriptions of `Report Missing Files` and `Find Missing Files`.
    - `manual_5.3/files/linked_libraries/link_append.txt`: Found documentation on missing libraries, creation of placeholder data-blocks, and relocating missing libraries.
    - `manual_5.3/files/blend/open_save.txt`: Checked for automatic missing path handling on file opening.
- Searched `Blender Sources/api_5.3/`:
  - Terms: `find_missing_files`, `report_missing_files`, `is_missing`, `lib_relocate`.
  - Pages opened and examined:
    - `api_5.3/bpy.ops.file.txt`: Checked signatures and parameter sets for `bpy.ops.file.report_missing_files` and `bpy.ops.file.find_missing_files`.
    - `api_5.3/bpy.types.ID.txt`: Checked definition of `is_missing`.
    - `api_5.3/bpy.ops.wm.txt`: Checked `lib_relocate` operator documentation.
- Searched `Blender Sources/release_notes_5.3/`:
  - Terms: `missing`, `find_missing_files`. Reviewed `sequencer.md` (strip icons for missing media) and `user_interface.md` (missing icons for operators); confirmed no core alterations to missing file operations in 5.3.
- Searched `Blender Sources/cycles_5.3/`:
  - Terms: `missing`, `find_missing`. Verified Cycles has no missing file reporting or directory searching operators.

**Outside the interface**:
Holds. In background mode (`blender -b`), Python scripts can invoke `bpy.ops.file.report_missing_files()` and `bpy.ops.file.find_missing_files(directory="/path/to/search")`. Unlike inside the GUI where `Find Missing Files` opens an interactive File Browser dialog, outside the interface the target directory path must be provided explicitly as a parameter to the operator call.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window on the Creator's Mac, and recorded in full under questions 6 and 10 of `Preparation/Blender Research/Findings/5.3/Reading a file.md`: `report_missing_files()` printed one `Warning: Path '…' not found, from local data-block '…'` line per missing file on the terminal, a missing linked blend-file included, and left the window manager's list of reports empty (tests 13 and 37); `find_missing_files(directory=…)` searched the folder given, rewrote what it found, and wrote the location back in the form it had before (tests 14, 35 to 38). After it rewrote a missing library's path, that library's data was whole again only after saving and reopening (test 37).

**Operating system**:
Holds across all operating systems Blender runs on (Linux, macOS, Windows). Recursive directory scanning and path updates operate across all supported filesystems.

**Brought up to date by the hand tests of 2026-10-17.** The tests above were run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and settle macOS on Apple silicon for what they cover; there, a file named `TEX.PNG` was not taken for a missing `tex.png` (test 35). No Linux or Windows machine ran them for 5.3.

---

### 7. Can something a file points at be traced to the scene that uses it?

**inferred**

*Inference step*: Blender does not have a single monolithic operator named "trace to scene". However, the sources document that `Scene` is itself an `ID` data-block (`class bpy.types.Scene(ID)`), and Blender maintains an explicit reference tracking graph across all data-blocks via `bpy.data.user_map()` and Outliner data trees. Because every external reference is represented by an ID data-block whose usages are mapped by `user_map()` (e.g. `Image` -> `Material` -> `Object` -> `Collection` -> `Scene`), following this mapping upward through data-block dependencies leads directly to the top-level `Scene` data-block(s) using it.

**Not covered**: Does not settle whether Blender provides a single one-click UI operator that directly identifies or jumps to the scene from a raw file path without either script execution or searching the Outliner tree. Does not settle tracing when an external file is referenced purely inside an external library that is not directly linked into the current file's main database.

Yes. Something a `.blend` file points at (such as an external image, sound, cache file, or linked data-block from an external library) can be traced to the scene(s) using it through the Python API dependency mapping functions and the Outliner hierarchy:

1. **Via the Python API (`bpy.data.user_map`)**:
   - `bpy.data.user_map(*, subset=None, key_types=None, value_types=None)` in `bpy.types.BlendData`:
     Returns a mapping dictionary `dict[bpy.types.ID, set[bpy.types.ID]]` that maps every ID data-block to the set of data-blocks referencing it.
   - Any external file is held by an ID data-block (e.g., an external texture image is held by an `Image` data-block; an external library is referenced by a `Library` data-block, whose linked IDs are listed in `library.users_id`).
   - Starting from the referenced ID data-block, querying `user_map()` yields its immediate users (for instance, an `Image` is used by a `Material`; the `Material` is used by an `Object`; the `Object` is part of a `Collection`).
   - Following this graph upward terminates at the top-level container: a `Scene` data-block (`bpy.types.Scene`, which is an `ID` data-block).
   - In addition, `ID.user_of_id(id)` allows testing whether a specific data-block or scene directly references another data-block.
   - If an external resource or linked data-block has 0 users (or only a fake user), `user_map()` has no user entries for it, confirming it is not used in any scene.

2. **Via the User Interface (Outliner)**:
   - **Outliner in "Scenes" Display Mode**:
     The manual documents that the Outliner's "Scenes" mode "Shows the view layers, collections, and objects across all scenes."
     Using the Outliner's search box (`Ctrl-F`), typing the name of a referenced data-block (such as a linked object, linked collection, or material) filters the tree and reveals which Scene(s) and Collection(s) contain and use that item.
   - **Outliner "Select Linked"**:
     Right-clicking an item in the Outliner and choosing `Select Linked` "Selects the data-blocks that use the currently selected one (e.g. selecting all the objects that use the selected material)."
   - **Outliner "Unused Data"**:
     Data-blocks not used in any scene are isolated in the "Unused Data" mode of the Outliner ("Lists the data-blocks that are unused or only have a fake user").

#### Citations and Exact Quotes

- From Python API documentation `https://docs.blender.org/api/5.3/bpy.types.BlendData.html` (read 2026-10-11):
  - "user_map(*, subset=None, key_types=None, value_types=None): Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them."
  - "Parameters: subset (Sequence[bpy.types.ID] | None) – When passed, only these data-blocks and their users will be included as keys/values in the map."
  - "Returns: dictionary that maps data-blocks ID’s to their users. Return type: dict[bpy.types.ID, set[bpy.types.ID]]"
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.types.Library.html` (read 2026-10-11):
  - "users_id: ID data-blocks that use this library Type: tuple[ID, …] Note: Takes O(n) time, where n is the total number of all linkable ID types in bpy.data. (readonly)"
- From Python API documentation `https://docs.blender.org/api/5.3/bpy.types.ID.html` (read 2026-10-11):
  - "Base type for data-blocks, defining a unique name, linking from other libraries and garbage collection"
  - "users: Number of times this data-block is referenced (in [0, inf], default 0, readonly)"
  - "user_of_id(id): Count the number of times that ID uses/references given one. Parameters: id (ID) – ID to count usages (never None). Returns: Number of usages/references of given id by current data-block (in [0, inf])"
- From manual page `https://docs.blender.org/manual/en/5.3/editors/outliner/interface.html` (read 2026-10-11):
  - "Scenes: Shows the view layers, collections, and objects across all scenes."
  - "Search: The textbox lets you filter the tree by typing a substring. You can focus it using Ctrl-F or clear it using Alt-F."
  - "Unused Data: Lists the data-blocks that are unused or only have a fake user."
- From manual page `https://docs.blender.org/manual/en/5.3/editors/outliner/editing.html` (read 2026-10-11):
  - "Select Linked: Selects the data-blocks that use the currently selected one (e.g. selecting all the objects that use the selected material)."
- From manual page `https://docs.blender.org/manual/en/5.3/files/data_blocks.html` (read 2026-10-11; address corrected from .txt by citation check 2026-10-17):
  - "Scene ... Primary store of all data displayed and animated. Used as top-level storage for objects & animation."
  - "They can reference each other, for reuse and instancing. (Child/parent, object/object-data, materials/images, in modifiers or constraints too…)"

#### Record of Where Looked

- Searched `Blender Sources/manual_5.3/`:
  - Terms: `user_map`, `users`, `scenes`, `select linked`, `display mode`, `trace`, `reference`, `hierarchy`.
  - Pages opened and examined:
    - `manual_5.3/editors/outliner/interface.txt`: Read documentation on Outliner display modes ("Scenes", "View Layer", "Blender File", "Data API", "Unused Data") and search box filtering.
    - `manual_5.3/editors/outliner/editing.txt`: Read documentation on `Select Linked` operator.
    - `manual_5.3/files/data_blocks.txt`: Examined data-block relationships, reference counting, and `Scene` as the top-level container.
    - `manual_5.3/scene_layout/object/selecting.txt`: Read documentation on 3D Viewport `Select Linked` options.
- Searched `Blender Sources/api_5.3/`:
  - Terms: `user_map`, `users_id`, `user_of_id`, `users`, `Scene`, `Library`.
  - Pages opened and examined:
    - `api_5.3/bpy.types.BlendData.txt`: Identified `bpy.data.user_map()`.
    - `api_5.3/bpy.types.Library.txt`: Checked `users_id` attribute.
    - `api_5.3/bpy.types.ID.txt`: Checked `user_of_id()`, `users`, and base ID behaviors.
    - `api_5.3/bpy.types.Scene.txt`: Verified that `Scene` is a subclass of `ID`.
- Searched `Blender Sources/release_notes_5.3/`:
  - Terms: `user_map`, `outliner`, `scene`. Ruled out new UI tracing mechanisms in 5.3.
- Searched `Blender Sources/cycles_5.3/`:
  - Terms: `scene`, `user_map`. Verified Cycles handles scene rendering rather than ID data-block dependency graphs.

**Outside the interface**:
Holds. In background mode (`blender -b`), Python scripts can call `bpy.data.user_map()` or traverse `scene.collection.all_objects` to trace any referenced data-block up to its containing scene(s) without needing a display or user interface.

**Brought up to date by the hand tests of 2026-10-17.** **Observed** with no window on the Creator's Mac, and recorded in full under question 7 of `Preparation/Blender Research/Findings/5.3/Reading a file.md`, tests 16 to 18: `bpy.data.user_map()` ran with no window, and walking it outwards reached the scene from every image that was used — directly, inside a node group, as a driver's target, through a custom property, or by an object in a collection excluded from the view layer. Asking the map for scene-type users directly returned an empty set each time, so the walk, not one call, is what reached the scene.

**Operating system**:
Holds across all operating systems Blender runs on (Linux, macOS, Windows). Data-block references and the `user_map` dependency structure are identical across platforms.

**Brought up to date by the hand tests of 2026-10-17.** Those tests were run on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and settle macOS on Apple silicon for what they cover. No Linux or Windows machine ran them for 5.3, so for those two systems the line above has no test behind it.

---

### What the hand tests settled for questions 5, 6 and 7

**Observed (2026-10-17).** Tests were run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — with Blender 5.3.0 Beta (hash `5d8151335b6b`), with no window unless a test says otherwise, `--factory-startup`, and Blender's temporary and settings folders pointed at a throwaway work folder. The full record, with what was done and what came back word for word, is in `Preparation/Blender Research/Findings/5.3/Reading a file.md`, under questions 5, 6, 7, 9 and 10, by test number. In short:

- **Question 5.** An image's recorded path was read, assigned, saved and read back after reopening (test 12). Twenty-three kinds of recorded location were read by name and put through every listing means, both sweeps and the one-pass visitor; seven kinds — the render output, bake output, File Output folder, cloth point cache, geometry-nodes bake folder, sequencer proxy folder and a custom property — were reached by none of them (tests 20 to 23). Assigning `Library.filepath` changed the recorded path at once; the objects became those of the new file only after `Library.reload()`, while the relocate command did both in one step (test 31).
- **Question 6.** Report Missing Files printed one line per missing file, a missing linked blend-file included, to the terminal with no window, and did not fill the window manager's list of reports (tests 13 and 37). Chosen from the File menu in a window, it showed its message in the status bar; called from a script in a window, it did not (test 15). Find Missing Files ran with no window when handed a folder and opened no file browser in a window (tests 14 and 15). Where two same-named files matched, the larger was taken, and on equal sizes the one in `aaa/` over `zzz/` (test 36).
- **Question 7.** Walking `bpy.data.user_map()` outwards reached the scene from an image used directly, inside a node group, as a driver's target, through a custom property, and by an object in a collection excluded from the view layer; asking the map directly for scene-type users returned an empty set each time (tests 16 to 18). The Outliner's Scenes view, filtered to a linked object's name, showed it under the one scene that holds it (test 19).

### Things a Person Could Settle by Hand

**Pruned by the hand tests of 2026-10-17.** All three items were run on the Creator's Mac: item 1 as tests 13 and 15, item 2 as test 36, item 3 as test 19, recorded as above. Nothing on this list is still open.

---

2026-10-16 — Gemini 3.8 Flash High (Antigravity) — Reading a file, Blender 5.3

# Node setups — Blender 5.2 — second pass

**Sources consulted, and when.**
Blender 5.2 manual (`manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`api_5.2`), Blender 5.2 release notes (`release_notes_5.2`), and Cycles' own files (`cycles_5.2`) — all read on **2026-07-30** (the snapshot date given for those four). Blender's requirements page (`requirements/requirements_2026-08-01.txt`) — read on **2026-08-01**. Nothing else was consulted; I did not use the web.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

A note on Cycles' own files: I opened `cycles_5.2` and listed its contents (`__init__.py`, `camera.py`, `engine.py`, `maketx.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`, plus `shader/`, `source/`, `license/`). They are the Cycles add-on's render-engine settings and compiled OSL shader objects. They contain nothing about geometry node setups or about library linking, so they are ruled out as a source for both questions. I say so once here rather than repeating it under each answer.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**Label: documented**, for the list of things a node can be pointed at. Two sub-points below are marked **inferred** where I had to take a step, and one is marked **not in the sources**.

The sources describe four separate ways a geometry node setup reaches outside the `.blend` file.

### (a) A file path typed into a node — the Import nodes

Six nodes take a path to a file as an ordinary input:

> "Nodes used to import data from external file formats.
>
> Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor.
>
> When dropping a supported file into a Geometry Node tree, the corresponding import node is added automatically and configured with the file path.
>
> CSV (.csv)
>
> Wavefront (.obj)
>
> Stanford PLY (.ply)
>
> STL (.stl)
>
> Text (.txt)
>
> OpenVDB (.vdb)"

Source: manual, Import Nodes — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/index.html — read 2026-07-30.

Each node's own page names its input:

- "The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter." — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/csv.html
- "The path to the .obj file to be imported." — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/obj.html
- "The path to the .ply file to be imported." — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/ply.html
- "The path to the .stl file to be imported." — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/stl.html
- "The path to the txt file." with "Currently, only files with the .txt extension are supported." — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/txt.html
- "The path to the VDB file to be imported." — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/vdb.html

All read 2026-07-30.

The manual's picture of the Import OBJ node, `_images/node-types_GeometryNodeImportOBJ.webp` (shown on the Import OBJ page above), shows a node whose only input row is labelled `Path` and carries a folder-browse button at its right-hand end. Label for that: **documented** (it is part of the manual). What is visible is a browse button; that a browse button opens a file browser is not stated on that page.

Two further points about where this path lives:

- The API pages for all six nodes (`bpy.types.GeometryNodeImportCSV`, `…ImportOBJ`, `…ImportPLY`, `…ImportSTL`, `…ImportText`, `…ImportVDB`, e.g. https://docs.blender.org/api/5.2/bpy.types.GeometryNodeImportOBJ.html, read 2026-07-30) list **no properties at all** — only the inherited class methods. So the path is not a property of the node; it is the value of an input socket.
- The API documents a socket type made for exactly this: `NodeSocketStringFilePath`, whose `default_value` is described as "(default "", never None, blend relative // prefix supported)". Source: https://docs.blender.org/api/5.2/bpy.types.NodeSocketStringFilePath.html, read 2026-07-30. There is also a node-group interface version of it, `NodeTreeInterfaceSocketStringFilePath`, with the same wording on its `default_value` (https://docs.blender.org/api/5.2/bpy.types.NodeTreeInterfaceSocketStringFilePath.html, read 2026-07-30) — meaning a node group can expose a file path as one of its own inputs.
  - **Inferred**: the Import nodes' `Path` inputs are sockets of this type, and therefore accept a `//` blend-relative path. The step: the manual calls the input a path to a file, the node has no properties so the path must be a socket, and `NodeSocketStringFilePath` is the only string socket type in the API documentation that is described as a file path. The documentation nowhere joins those two facts in one sentence. If you need certainty here, it is on the hand-test list at the end.

### (b) A file behind a data-block chosen on a node

Three kinds of data-block that reference an external file can be picked directly on a geometry node.

**Image files.** The Image node: "The Image node provides access to a image file which allows you to conveniently enter and switch images for multiple nodes in the tree", and its property "Image Data-Block — The data-block selector to select an existing image or open an image from the file browser." Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/constant/image.html, read 2026-07-30. Its picture `_images/node-types_GeometryNodeInputImage.webp` shows a data-block selector row with "New" and "Open" buttons. The Image Texture node: "The Image Texture node is used to add an image file as a texture." and "If this is not connected the image can be opened or selected from the node." Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/texture/image.html, read 2026-07-30. The Image Info node takes an image and reports "Frame Count — The number of frames in an image or video frame sequence", and "FPS — The number of frames per second. For static image is always 0." Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/scene/image_info.html, read 2026-07-30 — so the image involved may be a movie or a frame sequence, not only a still.

What an image path can stand for is spelled out in the 5.2 release notes, on the path-walking API:

> "- `EXPAND_TOKENS`: visit each UDIM tile path.
> - `EXPAND_SEQUENCES`: visit each image and volume sequence file path.
> - `EXPAND_CACHES`: visit cache files, like texture cache files associated with images."

Source: release notes, Python API — https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30. The property itself: "filepath — Image/Movie file name (default "", never None, blend relative // prefix supported)", https://docs.blender.org/api/5.2/bpy.types.Image.html, read 2026-07-30.

**Font files.** The Font node: "The Font node outputs a font data-block that can be used by other nodes that require a font input… For example, it can be connected to the String to Curves Node". Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/constant/font.html, read 2026-07-30. The String to Curves node has a "Font — Font glyph used to generate the curve" input: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/utilities/text/string_to_curves.html, read 2026-07-30. The data-block table calls the Font type: "References font files. Used by curve object-data of text objects." Source: https://docs.blender.org/manual/en/5.2/files/data_blocks.html, read 2026-07-30. Its property: "filepath — (default "", never None, blend relative // prefix supported)", https://docs.blender.org/api/5.2/bpy.types.VectorFont.html, read 2026-07-30.

**Sound files.** The Sample Sound Frequencies node has a "Sound — The sound data-block to sample" input, and the page describes "the intensity of specific frequency bands in an audio file". Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/utilities/sound/sample_sound_frequencies.html, read 2026-07-30. Its section index reads "Nodes to manipulate sound and audio files" (https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/utilities/sound/index.html). The 5.2 release notes add: "The node also uses the new Sound socket type. A sound can be opened directly in Geometry Nodes." Source: https://developer.blender.org/docs/release_notes/5.2/geometry_nodes/, read 2026-07-30. The manual's picture `_images/node-types_GeometryNodeSampleSoundFrequencies.webp` shows a data-block selector row with an "Open" button. Property: "filepath — Sound sample file used by this Sound data-block (default "", never None, blend relative // prefix supported)", https://docs.blender.org/api/5.2/bpy.types.Sound.html, read 2026-07-30.

All three of these types can instead be held inside the `.blend`: the data-block table marks Font, Image and Sound as supporting "Pack — File Packing, supports file contents being packed into the blend-file". Source: https://docs.blender.org/manual/en/5.2/files/data_blocks.html, read 2026-07-30.

### (c) Baked data written next to the file

A node setup can cause the file to point at a directory of baked data:

> "Bake Target
>
> Specifies where the baked data should be stored.
>
> Inherit from Modifier:
>
> The setting is copied from the modifier that contains the bake node. This is the default.
>
> Packed:
>
> The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk:
>
> The baked data is stored in a separate directory on disk.
>
> Custom Path
>
> Specify a path where the baked data should be stored manually.
>
> Bake Path
>
> Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

Source: manual, Bake Node — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-07-30.

And on what is written there:

> "The data format used to store geometry data is not considered to be an import/export format. Volume objects, however, are saved using the OpenVDB file format which can be used interoperably."

and

> "Data is not stored to disk until the Bake operation is ran."
>
> "Blend-files must be saved to a disk before data can be baked."

Source: manual, Baking — https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/baking.html — read 2026-07-30.

The corresponding path properties are described as blend-relative-capable: "bake_directory — Location on disk where the bake data is stored (default "", never None, blend relative // prefix supported)" on both https://docs.blender.org/api/5.2/bpy.types.NodesModifier.html and, as `directory`, on https://docs.blender.org/api/5.2/bpy.types.NodesModifierBake.html — read 2026-07-30. Note these two live on the modifier, not on the node; the node's own Bake Target and Custom Path are documented on the Bake Node page quoted above.

### (d) Another `.blend` file, when a node points at linked data

A geometry node setup can point at objects, collections and materials through data-block sockets — "Collection (white) — Used to pass a collection data-block. Object (orange) — Used to pass an object data-block. Material (salmon) — Used to pass a material data-block. Texture (pink) — Used to pass a texture data-block. Image (apricot) — Used to pass an image data-block. Font (brown) — Used to pass a font data-block." Source: manual, Node parts — https://docs.blender.org/manual/en/5.2/interface/controls/nodes/parts.html — read 2026-07-30.

Any of those data-blocks may itself be linked from another file: "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded." Source: https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html, read 2026-07-30. The data-block table records the Library type as "References to an external blend-file." Source: https://docs.blender.org/manual/en/5.2/files/data_blocks.html, read 2026-07-30.

**Inferred**: therefore a geometry node setup can make its `.blend` point at another `.blend`, by pointing a data-block socket at a linked object, collection, material, image, font or texture. The step: the sources say node sockets carry these data-blocks, and separately that these data-block types can be linked from another file; no page joins the two into a single statement about geometry nodes.

### What I could not settle
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

**Not in the sources**: whether the file paths held on Import nodes are treated by Blender's own external-file housekeeping — Pack Resources, Make Paths Relative, Make Paths Absolute, Report Missing Files, Find Missing Files (all listed at https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30). None of those descriptions names nodes or node trees; they say only "all used external files" and "all paths to external files". The nearest thing is the API's `bpy.data.file_path_foreach`, "Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data" (https://docs.blender.org/api/5.2/bpy.types.BlendData.html, read 2026-07-30) — a node tree is one of the data-block types in the table, but the documentation does not say that a node's socket value counts as one of "the file paths used by" it.

**Not covered.** This answer lists what a setup can be pointed at. It does not settle: when each file is read (at evaluation, at load, on demand); whether Import nodes cache what they read; what happens when one of these files is missing; whether an Import node path is written into the `.blend`'s list of external dependencies for packing and path remapping; or whether volume data-blocks reached through an Object Info node bring their own `.vdb` files along (the Volume type has a "filepath — Volume file used by this Volume data-block", https://docs.blender.org/api/5.2/bpy.types.Volume.html, but the data-block table shows Volume with no packing support, and no page connects a Volume object to a geometry node input).
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

**Outside the interface.** Mostly yes, but the sources are only partly explicit. What is stated: baking is an operation that must be run and requires a saved `.blend` ("Data is not stored to disk until the Bake operation is ran"; "Blend-files must be saved to a disk before data can be baked" — Baking page, read 2026-07-30). The 5.2 release notes state plainly that finding a file's external file paths is meant to work for non-interactive use: "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering" (https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30). One item is interface-only by its own description: creating an import node by "dragging supported file types into the node editor" (Import Nodes page). The node itself, and its path, are data in the file and do not depend on that. Beyond this, the sources do not say.

**Brought up to date by hand test, 2026-08-06 — observed.** A Blender started with no window does read an import node's file when it opens the `.blend`, and does read it again at render time; the rendered picture was the same with `--disable-depsgraph-on-file-load` in force as without it. Every route in this answer was built, set and read from a script in a Blender with no window at all, on a machine with no display server, so nothing here needs a person clicking. The one exception is dragging a file into the node editor to make an import node: no script can make a real drag happen, so it cannot be tested at all and sits in the "Cannot be run" section of `Audits/Blender Research/Node setups/Hand tests.md`. See "Observed by hand test" below, list entries 4 and 12. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers: a Blender started with no window reads an import node's file when it opens the `.blend`, and again at render time, and the picture was the same with `--disable-depsgraph-on-file-load` in force as without it. So this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so one thing could also be looked at there that no terminal run can settle: the Path row is drawn as a text field holding the path, with a folder button at its right-hand end and a socket dot on the node's left edge level with it, and the value it shows is the one a script reads back. See "Observed by hand test on the Creator's Mac" below, list entries 4, 12 and 14. Dragging a file into the node editor to make an import node still cannot be tested at all, on either machine.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back was built, changed and read entirely from a script in a Blender with no window at all, on a machine with no display server, so nothing in it needs a person clicking. It also turns up something anyone driving Blender from outside should know: asking for the evaluated geometry again does not by itself make an import node read its file again. The setup has to be marked for a fresh run, **and** the file's modification time has to have changed by a whole second. A script that writes a file and immediately asks for the geometry can therefore be handed the old contents, with nothing said about it. Observed on Linux in all four versions; untested on macOS, because the rented machine was given up before the Mac half could be run, and Windows was not tried at all. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

**Operating system.** The sources do not say. No page I read attaches any of these node file references to a particular operating system. The manual's only operating-system remark about paths that I found is in the OBJ and legacy FBX exporters — "Uses relative paths in every case (except when on a different drive on Windows)" (https://docs.blender.org/manual/en/5.2/files/import_export/obj.html, read 2026-07-30) — which is about those exporters, not about node setups, so I do not carry it over. Blender's requirements page (read 2026-08-01, https://www.blender.org/download/requirements/) does not name version 5.2 anywhere, so it does not speak for this version; it does say "Blender is cross-platform, it runs on every major operating system", and lists Windows, macOS and Linux.

**Brought up to date by hand test, 2026-08-06 — observed.** On Linux a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, and an image path behaves the same way — see "Observed by hand test" below, list entry 11. That is the one place these tests found an operating system could make a difference, and it has been settled for Linux only. Nothing here has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed. This is the one place the two machines disagreed.** The same test on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — came out the other way: `//MODEL.OBJ` and `//Model.Obj` both found `model.obj`, an image path in upper case loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back instead of correcting it to the file that exists. So **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. The two results are kept apart and never merged into one. Windows is still untested, because no Windows machine has run it. Everything else in this answer was run on both machines and agreed on both. See "Observed by hand test on the Creator's Mac" below, list entry 11.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back found a second place where an operating system could make a difference, and it could only be run on one of them. Whether an import node reads its file again turns on the file's modification time, and on the *whole second* of it: a change inside the same whole second is not noticed, a change that crosses one is, in either direction. That was settled on the rented Linux machine's ordinary file system, in all four versions, across eighty runs with no exception. Whether macOS keeps and compares that stamp the same way is **untested** — the rented machine was given up before the Mac half could be run — and no Windows machine has run it. So this holds for Linux and is untested on both of the others. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

**Ambiguity flagged.** "Point at" could mean only files Blender will go looking for when the setup is evaluated (a, b, and d), or it could also include files the setup writes (c). I answered the wider reading and marked which is which, because the narrower one would silently drop the bake directory.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

All of this was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all** — on **2026-08-06**, with `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/root/node-setups/config/5.2`, a settings folder made for these tests before anything was started, never Blender's real one. The machine was shared with other work while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below. **Every one of these was run again on the Creator's Mac on 2026-08-07; that block follows this one. All but one agreed. The one that did not is letter case, list entry 11.** The two machines' results are kept apart and never merged.


#### What the build itself reports about the import nodes (list entry 1)

Started Blender with no window and a script that listed the classes the build registers, then built a throwaway geometry node tree and read the socket type of every input on each import node, tried to link a String node into the Path input, and looked at String to Curves. Blender printed, in full:

```
BLENDER: 5.2.0 LTS
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
274
--- named classes present or absent ---
NodeSocketString PRESENT
NodeSocketStringFilePath PRESENT
NodeSocketFont PRESENT
NodeSocketSound PRESENT
NodeSocketImage PRESENT
NodeSocketObject PRESENT
NodeSocketCollection PRESENT
NodeSocketMaterial PRESENT
NodeSocketTexture PRESENT
NodeSocketText PRESENT
NodeTreeInterfaceSocketString PRESENT
NodeTreeInterfaceSocketStringFilePath PRESENT
NodeTreeInterfaceSocketFont PRESENT
NodeTreeInterfaceSocketSound PRESENT
NodeTreeInterfaceSocketImage PRESENT
GeometryNodeInputImage PRESENT
GeometryNodeInputFont PRESENT
GeometryNodeStringToCurves PRESENT
GeometryNodeSampleSoundFrequencies PRESENT
GeometryNodeImageTexture PRESENT
GeometryNodeImageInfo PRESENT
GeometryNodeBake PRESENT
GeometryNodeInputMaterial PRESENT
GeometryNodeInputObject PRESENT
GeometryNodeInputCollection PRESENT
FunctionNodeFormatString PRESENT
--- socket bl_idname of every input on each import node ---
GeometryNodeImportOBJ [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportPLY [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportSTL [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportCSV [('Path', 'NodeSocketStringFilePath'), ('Delimiter', 'NodeSocketString')]
GeometryNodeImportText [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportVDB [('Path', 'NodeSocketStringFilePath')]
--- can a String node output be linked into Import OBJ's Path? ---
string node output sockets: [('String', 'NodeSocketString')]
links.new returned: <bpy_struct, NodeLink at 0x72c7e57687d8>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Font', 'NodeSocketFont'), ('Align X', 'NodeSocketMenu'), ('Align Y', 'NodeSocketMenu'), ('Pivot Point', 'NodeSocketMenu'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Overflow', 'NodeSocketMenu'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: False
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

#### What path-related facilities this build has (list entry 2)

The same shape of run, reading each type's properties out of Blender's own description of it. In full:

```
BLENDER: 5.2.0 LTS
--- bpy.data path facilities ---
bpy.data.file_path_map PRESENT
bpy.data.file_path_foreach PRESENT
bpy.data.libraries PRESENT
--- bpy.utils.blend_paths ---
bpy.utils.blend_paths PRESENT
--- file operators ---
bpy.ops.file.pack_all PRESENT
bpy.ops.file.unpack_all PRESENT
bpy.ops.file.pack_libraries PRESENT
bpy.ops.file.unpack_libraries PRESENT
bpy.ops.file.make_paths_relative PRESENT
bpy.ops.file.make_paths_absolute PRESENT
bpy.ops.file.report_missing_files PRESENT
bpy.ops.file.find_missing_files PRESENT
--- Library properties and functions ---
Library.filepath PRESENT
Library.parent PRESENT
Library.version PRESENT
Library.is_archive PRESENT
Library.archive_libraries PRESENT
Library.archive_parent_library PRESENT
Library.packed_file PRESENT
Library.reload PRESENT
--- ID properties ---
ID.library PRESENT
ID.is_library_indirect PRESENT
ID.is_linked_packed PRESENT
ID.is_missing PRESENT
ID.library_weak_reference PRESENT
ID.name_full PRESENT
--- NodesModifier / NodesModifierBake properties ---
NodesModifier.bake_directory PRESENT
NodesModifier.bake_target PRESENT
NodesModifier.bakes PRESENT
NodesModifierBake.directory PRESENT
NodesModifierBake.bake_target PRESENT
NodesModifierBake.bake_id PRESENT
NodesModifierBake.node PRESENT
--- NodesModifier.bake_target default ---
PACKED
--- bpy.path.abspath accepts library= ---
bpy.path.abspath('//x.obj', library=None) returned: x.obj
--- bpy.data.libraries.load accepts pack= ---
other: ValueError `pack` must be False if `link` is False
--- file path foreach flag items, if the enum exists ---
raised: KeyError 'bpy_prop_collection[key]: key "file_path_foreach" not found'
--- bpy.data.file_path_map on an empty factory file: keys with a non-empty set ---
{}
--- bpy.utils.blend_paths on an empty factory file ---
['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

One line in that script asks for the flag list of `file_path_foreach` and comes back with a `KeyError`; that line settles nothing either way and is left in the record as it stands. An earlier form of the whole check asked `hasattr` on those classes instead and reported every property absent in every version — a fault of the check, not a fact about Blender, and the reason the run above reads them from `bl_rna` instead.

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3)

A folder was made holding `model.obj` (three vertices, with `mtllib model.mtl` and `usemtl red` in it), `model.mtl`, a four-by-four `tex.png`, and `scene.blend` saved beside them. Inside the file one object carried a Geometry Nodes modifier whose group was Import OBJ → Realize Instances → Join Geometry → Group Output, with an Image Texture node holding the image. The Path was set to five values in turn; then, with the file working, `model.obj` was overwritten on disk with a four-vertex shape and the geometry read again after each of several things. In full:

*(Note added by the citation check, 2026-08-07.* This test was rebuilt from the words above and run again on the same rented Linux machine. The first half — which path forms the Path accepts, and what a missing one does — came back identical. The second half depends on something these words do not say: **how** the geometry is read again. Read plainly, it gives three vertices at every step, as recorded below. If the object is marked for update before the evaluated geometry is asked for — an ordinary thing to do, and not ruled out by anything above — every reading from the overwrite onwards comes back as **four**. Both runs happened; neither overwrites the other, and this check did not pick between them. The question has gone back to the hand tests to be run again in a form that says which reading is meant.*)*

```
00:00.575  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.575  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.576  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/03/5.2/nothing-here.obj'.
00:00.576  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/03/5.2/nothing-here.obj'
00:00.585  blend            | Read blend: "/root/node-setups/work/03/5.2/scene.blend"
BLENDER: 5.2.0 LTS WORKDIR: /root/node-setups/work/03/5.2
Info: Saved as "scene.blend"
saved: /root/node-setups/work/03/5.2/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/root/node-setups/work/03/5.2/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/root/node-setups/work/03/5.2/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /root/node-setups/work/03/5.2/model.obj
--- does the file get read again when it changes on disk? ---
vertices with the 3-vertex file: 3
vertices straight after overwriting with the 4-vertex file: 3
vertices after touching the modifier: 3
vertices after stepping the frame: 3
vertices after setting the same path again: 3
--- and after saving, quitting the file and opening it again? ---
Info: Saved as "scene.blend"
vertices after re-opening the .blend (file on disk has 4): 3
Path socket after re-opening: '//model.obj'
DONE

Blender quit
```

#### Whether a Blender with no window reads the file when it opens the `.blend` (list entry 4)

The folder that test left behind, with the four-vertex `model.obj` on disk and the `.blend` still pointing at `//model.obj`, opened three times in fresh Blenders started with `-b`. In full:

```
=== fresh process, model.obj on disk now has 4 vertices ===
00:00.488  blend            | Read blend: "/root/node-setups/work/03/5.2/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
=== same, with model.obj moved away ===
00:00.491  blend            | Read blend: "/root/node-setups/work/03/5.2/scene.blend"
00:00.501  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/03/5.2/model.obj'.
00:00.501  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/03/5.2/model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
exit code: 0
=== same, with --disable-depsgraph-on-file-load ===
00:00.484  blend            | Read blend: "/root/node-setups/work/03/5.2/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

The third run does not isolate what `--disable-depsgraph-on-file-load` does, because the check itself asks for an evaluated dependency graph, which builds one. The render comparison below answers that half.

#### Whether a render with no window reads the file, judged by the picture (list entry 12)

A second file was built holding the same Import OBJ node, an orthographic camera over the shape, the Workbench engine and a sixty-four by sixty-four frame, and frame 1 was rendered four times. The pictures were compared colour value by colour value, not byte for byte, because Blender writes text including times into every image it saves. The steps that carry the answer, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /root/node-setups/work/12/5.2/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.2/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/5.2/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.2/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4316
--- without the file, against without the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/5.2/without_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.2/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.2 ===
```

#### Whether the file read can change from frame to frame (list entry 13)

Three shapes were put on disk — `data_1.obj` with three vertices, `data_2.obj` with four, `data_3.obj` with five — and a node group joined a String node holding `//data_`, a Value to String node fed by the Scene Time node's Frame output, and a String node holding `.obj`, into the Import OBJ node's Path. In full:

```
BLENDER: 5.2.0 LTS WORKDIR: /root/node-setups/work/13/5.2
Info: Saved as "perframe.blend"
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt'), ('Base', 'NodeSocketInt'), ('Padding', 'NodeSocketInt')]
Join Strings inputs: [('Delimiter', 'NodeSocketString', False), ('Strings', 'NodeSocketString', True)]
links made: 8
frame 1 -> vertices: 3
frame 2 -> vertices: 4
frame 3 -> vertices: 5
frame 1 -> vertices: 3
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

The first form of that script linked the three pieces in reading order and Blender joined them the other way round, giving paths like `.obj1//data_` and no geometry at any frame, with `1`, `2` and `3` appearing in the three failed paths. Both runs happened; the one above is the one that carries the answer.

#### Whether an import node's path takes part in the file's own path housekeeping (list entry 5)

A file was built holding an Import OBJ node's path and an ordinary image path side by side. The image is the control: it is a kind of file Blender's own documentation marks as packable, so a run in which nothing at all moved can be told apart from a run in which only the node's path failed to move. In full:

```
00:00.609  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/05/5.2/gone-model.obj'.
00:00.609  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/05/5.2/gone-model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
BLENDER: 5.2.0 LTS WORKDIR: /root/node-setups/work/05/5.2
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/root/node-setups/work/05/5.2/model.obj'        image filepath = '/root/node-setups/work/05/5.2/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/root/node-setups/work/05/5.2/model.obj'        image filepath = '/root/node-setups/work/05/5.2/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/root/node-setups/work/05/5.2/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/root/node-setups/work/05/5.2/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/05/5.2/tex.png', '/root/node-setups/work/05/5.2/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x78019093eb08> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0x780196451c08> -> {'//model.obj'}
file_path_foreach visited 2 paths:
    (bpy.data.images['tex.png'], '//tex.png', <BlendDataPathMeta object at 0x780193cebcd0>)
    (bpy.data.node_groups['NodeSetup'], '//model.obj', <BlendDataPathMeta object at 0x780193cebf70>)
--- Pack Resources, then save ---
Info: Packed 1 file(s)
operator returned: {'FINISHED'}
image packed after pack_all: True
Info: Saved as "packed.blend"
saved packed.blend
DONE

Blender quit
```

Then `packed.blend` was reopened in a fresh Blender with both outside files moved away. In full:

```
=== packed.blend reopened with model.obj and tex.png moved away ===
00:00.498  blend            | Read blend: "/root/node-setups/work/05/5.2/packed.blend"
00:00.508  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/05/5.2/model.obj'.
00:00.508  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/05/5.2/model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: True
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
```

#### Where a bake goes when the Bake Path is left empty (list entry 6)

A file was saved holding a cube with a Geometry Nodes modifier whose group was Group Input → Bake → Group Output, the modifier's Bake Target set to Disk and its Bake Path left empty; then the same again with the Bake Path set to `//mybakes`. In full:

```
BLENDER: 5.2.0 LTS WORKDIR: /root/node-setups/work/07/5.2
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /root/node-setups/work/07/5.2/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=1304606703 bake_target=INHERIT directory=''
--- after setting the modifier's Bake Target to Disk, path left empty ---
   modifier bake_target: DISK
   modifier bake_directory: ''
--- files beside the .blend before baking ---
   ['bake.blend']
--- calling the bake operator ---
Info: Bake directory of object Target, modifier GN is empty, setting default path
   operator returned: {'FINISHED'}
--- after baking ---
   modifier bake_directory: '//blendcache_bake/Target_GN'
   bake entry directory: ''
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/1304606703/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/mybakes/1304606703/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/1304606703/meta/00001_00000.json
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

#### Whether these paths are matched letter for letter (list entry 11)

With `model.obj` and `tex.png` on disk in lower case. In full:

```
00:00.582  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/11/5.2/MODEL.OBJ'.
00:00.582  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/11/5.2/MODEL.OBJ'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.583  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/11/5.2/Model.Obj'.
00:00.583  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/11/5.2/Model.Obj'
BLENDER: 5.2.0 LTS WORKDIR: /root/node-setups/work/11/5.2
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 0
PATH='//Model.Obj'    -> vertices: 0
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: False size: (0, 0) pixels: 0
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /root/node-setups/work/11/5.2/model.obj
DONE

Blender quit
```

This is Linux, whose ordinary file system tells letter case apart. It says nothing about macOS or Windows.

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

All of this was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**, with `/Applications/Blender.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/5.2`, a settings folder made for these tests before anything was started, never Blender's real one. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below.

Every test in the Linux block above was run again here, in the same order, from the same scripts, with only the paths of the program, the settings folder and the working folder changed. **Eight of the nine agreed with Linux. One did not: letter case, list entry 11.** The results from the two machines are set out separately and are not folded into one.

Where the two agreed, the Mac's own words are given for the lines that carry the answer; the rest of each run printed the same words as the Linux block above, apart from the working-folder paths, the program's own path and the build's hash line, which on this Mac reads `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)`. Where I have kept only part of a long run, I say so at that point. The whole of every run was kept as it was printed, in `/Users/artist/Documents/Smoozy hand tests/Node setups/logs/` on that machine.


#### What the build itself reports about the import nodes (list entry 1) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.2.0 LTS
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
274
--- named classes present or absent ---
NodeSocketString PRESENT
NodeSocketStringFilePath PRESENT
NodeSocketFont PRESENT
NodeSocketSound PRESENT
NodeSocketImage PRESENT
NodeSocketObject PRESENT
NodeSocketCollection PRESENT
NodeSocketMaterial PRESENT
NodeSocketTexture PRESENT
NodeSocketText PRESENT
NodeTreeInterfaceSocketString PRESENT
NodeTreeInterfaceSocketStringFilePath PRESENT
NodeTreeInterfaceSocketFont PRESENT
NodeTreeInterfaceSocketSound PRESENT
NodeTreeInterfaceSocketImage PRESENT
GeometryNodeInputImage PRESENT
GeometryNodeInputFont PRESENT
GeometryNodeStringToCurves PRESENT
GeometryNodeSampleSoundFrequencies PRESENT
GeometryNodeImageTexture PRESENT
GeometryNodeImageInfo PRESENT
GeometryNodeBake PRESENT
GeometryNodeInputMaterial PRESENT
GeometryNodeInputObject PRESENT
GeometryNodeInputCollection PRESENT
FunctionNodeFormatString PRESENT
--- socket bl_idname of every input on each import node ---
GeometryNodeImportOBJ [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportPLY [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportSTL [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportCSV [('Path', 'NodeSocketStringFilePath'), ('Delimiter', 'NodeSocketString')]
GeometryNodeImportText [('Path', 'NodeSocketStringFilePath')]
GeometryNodeImportVDB [('Path', 'NodeSocketStringFilePath')]
--- can a String node output be linked into Import OBJ's Path? ---
string node output sockets: [('String', 'NodeSocketString')]
links.new returned: <bpy_struct, NodeLink at 0xa439248d8>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Font', 'NodeSocketFont'), ('Align X', 'NodeSocketMenu'), ('Align Y', 'NodeSocketMenu'), ('Pivot Point', 'NodeSocketMenu'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Overflow', 'NodeSocketMenu'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: False
DONE
```

#### What path-related facilities this build has (list entry 2) — Mac

Agreed with Linux, apart from where Blender's own files sit inside the application bundle, which is a fact about macOS rather than about node setups. The whole run, uncut:

```
BLENDER: 5.2.0 LTS
--- bpy.data path facilities ---
bpy.data.file_path_map PRESENT
bpy.data.file_path_foreach PRESENT
bpy.data.libraries PRESENT
--- bpy.utils.blend_paths ---
bpy.utils.blend_paths PRESENT
--- file operators ---
bpy.ops.file.pack_all PRESENT
bpy.ops.file.unpack_all PRESENT
bpy.ops.file.pack_libraries PRESENT
bpy.ops.file.unpack_libraries PRESENT
bpy.ops.file.make_paths_relative PRESENT
bpy.ops.file.make_paths_absolute PRESENT
bpy.ops.file.report_missing_files PRESENT
bpy.ops.file.find_missing_files PRESENT
--- Library properties and functions ---
Library.filepath PRESENT
Library.parent PRESENT
Library.version PRESENT
Library.is_archive PRESENT
Library.archive_libraries PRESENT
Library.archive_parent_library PRESENT
Library.packed_file PRESENT
Library.reload PRESENT
--- ID properties ---
ID.library PRESENT
ID.is_library_indirect PRESENT
ID.is_linked_packed PRESENT
ID.is_missing PRESENT
ID.library_weak_reference PRESENT
ID.name_full PRESENT
--- NodesModifier / NodesModifierBake properties ---
NodesModifier.bake_directory PRESENT
NodesModifier.bake_target PRESENT
NodesModifier.bakes PRESENT
NodesModifierBake.directory PRESENT
NodesModifierBake.bake_target PRESENT
NodesModifierBake.bake_id PRESENT
NodesModifierBake.node PRESENT
--- NodesModifier.bake_target default ---
PACKED
--- bpy.path.abspath accepts library= ---
bpy.path.abspath('//x.obj', library=None) returned: x.obj
--- bpy.data.libraries.load accepts pack= ---
other: ValueError `pack` must be False if `link` is False
--- file path foreach flag items, if the enum exists ---
raised: KeyError 'bpy_prop_collection[key]: key "file_path_foreach" not found'
--- bpy.data.file_path_map on an empty factory file: keys with a non-empty set ---
{}
--- bpy.utils.blend_paths on an empty factory file ---
['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
```

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3) — Mac

Agreed with Linux. The whole run, uncut, including what Blender printed about the paths it could not open:

```
00:00.456  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.456  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.456  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/5.2/nothing-here.obj'.
00:00.456  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/5.2/nothing-here.obj'
00:00.461  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.2/scene.blend"
BLENDER: 5.2.0 LTS WORKDIR: /tmp/smoozy-node-setups/work/03/5.2
Info: Saved as "scene.blend"
saved: /tmp/smoozy-node-setups/work/03/5.2/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/tmp/smoozy-node-setups/work/03/5.2/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/tmp/smoozy-node-setups/work/03/5.2/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /tmp/smoozy-node-setups/work/03/5.2/model.obj
--- does the file get read again when it changes on disk? ---
vertices with the 3-vertex file: 3
vertices straight after overwriting with the 4-vertex file: 3
vertices after touching the modifier: 3
vertices after stepping the frame: 3
vertices after setting the same path again: 3
--- and after saving, quitting the file and opening it again? ---
Info: Saved as "scene.blend"
vertices after re-opening the .blend (file on disk has 4): 3
Path socket after re-opening: '//model.obj'
DONE
```

#### Whether a Blender with no window reads the file when it opens the `.blend` (list entry 4) — Mac

Agreed with Linux. The three runs, uncut:

```
########## BLENDER 5.2 — fresh process, file on disk now has 4 vertices ##########
00:00.407  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.2/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
########## BLENDER 5.2 — same, with model.obj moved away ##########
00:00.394  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.2/scene.blend"
00:00.399  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/5.2/model.obj'.
00:00.399  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/5.2/model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
exit code: 0
########## BLENDER 5.2 — same, with --disable-depsgraph-on-file-load ##########
00:00.404  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.2/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
```

#### Whether a render with no window reads the file, judged by the picture (list entry 12) — Mac

Agreed with Linux. The comparison of the pictures, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /tmp/smoozy-node-setups/work/12/5.2/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.2/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/5.2/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.2/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4316
--- without the file, against without the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/5.2/without_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.2/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.2 ===
```

#### Whether the file read can change from frame to frame (list entry 13) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.2.0 LTS WORKDIR: /tmp/smoozy-node-setups/work/13/5.2
Info: Saved as "perframe.blend"
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt'), ('Base', 'NodeSocketInt'), ('Padding', 'NodeSocketInt')]
Join Strings inputs: [('Delimiter', 'NodeSocketString', False), ('Strings', 'NodeSocketString', True)]
links made: 8
frame 1 -> vertices: 3
frame 2 -> vertices: 4
frame 3 -> vertices: 5
frame 1 -> vertices: 3
DONE
```

#### Whether an import node's path takes part in the file's own path housekeeping (list entry 5) — Mac

Agreed with Linux, and the difference between 4.5 and the three later versions came out the same way here. The whole run, uncut:

```
BLENDER: 5.2.0 LTS WORKDIR: /tmp/smoozy-node-setups/work/05/5.2
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/tmp/smoozy-node-setups/work/05/5.2/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/5.2/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/tmp/smoozy-node-setups/work/05/5.2/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/5.2/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/5.2/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/5.2/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/05/5.2/tex.png', '/tmp/smoozy-node-setups/work/05/5.2/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x795ce5c08> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0x7986ca708> -> {'//model.obj'}
file_path_foreach visited 2 paths:
    (bpy.data.images['tex.png'], '//tex.png', <BlendDataPathMeta object at 0x121db6610>)
    (bpy.data.node_groups['NodeSetup'], '//model.obj', <BlendDataPathMeta object at 0x121db6d70>)
--- Pack Resources, then save ---
Info: Packed 1 file(s)
operator returned: {'FINISHED'}
image packed after pack_all: True
Info: Saved as "packed.blend"
saved packed.blend
DONE
```

And `packed.blend` opened again with both `model.obj` and `tex.png` moved away:

```
########## BLENDER 5.2 — packed.blend reopened with model.obj and tex.png moved away ##########
00:00.411  blend            | Read blend: "/tmp/smoozy-node-setups/work/05/5.2/packed.blend"
00:00.416  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/05/5.2/model.obj'.
00:00.416  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/05/5.2/model.obj'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: True
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
```

#### Where a bake goes when the Bake Path is left empty (list entry 6) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.2.0 LTS WORKDIR: /tmp/smoozy-node-setups/work/07/5.2
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /tmp/smoozy-node-setups/work/07/5.2/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=1460919198 bake_target=INHERIT directory=''
--- after setting the modifier's Bake Target to Disk, path left empty ---
   modifier bake_target: DISK
   modifier bake_directory: ''
--- files beside the .blend before baking ---
   ['bake.blend']
--- calling the bake operator ---
Info: Bake directory of object Target, modifier GN is empty, setting default path
   operator returned: {'FINISHED'}
--- after baking ---
   modifier bake_directory: '//blendcache_bake/Target_GN'
   bake entry directory: ''
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/1460919198/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/1460919198/meta/00001_00000.json
   file on disk: <workdir>/mybakes/1460919198/meta/00001_00000.json
DONE
```

#### Whether these paths are matched letter for letter (list entry 11) — Mac: **this one did not agree with Linux**

With `model.obj` and `tex.png` on disk in lower case, exactly as on Linux. The whole run, uncut:

```
BLENDER: 5.2.0 LTS WORKDIR: /tmp/smoozy-node-setups/work/11/5.2
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 3
PATH='//Model.Obj'    -> vertices: 3
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: True size: (4, 4) pixels: 64
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /tmp/smoozy-node-setups/work/11/5.2/MODEL.OBJ
DONE
```

On Linux the same run gave `//MODEL.OBJ -> vertices: 0`, `//Model.Obj -> vertices: 0`, an image that did not load for `//TEX.PNG`, and a `bpy.path.resolve_ncase` that turned the upper-case name back into the lower-case file that exists. On this Mac all three upper-case forms found the file, the image loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back. The two results are recorded side by side and are not merged: **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. Windows is untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The working folder for this run was under `/tmp`, which on this Mac sits on the volume named `Macintosh HD - Data`, an APFS volume — the machine's own ordinary file system, not one made for the test.


#### What the Path row on an Import node offers in a window (list entry 14) — Mac only

This is the one the Linux machine cannot reach at all: it has no display server, so nothing drawn in a window can be looked at there. Blender was started **with** its window, opening the `scene.blend` that list entry 3 built, and driven by a script that switched an area to the geometry node editor, showed the node group, framed the nodes with Blender's own View All, took Blender's own screenshot and quit. No mouse was moved and nothing was clicked.

The first form of the script took the screenshot without framing anything, and the second added View All. Both came back showing only the Image Texture node, because every node in the group had been made at the same spot and they were drawn on top of one another. Both are recorded as they came back; neither settles the question. The third form spread the nodes apart first, the way a person would drag them, and that is the one that answers it. Blender printed:

```
00:00.634  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.2/scene.blend"
node locations as the file holds them:
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (0.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (0.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (0.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (0.0, 0.0)
    Image Texture GeometryNodeImageTexture (0.0, 0.0)
node locations after spreading them out:
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (400.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (800.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (1200.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (1600.0, 0.0)
    Image Texture GeometryNodeImageTexture (2000.0, 0.0)
area: NODE_EDITOR region: WINDOW 2477 x 1451
Warning: 2 × Draw window and swap: 270.3194 ms, average: 135.15968760 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 16.1912 ms, average: 8.09562486 ms
screenshot written to /tmp/smoozy-node-setups/work/14/5.2/spread-all.png
Warning: 2 × Draw window and swap: 11.5162 ms, average: 5.75810415 ms
view_selected returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 16.5469 ms, average: 8.27343739 ms
screenshot written to /tmp/smoozy-node-setups/work/14/5.2/import-node.png
Path socket on the Import OBJ node holds: '//model.obj'
Path socket type: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
00:54.777  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

The screenshot shows the Import OBJ node drawn with a title bar reading **Import OBJ**, an **Instances** output on its right edge, and one row below it: a text field holding **`//model.obj`**, a **folder button** at that field's right-hand end, and a **socket dot on the node's left edge level with that row**. All four versions drew the same three things. The only difference between them is the shape of the socket dots — round in 4.5, rectangular in 5.0, 5.1 and 5.2 — which is Blender's own drawing style, not anything about paths.

The screenshots are kept at `/tmp/smoozy-node-setups/work/14/5.2/` on that machine. This settles the drawing of the node for **macOS only**. There is no Linux machine with a screen in this project, and no Windows machine ran it, so what the row looks like on either of those is untested.


---

### Observed by hand test — the test the citation check sent back — 2026-08-07

**LABEL: observed.**

The citation check of 2026-08-07 rebuilt list entry 3 from its own words and ran it again. Its last section — whether an import node reads its file again when the file changes on disk — came out one way when the evaluated geometry was asked for plainly, and the other way when the object was marked for update first. Entry 3's words do not say which was done. The check did not choose between the two, and sent the question back to the hand tests. This is the run that settles it. Entry 3's own record is untouched; this sits beside it.

**Where and when.** On **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all** — on **2026-08-07**, in all four Blender versions. The work was done inside `/root/again-node-setups/`, with a settings folder made for each version *before* any Blender was started and `BLENDER_USER_RESOURCES` pointed at it, never at Blender's real one. It could not be run on the Creator's Mac: the rented machine was about to be given up, so this had to be run there or not at all. The scripts are in `Audits/Blender Research/Node setups/Hand tests.md`, list entry 18.

**One thing about the machine.** Other topics were running Blender on the same rented machine at the same time. Nothing here measures time or memory, so that does not bear on any number below. It does bear on one thing, and the test is built around it: how often a run happens to fall on one side of a second boundary rather than the other depends on how busy the machine is. That is why the runs that carry the answer set the file's stamp themselves instead of leaving it to the clock.

**What was done.** The folder entry 3 builds was built again — `model.obj` holding three vertices, `model.mtl`, a four-by-four `tex.png`, and `scene.blend` saved beside them, with one object carrying a Geometry Nodes modifier whose group is Import OBJ → Realize Instances → Join Geometry → Group Output and an Image Texture node holding the image. The Path was set to `//model.obj`. Then `model.obj` was overwritten on disk with a four-vertex shape and the geometry was read after each of entry 3's own steps. Four arrangements were run, each in a Blender of its own so one could not disturb another: **plain**, reading the way entry 3 reads; **tagged**, calling `ob.update_tag()` before every reading, the way the citation check read; **both**, taking the two readings side by side in one run; and **routes**, trying each way of nudging the setup in turn to see which one makes the difference. Every arrangement ends by putting the three-vertex file back, as a control.

**What a negative result would have looked like, written down before the runs.** If the plain arrangement had given four vertices, entry 3's recorded result would not reproduce at all, and the record would be wrong rather than merely under-described. If the tagged arrangement had given three at every step, then marking the object for update is not what the citation check saw, and the two arrangements would still be indistinguishable — which is a thing this write-up would have had to say. If a reading had stayed at four after the three-vertex file was put back, then whatever was being read was not the file on disk, and the words "read again" would be wrong for it.

**The four arrangements, word for word.**

```
=== plain: reading the way entry 3 reads ===
STEP with the 3-vertex file                         plain: 3    tagged: -
STEP straight after overwriting with the 4-vertex file plain: 3    tagged: -
STEP after touching the modifier                    plain: 3    tagged: -
STEP after stepping the frame                       plain: 3    tagged: -
STEP after setting the same path again              plain: 3    tagged: -
STEP after putting the 3-vertex file back           plain: 3    tagged: -

=== tagged: ob.update_tag() before every reading ===
STEP with the 3-vertex file                         plain: -    tagged: 3
STEP straight after overwriting with the 4-vertex file plain: -    tagged: 3
STEP after touching the modifier                    plain: -    tagged: 3
STEP after stepping the frame                       plain: -    tagged: 3
STEP after setting the same path again              plain: -    tagged: 3
STEP after putting the 3-vertex file back           plain: -    tagged: 3

=== both, side by side in one run ===
STEP with the 3-vertex file                         plain: 3    tagged: 3
STEP straight after overwriting with the 4-vertex file plain: 3    tagged: 3
STEP after touching the modifier                    plain: 3    tagged: 3
STEP after stepping the frame                       plain: 3    tagged: 3
STEP after setting the same path again              plain: 3    tagged: 3
STEP after putting the 3-vertex file back           plain: 3    tagged: 3

=== routes: which nudge makes the difference ===
route: (none, baseline with the 3-vertex file) -> 3
route: (none, straight after the overwrite) -> 3
route: toggling mod.show_viewport twice -> 3
route: stepping the frame -> 3
route: setting the same path again -> 3
route: setting a different path and back -> 3
route: ob.data.update_tag() -> 3
route: ob.update_tag() -> 3
route: taking the group off the modifier and putting it back -> 3
control: 3-vertex file back, no tag -> 3
control: 3-vertex file back, ob.update_tag() -> 3
```

**Entry 3's own arrangement, run ten times over.**

```
5.2 plain, ten repeats, count at the reading after the modifier touch: 3 3 3 3 3 3 3 3 3 3
```

**Why the four arrangements above do not settle it.** The plain arrangement was run three more times on 5.1, where it had just given four. Two of those three runs gave three and one gave four, from the very same script minutes apart. Nothing in the script changed between them. So whatever decides this is not the arrangement — it is something that varies from run to run, and the runs below name it.

**The stamp on the file, set by hand instead of left to the clock.** The next run puts the three-vertex file's modification time at a quarter of a second past a known whole second, then moves it by a chosen amount at the overwrite, so that whether the whole second changes is decided rather than left to chance. Each line prints the two stamps, whether the whole second changed, and four readings: the one before the overwrite, then three after it, with the modifier touched between them.

```
BLENDER 5.2.0 LTS  offset  +0.000  tag False  stamp 1786081767.250 -> 1786081767.250  whole second 1786081767 -> 1786081767  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.000  tag True   stamp 1786081768.250 -> 1786081768.250  whole second 1786081768 -> 1786081768  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.500  tag False  stamp 1786081769.250 -> 1786081769.750  whole second 1786081769 -> 1786081769  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.500  tag True   stamp 1786081770.250 -> 1786081770.750  whole second 1786081770 -> 1786081770  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.700  tag False  stamp 1786081771.250 -> 1786081771.950  whole second 1786081771 -> 1786081771  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.700  tag True   stamp 1786081772.250 -> 1786081772.950  whole second 1786081772 -> 1786081772  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.740  tag False  stamp 1786081773.250 -> 1786081773.990  whole second 1786081773 -> 1786081773  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.740  tag True   stamp 1786081774.250 -> 1786081774.990  whole second 1786081774 -> 1786081774  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  +0.760  tag False  stamp 1786081775.250 -> 1786081776.010  whole second 1786081775 -> 1786081776  changed: True   readings: 3 3 4 4
BLENDER 5.2.0 LTS  offset  +0.760  tag True   stamp 1786081775.250 -> 1786081776.010  whole second 1786081775 -> 1786081776  changed: True   readings: 3 4 4 4
BLENDER 5.2.0 LTS  offset  +0.900  tag False  stamp 1786081776.250 -> 1786081777.150  whole second 1786081776 -> 1786081777  changed: True   readings: 3 3 4 4
BLENDER 5.2.0 LTS  offset  +0.900  tag True   stamp 1786081777.250 -> 1786081778.150  whole second 1786081777 -> 1786081778  changed: True   readings: 3 4 4 4
BLENDER 5.2.0 LTS  offset  +1.000  tag False  stamp 1786081778.250 -> 1786081779.250  whole second 1786081778 -> 1786081779  changed: True   readings: 3 3 4 4
BLENDER 5.2.0 LTS  offset  +1.000  tag True   stamp 1786081779.250 -> 1786081780.250  whole second 1786081779 -> 1786081780  changed: True   readings: 3 4 4 4
BLENDER 5.2.0 LTS  offset  +2.000  tag False  stamp 1786081780.250 -> 1786081782.250  whole second 1786081780 -> 1786081782  changed: True   readings: 3 3 4 4
BLENDER 5.2.0 LTS  offset  +2.000  tag True   stamp 1786081781.250 -> 1786081783.250  whole second 1786081781 -> 1786081783  changed: True   readings: 3 4 4 4
BLENDER 5.2.0 LTS  offset  -0.200  tag False  stamp 1786081782.250 -> 1786081782.050  whole second 1786081782 -> 1786081782  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  -0.200  tag True   stamp 1786081783.250 -> 1786081783.050  whole second 1786081783 -> 1786081783  changed: False  readings: 3 3 3 3
BLENDER 5.2.0 LTS  offset  -0.500  tag False  stamp 1786081784.250 -> 1786081783.750  whole second 1786081784 -> 1786081783  changed: True   readings: 3 3 4 4
BLENDER 5.2.0 LTS  offset  -0.500  tag True   stamp 1786081784.250 -> 1786081783.750  whole second 1786081784 -> 1786081783  changed: True   readings: 3 4 4 4
```

**Whether a reading on its own is enough.** The last run overwrites the file, always moving the stamp a whole second, and then reads five times over — once with the modifier touched between readings and once without.

```
BLENDER 5.2.0 LTS  touch True  tag False  whole second 1786081837 -> 1786081838  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.2.0 LTS  touch True  tag True   whole second 1786081837 -> 1786081838  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.2.0 LTS  touch False tag False  whole second 1786081838 -> 1786081839  first reading 3 then [3, 3, 3, 3, 3]
BLENDER 5.2.0 LTS  touch False tag True   whole second 1786081839 -> 1786081840  first reading 3 then [4, 4, 4, 4, 4]
```

**What the runs show.** Three things, and all three are the same in 4.5, 5.0, 5.1 and 5.2.

1. **Asking for the evaluated geometry again does not by itself make the setup run again.** With nothing touched and nothing tagged, five readings in a row give the old count, however many times you ask. Something has to mark the setup for a fresh run: touching the modifier does it, and so does `ob.update_tag()`.
2. **Even then, the file is only read again if its modification time has changed by a whole second.** A change inside the same whole second is not noticed at all — the setup runs again and comes back with the old contents. A change that crosses a whole second is noticed every time. It is noticed in either direction: a stamp moved a second *backwards* counts as a change just as much as one moved forwards. Across eighty runs — four versions, ten stamp offsets, both arrangements — there was not one exception in either direction.
3. **Marking the object for update is one of the things that make the setup run again, and it is simply the earliest one.** With the stamp changed, the tagged arrangement shows the new contents at the very first reading after the overwrite; the plain arrangement shows them at the first reading that follows a nudge, which in entry 3's sequence is the reading after the modifier is touched. One reading of difference, and no more.

**So this is exactly what tells the two arrangements apart.** Not whether the file is read again — that is decided by the stamp on the file, and by nothing either arrangement does. What the tagging changes is *when* the new contents first appear: at the first reading rather than one reading later. That is the whole of the difference between them.

**And this is why the two recorded results disagreed.** Entry 3's script overwrites `model.obj` a few hundredths of a second after writing it, so in almost every run the whole second is unchanged and nothing is read again — three vertices at every step, which is what entry 3 records, and what the ten repeats above gave — and the same ten in each of the four versions, forty runs without one exception. Now and then the overwrite lands on the far side of a second boundary, and a four appears. With the object tagged the four appears at the first reading; without the tag it appears one reading later. Both recorded results are real, neither is wrong, and neither had to be chosen between: they differ by where the run happened to fall inside a second.

**What this does not settle.** Linux only. The machine was given up before this could be run on the Creator's Mac, and no Windows machine has run it, so whether either of those keeps and compares the stamp the same way is untested. Nothing here says *why* the whole second is the unit — that would be a claim about Blender's insides, and these runs only watch it from outside.


## 2. When a `.blend` file links a node setup from another `.blend` file, what does the linking file record about the files that setup uses, and how are they reached?

**Label: documented**, for the recording mechanism and for how paths on linked data are resolved. **Inferred** for the one case the documentation does not name — a file path held in a node socket rather than on a data-block.

### What is recorded

**One entry per source `.blend`.** The linking file gains a Library data-block. Its documented content is a path: "filepath — Path to the library .blend file (default "", never None, blend relative // prefix supported)". Source: https://docs.blender.org/api/5.2/bpy.types.Library.html, read 2026-07-30. The data-block table describes the type as "References to an external blend-file. Access from the Outliner's Blender File view." Source: https://docs.blender.org/manual/en/5.2/files/data_blocks.html, read 2026-07-30. The manual: "Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from." Source: https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html, read 2026-07-30.

Whether that path is relative is a choice made at link time: "Relative Path — Reference the external blend-file using a relative path rather than an absolute one." Same page. And in the API, `bpy.data.libraries.load(…, relative=False)`: "relative (bool) – When True the path is stored relative to the open blend file." Source: https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html, read 2026-07-30.

**The setup itself and everything it needs, as linked data-blocks.** The manual on reusing node groups: "Existing node groups can be placed again after they're initially defined, be it in the same node tree or a different one. It's also possible to import node groups from a different blend-file using File ‣ Link/Append." Source: https://docs.blender.org/manual/en/5.2/interface/controls/nodes/groups.html, read 2026-07-30. The geometry node editor lists them: "Node Groups — This section lists all the node groups, both those in the current blend-file and those Linked or Appended from another blend-file", and "Linked — Lists node groups that are referenced indirectly through other node groups." Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/group.html, read 2026-07-30.

Everything the linked setup needs comes in with it, marked as an indirect dependency. The API records this on every data-block: "is_library_indirect — Is this ID block linked indirectly", and "library — Library file the data-block is linked from". Source: https://docs.blender.org/api/5.2/bpy.types.ID.html, read 2026-07-30. The link operation's own record spells the mechanism out: "INDIRECT_USAGE — That item was added for an indirectly imported ID, as a dependency of another data-block", and "source_library — Library ID representing the blendfile from which the ID was imported." Source: https://docs.blender.org/api/5.2/bpy.types.BlendImportContextItem.html, read 2026-07-30.

**The file paths themselves stay on those linked data-blocks.** The image, font or sound a linked setup uses arrives as a linked data-block of its own, carrying its own `filepath` (quoted under question 1) and its own `library`. That linked data's paths are visible from the linking file's data: the path-walking API has a flag "SKIP_LINKED — Skip Linked. Skip paths of linked IDs" (https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html, read 2026-07-30) — a flag to *skip* them, which only makes sense because by default they are visited. Separately, `bpy.data.file_path_map` has "include_libraries (bool) – Include library file paths of linked data. False by default." Source: https://docs.blender.org/api/5.2/bpy.types.BlendData.html, read 2026-07-30 — i.e. the library `.blend` paths are recorded separately from the paths of the data inside it.

**Inferred**: the linking file does not keep its own copy of, say, the image path used by the linked setup; it keeps the library path plus the linked image data-block, and the image path travels on that data-block. The step: the sources say the image path is a property of the Image data-block, and that the Image data-block is linked and carries a `library`; they never say the path is copied into the linking file. Nothing I read states the negative outright.

### How they are reached

**The library `.blend` itself.** By the recorded path. If it is relative, "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file… When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so." Source: manual, Relative Paths — https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30.

**Files used by the linked setup — resolved against the library, not against the open file.** This is stated outright:

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
>
> `>>> bpy.path.abspath(image.filepath, library=image.library)`"

Source: API documentation, File Paths & String Encoding — https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html — read 2026-07-30. The matching function description: "library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start." Source: https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30.

The manual says the same thing from another angle, in the path-template variables: "blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from" and "blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from." Source: https://docs.blender.org/manual/en/5.2/files/file_paths.html, read 2026-07-30. That page also limits where templates work at all: "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

**Libraries the library itself links from.** These are recorded too, and their paths are relative to *that* file:

> "A `libraries` attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (`filepath`, `is_archive`). The `filepath` may be absolute, or relative to the loaded blend-file."

Source: https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html, read 2026-07-30. The 5.2 release notes announce this and show it, with a node library among the examples:

> "Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"

with output including `bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)`. Source: https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30.

**When a library cannot be found.** "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored." Source: https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html, read 2026-07-30. The API marks these: "is_missing — This data-block is a place-holder for missing linked data". Source: https://docs.blender.org/api/5.2/bpy.types.ID.html, read 2026-07-30. Repair is by "Relocate Library… It allows to replace it by a different file", same manual page. The link operation can be told to make these in advance: "USE_PLACEHOLDERS — Generate a placeholder (empty ID) if not found in any library files." Source: https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html, read 2026-07-30.

**Avoiding the reach altogether: packing.** "Pack Linked Libraries — Pack data-blocks that are linked from an external blend-file into the current one." and "Unpack Linked Libraries — Export previously packed data-blocks back to external blend-files." Source: https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30. The Library data-block records this state: "is_archive — This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library", plus `archive_libraries` and `archive_parent_library`. Source: https://docs.blender.org/api/5.2/bpy.types.Library.html, read 2026-07-30. Also "is_linked_packed — This data-block is linked and packed into the .blend file" on https://docs.blender.org/api/5.2/bpy.types.ID.html, read 2026-07-30. And packed libraries drop out of the library list: "Packed linked data-blocks are not linkable and are not listed in the input `libraries` attribute." Source: https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html, read 2026-07-30.

Where asset libraries are served over a network, packing is required rather than optional: "Each .blend file should be independent. Any external dependency, such as .png files for textures, must be packed into the blend file… Assets are downloaded to the local cache directory." Source: https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html, read 2026-07-30.

**One more thing the linking file records, for baked node setups.** Where a bake was made, the reference from baked data back to a data-block is stored with the library it came from: "lib_name — Used when the data block is not local to the current .blend file but is linked from some library". Source: https://docs.blender.org/api/5.2/bpy.types.NodesModifierDataBlock.html, read 2026-07-30. The manual's matching panel: "Baked geometries that reference other data-blocks such as materials are listed here. This panel allows changing these references after the data has been baked." with "Note: Currently only material data-blocks are supported." Source: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/baking.html, read 2026-07-30. And 5.2 added "support for geonodes packed bakes in nodes modifiers" to library overrides — https://developer.blender.org/docs/release_notes/5.2/core/, read 2026-07-30.

### The gap I want to name plainly

The documented statement above — "the path will be relative to the library rather than the open blend-file" — is written about paths held **on data-blocks** (`image.filepath`, with `image.library`). A path typed into an Import node is not held on a data-block of its own; it is a socket value inside the node tree (see question 1). The sources never say which file such a socket path resolves against when the node tree is linked.

**Inferred**, with the step written out: a node tree is itself a data-block ("Node Tree ✓ — Groups of re-usable nodes. Used in the node editors", data-block table), so a linked node group is a library-linked data-block, and its socket path is "a path property on a library-linked data-block" in the wording the path-template page uses — which would put it under the same rule. The step a reader has to supply is that a socket's `default_value` counts as a path property of the node tree data-block. Nothing states that. This is the one thing in this answer I would not build on without checking; it is the first item on the hand-test list.

The 5.2 bug-fix list does record a fix in this area without saying what the behaviour now is: "Error in relative path in GN modifier". Source: https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30. I report it because it is an explicit statement that something about relative paths in the geometry nodes modifier changed in 5.2; it does not settle the question.

**Not covered.** This answer does not settle: whether a linking file can change a path used by a linked setup (library overrides are documented as allowing "Adding new modifiers and constraints, anywhere in the stack" — https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html, read 2026-07-30 — but no page says whether a socket file path on linked node data is overridable); the order in which libraries and their own libraries are opened; what happens when a library is reachable but a file *it* points at is not; and whether Report Missing Files reports across libraries.
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer: which folder a `//` inside a linked setup is measured from, and what the linking file records, list entry 7; what packing a library carries and what it leaves behind, list entry 8; what is recorded when a library links onward, list entry 9; whether the library path is stored relative or absolute, list entry 10. Read this list as "the documentation does not say", not as "nobody knows".)*

**Outside the interface.** The recording mechanism is data in the file and is not interface-dependent: the whole of linking is exposed as `bpy.data.libraries.load` and `bpy.data.libraries.write`, and the library-path information quoted above is API documentation, not interface documentation (https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html, read 2026-07-30). The gotchas page is written for scripts. The remote-asset-library page gives a command run without the interface at all: "`blender -b -c asset_listing generate .`" (https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html, read 2026-07-30). What is interface-only is the *repair* path: Reload Library, Relocate Library and Relocate Linked ID are all documented as Outliner context-menu actions (link_append page). The API does expose `Library.reload()` (Library page) and the operators `bpy.ops.wm.lib_reload` and `bpy.ops.wm.lib_relocate` are listed in https://docs.blender.org/api/5.2/bpy.ops.wm.html, read 2026-07-30; the documentation does not say whether they work with no window open.

**Brought up to date by hand test, 2026-08-06 — observed.** All of it holds with no window. Linking, packing the library, reading what the linking file records, resolving a `//` path belonging to linked data, and creating placeholders for a library that cannot be found were every one of them done and read from a script in a Blender started with `-b`, on a machine with no display server at all. With the library gone, Blender printed a warning naming it, left a placeholder in its place and exited with code 0. See "Observed by hand test" below, list entries 7, 8, 9 and 10. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers, so this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so two things could be looked at there that no terminal run can settle. The Path field on a linked group's Import node is drawn **greyed out** in the window, in all four versions — while a script writing to that same socket is accepted, which the Linux runs showed. And the modifier for a linked group holding no Bake node shows no Bake panel at all, only a collapsed section headed Manage; that section could not be opened without a click, so what it holds is untested. What a linked group that *does* hold a Bake node gives the modifier was answered instead with no window at all. See "Observed by hand test on the Creator's Mac" below, list entries 7, 8, 9, 10, 15, 16 and 17.

**Operating system.** The sources do not say. Nothing in the linked-library documentation names an operating system. As above, Blender's requirements page (read 2026-08-01) does not name 5.2 and so cannot be used to speak for it.

**Brought up to date by hand test, 2026-08-06 — observed.** Everything below was run on Linux only, where a path is matched letter for letter — see answer 1's observed section, list entry 11. Nothing has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed.** The letter-case test was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — as well, and came out the other way: there the upper-case forms found the file. See answer 1's Mac observed section, list entry 11. So a path is matched letter for letter on Linux and is not on macOS. Everything else here was run on both machines and agreed on both. No Windows machine has run it, so Windows is untested.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

Same machine, same date, same Blender and the same settings folder as the observed section under answer 1: **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all**, `/opt/blender/5.2/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, started with `BLENDER_USER_RESOURCES=/root/node-setups/config/5.2`. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`. **Every one of these was run again on the Creator's Mac on 2026-08-07, and all of them agreed; that block follows this one.** The two machines' results are kept apart and never merged.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7)

Two folders were built. `lib/` held `lib.blend`, a `model.obj` with **three** vertices and a `tex.png`; inside `lib.blend`, a node group `LibSetup` with an Import OBJ node whose Path was `//model.obj` and an Image Texture node whose image was `//tex.png`. `shot/` held `use.blend`, which linked `LibSetup` and put it on an object. Then `use.blend` was opened four times: with no `model.obj` beside it; with a **different**, four-vertex `model.obj` beside it; with `lib/model.obj` moved away and the four-vertex copy still beside `use.blend`; and with the whole `lib` folder moved away. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.562  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/use.blend"
00:00.568  reports          | Read library: '/root/node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.656  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.656  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x75b6c90b3220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x75b6c90b3220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/tex.png', '/root/node-setups/work/06/5.2/lib/model.obj', '/root/node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x75b6c901e420> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x75b6c9042b20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x75b6c90b3220> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x75b6c901e420> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x75b6c9042b20> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x75b6c90b3220> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.524  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/use.blend"
00:00.530  reports          | Read library: '/root/node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.617  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.618  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x731300cab220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x731300cab220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/tex.png', '/root/node-setups/work/06/5.2/lib/model.obj', '/root/node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x731300c1a420> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x731300c2eb20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x731300cab220> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x731300c1a420> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x731300c2eb20> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x731300cab220> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.525  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/use.blend"
00:00.530  reports          | Read library: '/root/node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.540  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/model.obj'.
00:00.540  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/model.obj'
00:00.615  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.615  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7fd83c6b5d20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7fd83c6b5d20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/tex.png', '/root/node-setups/work/06/5.2/lib/model.obj', '/root/node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7fd83c628f20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7fd83c6599a0> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7fd83c6b5d20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7fd83c628f20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7fd83c6599a0> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7fd83c6b5d20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.2/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/root/node-setups/work/06/5.2/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.529  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/use.blend"
00:00.534  reports          | Read library: '/root/node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.534  reports          | WARNING Unable to open '/root/node-setups/work/06/5.2/lib/lib.blend': No such file or directory
00:00.534  reports          | Cannot find lib '/root/node-setups/work/06/5.2/lib/lib.blend'
00:00.536  reports          | LIB: NodeTree: 'LibSetup' missing from '/root/node-setups/work/06/5.2/lib/lib.blend', parent '<direct>'
00:00.537  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x74b27aebc120> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x74b27aebc120> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x74b27b048708> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x74b27aebc120> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.2/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 5.2 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8)

`use.blend` was opened, `bpy.ops.file.pack_libraries()` run, and the result saved as `packed_use.blend`; that file was then opened three times, with everything in place, with `lib.blend` alone moved away, and with the whole `lib` folder moved away. In full:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.509  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/use.blend"
00:00.514  reports          | Read library: '/root/node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0x77f04fc93220>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x77f04fc93220> is_linked_packed: False
Info: Saved as "packed_use.blend"
saved: /root/node-setups/work/06/5.2/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.541  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/packed_use.blend"
00:00.547  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.633  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.633  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x753a12ee8820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x753a12ee8820> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.2/lib/tex.png', '/root/node-setups/work/06/5.2/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x753a12e23920> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x753a12e42ea0> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x753a12e23920> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x753a12e42ea0> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.536  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/packed_use.blend"
00:00.541  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.628  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.628  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7ad3d96cb220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7ad3d96cb220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.2/lib/tex.png', '/root/node-setups/work/06/5.2/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7ad3d963a420> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7ad3d9646ea0> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7ad3d963a420> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7ad3d9646ea0> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.534  blend            | Read blend: "/root/node-setups/work/06/5.2/shot/packed_use.blend"
00:00.540  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.548  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/model.obj'.
00:00.548  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/model.obj'
00:00.552  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.552  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/06/5.2/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x73481e2d0820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x73481e2d0820> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.2/lib/tex.png', '/root/node-setups/work/06/5.2/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7348181f7920> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x73481e235d20> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7348181f7920> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x73481e235d20> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.2/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/root/node-setups/work/06/5.2/lib/lib.blend')
Warning: Path '/root/node-setups/work/06/5.2/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/root/node-setups/work/06/5.2/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 5.2 ===
```

#### What a linking file records when its library links onward (list entry 9)

Three folders: `deep/deep.blend` with a node group `DeepSetup` reading `//model.obj` and a **three**-vertex `model.obj` beside it; `mid/mid.blend` linking `DeepSetup`, wrapping it in a local group `MidSetup`, with a **four**-vertex decoy `model.obj` beside it; `top/top.blend` linking `MidSetup`, with a **five**-vertex decoy `model.obj` beside it. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
00:00.517  blend            | Read blend: "/root/node-setups/work/09/5.2/top/top.blend"
00:00.523  reports          | Read library: '/root/node-setups/work/09/5.2/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.528  reports          | Read library: '/root/node-setups/work/09/5.2/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/5.2/mid/mid.blend'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/09/5.2/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/5.2/deep/model.obj', '/root/node-setups/work/09/5.2/deep/deep.blend', '/root/node-setups/work/09/5.2/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x786d715b9d20> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x786d76e459a0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x786d76e1cf08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x786d76eb8120> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.498  blend            | Read blend: "/root/node-setups/work/09/5.2/top/top.blend"
00:00.503  reports          | Read library: '/root/node-setups/work/09/5.2/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:00.508  reports          | Read library: '/root/node-setups/work/09/5.2/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/5.2/mid/mid.blend'
00:00.518  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/09/5.2/deep/model.obj'.
00:00.518  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/09/5.2/deep/model.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /root/node-setups/work/09/5.2/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/5.2/deep/model.obj', '/root/node-setups/work/09/5.2/deep/deep.blend', '/root/node-setups/work/09/5.2/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x7505633b5d20> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x750568c499a0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x750568c16408> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x750568cadd20> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/root/node-setups/work/09/5.2/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/root/node-setups/work/09/5.2/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 5.2 ===
```

#### Whether the library path is recorded relative or absolute, and what Blender ships (list entry 10)

In full:

```
BLENDER: 5.2.0 LTS WORKDIR: /root/node-setups/work/10/5.2
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/root/node-setups/work/10/5.2/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/root/node-setups/work/10/5.2/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: []
--- the node assets Blender ships with ---
   assets folder: /opt/blender/5.2/5.2/datafiles/assets exists: True
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/nodes/geometry_nodes_dynamics_assets.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/nodes/shading_nodes_essentials.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/nodes/principal_components.blend
   shipped .blend: /opt/blender/5.2/5.2/datafiles/assets/nodes/procedural_hair_node_assets.blend
DONE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

Same machine, same date, same Blenders and the same settings folders as the observed Mac section under answer 1: **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**, `/Applications/Blender.app/Contents/MacOS/Blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)`, started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/5.2`. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing here measures time or memory. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`.

**Every one of these agreed with Linux**, and the two machines' results are kept separate rather than merged. Two further tests were run that the Linux half did not reach: the window tests, list entries 14 and 15, which need a screen; and two the Mac half showed the list had missed, list entries 16 and 17.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7) — Mac

Agreed with Linux: the `//` was measured from the **library** file, not from the linking file. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.434  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.437  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.479  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.479  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x9a17dc620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x9a17dc620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x9a4a01c20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x9a4a5a320> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x9a17dc620> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x9a4a01c20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x9a4a5a320> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x9a17dc620> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.423  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.427  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.468  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.468  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x8a788f020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x8a788f020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x8a7d3f820> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8a78366a0> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x8a788f020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x8a7d3f820> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8a78366a0> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x8a788f020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.423  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.426  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.432  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj'.
00:00.432  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj'
00:00.467  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.467  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x8a1cb7020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x8a1cb7020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x8a6a9b820> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8a6613820> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x8a1cb7020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x8a6a9b820> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8a6613820> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x8a1cb7020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.421  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.425  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.425  reports          | WARNING Unable to open '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend': No such file or directory
00:00.425  reports          | Cannot find lib '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend'
00:00.426  reports          | LIB: NodeTree: 'LibSetup' missing from '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', parent '<direct>'
00:00.426  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x9ef5e1820> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
   local=True : ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x9ef5e1820> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x9f04a5808> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x9ef5e1820> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 5.2 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8) — Mac

Agreed with Linux. The whole run, uncut:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.864  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.868  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0x9d10d8020>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x9d10d8020> is_linked_packed: False
Info: Saved as "packed_use.blend"
saved: /tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.470  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend"
00:00.475  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.521  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.521  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x79c78aa20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x79c78aa20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7a1810e20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7a16f66a0> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7a1810e20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7a16f66a0> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.437  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend"
00:00.441  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.486  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.486  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x78917f020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x78917f020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7848e9520> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x78493c020> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7848e9520> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x78493c020> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.441  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend"
00:00.444  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.449  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj'.
00:00.449  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj'
00:00.450  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'.
00:00.450  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.2/lib/written-from-linking-file.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/5.2/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 2, 44)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xb532cb620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xb532cb620> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.2/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xb532d1520> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb53278020> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xb532d1520> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb53278020> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.2/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend')
Warning: Path '/tmp/smoozy-node-setups/work/06/5.2/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 5.2 ===
```

#### What a linking file records when its library links onward (list entry 9) — Mac

Agreed with Linux. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
00:00.564  blend            | Read blend: "/tmp/smoozy-node-setups/work/09/5.2/top/top.blend"
00:00.568  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.2/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.574  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.2/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/5.2/mid/mid.blend'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/09/5.2/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/5.2/deep/model.obj', '/tmp/smoozy-node-setups/work/09/5.2/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/5.2/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x9e4e0f120> -> {'//model.obj', '//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x9e4e0eda0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x9e7e13608> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x9e4e7f620> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.586  blend            | Read blend: "/tmp/smoozy-node-setups/work/09/5.2/top/top.blend"
00:00.592  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.2/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.596  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.2/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/5.2/mid/mid.blend'
00:00.601  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/09/5.2/deep/model.obj'.
00:00.601  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/09/5.2/deep/model.obj'
BLENDER: 5.2.0 LTS OPEN FILE: /tmp/smoozy-node-setups/work/09/5.2/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/5.2/deep/model.obj', '/tmp/smoozy-node-setups/work/09/5.2/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/5.2/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x8f7f44020> -> {'//model.obj', '//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x8f70dfba0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x8f316d808> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x8f4fdb620> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/tmp/smoozy-node-setups/work/09/5.2/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/tmp/smoozy-node-setups/work/09/5.2/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 5.2 ===
```

#### Whether the library path is recorded relative or absolute (list entry 10) — Mac

Agreed with Linux. The whole run, uncut. **The last part of it, which looks for the node files Blender ships, found nothing on this Mac** — the check looks beside the Blender program, which is where those files sit on Linux and is not where they sit on macOS. That is a fault of the check, not a fact about Blender, and it is recorded as it stands; list entry 16 below asks the same question in a way that works here.

```
BLENDER: 5.2.0 LTS WORKDIR: /tmp/smoozy-node-setups/work/10/5.2
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/5.2/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/5.2/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: []
--- the node assets Blender ships with ---
   assets folder: /Applications/Blender.app/Contents/MacOS/5.2/datafiles/assets exists: False
DONE
```

#### Which node `.blend` files each Blender ships (list entry 16) — Mac

The shipped-file half of list entry 10, asked again by way of Blender's own answer to where its files are. The whole run, uncut:

```
BLENDER: 5.2.0 LTS
binary path: /Applications/Blender.app/Contents/MacOS/Blender
the folder the earlier check looked in: /Applications/Blender.app/Contents/MacOS/5.2/datafiles/assets
   exists: False
bpy.utils.resource_path('LOCAL'): /Applications/Blender.app/Contents/Resources/5.2
assets folder: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets exists: True
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/nodes/geometry_nodes_dynamics_assets.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/nodes/principal_components.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/nodes/procedural_hair_node_assets.blend
   shipped .blend: /Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/nodes/shading_nodes_essentials.blend
count of shipped .blend files under the assets folder: 15
DONE
```

#### What a linked group holding a Bake node gives the modifier, and where that bake goes (list entry 17) — Mac

Built because the linked group in list entry 7 holds no Bake node, so list entry 15 found the modifier had no bake to show. A library file held a group of Group Input, Bake, Group Output; a second file linked that group onto a cube; then the second file was opened, read and baked. The report and the bake, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/bakeuse.blend and report, then bake ===
00:00.406  blend            | Read blend: "/tmp/smoozy-node-setups/work/17/5.2/shot/bakeuse.blend"
00:00.409  reports          | Read library: '/tmp/smoozy-node-setups/work/17/5.2/lib/bakelib.blend', '//../lib/bakelib.blend', parent '<direct>'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
BLENDER: 5.2.0 LTS STEP: report WORKDIR: /tmp/smoozy-node-setups/work/17/5.2
node group: BakeLibSetup library: //../lib/bakelib.blend
modifier bake_target: PACKED
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=431155265 bake_target=INHERIT directory=''
files beside the .blend before baking: ['bakeuse.blend', 'bakeuse.blend1']
Info: Bake directory of object Target, modifier GN is empty, setting default path
bake operator returned: {'FINISHED'}
modifier bake_directory after: '//blendcache_bakeuse/Target_GN'
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/431155265/meta/00001_00000.json
DONE
```

The bake went beside the **linking** file, not beside the library: `blendcache_bakeuse` sits in the `shot` folder, next to `bakeuse.blend`, and the modifier's own Bake Path filled itself in as `//blendcache_bakeuse/Target_GN`, measured from the linking file. The library folder gained nothing.


#### Whether a linked group's Path field can be typed into in a window, and what the modifier shows (list entry 15) — Mac only

The Linux machine has no display server, so this cannot be reached there at all. Blender was started **with** its window, opening the `use.blend` that list entry 7 built, and driven by a script that showed the linked group in the geometry node editor, took Blender's own screenshot, then switched the properties editor to the modifier tab and took a second one, before quitting. No mouse was moved and nothing was clicked.

The first form of the script hit the same trouble as list entry 14 — every node in the group had been made at the same spot, so only the topmost was visible. It is recorded as it came back and settles nothing. The second form spread the nodes apart first. Blender printed:

```
00:00.628  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.632  blend.readfile   | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x78d7a2420> indirect: False missing: False
moving the linked group's nodes apart was accepted
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (400.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (800.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (1200.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (1600.0, 0.0)
    Image Texture GeometryNodeImageTexture (2000.0, 0.0)
Warning: 2 × Draw window and swap: 33.3472 ms, average: 16.67362498 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 16.4557 ms, average: 8.22787499 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.2/linked-nodes.png
Path socket on the linked Import OBJ node holds: '//model.obj'
Path socket: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
the node group's own library_weak_reference: None
modifier: GN type: NODES
   bake_target: PACKED bake_directory: ''
   number of bake entries: 0
Warning: 2 × Draw window and swap: 11.2686 ms, average: 5.63431252 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.2/modifier-panel2.png
00:01.035  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

and, on the run that tried to open the Manage section:

```
00:00.950  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.2/shot/use.blend"
00:00.953  blend.readfile   | Read library: '/tmp/smoozy-node-setups/work/06/5.2/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
modifier: GN bake entries: 0 bake_target: PACKED bake_directory: ''
ui_expand_flag before: no such property
setting ui_expand_flag raised: AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'
Warning: 3 × Draw window and swap: 102.3880 ms, average: 34.12933337 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.2/modifier-expanded.png
00:01.433  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

**The Path field on the linked group's Import OBJ node is drawn greyed out.** Put beside the screenshot from list entry 14, where the same node in a **local** group draws `//model.obj` in bright white with a bright folder button, the linked one draws the same text and the same button dimmed. All four versions drew it dimmed. That is worth setting against what list entry 7 found from a script: writing a new value into that same socket from Python **was** accepted. So the window and the script route differ — the window will not let the value be typed, and a script will.

The whole of the right-hand Group panel was drawn greyed out in the same way, in all four versions — the group's Name, its Node Width and the plus and minus buttons beside its sockets.

**The modifier's Bake panel was not there for this group.** The modifier showed only its node-group field, holding `LibSetup` and greyed out, and one collapsed section headed **Manage**. Blender reported `number of bake entries: 0` for the modifier, which is what a group holding no Bake node gives it — the group built for list entry 7 has none. The Manage section could not be opened from a script: `NodesModifier` has no property for the open-and-shut state of its sections, and the attempt printed `AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'`. Opening it needs a click, which is not allowed here, so **what the Manage section holds for a linked group is untested**. What a linked group with a Bake node in it gives the modifier is answered instead by list entry 17 above, without a window.

The screenshots are kept at `/tmp/smoozy-node-setups/work/15/5.2/` on that machine. This settles the drawing for **macOS only**; there is no Linux machine with a screen, and no Windows machine ran it.


---

## Things a person with Blender open could settle by hand — pruned 2026-08-07

The checks first listed here have all been run. They were gathered into
`Audits/Blender Research/Node setups/Hand tests.md`, run on the rented Linux machine in this
version on 2026-08-06 and on the Creator's Mac on 2026-08-07, and their results are written into
this document under "Observed by hand test" in each answer. What each of them asked is now settled
on both of those machines, so none is left standing here. The two machines agreed on everything
except letter case, and that one is set out twice rather than merged.

What is still open, and why:

1. **Every one of them, on Windows.** No Windows machine has run it, so none of them has been run.
   That is recorded in the "Cannot be run" section of the list. It matters most for letter case,
   which is the one place the two machines that do exist disagreed.
2. **Whether dragging a supported file into the node editor creates a configured import node.**
   No script can make a real drag happen, so this cannot be run on either machine.
3. **What the Manage section of the Geometry Nodes modifier holds for a linked group.** It is
   drawn shut, and there is no property for opening it from a script — the attempt printed
   `AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'`. Opening it needs a
   click. What a linked group holding a Bake node gives the modifier was settled another way, with
   no window at all; see list entry 17.
4. **Anything drawn in a window, on Linux.** The rented machine has no display server, so the two
   window tests, list entries 14 and 15, answer for macOS only and leave Linux open.
5. **Whether letter case behaves differently on a file system of the other kind on the same
   machine.** Each machine was tested on the ordinary file system it came with. Making a
   case-sensitive volume on the Mac, or a case-insensitive one on Linux, is a change to the
   machines rather than a test on them.


---

2026-08-06 — Opus 5 (Claude Code) — Node setups, Blender 5.2

# Node setups — Blender 5.0 — second pass

**Sources consulted, and when.** Blender's manual for 5.0, Blender's Python API documentation for 5.0, Blender's release notes for 5.0, and Cycles' own files for 5.0 — all read on **2026-07-30** (the date the copies were taken). Blender's requirements page, read as taken on **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**Label: documented**, for the list below. Two items inside it are marked separately as **inferred**; they are flagged where they appear.

The sources do not print one list under this heading. The list below is assembled from the pages that each describe one kind of pointer. There are four kinds.

### (a) A file named by a path typed into the node itself — the six Import nodes

The manual has a whole family of nodes whose only purpose is to read an outside file.

> "Import Nodes
>
> Nodes used to import data from external file formats.
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

— Blender manual, https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/index.html, read 2026-07-30.

Each of the six has a `Path` input, in its own words:

- OBJ: "The Import OBJ node loads geometry data from a .obj file and outputs it as instances." / "Path — The path to the .obj file to be imported." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/obj.html
- PLY: "The Import PLY node imports mesh data from a .ply (Polygon File Format or Stanford Triangle Format) file." / "Path — The path to the .ply file to be imported." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/ply.html
- STL: "The Import STL node imports mesh geometry from a .stl (Stereolithography) file." / "Path — The path to the .stl file to be imported." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/stl.html
- CSV: "The Import CSV node reads data from a CSV (Comma-Separated Values) file and generates a point cloud." / "Path — The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/csv.html
- Text: "The Import Text node reads the contents of a plain text file and outputs it as a single string." / "Currently, only files with the .txt extension are supported." / "Path — The path to the txt file." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/txt.html
- VDB: "The Import VDB node loads volumetric data from a .vdb file and outputs it as a Volume geometry. All grids present in the file are loaded and included in the resulting volume." / "Path — The path to the VDB file to be imported." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/vdb.html

All read 2026-07-30.

One of these pages closes off a chain that would otherwise open up. The OBJ page says:

> "Note
>
> The node imports only mesh geometry; materials and textures are not imported."

— same OBJ page, read 2026-07-30. So an OBJ brought in this way does not drag in the image files its materials would name.

**Picture.** The manual's picture of the Import OBJ node, `_images/node-types_GeometryNodeImportOBJ.webp` (shown on the Import OBJ page above), shows the node with one output socket `Instances` and, below it, a row containing an empty text field labelled `Path` with a folder icon at its right-hand end, and an input socket dot at its left-hand end. Label for what the picture shows: **documented** (it is part of the manual). What is visible: the path is an input socket, it is typed as text, and there is a file-browse button beside it.

**Whether these paths can be written the short, blend-relative way.** The API documentation has a socket type for exactly this:

> "NodeSocketStringFilePath(NodeSocketStandard) […] String socket of a node
>
> default_value
>
> Input value used for unconnected socket
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

— Blender Python API, https://docs.blender.org/api/5.0/bpy.types.NodeSocketStringFilePath.html, read 2026-07-30.

Label for the claim *that the Import nodes' `Path` sockets are of this type, and so accept the `//` prefix*: **inferred**. The step I took: the API documents exactly one string socket type carrying a file path (`NodeSocketStringFilePath`, distinct from the plain `NodeSocketString`, which has no `//` note — https://docs.blender.org/api/5.0/bpy.types.NodeSocketString.html); the Import nodes' `Path` is a string input socket drawn with a file-browse button; therefore it is that type. The sources nowhere say so in words. If a reader will not take that step, the honest position is that the sources do not state what the Import nodes' paths accept.

I also checked the API pages for the Import nodes themselves — `bpy.types.GeometryNodeImportOBJ`, `…ImportPLY`, `…ImportSTL`, `…ImportCSV`, `…ImportText`, `…ImportVDB`. They carry only a one-line description (e.g. "Import geometry from an OBJ file") and the standard inherited members; none of them has a `filepath` property of its own. I scanned every `bpy.types.GeometryNode*` and `bpy.types.FunctionNode*` page for a file-path property and found none. So the path lives on the socket, not on the node.

### (b) A file reached through a data-block chosen on the node

**Image files (and movie files).** Two nodes take an Image data-block.

> "The Image node provides access to a image file which allows you to conveniently enter and switch images for multiple nodes in the tree. […] Image Data-Block — The data-block selector to select an existing image or open an image from the file browser."

— https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/constant/image.html, read 2026-07-30. Its picture, `_images/node-types_GeometryNodeInputImage.webp`, shows a data-block selector row with a `New` button and a folder icon labelled `Open`.

> "The Image Texture node is used to add an image file as a texture. The image data is sampled with the input Vector and outputs a Color and Alpha value. […] Frame — If the Image supports animation, the frame can be set here."

— https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/texture/image.html, read 2026-07-30.

What an Image data-block points at:

> "filepath — Image/Movie file name — Type: string, default "", (never None, blend relative // prefix supported)"

— https://docs.blender.org/api/5.0/bpy.types.Image.html, read 2026-07-30.

And the manual's table of data-block types lists Image as supporting both linking and packing, described as "Image files. Used by shader nodes and textures." — https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30.

**Font files.** The String to Curves node takes a Font.

> "Properties — Font — Font glyph used to generate the curve."

— https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/utilities/text/string_to_curves.html, read 2026-07-30. Its picture, `_images/node-types_GeometryNodeStringToCurves.webp`, shows on the node body a font data-block selector reading `Bfont Reg…`, with a shield icon, a folder icon and an X beside it.

The same data-block table describes the Font type as "References font files. Used by curve object-data of text objects." (link ✓, pack ✓) — https://docs.blender.org/manual/en/5.0/files/data_blocks.html. The API confirms the file:

> "VectorFont(ID) […] Vector font for Text objects — filepath — Type: string, default "", (never None, blend relative // prefix supported)"

— https://docs.blender.org/api/5.0/bpy.types.VectorFont.html, read 2026-07-30.

**Other data-blocks held on sockets.** The API lists the socket types a node tree can carry, and among them are `OBJECT`, `IMAGE`, `COLLECTION`, `TEXTURE` and `MATERIAL` — https://docs.blender.org/api/5.0/bpy_types_enum_items/node_socket_type_items.html (Node Socket Type Items), read 2026-07-30. Nodes that use them include Object Info ("This can be useful to control parameters in the geometry node tree with an external object"), Collection Info ("…with an external collection") and Geometry Input ("The Geometry Input node provides geometry from an external object, collection, or input connection to be used inside a node group") — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/scene/object_info.html, .../collection_info.html, .../geometry/geometry_input.html, read 2026-07-30.

Label for the claim *that pointing a node at an object, collection or material can in turn make the file point at further outside files (e.g. the image files a material's shader nodes name, or another `.blend` if that data-block is linked)*: **inferred**. The step: the sources document that these are data-block references, and separately document that data-blocks can be linked from another blend-file and that Image data-blocks name image files; joining those two is my step, not the source's. The sources nowhere trace this chain.

### (c) Baked data written to disk

The Bake node and Simulation zones write files.

> "Bake Target — Specifies where the baked data should be stored.
>
> Inherit from Modifier: The setting is copied from the modifier that contains the bake node. This is the default.
>
> Packed: The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk: The baked data is stored in a separate directory on disk.
>
> Custom Path — Specify a path where the baked data should be stored manually.
>
> Bake Path — Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

— https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/geometry/operations/bake.html, read 2026-07-30.

The general baking page adds what the written data is, and one hard precondition:

> "The data format used to store geometry data is not considered to be an import/export format. Volume objects, however, are saved using the OpenVDB file format which can be used interoperably."

> "Important
>
> Blend-files must be saved to a disk before data can be baked.
>
> It's not guaranteed that data written with one Blender version can be read by another Blender version."

— https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/baking.html, read 2026-07-30.

The same page records that a bake can carry references to other data-blocks:

> "Data-Block References […] Baked geometries that reference other data-blocks such as materials are listed here. This panel allows changing these references after the data has been baked."
>
> "Note — Currently only material data-blocks are supported."

— same page, read 2026-07-30.

Where the path actually lives is on the modifier, not in the node tree:

> "bake_directory — Location on disk where the bake data is stored — Type: string, default "", (never None, blend relative // prefix supported)"

— https://docs.blender.org/api/5.0/bpy.types.NodesModifier.html, read 2026-07-30; and per-bake,

> "directory — Location on disk where the bake data is stored — Type: string, default "", (never None, blend relative // prefix supported)"
>
> "bake_target — Where to store the baked data — INHERIT Inherit from Modifier – Use setting from the modifier. PACKED Packed – Pack the baked data into the .blend file. DISK Disk – Store the baked data in a directory on disk."

— https://docs.blender.org/api/5.0/bpy.types.NodesModifierBake.html, read 2026-07-30.

The modifier page repeats it in the user's words: "Bake Path — Location on disk where the baked data for Simulation Zones and Bake Nodes are stored." — https://docs.blender.org/manual/en/5.0/modeling/modifiers/geometry_nodes.html, read 2026-07-30.

### (d) Other `.blend` files, through the node groups the setup is built from

> "Node Groups
>
> This section lists all the node groups, both those in the current blend-file and those Linked or Appended from another blend-file."

— https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/group.html, read 2026-07-30.

> "Existing node groups can be placed again after they're initially defined, be it in the same node tree or a different one. It's also possible to import node groups from a different blend-file using File ‣ Link/Append."

— https://docs.blender.org/manual/en/5.0/interface/controls/nodes/groups.html, read 2026-07-30.

The data-block table lists Node Tree as "Groups of re-usable nodes. Used in the node editors.", link ✓, pack — (the dash meaning "not applicable for most data-blocks which have no file reference") — https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30.

Node groups also arrive as assets, and how they arrive decides whether a pointer to another file is created:

> "Link: The asset will be linked to the current blend-file, and thus be read-only. Later changes to the asset file will be reflected in all files that link it.
>
> Append: The asset and all its dependencies will be Appended (copied) into the current file. […]
>
> Pack: Imports the asset as linked data and immediately packs it into the current blend-file. This ensures that the asset remains available even if the original library data is modified or becomes unavailable. Useful for maintaining self-contained files that do not rely on external asset library paths."

— https://docs.blender.org/manual/en/5.0/editors/asset_browser.html, read 2026-07-30. The Preferences page adds the fourth method, "Append (Reuse Data) — Specific to the Asset Browser. The first time an asset is used, it will be appended, including its dependencies, just like described previously. However, Blender will keep track of where it originated, and the next time the asset is used, as much data as possible will be reused." — https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html, read 2026-07-30.

Blender ships node groups this way itself: "Blender includes many assets out of the box, these are contained in the 'Essentials' library. Included in this library are: Hair node groups / Smooth By Angle Node Group […]" — https://docs.blender.org/manual/en/5.0/files/asset_libraries/introduction.html, read 2026-07-30; and the Smooth by Angle page repeats "This is a node group asset that is included in the bundled 'Essentials' asset library." — https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/normals/smooth_by_angle.html, read 2026-07-30.

The release notes state which method is now the default: "The new *Packing* import method is the default for custom asset libraries" — https://developer.blender.org/docs/release_notes/5.0/assets/, read 2026-07-30.

### A tool 5.0 provides for listing the whole set

The 5.0 release notes record a new way to enumerate what a file points at:

> "Add `bpy.data.file_path_foreach()`, which calls a callback function for each file path used in the blend file […] The callback function can return a new path, which will replace the visited path."

— https://developer.blender.org/docs/release_notes/5.0/python_api/, read 2026-07-30.

The API describes it and a companion: "Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data" and "file_path_map(…) — Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them." — https://docs.blender.org/api/5.0/bpy.types.BlendData.html, read 2026-07-30. The sources do **not** say whether the paths typed into geometry-node Import nodes are among the paths these visit.

**Not covered.** This answer does not settle: whether the Import nodes' paths are seen by *Make Paths Relative*, *Make Paths Absolute*, *Report Missing Files*, *Find Missing Files* or by packing (the manual describes those operations only as acting on "all paths to external files" / "all eligible external resource files", and never names node sockets — https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html and https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html); what Blender does when an Import node's file is missing; whether an Import node's file can be packed into the `.blend`; whether an OBJ's companion `.mtl` is touched (the page says materials are not imported, which strongly suggests not, but does not say the file is left unread); whether the CSV, OBJ, PLY, STL, TXT and VDB list is exhaustive for 5.0 or merely the six documented; and anything about export *out* of a geometry node setup.
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

**Outside the interface.** The sources do not say. What they do give: the bake operations exist as callable operators — `bpy.ops.object.geometry_node_bake_single`, `…_delete_single`, `…_pack_single`, `…_unpack_single` (https://docs.blender.org/api/5.0/bpy.ops.object.html, read 2026-07-30) — and the paths themselves are ordinary data properties readable and writable from Python, so nothing in the list above depends on a person clicking. The one stated precondition, "Blend-files must be saved to a disk before data can be baked", is written about the file, not about the interface, and I found no statement qualifying any of this by whether Blender was started with a window. The one place where the sources tie a node's path to the interface is the convenience of dragging a file into the node editor to create an Import node — that is plainly an interface action, and it only *creates* the node; the node's behaviour afterwards is not described as depending on it.

**Brought up to date by hand test, 2026-08-06 — observed.** A Blender started with no window does read an import node's file when it opens the `.blend`, and does read it again at render time; the rendered picture was the same with `--disable-depsgraph-on-file-load` in force as without it. Every route in this answer was built, set and read from a script in a Blender with no window at all, on a machine with no display server, so nothing here needs a person clicking. The one exception is dragging a file into the node editor to make an import node: no script can make a real drag happen, so it cannot be tested at all and sits in the "Cannot be run" section of `Audits/Blender Research/Node setups/Hand tests.md`. See "Observed by hand test" below, list entries 4 and 12. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers: a Blender started with no window reads an import node's file when it opens the `.blend`, and again at render time, and the picture was the same with `--disable-depsgraph-on-file-load` in force as without it. So this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so one thing could also be looked at there that no terminal run can settle: the Path row is drawn as a text field holding the path, with a folder button at its right-hand end and a socket dot on the node's left edge level with it, and the value it shows is the one a script reads back. See "Observed by hand test on the Creator's Mac" below, list entries 4, 12 and 14. Dragging a file into the node editor to make an import node still cannot be tested at all, on either machine.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back was built, changed and read entirely from a script in a Blender with no window at all, on a machine with no display server, so nothing in it needs a person clicking. It also turns up something anyone driving Blender from outside should know: asking for the evaluated geometry again does not by itself make an import node read its file again. The setup has to be marked for a fresh run, **and** the file's modification time has to have changed by a whole second. A script that writes a file and immediately asks for the geometry can therefore be handed the old contents, with nothing said about it. Observed on Linux in all four versions; untested on macOS, because the rented machine was given up before the Mac half could be run, and Windows was not tried at all. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

**Operating system.** The sources do not say that any of this differs by operating system. The requirements page speaks for 5.0 where it names it: "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (https://www.blender.org/download/requirements/, read 2026-08-01), and the page's opening line is "Blender is cross-platform, it runs on every major operating system." The only operating-system-shaped hints anywhere near file paths are in the path utilities: `bpy.path.basename` — "Equivalent to os.path.basename, but skips a '//' prefix. Use for Windows compatibility."; `bpy.path.native_pathsep` — "Replace the path separator with the systems native os.sep."; `bpy.path.resolve_ncase` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." (https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30). Those tell you that separators and letter-case differ between systems in general; they say nothing about geometry node setups.

**Brought up to date by hand test, 2026-08-06 — observed.** On Linux a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, and an image path behaves the same way — see "Observed by hand test" below, list entry 11. That is the one place these tests found an operating system could make a difference, and it has been settled for Linux only. Nothing here has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed. This is the one place the two machines disagreed.** The same test on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — came out the other way: `//MODEL.OBJ` and `//Model.Obj` both found `model.obj`, an image path in upper case loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back instead of correcting it to the file that exists. So **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. The two results are kept apart and never merged into one. Windows is still untested, because no Windows machine has run it. Everything else in this answer was run on both machines and agreed on both. See "Observed by hand test on the Creator's Mac" below, list entry 11.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back found a second place where an operating system could make a difference, and it could only be run on one of them. Whether an import node reads its file again turns on the file's modification time, and on the *whole second* of it: a change inside the same whole second is not noticed, a change that crosses one is, in either direction. That was settled on the rented Linux machine's ordinary file system, in all four versions, across eighty runs with no exception. Whether macOS keeps and compares that stamp the same way is **untested** — the rented machine was given up before the Mac half could be run — and no Windows machine has run it. So this holds for Linux and is untested on both of the others. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

**Where I looked, and what I ruled out.** I searched the whole 5.0 manual and the whole 5.0 API documentation, not files one at a time. Searches run over the manual: `import (obj|ply|stl|csv|text|vdb|openvdb)`; `file|path|directory|folder` across the entire `modeling/geometry_nodes` folder (which returned the list I worked through); `external|on disk|blend-file|library|asset|cache` across the same folder; `font` across the same folder; `relative path`; `indirect`; `relative to the`; `missing file|broken link|file not found`; `case sensitiv|backslash|drive letter|forward slash`; `dependenc`. Searches run over the API: every `bpy.types.GeometryNode*` and `bpy.types.FunctionNode*` page for a file-path property (none); every page containing the phrase `blend relative` (40 pages; the node-related ones are `NodeSocketStringFilePath`, `CompositorNodeOutputFile`, `ShaderNodeScript`, `ShaderNodeTexIES`, plus `NodesModifier` and `NodesModifierBake` — no geometry node); `FILE_PATH`; `file_path_foreach`. Searches over the 5.0 release notes: `file.?path|FILE_PATH`, `import (node|obj|csv|vdb|ply|stl|text)`. I read the whole of `release_notes_5.0/geometry_nodes.md` and found nothing there about file paths. I opened four manual pictures (`node-types_GeometryNodeImportOBJ.webp`, `node-types_GeometryNodeInputImage.webp`, `node-types_GeometryNodeStringToCurves.webp`, `modeling_geometry-nodes_simulation_baking.png`). I searched Cycles' own files for `filepath|geometry.?node|library`; Cycles' hits are all about shader/OSL and camera scripts, none about geometry nodes, so I ruled Cycles out as a source for this question. I ruled out the requirements page as a source for anything but the operating-system line, because it names no node behaviour at all (a search for `node|path|link|librar|file` in it returns one line, about registering the `.blend` file extension on Windows).

I looked specifically, and did not find, any page on: a geometry node that writes a file other than a bake; a geometry node that runs a script from a file; a geometry node that reads an Alembic or USD cache file; a geometry node that reads another `.blend` directly.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

All of this was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all** — on **2026-08-06**, with `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/root/node-setups/config/5.0`, a settings folder made for these tests before anything was started, never Blender's real one. The machine was shared with other work while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below. **Every one of these was run again on the Creator's Mac on 2026-08-07; that block follows this one. All but one agreed. The one that did not is letter case, list entry 11.** The two machines' results are kept apart and never merged.


#### What the build itself reports about the import nodes (list entry 1)

Started Blender with no window and a script that listed the classes the build registers, then built a throwaway geometry node tree and read the socket type of every input on each import node, tried to link a String node into the Path input, and looked at String to Curves. Blender printed, in full:

```
BLENDER: 5.0.1
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
241
--- named classes present or absent ---
NodeSocketString PRESENT
NodeSocketStringFilePath PRESENT
NodeSocketFont ABSENT
NodeSocketSound ABSENT
NodeSocketImage PRESENT
NodeSocketObject PRESENT
NodeSocketCollection PRESENT
NodeSocketMaterial PRESENT
NodeSocketTexture PRESENT
NodeSocketText ABSENT
NodeTreeInterfaceSocketString PRESENT
NodeTreeInterfaceSocketStringFilePath PRESENT
NodeTreeInterfaceSocketFont ABSENT
NodeTreeInterfaceSocketSound ABSENT
NodeTreeInterfaceSocketImage PRESENT
GeometryNodeInputImage PRESENT
GeometryNodeInputFont ABSENT
GeometryNodeStringToCurves PRESENT
GeometryNodeSampleSoundFrequencies ABSENT
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
links.new returned: <bpy_struct, NodeLink at 0x7bafbc7fb808>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: True
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

#### What path-related facilities this build has (list entry 2)

The same shape of run, reading each type's properties out of Blender's own description of it. In full:

```
BLENDER: 5.0.1
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
['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

One line in that script asks for the flag list of `file_path_foreach` and comes back with a `KeyError`; that line settles nothing either way and is left in the record as it stands. An earlier form of the whole check asked `hasattr` on those classes instead and reported every property absent in every version — a fault of the check, not a fact about Blender, and the reason the run above reads them from `bl_rna` instead.

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3)

A folder was made holding `model.obj` (three vertices, with `mtllib model.mtl` and `usemtl red` in it), `model.mtl`, a four-by-four `tex.png`, and `scene.blend` saved beside them. Inside the file one object carried a Geometry Nodes modifier whose group was Import OBJ → Realize Instances → Join Geometry → Group Output, with an Image Texture node holding the image. The Path was set to five values in turn; then, with the file working, `model.obj` was overwritten on disk with a four-vertex shape and the geometry read again after each of several things. In full:

*(Note added by the citation check, 2026-08-07.* This test was rebuilt from the words above and run again on the same rented Linux machine. The first half — which path forms the Path accepts, and what a missing one does — came back identical. The second half depends on something these words do not say: **how** the geometry is read again. Read plainly, it gives three vertices at every step, as recorded below. If the object is marked for update before the evaluated geometry is asked for — an ordinary thing to do, and not ruled out by anything above — every reading from the overwrite onwards comes back as **four**. Both runs happened; neither overwrites the other, and this check did not pick between them. The question has gone back to the hand tests to be run again in a form that says which reading is meant.*)*

```
00:00.334  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.334  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.335  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/03/5.0/nothing-here.obj'.
00:00.335  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/03/5.0/nothing-here.obj'
00:00.356  blend            | Read blend: "/root/node-setups/work/03/5.0/scene.blend"
BLENDER: 5.0.1 WORKDIR: /root/node-setups/work/03/5.0
Info: Saved as "scene.blend"
saved: /root/node-setups/work/03/5.0/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/root/node-setups/work/03/5.0/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/root/node-setups/work/03/5.0/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /root/node-setups/work/03/5.0/model.obj
--- does the file get read again when it changes on disk? ---
vertices with the 3-vertex file: 3
vertices straight after overwriting with the 4-vertex file: 3
vertices after touching the modifier: 4
vertices after stepping the frame: 4
vertices after setting the same path again: 4
--- and after saving, quitting the file and opening it again? ---
Info: Saved as "scene.blend"
vertices after re-opening the .blend (file on disk has 4): 4
Path socket after re-opening: '//model.obj'
DONE

Blender quit
```

#### Whether a Blender with no window reads the file when it opens the `.blend` (list entry 4)

The folder that test left behind, with the four-vertex `model.obj` on disk and the `.blend` still pointing at `//model.obj`, opened three times in fresh Blenders started with `-b`. In full:

```
=== fresh process, model.obj on disk now has 4 vertices ===
00:00.288  blend            | Read blend: "/root/node-setups/work/03/5.0/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
=== same, with model.obj moved away ===
00:00.278  blend            | Read blend: "/root/node-setups/work/03/5.0/scene.blend"
00:00.290  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/03/5.0/model.obj'.
00:00.290  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/03/5.0/model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE

Blender quit
exit code: 0
=== same, with --disable-depsgraph-on-file-load ===
00:00.263  blend            | Read blend: "/root/node-setups/work/03/5.0/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

The third run does not isolate what `--disable-depsgraph-on-file-load` does, because the check itself asks for an evaluated dependency graph, which builds one. The render comparison below answers that half.

#### Whether a render with no window reads the file, judged by the picture (list entry 12)

A second file was built holding the same Import OBJ node, an orthographic camera over the shape, the Workbench engine and a sixty-four by sixty-four frame, and frame 1 was rendered four times. The pictures were compared colour value by colour value, not byte for byte, because Blender writes text including times into every image it saves. The steps that carry the answer, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /root/node-setups/work/12/5.0/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.0/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/5.0/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.0/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4316
--- without the file, against without the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/5.0/without_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.0/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.0 ===
```

#### Whether the file read can change from frame to frame (list entry 13)

Three shapes were put on disk — `data_1.obj` with three vertices, `data_2.obj` with four, `data_3.obj` with five — and a node group joined a String node holding `//data_`, a Value to String node fed by the Scene Time node's Frame output, and a String node holding `.obj`, into the Import OBJ node's Path. In full:

```
BLENDER: 5.0.1 WORKDIR: /root/node-setups/work/13/5.0
Info: Saved as "perframe.blend"
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt')]
Join Strings inputs: [('Delimiter', 'NodeSocketString', False), ('Strings', 'NodeSocketString', True)]
links made: 8
frame 1 -> vertices: 3
frame 2 -> vertices: 4
frame 3 -> vertices: 5
frame 1 -> vertices: 3
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

The first form of that script linked the three pieces in reading order and Blender joined them the other way round, giving paths like `.obj1//data_` and no geometry at any frame, with `1`, `2` and `3` appearing in the three failed paths. Both runs happened; the one above is the one that carries the answer.

#### Whether an import node's path takes part in the file's own path housekeeping (list entry 5)

A file was built holding an Import OBJ node's path and an ordinary image path side by side. The image is the control: it is a kind of file Blender's own documentation marks as packable, so a run in which nothing at all moved can be told apart from a run in which only the node's path failed to move. In full:

```
00:00.397  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/05/5.0/gone-model.obj'.
00:00.397  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/05/5.0/gone-model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
BLENDER: 5.0.1 WORKDIR: /root/node-setups/work/05/5.0
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/root/node-setups/work/05/5.0/model.obj'        image filepath = '/root/node-setups/work/05/5.0/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/root/node-setups/work/05/5.0/model.obj'        image filepath = '/root/node-setups/work/05/5.0/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/root/node-setups/work/05/5.0/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/root/node-setups/work/05/5.0/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/05/5.0/tex.png', '/root/node-setups/work/05/5.0/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x7ecc66930b08> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0x7ecc669f8f08> -> {'//model.obj'}
file_path_foreach visited 2 paths:
    (bpy.data.images['tex.png'], '//tex.png', None)
    (bpy.data.node_groups['NodeSetup'], '//model.obj', None)
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
00:00.265  blend            | Read blend: "/root/node-setups/work/05/5.0/packed.blend"
00:00.277  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/05/5.0/model.obj'.
00:00.277  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/05/5.0/model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: True
DONE

Blender quit
```

#### Where a bake goes when the Bake Path is left empty (list entry 6)

A file was saved holding a cube with a Geometry Nodes modifier whose group was Group Input → Bake → Group Output, the modifier's Bake Target set to Disk and its Bake Path left empty; then the same again with the Bake Path set to `//mybakes`. In full:

```
BLENDER: 5.0.1 WORKDIR: /root/node-setups/work/07/5.0
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /root/node-setups/work/07/5.0/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=1276772900 bake_target=INHERIT directory=''
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
   file on disk: <workdir>/blendcache_bake/Target_GN/1276772900/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/mybakes/1276772900/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/1276772900/meta/00001_00000.json
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

#### Whether these paths are matched letter for letter (list entry 11)

With `model.obj` and `tex.png` on disk in lower case. In full:

```
00:00.336  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/11/5.0/MODEL.OBJ'.
00:00.336  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/11/5.0/MODEL.OBJ'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.337  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/11/5.0/Model.Obj'.
00:00.337  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/11/5.0/Model.Obj'
BLENDER: 5.0.1 WORKDIR: /root/node-setups/work/11/5.0
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 0
PATH='//Model.Obj'    -> vertices: 0
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: False size: (0, 0) pixels: 0
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /root/node-setups/work/11/5.0/model.obj
DONE

Blender quit
```

This is Linux, whose ordinary file system tells letter case apart. It says nothing about macOS or Windows.

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

All of this was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**, with `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/5.0`, a settings folder made for these tests before anything was started, never Blender's real one. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below.

Every test in the Linux block above was run again here, in the same order, from the same scripts, with only the paths of the program, the settings folder and the working folder changed. **Eight of the nine agreed with Linux. One did not: letter case, list entry 11.** The results from the two machines are set out separately and are not folded into one.

Where the two agreed, the Mac's own words are given for the lines that carry the answer; the rest of each run printed the same words as the Linux block above, apart from the working-folder paths, the program's own path and the build's hash line, which on this Mac reads `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)`. Where I have kept only part of a long run, I say so at that point. The whole of every run was kept as it was printed, in `/Users/artist/Documents/Smoozy hand tests/Node setups/logs/` on that machine.


#### What the build itself reports about the import nodes (list entry 1) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.0.1
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
241
--- named classes present or absent ---
NodeSocketString PRESENT
NodeSocketStringFilePath PRESENT
NodeSocketFont ABSENT
NodeSocketSound ABSENT
NodeSocketImage PRESENT
NodeSocketObject PRESENT
NodeSocketCollection PRESENT
NodeSocketMaterial PRESENT
NodeSocketTexture PRESENT
NodeSocketText ABSENT
NodeTreeInterfaceSocketString PRESENT
NodeTreeInterfaceSocketStringFilePath PRESENT
NodeTreeInterfaceSocketFont ABSENT
NodeTreeInterfaceSocketSound ABSENT
NodeTreeInterfaceSocketImage PRESENT
GeometryNodeInputImage PRESENT
GeometryNodeInputFont ABSENT
GeometryNodeStringToCurves PRESENT
GeometryNodeSampleSoundFrequencies ABSENT
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
links.new returned: <bpy_struct, NodeLink at 0x81c1f3888>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: True
DONE
```

#### What path-related facilities this build has (list entry 2) — Mac

Agreed with Linux, apart from where Blender's own files sit inside the application bundle, which is a fact about macOS rather than about node setups. The whole run, uncut:

```
BLENDER: 5.0.1
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
['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
```

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3) — Mac

Agreed with Linux. The whole run, uncut, including what Blender printed about the paths it could not open:

```
00:00.345  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.345  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.345  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/5.0/nothing-here.obj'.
00:00.345  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/5.0/nothing-here.obj'
00:00.348  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.0/scene.blend"
BLENDER: 5.0.1 WORKDIR: /tmp/smoozy-node-setups/work/03/5.0
Info: Saved as "scene.blend"
saved: /tmp/smoozy-node-setups/work/03/5.0/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/tmp/smoozy-node-setups/work/03/5.0/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/tmp/smoozy-node-setups/work/03/5.0/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /tmp/smoozy-node-setups/work/03/5.0/model.obj
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
########## BLENDER 5.0 — fresh process, file on disk now has 4 vertices ##########
00:00.277  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.0/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
########## BLENDER 5.0 — same, with model.obj moved away ##########
00:00.263  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.0/scene.blend"
00:00.269  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/5.0/model.obj'.
00:00.269  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/5.0/model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE

Blender quit
exit code: 0
########## BLENDER 5.0 — same, with --disable-depsgraph-on-file-load ##########
00:00.292  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.0/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

#### Whether a render with no window reads the file, judged by the picture (list entry 12) — Mac

Agreed with Linux. The comparison of the pictures, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /tmp/smoozy-node-setups/work/12/5.0/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.0/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/5.0/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.0/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4316
--- without the file, against without the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/5.0/without_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.0/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.0 ===
```

#### Whether the file read can change from frame to frame (list entry 13) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.0.1 WORKDIR: /tmp/smoozy-node-setups/work/13/5.0
Info: Saved as "perframe.blend"
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt')]
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
BLENDER: 5.0.1 WORKDIR: /tmp/smoozy-node-setups/work/05/5.0
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/tmp/smoozy-node-setups/work/05/5.0/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/5.0/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/tmp/smoozy-node-setups/work/05/5.0/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/5.0/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/5.0/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/5.0/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/05/5.0/tex.png', '/tmp/smoozy-node-setups/work/05/5.0/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0xa30f9a308> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0xa2efd8f08> -> {'//model.obj'}
file_path_foreach visited 2 paths:
    (bpy.data.images['tex.png'], '//tex.png', None)
    (bpy.data.node_groups['NodeSetup'], '//model.obj', None)
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
########## BLENDER 5.0 — packed.blend reopened with model.obj and tex.png moved away ##########
00:00.271  blend            | Read blend: "/tmp/smoozy-node-setups/work/05/5.0/packed.blend"
00:00.276  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/05/5.0/model.obj'.
00:00.276  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/05/5.0/model.obj'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: True
DONE

Blender quit
```

#### Where a bake goes when the Bake Path is left empty (list entry 6) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.0.1 WORKDIR: /tmp/smoozy-node-setups/work/07/5.0
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /tmp/smoozy-node-setups/work/07/5.0/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=1178541794 bake_target=INHERIT directory=''
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
   file on disk: <workdir>/blendcache_bake/Target_GN/1178541794/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/1178541794/meta/00001_00000.json
   file on disk: <workdir>/mybakes/1178541794/meta/00001_00000.json
DONE
```

#### Whether these paths are matched letter for letter (list entry 11) — Mac: **this one did not agree with Linux**

With `model.obj` and `tex.png` on disk in lower case, exactly as on Linux. The whole run, uncut:

```
BLENDER: 5.0.1 WORKDIR: /tmp/smoozy-node-setups/work/11/5.0
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 3
PATH='//Model.Obj'    -> vertices: 3
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: True size: (4, 4) pixels: 64
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /tmp/smoozy-node-setups/work/11/5.0/MODEL.OBJ
DONE
```

On Linux the same run gave `//MODEL.OBJ -> vertices: 0`, `//Model.Obj -> vertices: 0`, an image that did not load for `//TEX.PNG`, and a `bpy.path.resolve_ncase` that turned the upper-case name back into the lower-case file that exists. On this Mac all three upper-case forms found the file, the image loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back. The two results are recorded side by side and are not merged: **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. Windows is untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The working folder for this run was under `/tmp`, which on this Mac sits on the volume named `Macintosh HD - Data`, an APFS volume — the machine's own ordinary file system, not one made for the test.


#### What the Path row on an Import node offers in a window (list entry 14) — Mac only

This is the one the Linux machine cannot reach at all: it has no display server, so nothing drawn in a window can be looked at there. Blender was started **with** its window, opening the `scene.blend` that list entry 3 built, and driven by a script that switched an area to the geometry node editor, showed the node group, framed the nodes with Blender's own View All, took Blender's own screenshot and quit. No mouse was moved and nothing was clicked.

The first form of the script took the screenshot without framing anything, and the second added View All. Both came back showing only the Image Texture node, because every node in the group had been made at the same spot and they were drawn on top of one another. Both are recorded as they came back; neither settles the question. The third form spread the nodes apart first, the way a person would drag them, and that is the one that answers it. Blender printed:

```
00:00.534  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.0/scene.blend"
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
area: NODE_EDITOR region: WINDOW 2477 x 1458
Warning: 2 × Draw window and swap: 54.8952 ms, average: 27.44758129 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 18.5051 ms, average: 9.25254822 ms
screenshot written to /tmp/smoozy-node-setups/work/14/5.0/spread-all.png
Warning: 2 × Draw window and swap: 10.8809 ms, average: 5.44047356 ms
view_selected returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 16.0470 ms, average: 8.02350044 ms
screenshot written to /tmp/smoozy-node-setups/work/14/5.0/import-node.png
Path socket on the Import OBJ node holds: '//model.obj'
Path socket type: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
00:00.979  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

The screenshot shows the Import OBJ node drawn with a title bar reading **Import OBJ**, an **Instances** output on its right edge, and one row below it: a text field holding **`//model.obj`**, a **folder button** at that field's right-hand end, and a **socket dot on the node's left edge level with that row**. All four versions drew the same three things. The only difference between them is the shape of the socket dots — round in 4.5, rectangular in 5.0, 5.1 and 5.2 — which is Blender's own drawing style, not anything about paths.

The screenshots are kept at `/tmp/smoozy-node-setups/work/14/5.0/` on that machine. This settles the drawing of the node for **macOS only**. There is no Linux machine with a screen in this project, and no Windows machine ran it, so what the row looks like on either of those is untested.


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
5.0 plain, ten repeats, count at the reading after the modifier touch: 3 3 3 3 3 3 3 3 3 3
```

**Why the four arrangements above do not settle it.** The plain arrangement was run three more times on 5.1, where it had just given four. Two of those three runs gave three and one gave four, from the very same script minutes apart. Nothing in the script changed between them. So whatever decides this is not the arrangement — it is something that varies from run to run, and the runs below name it.

**The stamp on the file, set by hand instead of left to the clock.** The next run puts the three-vertex file's modification time at a quarter of a second past a known whole second, then moves it by a chosen amount at the overwrite, so that whether the whole second changes is decided rather than left to chance. Each line prints the two stamps, whether the whole second changed, and four readings: the one before the overwrite, then three after it, with the modifier touched between them.

```
BLENDER 5.0.1      offset  +0.000  tag False  stamp 1786081743.250 -> 1786081743.250  whole second 1786081743 -> 1786081743  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.000  tag True   stamp 1786081743.250 -> 1786081743.250  whole second 1786081743 -> 1786081743  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.500  tag False  stamp 1786081744.250 -> 1786081744.750  whole second 1786081744 -> 1786081744  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.500  tag True   stamp 1786081744.250 -> 1786081744.750  whole second 1786081744 -> 1786081744  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.700  tag False  stamp 1786081745.250 -> 1786081745.950  whole second 1786081745 -> 1786081745  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.700  tag True   stamp 1786081746.250 -> 1786081746.950  whole second 1786081746 -> 1786081746  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.740  tag False  stamp 1786081746.250 -> 1786081746.990  whole second 1786081746 -> 1786081746  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.740  tag True   stamp 1786081747.250 -> 1786081747.990  whole second 1786081747 -> 1786081747  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  +0.760  tag False  stamp 1786081747.250 -> 1786081748.010  whole second 1786081747 -> 1786081748  changed: True   readings: 3 3 4 4
BLENDER 5.0.1      offset  +0.760  tag True   stamp 1786081748.250 -> 1786081749.010  whole second 1786081748 -> 1786081749  changed: True   readings: 3 4 4 4
BLENDER 5.0.1      offset  +0.900  tag False  stamp 1786081748.250 -> 1786081749.150  whole second 1786081748 -> 1786081749  changed: True   readings: 3 3 4 4
BLENDER 5.0.1      offset  +0.900  tag True   stamp 1786081749.250 -> 1786081750.150  whole second 1786081749 -> 1786081750  changed: True   readings: 3 4 4 4
BLENDER 5.0.1      offset  +1.000  tag False  stamp 1786081749.250 -> 1786081750.250  whole second 1786081749 -> 1786081750  changed: True   readings: 3 3 4 4
BLENDER 5.0.1      offset  +1.000  tag True   stamp 1786081750.250 -> 1786081751.250  whole second 1786081750 -> 1786081751  changed: True   readings: 3 4 4 4
BLENDER 5.0.1      offset  +2.000  tag False  stamp 1786081750.250 -> 1786081752.250  whole second 1786081750 -> 1786081752  changed: True   readings: 3 3 4 4
BLENDER 5.0.1      offset  +2.000  tag True   stamp 1786081751.250 -> 1786081753.250  whole second 1786081751 -> 1786081753  changed: True   readings: 3 4 4 4
BLENDER 5.0.1      offset  -0.200  tag False  stamp 1786081752.250 -> 1786081752.050  whole second 1786081752 -> 1786081752  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  -0.200  tag True   stamp 1786081752.250 -> 1786081752.050  whole second 1786081752 -> 1786081752  changed: False  readings: 3 3 3 3
BLENDER 5.0.1      offset  -0.500  tag False  stamp 1786081753.250 -> 1786081752.750  whole second 1786081753 -> 1786081752  changed: True   readings: 3 3 4 4
BLENDER 5.0.1      offset  -0.500  tag True   stamp 1786081753.250 -> 1786081752.750  whole second 1786081753 -> 1786081752  changed: True   readings: 3 4 4 4
```

**Whether a reading on its own is enough.** The last run overwrites the file, always moving the stamp a whole second, and then reads five times over — once with the modifier touched between readings and once without.

```
BLENDER 5.0.1      touch True  tag False  whole second 1786081831 -> 1786081832  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.0.1      touch True  tag True   whole second 1786081832 -> 1786081833  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.0.1      touch False tag False  whole second 1786081833 -> 1786081834  first reading 3 then [3, 3, 3, 3, 3]
BLENDER 5.0.1      touch False tag True   whole second 1786081833 -> 1786081834  first reading 3 then [4, 4, 4, 4, 4]
```

**What the runs show.** Three things, and all three are the same in 4.5, 5.0, 5.1 and 5.2.

1. **Asking for the evaluated geometry again does not by itself make the setup run again.** With nothing touched and nothing tagged, five readings in a row give the old count, however many times you ask. Something has to mark the setup for a fresh run: touching the modifier does it, and so does `ob.update_tag()`.
2. **Even then, the file is only read again if its modification time has changed by a whole second.** A change inside the same whole second is not noticed at all — the setup runs again and comes back with the old contents. A change that crosses a whole second is noticed every time. It is noticed in either direction: a stamp moved a second *backwards* counts as a change just as much as one moved forwards. Across eighty runs — four versions, ten stamp offsets, both arrangements — there was not one exception in either direction.
3. **Marking the object for update is one of the things that make the setup run again, and it is simply the earliest one.** With the stamp changed, the tagged arrangement shows the new contents at the very first reading after the overwrite; the plain arrangement shows them at the first reading that follows a nudge, which in entry 3's sequence is the reading after the modifier is touched. One reading of difference, and no more.

**So this is exactly what tells the two arrangements apart.** Not whether the file is read again — that is decided by the stamp on the file, and by nothing either arrangement does. What the tagging changes is *when* the new contents first appear: at the first reading rather than one reading later. That is the whole of the difference between them.

**And this is why the two recorded results disagreed.** Entry 3's script overwrites `model.obj` a few hundredths of a second after writing it, so in almost every run the whole second is unchanged and nothing is read again — three vertices at every step, which is what entry 3 records, and what the ten repeats above gave — and the same ten in each of the four versions, forty runs without one exception. Now and then the overwrite lands on the far side of a second boundary, and a four appears. With the object tagged the four appears at the first reading; without the tag it appears one reading later. Both recorded results are real, neither is wrong, and neither had to be chosen between: they differ by where the run happened to fall inside a second.

**What this does not settle.** Linux only. The machine was given up before this could be run on the Creator's Mac, and no Windows machine has run it, so whether either of those keeps and compares the stamp the same way is untested. Nothing here says *why* the whole second is the unit — that would be a claim about Blender's insides, and these runs only watch it from outside.


## 2. When a `.blend` file links a node setup from another `.blend` file, what does the linking file record about the files that setup uses, and how are they reached?

**Ambiguity, flagged.** "The files that setup uses" can be read two ways. I answer the reading I think is meant: *the outside files the linked setup depends on — its images, fonts, imported files, bake data — and what the linking file holds about them.* The other reading is narrower: *the library `.blend` file itself, which is the file the setup "uses" in the sense of coming from.* I cover that one too, first, because the second reading's answer is a prerequisite for the first.

**Label: documented**, for the parts about the library record and about relative paths, cited below. **Observed**, for the one piece of evidence about how a relative path on a node inside a linked tree is turned into a real path — from Cycles' own files. **Inferred**, for generalising that piece beyond Cycles. Each is marked in place.

### What the linking file records about the other `.blend`

A data-block called a Library.

> "Library(ID) […] External .blend file from which data is linked
>
> filepath — Path to the library .blend file — Type: string, default "", (never None, blend relative // prefix supported)"

— https://docs.blender.org/api/5.0/bpy.types.Library.html, read 2026-07-30. The same page lists `parent — Type: Library, (readonly)` (a library reached through another library), and `version — Version of Blender the library .blend was saved with`.

The manual's data-block table describes the type as "References to an external blend-file. Access from the Outliner's Blender File view." — it cannot itself be linked, but it can be packed — https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30.

The manual describes the record from the user's side:

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded. […] Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."

— https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30.

**Whether that path is relative or absolute is a choice made at link time.** The same page, under Link ‣ Options:

> "Relative Path — Reference the external blend-file using a relative path rather than an absolute one."

The default comes from Preferences: "Default To – Relative Paths — Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set." — https://docs.blender.org/manual/en/5.0/editors/preferences/save_load.html, read 2026-07-30. And what "relative" means:

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

— https://docs.blender.org/manual/en/5.0/files/blend/open_save.html, read 2026-07-30 (section "Relative Paths"). The same section: "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so." and "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

The API states the same in the vocabulary of the linking operation itself, as one of the options of a blendfile import:

> "MAKE_PATHS_RELATIVE — Make paths of used library blendfiles relative to current blendfile."

— https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html, read 2026-07-30. And the Python entry point carries it as an argument: `bpy.data.libraries.load(filepath, *, link=False, relative=False, …)`, where "relative (bool) – When True the path is stored relative to the open blend file." — https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html, read 2026-07-30.

**The linked data-block's identity is recorded alongside the library**, because names are only unique per file:

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in. […] it is possible to have one local and several linked Objects sharing the same name."

— https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30. In the API: `name_full — Unique data-block ID name, including library one if any`, and `library — Library file the data-block is linked from` — https://docs.blender.org/api/5.0/bpy.types.ID.html, read 2026-07-30.

### What the linking file records about the files the *setup* uses

**It records nothing of its own about them. They arrive as part of the linked data, from the library file.**

The evidence, in three pieces.

*First*, the sources document a category for exactly this — data pulled in not because you asked for it but because the thing you asked for needs it:

> "is_library_indirect — Is this ID block linked indirectly — Type: boolean, default False, (readonly)"

— https://docs.blender.org/api/5.0/bpy.types.ID.html, read 2026-07-30. And, describing an import in progress:

> "INDIRECT_USAGE — That item was added for an indirectly imported ID, as a dependency of another data-block."

— https://docs.blender.org/api/5.0/bpy.types.BlendImportContextItem.html, read 2026-07-30.

*Second*, the manual treats "keep the links" as the normal state and copying as the exception. Under Append ‣ Options:

> "Localize All — Also copy all indirectly linked data, instead of maintaining the links."

— https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30. That option exists because, without it, indirectly used data stays linked rather than copied.

*Third*, the API distinguishes the paths *of* linked data-blocks from the paths of the library files themselves, and treats them as two different things you can ask for or skip:

> "file_path_map(*, subset=None, key_types=None, include_libraries=False) […] include_libraries (bool) – Include library file paths of linked data. False by default."

— https://docs.blender.org/api/5.0/bpy.types.BlendData.html, read 2026-07-30. And among the flags for `file_path_foreach`:

> "SKIP_LINKED: Skip Linked. Skip paths of linked IDs."

— Blender Python API, File Path Foreach Flag Items, https://docs.blender.org/api/5.0/bpy_types_enum_items/file_path_foreach_flag_items.html, read 2026-07-30.

So in the running linking file the paths of linked data-blocks are present and can be walked — but they belong to the linked data-blocks, which belong to the library. The sources nowhere describe the linking file storing a second copy of them.

The linked data cannot be edited from the linking file: "Linked data-blocks are initially not editable. This even includes the location/rotation/scale of linked objects" — https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30. Which is the same statement seen from the other side: a path inside a linked node setup is the library's to change, not the linking file's.

### How the files are reached

**The library file is reached first, by its recorded path.** If that fails:

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

— https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30. The repair is Relocate Library: "It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed)". A placeholder is visible in the API as `is_missing — This data-block is a place-holder for missing linked data` — https://docs.blender.org/api/5.0/bpy.types.ID.html, read 2026-07-30. Refreshing without reopening is Reload Library: "choose Reload to immediately update the current blend-file with the latest version of the linked data-blocks, without having to reopen the file."

**A relative path found inside the linked setup is measured from the library file, not from the linking file.** Two documented statements and one observed one.

Documented, in the manual, about path template variables:

> "blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from.
>
> blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."

— https://docs.blender.org/manual/en/5.0/files/file_paths.html, read 2026-07-30. (The same page also lists a variable available "only in path properties on a node": "node_name: The name of the node that the path property is on." — so the sources do contemplate path properties living on nodes. It also warns that "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node.") The 5.0 release notes record these variables as new: "The following new path template variables have been introduced […] `blend_name_lib` … `blend_dir_lib`" — https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30.

Documented, in the API, about turning a stored path into a real one:

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the "//" prefix.
>
> start (str | bytes) – Relative to this path, when not set the current filename is used.
>
> library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

— https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30.

**Observed**, in Cycles' own files, showing that machinery being used on a *node's* path with the *node tree's* library. What I examined: the file `Blender Sources/cycles_5.0/osl.py`. The exact lines:

- line 190: `def update_external_script(report, filepath, library):`
- line 197: `    script_path = bpy.path.abspath(filepath, library=library)`
- line 278: `        ok, oso_path, oso_file_remove = update_external_script(report, node.filepath, node.id_data.library)`

and, for the internal-script case, line 237: `    osl_path = bpy.path.abspath(script.filepath, library=script.library)`.

Read 2026-07-30. What the file says: when Cycles resolves the file path stored on a script node, it passes the library of the node's owning data-block (`node.id_data.library`) as the `library` argument — which, per the API quote above, replaces the starting directory. That is the file speaking, and it is solid ground.

**Inferred**, and this is the step: *that Blender resolves relative paths on other node types inside a linked node tree the same way — against the library file's directory.* The step I took: Cycles' file shows one node type doing it; the API's `library` argument and the manual's `blend_dir_lib` describe the rule in general terms ("path properties on library-linked data-blocks"); I joined those. What a render actually does with a linked geometry-node Import path is a step beyond what any file I read states, and I did not see it done.

**Making the whole thing not depend on the outside files at all: packing.** The manual offers "Pack Linked Libraries — Mark all linked library files in the current blend-file as packed. Actual packing will happen on the next save of the blend-file." and "Unpack Linked Libraries — Unpack all used linked library files from this blend-file." — https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html, read 2026-07-30. The 5.0 release notes describe what packed linked data now is, and use a node group as their example:

> "Packed linked data are a key part of an improved asset workflow in Blender.
>
> Packed IDs remain considered as linked data (i.e. they cannot be edited), but they are stored in the current blendfile. This means that they:
>
> * Are not lost in case the library data becomes unavailable.
> * Are not changed in case the library data is updated.
>
> These packed IDs are de-duplicated across blend-files, so e.g. if a shot file and several of its dependencies all use the same util geometry node, there will be a single copy of that geometry node in the shot file.
>
> In case there are several versions of a same ID (e.g. linked at different moments from a same library, which has been modified in-between), there will be several packed IDs.
>
> Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
>
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'."

— https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30. This shows up in the API as `ID.is_linked_packed — This data-block is linked and packed into the .blend file`, and on Library as `is_archive`, `archive_libraries` and `archive_parent_library` — https://docs.blender.org/api/5.0/bpy.types.ID.html and https://docs.blender.org/api/5.0/bpy.types.Library.html, read 2026-07-30. `bpy.data.libraries.load` carries it as an argument: "pack (bool) – If True, and link is also True, pack linked data-blocks into the current blend-file." The Asset Browser's "Pack" import method is the same idea at the asset level (quoted in answer 1).

**One thing the linking file *does* record locally, and it can be a path.** A node group's interface can expose a file-path input: the API has `NodeTreeInterfaceSocketStringFilePath` alongside `NodeTreeInterfaceSocketString` — https://docs.blender.org/api/5.0/bpy.types.NodeTreeInterfaceSocketStringFilePath.html, read 2026-07-30. And the modifier holds its own values for a group's inputs: "Inputs — A list of the node group's inputs which can have unique values even if the group is shared among multiple modifiers." — https://docs.blender.org/manual/en/5.0/modeling/modifiers/geometry_nodes.html, read 2026-07-30. Label for the claim *that a value typed into such an input on a modifier in the linking file is stored in the linking file and measured from it rather than from the library*: **inferred** for the storage (it follows from "unique values … even if the group is shared"), and **not in the sources** for how it is measured — nothing I read says which file such a value is relative to.

**Also recorded locally: what a bake refers to.** Baked geometry can name data-blocks, and the record includes which library each came from:

> "lib_name — Used when the data block is not local to the current .blend file but is linked from some library — Type: string, default "", (readonly, never None)"

— https://docs.blender.org/api/5.0/bpy.types.NodesModifierDataBlock.html, read 2026-07-30, alongside `id_name — Name that is mapped to the referenced data-block`. The manual's user-facing version is the "Data-Block References" panel quoted in answer 1.

**Not covered.** This answer does not settle: what the linking file does with a linked setup's Import-node path specifically — no source I read names geometry-node paths in a linking context; whether *Make Paths Relative* / *Make Paths Absolute* run in the linking file touch paths that belong to linked data (the `SKIP_LINKED` flag shows the distinction exists inside Blender, but the operators' own descriptions say only "all paths to external files"); what happens when a linked setup's image, font or imported file is missing while the library itself is found; where the bake data of a linked node group is written, and whether two files linking the same node group share or separate it (the sources describe this only for physics point caches, a different system, which I deliberately did not carry over); whether an Import node's path can be overridden by a library override; and the exact rules for a library reached through another library beyond the bare existence of `Library.parent`.
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer: which folder a `//` inside a linked setup is measured from, and what the linking file records, list entry 7; what packing a library carries and what it leaves behind, list entry 8; what is recorded when a library links onward, list entry 9; whether the library path is stored relative or absolute, list entry 10. Read this list as "the documentation does not say", not as "nobody knows".)*

**Outside the interface.** The recording and the reaching are file-format and path-resolution behaviour, not interface behaviour, and the sources give the same operations without a window: `bpy.data.libraries.load(filepath, *, link=…, relative=…, …)` performs the link and chooses relative or absolute storage; `Library.reload()` — "Reload this library and all its linked data-blocks"; `bpy.path.abspath(…, library=…)` performs the resolution; `bpy.data.file_path_foreach()` and `file_path_map()` walk the paths. Cycles' own file does its resolution in plain Python with no interface involved. The one part that is stated in interface terms and that I found no non-interface counterpart for is the repair path — Relocate Library and Relocate Linked ID are described only as Outliner context-menu items, and Find Missing Files opens a File Browser. Beyond that, the sources do not say whether anything differs when Blender is driven from outside its own window.

**Brought up to date by hand test, 2026-08-06 — observed.** All of it holds with no window. Linking, packing the library, reading what the linking file records, resolving a `//` path belonging to linked data, and creating placeholders for a library that cannot be found were every one of them done and read from a script in a Blender started with `-b`, on a machine with no display server at all. With the library gone, Blender printed a warning naming it, left a placeholder in its place and exited with code 0. See "Observed by hand test" below, list entries 7, 8, 9 and 10. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers, so this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so two things could be looked at there that no terminal run can settle. The Path field on a linked group's Import node is drawn **greyed out** in the window, in all four versions — while a script writing to that same socket is accepted, which the Linux runs showed. And the modifier for a linked group holding no Bake node shows no Bake panel at all, only a collapsed section headed Manage; that section could not be opened without a click, so what it holds is untested. What a linked group that *does* hold a Bake node gives the modifier was answered instead with no window at all. See "Observed by hand test on the Creator's Mac" below, list entries 7, 8, 9, 10, 15, 16 and 17.

**Operating system.** The sources do not say that any of this differs by operating system. As in answer 1, the only nearby facts are `bpy.path.basename` ("Use for Windows compatibility"), `bpy.path.native_pathsep` and `bpy.path.resolve_ncase` ("Resolve a case insensitive path on a case sensitive system") — https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30 — which say that separators and letter-case are system-dependent in general, but say nothing about linked node setups. Nothing I read describes a library path or a linked data-block's path behaving differently on one system than another.

**Brought up to date by hand test, 2026-08-06 — observed.** Everything below was run on Linux only, where a path is matched letter for letter — see answer 1's observed section, list entry 11. Nothing has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed.** The letter-case test was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — as well, and came out the other way: there the upper-case forms found the file. See answer 1's Mac observed section, list entry 11. So a path is matched letter for letter on Linux and is not on macOS. Everything else here was run on both machines and agreed on both. No Windows machine has run it, so Windows is untested.

**Where I looked, and what I ruled out.** Manual: I read in full `files/linked_libraries/link_append.txt`, `files/linked_libraries/index.txt`, `files/blend/open_save.txt`, `files/blend/packed_data.txt`, `files/file_paths.txt`, `files/data_blocks.txt` (the data-block table and the Name & Rename section), `interface/window_system/topbar.txt` (External Data), `interface/controls/nodes/groups.txt`, `editors/asset_browser.txt` (import methods), `editors/preferences/file_paths.txt`, `editors/preferences/save_load.txt`, `editors/outliner/interface.txt` (display modes), `modeling/geometry_nodes/group.txt`. Searches over the whole manual: `relative path`; `relative to the`; `indirect`; `library (file|blend|path)|the library file`; `missing file|broken link|file not found`; `Import Method`; `essentials`; `case sensitiv|backslash|drive letter|forward slash`; `dependenc` within `files/`. API: `bpy.types.Library`, `bpy.types.LibraryWeakReference`, `bpy.types.ID`, `bpy.types.BlendData` (`file_path_foreach`, `file_path_map`), `bpy_types_enum_items/file_path_foreach_flag_items`, `bpy.types.BlendDataLibraries`, `bpy.types.BlendImportContext`, `bpy.types.BlendImportContextItem`, `bpy.types.BlendImportContextLibrary`, `bpy.path`, `bpy.ops.file` (the missing-files and make-paths operators), `bpy.types.NodesModifierDataBlock`, `bpy.types.NodeTreeInterfaceSocketString` and `…StringFilePath`. Release notes 5.0: `core.md` (Packed Data, More Path Template Variables), `python_api.md` (Core), `assets.md`, `pipeline_io.md` — all read directly. Cycles: searched `cycles_5.0` for `filepath|geometry.?node|library`, then read `osl.py` lines 180–300 in full; `version_update.py` iterates `bpy.data.libraries` for versioning only, which I ruled out as not about paths; `camera.py` line 334 does the same `library=` resolution for a camera's custom script, which I noted but did not use, as it is not a node.

I looked specifically, and did not find, any page describing: what path a *linked* geometry-node Import node is measured from; whether the linking file caches the outside files a linked setup names; or how missing indirect files inside a found library are reported.

I did not need the web for either question, and did not use it.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

Same machine, same date, same Blender and the same settings folder as the observed section under answer 1: **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all**, `/opt/blender/5.0/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, started with `BLENDER_USER_RESOURCES=/root/node-setups/config/5.0`. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`. **Every one of these was run again on the Creator's Mac on 2026-08-07, and all of them agreed; that block follows this one.** The two machines' results are kept apart and never merged.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7)

Two folders were built. `lib/` held `lib.blend`, a `model.obj` with **three** vertices and a `tex.png`; inside `lib.blend`, a node group `LibSetup` with an Import OBJ node whose Path was `//model.obj` and an Image Texture node whose image was `//tex.png`. `shot/` held `use.blend`, which linked `LibSetup` and put it on an object. Then `use.blend` was opened four times: with no `model.obj` beside it; with a **different**, four-vertex `model.obj` beside it; with `lib/model.obj` moved away and the four-vertex copy still beside `use.blend`; and with the whole `lib` folder moved away. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.274  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/use.blend"
00:00.279  reports          | Read library: '/root/node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.308  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.308  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7ac7ec68b820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7ac7ec68b820> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/tex.png', '/root/node-setups/work/06/5.0/lib/model.obj', '/root/node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7ac7ec786a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7ac7ec99cc20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7ac7ec68b820> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7ac7ec786a20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7ac7ec99cc20> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7ac7ec68b820> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.273  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/use.blend"
00:00.278  reports          | Read library: '/root/node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.309  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.309  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7a7f20e6c820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7a7f20e6c820> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/tex.png', '/root/node-setups/work/06/5.0/lib/model.obj', '/root/node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7a7f20f67a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7a7f21183c20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7a7f20e6c820> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7a7f20f67a20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7a7f21183c20> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7a7f20e6c820> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.268  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/use.blend"
00:00.273  reports          | Read library: '/root/node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.284  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/model.obj'.
00:00.284  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/model.obj'
00:00.303  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.303  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7de5ba06c820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7de5ba06c820> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/tex.png', '/root/node-setups/work/06/5.0/lib/model.obj', '/root/node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7de5ba167a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7de5ba383c20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7de5ba06c820> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7de5ba167a20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7de5ba383c20> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7de5ba06c820> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.0/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/root/node-setups/work/06/5.0/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.267  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/use.blend"
00:00.272  reports          | Read library: '/root/node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.272  reports          | WARNING Unable to open '/root/node-setups/work/06/5.0/lib/lib.blend': No such file or directory
00:00.272  reports          | Cannot find lib '/root/node-setups/work/06/5.0/lib/lib.blend'
00:00.273  reports          | LIB: NodeTree: 'LibSetup' missing from '/root/node-setups/work/06/5.0/lib/lib.blend', parent '<direct>'
00:00.277  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x73774246c820> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x73774246c820> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x73774258bc08> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x73774246c820> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.0/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 5.0 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8)

`use.blend` was opened, `bpy.ops.file.pack_libraries()` run, and the result saved as `packed_use.blend`; that file was then opened three times, with everything in place, with `lib.blend` alone moved away, and with the whole `lib` folder moved away. In full:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.270  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/use.blend"
00:00.276  reports          | Read library: '/root/node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0x7a666cc6c820>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x7a666cc6c820> is_linked_packed: False
Info: Saved as "packed_use.blend"
saved: /root/node-setups/work/06/5.0/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.289  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/packed_use.blend"
00:00.295  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.324  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.324  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7acbefa90220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7acbefa90220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.0/lib/tex.png', '/root/node-setups/work/06/5.0/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7acbefb89a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7acbefdca620> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7acbefb89a20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7acbefdca620> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.270  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/packed_use.blend"
00:00.275  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.305  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.305  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x719bc4c71220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x719bc4c71220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.0/lib/tex.png', '/root/node-setups/work/06/5.0/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x719bc4d68a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x719bc4fa9620> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x719bc4d68a20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x719bc4fa9620> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.277  blend            | Read blend: "/root/node-setups/work/06/5.0/shot/packed_use.blend"
00:00.282  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.292  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/model.obj'.
00:00.292  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/model.obj'
00:00.294  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.294  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/06/5.0/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7965b9271220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7965b9271220> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.0/lib/tex.png', '/root/node-setups/work/06/5.0/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7965b9368a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7965b95a9620> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7965b9368a20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7965b95a9620> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.0/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/root/node-setups/work/06/5.0/lib/lib.blend')
Warning: Path '/root/node-setups/work/06/5.0/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/root/node-setups/work/06/5.0/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 5.0 ===
```

#### What a linking file records when its library links onward (list entry 9)

Three folders: `deep/deep.blend` with a node group `DeepSetup` reading `//model.obj` and a **three**-vertex `model.obj` beside it; `mid/mid.blend` linking `DeepSetup`, wrapping it in a local group `MidSetup`, with a **four**-vertex decoy `model.obj` beside it; `top/top.blend` linking `MidSetup`, with a **five**-vertex decoy `model.obj` beside it. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
00:00.270  blend            | Read blend: "/root/node-setups/work/09/5.0/top/top.blend"
00:00.275  reports          | Read library: '/root/node-setups/work/09/5.0/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.280  reports          | Read library: '/root/node-setups/work/09/5.0/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/5.0/mid/mid.blend'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/09/5.0/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/5.0/deep/model.obj', '/root/node-setups/work/09/5.0/deep/deep.blend', '/root/node-setups/work/09/5.0/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x729459b41f20> -> {'//model.obj', '//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x72945998cc20> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x7294599a9008> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x72945966c820> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.276  blend            | Read blend: "/root/node-setups/work/09/5.0/top/top.blend"
00:00.281  reports          | Read library: '/root/node-setups/work/09/5.0/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.286  reports          | Read library: '/root/node-setups/work/09/5.0/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/5.0/mid/mid.blend'
00:00.298  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/09/5.0/deep/model.obj'.
00:00.298  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/09/5.0/deep/model.obj'
BLENDER: 5.0.1 OPEN FILE: /root/node-setups/work/09/5.0/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/5.0/deep/model.obj', '/root/node-setups/work/09/5.0/deep/deep.blend', '/root/node-setups/work/09/5.0/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x79f3fc15af20> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x79f3fbfa5c20> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x79f3fbfc2008> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x79f3fbc8b820> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/root/node-setups/work/09/5.0/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/root/node-setups/work/09/5.0/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 5.0 ===
```

#### Whether the library path is recorded relative or absolute, and what Blender ships (list entry 10)

In full:

```
BLENDER: 5.0.1 WORKDIR: /root/node-setups/work/10/5.0
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/root/node-setups/work/10/5.0/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/root/node-setups/work/10/5.0/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: no such attribute
--- the node assets Blender ships with ---
   assets folder: /opt/blender/5.0/5.0/datafiles/assets exists: True
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/nodes/shading_nodes_essentials.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /opt/blender/5.0/5.0/datafiles/assets/nodes/procedural_hair_node_assets.blend
DONE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

Same machine, same date, same Blenders and the same settings folders as the observed Mac section under answer 1: **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**, `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)`, started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/5.0`. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing here measures time or memory. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`.

**Every one of these agreed with Linux**, and the two machines' results are kept separate rather than merged. Two further tests were run that the Linux half did not reach: the window tests, list entries 14 and 15, which need a screen; and two the Mac half showed the list had missed, list entries 16 and 17.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7) — Mac

Agreed with Linux: the `//` was measured from the **library** file, not from the linking file. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.328  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.331  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.349  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.349  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xad8f85e20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xad8f85e20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xadb59f120> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xadb7c8920> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xad8f85e20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xadb59f120> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xadb7c8920> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0xad8f85e20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.290  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.294  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.309  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.309  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x8b0ecc620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x8b0ecc620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x8b0b25520> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8b0aed520> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x8b0ecc620> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x8b0b25520> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8b0aed520> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x8b0ecc620> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.287  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.290  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.295  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj'.
00:00.295  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj'
00:00.305  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.305  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xc1da78c20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xc1da78c20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xc1d4d3820> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc1d78db20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xc1da78c20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xc1d4d3820> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc1d78db20> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xc1da78c20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.281  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.284  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.284  reports          | WARNING Unable to open '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend': No such file or directory
00:00.284  reports          | Cannot find lib '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend'
00:00.285  reports          | LIB: NodeTree: 'LibSetup' missing from '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', parent '<direct>'
00:00.285  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x872614020> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
   local=True : ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x872614020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x8751fed08> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x872614020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 5.0 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8) — Mac

Agreed with Linux. The whole run, uncut:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.285  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.288  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0xc39360620>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xc39360620> is_linked_packed: False
Info: Saved as "packed_use.blend"
saved: /tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.264  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend"
00:00.268  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.284  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.284  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x850529220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x850529220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x853f95520> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x85430d220> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x853f95520> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x85430d220> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.264  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend"
00:00.267  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.282  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.282  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x9a036a420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x9a036a420> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x99fe90e20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x99f87f620> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x99fe90e20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x99f87f620> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.278  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend"
00:00.281  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.285  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj'.
00:00.285  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj'
00:00.286  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'.
00:00.286  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.0/lib/written-from-linking-file.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.0/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 0, 119)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xa936dd220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xa936dd220> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.0/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xa94654e20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xa93209b20> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xa94654e20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xa93209b20> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.0/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend')
Warning: Path '/tmp/smoozy-node-setups/work/06/5.0/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 5.0 ===
```

#### What a linking file records when its library links onward (list entry 9) — Mac

Agreed with Linux. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
00:00.310  blend            | Read blend: "/tmp/smoozy-node-setups/work/09/5.0/top/top.blend"
00:00.314  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.0/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.317  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.0/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/5.0/mid/mid.blend'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/09/5.0/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/5.0/deep/model.obj', '/tmp/smoozy-node-setups/work/09/5.0/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/5.0/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0xc2926f920> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0xc2926f620> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0xc29bbc608> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0xc29bc3620> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.291  blend            | Read blend: "/tmp/smoozy-node-setups/work/09/5.0/top/top.blend"
00:00.294  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.0/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.298  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.0/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/5.0/mid/mid.blend'
00:00.303  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/09/5.0/deep/model.obj'.
00:00.303  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/09/5.0/deep/model.obj'
BLENDER: 5.0.1 OPEN FILE: /tmp/smoozy-node-setups/work/09/5.0/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/5.0/deep/model.obj', '/tmp/smoozy-node-setups/work/09/5.0/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/5.0/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0xa6fe29220> -> {'//model.obj', '//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0xa6fe28920> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0xa6ebc9808> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0xa6ebc8620> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/tmp/smoozy-node-setups/work/09/5.0/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/tmp/smoozy-node-setups/work/09/5.0/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 5.0 ===
```

#### Whether the library path is recorded relative or absolute (list entry 10) — Mac

Agreed with Linux. The whole run, uncut. **The last part of it, which looks for the node files Blender ships, found nothing on this Mac** — the check looks beside the Blender program, which is where those files sit on Linux and is not where they sit on macOS. That is a fault of the check, not a fact about Blender, and it is recorded as it stands; list entry 16 below asks the same question in a way that works here.

```
BLENDER: 5.0.1 WORKDIR: /tmp/smoozy-node-setups/work/10/5.0
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/5.0/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/5.0/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: no such attribute
--- the node assets Blender ships with ---
   assets folder: /Applications/Blender 5.0.1.app/Contents/MacOS/5.0/datafiles/assets exists: False
DONE
```

#### Which node `.blend` files each Blender ships (list entry 16) — Mac

The shipped-file half of list entry 10, asked again by way of Blender's own answer to where its files are. The whole run, uncut:

```
BLENDER: 5.0.1
binary path: /Applications/Blender 5.0.1.app/Contents/MacOS/Blender
the folder the earlier check looked in: /Applications/Blender 5.0.1.app/Contents/MacOS/5.0/datafiles/assets
   exists: False
bpy.utils.resource_path('LOCAL'): /Applications/Blender 5.0.1.app/Contents/Resources/5.0
assets folder: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets exists: True
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/nodes/procedural_hair_node_assets.blend
   shipped .blend: /Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/nodes/shading_nodes_essentials.blend
count of shipped .blend files under the assets folder: 13
DONE
```

#### What a linked group holding a Bake node gives the modifier, and where that bake goes (list entry 17) — Mac

Built because the linked group in list entry 7 holds no Bake node, so list entry 15 found the modifier had no bake to show. A library file held a group of Group Input, Bake, Group Output; a second file linked that group onto a cube; then the second file was opened, read and baked. The report and the bake, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/bakeuse.blend and report, then bake ===
00:00.271  blend            | Read blend: "/tmp/smoozy-node-setups/work/17/5.0/shot/bakeuse.blend"
00:00.275  reports          | Read library: '/tmp/smoozy-node-setups/work/17/5.0/lib/bakelib.blend', '//../lib/bakelib.blend', parent '<direct>'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
BLENDER: 5.0.1 STEP: report WORKDIR: /tmp/smoozy-node-setups/work/17/5.0
node group: BakeLibSetup library: //../lib/bakelib.blend
modifier bake_target: PACKED
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=191371534 bake_target=INHERIT directory=''
files beside the .blend before baking: ['bakeuse.blend', 'bakeuse.blend1']
Info: Bake directory of object Target, modifier GN is empty, setting default path
bake operator returned: {'FINISHED'}
modifier bake_directory after: '//blendcache_bakeuse/Target_GN'
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/191371534/meta/00001_00000.json
DONE
```

The bake went beside the **linking** file, not beside the library: `blendcache_bakeuse` sits in the `shot` folder, next to `bakeuse.blend`, and the modifier's own Bake Path filled itself in as `//blendcache_bakeuse/Target_GN`, measured from the linking file. The library folder gained nothing.


#### Whether a linked group's Path field can be typed into in a window, and what the modifier shows (list entry 15) — Mac only

The Linux machine has no display server, so this cannot be reached there at all. Blender was started **with** its window, opening the `use.blend` that list entry 7 built, and driven by a script that showed the linked group in the geometry node editor, took Blender's own screenshot, then switched the properties editor to the modifier tab and took a second one, before quitting. No mouse was moved and nothing was clicked.

The first form of the script hit the same trouble as list entry 14 — every node in the group had been made at the same spot, so only the topmost was visible. It is recorded as it came back and settles nothing. The second form spread the nodes apart first. Blender printed:

```
00:00.720  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.723  blend.readfile   | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xc49f75220> indirect: False missing: False
moving the linked group's nodes apart was accepted
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (400.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (800.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (1200.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (1600.0, 0.0)
    Image Texture GeometryNodeImageTexture (2000.0, 0.0)
Warning: 2 × Draw window and swap: 75.2180 ms, average: 37.60898113 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 22.1140 ms, average: 11.05701923 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.0/linked-nodes.png
Path socket on the linked Import OBJ node holds: '//model.obj'
Path socket: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
the node group's own library_weak_reference: None
modifier: GN type: NODES
   bake_target: PACKED bake_directory: ''
   number of bake entries: 0
Warning: 2 × Draw window and swap: 11.3649 ms, average: 5.68246841 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.0/modifier-panel2.png
00:01.158  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

and, on the run that tried to open the Manage section:

```
00:00.562  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.0/shot/use.blend"
00:00.566  blend.readfile   | Read library: '/tmp/smoozy-node-setups/work/06/5.0/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
modifier: GN bake entries: 0 bake_target: PACKED bake_directory: ''
ui_expand_flag before: no such property
setting ui_expand_flag raised: AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'
Warning: 3 × Draw window and swap: 139.8320 ms, average: 46.61067327 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.0/modifier-expanded.png
00:00.965  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

**The Path field on the linked group's Import OBJ node is drawn greyed out.** Put beside the screenshot from list entry 14, where the same node in a **local** group draws `//model.obj` in bright white with a bright folder button, the linked one draws the same text and the same button dimmed. All four versions drew it dimmed. That is worth setting against what list entry 7 found from a script: writing a new value into that same socket from Python **was** accepted. So the window and the script route differ — the window will not let the value be typed, and a script will.

The whole of the right-hand Group panel was drawn greyed out in the same way, in all four versions — the group's Name, its Node Width and the plus and minus buttons beside its sockets.

**The modifier's Bake panel was not there for this group.** The modifier showed only its node-group field, holding `LibSetup` and greyed out, and one collapsed section headed **Manage**. Blender reported `number of bake entries: 0` for the modifier, which is what a group holding no Bake node gives it — the group built for list entry 7 has none. The Manage section could not be opened from a script: `NodesModifier` has no property for the open-and-shut state of its sections, and the attempt printed `AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'`. Opening it needs a click, which is not allowed here, so **what the Manage section holds for a linked group is untested**. What a linked group with a Bake node in it gives the modifier is answered instead by list entry 17 above, without a window.

The screenshots are kept at `/tmp/smoozy-node-setups/work/15/5.0/` on that machine. This settles the drawing for **macOS only**; there is no Linux machine with a screen, and no Windows machine ran it.


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

2026-08-06 — Opus 5 (Claude Code) — Node setups, Blender 5.0

# Node setups — Blender 5.1 (second pass)

**Sources consulted, and when.** Blender 5.1 manual (`manual_5.1`, including its `_images` pictures), Blender 5.1 Python API documentation (`api_5.1`), Blender 5.1 release notes (`release_notes_5.1`), and Cycles' own files (`cycles_5.1`) — all read 2026-07-30. One quote is taken from the Blender 4.5 release notes (`release_notes_4.5`), read 2026-07-30, and is clearly marked as such; it is used only to record when a feature was added, never as a statement about 5.1. Blender's requirements page was not needed and was not used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**LABEL: documented**, with two parts marked **observed** and **inferred** where noted below.

The sources describe five distinct ways a geometry node setup reaches outside its own `.blend` file. I take each in turn.

### 1a. Six import nodes, each holding a path to a file of a named format

The manual has a whole category for these.

> "Import Nodes
>
> Nodes used to import data from external file formats.
>
> Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor.
>
> When dropping a supported file into a Geometry Node tree,
> the corresponding import node is added automatically and configured with the file path.
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

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/index.html — read 2026-07-30.

Each node's own page names the format and the path input:

- **`.obj`** — "The Import OBJ node loads geometry data from a .obj file and outputs it as instances." Input: "Path — The path to the .obj file to be imported." The page also carries a limit worth recording: "The node imports only mesh geometry; materials and textures are not imported." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/obj.html)
- **`.ply`** — "The Import PLY node imports mesh data from a .ply (Polygon File Format or Stanford Triangle Format) file." Input: "Path — The path to the .ply file to be imported." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/ply.html)
- **`.stl`** — "The Import STL node imports mesh geometry from a .stl (Stereolithography) file." Input: "Path — The path to the .stl file to be imported." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/stl.html)
- **`.csv`** — "The Import CSV node reads data from a CSV (Comma-Separated Values) file and generates a point cloud." Input: "Path — The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/csv.html)
- **`.txt`** — "The Import Text node reads the contents of a plain text file and outputs it as a single string." Input: "Path — The path to the txt file." And a stated restriction: "Currently, only files with the .txt extension are supported." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/txt.html)
- **`.vdb`** — "The Import VDB node loads volumetric data from a .vdb file and outputs it as a Volume geometry. All grids present in the file are loaded and included in the resulting volume." Input: "Path — The path to the VDB file to be imported." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/vdb.html)

All read 2026-07-30.

I checked that this list of six is complete for 5.1 by listing every node type documented in the Python API whose name begins `GeometryNode` (252 pages) and every one beginning `FunctionNode`, then filtering for names containing *file*, *path*, *script*, *import*, *font*, *image*, *text*, *sound* or *movie*. The only import nodes present are `GeometryNodeImportCSV`, `GeometryNodeImportOBJ`, `GeometryNodeImportPLY`, `GeometryNodeImportSTL`, `GeometryNodeImportText`, `GeometryNodeImportVDB`. (Blender 5.1 Python API, https://docs.blender.org/api/5.1/ — read 2026-07-30.) There is no Alembic, USD, glTF or FBX import node, and no script node, in geometry nodes.

**The path is a socket, not a fixed setting on the node.** This matters, and the manual shows it twice.

First, in the picture of the node. The manual's Import OBJ page carries `[image: _images/node-types_GeometryNodeImportOBJ.webp]`. Looking at that picture: the Path field sits in the node body with a small folder icon on its right, and a blue socket dot on the node's left edge level with it. What is visible is that Path is a connectable socket that also offers a file browser button. (Blender 5.1 manual image `_images/node-types_GeometryNodeImportOBJ.webp`, belonging to https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/obj.html — read 2026-07-30. Label: **documented**, since the picture is part of the manual.)

Second, and more strongly, the manual shows a worked example of a path being built at evaluation time. The Format String node page carries `[image: _images/importing_csv.jpeg]`, captioned "Format String node example with CSV data import." In that screenshot the Import CSV node's **Path** socket is fed by a link from a Join Strings node, which in turn joins a String node holding `/Users/jan-hendrik/Downloads/` with the output of a Format String node whose Format is `data_{:#}.csv` and whose value input comes from the **Frame** output of a Scene Time node. So the file that gets read is computed per frame from other node values, not stored as a single constant. (Blender 5.1 manual image `_images/importing_csv.jpeg`, belonging to https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/utilities/text/format_string.html — read 2026-07-30. Label: **documented**.)

The same page's own examples confirm the shape of such a computed path:

> "Path with Frame Number
>
> Format: /output/image_{:04}.png
>
> Input: Integer 42
>
> Result: /output/image_0042.png"

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/utilities/text/format_string.html — read 2026-07-30.

*(Note added by the citation check, 2026-08-07: the sentence quoted next is not in the 5.1 sources at all — it is in the 4.5 release notes, which is the address given. Findings are meant to be checked against their own version's copy of the documentation, so this one crosses that line; it is kept because the document says plainly, in the same breath, that it is a statement about 4.5 and is quoted only for provenance.)*

The Blender **4.5** release notes, where these six nodes were introduced, say the same thing in one line: "Loading sequences is currently best done with the new *Format String* node." (https://developer.blender.org/docs/release_notes/4.5/geometry_nodes/ — read 2026-07-30. Quoted only to record where the nodes came from; it is a statement about 4.5, not 5.1.)

The Python API pages for the six import node types list **no path property at all** — only `is_registered_node_type`, `input_template` and `output_template`. For example the whole substantive text of `GeometryNodeImportOBJ` is "Import geometry from an OBJ file". (https://docs.blender.org/api/5.1/bpy.types.GeometryNodeImportOBJ.html — read 2026-07-30.) So the path really does live in a socket value, not in a named property on the node.

The socket type that carries such a value is documented:

> "NodeSocketStringFilePath(NodeSocketStandard) […] String socket of a node
>
> default_value
>
> (default “”, never None, blend relative // prefix supported)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.NodeSocketStringFilePath.html — read 2026-07-30.

"blend relative // prefix supported" is the same wording the API uses for `Image.filepath`, `VectorFont.filepath` and `Library.filepath`, so a `//` path in an import node's Path socket is a relative path in the ordinary Blender sense.

### 1b. Image files, image sequences, movie files and UDIM tiles, through an Image data-block

Three geometry nodes take an Image:

> "The Image Texture node is used to add an image file as a texture. The image data is sampled with the input Vector and outputs a Color and Alpha value."

and, on the same page, about the **Frame** input:

> "If the Image supports animation, the frame can be set here. This can be keyframed so that the image changes between frames."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/texture/image.html — read 2026-07-30.

> "The Image node provides access to a image file which allows you to conveniently enter and switch images for multiple nodes in the tree. […] Image Data-Block — The data-block selector to select an existing image or open an image from the file browser."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/constant/image.html — read 2026-07-30.

> "The Image Info node gets information from image and animation. […] Frame Count — The number of frames in an image or video frame sequence. For a static image, always 1. FPS — The number of frames per second. For static image is always 0."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/scene/image_info.html — read 2026-07-30.

What an Image data-block can be pointed at is spelled out in the API:

> "source
>
> Where the image comes from (default 'FILE')
>
> FILE Single Image – Single image file.
>
> SEQUENCE Image Sequence – Multiple image files, as a sequence.
>
> MOVIE Movie – Movie file.
>
> GENERATED Generated – Generated image.
>
> VIEWER Viewer – Compositing node viewer.
>
> TILED UDIM Tiles – Tiled UDIM image texture."

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.Image.html — read 2026-07-30.

So through these three nodes a geometry node setup can make the file point at a single image file, a numbered sequence of image files, a movie file, or a set of UDIM tiles. The step from "the node takes an Image" to "the file therefore points at whatever files that Image points at" is small but it is a step, so: **inferred** for that joining, **documented** for each half.

I looked at the manual's pictures of both nodes (`_images/node-types_GeometryNodeImageTexture.webp` and `_images/node-types_GeometryNodeInputImage.webp`). Both show a data-block selector row with a "New" button and an "Open" button beside a folder icon. What is visible is that an image can be opened from disk directly on the node.

### 1c. A font file, through the Font socket on String to Curves

> "Font
>
> Font glyph used to generate the curve."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/utilities/text/string_to_curves.html — read 2026-07-30.

The 5.1 release notes state that this became a socket in this version:

> "The String to Curves node was improved in multiple ways […] Most importantly, all inputs are now exposed as sockets, including the font in form of a new socket type."

and, in the bullet list on the same page:

> "There is a new **Font** socket (blender/blender@624f454e08b9e33464e662307ab3d34157b90030)."

Source: Blender 5.1 release notes, https://developer.blender.org/docs/release_notes/5.1/geometry_nodes/ — read 2026-07-30.

The socket carries a `VectorFont`:

> "NodeSocketFont(NodeSocketStandard) […] Font socket of a node
>
> default_value
>
> Type: VectorFont"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.NodeSocketFont.html — read 2026-07-30.

And a `VectorFont` holds a file path:

> "VectorFont(ID) […] Vector font for Text objects
>
> filepath
>
> (default “”, never None, blend relative // prefix supported)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.VectorFont.html — read 2026-07-30.

I looked at `_images/node-types_GeometryNodeStringToCurves.webp`. What is visible: a third input row showing an "F" data-block selector reading "Bfont Reg…", followed by a shield icon, a folder icon and an X. So a font can be picked or opened from the node itself.

### 1d. Baked data written to, and afterwards read from, a directory on disk

> "Baking allows saving and loading intermediate geometries. […] Data can be baked using two methods: Bake Node – used to bake any portion of the node tree. Simulation Zone Baking – used to bake animations where the result of one geometry state can influence the next state.
>
> Data is not stored to disk until the Bake operation is ran.
>
> Important
>
> Blend-files must be saved to a disk before data can be baked."

and

> "The data format used to store geometry data is not considered to be an import/export format. Volume objects, however, are saved using the OpenVDB file format which can be used interoperably."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/baking.html — read 2026-07-30.

The Bake node page gives the choice of where it goes:

> "Bake Target
>
> Specifies where the baked data should be stored.
>
> Inherit from Modifier: The setting is copied from the modifier that contains the bake node. This is the default.
>
> Packed: The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk: The baked data is stored in a separate directory on disk.
>
> Custom Path
>
> Specify a path where the baked data should be stored manually.
>
> Bake Path
>
> Location on disk where the baked data is stored.
> Note, this path is also used for simulation zones."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-07-30.

The Python API confirms the paths are ordinary Blender paths, and that there are two of them — one on the modifier and one per bake:

> "bake_directory
>
> Location on disk where the bake data is stored (default “”, never None, blend relative // prefix supported)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.NodesModifier.html — read 2026-07-30.

> "directory
>
> Location on disk where the bake data is stored (default “”, never None, blend relative // prefix supported)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.NodesModifierBake.html — read 2026-07-30.

Note the ownership: `bake_directory` is a property of the **modifier**, not of the node tree, and `NodesModifierBake.directory` likewise. So the directory that a Bake node's data lands in is recorded outside the node setup even though the Bake node is inside it. That matters for question 2, and I return to it there.

### 1e. Other `.blend` files, through data-blocks the setup names

A geometry node setup can name a Material, an Object or a Collection:

> "The Material input node outputs a single material." / "The Object input node outputs a single object." / "The Collection input node outputs a single collection."

Sources: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/constant/material.html , .../object.html , .../collection.html — read 2026-07-30.

Any of these — and the Image and the Font above, and a nested node group — may be a data-block linked from another `.blend` file, in which case the file points at that `.blend`:

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

> "Library(ID) […] External .blend file from which data is linked
>
> filepath
>
> Path to the library .blend file (default “”, never None, blend relative // prefix supported)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.Library.html — read 2026-07-30.

The joining of those two — "a geometry node's data-block socket may hold a linked data-block, therefore the setup can make the file point at another `.blend`" — is **inferred**. The sources say each half; they do not say the two together for geometry nodes specifically.

The same reasoning gives a further reach that I record but do not chase: a Material named by a geometry node setup carries its own shader node tree, which has its own file references. The sources do not join those two either.

### 1f. A file path exposed as a node group input

A node group's own interface can declare a string input that is a path:

> "Strings
>
> None: Standard text string.
>
> File Path: The string is interpreted as a path to a file, enabling file selection dialogs in the user interface."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/groups.html — read 2026-07-30.

The matching API type exists: `NodeTreeInterfaceSocketStringFilePath`, "String socket of a node", with a `default_value` described as "Input value used for unconnected socket". (https://docs.blender.org/api/5.1/bpy.types.NodeTreeInterfaceSocketStringFilePath.html — read 2026-07-30.)

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

- Whether these paths take part in Blender's general external-file housekeeping. I could not settle whether **Pack Resources**, **Make Paths Relative**, **Make Paths Absolute**, **Report Missing Files** or **Find Missing Files** reach the path value sitting in an import node's socket. The manual describes all five only in general terms — "Pack all used external files into the blend-file", "Make all paths to external files relative to the current blend-file", "Report all missing external files" (https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html and https://docs.blender.org/api/5.1/bpy.ops.file.html, read 2026-07-30) — and never says what counts as an external file for this purpose. See the hand test at the end.
- Which image, movie and font file formats are accepted. Not chased; the question asks what kinds of files, not which encodings.
- What happens when a path points at nothing. The modifier page says only "Warnings — Nodes that show a warning message in the node editor will also show that message here" (https://docs.blender.org/manual/en/5.1/modeling/modifiers/geometry_nodes.html). It does not say a missing file produces such a warning.
- Where baked data goes when the bake path is left empty. `bake_directory` defaults to `""`; no default location is documented for geometry node bakes. (The manual does document a default for *physics* caches — "the drive in a folder named blendcache_[filename] alongside the blend-file", https://docs.blender.org/manual/en/5.1/physics/baking.html — but that page is about physics, not geometry nodes, so I do not carry it across.)
- Anything a geometry node setup might read that is not a file — network locations, standard input, and so on. Nothing in the sources suggests any such thing exists; I did not find it and did not look hard for it.

### Where I looked

- Listed the whole `modeling/geometry_nodes` tree of the 5.1 manual, then read `input/import/index.txt` and all six import node pages; `input/constant/index.txt` and `image.txt`, `material.txt`, `object.txt`, `collection.txt`; `input/scene/index.txt` and `image_info.txt`; `texture/image.txt`; `utilities/text/string_to_curves.txt` and `format_string.txt`; `baking.txt`; `geometry/operations/bake.txt`; `index.txt`; and `modeling/modifiers/geometry_nodes.txt`.
- Grepped that whole tree for: `import`, `file`, `cache`, `sequence`, `alembic`, `usd`, `ies`, `sound`, `movie`, `video`, `udim`, `script`. The only hits outside what is quoted above were incidental uses of the word "file" meaning the `.blend` file, and one photo credit.
- Listed every `bpy.types.GeometryNode*` and `bpy.types.FunctionNode*` page in the 5.1 API (252 geometry node pages) and filtered by name for file-related words. Read `GeometryNodeImportOBJ`, `GeometryNodeImportCSV`, `GeometryNodeImportVDB`, `NodeSocketStringFilePath`, `NodeTreeInterfaceSocketStringFilePath`, `NodeSocketFont`, `NodeTreeInterfaceSocketFont`, `VectorFont`, `Image`, `NodesModifier`, `NodesModifierBake`.
- Searched the whole 5.1 API for `FILE_PATH` (hits: `Operator`, `AddonPreferences`, `FileHandler`, `WindowManager`, and two property-subtype enum lists — none of them node sockets, which is how I found the socket types by name instead).
- Searched the whole 5.1 manual for `Bake Directory`, `bake_directory`, `blendcache`, `bake path`, `Bake Path`.
- Read the 5.1 release notes for geometry nodes, core, pipeline & I/O and the Python API in full; grepped all 5.1 release note files for `librar`. Read the 4.5 geometry nodes release note section on import nodes, for provenance only.
- Opened four manual pictures: `node-types_GeometryNodeImportOBJ.webp`, `node-types_GeometryNodeStringToCurves.webp`, `node-types_GeometryNodeImageTexture.webp`, `node-types_GeometryNodeInputImage.webp`, and the example `importing_csv.jpeg`.
- Grepped Cycles' own files (`cycles_5.1`) for `geometry_nodes`, `GeometryNode`, `node_group`, `filepath`, `FILE_PATH`. Cycles has nothing about geometry nodes' file references; the only matches were a shader kernel header using a compile-time switch named `ADAPT_TO_GEOMETRY_NODES`, and the OSL shader-script handling I use in question 2. Ruled out as not bearing on this question.

### Outside the interface

Mostly holds, with two documented exceptions and one gap.

- The two exceptions are both about *creating* the reference rather than *having* it. Drag-and-drop is stated as an editor action: "Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor" (manual, import nodes index). And the File Path socket subtype is described as "enabling file selection dialogs in the user interface" (manual, node groups). Both need a person at a window. Neither is needed for a setup that already exists.
- The gap: the sources nowhere state that an existing geometry node setup reads the same files when Blender is run without its window. That the paths are stored data and so unchanged is **inferred**, not documented. The one hard fact I can offer on the far side is that the bake operators exist as callable operators, not only as buttons — `bpy.ops.object.geometry_node_bake_single`, `geometry_node_bake_delete_single`, `geometry_node_bake_pack_single`, `geometry_node_bake_unpack_single` (https://docs.blender.org/api/5.1/bpy.ops.object.html — read 2026-07-30). The API does not say whether they run without a window.
- One related documented fact about paths in general, from the command line page: "-o, --render-output <path> — Set the render path and file name. Use // at the start of the path to render relative to the blend-file." (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.) That establishes that `//` paths are meaningful outside the interface, but it is about the render output path, not about node sockets.

**Brought up to date by hand test, 2026-08-06 — observed.** A Blender started with no window does read an import node's file when it opens the `.blend`, and does read it again at render time; the rendered picture was the same with `--disable-depsgraph-on-file-load` in force as without it. Every route in this answer was built, set and read from a script in a Blender with no window at all, on a machine with no display server, so nothing here needs a person clicking. The one exception is dragging a file into the node editor to make an import node: no script can make a real drag happen, so it cannot be tested at all and sits in the "Cannot be run" section of `Audits/Blender Research/Node setups/Hand tests.md`. See "Observed by hand test" below, list entries 4 and 12. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers: a Blender started with no window reads an import node's file when it opens the `.blend`, and again at render time, and the picture was the same with `--disable-depsgraph-on-file-load` in force as without it. So this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so one thing could also be looked at there that no terminal run can settle: the Path row is drawn as a text field holding the path, with a folder button at its right-hand end and a socket dot on the node's left edge level with it, and the value it shows is the one a script reads back. See "Observed by hand test on the Creator's Mac" below, list entries 4, 12 and 14. Dragging a file into the node editor to make an import node still cannot be tested at all, on either machine.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back was built, changed and read entirely from a script in a Blender with no window at all, on a machine with no display server, so nothing in it needs a person clicking. It also turns up something anyone driving Blender from outside should know: asking for the evaluated geometry again does not by itself make an import node read its file again. The setup has to be marked for a fresh run, **and** the file's modification time has to have changed by a whole second. A script that writes a file and immediately asks for the geometry can therefore be handed the old contents, with nothing said about it. Observed on Linux in all four versions; untested on macOS, because the rented machine was given up before the Mac half could be run, and Windows was not tried at all. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Operating system

The sources do not say. Every page quoted above is written without any operating-system qualification, and I found none anywhere near them. In particular nothing states how the `//` prefix, path separators, or drive letters behave differently across systems for any of these node paths.

**Brought up to date by hand test, 2026-08-06 — observed.** On Linux a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, and an image path behaves the same way — see "Observed by hand test" below, list entry 11. That is the one place these tests found an operating system could make a difference, and it has been settled for Linux only. Nothing here has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed. This is the one place the two machines disagreed.** The same test on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — came out the other way: `//MODEL.OBJ` and `//Model.Obj` both found `model.obj`, an image path in upper case loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back instead of correcting it to the file that exists. So **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. The two results are kept apart and never merged into one. Windows is still untested, because no Windows machine has run it. Everything else in this answer was run on both machines and agreed on both. See "Observed by hand test on the Creator's Mac" below, list entry 11.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back found a second place where an operating system could make a difference, and it could only be run on one of them. Whether an import node reads its file again turns on the file's modification time, and on the *whole second* of it: a change inside the same whole second is not noticed, a change that crosses one is, in either direction. That was settled on the rented Linux machine's ordinary file system, in all four versions, across eighty runs with no exception. Whether macOS keeps and compares that stamp the same way is **untested** — the rented machine was given up before the Mac half could be run — and no Windows machine has run it. So this holds for Linux and is untested on both of the others. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

All of this was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all** — on **2026-08-06**, with `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/root/node-setups/config/5.1`, a settings folder made for these tests before anything was started, never Blender's real one. The machine was shared with other work while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below. **Every one of these was run again on the Creator's Mac on 2026-08-07; that block follows this one. All but one agreed. The one that did not is letter case, list entry 11.** The two machines' results are kept apart and never merged.


#### What the build itself reports about the import nodes (list entry 1)

Started Blender with no window and a script that listed the classes the build registers, then built a throwaway geometry node tree and read the socket type of every input on each import node, tried to link a String node into the Path input, and looked at String to Curves. Blender printed, in full:

```
BLENDER: 5.1.2
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
252
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
links.new returned: <bpy_struct, NodeLink at 0x7d681b2016d8>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Font', 'NodeSocketFont'), ('Align X', 'NodeSocketMenu'), ('Align Y', 'NodeSocketMenu'), ('Pivot Point', 'NodeSocketMenu'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Overflow', 'NodeSocketMenu'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: False
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

#### What path-related facilities this build has (list entry 2)

The same shape of run, reading each type's properties out of Blender's own description of it. In full:

```
BLENDER: 5.1.2
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
['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

One line in that script asks for the flag list of `file_path_foreach` and comes back with a `KeyError`; that line settles nothing either way and is left in the record as it stands. An earlier form of the whole check asked `hasattr` on those classes instead and reported every property absent in every version — a fault of the check, not a fact about Blender, and the reason the run above reads them from `bl_rna` instead.

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3)

A folder was made holding `model.obj` (three vertices, with `mtllib model.mtl` and `usemtl red` in it), `model.mtl`, a four-by-four `tex.png`, and `scene.blend` saved beside them. Inside the file one object carried a Geometry Nodes modifier whose group was Import OBJ → Realize Instances → Join Geometry → Group Output, with an Image Texture node holding the image. The Path was set to five values in turn; then, with the file working, `model.obj` was overwritten on disk with a four-vertex shape and the geometry read again after each of several things. In full:

*(Note added by the citation check, 2026-08-07.* This test was rebuilt from the words above and run again on the same rented Linux machine. The first half — which path forms the Path accepts, and what a missing one does — came back identical. The second half depends on something these words do not say: **how** the geometry is read again. Read plainly, it gives three vertices at every step, as recorded below. If the object is marked for update before the evaluated geometry is asked for — an ordinary thing to do, and not ruled out by anything above — every reading from the overwrite onwards comes back as **four**. Both runs happened; neither overwrites the other, and this check did not pick between them. The question has gone back to the hand tests to be run again in a form that says which reading is meant.*)*

```
00:00.399  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.399  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.400  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/03/5.1/nothing-here.obj'.
00:00.400  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/03/5.1/nothing-here.obj'
00:00.407  blend            | Read blend: "/root/node-setups/work/03/5.1/scene.blend"
BLENDER: 5.1.2 WORKDIR: /root/node-setups/work/03/5.1
Info: Saved as "scene.blend"
saved: /root/node-setups/work/03/5.1/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/root/node-setups/work/03/5.1/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/root/node-setups/work/03/5.1/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /root/node-setups/work/03/5.1/model.obj
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
00:00.362  blend            | Read blend: "/root/node-setups/work/03/5.1/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
=== same, with model.obj moved away ===
00:00.368  blend            | Read blend: "/root/node-setups/work/03/5.1/scene.blend"
00:00.378  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/03/5.1/model.obj'.
00:00.378  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/03/5.1/model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
exit code: 0
=== same, with --disable-depsgraph-on-file-load ===
00:00.395  blend            | Read blend: "/root/node-setups/work/03/5.1/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

The third run does not isolate what `--disable-depsgraph-on-file-load` does, because the check itself asks for an evaluated dependency graph, which builds one. The render comparison below answers that half.

#### Whether a render with no window reads the file, judged by the picture (list entry 12)

A second file was built holding the same Import OBJ node, an orthographic camera over the shape, the Workbench engine and a sixty-four by sixty-four frame, and frame 1 was rendered four times. The pictures were compared colour value by colour value, not byte for byte, because Blender writes text including times into every image it saves. The steps that carry the answer, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /root/node-setups/work/12/5.1/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.1/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/5.1/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.1/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4316
--- without the file, against without the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/5.1/without_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/5.1/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.1 ===
```

#### Whether the file read can change from frame to frame (list entry 13)

Three shapes were put on disk — `data_1.obj` with three vertices, `data_2.obj` with four, `data_3.obj` with five — and a node group joined a String node holding `//data_`, a Value to String node fed by the Scene Time node's Frame output, and a String node holding `.obj`, into the Import OBJ node's Path. In full:

```
BLENDER: 5.1.2 WORKDIR: /root/node-setups/work/13/5.1
Info: Saved as "perframe.blend"
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt')]
Join Strings inputs: [('Delimiter', 'NodeSocketString', False), ('Strings', 'NodeSocketString', True)]
links made: 8
frame 1 -> vertices: 3
frame 2 -> vertices: 4
frame 3 -> vertices: 5
frame 1 -> vertices: 3
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

The first form of that script linked the three pieces in reading order and Blender joined them the other way round, giving paths like `.obj1//data_` and no geometry at any frame, with `1`, `2` and `3` appearing in the three failed paths. Both runs happened; the one above is the one that carries the answer.

#### Whether an import node's path takes part in the file's own path housekeeping (list entry 5)

A file was built holding an Import OBJ node's path and an ordinary image path side by side. The image is the control: it is a kind of file Blender's own documentation marks as packable, so a run in which nothing at all moved can be told apart from a run in which only the node's path failed to move. In full:

```
00:00.460  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/05/5.1/gone-model.obj'.
00:00.460  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/05/5.1/gone-model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
BLENDER: 5.1.2 WORKDIR: /root/node-setups/work/05/5.1
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/root/node-setups/work/05/5.1/model.obj'        image filepath = '/root/node-setups/work/05/5.1/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/root/node-setups/work/05/5.1/model.obj'        image filepath = '/root/node-setups/work/05/5.1/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/root/node-setups/work/05/5.1/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/root/node-setups/work/05/5.1/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/05/5.1/tex.png', '/root/node-setups/work/05/5.1/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x7597b88c7208> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0x7597b88cb108> -> {'//model.obj'}
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
00:00.379  blend            | Read blend: "/root/node-setups/work/05/5.1/packed.blend"
00:00.389  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/05/5.1/model.obj'.
00:00.389  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/05/5.1/model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
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
BLENDER: 5.1.2 WORKDIR: /root/node-setups/work/07/5.1
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /root/node-setups/work/07/5.1/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=1355529874 bake_target=INHERIT directory=''
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
   file on disk: <workdir>/blendcache_bake/Target_GN/1355529874/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/mybakes/1355529874/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/1355529874/meta/00001_00000.json
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

#### Whether these paths are matched letter for letter (list entry 11)

With `model.obj` and `tex.png` on disk in lower case. In full:

```
00:00.393  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/11/5.1/MODEL.OBJ'.
00:00.394  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/11/5.1/MODEL.OBJ'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.395  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/11/5.1/Model.Obj'.
00:00.395  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/11/5.1/Model.Obj'
BLENDER: 5.1.2 WORKDIR: /root/node-setups/work/11/5.1
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 0
PATH='//Model.Obj'    -> vertices: 0
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: False size: (0, 0) pixels: 0
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /root/node-setups/work/11/5.1/model.obj
DONE

Blender quit
```

This is Linux, whose ordinary file system tells letter case apart. It says nothing about macOS or Windows.

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

All of this was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**, with `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/5.1`, a settings folder made for these tests before anything was started, never Blender's real one. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below.

Every test in the Linux block above was run again here, in the same order, from the same scripts, with only the paths of the program, the settings folder and the working folder changed. **Eight of the nine agreed with Linux. One did not: letter case, list entry 11.** The results from the two machines are set out separately and are not folded into one.

Where the two agreed, the Mac's own words are given for the lines that carry the answer; the rest of each run printed the same words as the Linux block above, apart from the working-folder paths, the program's own path and the build's hash line, which on this Mac reads `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)`. Where I have kept only part of a long run, I say so at that point. The whole of every run was kept as it was printed, in `/Users/artist/Documents/Smoozy hand tests/Node setups/logs/` on that machine.


#### What the build itself reports about the import nodes (list entry 1) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.1.2
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
252
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
links.new returned: <bpy_struct, NodeLink at 0xc19631008>
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
BLENDER: 5.1.2
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
['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
```

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3) — Mac

Agreed with Linux. The whole run, uncut, including what Blender printed about the paths it could not open:

```
00:00.357  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.357  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.357  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/5.1/nothing-here.obj'.
00:00.357  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/5.1/nothing-here.obj'
00:00.361  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.1/scene.blend"
BLENDER: 5.1.2 WORKDIR: /tmp/smoozy-node-setups/work/03/5.1
Info: Saved as "scene.blend"
saved: /tmp/smoozy-node-setups/work/03/5.1/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/tmp/smoozy-node-setups/work/03/5.1/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/tmp/smoozy-node-setups/work/03/5.1/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /tmp/smoozy-node-setups/work/03/5.1/model.obj
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
########## BLENDER 5.1 — fresh process, file on disk now has 4 vertices ##########
00:00.315  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.1/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)

Blender quit
########## BLENDER 5.1 — same, with model.obj moved away ##########
00:00.322  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.1/scene.blend"
00:00.328  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/5.1/model.obj'.
00:00.328  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/5.1/model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
exit code: 0
########## BLENDER 5.1 — same, with --disable-depsgraph-on-file-load ##########
00:00.317  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.1/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)

Blender quit
```

#### Whether a render with no window reads the file, judged by the picture (list entry 12) — Mac

Agreed with Linux. The comparison of the pictures, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /tmp/smoozy-node-setups/work/12/5.1/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.1/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/5.1/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.1/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4316   mean value B: 0.4316
--- without the file, against without the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/5.1/without_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/5.1/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.1 ===
```

#### Whether the file read can change from frame to frame (list entry 13) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 5.1.2 WORKDIR: /tmp/smoozy-node-setups/work/13/5.1
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
BLENDER: 5.1.2 WORKDIR: /tmp/smoozy-node-setups/work/05/5.1
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/tmp/smoozy-node-setups/work/05/5.1/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/5.1/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/tmp/smoozy-node-setups/work/05/5.1/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/5.1/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/5.1/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/5.1/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/05/5.1/tex.png', '/tmp/smoozy-node-setups/work/05/5.1/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0xaa55cce08> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0xaa35b7908> -> {'//model.obj'}
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
########## BLENDER 5.1 — packed.blend reopened with model.obj and tex.png moved away ##########
00:00.317  blend            | Read blend: "/tmp/smoozy-node-setups/work/05/5.1/packed.blend"
00:00.322  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/05/5.1/model.obj'.
00:00.322  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/05/5.1/model.obj'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
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
BLENDER: 5.1.2 WORKDIR: /tmp/smoozy-node-setups/work/07/5.1
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /tmp/smoozy-node-setups/work/07/5.1/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=2139603544 bake_target=INHERIT directory=''
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
   file on disk: <workdir>/blendcache_bake/Target_GN/2139603544/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/2139603544/meta/00001_00000.json
   file on disk: <workdir>/mybakes/2139603544/meta/00001_00000.json
DONE
```

#### Whether these paths are matched letter for letter (list entry 11) — Mac: **this one did not agree with Linux**

With `model.obj` and `tex.png` on disk in lower case, exactly as on Linux. The whole run, uncut:

```
BLENDER: 5.1.2 WORKDIR: /tmp/smoozy-node-setups/work/11/5.1
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 3
PATH='//Model.Obj'    -> vertices: 3
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: True size: (4, 4) pixels: 64
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /tmp/smoozy-node-setups/work/11/5.1/MODEL.OBJ
DONE
```

On Linux the same run gave `//MODEL.OBJ -> vertices: 0`, `//Model.Obj -> vertices: 0`, an image that did not load for `//TEX.PNG`, and a `bpy.path.resolve_ncase` that turned the upper-case name back into the lower-case file that exists. On this Mac all three upper-case forms found the file, the image loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back. The two results are recorded side by side and are not merged: **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. Windows is untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The working folder for this run was under `/tmp`, which on this Mac sits on the volume named `Macintosh HD - Data`, an APFS volume — the machine's own ordinary file system, not one made for the test.


#### What the Path row on an Import node offers in a window (list entry 14) — Mac only

This is the one the Linux machine cannot reach at all: it has no display server, so nothing drawn in a window can be looked at there. Blender was started **with** its window, opening the `scene.blend` that list entry 3 built, and driven by a script that switched an area to the geometry node editor, showed the node group, framed the nodes with Blender's own View All, took Blender's own screenshot and quit. No mouse was moved and nothing was clicked.

The first form of the script took the screenshot without framing anything, and the second added View All. Both came back showing only the Image Texture node, because every node in the group had been made at the same spot and they were drawn on top of one another. Both are recorded as they came back; neither settles the question. The third form spread the nodes apart first, the way a person would drag them, and that is the one that answers it. Blender printed:

```
00:00.546  blend            | Read blend: "/tmp/smoozy-node-setups/work/03/5.1/scene.blend"
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
Warning: 2 × Draw window and swap: 90.3389 ms, average: 45.16947269 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 15.6150 ms, average: 7.80749321 ms
screenshot written to /tmp/smoozy-node-setups/work/14/5.1/spread-all.png
Warning: 2 × Draw window and swap: 10.7119 ms, average: 5.35595417 ms
view_selected returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 16.3131 ms, average: 8.15653801 ms
screenshot written to /tmp/smoozy-node-setups/work/14/5.1/import-node.png
Path socket on the Import OBJ node holds: '//model.obj'
Path socket type: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
00:11.885  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

The screenshot shows the Import OBJ node drawn with a title bar reading **Import OBJ**, an **Instances** output on its right edge, and one row below it: a text field holding **`//model.obj`**, a **folder button** at that field's right-hand end, and a **socket dot on the node's left edge level with that row**. All four versions drew the same three things. The only difference between them is the shape of the socket dots — round in 4.5, rectangular in 5.0, 5.1 and 5.2 — which is Blender's own drawing style, not anything about paths.

The screenshots are kept at `/tmp/smoozy-node-setups/work/14/5.1/` on that machine. This settles the drawing of the node for **macOS only**. There is no Linux machine with a screen in this project, and no Windows machine ran it, so what the row looks like on either of those is untested.


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
STEP after touching the modifier                    plain: 4    tagged: -
STEP after stepping the frame                       plain: 4    tagged: -
STEP after setting the same path again              plain: 4    tagged: -
STEP after putting the 3-vertex file back           plain: 4    tagged: -

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
route: toggling mod.show_viewport twice -> 4
route: stepping the frame -> 4
route: setting the same path again -> 4
route: setting a different path and back -> 4
route: ob.data.update_tag() -> 4
route: ob.update_tag() -> 4
route: taking the group off the modifier and putting it back -> 4
control: 3-vertex file back, no tag -> 4
control: 3-vertex file back, ob.update_tag() -> 4
```

**Entry 3's own arrangement, run ten times over.**

```
5.1 plain, ten repeats, count at the reading after the modifier touch: 3 3 3 3 3 3 3 3 3 3
```

**The plain arrangement, run three times over.** This is where the two arrangements stopped being the question. The very same script, on the very same machine, minutes apart:

```
--- 5.1 plain, repeat 1 ---
STEP with the 3-vertex file                         plain: 3    tagged: -
STEP straight after overwriting with the 4-vertex file plain: 3    tagged: -
STEP after touching the modifier                    plain: 3    tagged: -
STEP after stepping the frame                       plain: 3    tagged: -
STEP after setting the same path again              plain: 3    tagged: -
STEP after putting the 3-vertex file back           plain: 3    tagged: -
--- 5.1 plain, repeat 2 ---
STEP with the 3-vertex file                         plain: 3    tagged: -
STEP straight after overwriting with the 4-vertex file plain: 3    tagged: -
STEP after touching the modifier                    plain: 3    tagged: -
STEP after stepping the frame                       plain: 3    tagged: -
STEP after setting the same path again              plain: 3    tagged: -
STEP after putting the 3-vertex file back           plain: 3    tagged: -
--- 5.1 plain, repeat 3 ---
STEP with the 3-vertex file                         plain: 3    tagged: -
STEP straight after overwriting with the 4-vertex file plain: 3    tagged: -
STEP after touching the modifier                    plain: 4    tagged: -
STEP after stepping the frame                       plain: 4    tagged: -
STEP after setting the same path again              plain: 4    tagged: -
STEP after putting the 3-vertex file back           plain: 4    tagged: -
```

Two runs gave three, one gave four. Nothing in the script changed between them. So whatever decides this is not the arrangement — it is something that varies from run to run.

**The stamp on the file, set by hand instead of left to the clock.** The next run puts the three-vertex file's modification time at a quarter of a second past a known whole second, then moves it by a chosen amount at the overwrite, so that whether the whole second changes is decided rather than left to chance. Each line prints the two stamps, whether the whole second changed, and four readings: the one before the overwrite, then three after it, with the modifier touched between them.

```
BLENDER 5.1.2      offset  +0.000  tag False  stamp 1786081754.250 -> 1786081754.250  whole second 1786081754 -> 1786081754  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.000  tag True   stamp 1786081755.250 -> 1786081755.250  whole second 1786081755 -> 1786081755  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.500  tag False  stamp 1786081755.250 -> 1786081755.750  whole second 1786081755 -> 1786081755  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.500  tag True   stamp 1786081756.250 -> 1786081756.750  whole second 1786081756 -> 1786081756  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.700  tag False  stamp 1786081757.250 -> 1786081757.950  whole second 1786081757 -> 1786081757  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.700  tag True   stamp 1786081757.250 -> 1786081757.950  whole second 1786081757 -> 1786081757  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.740  tag False  stamp 1786081758.250 -> 1786081758.990  whole second 1786081758 -> 1786081758  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.740  tag True   stamp 1786081759.250 -> 1786081759.990  whole second 1786081759 -> 1786081759  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  +0.760  tag False  stamp 1786081759.250 -> 1786081760.010  whole second 1786081759 -> 1786081760  changed: True   readings: 3 3 4 4
BLENDER 5.1.2      offset  +0.760  tag True   stamp 1786081760.250 -> 1786081761.010  whole second 1786081760 -> 1786081761  changed: True   readings: 3 4 4 4
BLENDER 5.1.2      offset  +0.900  tag False  stamp 1786081761.250 -> 1786081762.150  whole second 1786081761 -> 1786081762  changed: True   readings: 3 3 4 4
BLENDER 5.1.2      offset  +0.900  tag True   stamp 1786081761.250 -> 1786081762.150  whole second 1786081761 -> 1786081762  changed: True   readings: 3 4 4 4
BLENDER 5.1.2      offset  +1.000  tag False  stamp 1786081762.250 -> 1786081763.250  whole second 1786081762 -> 1786081763  changed: True   readings: 3 3 4 4
BLENDER 5.1.2      offset  +1.000  tag True   stamp 1786081763.250 -> 1786081764.250  whole second 1786081763 -> 1786081764  changed: True   readings: 3 4 4 4
BLENDER 5.1.2      offset  +2.000  tag False  stamp 1786081763.250 -> 1786081765.250  whole second 1786081763 -> 1786081765  changed: True   readings: 3 3 4 4
BLENDER 5.1.2      offset  +2.000  tag True   stamp 1786081764.250 -> 1786081766.250  whole second 1786081764 -> 1786081766  changed: True   readings: 3 4 4 4
BLENDER 5.1.2      offset  -0.200  tag False  stamp 1786081765.250 -> 1786081765.050  whole second 1786081765 -> 1786081765  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  -0.200  tag True   stamp 1786081765.250 -> 1786081765.050  whole second 1786081765 -> 1786081765  changed: False  readings: 3 3 3 3
BLENDER 5.1.2      offset  -0.500  tag False  stamp 1786081766.250 -> 1786081765.750  whole second 1786081766 -> 1786081765  changed: True   readings: 3 3 4 4
BLENDER 5.1.2      offset  -0.500  tag True   stamp 1786081767.250 -> 1786081766.750  whole second 1786081767 -> 1786081766  changed: True   readings: 3 4 4 4
```

**Whether a reading on its own is enough.** The last run overwrites the file, always moving the stamp a whole second, and then reads five times over — once with the modifier touched between readings and once without.

```
BLENDER 5.1.2      touch True  tag False  whole second 1786081834 -> 1786081835  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.1.2      touch True  tag True   whole second 1786081834 -> 1786081835  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.1.2      touch False tag False  whole second 1786081835 -> 1786081836  first reading 3 then [3, 3, 3, 3, 3]
BLENDER 5.1.2      touch False tag True   whole second 1786081836 -> 1786081837  first reading 3 then [4, 4, 4, 4, 4]
```

**What the runs show.** Three things, and all three are the same in 4.5, 5.0, 5.1 and 5.2.

1. **Asking for the evaluated geometry again does not by itself make the setup run again.** With nothing touched and nothing tagged, five readings in a row give the old count, however many times you ask. Something has to mark the setup for a fresh run: touching the modifier does it, and so does `ob.update_tag()`.
2. **Even then, the file is only read again if its modification time has changed by a whole second.** A change inside the same whole second is not noticed at all — the setup runs again and comes back with the old contents. A change that crosses a whole second is noticed every time. It is noticed in either direction: a stamp moved a second *backwards* counts as a change just as much as one moved forwards. Across eighty runs — four versions, ten stamp offsets, both arrangements — there was not one exception in either direction.
3. **Marking the object for update is one of the things that make the setup run again, and it is simply the earliest one.** With the stamp changed, the tagged arrangement shows the new contents at the very first reading after the overwrite; the plain arrangement shows them at the first reading that follows a nudge, which in entry 3's sequence is the reading after the modifier is touched. One reading of difference, and no more.

**So this is exactly what tells the two arrangements apart.** Not whether the file is read again — that is decided by the stamp on the file, and by nothing either arrangement does. What the tagging changes is *when* the new contents first appear: at the first reading rather than one reading later. That is the whole of the difference between them.

**And this is why the two recorded results disagreed.** Entry 3's script overwrites `model.obj` a few hundredths of a second after writing it, so in almost every run the whole second is unchanged and nothing is read again — three vertices at every step, which is what entry 3 records, and what the ten repeats above gave — and the same ten in each of the four versions, forty runs without one exception. Now and then the overwrite lands on the far side of a second boundary, and a four appears. With the object tagged the four appears at the first reading; without the tag it appears one reading later. Both recorded results are real, neither is wrong, and neither had to be chosen between: they differ by where the run happened to fall inside a second.

**What this does not settle.** Linux only. The machine was given up before this could be run on the Creator's Mac, and no Windows machine has run it, so whether either of those keeps and compares the stamp the same way is untested. Nothing here says *why* the whole second is the unit — that would be a claim about Blender's insides, and these runs only watch it from outside.


## 2. When a `.blend` file links a node setup from another `.blend` file, what does the linking file record about the files that setup uses, and how are they reached?

**LABEL: mostly documented; the central claim about how relative paths are reached is documented for the general case and observed in Cycles' files; the joining of that to geometry node setups specifically is inferred.**

**The question is ambiguous and I say so plainly.** "Links a node setup" can mean (a) the node group data-block itself is linked from the other `.blend`, or (b) an object carrying a geometry nodes modifier is linked, bringing its node group with it. I answer reading (a), because the sheet says "links a node setup", and a node setup is a node group. Under reading (b) everything below still applies to the node group, with the addition that the modifier and its settings would then also be linked rather than local — which reverses the point in section 2d.

### 2a. What the linking file records: a library, and names

Linking records a reference, not a copy:

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."

and

> "Linked data-blocks are indicated with a chain icon in the Outliner. They’re also listed in the Outliner’s Blender File Display Mode, along with the path of the blend-file they originate from."

and the option that governs the form of that path:

> "Relative Path
>
> Reference the external blend-file using a relative path rather than an absolute one."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

The thing recorded is a Library data-block:

> "Library(ID) […] External .blend file from which data is linked
>
> filepath
>
> Path to the library .blend file (default “”, never None, blend relative // prefix supported)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.Library.html — read 2026-07-30.

and each linked data-block knows which library it came from, and whether it was asked for directly or dragged in as a dependency:

> "library
>
> Library file the data-block is linked from (readonly)"

> "is_library_indirect
>
> Is this ID block linked indirectly (default False, readonly)"

> "name_full
>
> Unique data-block ID name, including library one if any (default “”, readonly, never None)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.ID.html — read 2026-07-30.

There is one place in the geometry nodes machinery where the manual and API show the exact shape of such a record. Baked geometry keeps its own references to data-blocks, and each is stored as a name plus a library name plus a type:

> "id_name — Name that is mapped to the referenced data-block (default “”, readonly, never None)"
>
> "lib_name — Used when the data block is not local to the current .blend file but is linked from some library (default “”, readonly, never None)"
>
> "id_type — (default 'ACTION', readonly)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.NodesModifierDataBlock.html — read 2026-07-30. The manual's side of this is: "Baked geometries that reference other data-blocks such as materials are listed here. This panel allows changing these references after the data has been baked. […] Note — Currently only material data-blocks are supported." (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/baking.html — read 2026-07-30.)

So the pattern Blender uses for a cross-file reference is: a library, identified by name, carrying the path to the `.blend`; and within it, a data-block identified by name.

### 2b. What it records about the files the setup uses: as far as the sources go, nothing extra

The linking file records the library. It does not record the setup's own files separately. The evidence is indirect but consistent.

Data-blocks the setup depends on are themselves linked, and are marked as having arrived indirectly (`is_library_indirect`, quoted above). The append side of the manual makes the existence of that maintained chain explicit, by offering an option to break it:

> "Localize All
>
> Also copy all indirectly linked data, instead of maintaining the links."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

And the preferences page for asset import spells out what "dependencies" covers:

> "“Dependencies” in this case means everything the asset refers to. For an object, this can be its mesh and materials, but also other objects used by modifiers, constraints, or drivers."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html — read 2026-07-30.

For a linked geometry node group this means: the Image, the VectorFont, the Material, the Object and the Collection that the setup names come across as indirectly linked data-blocks belonging to the library, carrying their own `filepath` values as stored in the library file. The linking file holds no separate copy of those paths. Reading that off the two quotes above is a step: **inferred**.

The import nodes are different in kind and worth stating separately. Their paths are socket values inside the node tree (established in question 1: the API pages for those node types carry no path property, and `NodeSocketStringFilePath` holds the value). A socket value is not a data-block, so there is nothing for the linking file to record about it at all — the string simply travels inside the linked node tree, exactly as stored in the library. **Inferred**, from the socket-not-property finding plus "Link creates a reference to data in a source file".

### 2c. How they are reached: a relative path belonging to linked data resolves against the library, not against the linking file

This is the load-bearing part of the answer, and it has three pieces of evidence.

**First, what a relative path means in general:**

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read 2026-07-30.

Note the wording is "the referencing blend-file", not "the open blend-file".

**Second, the API function that turns such a path into a real one takes a library:**

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the “//” prefix.
>
> […]
>
> start (str | bytes | None) – Relative to this path, when not set the current filename is used.
>
> library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.path.html — read 2026-07-30.

"its path replaces start" is as direct a statement as the sources give: for a path belonging to a linked data-block, the base is the library `.blend`, not the open one.

**Third, the manual says the same thing from a different direction,** in the list of variables usable in path templates:

> "blend_name_lib:
>
> Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from.
>
> blend_dir_lib:
>
> Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.

That page is about path templates, and it limits itself: "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node." So it does not itself settle general path resolution. What it does show is that Blender distinguishes the open file from the library a data-block came from when filling in these variables for a path property on linked data. *(Reworded by the citation check, 2026-08-07: the two phrases here were written in quotation marks but are this document's own words, not the source's, and in a paragraph otherwise full of real quotations that reads as a citation. Narrowed to what the quoted variable descriptions carry.)*

**Fourth — and this one is observed, not read.** Cycles' own files show the mechanism in use. In `cycles_5.1/osl.py`, the function that turns a shader script node's stored path into a real one is called with the node tree's library:

```
        ok, oso_path, oso_file_remove = update_external_script(report, node.filepath, node.id_data.library)
```

Source: `cycles_5.1/osl.py`, inside `update_script_node`, the branch for `node.mode == 'EXTERNAL'` (line 280 of that file). And the function it calls resolves the path with exactly the `library` argument documented above:

```
def update_external_script(report, filepath, library):
    """compile and update OSL script"""
    import os
    import shutil

    oso_file_remove = False

    script_path = bpy.path.abspath(filepath, library=library)
```

Source: `cycles_5.1/osl.py`, function `update_external_script` (lines 192–199 of that file). The same pattern appears twice more in that file: `osl_path = bpy.path.abspath(script.filepath, library=script.library)` in `update_internal_script`, and `update_external_script(report, cam.custom_filepath, cam.library)` for a camera's custom shader.

Label: **observed**. What the file *says* is that Cycles passes the owning data-block's library when resolving a node's stored file path. Read 2026-07-30.

The step from there to geometry nodes is a step, and I mark it: this is a **shader** node in Cycles, not a geometry node, and Cycles' own code is not evidence about how Blender's geometry nodes resolve their import node paths. What it does establish is that "a file path stored on a node inside a linked node tree is resolved against that tree's library" is a real, used pattern in Blender 5.1, not a guess. Whether geometry nodes' import nodes do the same is **not in the sources** — see the hand test at the end.

### 2d. What the linking file does record locally

Two things sit on the linking side even when the node group is linked, both from question 1's material.

The modifier's input values are per-modifier:

> "Inputs
>
> A list of the node group’s inputs which can have unique values even if the group is shared among multiple modifiers."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/modeling/modifiers/geometry_nodes.html — read 2026-07-30.

So if the linked node group exposes a File Path input on its interface ("File Path: The string is interpreted as a path to a file", node groups page, quoted in question 1), the value actually used is set on the modifier in the linking file, not in the library. That the modifier therefore lives in the linking file, and hence that this particular path is recorded locally, is **inferred** from "unique values even if the group is shared among multiple modifiers".

Likewise the bake directory: `NodesModifier.bake_directory` and `NodesModifierBake.directory` are properties of the modifier, not the node tree (both quoted in question 1). The manual places the setting on the modifier too: "Bake Path — Location on disk where the baked data for Simulation Zones and Bake Nodes are stored." (https://docs.blender.org/manual/en/5.1/modeling/modifiers/geometry_nodes.html — read 2026-07-30.) So a linked node group's Bake node writes to a directory named by the linking file.

### 2e. What happens when the library cannot be found

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

> "is_missing
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.ID.html — read 2026-07-30.

The same page also documents Relocate Library ("It allows to replace it by a different file") and Relocate Linked ID.

One warning on that page bears on node setups in both directions:

> "Circular Dependencies
>
> In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links."

Source: same page, read 2026-07-30.

### 2f. Two facilities that show how Blender itself enumerates all this

The Python API has a pair of functions for walking every file path used by every data-block, and their options make the linked case explicit:

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them.
>
> […]
>
> include_libraries (bool) – Include library file paths of linked data. False by default."

> "file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})
>
> Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data. […] The function should return either None or a str. In the latter case, the visited file path will be replaced with the returned string."

Source: Blender 5.1 Python API, https://docs.blender.org/api/5.1/bpy.types.BlendData.html — read 2026-07-30.

The flags they accept name the linked case directly:

> "SKIP_LINKED: Skip Linked. Skip paths of linked IDs."
>
> "SKIP_PACKED: Skip Packed. Skip paths when their matching data is packed."
>
> "RESOLVE_TOKEN: Resolve Token. Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles."
>
> "SKIP_WEAK_REFERENCES: Skip Weak References. Skip weak reference paths. Those paths are typically ‘nice to have’ extra information, but are not used as actual source of data by the current .blend file."
>
> "SKIP_MULTIFILE: Skip Multi-file. Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed."
>
> "RELOAD_EDITED: Reload Edited. Reload data when the path is edited."

Source: Blender 5.1 Python API, "File Path Foreach Flag Items" enum, https://docs.blender.org/api/5.1/bpy_types_enum_items/file_path_foreach_flag_items.html — read 2026-07-30.

Two readings follow, both worth recording. `SKIP_LINKED` exists and is **not** in the default flag set for `file_path_foreach`, so by default a walk of the open file's paths includes the paths belonging to linked data-blocks — that is, the files a linked node setup uses are visible from the linking file. And `include_libraries` defaults to False in `file_path_map`, so the library `.blend` paths themselves are a separate category that must be asked for. Both readings are **inferred** from the defaults as documented; the pages do not say it in words.

### 2g. Packing, on both levels

Two different pack operations apply here:

> "Pack Linked Libraries
>
> Mark all linked library files in the current blend-file as packed. Actual packing will happen on the next save of the blend-file.
>
> Unpack Linked Libraries
>
> Unpack all used linked library files from this blend-file."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html — read 2026-07-30.

The API side shows the shape this takes:

> "is_linked_packed — This data-block is linked and packed into the .blend file (default False, readonly)" (ID page)

> "archive_libraries — Archive libraries of packed IDs, generated (and owned) by this source library (default None, readonly)"
>
> "is_archive — This library is an ‘archive’ storage for packed linked IDs originally linked from its ‘archive parent’ library. (default False, readonly)"
>
> "packed_file — (readonly)" (Library page)

Sources: https://docs.blender.org/api/5.1/bpy.types.ID.html and https://docs.blender.org/api/5.1/bpy.types.Library.html — read 2026-07-30.

The asset preferences describe the same thing as a one-step import method:

> "Pack
>
> Imports the asset as linked data and immediately packs it into the current blend-file. This ensures that the asset remains available even if the original library data is modified or becomes unavailable.
>
> Useful for maintaining self-contained files that do not rely on external asset library paths."

Source: Blender 5.1 manual, https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html — read 2026-07-30.

And the general packing page carries the limit worth knowing: "Warning — Not all external files can be packed. Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file." (packed_data page, read 2026-07-30.) That warning names Sequence Editor videos and Movie Clips; it says nothing about geometry node import files.

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer: which folder a `//` inside a linked setup is measured from, and what the linking file records, list entry 7; what packing a library carries and what it leaves behind, list entry 8; what is recorded when a library links onward, list entry 9; whether the library path is stored relative or absolute, list entry 10. Read this list as "the documentation does not say", not as "nobody knows".)*

- **The central gap.** No source states, in words, what an import node's Path socket is resolved against when the node tree is linked. The general rule (`bpy.path.abspath`'s `library` argument, and `blend_dir_lib`) and Cycles' use of it make the library the strong candidate, but a socket value is not a "path property on a data-block" in the plain sense, and I did not find any source that closes that gap. This is the one thing I would most want settled by hand.
- Whether a linked node group's Image, Font and Material dependencies are linked automatically at all, stated for geometry nodes specifically. The "Localize All" option and the asset-dependency wording make it clear the mechanism exists generally; neither mentions node groups.
- What happens on save: whether the linking file writes anything at all about the library's own external files, or only the library reference. The sources describe the runtime picture (`is_library_indirect`, `file_path_map`), not the file format.
- Whether relative-path bake directories on a modifier that uses a *linked* node group resolve against the linking file. The modifier is local, so the linking file is the obvious base, but the sources do not say it.
- Library overrides. `library_overrides.txt` was read and covers overriding *properties* of linked data; it says nothing about file paths, and I ruled it out.
- Anything about how this behaves when the same library is reached by two different relative paths from two files.

### Where I looked

- Read in full: `files/linked_libraries/index.txt`, `files/linked_libraries/link_append.txt`, `files/blend/packed_data.txt`, `files/file_paths.txt`, the "Relative Paths" section of `files/blend/open_save.txt`, `editors/preferences/file_paths.txt` (Import Method and Relative Path), the External Data and Clean Up sections of `interface/window_system/topbar.txt`, `interface/controls/nodes/groups.txt` (socket subtypes), `modeling/modifiers/geometry_nodes.txt`, `modeling/geometry_nodes/baking.txt`.
- Skimmed `files/linked_libraries/library_overrides.txt` and `editors/outliner/interface.txt` (Blender File display mode). Ruled both out as not bearing on file paths.
- Read in the API: `bpy.types.Library`, `bpy.types.BlendDataLibraries` (`load` and `write`, including the `path_remap` options "RELATIVE Remap paths that are already relative to the new location / RELATIVE_ALL Remap all paths to be relative to the new location / ABSOLUTE Make all paths absolute on writing", and the note "Indirectly referenced data-blocks will be expanded and written too"), `bpy.types.ID`, `bpy.types.BlendData`, `bpy.path`, `bpy.types.NodesModifier`, `bpy.types.NodesModifierBake`, `bpy.types.NodesModifierDataBlock`, `bpy.ops.file` (the five path operators), and the File Path Foreach Flag Items enum.
- Searched the whole 5.1 manual for: `relative path`, `Relative Paths`, `relative to the`, `indirect`, `linked library`, `library file`, `from the library`, `library blend`. Searched the whole 5.1 API for `path_remap`, `make_paths_relative`, `make_paths_absolute`, `report_missing_files`, `find_missing_files`, and for `library` within `bpy.path` and `bpy.types.BlendData`.
- Searched all 5.1 release notes for `librar`; the only hits were asset-library UI changes and bug-fix lines. One bug-fix line is adjacent but is about a fixed defect, not about behaviour I can rely on, so I record it and do not build on it: "Bpy.data.libraries.write with path_remap='RELATIVE_ALL' does not recalculate existing relative paths" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30).
- Grepped `cycles_5.1` for `filepath`, `FILE_PATH`, `library`, `node_group`; read `osl.py` around `update_external_script`, `update_internal_script` and `update_script_node`.

### Outside the interface

Partly documented, partly not.

- Linking and appending are documented as menu actions (`File ‣ Link…`, `File ‣ Append…`) but the same work has a documented non-interface form: `bpy.data.libraries.load(filepath, *, link=False, pack=False, relative=False, …)`, with `link` described as "When False reference to the original file is lost", `relative` as "When True the path is stored relative to the open blend file", and `pack` as "If True, and link is also True, pack linked data-blocks into the current blend-file". (https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html — read 2026-07-30.) So the recording side of this answer is reachable without a window.
- `bpy.path.abspath`, `bpy.data.file_path_map` and `bpy.data.file_path_foreach` are plain functions with no stated interface requirement.
- The Outliner listing, Relocate Library, Relocate Linked ID and Find Missing Files are all documented only as things done in an editor. Whether equivalents exist outside is not stated for the first three; `bpy.ops.file.find_missing_files` and `bpy.ops.file.report_missing_files` do exist as operators, but the API does not say whether they run without a window.
- The sources do not say whether a linked library is found and read the same way when Blender is started without its window. Nothing suggests otherwise; nothing states it.

**Brought up to date by hand test, 2026-08-06 — observed.** All of it holds with no window. Linking, packing the library, reading what the linking file records, resolving a `//` path belonging to linked data, and creating placeholders for a library that cannot be found were every one of them done and read from a script in a Blender started with `-b`, on a machine with no display server at all. With the library gone, Blender printed a warning naming it, left a placeholder in its place and exited with code 0. See "Observed by hand test" below, list entries 7, 8, 9 and 10. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers, so this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so two things could be looked at there that no terminal run can settle. The Path field on a linked group's Import node is drawn **greyed out** in the window, in all four versions — while a script writing to that same socket is accepted, which the Linux runs showed. And the modifier for a linked group holding no Bake node shows no Bake panel at all, only a collapsed section headed Manage; that section could not be opened without a click, so what it holds is untested. What a linked group that *does* hold a Bake node gives the modifier was answered instead with no window at all. See "Observed by hand test on the Creator's Mac" below, list entries 7, 8, 9, 10, 15, 16 and 17.

### Operating system

The sources do not say. None of the pages quoted here carries an operating-system qualification, and I found nothing stating that library paths, relative paths, or packing behave differently on any particular system.

**Brought up to date by hand test, 2026-08-06 — observed.** Everything below was run on Linux only, where a path is matched letter for letter — see answer 1's observed section, list entry 11. Nothing has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed.** The letter-case test was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — as well, and came out the other way: there the upper-case forms found the file. See answer 1's Mac observed section, list entry 11. So a path is matched letter for letter on Linux and is not on macOS. Everything else here was run on both machines and agreed on both. No Windows machine has run it, so Windows is untested.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

Same machine, same date, same Blender and the same settings folder as the observed section under answer 1: **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all**, `/opt/blender/5.1/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, started with `BLENDER_USER_RESOURCES=/root/node-setups/config/5.1`. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`. **Every one of these was run again on the Creator's Mac on 2026-08-07, and all of them agreed; that block follows this one.** The two machines' results are kept apart and never merged.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7)

Two folders were built. `lib/` held `lib.blend`, a `model.obj` with **three** vertices and a `tex.png`; inside `lib.blend`, a node group `LibSetup` with an Import OBJ node whose Path was `//model.obj` and an Image Texture node whose image was `//tex.png`. `shot/` held `use.blend`, which linked `LibSetup` and put it on an object. Then `use.blend` was opened four times: with no `model.obj` beside it; with a **different**, four-vertex `model.obj` beside it; with `lib/model.obj` moved away and the four-vertex copy still beside `use.blend`; and with the whole `lib` folder moved away. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.383  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/use.blend"
00:00.389  reports          | Read library: '/root/node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.427  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.427  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7cd0fb754f20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7cd0fb754f20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/tex.png', '/root/node-setups/work/06/5.1/lib/model.obj', '/root/node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7cd0fb6c4120> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7cd0fb6f5320> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7cd0fb754f20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7cd0fb6c4120> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7cd0fb6f5320> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7cd0fb754f20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.376  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/use.blend"
00:00.382  reports          | Read library: '/root/node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.419  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.419  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7cc171f84f20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7cc171f84f20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/tex.png', '/root/node-setups/work/06/5.1/lib/model.obj', '/root/node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7cc171ef4120> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7cc171f25320> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7cc171f84f20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7cc171ef4120> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7cc171f25320> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7cc171f84f20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.370  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/use.blend"
00:00.375  reports          | Read library: '/root/node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.384  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/model.obj'.
00:00.384  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/model.obj'
00:00.411  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.411  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7c476b594f20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7c476b594f20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/tex.png', '/root/node-setups/work/06/5.1/lib/model.obj', '/root/node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7c476b504120> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7c476b535320> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7c476b594f20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7c476b504120> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7c476b535320> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x7c476b594f20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.1/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/root/node-setups/work/06/5.1/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.371  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/use.blend"
00:00.377  reports          | Read library: '/root/node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.377  reports          | WARNING Unable to open '/root/node-setups/work/06/5.1/lib/lib.blend': No such file or directory
00:00.377  reports          | Cannot find lib '/root/node-setups/work/06/5.1/lib/lib.blend'
00:00.378  reports          | LIB: NodeTree: 'LibSetup' missing from '/root/node-setups/work/06/5.1/lib/lib.blend', parent '<direct>'
00:00.379  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7a4725184f20> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x7a4725184f20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x7a472acd4408> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7a4725184f20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.1/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 5.1 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8)

`use.blend` was opened, `bpy.ops.file.pack_libraries()` run, and the result saved as `packed_use.blend`; that file was then opened three times, with everything in place, with `lib.blend` alone moved away, and with the whole `lib` folder moved away. In full:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.351  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/use.blend"
00:00.357  reports          | Read library: '/root/node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0x7b9aad96cf20>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x7b9aad96cf20> is_linked_packed: False
Info: Saved as "packed_use.blend"
saved: /root/node-setups/work/06/5.1/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.353  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/packed_use.blend"
00:00.358  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.393  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.393  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x77ce45386420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x77ce45386420> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.1/lib/tex.png', '/root/node-setups/work/06/5.1/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x77ce452f5620> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x77ce45339920> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x77ce452f5620> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x77ce45339920> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.387  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/packed_use.blend"
00:00.393  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.430  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.430  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x79fb99d86420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x79fb99d86420> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.1/lib/tex.png', '/root/node-setups/work/06/5.1/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x79fb99cf5620> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x79fb99d39920> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x79fb99cf5620> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x79fb99d39920> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.378  blend            | Read blend: "/root/node-setups/work/06/5.1/shot/packed_use.blend"
00:00.384  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.393  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/model.obj'.
00:00.393  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/model.obj'
00:00.397  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.397  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/06/5.1/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x70fec6b66420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x70fec6b66420> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/5.1/lib/tex.png', '/root/node-setups/work/06/5.1/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x70fec6abd620> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x70fec6b01920> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x70fec6abd620> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x70fec6b01920> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/5.1/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/root/node-setups/work/06/5.1/lib/lib.blend')
Warning: Path '/root/node-setups/work/06/5.1/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/root/node-setups/work/06/5.1/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 5.1 ===
```

#### What a linking file records when its library links onward (list entry 9)

Three folders: `deep/deep.blend` with a node group `DeepSetup` reading `//model.obj` and a **three**-vertex `model.obj` beside it; `mid/mid.blend` linking `DeepSetup`, wrapping it in a local group `MidSetup`, with a **four**-vertex decoy `model.obj` beside it; `top/top.blend` linking `MidSetup`, with a **five**-vertex decoy `model.obj` beside it. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
00:00.374  blend            | Read blend: "/root/node-setups/work/09/5.1/top/top.blend"
00:00.380  reports          | Read library: '/root/node-setups/work/09/5.1/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.385  reports          | Read library: '/root/node-setups/work/09/5.1/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/5.1/mid/mid.blend'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/09/5.1/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/5.1/deep/model.obj', '/root/node-setups/work/09/5.1/deep/deep.blend', '/root/node-setups/work/09/5.1/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x7cf408aa1f20> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x7cf408b31320> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x7cf408af9d08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x7cf408b84f20> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.374  blend            | Read blend: "/root/node-setups/work/09/5.1/top/top.blend"
00:00.380  reports          | Read library: '/root/node-setups/work/09/5.1/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.385  reports          | Read library: '/root/node-setups/work/09/5.1/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/5.1/mid/mid.blend'
00:00.395  io.obj           | ERROR Cannot read from OBJ file:'/root/node-setups/work/09/5.1/deep/model.obj'.
00:00.395  reports          | ERROR OBJ Import: Cannot open file '/root/node-setups/work/09/5.1/deep/model.obj'
BLENDER: 5.1.2 OPEN FILE: /root/node-setups/work/09/5.1/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/5.1/deep/model.obj', '/root/node-setups/work/09/5.1/deep/deep.blend', '/root/node-setups/work/09/5.1/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x7300f0ca1f20> -> {'//model.obj', '//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x7300f0d31320> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x7300f0cf9d08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x7300f0d84f20> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/root/node-setups/work/09/5.1/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/root/node-setups/work/09/5.1/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 5.1 ===
```

#### Whether the library path is recorded relative or absolute, and what Blender ships (list entry 10)

In full:

```
BLENDER: 5.1.2 WORKDIR: /root/node-setups/work/10/5.1
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/root/node-setups/work/10/5.1/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/root/node-setups/work/10/5.1/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: no such attribute
--- the node assets Blender ships with ---
   assets folder: /opt/blender/5.1/5.1/datafiles/assets exists: True
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/nodes/shading_nodes_essentials.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /opt/blender/5.1/5.1/datafiles/assets/nodes/procedural_hair_node_assets.blend
DONE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

Same machine, same date, same Blenders and the same settings folders as the observed Mac section under answer 1: **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**, `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)`, started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/5.1`. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing here measures time or memory. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`.

**Every one of these agreed with Linux**, and the two machines' results are kept separate rather than merged. Two further tests were run that the Linux half did not reach: the window tests, list entries 14 and 15, which need a screen; and two the Mac half showed the list had missed, list entries 16 and 17.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7) — Mac

Agreed with Linux: the `//` was measured from the **library** file, not from the linking file. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.328  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.331  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.355  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.355  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x8cf0b6420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x8cf0b6420> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x8d3b94020> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8cf118320> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x8cf0b6420> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x8d3b94020> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8cf118320> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x8cf0b6420> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.490  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.493  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.510  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.510  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xc30851e20> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xc30851e20> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xc308a0020> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc2fa21820> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xc30851e20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xc308a0020> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc2fa21820> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xc30851e20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.325  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.329  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.333  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj'.
00:00.333  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj'
00:00.347  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.347  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xb26dca420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xb26dca420> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xb2c06aa20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb26e3c320> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xb26dca420> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xb2c06aa20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb26e3c320> -> {'//model.obj', '//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0xb26dca420> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.444  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.448  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.448  reports          | WARNING Unable to open '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend': No such file or directory
00:00.448  reports          | Cannot find lib '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend'
00:00.449  reports          | LIB: NodeTree: 'LibSetup' missing from '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', parent '<direct>'
00:00.449  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xb11bcc020> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
   local=True : ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0xb11bcc020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0xb14a1fc08> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0xb11bcc020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 5.1 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8) — Mac

Agreed with Linux. The whole run, uncut:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.572  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.579  reports          | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0xc91d0e420>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xc91d0e420> is_linked_packed: False
Info: Saved as "packed_use.blend"
saved: /tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.539  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend"
00:00.548  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.575  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.575  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xc02833620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xc02833620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xc07d84020> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc0375db20> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xc07d84020> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc0375db20> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.469  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend"
00:00.473  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.497  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.497  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xaa8f9a420> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xaa8f9a420> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xaadaaf120> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xaa8804320> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xaadaaf120> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xaa8804320> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.589  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend"
00:00.594  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.601  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj'.
00:00.601  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj'
00:00.602  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'.
00:00.602  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/5.1/lib/written-from-linking-file.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/06/5.1/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 1, 30)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x8adfd5820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x8adfd5820> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/5.1/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png', '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x8afc36a20> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8adf9b620> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x8afc36a20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x8adf9b620> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/5.1/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend')
Warning: Path '/tmp/smoozy-node-setups/work/06/5.1/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 5.1 ===
```

#### What a linking file records when its library links onward (list entry 9) — Mac

Agreed with Linux. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
00:00.337  blend            | Read blend: "/tmp/smoozy-node-setups/work/09/5.1/top/top.blend"
00:00.340  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.1/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.343  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.1/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/5.1/mid/mid.blend'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/09/5.1/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/5.1/deep/model.obj', '/tmp/smoozy-node-setups/work/09/5.1/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/5.1/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0xc767af020> -> {'//model.obj', '//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0xc767ae720> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0xc78e63008> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0xc74a05220> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.336  blend            | Read blend: "/tmp/smoozy-node-setups/work/09/5.1/top/top.blend"
00:00.340  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.1/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.344  reports          | Read library: '/tmp/smoozy-node-setups/work/09/5.1/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/5.1/mid/mid.blend'
00:00.349  io.obj           | ERROR Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/09/5.1/deep/model.obj'.
00:00.349  reports          | ERROR OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/09/5.1/deep/model.obj'
BLENDER: 5.1.2 OPEN FILE: /tmp/smoozy-node-setups/work/09/5.1/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/5.1/deep/model.obj', '/tmp/smoozy-node-setups/work/09/5.1/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/5.1/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x7ac7e8320> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x7ac095520> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x7a8704c08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x7ac797020> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/tmp/smoozy-node-setups/work/09/5.1/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/tmp/smoozy-node-setups/work/09/5.1/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 5.1 ===
```

#### Whether the library path is recorded relative or absolute (list entry 10) — Mac

Agreed with Linux. The whole run, uncut. **The last part of it, which looks for the node files Blender ships, found nothing on this Mac** — the check looks beside the Blender program, which is where those files sit on Linux and is not where they sit on macOS. That is a fault of the check, not a fact about Blender, and it is recorded as it stands; list entry 16 below asks the same question in a way that works here.

```
BLENDER: 5.1.2 WORKDIR: /tmp/smoozy-node-setups/work/10/5.1
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/5.1/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/5.1/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: no such attribute
--- the node assets Blender ships with ---
   assets folder: /Applications/Blender 5.1.2.app/Contents/MacOS/5.1/datafiles/assets exists: False
DONE
```

#### Which node `.blend` files each Blender ships (list entry 16) — Mac

The shipped-file half of list entry 10, asked again by way of Blender's own answer to where its files are. The whole run, uncut:

```
BLENDER: 5.1.2
binary path: /Applications/Blender 5.1.2.app/Contents/MacOS/Blender
the folder the earlier check looked in: /Applications/Blender 5.1.2.app/Contents/MacOS/5.1/datafiles/assets
   exists: False
bpy.utils.resource_path('LOCAL'): /Applications/Blender 5.1.2.app/Contents/Resources/5.1
assets folder: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets exists: True
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/nodes/procedural_hair_node_assets.blend
   shipped .blend: /Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/nodes/shading_nodes_essentials.blend
count of shipped .blend files under the assets folder: 13
DONE
```

#### What a linked group holding a Bake node gives the modifier, and where that bake goes (list entry 17) — Mac

Built because the linked group in list entry 7 holds no Bake node, so list entry 15 found the modifier had no bake to show. A library file held a group of Group Input, Bake, Group Output; a second file linked that group onto a cube; then the second file was opened, read and baked. The report and the bake, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/bakeuse.blend and report, then bake ===
00:00.324  blend            | Read blend: "/tmp/smoozy-node-setups/work/17/5.1/shot/bakeuse.blend"
00:00.327  reports          | Read library: '/tmp/smoozy-node-setups/work/17/5.1/lib/bakelib.blend', '//../lib/bakelib.blend', parent '<direct>'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
BLENDER: 5.1.2 STEP: report WORKDIR: /tmp/smoozy-node-setups/work/17/5.1
node group: BakeLibSetup library: //../lib/bakelib.blend
modifier bake_target: PACKED
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=1103713778 bake_target=INHERIT directory=''
files beside the .blend before baking: ['bakeuse.blend', 'bakeuse.blend1']
Info: Bake directory of object Target, modifier GN is empty, setting default path
bake operator returned: {'FINISHED'}
modifier bake_directory after: '//blendcache_bakeuse/Target_GN'
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/1103713778/meta/00001_00000.json
DONE
```

The bake went beside the **linking** file, not beside the library: `blendcache_bakeuse` sits in the `shot` folder, next to `bakeuse.blend`, and the modifier's own Bake Path filled itself in as `//blendcache_bakeuse/Target_GN`, measured from the linking file. The library folder gained nothing.


#### Whether a linked group's Path field can be typed into in a window, and what the modifier shows (list entry 15) — Mac only

The Linux machine has no display server, so this cannot be reached there at all. Blender was started **with** its window, opening the `use.blend` that list entry 7 built, and driven by a script that showed the linked group in the geometry node editor, took Blender's own screenshot, then switched the properties editor to the modifier tab and took a second one, before quitting. No mouse was moved and nothing was clicked.

The first form of the script hit the same trouble as list entry 14 — every node in the group had been made at the same spot, so only the topmost was visible. It is recorded as it came back and settles nothing. The second form spread the nodes apart first. Blender printed:

```
00:00.749  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.752  blend.readfile   | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x874339820> indirect: False missing: False
moving the linked group's nodes apart was accepted
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (400.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (800.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (1200.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (1600.0, 0.0)
    Image Texture GeometryNodeImageTexture (2000.0, 0.0)
Warning: 2 × Draw window and swap: 41.7519 ms, average: 20.87593079 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 17.1530 ms, average: 8.57651234 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.1/linked-nodes.png
Path socket on the linked Import OBJ node holds: '//model.obj'
Path socket: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
the node group's own library_weak_reference: None
modifier: GN type: NODES
   bake_target: PACKED bake_directory: ''
   number of bake entries: 0
Warning: 2 × Draw window and swap: 12.0280 ms, average: 6.01398945 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.1/modifier-panel2.png
00:01.149  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

and, on the run that tried to open the Manage section:

```
00:00.748  blend            | Read blend: "/tmp/smoozy-node-setups/work/06/5.1/shot/use.blend"
00:00.752  blend.readfile   | Read library: '/tmp/smoozy-node-setups/work/06/5.1/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
modifier: GN bake entries: 0 bake_target: PACKED bake_directory: ''
ui_expand_flag before: no such property
setting ui_expand_flag raised: AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'
Warning: 3 × Draw window and swap: 107.8370 ms, average: 35.94565392 ms
screenshot written to /tmp/smoozy-node-setups/work/15/5.1/modifier-expanded.png
00:01.087  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

**The Path field on the linked group's Import OBJ node is drawn greyed out.** Put beside the screenshot from list entry 14, where the same node in a **local** group draws `//model.obj` in bright white with a bright folder button, the linked one draws the same text and the same button dimmed. All four versions drew it dimmed. That is worth setting against what list entry 7 found from a script: writing a new value into that same socket from Python **was** accepted. So the window and the script route differ — the window will not let the value be typed, and a script will.

The whole of the right-hand Group panel was drawn greyed out in the same way, in all four versions — the group's Name, its Node Width and the plus and minus buttons beside its sockets.

**The modifier's Bake panel was not there for this group.** The modifier showed only its node-group field, holding `LibSetup` and greyed out, and one collapsed section headed **Manage**. Blender reported `number of bake entries: 0` for the modifier, which is what a group holding no Bake node gives it — the group built for list entry 7 has none. The Manage section could not be opened from a script: `NodesModifier` has no property for the open-and-shut state of its sections, and the attempt printed `AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'`. Opening it needs a click, which is not allowed here, so **what the Manage section holds for a linked group is untested**. What a linked group with a Bake node in it gives the modifier is answered instead by list entry 17 above, without a window.

The screenshots are kept at `/tmp/smoozy-node-setups/work/15/5.1/` on that machine. This settles the drawing for **macOS only**; there is no Linux machine with a screen, and no Windows machine ran it.


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

2026-08-06 — Opus 5 (Claude Code) — Node setups, Blender 5.1

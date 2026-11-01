# Node setups — Blender 4.5

**Sources consulted:** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own add-on files — all read on **2026-07-30**. Blender's requirements page, read on **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**LABEL: documented** for routes A–E below, apart from a few steps marked **inferred** where they sit. **inferred** for route F, with the step written out. **not in the sources** for the sub-questions listed under "Not covered", and for the section headed "What is *not* on the list".

The sources describe six routes. I list them separately because they behave differently.

### A. Files named by a path on an import node

Six nodes read a file from a path given on the node.

> "Nodes used to import data from external file formats."
>
> "Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor.
>
> When dropping a supported file into a Geometry Node tree, the corresponding import node is added automatically and configured with the file path."
>
> "CSV (.csv) / Wavefront (.obj) / Stanford PLY (.ply) / STL (.stl) / Text (.txt) / OpenVDB (.vdb)"

— Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/index.html, read 2026-07-30

Each of the six pages names the file kind and the input:

> "The Import OBJ node loads geometry data from a .obj file and outputs it as instances." … "Path — The path to the .obj file to be imported."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/obj.html

> "The Import PLY node imports mesh data from a .ply (Polygon File Format or Stanford Triangle Format) file." … "Path — The path to the .ply file to be imported."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/ply.html

> "The Import STL node imports mesh geometry from a .stl (Stereolithography) file." … "Path — The path to the .stl file to be imported."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/stl.html

> "The Import VDB node loads volumetric data from a .vdb file and outputs it as a Volume geometry. All grids present in the file are loaded and included in the resulting volume." … "Path — The path to the VDB file to be imported."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/vdb.html

> "The Import CSV node reads data from a CSV (Comma-Separated Values) file and generates a point cloud." … "Path — The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/csv.html

> "The Import Text node reads the contents of a plain text file and outputs it as a single string." … "Currently, only files with the .txt extension are supported." … "Path — The path to the txt file."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/txt.html

All read 2026-07-30.

The release notes state these nodes are new in this version:

> "### Import Nodes
>
> Several nodes for importing external data have been added.
>
> * **Import PLY** … * **Import OBJ** … * **Import CSV** … * **Import STL** … * **Import Text** … * **Import VDB** …
>
> Files can be dropped directly into Geometry Nodes to create the corresponding import node."

— Blender release notes, https://developer.blender.org/docs/release_notes/4.5/geometry_nodes/, read 2026-07-30

**Two things about that path matter, and both are documented.**

First, the path is an *input socket*, not a fixed setting. The manual lists "Path" under **Inputs** on every one of the six pages, and each page's "Properties" section says the node has no properties (all six — OBJ, PLY, STL, VDB, CSV and Text — say "This node has no properties"; the CSV page's `Delimiter` sits under **Inputs**, not under Properties). *(Corrected by the citation check, 2026-08-07: this sentence previously said the CSV page listed the delimiter under Properties. It does not.)* The manual picture of the node shows an input socket dot next to the Path field and a file-browser folder button at its right — image `_images/node-types_GeometryNodeImportOBJ.webp`, shown on https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/obj.html, viewed 2026-07-30. **This part is documented.**

Because it is an input, the path can be computed while the setup runs. The release notes say so and point at the node meant for it:

> "Loading sequences is currently best done with the new *Format String* node."

— https://developer.blender.org/docs/release_notes/4.5/geometry_nodes/, read 2026-07-30

and the Format String node's own page gives a path as its final worked example:

> "Path with Frame Number
>
> Format: /output/image_{:04}.png
>
> Input: Integer 42
>
> Result: /output/image_0042.png"

— https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/utilities/text/format_string.html, read 2026-07-30

So the set of files such a setup reads is not necessarily a set of fixed strings stored in the file. It can depend on values computed at evaluation time (for example the frame). *(The two quotes above are documented; the sentence drawing them together is **inferred** — the step is: the Path is an input socket, the Format String node produces a string, therefore a Format String output can supply the Path.)*

Second, on the `//` prefix. The API documents a dedicated file-path string socket type:

> "class bpy.types.NodeSocketStringFilePath(NodeSocketStandard)
>
> String socket of a node
>
> default_value
>
> Input value used for unconnected socket
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

— Blender Python API, https://docs.blender.org/api/4.5/bpy.types.NodeSocketStringFilePath.html, read 2026-07-30

There is a matching group-interface socket type, `bpy.types.NodeTreeInterfaceSocketStringFilePath` (https://docs.blender.org/api/4.5/bpy.types.NodeTreeInterfaceSocketStringFilePath.html, read 2026-07-30), which means a file path can be exposed as an input of a node group. Note that this interface type's `default_value` is documented as `string, default "", (never None)` — without the "// prefix supported" note that the node socket carries. I report that difference as it stands; I do not know what it means.

Nothing in the sources says in so many words that the import nodes use `NodeSocketStringFilePath`. Concluding they do is **inferred** (step: the type is named "file path", it is the only file-path string socket in the API, and the import nodes are the nodes with a file-path input drawn with a file-browser button). *(Added by the citation check, 2026-08-07: hand test list entry 1 below settles this outright — every import node's Path reports itself as `NodeSocketStringFilePath`. The inference stands as written; it is no longer the only thing carrying the point.)*

### B. Image files, through an Image data-block

Three geometry nodes take an Image:

> "The Image Texture node is used to add an image file as a texture. The image data is sampled with the input Vector and outputs a Color and Alpha value." … "Image — The image socket can be used to connect to the Group Input node. If this is not connected the image can be opened or selected from the node."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/texture/image.html

> "The Image node provides access to a image file which allows you to conveniently enter and switch images for multiple nodes in the tree." … "Image Data-Block — The data-block selector to select an existing image or open an image from the file browser."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/constant/image.html

> "The Image Info node gets information from image and animation." … "Frame Count — The number of frames in an image or video frame sequence. For a static image, always 1."
> — https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/scene/image_info.html

All read 2026-07-30.

An Image data-block is a file reference:

> "Image | ✓ | ✓ | Image files. Used by shader nodes and textures."

— data-block table, https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30 (the two ticks are the "Link" and "Pack" columns of that table)

and it can stand for more than one file:

> "source
>
> Where the image comes from
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

and

> "filepath
>
> Image/Movie file name
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

— https://docs.blender.org/api/4.5/bpy.types.Image.html, read 2026-07-30

So this route can reach an image file. That it can also reach a numbered sequence of image files, a movie file, or a set of UDIM tile files is **inferred** — the step is that the `source` list above belongs to the Image data-block in general, and nothing quoted here says the Image socket on these three geometry nodes takes an Image of every kind on that list.

### C. A font file, through the String to Curves node

> "Properties
>
> Font
>
> Font glyph used to generate the curve."

— String to Curves node, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/utilities/text/string_to_curves.html, read 2026-07-30

> "Font | ✓ | ✓ | References font files. Used by curve object-data of text objects."

— data-block table, https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30

> "filepath
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

— `bpy.types.VectorFont`, https://docs.blender.org/api/4.5/bpy.types.VectorFont.html, read 2026-07-30

The step from the node's Font property to a file on disk is **inferred**. The node page says only "Font glyph used to generate the curve"; it does not say in so many words that the property holds a Font data-block. The two quotes after it are what carry a Font data-block to a file.

### D. Baked data written to a directory on disk

The Bake node, and simulation zones, can write to disk:

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
> Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

— Bake node, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/geometry/operations/bake.html, read 2026-07-30

> "The data format used to store geometry data is not considered to be an import/export format. Volume objects, however, are saved using the OpenVDB file format which can be used interoperably."
>
> "Data is not stored to disk until the Bake operation is ran."
>
> "Blend-files must be saved to a disk before data can be baked."

— https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/baking.html, read 2026-07-30

The path itself is stored on the modifier rather than in the node tree:

> "bake_directory
>
> Location on disk where the bake data is stored
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

— `bpy.types.NodesModifier`, https://docs.blender.org/api/4.5/bpy.types.NodesModifier.html, read 2026-07-30

> "directory
>
> Location on disk where the bake data is stored
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"
>
> "node
>
> Bake node or simulation output node that corresponds to this bake. This node may be deeply nested in the modifier node group. It can be none in some cases like missing linked data blocks."

— `bpy.types.NodesModifierBake`, https://docs.blender.org/api/4.5/bpy.types.NodesModifierBake.html, read 2026-07-30

So the Bake node is where the disk option is chosen, and the path itself is recorded on the modifier that runs the setup rather than in the node tree.

### E. Another `.blend` file, when the setup uses a node group linked from one

> "Existing node groups can be placed again after they're initially defined, be it in the same node tree or a different one. It's also possible to import node groups from a different blend-file using File ‣ Link/Append."

— https://docs.blender.org/manual/en/4.5/interface/controls/nodes/groups.html, read 2026-07-30

> "This section lists all the node groups, both those in the current blend-file and those Linked or Appended from another blend-file."

— https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/group.html, read 2026-07-30

Blender ships node groups this way itself:

> "This is a node group asset that is included in the bundled "Essentials" asset library."

— Smooth By Angle node group, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/normals/smooth_by_angle.html, read 2026-07-30

> "Blender includes many assets out of the box, these are contained in the "Essentials" library. Included in this library are: Hair node groups / Smooth By Angle Node Group / Brushes: …"

— https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html, read 2026-07-30

Whether using such an asset leaves a reference to its file depends on how it was brought in:

> "Link — The asset will be linked to the current blend-file, and thus be read-only. Later changes to the asset file will be reflected in all files that link it in.
>
> Append — All of the asset and all its dependencies will be appended to the current file. … Since the file now has its own copy of the asset, later changes to the asset file will not be reflected in the file it's appended to.
>
> Append (Reuse Data) — … the first time an asset is used, it will be appended, including its dependencies, just like described previously. However, Blender will keep track of where it originated, and the next time the asset is used, as much data as possible will be reused."

— https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30

### F. Files reached through other data-blocks the setup names — **inferred**

Several geometry nodes take a data-block rather than a path: Object Info and Collection Info (https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/scene/object_info.html and .../collection_info.html), the Object, Collection and Material input nodes (.../input/constant/), and Set Material / Replace Material / Material Selection (.../material/). All read 2026-07-30.

**The step:** the data-block table (https://docs.blender.org/manual/en/4.5/files/data_blocks.html) says of the types in it that the Link column means "Library Linking, supports being linked into other blend-files.", and marks Object, Collection, Material, Node Tree, Image and Font in that column *(wording corrected against the source by the citation check, 2026-08-07)*, and that some data-blocks reference files of their own (for example `bpy.types.Volume.filepath`, "Volume file used by this Volume data-block", https://docs.blender.org/api/4.5/bpy.types.Volume.html, read 2026-07-30). Therefore a setup that names one of these data-blocks can, through it, make the file depend on whatever that data-block depends on — a library `.blend` if the data-block is linked, or a `.vdb` if it is a volume, or image files if it is a material using image textures. No source in this set says this about geometry nodes in so many words, which is why the label here is inferred and not documented.

One documented trace of this is the baking panel:

> "Data-Block References
>
> Baked geometries that reference other data-blocks such as materials are listed here.
>
> This panel allows changing these references after the data has been baked.
>
> Note: Currently only material data-blocks are supported."

— https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/baking.html, read 2026-07-30

and the API type behind it records where such a data-block came from:

> "lib_name
>
> Used when the data block is not local to the current .blend file but is linked from some library"

— `bpy.types.NodesModifierDataBlock`, https://docs.blender.org/api/4.5/bpy.types.NodesModifierDataBlock.html, read 2026-07-30

### What is *not* on the list

**LABEL: not in the sources.** The sources name no script node in the geometry node set — nothing in the node list or the geometry nodes manual pages refers to an external script file. Searching `manual_4.5/modeling/geometry_nodes/` for "script" and "osl" returned nothing, and the full list of `GeometryNode` subclasses at https://docs.blender.org/api/4.5/bpy.types.GeometryNode.html (read 2026-07-30) names no script node. That list likewise names no Alembic, USD or cache node. This is a record of what the search found and did not find, not a statement that no such node exists.

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

- Whether the paths on import nodes are seen by Blender's path-management machinery. `bpy.ops.file.make_paths_relative()` is documented only as "Make all paths to external files relative to current .blend", `bpy.ops.file.make_paths_absolute()` as "Make all paths to external files absolute", and `bpy.ops.file.report_missing_files()` as "Report all missing external files" (https://docs.blender.org/api/4.5/bpy.ops.file.html, read 2026-07-30). Nothing says which paths count as "external files". The manual's External Data menu is equally general: "External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path." (https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html, read 2026-07-30). **Settled by hand test list entry 5 below: in 4.5 they are not — the node's path did not move when Make Paths Relative ran, and Report Missing Files did not name it.**
- Whether an import node's file can be packed into the `.blend`. The packing table lists a Pack column only for data-block types (Font, Image, Library, Sounds get a tick; Movie Clip and Text get a cross) — https://docs.blender.org/manual/en/4.5/files/data_blocks.html. An import node's file is not a data-block at all, so it does not appear in that table. The sources do not say it can or cannot be packed.
- Where baked data goes when no bake path is set (`bake_directory` default is `""`). Not stated on either baking page, and I read both in full — see "Where I looked" below. **Settled by hand test list entry 6 below: Blender fills the path in itself as `//blendcache_<file>/<object>_<modifier>`.**
- Whether an import node re-reads its file every evaluation, holds it in memory, or reloads when the file changes on disk. Not stated anywhere in the geometry nodes pages (searched for "cache", "reload", "dependenc", "missing file" across `manual_4.5/modeling/geometry_nodes/`). **Partly settled by hand test list entry 3 below — see the note added to that entry, because the answer turns on how the geometry is asked for.**
- What happens when an import node's file is absent or unreadable. Not stated on any of the import node pages. The search behind this one ran only inside the geometry nodes folder of the manual, so its record of looking is thinner than the others here, and it has been sent back to be searched again. **Settled by hand test list entries 3 and 4 below: Blender prints an error naming the file and the node yields nothing; the file still opens and Blender still exits normally.**
- Whether the `//` prefix is honoured in an import node's Path. See the inference above; not stated outright. **Settled by hand test list entry 3 below: it is.**

### Where I looked

- Listed every page under `manual_4.5/modeling/geometry_nodes/` and read: `input/import/` (all seven pages), `texture/image.txt`, `input/constant/image.txt`, `input/scene/image_info.txt`, `utilities/text/string_to_curves.txt`, `utilities/text/format_string.txt`, `geometry/operations/bake.txt`, `baking.txt`, `group.txt`, `simulation/simulation_zone.txt`, `normals/smooth_by_angle.txt`.
- Searched the whole manual for: "import" inside the geometry nodes tree; "Import OBJ", "Import VDB", "Import CSV", "import node"; "file path"; "relative"; "pack"; "Make Paths Relative", "Report Missing Files", "Find Missing Files"; "Essentials"; "blendcache", "Bake Path"; "script", "osl" inside the geometry nodes tree; "dependenc", "external file", "missing file", "cache", "reload" inside the geometry nodes tree.
- Searched the whole API documentation for: `GeometryNodeImport*`; `FILE_PATH`; `StringFilePath`; `NodesModifier*`; `filepath` on Image, VectorFont, Volume; `bpy.ops.file` path operators.
- Read `manual_4.5/files/data_blocks.txt`, `files/file_paths.txt`, `files/blend/packed_data.txt`, `files/blend/open_save.txt`, `interface/window_system/topbar.txt`, `modeling/modifiers/generate/geometry_nodes.txt`.
- Read the whole of `release_notes_4.5/geometry_nodes.md`, and searched `release_notes_4.5/` for "path", "librar", "file path". Note: the release notes in this snapshot are text files only — the pictures they refer to (for example `images/geometry-nodes-import-sequence.png`) are not present, so I could not look at them.
- Cycles' own files: searched `cycles_4.5/*.py` for "geometry node", "node_group", "filepath", "abspath". The only hits are Cycles' own render-output and denoising operators (`cycles_4.5/operators.py`, `cycles_4.5/engine.py`); nothing there concerns geometry node setups or their file references. Ruled out.
- The requirements page names no node or file-reference behaviour at all. Ruled out for this question.

### Outside the interface

The sources do not say. Two documented items bear on it, neither settling it:

- Dragging a file into the node editor to create a configured import node is plainly an interface action — "Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor" (manual, import nodes index, read 2026-07-30). Driven from outside, the node and path would have to be set some other way. What is stored in the file afterwards is the same either way. *(That last sentence is **inferred**: the manual describes the drag as a way of adding and configuring the node, not as a different kind of node.)*
- The command-line documentation shows that node evaluation is expected to happen when a file is loaded without the interface: "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)." (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). That the default is to build and evaluate them is **inferred** — the step is that the option is worded as switching the building off, which only reads sensibly if it is otherwise on. Whether that evaluation reads the import nodes' files is **not stated**.

**Brought up to date by hand test, 2026-08-06 — observed.** A Blender started with no window does read an import node's file when it opens the `.blend`, and does read it again at render time; the rendered picture was the same with `--disable-depsgraph-on-file-load` in force as without it. Every route in this answer was built, set and read from a script in a Blender with no window at all, on a machine with no display server, so nothing here needs a person clicking. The one exception is dragging a file into the node editor to make an import node: no script can make a real drag happen, so it cannot be tested at all and sits in the "Cannot be run" section of `Audits/Blender Research/Node setups/Hand tests.md`. See "Observed by hand test" below, list entries 4 and 12. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers: a Blender started with no window reads an import node's file when it opens the `.blend`, and again at render time, and the picture was the same with `--disable-depsgraph-on-file-load` in force as without it. So this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so one thing could also be looked at there that no terminal run can settle: the Path row is drawn as a text field holding the path, with a folder button at its right-hand end and a socket dot on the node's left edge level with it, and the value it shows is the one a script reads back. See "Observed by hand test on the Creator's Mac" below, list entries 4, 12 and 14. Dragging a file into the node editor to make an import node still cannot be tested at all, on either machine.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back was built, changed and read entirely from a script in a Blender with no window at all, on a machine with no display server, so nothing in it needs a person clicking. It also turns up something anyone driving Blender from outside should know: asking for the evaluated geometry again does not by itself make an import node read its file again. The setup has to be marked for a fresh run, **and** the file's modification time has to have changed by a whole second. A script that writes a file and immediately asks for the geometry can therefore be handed the old contents, with nothing said about it. Observed on Linux in all four versions; untested on macOS, because the rented machine was given up before the Mac half could be run, and Windows was not tried at all. See "Observed by hand test — the test the citation check sent back" below, list entry 18.


### Operating system

The sources do not say that any of this differs by operating system. The requirements page states Blender 4.5 LTS is supported on Windows, macOS and Linux (it names 4.5 explicitly under each of the three "Requirements for Older Versions" headings — https://www.blender.org/download/requirements/, read 2026-08-01), and nothing on that page concerns file references in node setups. The only OS-flavoured note I found anywhere near paths is `bpy.path.basename`: "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." (https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30), which is about writing Python, not about what the file points at.

**Brought up to date by hand test, 2026-08-06 — observed.** On Linux a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, and an image path behaves the same way — see "Observed by hand test" below, list entry 11. That is the one place these tests found an operating system could make a difference, and it has been settled for Linux only. Nothing here has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed. This is the one place the two machines disagreed.** The same test on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — came out the other way: `//MODEL.OBJ` and `//Model.Obj` both found `model.obj`, an image path in upper case loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back instead of correcting it to the file that exists. So **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. The two results are kept apart and never merged into one. Windows is still untested, because no Windows machine has run it. Everything else in this answer was run on both machines and agreed on both. See "Observed by hand test on the Creator's Mac" below, list entry 11.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back found a second place where an operating system could make a difference, and it could only be run on one of them. Whether an import node reads its file again turns on the file's modification time, and on the *whole second* of it: a change inside the same whole second is not noticed, a change that crosses one is, in either direction. That was settled on the rented Linux machine's ordinary file system, in all four versions, across eighty runs with no exception. Whether macOS keeps and compares that stamp the same way is **untested** — the rented machine was given up before the Mac half could be run — and no Windows machine has run it. So this holds for Linux and is untested on both of the others. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

All of this was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all** — on **2026-08-06**, with `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/root/node-setups/config/4.5`, a settings folder made for these tests before anything was started, never Blender's real one. The machine was shared with other work while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below. **Every one of these was run again on the Creator's Mac on 2026-08-07; that block follows this one. All but one agreed. The one that did not is letter case, list entry 11.** The two machines' results are kept apart and never merged.


#### What the build itself reports about the import nodes (list entry 1)

Started Blender with no window and a script that listed the classes the build registers, then built a throwaway geometry node tree and read the socket type of every input on each import node, tried to link a String node into the Path input, and looked at String to Curves. Blender printed, in full:

```
BLENDER: 4.5.12 LTS
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
223
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
links.new returned: <bpy_struct, NodeLink at 0x7026652df048>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: True
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)

Blender quit
```

#### What path-related facilities this build has (list entry 2)

The same shape of run, reading each type's properties out of Blender's own description of it. In full:

```
BLENDER: 4.5.12 LTS
--- bpy.data path facilities ---
bpy.data.file_path_map PRESENT
bpy.data.file_path_foreach ABSENT
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
Library.is_archive ABSENT
Library.archive_libraries ABSENT
Library.archive_parent_library ABSENT
Library.packed_file PRESENT
Library.reload PRESENT
--- ID properties ---
ID.library PRESENT
ID.is_library_indirect PRESENT
ID.is_linked_packed ABSENT
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
TypeError: 'pack' is an invalid keyword argument for load()
--- file path foreach flag items, if the enum exists ---
raised: KeyError 'bpy_prop_collection[key]: key "file_path_foreach" not found'
--- bpy.data.file_path_map on an empty factory file: keys with a non-empty set ---
{}
--- bpy.utils.blend_paths on an empty factory file ---
['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)

Blender quit
```

One line in that script asks for the flag list of `file_path_foreach` and comes back with a `KeyError`; that line settles nothing either way and is left in the record as it stands. An earlier form of the whole check asked `hasattr` on those classes instead and reported every property absent in every version — a fault of the check, not a fact about Blender, and the reason the run above reads them from `bl_rna` instead.

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3)

A folder was made holding `model.obj` (three vertices, with `mtllib model.mtl` and `usemtl red` in it), `model.mtl`, a four-by-four `tex.png`, and `scene.blend` saved beside them. Inside the file one object carried a Geometry Nodes modifier whose group was Import OBJ → Realize Instances → Join Geometry → Group Output, with an Image Texture node holding the image. The Path was set to five values in turn; then, with the file working, `model.obj` was overwritten on disk with a four-vertex shape and the geometry read again after each of several things. In full:

*(Note added by the citation check, 2026-08-07.* This test was rebuilt from the words above and run again on the same rented Linux machine. The first half — which path forms the Path accepts, and what a missing one does — came back identical. The second half depends on something these words do not say: **how** the geometry is read again. Read plainly, it gives three vertices at every step, as recorded below. If the object is marked for update before the evaluated geometry is asked for — an ordinary thing to do, and not ruled out by anything above — every reading from the overwrite onwards comes back as **four**. Both runs happened; neither overwrites the other, and this check did not pick between them. The question has gone back to the hand tests to be run again in a form that says which reading is meant.*)*

```
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'./model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: OBJ Import: Cannot open file './model.obj'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/03/4.5/nothing-here.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/03/4.5/nothing-here.obj'
BLENDER: 4.5.12 LTS WORKDIR: /root/node-setups/work/03/4.5
Info: Saved as "scene.blend"
saved: /root/node-setups/work/03/4.5/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/root/node-setups/work/03/4.5/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/root/node-setups/work/03/4.5/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /root/node-setups/work/03/4.5/model.obj
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
Read blend: "/root/node-setups/work/03/4.5/scene.blend"

Blender quit
```

#### Whether a Blender with no window reads the file when it opens the `.blend` (list entry 4)

The folder that test left behind, with the four-vertex `model.obj` on disk and the `.blend` still pointing at `//model.obj`, opened three times in fresh Blenders started with `-b`. In full:

```
=== fresh process, model.obj on disk now has 4 vertices ===
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/03/4.5/scene.blend"

Blender quit
=== same, with model.obj moved away ===
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/03/4.5/model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/03/4.5/scene.blend"
Error: OBJ Import: Cannot open file '/root/node-setups/work/03/4.5/model.obj'
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE

Blender quit
exit code: 0
=== same, with --disable-depsgraph-on-file-load ===
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/03/4.5/scene.blend"

Blender quit
```

The third run does not isolate what `--disable-depsgraph-on-file-load` does, because the check itself asks for an evaluated dependency graph, which builds one. The render comparison below answers that half.

#### Whether a render with no window reads the file, judged by the picture (list entry 12)

A second file was built holding the same Import OBJ node, an orthographic camera over the shape, the Workbench engine and a sixty-four by sixty-four frame, and frame 1 was rendered four times. The pictures were compared colour value by colour value, not byte for byte, because Blender writes text including times into every image it saves. The steps that carry the answer, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /root/node-setups/work/12/4.5/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/4.5/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4307   mean value B: 0.4214
--- with the file, against with the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/4.5/with_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/4.5/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4307   mean value B: 0.4307
--- without the file, against without the file and the depsgraph switch ---
picture A: /root/node-setups/work/12/4.5/without_0001.png size (64, 64) values 16384
picture B: /root/node-setups/work/12/4.5/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4214   mean value B: 0.4214
=== END 4.5 ===
```

#### Whether the file read can change from frame to frame (list entry 13)

Three shapes were put on disk — `data_1.obj` with three vertices, `data_2.obj` with four, `data_3.obj` with five — and a node group joined a String node holding `//data_`, a Value to String node fed by the Scene Time node's Frame output, and a String node holding `.obj`, into the Import OBJ node's Path. In full:

```
BLENDER: 4.5.12 LTS WORKDIR: /root/node-setups/work/13/4.5
Info: Saved as "perframe.blend"
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt')]
Join Strings inputs: [('Delimiter', 'NodeSocketString', False), ('Strings', 'NodeSocketString', True)]
links made: 8
frame 1 -> vertices: 3
frame 2 -> vertices: 4
frame 3 -> vertices: 5
frame 1 -> vertices: 3
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)

Blender quit
```

The first form of that script linked the three pieces in reading order and Blender joined them the other way round, giving paths like `.obj1//data_` and no geometry at any frame, with `1`, `2` and `3` appearing in the three failed paths. Both runs happened; the one above is the one that carries the answer.

#### Whether an import node's path takes part in the file's own path housekeeping (list entry 5)

A file was built holding an Import OBJ node's path and an ordinary image path side by side. The image is the control: it is a kind of file Blender's own documentation marks as packable, so a run in which nothing at all moved can be told apart from a run in which only the node's path failed to move. In full:

```
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/05/4.5/gone-model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: OBJ Import: Cannot open file '/root/node-setups/work/05/4.5/gone-model.obj'
BLENDER: 4.5.12 LTS WORKDIR: /root/node-setups/work/05/4.5
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/root/node-setups/work/05/4.5/model.obj'        image filepath = '/root/node-setups/work/05/4.5/tex.png'
Info: Total files 3 | Changed 1 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/root/node-setups/work/05/4.5/model.obj'        image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 3 | Changed 3 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '/root/node-setups/work/05/4.5/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/root/node-setups/work/05/4.5/gone-tex.png' not found, from local data-block 'IMtex.png'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png']
blend_paths(absolute=True): ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/05/4.5/tex.png']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x7fc0276dc308> -> {'//tex.png'}
file_path_foreach: not present in this version
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
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/05/4.5/model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/05/4.5/packed.blend"
Error: OBJ Import: Cannot open file '/root/node-setups/work/05/4.5/model.obj'
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
BLENDER: 4.5.12 LTS WORKDIR: /root/node-setups/work/07/4.5
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /root/node-setups/work/07/4.5/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=587785519 bake_target=INHERIT directory=''
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
   file on disk: <workdir>/blendcache_bake/Target_GN/587785519/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/587785519/blobs/00001_00000.blob
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/mybakes/587785519/meta/00001_00000.json
   file on disk: <workdir>/mybakes/587785519/blobs/00001_00000.blob
   file on disk: <workdir>/blendcache_bake/Target_GN/587785519/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/587785519/blobs/00001_00000.blob
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)

Blender quit
```

#### Whether these paths are matched letter for letter (list entry 11)

With `model.obj` and `tex.png` on disk in lower case. In full:

```
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/11/4.5/MODEL.OBJ'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: OBJ Import: Cannot open file '/root/node-setups/work/11/4.5/MODEL.OBJ'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/11/4.5/Model.Obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/11/4.5/Model.Obj'
BLENDER: 4.5.12 LTS WORKDIR: /root/node-setups/work/11/4.5
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 0
PATH='//Model.Obj'    -> vertices: 0
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: False size: (0, 0) pixels: 0
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /root/node-setups/work/11/4.5/model.obj
DONE

Blender quit
```

This is Linux, whose ordinary file system tells letter case apart. It says nothing about macOS or Windows.

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

All of this was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — on **2026-08-07**, with `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)`. Every run was started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/4.5`, a settings folder made for these tests before anything was started, never Blender's real one. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing recorded here measures time or the memory on the card, so that does not bear on the results. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests.md`, where each test also carries the number used below.

Every test in the Linux block above was run again here, in the same order, from the same scripts, with only the paths of the program, the settings folder and the working folder changed. **Eight of the nine agreed with Linux. One did not: letter case, list entry 11.** The results from the two machines are set out separately and are not folded into one.

Where the two agreed, the Mac's own words are given for the lines that carry the answer; the rest of each run printed the same words as the Linux block above, apart from the working-folder paths, the program's own path and the build's hash line, which on this Mac reads `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)`. Where I have kept only part of a long run, I say so at that point. The whole of every run was kept as it was printed, in `/Users/artist/Documents/Smoozy hand tests/Node setups/logs/` on that machine.


#### What the build itself reports about the import nodes (list entry 1) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 4.5.12 LTS
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
223
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
links.new returned: <bpy_struct, NodeLink at 0xbb0a59748>
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
BLENDER: 4.5.12 LTS
--- bpy.data path facilities ---
bpy.data.file_path_map PRESENT
bpy.data.file_path_foreach ABSENT
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
Library.is_archive ABSENT
Library.archive_libraries ABSENT
Library.archive_parent_library ABSENT
Library.packed_file PRESENT
Library.reload PRESENT
--- ID properties ---
ID.library PRESENT
ID.is_library_indirect PRESENT
ID.is_linked_packed ABSENT
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
TypeError: 'pack' is an invalid keyword argument for load()
--- file path foreach flag items, if the enum exists ---
raised: KeyError 'bpy_prop_collection[key]: key "file_path_foreach" not found'
--- bpy.data.file_path_map on an empty factory file: keys with a non-empty set ---
{}
--- bpy.utils.blend_paths on an empty factory file ---
['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
```

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (list entry 3) — Mac

Agreed with Linux. The whole run, uncut, including what Blender printed about the paths it could not open:

```
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'./model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Error: OBJ Import: Cannot open file './model.obj'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/4.5/nothing-here.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/4.5/nothing-here.obj'
BLENDER: 4.5.12 LTS WORKDIR: /tmp/smoozy-node-setups/work/03/4.5
Info: Saved as "scene.blend"
saved: /tmp/smoozy-node-setups/work/03/4.5/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/tmp/smoozy-node-setups/work/03/4.5/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: []
PATH='/tmp/smoozy-node-setups/work/03/4.5/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: []
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: []
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /tmp/smoozy-node-setups/work/03/4.5/model.obj
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
########## BLENDER 4.5 — fresh process, file on disk now has 4 vertices ##########
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/03/4.5/scene.blend"

Blender quit
########## BLENDER 4.5 — same, with model.obj moved away ##########
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/03/4.5/model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/03/4.5/scene.blend"
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/03/4.5/model.obj'
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE

Blender quit
exit code: 0
########## BLENDER 4.5 — same, with --disable-depsgraph-on-file-load ##########
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/03/4.5/scene.blend"

Blender quit
```

#### Whether a render with no window reads the file, judged by the picture (list entry 12) — Mac

Agreed with Linux. The comparison of the pictures, uncut:

```
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /tmp/smoozy-node-setups/work/12/4.5/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/4.5/without_0001.png size (64, 64) values 16384
RESULT: 558 of 16384 colour values differ between the two pictures
mean value A: 0.4307   mean value B: 0.4214
--- with the file, against with the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/4.5/with_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/4.5/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4307   mean value B: 0.4307
--- without the file, against without the file and the depsgraph switch ---
picture A: /tmp/smoozy-node-setups/work/12/4.5/without_0001.png size (64, 64) values 16384
picture B: /tmp/smoozy-node-setups/work/12/4.5/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4214   mean value B: 0.4214
=== END 4.5 ===
```

#### Whether the file read can change from frame to frame (list entry 13) — Mac

Agreed with Linux. The whole run, uncut:

```
BLENDER: 4.5.12 LTS WORKDIR: /tmp/smoozy-node-setups/work/13/4.5
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
BLENDER: 4.5.12 LTS WORKDIR: /tmp/smoozy-node-setups/work/05/4.5
Info: Saved as "scene.blend"
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/tmp/smoozy-node-setups/work/05/4.5/model.obj'   image filepath = '/tmp/smoozy-node-setups/work/05/4.5/tex.png'
Info: Total files 3 | Changed 1 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/tmp/smoozy-node-setups/work/05/4.5/model.obj'   image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: Total files 3 | Changed 3 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                                    image filepath = '/tmp/smoozy-node-setups/work/05/4.5/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                                    image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                               image filepath = '//gone-tex.png'
Warning: Path '/tmp/smoozy-node-setups/work/05/4.5/gone-tex.png' not found, from local data-block 'IMtex.png'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png']
blend_paths(absolute=True): ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/05/4.5/tex.png']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x8bb82f808> -> {'//tex.png'}
file_path_foreach: not present in this version
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
########## BLENDER 4.5 — packed.blend reopened with model.obj and tex.png moved away ##########
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/05/4.5/model.obj'.
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/05/4.5/packed.blend"
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/05/4.5/model.obj'
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
BLENDER: 4.5.12 LTS WORKDIR: /tmp/smoozy-node-setups/work/07/4.5
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /tmp/smoozy-node-setups/work/07/4.5/bake.blend
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=1606096776 bake_target=INHERIT directory=''
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
   file on disk: <workdir>/blendcache_bake/Target_GN/1606096776/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/1606096776/blobs/00001_00000.blob
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/1606096776/meta/00001_00000.json
   file on disk: <workdir>/blendcache_bake/Target_GN/1606096776/blobs/00001_00000.blob
   file on disk: <workdir>/mybakes/1606096776/meta/00001_00000.json
   file on disk: <workdir>/mybakes/1606096776/blobs/00001_00000.blob
DONE
```

#### Whether these paths are matched letter for letter (list entry 11) — Mac: **this one did not agree with Linux**

With `model.obj` and `tex.png` on disk in lower case, exactly as on Linux. The whole run, uncut:

```
BLENDER: 4.5.12 LTS WORKDIR: /tmp/smoozy-node-setups/work/11/4.5
Info: Saved as "scene.blend"
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 3
PATH='//Model.Obj'    -> vertices: 3
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'    -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'    -> has_data: True size: (4, 4) pixels: 64
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /tmp/smoozy-node-setups/work/11/4.5/MODEL.OBJ
DONE
```

On Linux the same run gave `//MODEL.OBJ -> vertices: 0`, `//Model.Obj -> vertices: 0`, an image that did not load for `//TEX.PNG`, and a `bpy.path.resolve_ncase` that turned the upper-case name back into the lower-case file that exists. On this Mac all three upper-case forms found the file, the image loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back. The two results are recorded side by side and are not merged: **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. Windows is untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The working folder for this run was under `/tmp`, which on this Mac sits on the volume named `Macintosh HD - Data`, an APFS volume — the machine's own ordinary file system, not one made for the test.


#### What the Path row on an Import node offers in a window (list entry 14) — Mac only

This is the one the Linux machine cannot reach at all: it has no display server, so nothing drawn in a window can be looked at there. Blender was started **with** its window, opening the `scene.blend` that list entry 3 built, and driven by a script that switched an area to the geometry node editor, showed the node group, framed the nodes with Blender's own View All, took Blender's own screenshot and quit. No mouse was moved and nothing was clicked.

The first form of the script took the screenshot without framing anything, and the second added View All. Both came back showing only the Image Texture node, because every node in the group had been made at the same spot and they were drawn on top of one another. Both are recorded as they came back; neither settles the question. The third form spread the nodes apart first, the way a person would drag them, and that is the one that answers it. Blender printed:

```
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
area: NODE_EDITOR region: WINDOW 2495 x 1453
Warning: 2 × Draw window and swap: 54.6100 ms, average: 27.30500698 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 18.9052 ms, average: 9.45258141 ms
screenshot written to /tmp/smoozy-node-setups/work/14/4.5/spread-all.png
Warning: 2 × Draw window and swap: 10.6618 ms, average: 5.33092022 ms
view_selected returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 16.0210 ms, average: 8.01050663 ms
screenshot written to /tmp/smoozy-node-setups/work/14/4.5/import-node.png
Path socket on the Import OBJ node holds: '//model.obj'
Path socket type: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
Read blend: "/tmp/smoozy-node-setups/work/03/4.5/scene.blend"
Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

The screenshot shows the Import OBJ node drawn with a title bar reading **Import OBJ**, an **Instances** output on its right edge, and one row below it: a text field holding **`//model.obj`**, a **folder button** at that field's right-hand end, and a **socket dot on the node's left edge level with that row**. All four versions drew the same three things. The only difference between them is the shape of the socket dots — round in 4.5, rectangular in 5.0, 5.1 and 5.2 — which is Blender's own drawing style, not anything about paths.

The screenshots are kept at `/tmp/smoozy-node-setups/work/14/4.5/` on that machine. This settles the drawing of the node for **macOS only**. There is no Linux machine with a screen in this project, and no Windows machine ran it, so what the row looks like on either of those is untested.


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
4.5 plain, ten repeats, count at the reading after the modifier touch: 3 3 3 3 3 3 3 3 3 3
```

**Why the four arrangements above do not settle it.** The plain arrangement was run three more times on 5.1, where it had just given four. Two of those three runs gave three and one gave four, from the very same script minutes apart. Nothing in the script changed between them. So whatever decides this is not the arrangement — it is something that varies from run to run, and the runs below name it.

**The stamp on the file, set by hand instead of left to the clock.** The next run puts the three-vertex file's modification time at a quarter of a second past a known whole second, then moves it by a chosen amount at the overwrite, so that whether the whole second changes is decided rather than left to chance. Each line prints the two stamps, whether the whole second changed, and four readings: the one before the overwrite, then three after it, with the modifier touched between them.

```
BLENDER 4.5.12 LTS offset  +0.000  tag False  stamp 1786081732.250 -> 1786081732.250  whole second 1786081732 -> 1786081732  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.000  tag True   stamp 1786081733.250 -> 1786081733.250  whole second 1786081733 -> 1786081733  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.500  tag False  stamp 1786081733.250 -> 1786081733.750  whole second 1786081733 -> 1786081733  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.500  tag True   stamp 1786081734.250 -> 1786081734.750  whole second 1786081734 -> 1786081734  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.700  tag False  stamp 1786081734.250 -> 1786081734.950  whole second 1786081734 -> 1786081734  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.700  tag True   stamp 1786081735.250 -> 1786081735.950  whole second 1786081735 -> 1786081735  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.740  tag False  stamp 1786081735.250 -> 1786081735.990  whole second 1786081735 -> 1786081735  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.740  tag True   stamp 1786081736.250 -> 1786081736.990  whole second 1786081736 -> 1786081736  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  +0.760  tag False  stamp 1786081736.250 -> 1786081737.010  whole second 1786081736 -> 1786081737  changed: True   readings: 3 3 4 4
BLENDER 4.5.12 LTS offset  +0.760  tag True   stamp 1786081737.250 -> 1786081738.010  whole second 1786081737 -> 1786081738  changed: True   readings: 3 4 4 4
BLENDER 4.5.12 LTS offset  +0.900  tag False  stamp 1786081737.250 -> 1786081738.150  whole second 1786081737 -> 1786081738  changed: True   readings: 3 3 4 4
BLENDER 4.5.12 LTS offset  +0.900  tag True   stamp 1786081738.250 -> 1786081739.150  whole second 1786081738 -> 1786081739  changed: True   readings: 3 4 4 4
BLENDER 4.5.12 LTS offset  +1.000  tag False  stamp 1786081738.250 -> 1786081739.250  whole second 1786081738 -> 1786081739  changed: True   readings: 3 3 4 4
BLENDER 4.5.12 LTS offset  +1.000  tag True   stamp 1786081739.250 -> 1786081740.250  whole second 1786081739 -> 1786081740  changed: True   readings: 3 4 4 4
BLENDER 4.5.12 LTS offset  +2.000  tag False  stamp 1786081740.250 -> 1786081742.250  whole second 1786081740 -> 1786081742  changed: True   readings: 3 3 4 4
BLENDER 4.5.12 LTS offset  +2.000  tag True   stamp 1786081740.250 -> 1786081742.250  whole second 1786081740 -> 1786081742  changed: True   readings: 3 4 4 4
BLENDER 4.5.12 LTS offset  -0.200  tag False  stamp 1786081741.250 -> 1786081741.050  whole second 1786081741 -> 1786081741  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  -0.200  tag True   stamp 1786081741.250 -> 1786081741.050  whole second 1786081741 -> 1786081741  changed: False  readings: 3 3 3 3
BLENDER 4.5.12 LTS offset  -0.500  tag False  stamp 1786081742.250 -> 1786081741.750  whole second 1786081742 -> 1786081741  changed: True   readings: 3 3 4 4
BLENDER 4.5.12 LTS offset  -0.500  tag True   stamp 1786081742.250 -> 1786081741.750  whole second 1786081742 -> 1786081741  changed: True   readings: 3 4 4 4
```

**Whether a reading on its own is enough.** The last run overwrites the file, always moving the stamp a whole second, and then reads five times over — once with the modifier touched between readings and once without.

```
BLENDER 4.5.12 LTS touch True  tag False  whole second 1786081829 -> 1786081830  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 4.5.12 LTS touch True  tag True   whole second 1786081830 -> 1786081831  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 4.5.12 LTS touch False tag False  whole second 1786081830 -> 1786081831  first reading 3 then [3, 3, 3, 3, 3]
BLENDER 4.5.12 LTS touch False tag True   whole second 1786081831 -> 1786081832  first reading 3 then [4, 4, 4, 4, 4]
```

**What the runs show.** Three things, and all three are the same in 4.5, 5.0, 5.1 and 5.2.

1. **Asking for the evaluated geometry again does not by itself make the setup run again.** With nothing touched and nothing tagged, five readings in a row give the old count, however many times you ask. Something has to mark the setup for a fresh run: touching the modifier does it, and so does `ob.update_tag()`.
2. **Even then, the file is only read again if its modification time has changed by a whole second.** A change inside the same whole second is not noticed at all — the setup runs again and comes back with the old contents. A change that crosses a whole second is noticed every time. It is noticed in either direction: a stamp moved a second *backwards* counts as a change just as much as one moved forwards. Across eighty runs — four versions, ten stamp offsets, both arrangements — there was not one exception in either direction.
3. **Marking the object for update is one of the things that make the setup run again, and it is simply the earliest one.** With the stamp changed, the tagged arrangement shows the new contents at the very first reading after the overwrite; the plain arrangement shows them at the first reading that follows a nudge, which in entry 3's sequence is the reading after the modifier is touched. One reading of difference, and no more.

**So this is exactly what tells the two arrangements apart.** Not whether the file is read again — that is decided by the stamp on the file, and by nothing either arrangement does. What the tagging changes is *when* the new contents first appear: at the first reading rather than one reading later. That is the whole of the difference between them.

**And this is why the two recorded results disagreed.** Entry 3's script overwrites `model.obj` a few hundredths of a second after writing it, so in almost every run the whole second is unchanged and nothing is read again — three vertices at every step, which is what entry 3 records, and what the ten repeats above gave — and the same ten in each of the four versions, forty runs without one exception. Now and then the overwrite lands on the far side of a second boundary, and a four appears. With the object tagged the four appears at the first reading; without the tag it appears one reading later. Both recorded results are real, neither is wrong, and neither had to be chosen between: they differ by where the run happened to fall inside a second.

**What this does not settle.** Linux only. The machine was given up before this could be run on the Creator's Mac, and no Windows machine has run it, so whether either of those keeps and compares the stamp the same way is untested. Nothing here says *why* the whole second is the unit — that would be a claim about Blender's insides, and these runs only watch it from outside.


## 2. When a `.blend` file links a node setup from another `.blend` file, what does the linking file record about the files that setup uses, and how are they reached?

**LABEL: documented**, for the core of it. Two parts are **inferred** and marked as such; the rest is **not in the sources** and listed under "Not covered".

*Ambiguity flagged:* "the files that setup uses" can be read two ways — (a) the source `.blend` the setup itself comes from, and (b) the other files the setup in turn needs (images, fonts, imported files, further `.blend` files). I answer both, because the question's phrase "the files" is plural and reading (b) alone would leave the answer incomplete. If only (b) was meant, sections 2.2 and 2.3 are the answer.

### 2.1 The source `.blend` itself: a Library data-block holding a path

Linking a node setup is done with File ‣ Link:

> "It's also possible to import node groups from a different blend-file using File ‣ Link/Append."
> — https://docs.blender.org/manual/en/4.5/interface/controls/nodes/groups.html, read 2026-07-30

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."
>
> "Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."
>
> "Relative Path — Reference the external blend-file using a relative path rather than an absolute one."

— https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30

What is recorded is a data-block of its own:

> "Library | ✗ | ✓ | References to an external blend-file. Access from the Outliner's Blender File view."

— data-block table, https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30. The same page states these types are "data-blocks types stored in blend-files". (The `✗` is the Link column, the `✓` the Pack column.)

> "class bpy.types.Library(ID)
>
> External .blend file from which data is linked
>
> filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"
>
> "parent
>
> Type:
>
> Library, (readonly)"
>
> "version
>
> Version of Blender the library .blend was saved with"

— https://docs.blender.org/api/4.5/bpy.types.Library.html, read 2026-07-30

Each linked data-block records which library it came from:

> "library — Library file the data-block is linked from. Type: Library, (readonly)"
>
> "is_library_indirect — Is this ID block linked indirectly"
>
> "name_full — Unique data-block ID name, including library one is any"

— https://docs.blender.org/api/4.5/bpy.types.ID.html, read 2026-07-30

Whether the recorded path is relative or absolute is a choice made at link time — the Relative Path option quoted above, `relative_path` on the operator ("Relative Path, Select the file relative to the blend file", `bpy.ops.wm.link`, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30), and `relative` on the Python loader ("When True the path is stored relative to the open blend file", `bpy.types.BlendDataLibraries.load`, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, read 2026-07-30). The manual explains what a relative path means:

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
>
> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

— https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30

**Appended, not linked:** even then a reference to the source file can be kept, in a different and weaker form:

> "class bpy.types.LibraryWeakReference(bpy_struct)
>
> Read-only external reference to a linked data-block and its library file
>
> filepath — Path to the library .blend file
>
> id_name — Full ID name in the library .blend file (including the two leading 'id type' chars)"

— https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html, read 2026-07-30, reached from `ID.library_weak_reference`, "Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)" (https://docs.blender.org/api/4.5/bpy.types.ID.html). The manual describes the same behaviour from the user's side under Append (Reuse Data): "Blender will keep track of where it originated" (https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30).

### 2.2 What the setup itself needs: indirectly linked data-blocks

The things the linked setup uses do not have to be asked for; they come with it, as indirect links.

> "import_info
>
> Various status info about an item after it has been imported
>
> INDIRECT_USAGE
> That item was added for an indirectly imported ID, as a dependency of another data-block."

— `bpy.types.BlendImportContextItem`, https://docs.blender.org/api/4.5/bpy.types.BlendImportContextItem.html, read 2026-07-30

> "APPEND_RECURSIVE
> Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."
>
> "USE_PLACEHOLDERS
> Generate a placeholder (empty ID) if not found in any library files."
>
> "MAKE_PATHS_RELATIVE
> Make paths of used library blendfiles relative to current blendfile."

— `bpy.types.BlendImportContext.options`, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html, read 2026-07-30

The manual says the same for append from the user's side:

> "Localize All — Also copy all indirectly linked data, instead of maintaining the links."

— https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30

and for writing a library out from Python:

> "Note
>
> Indirectly referenced data-blocks will be expanded and written too."

— `bpy.types.BlendDataLibraries.write`, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, read 2026-07-30

The phrase "indirect dependencies … coming from other libraries" shows a linked setup's own dependencies may themselves live in further `.blend` files, and that a chain is recorded — which is also what `Library.parent` (type `Library`) is for. **Inferred**, step: `Library.parent` is documented as existing and having type `Library`, but its meaning is not described anywhere in these sources; I am reading it together with the "other libraries" wording above.

### 2.3 How the non-`.blend` files are reached: paths resolve against the library, not the open file

This is the part that decides where Blender actually looks, and it is stated outright:

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
>
> >>> bpy.path.abspath(image.filepath, library=image.library)"

— Blender Python API, https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html, read 2026-07-30

The same rule is written into the two path helpers:

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the "//" prefix.
>
> library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

— https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30

> "library (bpy.types.Library | None) – The library this path is relative to."

— `bpy_extras.io_utils.path_reference`, https://docs.blender.org/api/4.5/bpy_extras.io_utils.html, read 2026-07-30

So for an image or a font used inside a linked node setup: the path string travels with the linked data-block (it is a property of that data-block — `Image.filepath`, `VectorFont.filepath`, both "blend relative // prefix supported"), and a `//` in it is measured from the library file's own folder, not from the folder of the file doing the linking.

**Inferred**, and worth stating plainly because the question asks what the *linking* file records: nothing in these sources says the linking file stores its own copy of those media paths. What it records is the library, and the identity of the data-blocks taken from it. The step: `ID.library` says a linked data-block belongs to a library and `filepath` is a property of the linked data-block; the resolution rule above measures it from the library. I found no source stating the media path is written into the linking file, and no source stating it is not.

### 2.4 When the source file is missing

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

— https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30

> "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"

— https://docs.blender.org/api/4.5/bpy.types.ID.html, read 2026-07-30

The library can be pointed at a different file after the fact ("Relocate Library … It allows to replace it by a different file", same manual page), and 4.5 added the same for a single linked data-block:

> "* Linked IDs can now also be relocated, if they are not a dependency of any other linked data"

— https://developer.blender.org/docs/release_notes/4.5/core/, read 2026-07-30

matching the manual's "Relocate Linked ID … This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."

### 2.5 One thing recorded locally, not in the library

When the linked setup contains a Bake node, the bake settings and the data-block references for the bake live on the local modifier, and there the library is named by a string:

> "lib_name — Used when the data block is not local to the current .blend file but is linked from some library"

— `bpy.types.NodesModifierDataBlock`, https://docs.blender.org/api/4.5/bpy.types.NodesModifierDataBlock.html, read 2026-07-30

and the bake entry points back into the linked tree:

> "node — Bake node or simulation output node that corresponds to this bake. This node may be deeply nested in the modifier node group. It can be none in some cases like missing linked data blocks."

— https://docs.blender.org/api/4.5/bpy.types.NodesModifierBake.html, read 2026-07-30

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer: which folder a `//` inside a linked setup is measured from, and what the linking file records, list entry 7; what packing a library carries and what it leaves behind, list entry 8; what is recorded when a library links onward, list entry 9; whether the library path is stored relative or absolute, list entry 10. Read this list as "the documentation does not say", not as "nobody knows".)*

- **Import node paths inside a linked setup.** An import node's Path is a socket value inside the node tree, not a data-block. The sources do not say whether a `//` in such a string is measured from the library file or from the open file. The resolution rule in 2.3 is written for data-block file paths (`image.filepath`) and I will not stretch it to a socket string. This is the sharpest gap in this answer.
- Whether the linking file writes a Library record for *indirect* libraries as well as direct ones. The manual says the Outliner's Blender File view lists linked data-blocks "along with the path of the blend-file they originate from", without distinguishing direct from indirect; `Library.parent` implies a chain. Neither settles what is written to the file.
- Whether relinking, reloading or relocating updates anything about the media files used by the setup. The manual pages describing those operations do not say — see "Where I looked" below.
- What happens if the library is found but a file *it* refers to is not. `is_missing` and the placeholder mechanism are documented for missing *libraries* only.
- Whether packing behaves differently for indirectly linked media. "Pack Linked Libraries — Mark all linked library files in the current blend-file as packed" (https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html) says nothing about what a packed library does with its own external files.
- Node group type is not distinguished when linking: "When appending node groups from another blend-file, Blender does not distinguish between types such as material or compositing groups." (https://docs.blender.org/manual/en/4.5/interface/controls/nodes/groups.html). The sources do not say what this means for a geometry node group specifically.

### Where I looked

- Read in full: `manual_4.5/files/linked_libraries/link_append.txt` and `index.txt`, `files/data_blocks.txt`, `files/blend/open_save.txt`, `files/blend/packed_data.txt`, `files/file_paths.txt`, `interface/controls/nodes/groups.txt`, `editors/preferences/file_paths.txt` (asset import methods), `editors/asset_browser.txt` (import methods and asset use), `files/asset_libraries/introduction.txt`.
- Read in full: `api_4.5/bpy.types.Library.txt`, `bpy.types.LibraryWeakReference.txt`, `bpy.types.BlendDataLibraries.txt`, `bpy.types.BlendImportContext.txt`, `bpy.types.BlendImportContextItem.txt`, `bpy.path.txt`, `info_gotchas_file_paths_and_encoding.txt`, `bpy.types.NodesModifierBake.txt`, `bpy.types.NodesModifierDataBlock.txt`; the library sections of `bpy.types.ID.txt`; `bpy.ops.wm.link` and `bpy.ops.wm.append` parameter lists.
- Searched the whole manual for "indirect" (only hits: node groups containing themselves, Cycles' Indirect Only, and the Append "Localize All" option), "Relative Path", "Make Paths Relative", "Broken Libraries", "Relocate".
- Searched the whole API documentation for "relative to the library", "library" together with "relative", "library_weak_reference", "is_library_indirect".
- Searched `release_notes_4.5/` for "librar" and "path"; read the Linking & Library Overrides and Path Templates sections of `core.md` and the path items of `python_api.md`.
- Cycles' own files: searched `cycles_4.5/` for "librar", "node_group", "filepath". Nothing about linking or library paths. Ruled out.
- The requirements page says nothing about linking. Ruled out.

### Outside the interface

Partly documented, and it holds.

Linking has a full Python route that does not need the interface: `bpy.data.libraries.load(filepath, link=False, relative=False, …)` with the worked example `with bpy.data.libraries.load(filepath, link=True) as (data_from, data_to)` (https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, read 2026-07-30), and `bpy.ops.wm.link(...)` with the same `relative_path` option (https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30). The `relative` parameter is documented as controlling what is *stored*, so what ends up in the file is the same kind of record either way — **inferred**, step: the parameter's own words are "the path is stored relative to the open blend file", which is a statement about storage rather than about the interface.

What the sources do not say: whether missing-library placeholder creation, or resolution of a library's own external files, behaves any differently when Blender is started without its window. The one background-mode note I found is about the dependency graph, quoted in answer 1, not about linking.

**Brought up to date by hand test, 2026-08-06 — observed.** All of it holds with no window. Linking, packing the library, reading what the linking file records, resolving a `//` path belonging to linked data, and creating placeholders for a library that cannot be found were every one of them done and read from a script in a Blender started with `-b`, on a machine with no display server at all. With the library gone, Blender printed a warning naming it, left a placeholder in its place and exited with code 0. See "Observed by hand test" below, list entries 7, 8, 9 and 10. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers, so this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so two things could be looked at there that no terminal run can settle. The Path field on a linked group's Import node is drawn **greyed out** in the window, in all four versions — while a script writing to that same socket is accepted, which the Linux runs showed. And the modifier for a linked group holding no Bake node shows no Bake panel at all, only a collapsed section headed Manage; that section could not be opened without a click, so what it holds is untested. What a linked group that *does* hold a Bake node gives the modifier was answered instead with no window at all. See "Observed by hand test on the Creator's Mac" below, list entries 7, 8, 9, 10, 15, 16 and 17.


### Operating system

The sources do not say that any of this differs by operating system. The requirements page names Blender 4.5 LTS support on Windows, macOS and Linux (https://www.blender.org/download/requirements/, read 2026-08-01) and says nothing about linking or paths. One documented OS-flavoured wrinkle about relative paths exists in these sources, but it is about the OBJ *exporter's* Path Mode, not about library paths — "Relative: Uses relative paths in every case (except when on a different drive on Windows)." (https://docs.blender.org/manual/en/4.5/files/import_export/obj.html, read 2026-07-30). I record it only so it is clear I saw it and did not use it.

**Brought up to date by hand test, 2026-08-06 — observed.** Everything below was run on Linux only, where a path is matched letter for letter — see answer 1's observed section, list entry 11. Nothing has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed.** The letter-case test was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — as well, and came out the other way: there the upper-case forms found the file. See answer 1's Mac observed section, list entry 11. So a path is matched letter for letter on Linux and is not on macOS. Everything else here was run on both machines and agreed on both. No Windows machine has run it, so Windows is untested.

### Observed by hand test — 2026-08-06

**LABEL: observed.**

Same machine, same date, same Blender and the same settings folder as the observed section under answer 1: **a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and no display server at all**, `/opt/blender/4.5/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, started with `BLENDER_USER_RESOURCES=/root/node-setups/config/4.5`. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`. **Every one of these was run again on the Creator's Mac on 2026-08-07, and all of them agreed; that block follows this one.** The two machines' results are kept apart and never merged.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7)

Two folders were built. `lib/` held `lib.blend`, a `model.obj` with **three** vertices and a `tex.png`; inside `lib.blend`, a node group `LibSetup` with an Import OBJ node whose Path was `//model.obj` and an Image Texture node whose image was `//tex.png`. `shot/` held `use.blend`, which linked `LibSetup` and put it on an object. Then `use.blend` was opened four times: with no `model.obj` beside it; with a **different**, four-vertex `model.obj` beside it; with `lib/model.obj` moved away and the four-vertex copy still beside `use.blend`; and with the whole `lib` folder moved away. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/root/node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x76c31c26f020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x76c31c26f020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/tex.png', '/root/node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x76c31c302e20> -> {'//tex.png'}
    <bpy_struct, Library("lib.blend") at 0x76c31c26f020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x76c31c302e20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x76c31c3eb6a0> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x76c31c26f020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/root/node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x78f9f8478020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x78f9f8478020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/tex.png', '/root/node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x78f9f850ae20> -> {'//tex.png'}
    <bpy_struct, Library("lib.blend") at 0x78f9f8478020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x78f9f850ae20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x78f9f85f26a0> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x78f9f8478020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/root/node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/model.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/model.obj'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x75dafee78020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x75dafee78020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/tex.png', '/root/node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x75dafef0ae20> -> {'//tex.png'}
    <bpy_struct, Library("lib.blend") at 0x75dafee78020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x75dafef0ae20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x75dafeff36a0> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x75dafee78020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/root/node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Warning: Unable to open '/root/node-setups/work/06/4.5/lib/lib.blend': No such file or directory
Info: Cannot find lib '/root/node-setups/work/06/4.5/lib/lib.blend'
Info: LIB: NodeTree: 'LibSetup' missing from '/root/node-setups/work/06/4.5/lib/lib.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7a8bd2a67020> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x7a8bd2a67020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x7a8bd2ac8e08> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7a8bd2a67020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/4.5/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 4.5 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8)

`use.blend` was opened, `bpy.ops.file.pack_libraries()` run, and the result saved as `packed_use.blend`; that file was then opened three times, with everything in place, with `lib.blend` alone moved away, and with the whole `lib` folder moved away. In full:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/root/node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0x7e47da499020>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x7e47da499020> is_linked_packed: no such property
Info: Saved as "packed_use.blend"
saved: /root/node-setups/work/06/4.5/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/packed_use.blend"
Info: Read packed library: '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x70d1f606e020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x70d1f606e020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/4.5/lib/tex.png']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x70d1f6152e20> -> {'//tex.png'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x70d1f6152e20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x70d1f6256a20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/packed_use.blend"
Info: Read packed library: '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x75cb2287e020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x75cb2287e020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/4.5/lib/tex.png']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x75cb22962e20> -> {'//tex.png'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x75cb22962e20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x75cb22a56a20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/06/4.5/shot/packed_use.blend"
Info: Read packed library: '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/model.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/model.obj'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/06/4.5/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x72973a476020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x72973a476020> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /root/node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/root/node-setups/work/06/4.5/lib/tex.png']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x72973a55ae20> -> {'//tex.png'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x72973a55ae20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x72973a64ea20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/root/node-setups/work/06/4.5/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/root/node-setups/work/06/4.5/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 4.5 ===
```

#### What a linking file records when its library links onward (list entry 9)

Three folders: `deep/deep.blend` with a node group `DeepSetup` reading `//model.obj` and a **three**-vertex `model.obj` beside it; `mid/mid.blend` linking `DeepSetup`, wrapping it in a local group `MidSetup`, with a **four**-vertex decoy `model.obj` beside it; `top/top.blend` linking `MidSetup`, with a **five**-vertex decoy `model.obj` beside it. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/09/4.5/top/top.blend"
Info: Read library: '/root/node-setups/work/09/4.5/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Info: Read library: '/root/node-setups/work/09/4.5/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/4.5/mid/mid.blend'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/09/4.5/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/4.5/deep/deep.blend', '/root/node-setups/work/09/4.5/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x72f1e0ced5a0> -> {'//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x72f1e0bf26a0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x72f1e0af0b08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x72f1e0a78020> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/node-setups/work/09/4.5/top/top.blend"
Info: Read library: '/root/node-setups/work/09/4.5/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Info: Read library: '/root/node-setups/work/09/4.5/deep/deep.blend', '//../deep/deep.blend', parent '/root/node-setups/work/09/4.5/mid/mid.blend'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/root/node-setups/work/09/4.5/deep/model.obj'.
Error: OBJ Import: Cannot open file '/root/node-setups/work/09/4.5/deep/model.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /root/node-setups/work/09/4.5/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/node-setups/work/09/4.5/deep/deep.blend', '/root/node-setups/work/09/4.5/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x7f5dc30f55a0> -> {'//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x7f5dc2ffa6a0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x7f5dc2ef8b08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x7f5dc2e80020> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 4.5 ===
```

#### Whether the library path is recorded relative or absolute, and what Blender ships (list entry 10)

In full:

```
BLENDER: 4.5.12 LTS WORKDIR: /root/node-setups/work/10/4.5
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/root/node-setups/work/10/4.5/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/root/node-setups/work/10/4.5/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: no such attribute
--- the node assets Blender ships with ---
   assets folder: /opt/blender/4.5/4.5/datafiles/assets exists: True
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/geometry_nodes/smooth_by_angle.blend
   shipped .blend: /opt/blender/4.5/4.5/datafiles/assets/geometry_nodes/procedural_hair_node_assets.blend
DONE
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)

Blender quit
```

---

### Observed by hand test on the Creator's Mac — 2026-08-07

**LABEL: observed.**

Same machine, same date, same Blenders and the same settings folders as the observed Mac section under answer 1: **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**, `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)`, started with `BLENDER_USER_RESOURCES` pointing at `/Users/artist/Documents/Smoozy hand tests/Node setups/config/4.5`. Working folders were fresh, under `/tmp/smoozy-node-setups/work/`. Two other topics were running Blender on the same Mac while these ran; nothing here measures time or memory. The set-ups and commands are in `Audits/Blender Research/Node setups/Hand tests.md`.

**Every one of these agreed with Linux**, and the two machines' results are kept separate rather than merged. Two further tests were run that the Linux half did not reach: the window tests, list entries 14 and 15, which need a screen; and two the Mac half showed the list had missed, list entries 16 and 17.


#### Which folder a `//` inside a linked node group is measured from, and what the linking file records (list entry 7) — Mac

Agreed with Linux: the `//` was measured from the **library** file, not from the linking file. The four opens, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x8891a4620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x8891a4620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x884bece20> -> {'//tex.png'}
    <bpy_struct, Library("lib.blend") at 0x8891a4620> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x884bece20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x889710a20> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x8891a4620> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x9c38a5220> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x9c38a5220> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x9c38c2320> -> {'//tex.png'}
    <bpy_struct, Library("lib.blend") at 0x9c38a5220> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x9c38c2320> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x9c3f0dba0> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x9c38a5220> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/model.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/model.obj'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7fb923020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7fb923020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7fb95a320> -> {'//tex.png'}
    <bpy_struct, Library("lib.blend") at 0x7fb923020> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7fb95a320> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7fb992d20> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7fb923020> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Warning: Unable to open '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend': No such file or directory
Info: Cannot find lib '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend'
Info: LIB: NodeTree: 'LibSetup' missing from '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7910e9220> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
   local=True : ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x7910e9220> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x791070a08> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x7910e9220> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
=== END 4.5 ===
```

#### What Pack Linked Libraries carries, and what it leaves behind (list entry 8) — Mac

Agreed with Linux. The whole run, uncut:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0xb3f8bd820>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xb3f8bd820> is_linked_packed: no such property
Info: Saved as "packed_use.blend"
saved: /tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend"
Info: Read packed library: '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xc2b4dc020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xc2b4dc020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xc2b4b8e20> -> {'//tex.png'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xc2b4b8e20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xc2b4d8ca0> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend"
Info: Read packed library: '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x98eea3020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x98eea3020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x98eec2a20> -> {'//tex.png'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x98eec2a20> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x98eef82a0> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend"
Info: Read packed library: '//../lib/lib.blend', parent '<direct>'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/model.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/model.obj'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/06/4.5/lib/written-from-linking-file.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/06/4.5/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None version=(4, 5, 92)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x7da21c620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x7da21c620> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /tmp/smoozy-node-setups/work/06/4.5/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png']
   local=True : []
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x7d61a2a20> -> {'//tex.png'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x7d61a2a20> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x7da22afa0> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/tmp/smoozy-node-setups/work/06/4.5/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== END 4.5 ===
```

#### What a linking file records when its library links onward (list entry 9) — Mac

Agreed with Linux. The two opens of `top.blend`, uncut — the three steps that only build the files are left out:

```
=== STEP 4: open top/top.blend and report ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/09/4.5/top/top.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/09/4.5/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Info: Read library: '/tmp/smoozy-node-setups/work/09/4.5/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/4.5/mid/mid.blend'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/09/4.5/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/4.5/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/4.5/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0x75a532820> -> {'//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0x75a4f0ca0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0x755441408> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0x75970d220> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/09/4.5/top/top.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/09/4.5/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Info: Read library: '/tmp/smoozy-node-setups/work/09/4.5/deep/deep.blend', '//../deep/deep.blend', parent '/tmp/smoozy-node-setups/work/09/4.5/mid/mid.blend'
ERROR (io.obj): source/blender/io/wavefront_obj/importer/obj_import_file_reader.cc:432 OBJParser: Cannot read from OBJ file:'/tmp/smoozy-node-setups/work/09/4.5/deep/model.obj'.
Error: OBJ Import: Cannot open file '/tmp/smoozy-node-setups/work/09/4.5/deep/model.obj'
BLENDER: 4.5.12 LTS OPEN FILE: /tmp/smoozy-node-setups/work/09/4.5/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/tmp/smoozy-node-setups/work/09/4.5/deep/deep.blend', '/tmp/smoozy-node-setups/work/09/4.5/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0xc45a70a20> -> {'//../deep/deep.blend'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0xc45a1ed20> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0xc41370a08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0xc459bb020> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== END 4.5 ===
```

#### Whether the library path is recorded relative or absolute (list entry 10) — Mac

Agreed with Linux. The whole run, uncut. **The last part of it, which looks for the node files Blender ships, found nothing on this Mac** — the check looks beside the Blender program, which is where those files sit on Linux and is not where they sit on macOS. That is a fault of the check, not a fact about Blender, and it is recorded as it stands; list entry 16 below asks the same question in a way that works here.

```
BLENDER: 4.5.12 LTS WORKDIR: /tmp/smoozy-node-setups/work/10/4.5
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
   stored library filepath: '//../lib/lib.blend'
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/4.5/lib/lib.blend'
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
   stored library filepath: '/tmp/smoozy-node-setups/work/10/4.5/lib/lib.blend'
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
   libraries now: [('lib.blend', '//../lib/lib.blend')]
--- what the libraries attribute of load() offers about the library's own libraries ---
   df.libraries: no such attribute
--- the node assets Blender ships with ---
   assets folder: /Applications/Blender 4.5.12.app/Contents/MacOS/4.5/datafiles/assets exists: False
DONE
```

#### Which node `.blend` files each Blender ships (list entry 16) — Mac

The shipped-file half of list entry 10, asked again by way of Blender's own answer to where its files are. The whole run, uncut:

```
BLENDER: 4.5.12 LTS
binary path: /Applications/Blender 4.5.12.app/Contents/MacOS/Blender
the folder the earlier check looked in: /Applications/Blender 4.5.12.app/Contents/MacOS/4.5/datafiles/assets
   exists: False
bpy.utils.resource_path('LOCAL'): /Applications/Blender 4.5.12.app/Contents/Resources/4.5
assets folder: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets exists: True
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/geometry_nodes/procedural_hair_node_assets.blend
   shipped .blend: /Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/geometry_nodes/smooth_by_angle.blend
count of shipped .blend files under the assets folder: 11
DONE
```

#### What a linked group holding a Bake node gives the modifier, and where that bake goes (list entry 17) — Mac

Built because the linked group in list entry 7 holds no Bake node, so list entry 15 found the modifier had no bake to show. A library file held a group of Group Input, Bake, Group Output; a second file linked that group onto a cube; then the second file was opened, read and baked. The report and the bake, uncut — the two steps that only build the files are left out:

```
=== STEP 3: open shot/bakeuse.blend and report, then bake ===
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/tmp/smoozy-node-setups/work/17/4.5/shot/bakeuse.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/17/4.5/lib/bakelib.blend', '//../lib/bakelib.blend', parent '<direct>'
BLENDER: 4.5.12 LTS STEP: report WORKDIR: /tmp/smoozy-node-setups/work/17/4.5
node group: BakeLibSetup library: //../lib/bakelib.blend
modifier bake_target: PACKED
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=1229320680 bake_target=INHERIT directory=''
files beside the .blend before baking: ['bakeuse.blend', 'bakeuse.blend1']
Info: Bake directory of object Target, modifier GN is empty, setting default path
bake operator returned: {'FINISHED'}
modifier bake_directory after: '//blendcache_bakeuse/Target_GN'
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/1229320680/meta/00001_00000.json
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/1229320680/blobs/00001_00000.blob
DONE
```

The bake went beside the **linking** file, not beside the library: `blendcache_bakeuse` sits in the `shot` folder, next to `bakeuse.blend`, and the modifier's own Bake Path filled itself in as `//blendcache_bakeuse/Target_GN`, measured from the linking file. The library folder gained nothing.


#### Whether a linked group's Path field can be typed into in a window, and what the modifier shows (list entry 15) — Mac only

The Linux machine has no display server, so this cannot be reached there at all. Blender was started **with** its window, opening the `use.blend` that list entry 7 built, and driven by a script that showed the linked group in the geometry node editor, took Blender's own screenshot, then switched the properties editor to the modifier tab and took a second one, before quitting. No mouse was moved and nothing was clicked.

The first form of the script hit the same trouble as list entry 14 — every node in the group had been made at the same spot, so only the topmost was visible. It is recorded as it came back and settles nothing. The second form spread the nodes apart first. Blender printed:

```
node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xb8d50b020> indirect: False missing: False
moving the linked group's nodes apart was accepted
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (400.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (800.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (1200.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (1600.0, 0.0)
    Image Texture GeometryNodeImageTexture (2000.0, 0.0)
Warning: 2 × Draw window and swap: 39.6209 ms, average: 19.81043816 ms
view_all returned: {'FINISHED'}
Warning: 2 × Draw window and swap: 15.8269 ms, average: 7.91347027 ms
screenshot written to /tmp/smoozy-node-setups/work/15/4.5/linked-nodes.png
Path socket on the linked Import OBJ node holds: '//model.obj'
Path socket: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
the node group's own library_weak_reference: None
modifier: GN type: NODES
   bake_target: PACKED bake_directory: ''
   number of bake entries: 0
Warning: 2 × Draw window and swap: 11.2040 ms, average: 5.60200214 ms
screenshot written to /tmp/smoozy-node-setups/work/15/4.5/modifier-panel2.png
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

and, on the run that tried to open the Manage section:

```
modifier: GN bake entries: 0 bake_target: PACKED bake_directory: ''
ui_expand_flag before: no such property
setting ui_expand_flag raised: AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'
Warning: 3 × Draw window and swap: 139.3731 ms, average: 46.45768801 ms
screenshot written to /tmp/smoozy-node-setups/work/15/4.5/modifier-expanded.png
Read blend: "/tmp/smoozy-node-setups/work/06/4.5/shot/use.blend"
Info: Read library: '/tmp/smoozy-node-setups/work/06/4.5/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"

Blender quit
```

**The Path field on the linked group's Import OBJ node is drawn greyed out.** Put beside the screenshot from list entry 14, where the same node in a **local** group draws `//model.obj` in bright white with a bright folder button, the linked one draws the same text and the same button dimmed. All four versions drew it dimmed. That is worth setting against what list entry 7 found from a script: writing a new value into that same socket from Python **was** accepted. So the window and the script route differ — the window will not let the value be typed, and a script will.

The whole of the right-hand Group panel was drawn greyed out in the same way, in all four versions — the group's Name, its Node Width and the plus and minus buttons beside its sockets.

**The modifier's Bake panel was not there for this group.** The modifier showed only its node-group field, holding `LibSetup` and greyed out, and one collapsed section headed **Manage**. Blender reported `number of bake entries: 0` for the modifier, which is what a group holding no Bake node gives it — the group built for list entry 7 has none. The Manage section could not be opened from a script: `NodesModifier` has no property for the open-and-shut state of its sections, and the attempt printed `AttributeError 'NodesModifier' object has no attribute 'ui_expand_flag'`. Opening it needs a click, which is not allowed here, so **what the Manage section holds for a linked group is untested**. What a linked group with a Bake node in it gives the modifier is answered instead by list entry 17 above, without a window.

The screenshots are kept at `/tmp/smoozy-node-setups/work/15/4.5/` on that machine. This settles the drawing for **macOS only**; there is no Linux machine with a screen, and no Windows machine ran it.


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

2026-08-06 — Opus 5 (Claude Code) — Node setups, Blender 4.5

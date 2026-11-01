# Node setups — Blender 4.5 — second pass

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own files — all read from the snapshot taken on **2026-07-30**. Blender's requirements page, read from the file taken on **2026-08-01**. I also opened one picture from the manual's `_images` folder (see Question 1).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Note on labels: each question carries several separate findings, and a single label for the whole answer would hide which parts are read and which are reasoned. So each finding below carries its own label, and the question's LABEL line names the mix.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**LABEL: mixed — items 1a, 1b, 1c and 1e are documented; items 1d and 1f are inferred (the step is written out in each).**

### 1a. Six kinds of file named directly on a node, through a Path input — documented

Blender 4.5 has a family of import nodes in the geometry node editor. Each one takes a path to a file on disk.

Exact words:

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

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/index.html — read 2026-07-30.

The individual pages each describe the Path input:

> "Path
>
> The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/csv.html

> "Path
>
> The path to the .obj file to be imported."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/obj.html

> "Path
>
> The path to the .ply file to be imported."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/ply.html

> "Path
>
> The path to the .stl file to be imported."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/stl.html

> "Path
>
> The path to the txt file."

and, on the same page:

> "Currently, only files with the .txt extension are supported."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/txt.html

> "Path
>
> The path to the VDB file to be imported."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/vdb.html

All read 2026-07-30.

The release notes state these nodes are new in this version:

> "### Import Nodes
>
> Several nodes for importing external data have been added.
>
> * **Import PLY** (blender/blender@f6e6f2b2d222)
> * **Import OBJ** (blender/blender@f6e6f2b2d222)
> * **Import CSV** (blender/blender@1a62fdc82a62, blender/blender@ac2cd6c1effd)
> * **Import STL** (blender/blender@d1455c413893)
> * **Import Text** (blender/blender@9287bb759b02)
> * **Import VDB** (blender/blender@136331984458)"

Source: Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/geometry_nodes/ — read 2026-07-30.

**From a picture.** The manual shows the Import OBJ node at `_images/node-types_GeometryNodeImportOBJ.webp`, referenced from the Import OBJ page. What is visible: a node headed "Import OBJ", an output socket labelled "Instances", and one input row labelled "Path" drawn as an empty text field with a small folder icon at its right-hand end. I also looked at `_images/node-types_GeometryNodeImportCSV.webp`, which shows the same "Path" field with folder icon plus a second input row "Delimi…" holding a comma. Label for what these pictures show: **documented** (they are part of the manual). I take from them only that the Path input is a text field with a file-browser button beside it; I do not take from them what the button opens or what the field accepts.

### 1b. Image files, through an Image data-block — documented

Three geometry nodes deal in images.

> "The Image node provides access to a image file which allows you to conveniently enter and switch images for multiple nodes in the tree."

and, on the same page:

> "Image Data-Block
>
> The data-block selector to select an existing image or open an image from the file browser."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/constant/image.html — read 2026-07-30.

> "The Image Texture node is used to add an image file as a texture. The image data is sampled with the input Vector and outputs a Color and Alpha value."

and:

> "Frame
>
> If the Image supports animation, the frame can be set here. This can be keyframed so that the image changes between frames."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/texture/image.html — read 2026-07-30.

> "The Image Info node gets information from image and animation."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/scene/image_info.html — read 2026-07-30.

What an Image data-block points at:

> "filepath
>
> Image/Movie file name
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

and:

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

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Image.html — read 2026-07-30.

So a geometry node setup can make the file point at an image file. **Label corrected by the citation check, 2026-08-07: that it can also reach a numbered sequence of image files, a movie file, or a set of UDIM tile files is *inferred*, not documented.** The step: the `source` list quoted above belongs to the Image data-block in general, and nothing quoted here says that the Image socket on these three geometry nodes takes an Image of every kind on that list. The first-pass document for 4.5 labels the same claim inferred; the two now agree.

The API confirms the node holds a pointer to the data-block, not a path:

> "image
>
> Type:
>
> Image"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.GeometryNodeInputImage.html — read 2026-07-30.

### 1c. A font file, through the String to Curves node — documented

The manual gives the property:

> "Font
>
> Font glyph used to generate the curve."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/utilities/text/string_to_curves.html — read 2026-07-30.

The API says what that property holds:

> "font
>
> Font of the text. Falls back to the UI font by default.
>
> Type:
>
> VectorFont"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.GeometryNodeStringToCurves.html — read 2026-07-30.

And what a VectorFont points at:

> "VectorFont(ID)
>
> …
>
> Vector font for Text objects
>
> filepath
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.VectorFont.html — read 2026-07-30.

So a font file on disk. Note the words "Falls back to the UI font by default" — a setup that never picks a font need not point at any font file.

### 1d. Other `.blend` files, when the things a node points at are linked — inferred

The step I took: the manual documents that several geometry nodes hold references to objects, collections, materials and node groups; and, separately, that any data-block can be linked from another blend-file, in which case Blender records the path of that file. Put together, a geometry node setup that references a linked object, collection, material, image, font or node group makes the file point at the blend-file that data-block came from. No single page says this about geometry nodes; that joining step is mine.

The node side:

> "The Object input node outputs a single object."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/constant/object.html

> "The Collection input node outputs a single collection."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/constant/collection.html

> "The Material input node outputs a single material."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/constant/material.html

> "The Object Info node gets information from objects."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/scene/object_info.html

> "The Collection Info node gets information from collections."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/scene/collection_info.html

> "Node Groups
>
> This section lists all the node groups, both those in the current blend-file and those Linked or Appended from another blend-file."

https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/group.html

All Blender manual, read 2026-07-30.

The linking side:

> "Library(ID)
>
> …
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

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.

### 1e. Bake files on disk, written by Bake nodes and Simulation zones — documented

> "Bake Target
>
> Specifies where baked data should be stored. This can be overridden for individual bakes.
>
> Packed:
>
> The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk:
>
> The baked data is stored in a separate directory on disk.
>
> Bake Path
>
> Location on disk where the baked data for Simulation Zones and Bake Nodes are stored."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/modifiers/generate/geometry_nodes.html — read 2026-07-30.

The same choice exists per bake node:

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

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-07-30.

On the format, and on the requirement that the file be saved first:

> "The data format used to store geometry data is not considered to be an import/export format. Volume objects, however, are saved using the OpenVDB file format which can be used interoperably."

and:

> "Data is not stored to disk until the Bake operation is ran."

and:

> "Blend-files must be saved to a disk before data can be baked."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/baking.html — read 2026-07-30.

The paths in the API:

> "bake_directory
>
> Location on disk where the bake data is stored
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

and:

> "bake_target
>
> Where to store the baked data
>
> PACKED Packed – Pack the baked data into the .blend file.
>
> DISK Disk – Store the baked data in a directory on disk.
>
> Type:
>
> enum in ['PACKED', 'DISK'], default 'PACKED'"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.NodesModifier.html — read 2026-07-30.

Per-bake, the same:

> "directory
>
> Location on disk where the bake data is stored
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.NodesModifierBake.html — read 2026-07-30.

Note the default: `bake_target` is `'PACKED'` by default, so by default a bake makes no separate file.

### 1f. Files reached through a material the setup assigns — inferred

The step I took: a geometry node setup can put a material on geometry (the Material node, and the material nodes listed in the manual's geometry-nodes material section); a material's own shader nodes can use image files. So a geometry node setup can make the file point, at one remove, at whatever image files that material uses. The sources do not join these two things anywhere; the joining is mine. *(Narrowed by the citation check, 2026-08-07: of the two halves being joined, only the second is quoted below. That a geometry node setup can put a material on geometry is asserted here without a quotation — the Material node and the geometry-nodes material section are named but not quoted. What the one quotation carries is that a shader Image Texture node holds an Image data-block; the rest of the step rests on the reader's knowledge of the node list.)*

The material side:

> "Image
>
> Image data-block to use."

Source: Blender manual, Image Texture Node (shader), https://docs.blender.org/manual/en/4.5/render/shader_nodes/textures/image.html — read 2026-07-30.

### What I could not settle
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

- **Whether an import node's Path accepts a blend-relative `//` path.** The API documents a socket type whose value does carry that note:

  > "NodeSocketStringFilePath(NodeSocketStandard)
  >
  > …
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

  Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.NodeSocketStringFilePath.html — read 2026-07-30.

  The plain string socket does not carry that note:

  > "default_value
  >
  > Input value used for unconnected socket
  >
  > Type:
  >
  > string, default "", (never None)"

  Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.NodeSocketString.html — read 2026-07-30.

  **Not in the sources:** nothing I found says which nodes use the file-path socket. The import node API pages list no sockets at all, and the manual pages call the input only "Path". So I cannot say the import nodes use it. Hand test 1 below settles it.

- **Whether these paths take part in Blender's ordinary path housekeeping** — Make Paths Relative, Make Paths Absolute, Report Missing Files, Find Missing Files, Pack Resources. **Not in the sources.** The operators are described in whole-file terms only:

  > "Make Paths Relative
  >
  > Make all paths to external files relative to the current blend-file."
  >
  > "Report Missing Files
  >
  > This option is useful to check if there are links to unpacked files that no longer exist."

  Source: Blender manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30. Nothing names geometry node import paths as included or excluded. Hand tests 2 and 3 settle it.

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

This answer names what a setup can point at. It does not settle: when each file is read (on evaluation, on load, once, or every frame); whether a missing file raises a node warning, an error, or silence; whether any of these files can be packed into the `.blend` (except bakes, where packing is documented); what the bake path defaults to when it is left empty; or whether an import node's path can be driven from a String socket coming from elsewhere in the tree rather than typed on the node. It also does not settle whether the list is complete — I looked at every geometry node page in the manual and every `GeometryNode*` page in the API for data-block pointers, but a node that reaches a file by some route the documentation does not describe would not show up in either.

### Outside the interface

Partly answered by the sources. Two of the routes above are described as interface actions and cannot exist when Blender is driven from outside: dragging a file into the node editor to create an import node ("Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor", import index page), and dragging a material into the editor to create a Material node. The nodes and paths themselves are ordinary data with ordinary API properties (`bake_directory`, `bake_target`, `font`, `image`), so they exist regardless of how Blender is started.

The manual documents that in background mode Blender does evaluate the scene when a file is loaded:

> "--disable-depsgraph-on-file-load
>
> Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. The word "systematically" is Blender's; the option exists to switch that off.

**Not in the sources** for the rest: whether import nodes read their files, and whether disk bakes are read, when Blender runs with `-b`. I searched the command-line arguments page for "bake", "geometry", "node" and "import" and found no geometry-node bake option; there is no documented command-line switch for baking geometry nodes.

**Brought up to date by hand test, 2026-08-06 — observed.** A Blender started with no window does read an import node's file when it opens the `.blend`, and does read it again at render time; the rendered picture was the same with `--disable-depsgraph-on-file-load` in force as without it. Every route in this answer was built, set and read from a script in a Blender with no window at all, on a machine with no display server, so nothing here needs a person clicking. The one exception is dragging a file into the node editor to make an import node: no script can make a real drag happen, so it cannot be tested at all and sits in the "Cannot be run" section of `Audits/Blender Research/Node setups/Hand tests.md`. See "Observed by hand test" below, list entries 4 and 12. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers: a Blender started with no window reads an import node's file when it opens the `.blend`, and again at render time, and the picture was the same with `--disable-depsgraph-on-file-load` in force as without it. So this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so one thing could also be looked at there that no terminal run can settle: the Path row is drawn as a text field holding the path, with a folder button at its right-hand end and a socket dot on the node's left edge level with it, and the value it shows is the one a script reads back. See "Observed by hand test on the Creator's Mac" below, list entries 4, 12 and 14. Dragging a file into the node editor to make an import node still cannot be tested at all, on either machine.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back was built, changed and read entirely from a script in a Blender with no window at all, on a machine with no display server, so nothing in it needs a person clicking. It also turns up something anyone driving Blender from outside should know: asking for the evaluated geometry again does not by itself make an import node read its file again. The setup has to be marked for a fresh run, **and** the file's modification time has to have changed by a whole second. A script that writes a file and immediately asks for the geometry can therefore be handed the old contents, with nothing said about it. Observed on Linux in all four versions; untested on macOS, because the rented machine was given up before the Mac half could be run, and Windows was not tried at all. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Operating system

**Not in the sources** as far as these nodes are concerned — no page about any of the nodes above names an operating system. The requirements page says "Blender is cross-platform, it runs on every major operating system" (https://www.blender.org/download/requirements/, read 2026-08-01) and names Blender 4.5 LTS under Windows, macOS and Linux headings.

Two general path facts in the API do differ by system and would apply to any path:

> "bpy.path.native_pathsep(path)
>
> Replace the path separator with the systems native os.sep."

> "bpy.path.resolve_ncase(path)
>
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.path.html — read 2026-07-30. These say separators and letter case behave differently on different systems; they do not say anything specific about geometry nodes.

**Brought up to date by hand test, 2026-08-06 — observed.** On Linux a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, and an image path behaves the same way — see "Observed by hand test" below, list entry 11. That is the one place these tests found an operating system could make a difference, and it has been settled for Linux only. Nothing here has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed. This is the one place the two machines disagreed.** The same test on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — came out the other way: `//MODEL.OBJ` and `//Model.Obj` both found `model.obj`, an image path in upper case loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back instead of correcting it to the file that exists. So **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. The two results are kept apart and never merged into one. Windows is still untested, because no Windows machine has run it. Everything else in this answer was run on both machines and agreed on both. See "Observed by hand test on the Creator's Mac" below, list entry 11.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back found a second place where an operating system could make a difference, and it could only be run on one of them. Whether an import node reads its file again turns on the file's modification time, and on the *whole second* of it: a change inside the same whole second is not noticed, a change that crosses one is, in either direction. That was settled on the rented Linux machine's ordinary file system, in all four versions, across eighty runs with no exception. Whether macOS keeps and compares that stamp the same way is **untested** — the rented machine was given up before the Mac half could be run — and no Windows machine has run it. So this holds for Linux and is untested on both of the others. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Where I looked

- Listed and read the whole geometry-nodes section of the manual: `modeling/geometry_nodes/` — every folder (attribute, curve, geometry, grease_pencil, hair, input, instances, material, mesh, normals, output, point, simulation, texture, utilities, volume) and the top-level pages (baking, fields, gizmos, group, index, inspection, instances, introduction, tools, attributes_reference).
- Read in full: all six import node pages plus their index; Image node; Image Texture node; Image Info node; String to Curves; Object node; Collection node; Material node; Object Info; Collection Info; Baking; Bake node; Geometry Nodes modifier.
- Searched the geometry-nodes section for: "data-block", "datablock", "dependenc", "relative", "pack", "missing", "absolute path", "//". Hits only as reported above; "missing" and "absolute path" returned nothing at all in that section.
- Searched the whole manual for: "blendcache", "bake_directory", "Bake Directory", "Custom Path", "Bake Path", "Relative Paths", "Make Paths Relative", "Report Missing Files", "Find Missing Files", "relative to the", "backslash", "drive letter", "case sensitive".
- Searched the whole API documentation for: "FILE_PATH", "NodeSocketStringFilePath", "StringFilePath", "indirect". Listed every `bpy.types.GeometryNode*.txt` and `bpy.types.FunctionNode*.txt` page and searched them for lines naming a data-block type (Image, Object, Collection, Material, VectorFont, Text, Sound, MovieClip, Volume, NodeTree) — the only hits were Collection, Image, Object, Material (the four input nodes), VectorFont (String to Curves), and NodeTree (group nodes). This is what makes me reasonably confident the list of data-block routes is complete, though see "Not covered".
- Read the API pages for Image, VectorFont, Library, NodesModifier, NodesModifierBake, NodesModifierDataBlock, GeometryNodeInputImage, GeometryNodeStringToCurves, GeometryNodeImportOBJ, NodeSocketImage, NodeSocketObject, NodeSocketCollection, NodeSocketMaterial, NodeSocketString, NodeSocketStringFilePath, NodeTreeInterfaceSocketString, NodeTreeInterfaceSocketStringFilePath.
- Read the 4.5 release notes for geometry nodes, core and pipeline/IO.
- Searched Cycles' own files for "geometry_node", "filepath", "node_group", "library" — nothing about geometry nodes; Cycles' Python files deal with OSL scripts and rendering, not geometry node trees. Ruled out as a source for this question.
- Ruled out the requirements page: it is about hardware and operating systems and says nothing about nodes.

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

**Reading the question.** I read "the files that setup uses" as the files the linked setup depends on — the images, fonts, import paths and so on inside it — and the question as asking what the *linking* file writes down about them. A second reading is possible: "the files that setup uses" could mean only the one file the setup itself lives in. I answer the first reading, and the second reading is covered by the first paragraph of 2a below.

**LABEL: mixed — 2a, 2b, 2d and 2e are documented; 2c is documented for the general case and inferred where it is applied to node setups; 2f is observed.**

### 2a. It records the library file and the name — documented

The linking file gains a Library data-block naming the other `.blend`:

> "Library(ID)
>
> …
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

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30.

Every linked data-block carries a pointer to that library, and its name is only unique within it:

> "library
>
> Library file the data-block is linked from
>
> Type:
>
> Library, (readonly)"

and:

> "name
>
> Unique data-block ID name (within a same type and library)"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30.

The manual says the same from the user's side:

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."

and:

> "Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

That path may be relative or absolute, chosen at link time:

> "Relative Path
>
> Reference the external blend-file using a relative path rather than an absolute one."

Same page, read 2026-07-30. The preference that sets the default:

> "Default To – Relative Paths
>
> Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html — read 2026-07-30.

### 2b. The files inside the setup arrive as further linked data-blocks from the same library, not as paths of the linking file's own — documented

The API describes a link or append operation as producing items, some of which are there only because something else needed them:

> "INDIRECT_USAGE
>
> That item was added for an indirectly imported ID, as a dependency of another data-block."

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendImportContextItem.html — read 2026-07-30.

Those dependencies are marked as indirect on the data-block itself:

> "is_library_indirect
>
> Is this ID block linked indirectly
>
> Type:
>
> boolean, default False, (readonly)"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30.

And they stay linked unless you ask otherwise. The append option:

> "Localize All
>
> Also copy all indirectly linked data, instead of maintaining the links."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

The same option in the API, with a detail the manual leaves out:

> "APPEND_RECURSIVE
>
> Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30.

From the Asset Browser side, appending is described as copying dependencies too:

> "The asset and all its dependencies will be Appended (copied) into the current file."

whereas linking is not:

> "The asset will be linked to the current blend-file, and thus be read-only. Later changes to the asset file will be reflected in all files that link it."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/editors/asset_browser.html — read 2026-07-30.

So when you link (rather than append) a node setup, what the linking file holds about, say, the image the setup samples is an indirectly-linked Image data-block belonging to the library — the image's own file path lives on that data-block, which belongs to the library file, not to the linking file.

### 2c. A relative path inside the linked setup is measured from the library file — documented in general, inferred for node setups

The general rule, in the manual's own words:

> "Many blend-files reference external images or other linked blend-files. A path tells Blender where to look for these files. If the external files are moved, the blend-file that references them will not look right.
>
> When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30. The phrase that carries the weight is "the referencing blend-file" — for data that lives in a library, the referencing file is the library.

The API provides the matching tool, and its wording states the rule plainly:

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the "//" prefix.
>
> Parameters:
>
> start (str | bytes) – Relative to this path, when not set the current filename is used.
>
> library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.path.html — read 2026-07-30.

The step I took, which makes this **inferred** for node setups specifically: neither passage mentions node trees. I am reading "a path that belongs to data from library L is resolved against L's own path, not the open file's" out of "its path replaces start" and "the referencing blend-file". Nothing in the sources applies that to a geometry node import path by name. See 2f for the nearest thing to direct evidence, and hand test 4.

### 2d. Libraries can chain, and the search order is recorded — documented

A library can itself have a parent library:

> "parent
>
> Type:
>
> Library, (readonly)"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30. (The API gives this property no description.)

When an item is imported, more than one library may be searched:

> "source_libraries
>
> List of libraries to search and import that ID from. The ID will be imported from the first file in that list that contains it"

and:

> "source_library
>
> Library ID representing the blendfile from which the ID was imported. None until the ID has been linked or appended"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendImportContextItem.html — read 2026-07-30.

### 2e. What happens when the library cannot be found, and the ways to repair it — documented

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

and:

> "Relocate Library
>
> …
>
> a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

and:

> "Reload Library
>
> …
>
> you can right-click a linked blend-file and choose Reload to immediately update the current blend-file with the latest version of the linked data-blocks, without having to reopen the file."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

The placeholder is visible in the API:

> "is_missing
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30.

An option controls whether placeholders are made at all:

> "USE_PLACEHOLDERS
>
> Generate a placeholder (empty ID) if not found in any library files."

and, on the same list:

> "MAKE_PATHS_RELATIVE
>
> Make paths of used library blendfiles relative to current blendfile."

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30.

The linked file can also be carried inside the linking file:

> "bpy.ops.file.pack_libraries()
>
> Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again"

and:

> "bpy.ops.file.unpack_libraries()
>
> Restore all packed linked data-blocks to their original locations"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30.

The manual's version:

> "Pack Linked Libraries
>
> …
>
> Mark all linked library files in the current blend-file as packed. Actual packing will happen on the next save of the blend-file."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — read 2026-07-30.

One change in this version is stated in the release notes:

> "* Linked IDs can now also be relocated, if they are not a dependency of any other linked data (blender/blender!135936, blender/blender@0eba8caaf9, blender/blender@3fa35aa4ee)."

Source: Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30. The manual's matching page:

> "Relocate Linked ID
>
> …
>
> A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30.

### 2f. Blender's own add-on code resolves a node's file path against the node tree's library — observed

This is the nearest thing in the material I was given to direct evidence for 2c, and it is code, not documentation. Cycles' OSL handling takes a file path off a node and turns it into an absolute path using the library of the node tree that contains the node.

Exact lines:

`Blender Sources/cycles_4.5/osl.py`, lines 162–169:

```
def update_external_script(report, filepath, library):
    """compile and update OSL script"""
    import os
    import shutil

    oso_file_remove = False

    script_path = bpy.path.abspath(filepath, library=library)
```

`Blender Sources/cycles_4.5/osl.py`, line 250, inside `update_script_node`:

```
        ok, oso_path, oso_file_remove = update_external_script(report, node.filepath, node.id_data.library)
```

`Blender Sources/cycles_4.5/osl.py`, line 209, inside `update_internal_script`:

```
    osl_path = bpy.path.abspath(script.filepath, library=script.library)
```

Read 2026-07-30.

What the file says, and only that: this code passes the containing node tree's library (`node.id_data.library`) as the `library` argument of `bpy.path.abspath` when turning a node's `filepath` into an absolute path. Combined with the documented meaning of that argument ("when the library is not None its path replaces start"), the path is resolved against the library file.

The line I am not crossing: this is a Cycles shader Script node in a shader tree, not a geometry node in a geometry node tree, and this is what the file says rather than what a render does. Concluding that geometry node import paths behave the same way would be **inferred**, and I do not have a source that says so. Hand test 4 settles it.

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer: which folder a `//` inside a linked setup is measured from, and what the linking file records, list entry 7; what packing a library carries and what it leaves behind, list entry 8; what is recorded when a library links onward, list entry 9; whether the library path is stored relative or absolute, list entry 10. Read this list as "the documentation does not say", not as "nobody knows".)*

This answer does not settle: whether the linking file writes down anything at all about a file used deep inside the library (an image's path, an import node's path) — I found no property or page that says it does, and nothing that says it does not; whether "Make Paths Relative" in the linking file touches paths belonging to linked data; whether "Pack Resources" packs images that arrive indirectly through a library; what happens when the library is found but a file the library needs is not; whether a linked node group's import-node path can be changed at all in the linking file (linked data is described as "initially not editable", link_append page, but the sources do not discuss node paths specifically); and the order in which a library and the files it needs are opened.

One near-miss worth recording, because it is the only place I found where the linking file writes down a library by name rather than by path. For baked geometry that references data-blocks, the modifier stores:

> "id_name
>
> Name that is mapped to the referenced data-block"

and:

> "lib_name
>
> Used when the data block is not local to the current .blend file but is linked from some library"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.NodesModifierDataBlock.html — read 2026-07-30. The manual's matching panel:

> "Baked geometries that reference other data-blocks such as materials are listed here. This panel allows changing these references after the data has been baked."

with:

> "Currently only material data-blocks are supported."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/baking.html — read 2026-07-30. This is about baked data, not about linking a node setup, so I do not offer it as an answer to the question — only as the closest documented case of a library being recorded by name.

### Outside the interface

Partly answered. Linking has a documented non-interface form, so nothing here depends on a person clicking:

> "bpy.ops.wm.link(...)
>
> Link from a Library .blend file
>
> Parameters:
>
> filepath (string, (optional, never None)) – File Path, Path to file
>
> directory (string, (optional, never None)) – Directory, Directory of the file
>
> filename (string, (optional, never None)) – File Name, Name of the file
>
> …
>
> relative_path (boolean, (optional)) – Relative Path, Select the file relative to the blend file"

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30.

And a scripted form that reads a library's contents directly:

> "load(filepath, link=False, relative=False, assets_only=False, …)
>
> Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked.
>
> …
>
> link (bool) – When False reference to the original file is lost.
>
> relative (bool) – When True the path is stored relative to the open blend file."

Source: Blender Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html — read 2026-07-30.

Two of the repair actions above — Relocate Library, Reload Library, Relocate Linked ID — are documented only as Outliner context-menu items, so as described they are interface actions. One behaviour is documented as differing when Blender is started from the command line:

> "--disable-liboverride-auto-resync
>
> Do not perform library override automatic resync when loading a new blend-file."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. That is about library overrides, not about plain linking.

**Not in the sources** for the rest: whether missing-library placeholders are created the same way when a file is opened in background mode, and whether anything about path resolution changes there.

**Brought up to date by hand test, 2026-08-06 — observed.** All of it holds with no window. Linking, packing the library, reading what the linking file records, resolving a `//` path belonging to linked data, and creating placeholders for a library that cannot be found were every one of them done and read from a script in a Blender started with `-b`, on a machine with no display server at all. With the library gone, Blender printed a warning naming it, left a placeholder in its place and exited with code 0. See "Observed by hand test" below, list entries 7, 8, 9 and 10. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers, so this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so two things could be looked at there that no terminal run can settle. The Path field on a linked group's Import node is drawn **greyed out** in the window, in all four versions — while a script writing to that same socket is accepted, which the Linux runs showed. And the modifier for a linked group holding no Bake node shows no Bake panel at all, only a collapsed section headed Manage; that section could not be opened without a click, so what it holds is untested. What a linked group that *does* hold a Bake node gives the modifier was answered instead with no window at all. See "Observed by hand test on the Creator's Mac" below, list entries 7, 8, 9, 10, 15, 16 and 17.

### Operating system

**Not in the sources.** No page about linking, libraries or relative paths names an operating system. The two general facts quoted under Question 1 — `bpy.path.native_pathsep` ("Replace the path separator with the systems native os.sep") and `bpy.path.resolve_ncase` ("Resolve a case insensitive path on a case sensitive system…") — say path separators and letter case differ between systems, which would apply to a library path like any other, but no source draws that connection. The requirements page (https://www.blender.org/download/requirements/, read 2026-08-01) names Blender 4.5 LTS under Windows, macOS and Linux, and says "Blender is cross-platform, it runs on every major operating system".

**Brought up to date by hand test, 2026-08-06 — observed.** Everything below was run on Linux only, where a path is matched letter for letter — see answer 1's observed section, list entry 11. Nothing has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed.** The letter-case test was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — as well, and came out the other way: there the upper-case forms found the file. See answer 1's Mac observed section, list entry 11. So a path is matched letter for letter on Linux and is not on macOS. Everything else here was run on both machines and agreed on both. No Windows machine has run it, so Windows is untested.

### Where I looked

- Read in full: the manual's Link & Append page and Linked Libraries index; Relative Paths section of Save & Load (`files/blend/open_save`); Packed Data; the External Data and Clean Up parts of the Topbar page; the Asset Browser page's Import Settings and asset-use sections; the Outliner interface page's display-mode list; the Save & Load preferences page.
- Read the API pages for Library, LibraryWeakReference, ID, BlendDataLibraries, BlendImportContext, BlendImportContextItem, NodesModifierDataBlock, bpy.path, and the linking parts of bpy.ops.wm and bpy.ops.file.
- Searched the whole manual for "indirect" (37 hits, all but two about indirect lighting or indirect animation — the two relevant ones are quoted above), "Relative Paths", "Make Paths Relative", "Report Missing Files", "Find Missing Files", "relative to the", "Import Method", "Append (Reuse Data)".
- Searched the whole API documentation for "indirect", "library_weak_reference", "is_library_indirect", "NodeSocketStringFilePath".
- Searched the 4.5 release notes for "librar", "link", "path" — the only relevant entries are the two quoted (core: linked ID relocation; core: path templates, which the notes say applies only to render output and the compositor's File Output node).
- Searched Cycles' own files for "library" and "filepath": the hits are `osl.py` (quoted above), `operators.py` (denoising input and output paths, unrelated) and `version_update.py` (checks `scene.library`, `mat.library` and similar while updating old files — I read those lines and they only skip or collect libraries during version conversion; nothing about how paths are resolved).
- Ruled out the requirements page for anything but the operating-system line.
- Terms I searched for and found nothing useful under: "indirect dependency", "indirectly linked" (in the manual, only the one Localize All line), "blendcache" for library caches, "search path" for libraries.

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

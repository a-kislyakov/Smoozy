# Node setups — findings for Blender 5.3

- Sources consulted: the Blender 5.3 manual, the 5.3 Python API documentation and the 5.3 release notes (snapshot of 2026-10-11, used as the date read), and Cycles' own files for 5.3 (taken 2026-10-16, used as the date read). All of them sit inside `Blender Sources/`. I also opened two other files inside `Blender Sources/`: the folder's own note (`About this folder.md`) and the manual's licence page. The folder note links to a method file outside the folder; I did not follow that link.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I ran nothing. No Blender was started. Where a finding is labelled **observed**, it comes from reading Cycles' own files, as the brief allows.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**Short answer.** The sources name seven kinds:

1. The file named in an **import node's Path**: `.obj`, `.ply`, `.stl`, `.csv`, `.txt`, `.vdb`. The API and release notes add `.spz`.
2. **Image files**, through image data-blocks used by the Image, Image Texture and Image Info nodes or by image inputs. One image can stand for a sequence of files, a movie or a set of UDIM tiles.
3. **Font files**, through font data-blocks used by the Font node and String to Curves.
4. **Sound files**, through sound data-blocks used by Sample Sound Frequencies.
5. **Bake folders on disk**, written by Bake nodes and Simulation Zones when the bake target is Disk.
6. **Other `.blend` files**: node groups linked from another file. By inference, also objects, collections and materials the setup picks that are themselves linked.
7. **File paths typed into the modifier**: a group input with the File Path subtype, shown on the Geometry Nodes modifier (inferred).

Details follow.

### 1.1 Import nodes name a file directly — documented

- "Nodes used to import data from external file formats." The page then lists "CSV (.csv)", "Wavefront (.obj)", "Stanford PLY (.ply)", "STL (.stl)", "Text (.txt)", "OpenVDB (.vdb)".
- "the corresponding import node is added automatically and configured with the file path."
  - Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/index.html — read 2026-10-11
- Each node's input is called Path:
  - "The path to the .obj file to be imported." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/obj.html
  - "The path to the CSV file." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/csv.html
  - "The path to the .ply file to be imported." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/ply.html
  - "The path to the .stl file to be imported." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/stl.html
  - "The path to the txt file." and "Currently, only files with the .txt extension are supported." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/txt.html
  - "The path to the VDB file to be imported." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/vdb.html
  - All manual, read 2026-10-11.
- An SPZ import node exists. The manual's import list does not include it.
  - "Import a point cloud object that is rendered as gaussian splat from an SPZ file" — API — https://docs.blender.org/api/5.3/bpy.types.GeometryNodeImportSPZ.html — read 2026-10-11
  - "Added `Import SPZ` node." — release notes — https://developer.blender.org/docs/release_notes/5.3/rendering/ — read 2026-10-11

### 1.2 The import path need not be written inside the node — inferred

- Every import node page lists Path under "Inputs". The manual picture `_images/node-types_GeometryNodeImportOBJ.webp` on https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/obj.html (read 2026-10-11) shows the Path field with a socket dot at its left edge and a folder button at its right (documented: the picture shows this).
- Step I took: in a node setup, an input with a socket can take a link from another node. So the file name can come from a group input, or be built by text nodes such as Join Strings or Format String while the setup runs. The sources never say this about import nodes in so many words.
- A group input can be a file path, and the modifier keeps its own value for it:
  - "File Path: The string is interpreted as a path to a file, enabling file selection dialogs in the user interface." — Manual — https://docs.blender.org/manual/en/5.3/interface/controls/nodes/groups.html — read 2026-10-11
  - "A list of the node group’s inputs which can have unique values even if the group is shared among multiple modifiers." — Manual — https://docs.blender.org/manual/en/5.3/modeling/modifiers/geometry_nodes.html — read 2026-10-11
  - Step: a file path set on the modifier is stored with that modifier, not inside the node group.
- Path text can be relative to the `.blend` file. The API says so for the file-path socket type: "default_value … (default “”, never None, blend relative // prefix supported)" — https://docs.blender.org/api/5.3/bpy.types.NodeSocketStringFilePath.html — read 2026-10-11. That the import nodes' Path uses this socket type is my inference (the folder button in the picture matches "enabling file selection dialogs"). It is not stated.

### 1.3 Image files — documented for image data-blocks, inferred for sequences and tiles

- "The Image node provides access to a image file"; "select an existing image or open an image from the file browser." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/constant/image.html
- "The Image Texture node is used to add an image file as a texture."; "If this is not connected the image can be opened or selected from the node." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/texture/image.html
- Image Info: "Source image to get parameters from." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/scene/image_info.html
- The image data-block holds the path: "Image data-block referencing an external or packed image"; filepath "Image/Movie file name (default “”, never None, blend relative // prefix supported)". — API — https://docs.blender.org/api/5.3/bpy.types.Image.html
- All read 2026-10-11.

One image can mean many files — **inferred**. The image's source can be "Image Sequence – Multiple image files, as a sequence.", "Movie – Movie file." or "UDIM Tiles – Tiled UDIM image texture." (API, same page). The Image Texture node says "If the Image supports animation, the frame can be set here." Step: an image of one of those kinds, used by a geometry node, makes the file depend on every file in the sequence or tile set, or on the movie. The sources do not say which image kinds the geometry nodes accept.

### 1.4 Font files — documented node; the font file itself is inferred

- "The Font node outputs a font data-block that can be used by other nodes"; "By default, Blender’s built-in font is selected." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/constant/font.html
- String to Curves, Font input: "Font glyph used to generate the curve." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/utilities/text/string_to_curves.html
- The font data-block has a path and can be packed: filepath "(default “”, never None, blend relative // prefix supported)"; also a packed file property. — API — https://docs.blender.org/api/5.3/bpy.types.VectorFont.html
- All read 2026-10-11.
- Step (inferred): the API gives this path no description. That it names a font file comes from the preferences page, whose Fonts entry reads: "Default location to browse for text object font files." (https://docs.blender.org/manual/en/5.3/editors/preferences/file_paths.html, read 2026-10-11).

### 1.5 Sound files — documented

- "Nodes to manipulate sound and audio files." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/utilities/sound/index.html
- "retrieves the amplitude of a frequency range from a sound data-block at a given point in time." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/utilities/sound/sample_sound_frequencies.html
- "Sound data-block referencing an external or packed sound file"; filepath "Sound sample file used by this Sound data-block (default “”, never None, blend relative // prefix supported)". — API — https://docs.blender.org/api/5.3/bpy.types.Sound.html
- All read 2026-10-11.

Images, fonts and sounds can be packed into the `.blend` file. When packed, no outside file is needed: "an image texture that is an external image file can be put “inside” the blend-file." — Manual — https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html — read 2026-10-11. The sources say nothing about packing the files named by import nodes.

### 1.6 Bake folders on disk — documented and inferred

- Bake node, Bake: "Performs all necessary geometry calculations and saves the data on disk or packed into the .blend file."
- Bake Target, Disk: "The baked data is stored in a separate directory on disk."
- Packed: "The baked data is packed into the .blend file. So no separate file is necessary."
- Bake Path: "Location on disk where the baked data is stored. Note, this path is also used for simulation zones."
  - Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-10-11
- On the modifier: "Bake Target: Specifies where baked data should be stored. This can be overridden for individual bakes."; "Bake Path: Location on disk where the baked data for Simulation Zones and Bake Nodes are stored."
  - Manual — https://docs.blender.org/manual/en/5.3/modeling/modifiers/geometry_nodes.html — read 2026-10-11
- What lands in the folder: "Volume objects, however, are saved using the OpenVDB file format which can be used interoperably." Also: "Data is not stored to disk until the Bake operation is ran." and "Blend-files must be saved to a disk before data can be baked."
  - Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/baking.html — read 2026-10-11
- Default location: "The location the file is saved determines where the baked data is also saved." — Manual — https://docs.blender.org/manual/en/5.3/physics/simulation_nodes.html — read 2026-10-11
- API, on the modifier: bake_directory "Location on disk where the bake data is stored (default “”, never None, blend relative // prefix supported, Supports template expressions)"; bake_target "(default 'PACKED')". — https://docs.blender.org/api/5.3/bpy.types.NodesModifier.html — read 2026-10-11
- API, per bake: directory, with the same wording; use_custom_path "Specify a path where the baked data should be stored manually"; bake_target "(default 'INHERIT')". — https://docs.blender.org/api/5.3/bpy.types.NodesModifierBake.html — read 2026-10-11
- Release note: "Bake path template support (blender/blender!164240)." — https://developer.blender.org/docs/release_notes/5.3/geometry_nodes/ — read 2026-10-11

**The sources disagree on bake paths.** The manual's path template page still says: "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node." (https://docs.blender.org/manual/en/5.3/files/file_paths.html, read 2026-10-11). The API and the 5.3 release note say bake paths do support templates. So the stored bake path may not be the final folder name.

**Inferred: the bake path is recorded on the modifier, not in the node group.** The API puts the bakes on the modifier: bakes "All potential bakes, as defined by the assigned Geometry Nodes", and each bake's node is "Bake node or simulation output node that corresponds to this bake. This node may be deeply nested in the modifier node group." (API pages above). Step: the bake folder belongs to the modifier on an object, even though the Bake node sits inside the node group.

### 1.7 Other `.blend` files — documented for node groups, inferred for other data

- "This section lists all the node groups, both those in the current blend-file and those Linked or Appended from another blend-file." — Manual — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/group.html
- "It’s also possible to import node groups from a different blend-file using File ‣ Link/Append." — Manual — https://docs.blender.org/manual/en/5.3/interface/controls/nodes/groups.html
- Asset Browser, Link: "The asset will be linked to the current blend-file, and thus be read-only."; "Geometry nodes can also be dragged onto objects to add a Geometry Nodes Modifier." — Manual — https://docs.blender.org/manual/en/5.3/editors/asset_browser.html
- All read 2026-10-11.
- Inferred: Object Info, Collection Info, Material, and Instance on Elements in its data-block mode each pick an object, collection or material. Linking works for these too: "These functions help you reuse objects, materials and other data-blocks from another blend-file." (https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html, read 2026-10-11). So a setup that picks a linked object, collection or material makes the file depend on that other `.blend`. Those data-blocks may point at files of their own; that is the ordinary kind of reference the sheet excludes, so I did not follow it.
- Bundled node groups: "This is a node group asset that is included in the bundled “Essentials” asset library." — https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/normals/smooth_by_angle.html. The Import Method choice shows only "when an asset library other than Current File or Essentials is selected" (https://docs.blender.org/manual/en/5.3/editors/asset_browser.html). Both read 2026-10-11. Whether using an Essentials node group makes the file point at a bundled `.blend` file is **not in the sources**. *(Hand test, 2026-10-18: answered by observation on the Creator's Mac — see "What adding Smooth by Angle records in the file", 5.3 list entry 26, under question 1's observed section below.)*

**Not covered.**
- Whether the File ‣ External Data tools see import-node paths, file-path modifier inputs or bake folders: Report Missing Files, Find Missing Files, Make Paths Relative/Absolute, Pack Resources. The sources say nothing either way.
- The folder layout and file names inside a bake folder.
- The bake path used when none is set. The API default is an empty text.
- Whether the API default 'PACKED' is what a new modifier actually shows.
- Whether the built-in font refers to any file.
- Whether import nodes read their file again each time the setup runs.
- The API has a Text data-block socket type. No geometry node page uses one, and I found no geometry node that reads a Text data-block.
- Cycles' own files say nothing on this question. They hold only one geometry-nodes remark, in a radial tiling source file, which is unrelated.

*(Note added by the hand tests, 2026-10-18. Every line in this list stays true of the **sources**. Most of what it names was afterwards observed on the Creator's Mac, in the section "Observed by hand test on the Creator's Mac — 2026-10-18" below: the External Data tools and import-node paths, 5.3 list entry 10; and file-path modifier inputs, entry 11; and bake folders, entry 15; the layout inside a bake folder and the path used when none is set, entry 14; what a new modifier shows for Bake Target, entry 17; the built-in font, entry 12; whether import nodes read their file again, entries 4 and 6; nodes with a Text data-block input, entry 2. The disagreement between the sources on bake path templates, under 1.6, is observed in entry 16. Read this list as "the documentation does not say", not as "nobody knows".)*

**Where I looked.** I listed every page under `modeling/geometry_nodes/` and read the import, constant input, scene input, texture, sound, text, baking, bake node, simulation zone and group pages. I also read the Geometry Nodes modifier page, the Simulation Nodes physics page, the node groups and node parts pages, and the file path, packed data and link/append pages.
- Manual searches over geometry nodes: "file path", "filepath", "external file", "on disk", "directory", ".vdb", "data-block", "bake", "import node", "Essentials", "bake path", "blendcache", "internal dependencies".
- API: I scanned every `GeometryNode…`, `FunctionNode…` and `NodeGeometry…` page for path, file, directory, image, font, sound, object, collection and material. I also listed every node socket type and read the import node, socket, modifier, bake, Image, VectorFont, Sound and Volume pages.
- Release notes: I read all of geometry_nodes, rendering, core, pipeline_io, python_api and assets, and searched for "SPZ" and "splat".
- Cycles files: searched for "geometry node", "GeometryNode", "node_group", "library" and "bake_directory".
- Ruled out: the physics point-cache option "Use Library Path" (https://docs.blender.org/manual/en/5.3/physics/baking.html, read 2026-10-11). It belongs to classic physics caches, not node setups.

**Outside the interface.** Everything above is stored data in the file, so it exists however Blender is started. That much is inferred; the sources do not say it outright.
- Each import node, the image and font nodes, and the bake settings all have Python API entries.
- Bakes can be run without clicking: "Bake a single bake node or simulation" (`bpy.ops.object.geometry_node_bake_single`) and "Bake simulations in geometry nodes modifiers" (`bpy.ops.object.simulation_nodes_cache_bake`) — https://docs.blender.org/api/5.3/bpy.ops.object.html — read 2026-10-11.
- Dropping a file into the node editor is an interface action. An operator exists for it: "Add an import node to the node tree" (https://docs.blender.org/api/5.3/bpy.ops.node.html — read 2026-10-11).
- Path templates are filled in "internally at time of use (e.g. while rendering)" (file paths page).
- The sources do not say whether any of this behaves differently in background mode.

**Brought up to date by hand test, 2026-10-18 — observed, on the Creator's Mac only.** Every test below except four was run in a Blender started from the terminal with no window, and nothing in them needed a window. Such a Blender reads an import node's file when it opens the `.blend`, and a render started from the command line with no window reads it too; the picture was the same with `--disable-depsgraph-on-file-load` as without it (entries 5 and 8). Housekeeping, packing, bakes to disk, bake path templates and the built-in font were all driven from scripts (entries 10, 11, 12, 14, 15, 16). Two things a script driving Blender from outside meets that a person clicking does not: asking for the evaluated geometry again does not by itself make an import node read its file again — something has to mark the setup for a fresh run, and the file's modification time has to have changed by a whole second (entry 6); and in 5.3 a value set on the modifier for a group input is reached through `mod.properties.inputs.<identifier>.value`, not `mod[identifier]`, which 5.3 refused (entry 11). The four window tests (entries 13 and 17, and 23 and 24 under question 2) were run by opening Blender's window from the terminal and taking Blender's own screenshots, with nothing clicked. Dragging a file into the node editor was not tested: no script can make a real drag happen.

**Operating system.** Not in the sources. Nothing in the pages above treats Windows, macOS or Linux differently for these nodes, their paths or bake folders.

**Brought up to date by hand test, 2026-10-18 — observed.** Everything observed below was run on **the Creator's Mac, macOS 26.6.2, on its APFS volume as the Mac came**, and holds for that machine only. There, an import node's path and an image path were **not** matched letter for letter: `//MODEL.OBJ` found `model.obj` (entry 7). And a changed file was read again only when its modification time moved across a whole second (entry 6). **Nothing was run on Linux or Windows in 5.3**: no such machine was available to this sitting. That is a matter of nobody having run it, not of it being impossible — the equipment section of `Preparation/Blender Research/Method.md` describes the route to a rented Windows machine, and every Windows machine this project has read reported a Server edition, so a Windows answer must name its edition. Whether letter case behaves otherwise on a case-sensitive volume on the same Mac was not tried either: making one is a change to the machine.

### Observed by hand test on the Creator's Mac — 2026-10-18

**LABEL: observed.**

All of this was run on **the Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83)** — on **2026-10-18**, with `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`. Every run was started with `--factory-startup`, with `BLENDER_USER_RESOURCES` pointing at a settings folder made for these tests before anything was started (Blender was asked and named that folder as the one in use), and with `TMPDIR` pointing at the work folder. Working folders were fresh, under `/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/`, on the volume `Macintosh HD - Data`, which is APFS, the Mac's ordinary file system. Another Blender, the Creator's own, was open on the Mac throughout and was not touched; nothing here measures time or memory. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests — 5.3.md`, under the entry numbers used below. **That list numbers its entries afresh; its numbers are not the numbers of the list the earlier versions used.**

Nothing here was run on Linux or Windows. No such machine was available to this sitting.

#### What the build itself reports about the import nodes (5.3 list entry 1)

Blender was started with no window and a script that listed the classes the build registers whose names begin `GeometryNodeImport`, counted those beginning `GeometryNode`, and checked 26 named classes. It then made a throwaway geometry node group, added each import node to it and read the socket type of every input; linked a String node's output into Import OBJ's Path; and read the inputs of String to Curves. Blender printed, in full:

```
BLENDER: 5.3.0 Beta
--- geometry node classes whose name starts with GeometryNodeImport ---
['GeometryNodeImportCSV', 'GeometryNodeImportOBJ', 'GeometryNodeImportPLY', 'GeometryNodeImportSPZ', 'GeometryNodeImportSTL', 'GeometryNodeImportText', 'GeometryNodeImportVDB']
--- count of GeometryNode* classes ---
292
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
GeometryNodeImportSPZ [('Path', 'NodeSocketStringFilePath')]
--- can a String node output be linked into Import OBJ's Path? ---
string node output sockets: [('String', 'NodeSocketString')]
links.new returned: <bpy_struct, NodeLink at 0xa3b28d468>
number of links in tree now: 1
link: FunctionNodeInputString String -> GeometryNodeImportOBJ Path is_valid: True
--- String to Curves: is the font an input socket or a property? ---
inputs: [('String', 'NodeSocketString'), ('Size', 'NodeSocketFloatDistance'), ('Font', 'NodeSocketFont'), ('Align X', 'NodeSocketMenu'), ('Align Y', 'NodeSocketMenu'), ('Pivot Point', 'NodeSocketMenu'), ('Character Spacing', 'NodeSocketFloat'), ('Word Spacing', 'NodeSocketFloat'), ('Line Spacing', 'NodeSocketFloat'), ('Overflow', 'NodeSocketMenu'), ('Text Box Width', 'NodeSocketFloatDistance'), ('Text Box Height', 'NodeSocketFloatDistance')]
has .font attribute: False
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: seven import node classes, `GeometryNodeImportSPZ` among them. Every import node's Path input, the SPZ node's included, is of the type `NodeSocketStringFilePath`. The link from a String node into Path was made and reported valid. String to Curves takes its font through an input socket of the type `NodeSocketFont` and has no `font` property.

#### Every node that can take a file path or a data-block (5.3 list entry 2)

Blender was started with no window and a script that took every class the build registers whose name begins `GeometryNode` or `FunctionNode` and is a kind of node, and added each in turn to a throwaway geometry node group. For each node it made, it printed the node if any input was of a file-path, image, font, sound, object, collection, material, texture or text-data-block type, or if the node had a property (beyond those every node has) pointing at a data-block or holding a file path. Blender printed, in full:

```
BLENDER: 5.3.0 Beta
GeometryNodeBoneInfo inputs: [('Armature', 'NodeSocketObject')] properties: []
GeometryNodeCameraInfo inputs: [('Camera', 'NodeSocketObject')] properties: []
GeometryNodeCollectionChildren inputs: [('Collection', 'NodeSocketCollection')] properties: []
GeometryNodeCollectionInfo inputs: [('Collection', 'NodeSocketCollection')] properties: []
GeometryNodeGroup inputs: [] properties: [('node_tree', 'NodeTree')]
GeometryNodeImageInfo inputs: [('Image', 'NodeSocketImage')] properties: []
GeometryNodeImageTexture inputs: [('Image', 'NodeSocketImage')] properties: []
GeometryNodeImportCSV inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeImportOBJ inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeImportPLY inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeImportSPZ inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeImportSTL inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeImportText inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeImportVDB inputs: [('Path', 'NodeSocketStringFilePath')] properties: []
GeometryNodeInputCollection inputs: [] properties: [('collection', 'Collection')]
GeometryNodeInputFont inputs: [] properties: [('font', 'VectorFont')]
GeometryNodeInputImage inputs: [] properties: [('image', 'Image')]
GeometryNodeInputMaterial inputs: [] properties: [('material', 'Material')]
GeometryNodeInputObject inputs: [] properties: [('object', 'Object')]
GeometryNodeMaterialSelection inputs: [('Material', 'NodeSocketMaterial')] properties: []
GeometryNodeObjectInfo inputs: [('Object', 'NodeSocketObject')] properties: []
GeometryNodeReplaceMaterial inputs: [('Old', 'NodeSocketMaterial'), ('New', 'NodeSocketMaterial')] properties: []
GeometryNodeSampleSoundFrequencies inputs: [('Sound', 'NodeSocketSound')] properties: []
GeometryNodeSetMaterial inputs: [('Material', 'NodeSocketMaterial')] properties: []
GeometryNodeStringToCurves inputs: [('Font', 'NodeSocketFont')] properties: []
--- counts ---
node classes tried: 342 made: 338 could not be made: 4
could not be made: [('FunctionNode', 'RuntimeError'), ('GeometryNode', 'RuntimeError'), ('GeometryNodeApplySimulatedData', 'RuntimeError'), ('GeometryNodeCustomGroup', 'RuntimeError')]
nodes with a file-path or data-block input or property: 25
nodes with a NodeSocketText input: []
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: 25 nodes. The seven import nodes take a file path. Image Info and Image Texture take an image through an input, and the Image node holds one as a property. String to Curves takes a font through an input, and the Font node holds one as a property. Sample Sound Frequencies takes a sound through an input. The rest take or hold objects, collections, materials, or (the group node) another node group. **No node made here has an input of the text-data-block type, and none has an input of the texture type.** The Bake node is not among the 25: it has no file-path input and no file-path property. Four classes could not be made in a group; two of them, `FunctionNode` and `GeometryNode`, are the names of the node kinds themselves rather than nodes.

#### What path facilities this build has (5.3 list entry 3)

The same shape of run, reading each type's properties and functions out of Blender's own description of it rather than asking Python whether an attribute exists. It also read the default of the modifier's Bake Target from that description, called `bpy.path.abspath('//x.obj', library=None)`, asked `bpy.data.libraries.load` for `pack=True` without `link`, and listed what an empty factory file already points at. This bears on question 2 as well. Blender printed, in full:

```
BLENDER: 5.3.0 Beta
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
['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

The line asking for the flag list of `file_path_foreach` comes back with a `KeyError` because that function is not in the description the line reads from; it settles nothing either way and is left as it stands. What came back: every facility named is present; the modifier's Bake Target defaults to `PACKED` in the build's own description; and an empty factory file already lists one of Blender's own brush files, twice.

#### Which path forms the Path accepts, what a missing one does, and when the file is read again (5.3 list entry 4)

A folder was made holding `model.obj` (three vertices, with `mtllib model.mtl` and `usemtl red` in it), `model.mtl`, a four-by-four `tex.png`, and `scene.blend` saved beside them. Inside the file the factory cube carried a Geometry Nodes modifier whose group was Import OBJ → Realize Instances → Join Geometry → Group Output, with an Image Texture node holding the image and connected to nothing. Blender was started in the folder above the work folder, so `./model.obj` names a file that is not there. The Path was set to five values in turn and the cube's evaluated vertex count read after each. Then, with Path at `//model.obj`, `model.obj` was overwritten on disk with a four-vertex shape and the count read after each of: nothing; turning the modifier off and on; stepping to frame 2; setting the same Path again. The file's modification time before and after the overwrite was printed. Last, the `.blend` was saved and opened again in the same Blender and read once more. The materials column reads the evaluated mesh's own material slots. Blender printed, in full:

```
Info: Saved as "scene.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3
saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/scene.blend
image filepath after save: //tex.png
--- materials in bpy.data before any import ---
['Dots Stroke', 'Material']
PATH='/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/model.obj' (absolute path to model.obj) -> vertices: 3 ; materials on evaluated mesh: []
PATH='//model.obj' (blend-relative //model.obj) -> vertices: 3 ; materials on evaluated mesh: []
00:00.533  io.obj           | ERROR Cannot read from OBJ file:'./model.obj'.
00:00.533  reports          | ERROR OBJ Import: Cannot open file './model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
PATH='./model.obj' (relative-with-dot ./model.obj) -> vertices: 0 ; materials on evaluated mesh: ['Material']
00:00.534  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/nothing-here.obj'.
00:00.534  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/nothing-here.obj'
PATH='/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/nothing-here.obj' (a path that does not exist) -> vertices: 0 ; materials on evaluated mesh: ['Material']
PATH='' (empty path) -> vertices: 0 ; materials on evaluated mesh: ['Material']
--- materials in bpy.data after the imports ---
['Dots Stroke', 'Material']
--- what the Path socket holds when read back ---
stored value: '//model.obj'
bpy.path.abspath of it: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/model.obj
--- does the file get read again when it changes on disk? ---
vertices with the 3-vertex file: 3
vertices straight after overwriting with the 4-vertex file: 3
vertices after touching the modifier: 4
vertices after stepping the frame: 4
vertices after setting the same path again: 4
(model.obj stamp before the overwrite 1792300384.949, after 1792300385.007, whole second changed: True)
--- and after saving, quitting the file and opening it again? ---
00:00.540  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/scene.blend"
Info: Saved "scene.blend"
vertices after re-opening the .blend (file on disk has 4): 4
Path socket after re-opening: '//model.obj'
DONE

Blender quit
```

What came back: the absolute path and `//model.obj` each gave 3 vertices; `./model.obj`, a path to nothing and an empty Path each gave 0, the first two with an `ERROR` line naming the path as typed or as made absolute, the empty one with none. The Path keeps `//model.obj` as typed. After the overwrite, the count stayed at 3 at the first reading and was 4 from the reading after the modifier was turned off and on; this run's overwrite moved the file's modification time across a whole second (`…384.949` to `…385.007`). Entry 6 below runs that question properly. After saving and reopening in the same Blender, the count was 4.

The materials column comes from this sitting's own script, which reads the evaluated mesh's material slots; with an empty result those slots still held the cube's own `Material`. A reader setting this column beside another version's should first check that both were read the same way; the list named above says how, under *Controls in 5.2.2*.

#### Whether a Blender with no window reads the file when it opens the `.blend` (5.3 list entry 5)

The folder entry 4 left behind — `scene.blend` with Path `//model.obj`, and the four-vertex shape now in `model.obj` on disk — was opened three times, each in a fresh Blender started with `-b` and the file named on the command line, with a script that read the cube's evaluated vertex count, the Path, and every image. The second run had `model.obj` moved away first (and put back after); the third added `--disable-depsgraph-on-file-load` before the file name. The three runs, uncut:

```
########## fresh process, file on disk now has 4 vertices
00:00.442  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
########## same, with model.obj moved away
00:00.328  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/scene.blend"
00:00.333  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/model.obj'.
00:00.333  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
exit code: 0
########## same, with --disable-depsgraph-on-file-load
00:00.330  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/scene.blend"
VERTICES: 4
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: False
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: 4 vertices in a fresh Blender, the count of the file on disk at that moment; 0 with the file gone, with an `ERROR` line naming the file as an absolute path and an exit code of 0; 4 again with `--disable-depsgraph-on-file-load`. The third run does not isolate what that switch does, because the script itself asks for evaluated geometry, which builds the evaluation; entry 8's render answers that half.

#### When a changed file is read again: the whole second (5.3 list entry 6)

**This is the first time this test has been run on macOS.** It asks whether an import node reads its file again after the file changes on disk, and what decides it.

**What was done.** Each run, in a Blender of its own with no window, built entry 4's folder afresh (`model.obj` with three vertices, `model.mtl`, `tex.png`, `scene.blend`, the same group on the cube), set Path to `//model.obj`, overwrote `model.obj` with a four-vertex shape and read the cube's evaluated vertex count. Four arrangements followed entry 4's own steps (straight after the overwrite; after turning the modifier off and on; after stepping the frame; after setting the same Path again; after putting the three-vertex file back): **plain**, reading as entry 4 reads; **tagged**, calling `ob.update_tag()` before every reading; **both**, the two side by side; and **routes**, trying each way of nudging the setup in turn. Then the plain arrangement ten times over. Then twenty runs in which the script set the three-vertex file's modification time itself, to a quarter of a second past a whole second, read once, overwrote the file, moved its time by a chosen amount, and read three more times — straight after, then after each of two modifier toggles — with and without the tag. Last, four runs that moved the time by a whole second and read five times, with or without a toggle before each reading and with or without the tag.

**What a negative result would have looked like.** A line with the whole second unchanged whose readings reach 4, or a line with it changed whose readings never do.

**The first form of the script, and why it is not the one that answers.** The first form gave the Path its value while building the group, before the `.blend` had ever been saved; the second gives it its value after the first save, as entry 4 does. With the first form, the first untagged reading in every arrangement that had one came back 0, with no error printed, and the plain arrangement went 0, 0, 4, 4, 4, 4 with the whole second unchanged. Both forms happened; the first is kept in the list named above. The words of the first form's plain run:

```
STEP with the 3-vertex file                             plain: 0    tagged: -
STEP straight after overwriting with the 4-vertex file  plain: 0    tagged: -
STEP after touching the modifier                        plain: 4    tagged: -
STEP after stepping the frame                           plain: 4    tagged: -
STEP after setting the same path again                  plain: 4    tagged: -
STEP after putting the 3-vertex file back               plain: 4    tagged: -
(stamps: overwrite 1792300462.130 -> 1792300462.166 whole second changed False; put back 1792300462.166 -> 1792300462.167 whole second changed False)
```

**The four arrangements, second form, word for word:**

```
=== plain ===
Info: Saved as "scene.blend"
STEP with the 3-vertex file                             plain: 3    tagged: -
STEP straight after overwriting with the 4-vertex file  plain: 3    tagged: -
STEP after touching the modifier                        plain: 3    tagged: -
STEP after stepping the frame                           plain: 3    tagged: -
STEP after setting the same path again                  plain: 3    tagged: -
STEP after putting the 3-vertex file back               plain: 3    tagged: -
(stamps: overwrite 1792300499.686 -> 1792300499.720 whole second changed False; put back 1792300499.720 -> 1792300499.720 whole second changed False)
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
=== tagged ===
Info: Saved as "scene.blend"
STEP with the 3-vertex file                             plain: -    tagged: 3
STEP straight after overwriting with the 4-vertex file  plain: -    tagged: 3
STEP after touching the modifier                        plain: -    tagged: 3
STEP after stepping the frame                           plain: -    tagged: 3
STEP after setting the same path again                  plain: -    tagged: 3
STEP after putting the 3-vertex file back               plain: -    tagged: 3
(stamps: overwrite 1792300500.531 -> 1792300500.567 whole second changed False; put back 1792300500.567 -> 1792300500.568 whole second changed False)
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
=== both ===
Info: Saved as "scene.blend"
STEP with the 3-vertex file                             plain: 3    tagged: 3
STEP straight after overwriting with the 4-vertex file  plain: 3    tagged: 3
STEP after touching the modifier                        plain: 3    tagged: 3
STEP after stepping the frame                           plain: 3    tagged: 3
STEP after setting the same path again                  plain: 3    tagged: 3
STEP after putting the 3-vertex file back               plain: 3    tagged: 3
(stamps: overwrite 1792300501.389 -> 1792300501.425 whole second changed False; put back 1792300501.425 -> 1792300501.426 whole second changed False)
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
=== routes ===
Info: Saved as "scene.blend"
route: (none, baseline with the 3-vertex file) -> 3
route: (none, straight after the overwrite) -> 3
route: toggling mod.show_viewport twice -> 3
route: stepping the frame -> 3
route: setting the same path again -> 3
route: setting a different path and back -> 3
route: ob.data.update_tag() -> 3
route: ob.update_tag() -> 3
route: taking the group off the modifier and putting it back -> 3
(stamp now 1792300502.266)
control: 3-vertex file back, no tag -> 3
control: 3-vertex file back, ob.update_tag() -> 3
(stamp now 1792300502.267)
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**The rest, 34 runs.** Every one of them also printed `Info: Saved as "scene.blend"`, `DONE`, the version line and `Blender quit`, and nothing else; those four lines are left out below and the lines that carry the answer are kept uncut. The ten plain repeats, each giving the count at the reading after the modifier toggle:

```
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
REPEAT 3
```

The stamp set by hand. Each line gives the offset, whether the tag was used, the two stamps, whether the whole second changed, and the four readings (before the overwrite; straight after; after the first toggle; after the second):

```
BLENDER 5.3.0 Beta  offset +0.000  tag False  stamp 1792300518.250 -> 1792300518.250  whole second 1792300518 -> 1792300518  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.000  tag True   stamp 1792300518.250 -> 1792300518.250  whole second 1792300518 -> 1792300518  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.500  tag False  stamp 1792300519.250 -> 1792300519.750  whole second 1792300519 -> 1792300519  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.500  tag True   stamp 1792300520.250 -> 1792300520.750  whole second 1792300520 -> 1792300520  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.700  tag False  stamp 1792300521.250 -> 1792300521.950  whole second 1792300521 -> 1792300521  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.700  tag True   stamp 1792300522.250 -> 1792300522.950  whole second 1792300522 -> 1792300522  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.740  tag False  stamp 1792300523.250 -> 1792300523.990  whole second 1792300523 -> 1792300523  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.740  tag True   stamp 1792300523.250 -> 1792300523.990  whole second 1792300523 -> 1792300523  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset +0.760  tag False  stamp 1792300524.250 -> 1792300525.010  whole second 1792300524 -> 1792300525  changed: True   readings: 3 3 4 4
BLENDER 5.3.0 Beta  offset +0.760  tag True   stamp 1792300525.250 -> 1792300526.010  whole second 1792300525 -> 1792300526  changed: True   readings: 3 4 4 4
BLENDER 5.3.0 Beta  offset +0.900  tag False  stamp 1792300526.250 -> 1792300527.150  whole second 1792300526 -> 1792300527  changed: True   readings: 3 3 4 4
BLENDER 5.3.0 Beta  offset +0.900  tag True   stamp 1792300527.250 -> 1792300528.150  whole second 1792300527 -> 1792300528  changed: True   readings: 3 4 4 4
BLENDER 5.3.0 Beta  offset +1.000  tag False  stamp 1792300528.250 -> 1792300529.250  whole second 1792300528 -> 1792300529  changed: True   readings: 3 3 4 4
BLENDER 5.3.0 Beta  offset +1.000  tag True   stamp 1792300529.250 -> 1792300530.250  whole second 1792300529 -> 1792300530  changed: True   readings: 3 4 4 4
BLENDER 5.3.0 Beta  offset +2.000  tag False  stamp 1792300529.250 -> 1792300531.250  whole second 1792300529 -> 1792300531  changed: True   readings: 3 3 4 4
BLENDER 5.3.0 Beta  offset +2.000  tag True   stamp 1792300530.250 -> 1792300532.250  whole second 1792300530 -> 1792300532  changed: True   readings: 3 4 4 4
BLENDER 5.3.0 Beta  offset -0.200  tag False  stamp 1792300531.250 -> 1792300531.050  whole second 1792300531 -> 1792300531  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset -0.200  tag True   stamp 1792300532.250 -> 1792300532.050  whole second 1792300532 -> 1792300532  changed: False  readings: 3 3 3 3
BLENDER 5.3.0 Beta  offset -0.500  tag False  stamp 1792300533.250 -> 1792300532.750  whole second 1792300533 -> 1792300532  changed: True   readings: 3 3 4 4
BLENDER 5.3.0 Beta  offset -0.500  tag True   stamp 1792300534.250 -> 1792300533.750  whole second 1792300534 -> 1792300533  changed: True   readings: 3 4 4 4
```

Five readings after a whole-second change, with or without a toggle before each, and with or without the tag:

```
BLENDER 5.3.0 Beta  touch True  tag False  whole second 1792300534 -> 1792300535  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.3.0 Beta  touch True  tag True   whole second 1792300535 -> 1792300536  first reading 3 then [4, 4, 4, 4, 4]
BLENDER 5.3.0 Beta  touch False tag False  whole second 1792300536 -> 1792300537  first reading 3 then [3, 3, 3, 3, 3]
BLENDER 5.3.0 Beta  touch False tag True   whole second 1792300537 -> 1792300538  first reading 3 then [4, 4, 4, 4, 4]
```

**What came back.** In every line where the whole second of the file's modification time stayed the same, all readings were 3, however far inside the second the time moved (up to 0.74 of a second forwards and 0.2 back). In every line where it changed, forwards or backwards, the four-vertex contents appeared: at the reading straight after the overwrite when the object was tagged, and at the reading after the first toggle when it was not. With the time moved a whole second and nothing toggled and nothing tagged, five readings in a row stayed at 3. No line went the other way. Every run in this entry was on macOS, on the APFS volume named above.

#### Whether these paths are matched letter for letter (5.3 list entry 7)

A folder was built as in entry 4, with `model.obj` and `tex.png` named in lower case on disk, and `scene.blend` saved beside them. The Path was set to `//model.obj`, `//MODEL.OBJ` and `//Model.Obj` in turn and the vertex count read after each. Then two new image data-blocks were made with their source set to a file and their paths set to `//tex.png` and `//TEX.PNG`, reloaded, and their pixels asked for. Last, `bpy.path.resolve_ncase` was asked about the absolute form of `//MODEL.OBJ`. Blender printed, in full:

```
Info: Saved as "scene.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/07/5.3
files actually on disk: ['model.mtl', 'model.obj', 'scene.blend', 'tex.png']
--- file actually on disk is named model.obj ---
PATH='//model.obj'    -> vertices: 3
PATH='//MODEL.OBJ'    -> vertices: 3
PATH='//Model.Obj'    -> vertices: 3
--- and the same question for an image data-block ---
IMAGE FILEPATH='//tex.png'  -> has_data: True size: (4, 4) pixels: 64
IMAGE FILEPATH='//TEX.PNG'  -> has_data: True size: (4, 4) pixels: 64
--- what bpy.path.resolve_ncase makes of the upper-case name ---
    /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/07/5.3/MODEL.OBJ
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: all three spellings of the `.obj` path found the file; the image with the upper-case path loaded; and `bpy.path.resolve_ncase` handed the upper-case name back as it was given. This was on macOS, on APFS as the Mac came, which is not set to tell letter case apart. It says nothing about Linux or Windows, neither of which ran it in 5.3.

#### Whether a render with no window reads the file, judged by the picture (5.3 list entry 8)

A second file, `render.blend`, was built holding the same Import OBJ group (without the image) on the cube, the factory camera made orthographic with a scale of 2, placed at (0.5, 0.5, 5) and looking straight down at the three-vertex shape, the Workbench engine, 64 by 64 pixels and PNG output. Frame 1 was rendered four times from the command line, each in a fresh Blender with no window (`-b render.blend -o //<name>_#### -F PNG -f 1`): with `model.obj` in place; with it moved away; with it still away and `--disable-depsgraph-on-file-load` before the file name; and with it back and that switch. The pictures were then loaded and compared colour value by colour value, not byte for byte, because Blender writes text including times into every saved picture. The four renders and the comparison, uncut:

```
00:00.325  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/render.blend"
00:00.331  render           | Rendering single frame
00:00.331  render           | Rendering frame 1
00:00.335  render           | Start rendering: Scene, ViewLayer
00:00.335  render           | Engine: Workbench
00:00.551  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/with_0001.png'
00:00.551  render           | Time: 00:00.21 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
00:00.351  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/render.blend"
00:00.356  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/model.obj'.
00:00.356  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.356  render           | Rendering single frame
00:00.356  render           | Rendering frame 1
00:00.356  render           | Start rendering: Scene, ViewLayer
00:00.356  render           | Engine: Workbench
00:00.525  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/without_0001.png'
00:00.525  render           | Time: 00:00.16 (Saving: 00:00.06)

Blender quit
00:00.328  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/render.blend"
00:00.333  render           | Rendering single frame
00:00.333  render           | Rendering frame 1
00:00.335  render           | Start rendering: Scene, ViewLayer
00:00.335  render           | Engine: Workbench
00:00.336  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/model.obj'.
00:00.336  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.505  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/nodeps_0001.png'
00:00.505  render           | Time: 00:00.17 (Saving: 00:00.06)

Blender quit
00:00.335  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/render.blend"
00:00.339  render           | Rendering single frame
00:00.339  render           | Rendering frame 1
00:00.340  render           | Start rendering: Scene, ViewLayer
00:00.340  render           | Engine: Workbench
00:00.514  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/nodepswith_0001.png'
00:00.514  render           | Time: 00:00.17 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
=== STEP 6: compare the pictures, colour value by colour value ===
--- with the file, against without it ---
picture A: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/with_0001.png size (64, 64) values 16384
picture B: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/without_0001.png size (64, 64) values 16384
RESULT: 1878 of 16384 colour values differ between the two pictures
mean value A: 0.4587   mean value B: 0.4223
--- with the file, against with the file and the depsgraph switch ---
picture A: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/with_0001.png size (64, 64) values 16384
picture B: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/nodepswith_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4587   mean value B: 0.4587
--- without the file, against without the file and the depsgraph switch ---
picture A: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/without_0001.png size (64, 64) values 16384
picture B: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/08/5.3/nodeps_0001.png size (64, 64) values 16384
RESULT: 0 of 16384 colour values differ between the two pictures
mean value A: 0.4223   mean value B: 0.4223
=== END 5.3.0 Beta ===
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: the picture rendered with the file differed from the one rendered without it in 1,878 of 16,384 colour values; the switch made no difference to either picture. With the file gone, the `ERROR` line came before `Rendering single frame` without the switch, and after `Engine: Workbench` with it. The count of differing values depends on this sitting's own camera and shape; the 1,878 is not a figure to set beside another version's unless the scene was the same.

#### Whether the file read can change from frame to frame (5.3 list entry 9)

Three shapes were put on disk — `data_1.obj` with three vertices, `data_2.obj` with four, `data_3.obj` with five — and `perframe.blend` saved beside them. A group on the cube joined three strings with a Join Strings node into Import OBJ's Path: a String node holding `//data_`, a Value to String node fed by the Scene Time node's Frame output, and a String node holding `.obj`; then Realize Instances → Join Geometry → Group Output. The three strings were linked into Join Strings in reading order first; the script was written so that if every frame gave 0, it would take those three links out, link them the other way round, and read again. The vertex count was read at frames 1, 2, 3 and 1. Blender printed, in full:

```
Info: Saved as "perframe.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/09/5.3
Value To String inputs: [('Value', 'NodeSocketFloat'), ('Decimals', 'NodeSocketInt'), ('Base', 'NodeSocketInt'), ('Padding', 'NodeSocketInt')]
Join Strings inputs: [('Delimiter', 'NodeSocketString', False), ('Strings', 'NodeSocketString', True)]
links made: 8
first form: linked in reading order (//data_, number, .obj) - order of the links into Strings, as Blender holds them: ['String.001', 'Value to String', 'String']
00:00.517  io.obj           | ERROR Cannot read from OBJ file:'.obj1//data_'.
00:00.517  reports          | ERROR OBJ Import: Cannot open file '.obj1//data_'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
frame 1 -> vertices: 0
00:00.517  io.obj           | ERROR Cannot read from OBJ file:'.obj2//data_'.
00:00.517  reports          | ERROR OBJ Import: Cannot open file '.obj2//data_'
frame 2 -> vertices: 0
00:00.517  io.obj           | ERROR Cannot read from OBJ file:'.obj3//data_'.
00:00.517  reports          | ERROR OBJ Import: Cannot open file '.obj3//data_'
frame 3 -> vertices: 0
frame 1 -> vertices: 0
second form: linked in reverse order (.obj, number, //data_) - order of the links into Strings, as Blender holds them: ['String', 'Value to String', 'String.001']
frame 1 -> vertices: 3
frame 2 -> vertices: 4
frame 3 -> vertices: 5
frame 1 -> vertices: 3
DONE

Blender quit
```

What came back: linked in reading order, Blender joined the strings the other way round, giving paths such as `.obj1//data_`, and every frame gave 0, with the frame number appearing in each failed path. Linked the other way round, frames 1, 2, 3 and 1 gave 3, 4, 5 and 3: a different file at each frame, worked out while the setup ran. Both forms happened in one run; the second carries the answer.

#### Whether an import node's path takes part in the file's own path housekeeping (5.3 list entry 10)

A folder was built as in entry 4, holding the Import OBJ node's path and an ordinary image path side by side. The image is the control: it is a kind of file Blender's own documentation marks as packable, so a run in which nothing moved can be told apart from one in which only the node's path failed to move. The script ran File ‣ External Data's commands from Python — Make Paths Relative starting from two absolute paths, Make Paths Absolute starting from two relative ones, Report Missing Files with both files present and then with both paths pointing at files that do not exist — and listed what `bpy.utils.blend_paths`, `bpy.data.file_path_map` and `bpy.data.file_path_foreach` find. Then it ran Pack Resources and saved `packed.blend`. Blender printed, in full:

```
Info: Saved as "scene.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3
--- Make Paths Relative, starting from two absolute paths ---
before                       import node Path = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/model.obj' image filepath = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '//model.obj'                        image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       import node Path = '//model.obj'                        image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        import node Path = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/model.obj' image filepath = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/tex.png'
--- Report Missing Files, with both files present ---
state                        import node Path = '//model.obj'                        image filepath = '//tex.png'
Info: No missing files
operator returned: {'FINISHED'}
--- Report Missing Files, with both files missing ---
state                        import node Path = '//gone-model.obj'                   image filepath = '//gone-tex.png'
00:00.372  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/gone-model.obj'.
00:00.372  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/gone-model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/gone-tex.png' not found, from local data-block 'IMtex.png'
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/gone-model.obj' not found, from local data-block 'NTNodeSetup'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
blend_paths(absolute=False): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/model.obj']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0x9a31d8008> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("NodeSetup") at 0x99fd86708> -> {'//model.obj'}
file_path_foreach visited 2 paths:
    (bpy.data.images['tex.png'], '//tex.png', <BlendDataPathMeta object at 0x11a7bce10>)
    (bpy.data.node_groups['NodeSetup'], '//model.obj', <BlendDataPathMeta object at 0x11a7bdbf0>)
--- Pack Resources, then save ---
Info: Packed 1 file(s)
operator returned: {'FINISHED'}
image packed after pack_all: True
import node Path after pack_all: '//model.obj'
Info: Saved as "packed.blend"
saved packed.blend
DONE

Blender quit
```

Then `packed.blend` was opened in a fresh Blender with both `model.obj` and `tex.png` moved away, and read as in entry 5:

```
########## packed.blend reopened with model.obj and tex.png moved away
00:00.336  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/packed.blend"
00:00.341  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/model.obj'.
00:00.341  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/10/5.3/model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
VERTICES: 0
PATH SOCKET: '//model.obj'
IMAGE: Render Result '' has_data: False size: (0, 0) packed: False
IMAGE: tex.png '//tex.png' has_data: False size: (4, 4) packed: True
IMAGE: Viewer Node '' has_data: False size: (256, 256) packed: False
DONE

Blender quit
```

What came back: Make Paths Relative and Make Paths Absolute each changed the import node's Path exactly as they changed the image's. Report Missing Files named the node's missing `.obj`, from the node group, beside the image. `blend_paths`, `file_path_map` and `file_path_foreach` each listed the node's path, under the node group. Pack Resources packed one file, the image; the node's Path still read `//model.obj`; and the saved file, opened with both outside files gone, gave 0 vertices and an `ERROR` naming `model.obj`, while its image was still packed.

#### A file path set on the modifier, through a group input (5.3 list entry 11)

This tests the seventh kind in the answer above, which it labels inferred. A folder held `model.obj` (three vertices), `model.mtl` and `tex.png`, and `scene.blend` was saved beside them. The cube's Geometry Nodes modifier used a group with a string input named File, made a file path by setting its subtype to `FILE_PATH`; inside the group that input was linked into Import OBJ's Path, then Realize Instances → Group Output, and an Image Texture node held the image as the control (as in entry 10). The modifier's own value for File was set to `//model.obj`, and the cube's vertex count read with the object marked for update. Then the script ran Make Paths Relative, Make Paths Absolute and Report Missing Files with both paths pointing at files that do not exist, and listed what `blend_paths`, `file_path_map` and `file_path_foreach` find.

**The first form of the script** set the modifier's value as `mod["Socket_2"] = "//model.obj"`, and Blender 5.3 refused it. Its run, in full:

```
Info: Saved as "scene.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3
group input: File NodeSocketStringFilePath subtype: FILE_PATH identifier: Socket_2 group's own default: ''
Import OBJ Path socket is_linked: True  its own value: ''
Traceback (most recent call last):
  File "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/scripts/11_modifier_path.py", line 30, in <module>
    mod[key] = "//model.obj"
    ~~~^^^^^
TypeError: bpy_struct[key] = val: id properties not supported for this type
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

Blender was then asked, in a separate run, what the modifier's `properties` member holds. It answered that `mod.properties` is a `GeometryNodesModifierInterface` with `inputs`, `outputs` and `panels`; that `mod.properties.inputs` has one member per group input, named by the input's identifier (`Socket_0`, `Socket_2`); and that each of those has a `value`. **The second form** sets and reads `mod.properties.inputs.Socket_2.value` instead, in a fresh folder, and is otherwise the same. Its run, in full:

```
Info: Saved as "scene.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second
group input: File NodeSocketStringFilePath subtype: FILE_PATH identifier: Socket_2 group's own default: ''
Import OBJ Path socket is_linked: True  its own value: ''
modifier's slot for the input: Socket_2  value property subtype: FILE_PATH
modifier value set to '//model.obj' -> vertices: 3
--- Make Paths Relative, starting from two absolute paths ---
before                       modifier File = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/model.obj' image filepath = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/tex.png'
Info: Total files 3 | Changed 1 | Failed 0
operator returned: {'FINISHED'}
after                        modifier File = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/model.obj' image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       modifier File = '//model.obj'                        image filepath = '//tex.png'
Info: Total files 3 | Changed 3 | Failed 0
operator returned: {'FINISHED'}
after                        modifier File = '//model.obj'                        image filepath = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/tex.png'
--- Report Missing Files, with both files missing ---
state                        modifier File = '//gone-model.obj'                   image filepath = '//gone-tex.png'
00:00.363  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/gone-model.obj'.
00:00.363  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/gone-model.obj'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
vertices now: 0
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/11/5.3-second/gone-tex.png' not found, from local data-block 'IMtex.png'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, both files present ---
vertices now: 3
blend_paths(absolute=False): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png']
file_path_map, entries with a non-empty set:
    <bpy_struct, Image("tex.png") at 0xbdb818008> -> {'//tex.png'}
file_path_foreach visited 1 paths:
    (bpy.data.images['tex.png'], '//tex.png')
DONE

Blender quit
```

What came back: with the file path set on the modifier and nothing typed into the node, the import read the file (3 vertices), and with it pointing at a missing file, 0. Make Paths Relative and Make Paths Absolute each changed the image's path and **left the modifier's value as it was**, and each counted one file fewer than entry 10's run (3 rather than 4). Report Missing Files named the missing image and **did not name the modifier's missing `.obj`**, though the import had just printed an `ERROR` for it. `blend_paths`, `file_path_map` and `file_path_foreach` listed the image and **not the modifier's path**. The modifier's value is held in a slot whose `value` is described as a file path (`FILE_PATH`).

#### Whether the built-in font points at a file (5.3 list entry 12)

`font.blend` was saved in a fresh folder, holding a group on the cube: String to Curves (the text "Ab") → Realize Instances → Fill Curve → Group Output, with nothing put into String to Curves' Font socket by the script. The script read the vertex count of the filled text, what the Font socket holds, every font in the file with its path, whether it is packed, its number of users and its library, and what `blend_paths` (relative, and absolute with packed data included) and `file_path_map` list. It read all that three times: as built; after adding a Font node and linking it into String to Curves' Font socket; and after saving and opening the file again in the same Blender. Blender printed, in full:

```
Info: Saved as "font.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/12/5.3
--- String to Curves with nothing in its Font socket ---
vertices of the filled text: 111
String to Curves Font socket value: <bpy_struct, VectorFont("Bfont Regular") at 0xcb48ade08>
fonts in bpy.data: [('Bfont Regular', '<builtin>', False, 1, None)]
blend_paths(absolute=False): ['//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
blend_paths(absolute=True, packed=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
file_path_map, entries with a non-empty set: []
Font node added; its ID properties: [('font', bpy.data.fonts['Bfont Regular'])]
--- String to Curves fed by a new Font node ---
vertices of the filled text: 111
String to Curves Font socket value: <bpy_struct, VectorFont("Bfont Regular") at 0xcb48ade08>
fonts in bpy.data: [('Bfont Regular', '<builtin>', False, 2, None)]
blend_paths(absolute=False): ['//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
blend_paths(absolute=True, packed=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
file_path_map, entries with a non-empty set: []
00:00.370  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/12/5.3/font.blend"
Info: Saved "font.blend"
--- after saving and opening the file again ---
vertices of the filled text: 111
String to Curves Font socket value: <bpy_struct, VectorFont("Bfont Regular") at 0xcb1a74020>
fonts in bpy.data: [('Bfont Regular', '<builtin>', False, 3, None)]
blend_paths(absolute=False): ['//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
blend_paths(absolute=True, packed=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
file_path_map, entries with a non-empty set: []
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: String to Curves' Font socket, which the script never set, already held a font data-block named `Bfont Regular`, and the filled text had 111 vertices, so a font was used. A new Font node held the same data-block. That font's path reads `<builtin>`; it is not packed and not linked. Neither `blend_paths`, even with packed data included, nor `file_path_map` listed it at any of the three readings; the only file either listed was one of Blender's own brush files, which an empty factory file already lists (entry 3).

#### What the Path row on an Import node shows in a window (5.3 list entry 13)

Blender was started **with** its window, from the terminal, opening the `scene.blend` entry 4 built, with a script that waited for the window to draw and then: spread the group's six nodes 400 units apart (every node had been made at the same spot); turned the largest area into a geometry node editor pinned to the group; ran View All and took Blender's own screenshot of the window; selected the Import OBJ node alone and ran View Selected; took a second screenshot; printed what the Path socket holds; and quit Blender. No mouse was moved and nothing was clicked.

**The first form** started Blender with `--factory-startup` and `--no-window-focus`. Both of its screenshots came back identical byte for byte: Blender's first-start *Quick Setup* panel (language, theme, keymap, and a Continue button) sat over the middle of the window, and the node editor had not been redrawn — only a sliver of the Import OBJ node showed at its right edge, behind the sidebar. It settles nothing and is recorded as it came back. **The second form** first saved preferences, with the splash screen turned off, into this sitting's own settings folder (Blender wrote them to `…/config/5.3/config/userpref.blend` and nowhere else), and then started Blender without `--factory-startup` and without `--no-window-focus`, so the window opened in front. Blender printed, in the second form:

```
00:00.584  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/04/5.3/scene.blend"
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
view_all returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/13/5.3-second/spread-all.png exists: True
view_selected returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/13/5.3-second/import-node.png exists: True
Path socket on the Import OBJ node holds: '//model.obj'
Path socket type: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
00:06.735  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/tmp/quit.blend"

Blender quit
```

**What the screenshots show.** The first, after View All, shows the Group Input and Group Output nodes small in the middle of the editor and the rest hidden behind the editor's sidebar, which was open on its Group tab; it settles nothing about the Path row. The second, after View Selected, shows the Import OBJ node large: a title bar reading **Import OBJ**, an **Instances** output with its socket on the node's right edge, and one row below it: a text field holding **`//model.obj`**, a **folder button** at that field's right-hand end, and a **socket on the node's left edge level with that row**, drawn as a small blue rectangle. The value in the field is the one the script read back. This was drawn on macOS only.

#### Where a bake goes when the Bake Path is left empty (5.3 list entry 14)

A file `bake.blend` was saved in a fresh folder, holding the factory cube renamed `Target` with a Geometry Nodes modifier named `GN` whose group was Group Input → Bake → Group Output. The script listed the bake commands the build offers with their parameters, printed the modifier's and the bake's settings, set the modifier's Bake Target to Disk with Bake Path left empty, listed the folder, baked with `bpy.ops.object.geometry_node_bake_single`, and listed every file under the folder. Then it set Bake Path to `//mybakes`, baked again and listed again.

**The first form of the script** added a Bake node and looked for its geometry socket to link, and found none. A Bake node made by a script starts with no items: Blender, asked separately, listed one unnamed input and one unnamed output of the type `NodeSocketVirtual` and an empty `bake_items`. The first form stopped there:

```
Traceback (most recent call last):
  File "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/scripts/14_bake_default.py", line 15, in <module>
    ng = build_bake_group("BakeSetup")
  File "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/scripts/nshelp.py", line 65, in build_bake_group
    geo_in = [s for s in bake.inputs if s.bl_idname == 'NodeSocketGeometry'][0]
             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^^
IndexError: list index out of range
```

**The second form** gives the Bake node one Geometry item (`bake.bake_items.new('GEOMETRY', "Geometry")`) before linking, and was run in a fresh folder. Blender printed, in full:

```
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/14/5.3-second
--- bake operators this build offers, with their parameters ---
   bpy.ops.object.geometry_node_bake_delete_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_pack_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_single(session_uid, modifier_name, bake_id)
   bpy.ops.object.geometry_node_bake_unpack_single(session_uid, modifier_name, bake_id, method)
Info: Saved as "bake.blend"
saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/14/5.3-second/bake.blend
scene frame range: 1 - 250
--- bake settings before baking ---
   modifier bake_target: PACKED
   modifier bake_directory: ''
   bake entry: bake_id=239350417 bake_target=INHERIT directory='' bake_mode=STILL use_custom_path=False node=Bake
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
   3 files under the work folder
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/239350417/blobs/00001_00000.blob
   file on disk: <workdir>/blendcache_bake/Target_GN/239350417/meta/00001_00000.json
--- now with an explicit bake directory ---
   operator returned: {'FINISHED'}
   modifier bake_directory: '//mybakes'
   5 files under the work folder
   file on disk: <workdir>/bake.blend
   file on disk: <workdir>/blendcache_bake/Target_GN/239350417/blobs/00001_00000.blob
   file on disk: <workdir>/blendcache_bake/Target_GN/239350417/meta/00001_00000.json
   file on disk: <workdir>/mybakes/239350417/blobs/00001_00000.blob
   file on disk: <workdir>/mybakes/239350417/meta/00001_00000.json
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: a new modifier held Bake Target `PACKED` and an empty Bake Path; its one bake held `INHERIT`, an empty path, Custom Path off, and the mode `STILL`. With Bake Target set to Disk and the path empty, Blender printed that the path was empty and it was setting a default one, baked, and filled the modifier's Bake Path in as `//blendcache_bake/Target_GN` — a folder named `blendcache_` plus the `.blend`'s own name, beside the `.blend`, then the object and modifier names. Inside it, a folder named with the bake's number held `meta/00001_00000.json` and `blobs/00001_00000.blob`. With Bake Path set to `//mybakes`, the second bake went to `mybakes/<bake number>/` with the same two files, and the first folder was left in place. The `<workdir>` in the file lines stands for the work folder named on the first line.

#### Whether a bake path takes part in the path housekeeping (5.3 list entry 15)

A file `bake.blend` was saved in a fresh folder beside a `tex.png`, holding the cube renamed `Target` with the Bake group of entry 14 (one Geometry item), an Image Texture node in the same group holding the image as the control, and the modifier's Bake Target set to Disk. The script ran Make Paths Relative and Make Paths Absolute with the modifier's Bake Path and the image's path side by side; the same two commands on the one bake's own path, with its Custom Path turned on; Report Missing Files with nothing baked yet, Bake Path naming a folder that does not exist and the image missing; `blend_paths` and `file_path_map` before and after baking to `//mybakes`; Report Missing Files after baking, and again with the baked folder renamed away; and Pack Resources, comparing the size of a copy saved just before it with one saved just after. Blender printed, in full:

```
Info: Saved as "bake.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3
--- Make Paths Relative, starting from two absolute paths ---
before                       modifier Bake Path = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/mybakes' image filepath = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/tex.png'
Info: Total files 4 | Changed 2 | Failed 0
operator returned: {'FINISHED'}
after                        modifier Bake Path = '//mybakes'                          image filepath = '//tex.png'
--- Make Paths Absolute, starting from two relative paths ---
before                       modifier Bake Path = '//mybakes'                          image filepath = '//tex.png'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
after                        modifier Bake Path = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/mybakes' image filepath = '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/tex.png'
--- the same pair of commands on a single bake's own path (Custom Path on) ---
bake entry directory before Make Paths Relative: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/onebake'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
bake entry directory after Make Paths Relative: '//onebake'
Info: Total files 4 | Changed 4 | Failed 0
operator returned: {'FINISHED'}
bake entry directory after Make Paths Absolute: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/onebake'
--- Report Missing Files, nothing baked yet, Bake Path naming a folder that does not exist, image missing ---
state                        modifier Bake Path = '//gone-bakes'                       image filepath = '//gone-tex.png'
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/gone-bakes' not found, from local data-block 'OBTarget'
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/gone-tex.png' not found, from local data-block 'IMtex.png'
operator returned: {'FINISHED'}
--- bpy.utils.blend_paths and bpy.data.file_path_map, before baking ---
state                        modifier Bake Path = '//mybakes'                          image filepath = '//tex.png'
blend_paths(absolute=False): ['//mybakes', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png']
file_path_map, entries with a non-empty set: [(bpy.data.objects['Target'], {'//mybakes'}), (bpy.data.images['tex.png'], {'//tex.png'})]
--- bake to //mybakes, then the same readings ---
bake operator returned: {'FINISHED'}
files under the work folder: 4 first few: ['<workdir>/bake.blend', '<workdir>/tex.png', '<workdir>/mybakes/19029844/blobs/00001_00000.blob', '<workdir>/mybakes/19029844/meta/00001_00000.json']
blend_paths(absolute=False): ['//mybakes', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png']
file_path_map, entries with a non-empty set: [(bpy.data.objects['Target'], {'//mybakes'}), (bpy.data.images['tex.png'], {'//tex.png'})]
--- Report Missing Files after baking, then with the baked folder renamed away ---
Info: No missing files
operator returned: {'FINISHED'}
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/15/5.3/mybakes' not found, from local data-block 'OBTarget'
operator returned: {'FINISHED'}
--- Pack Resources, then save, and the size of the saved file against an unpacked save ---
Info: Saved copy as "unpacked.blend"
Info: Packed 1 file(s)
operator returned: {'FINISHED'}
image packed after pack_all: True
modifier Bake Path after pack_all: '//mybakes' ; bake_target: DISK
Info: Saved copy as "packed.blend"
bytes: unpacked.blend 96987  packed.blend 97824  tex.png 75  baked files total 2425
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: Make Paths Relative and Make Paths Absolute each changed the modifier's Bake Path as they changed the image's, and each changed the one bake's own Custom Path too. Report Missing Files named the Bake Path's missing folder, from the object (`OBTarget`), both before anything was baked and after the baked folder was renamed away; with the folder in place it reported nothing missing. `blend_paths` and `file_path_map` listed `//mybakes`, under the object, before and after baking. Pack Resources packed one file, the image; Bake Path and Bake Target were unchanged; the file saved after it was 837 bytes larger than the one saved before it, against an image of 75 bytes and baked files of 2,425 bytes in all.

#### Whether a bake path can carry a template (5.3 list entry 16)

A file `t.blend` was saved in a fresh folder, holding the cube renamed `Target` with the Bake group of entry 14. The script set the modifier's Bake Target to Disk and its Bake Path to `//bakes/{blend_name}/`, baked, read the Bake Path again, and listed every folder under the work folder. Blender printed, in full:

```
Info: Saved as "t.blend"
BLENDER: 5.3.0 Beta WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/16/5.3 saved as: t.blend
modifier bake_target: DISK  bake_directory set to: '//bakes/{blend_name}/'
bake operator returned: {'FINISHED'}
modifier bake_directory after baking: '//bakes/{blend_name}/'
folders under the work folder:
    <workdir> (1 files)
    <workdir>/bakes (0 files)
    <workdir>/bakes/t (0 files)
    <workdir>/bakes/t/1355244011 (0 files)
    <workdir>/bakes/t/1355244011/blobs (1 files)
    <workdir>/bakes/t/1355244011/meta (1 files)
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: the bake went to a folder named `t` — the `.blend`'s name without its extension — under `bakes`, and no folder named `{blend_name}` appeared. The Bake Path kept the template text `//bakes/{blend_name}/` as typed after the bake.

#### What the modifier's bake settings show in a window on a new modifier (5.3 list entry 17)

A file `simzone.blend` was built with no window: the factory cube with a Geometry Nodes modifier `GN` whose group was Group Input → Simulation Input → Simulation Output → Group Output, the two simulation nodes paired, and nothing on the modifier changed after it was added. Blender printed, in full:

```
pair_with_output returned: True
simulation input outputs: [('Delta Time', 'NodeSocketFloat'), ('Geometry', 'NodeSocketGeometry'), ('', 'NodeSocketVirtual')]
links: [('Group Input', 'Geometry', 'Simulation Input', 'Geometry'), ('Simulation Input', 'Geometry', 'Simulation Output', 'Geometry'), ('Simulation Output', 'Geometry', 'Group Output', 'Geometry')]
Info: Saved as "simzone.blend"
BLENDER: 5.3.0 Beta saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/17/5.3/simzone.blend
--- what the new modifier holds, untouched ---
modifier bake_target: PACKED  bake_directory: ''
bake entry: bake_id=1621785217 bake_target=INHERIT directory='' bake_mode=STILL use_custom_path=False node=Simulation Output
modifier open_manage_panel: False
modifier open_bake_panel: False
modifier show_manage_panel: True
modifier open_bake_data_blocks_panel: False
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

Then Blender was started **with** its window on that file (as entry 13's second form: this sitting's own preferences, window in front), with a script that turned the largest area into the Properties editor on the Modifier tab and took Blender's own screenshot; then set the modifier's `open_manage_panel` and `open_bake_panel` switches to True and took a second. Nothing was clicked.

**The first form** took each screenshot one second after its change. Its first screenshot still showed the 3D viewport the area had been, and its second showed the Properties editor with the modifier's **Manage** section collapsed, the switches having been set. It is recorded as it came back. **The second form** asked every area to redraw and waited before each screenshot, and read the switches back after each. It printed:

```
00:00.863  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/17/5.3/simzone.blend"
active object: Cube  properties tab: MODIFIER
switches as the file holds them: open_manage_panel=False open_bake_panel=False
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/17/5.3/shots-second/modifier-as-opened.png exists: True
switches after the first drawing: open_manage_panel=False open_bake_panel=False
switches set to: open_manage_panel=True open_bake_panel=True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/17/5.3/shots-second/modifier-manage-open.png exists: True
switches after the second drawing: open_manage_panel=True open_bake_panel=True
modifier bake_target: PACKED  bake_directory: ''
00:08.539  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/tmp/quit.blend"

Blender quit
```

**What the screenshots show.** The second form's first screenshot again still showed the 3D viewport; it settles nothing. Its second shows the Properties editor on the Modifier tab: the modifier `GN` with its group `SimSetup`, then a section **Manage**, open, holding a section **Bake**, open, with two rows — **Bake Target** reading **Packed** in a dropdown, and **Bake Path** an **empty** text field with a folder button at its right end — and below them a collapsed section **Named Attributes**. Nothing on the modifier had been changed but the two open-and-shut switches. This was drawn on macOS only.

#### Which node `.blend` files this Blender ships (5.3 list entry 25)

Blender was started with no window and a script that printed where the program sits, checked the folder beside the program that an earlier sitting's check had looked in, asked Blender for its own resource folder (`bpy.utils.resource_path('LOCAL')`), listed every `.blend` under that folder's `datafiles/assets`, and then opened each shipped file under `nodes/` with `bpy.data.libraries.load` — linking nothing — to count its node groups and see whether one is named `Smooth by Angle`. Blender printed, in full:

```
BLENDER: 5.3.0 Beta
binary path: /Applications/Blender 5.3b.app/Contents/MacOS/Blender
the folder the earlier check looked in: /Applications/Blender 5.3b.app/Contents/MacOS/5.3/datafiles/assets
   exists: False
bpy.utils.resource_path('LOCAL'): /Applications/Blender 5.3b.app/Contents/Resources/5.3
assets folder: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets exists: True
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-curve_sculpt.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_sculpt.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_vertex.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_weight.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-mesh_sculpt.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-mesh_texture.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-mesh_vertex.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-mesh_weight.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/compositing_nodes_essentials.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_dynamics_assets.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/principal_components.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/procedural_hair_node_assets.blend
   shipped .blend: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/shading_nodes_essentials.blend
count of shipped .blend files under the assets folder: 15
--- which shipped node file holds a node group named Smooth by Angle ---
   compositing_nodes_essentials.blend: 25 node groups; 'Smooth by Angle' in it: False
   geometry_nodes_dynamics_assets.blend: 55 node groups; 'Smooth by Angle' in it: False
   geometry_nodes_essentials.blend: 33 node groups; 'Smooth by Angle' in it: True
   principal_components.blend: 2 node groups; 'Smooth by Angle' in it: False
   procedural_hair_node_assets.blend: 32 node groups; 'Smooth by Angle' in it: False
   shading_nodes_essentials.blend: 4 node groups; 'Smooth by Angle' in it: False
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: fifteen `.blend` files ship inside the application, nine brush files and six node files under `datafiles/assets/nodes/`. The node group `Smooth by Angle` is in `geometry_nodes_essentials.blend`, one of 33 groups there, and in none of the other five.

#### What adding Smooth by Angle records in the file (5.3 list entry 26)

In a Blender with no window and factory settings, the factory cube was made the only selected object and the active one, and Shade Auto Smooth was run with no arguments (`bpy.ops.object.shade_auto_smooth()`; the build lists its parameters as `use_auto_smooth` and `angle`). Before and after, a script printed the modifiers and their groups; each node group with its library, whether it is linked-packed, whether it carries asset data, and its weak reference; the libraries in the file; `blend_paths()` and `blend_paths(absolute=True, packed=True)`; and `file_path_map(include_libraries=True)`. The file was saved as `smooth.blend` and opened in a fresh Blender, which printed the same readings. The researcher asked for the Outliner in Blender File mode; that view lists the file's libraries, which is the `libraries:` line here. Both runs, in full:

```
BLENDER: 5.3.0 Beta MODE: add
--- before ---
libraries: []
blend_paths(): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
blend_paths(absolute=True, packed=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
file_path_map(include_libraries=True), entries with a non-empty set: []
shade_auto_smooth parameters: ['use_auto_smooth', 'angle']
operator returned: {'FINISHED'}
--- after Shade Auto Smooth ---
modifier on Cube: Smooth by Angle type NODES node group: Smooth by Angle
node group: 'Smooth by Angle' library: /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend is_linked_packed: True asset_data: True weak reference: None
libraries: [('geometry_nodes_essentials.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', False), ('geometry_nodes_essentials.blend.001', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', True)]
blend_paths(): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend']
blend_paths(absolute=True, packed=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend']
file_path_map(include_libraries=True), entries with a non-empty set: [(bpy.data.node_groups['Smooth by Angle'], {'/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend'}), (bpy.data.libraries['geometry_nodes_essentials.blend'], {'/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend'}), (bpy.data.libraries['geometry_nodes_essentials.blend.001'], {'/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend'})]
Info: Saved as "smooth.blend"
saved smooth.blend
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
00:00.326  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/26/5.3/smooth.blend"
BLENDER: 5.3.0 Beta MODE: reopen
--- smooth.blend opened in a fresh Blender ---
modifier on Cube: Smooth by Angle type NODES node group: Smooth by Angle
node group: 'Smooth by Angle' library: //../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend is_linked_packed: True asset_data: True weak reference: None
libraries: [('geometry_nodes_essentials.blend', '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', False), ('geometry_nodes_essentials.blend.001', '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', True)]
blend_paths(): ['//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend']
blend_paths(absolute=True, packed=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend']
file_path_map(include_libraries=True), entries with a non-empty set: [(bpy.data.node_groups['Smooth by Angle'], {'//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend'}), (bpy.data.libraries['geometry_nodes_essentials.blend'], {'//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend'}), (bpy.data.libraries['geometry_nodes_essentials.blend.001'], {'//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend'})]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

Last, the saved `smooth.blend` was copied into a folder two levels deeper (`…/26/5.3/one/two/`), so that its relative library path no longer leads to the application, and both the copy and the original were opened in a fresh Blender with no window and a script that printed each library's path made absolute and whether that exists, the group's library and state and node count, the modifier's group, the cube's evaluated vertex count, and Report Missing Files:

```
=== a copy of smooth.blend opened from <work>/26/5.3/one/two/ ===
00:00.329  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/26/5.3/one/two/smooth.blend"
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/26/5.3/one/two/smooth.blend
library: geometry_nodes_essentials.blend '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend' is_archive=False -> absolute /private/tmp/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend exists: False packed_file: False
library: geometry_nodes_essentials.blend.001 '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend' is_archive=True -> absolute /private/tmp/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend exists: False packed_file: False
node group: 'Smooth by Angle' library: geometry_nodes_essentials.blend.001 is_missing: False is_linked_packed: True nodes: 14
modifier: Smooth by Angle node group: Smooth by Angle
evaluated vertices of the cube: 8
Warning: Path '/private/tmp/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend' not found, from local data-block 'LIgeometry_nodes_essentials.blend'
Warning: Path '/private/tmp/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend' not found, from local data-block 'LIgeometry_nodes_essentials.blend.001'
Report Missing Files returned: {'FINISHED'}
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
=== the saved smooth.blend opened where it was saved, read the same way ===
00:00.331  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/26/5.3/smooth.blend"
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/26/5.3/smooth.blend
library: geometry_nodes_essentials.blend '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend' is_archive=False -> absolute /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend exists: True packed_file: False
library: geometry_nodes_essentials.blend.001 '//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend' is_archive=True -> absolute /Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend exists: True packed_file: False
node group: 'Smooth by Angle' library: geometry_nodes_essentials.blend.001 is_missing: False is_linked_packed: True nodes: 14
modifier: Smooth by Angle node group: Smooth by Angle
evaluated vertices of the cube: 8
Info: No missing files
Report Missing Files returned: {'FINISHED'}
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: Shade Auto Smooth added a Geometry Nodes modifier named **Smooth by Angle** using a node group **Smooth by Angle** that is **linked from Blender's own `geometry_nodes_essentials.blend`, inside the application, and packed** (`is_linked_packed: True`), with asset data and no weak reference. The file then records **two libraries**, both naming that bundled file: `geometry_nodes_essentials.blend`, and `geometry_nodes_essentials.blend.001`, which is an archive (`is_archive` True) and is the one the group belongs to. Before saving, both were recorded by absolute path; **after saving, the file records them relative to itself**, as `//../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/nodes/geometry_nodes_essentials.blend`, and `blend_paths` and `file_path_map` list that file. Opened from a folder where that relative path leads nowhere, Blender printed no loading warning, the group was present, not missing, linked-packed and holding its 14 nodes, and the modifier still used it; Report Missing Files named both library paths as not found. Opened where it was saved, Report Missing Files found nothing missing.

---

## 2. When a `.blend` file links a node setup from another `.blend` file, what does the linking file record about the files that setup uses, and how are they reached?

**Ambiguity.** This question has two readings. "Links a node setup" may mean linking just the node group, then using it in a modifier that lives in the linking file. Or it may mean linking an object or material that carries the setup with it. "Node setup" may also mean any node setup, not only geometry nodes. I answer the general linking behaviour, which covers all readings, and mark the parts that apply to only one.

**Short answer.**
- The linking file records the source `.blend` (the "library") and its path, relative or absolute.
- What the linked setup uses comes along as linked data from that library, not as copies. Paths written on that data are read relative to the library file, not the linking file.
- If the library itself links from further files, those become libraries too, with paths that may be relative to the library that named them.
- The linking file can instead pack the linked data inside itself.
- The sources do not say whether the linking file stores its own copy of the image, font, sound or import paths the setup uses.

### 2.1 The linking file records the library and its path — documented

- "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."
- "They’re also listed in the Outliner’s Blender File Display Mode, along with the path of the blend-file they originate from."
- Option "Relative Path: Reference the external blend-file using a relative path rather than an absolute one."
  - Manual — https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html — read 2026-10-11
- Library: "External .blend file from which data is linked"; filepath "Path to the library .blend file. … (default “”, never None, blend relative // prefix supported)". — API — https://docs.blender.org/api/5.3/bpy.types.Library.html
- Each linked data-block: library "Library file the data-block is linked from". — API — https://docs.blender.org/api/5.3/bpy.types.ID.html
- Import option: "Make paths of used library blendfiles relative to current blendfile." — API — https://docs.blender.org/api/5.3/bpy.types.BlendImportContext.html
- How a relative path is reached: "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file." — Manual — https://docs.blender.org/manual/en/5.3/files/blend/open_save.html
- All read 2026-10-11.

### 2.2 What the linked setup uses comes along as linked data — inferred

- On Append: "Localize All: Also copy all indirectly linked data, instead of maintaining the links." — Manual, link/append page above.
- "Is this ID block linked indirectly" — API, `ID.is_library_indirect` — https://docs.blender.org/api/5.3/bpy.types.ID.html
- "NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local." — API, BlendImportContext page above.
- All read 2026-10-11.
- Step: Blender separates data you link directly from data that data uses, which is "indirectly linked". Linking keeps those links instead of copying. So an image, font, sound or inner node group used by the linked setup arrives as indirectly linked data from the library, still carrying the path written in the library. No source says this in one sentence for node setups.
- If the library cannot be found: "it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored." — Manual, link/append page above. Step (inferred): the linking file therefore records which linked data-blocks it uses, not only the library path.

### 2.3 Libraries used by the library — documented for the listing, inferred for loading

- "A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file." — API — https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html — read 2026-10-11
- Library also has a "parent" property, with no description (API, Library page).
- Step (inferred): suppose the setup's file itself links something from a third file. Then the linking file depends on that third file too. Blender reaches it through the path the middle file recorded, which may be relative to the middle file.

### 2.4 Paths on linked data are read relative to the library — documented, observed and inferred

- "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file." Its example is an image path: `bpy.path.abspath(image.filepath, library=image.library)`. — API — https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html — read 2026-10-11
- "library … The library this path is from. This is only included for convenience, when the library is not None its path replaces start." — API — https://docs.blender.org/api/5.3/bpy.path.html — read 2026-10-11
- Path templates have a variable for this: "blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from." — Manual — https://docs.blender.org/manual/en/5.3/files/file_paths.html — read 2026-10-11

**Observed in Cycles' own files**, for shader node setups only: Cycles' Script node update routine passes the node tree library to `bpy.path.abspath`.
- `cycles_5.3/osl.py`, line 280, in `update_script_node`: `ok, oso_path, oso_file_remove = update_external_script(report, node.filepath, node.id_data.library)`
- `cycles_5.3/osl.py`, line 199, in `update_external_script` (defined at line 192): `script_path = bpy.path.abspath(filepath, library=library)`
- `cycles_5.3/osl.py`, line 239, for a script held in a text data-block: `osl_path = bpy.path.abspath(script.filepath, library=script.library)`
- `cycles_5.3/__init__.py`, lines 103–106: the render engine's `update_script_node` calls `osl.update_script_node(node, self.report)` when OSL is available.
- Read 2026-10-16.
- Inferred from this: when a linked shader setup holds a Script node with a relative external script path, Cycles looks for that script beside the library file, not beside the linking file. This is what the code says. I did not run it.

Not covered here: whether geometry nodes resolve a relative import-node Path, inside a linked node group, against the library or against the open file. The general statement above covers "Blender data from linked libraries", but no source names import nodes or node sockets. See the hand tests. *(Hand test, 2026-10-18: observed on the Creator's Mac — see "Which folder a `//` inside a linked group is measured from", 5.3 list entry 18, under question 2's observed section below.)*

### 2.5 What the linking file itself stores about those files — not in the sources

- No source says whether the linking file keeps its own copy of an image, font, sound or import path used by linked data, or reads them from the library each time it loads.
- "Returns a list of paths to external files referenced by the loaded .blend file."; "local (bool) – When true skip linked library paths." — API — https://docs.blender.org/api/5.3/bpy.utils.html — read 2026-10-11. This shows Blender can list paths that come from linked data. It does not say where they are stored. "linked library paths" is also ambiguous: it could mean the library `.blend` paths themselves, or the paths found on linked data.
- *(Hand test, 2026-10-18: what the linking file holds once the library is gone, and what each `blend_paths` call lists, were observed on the Creator's Mac — 5.3 list entry 18, below.)*

### 2.6 Packing the linked setup into the linking file — documented

- Asset Browser, Pack: "Imports the asset as linked data and immediately packs it into the current blend-file." — Manual — https://docs.blender.org/manual/en/5.3/editors/asset_browser.html
- "Pack Linked Libraries … Mark all linked library files in the current blend-file as packed. Actual packing will happen on the next save of the blend-file." — Manual — https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html
- "Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again" — API — https://docs.blender.org/api/5.3/bpy.ops.file.html
- "pack (bool) – If True, and link is also True, pack linked data-blocks into the current blend-file." — API, BlendDataLibraries page above.
- "This library is an ‘archive’ storage for packed linked IDs originally linked from its ‘archive parent’ library." (Library); "Hash representing a unique version of a packed linked data and all of its dependencies" (`ID.deep_hash`). — API pages above.
- All read 2026-10-11.
- Release note: "A linked packed ID is now considered directly linked if it's used by a local ID, _or by any other packed ID also directly linked_". Also: "code writing linked data now only writes directly linked packed IDs". — https://developer.blender.org/docs/release_notes/5.3/core/ — read 2026-10-11
- Not covered: whether packing the linked setup also packs the outside files it uses (images, fonts, sounds, import-node files, bake data). The sources speak only of data-blocks.

### 2.7 A linked node group used by a modifier in the linking file — inferred

- The modifier's inputs "can have unique values even if the group is shared among multiple modifiers." — Manual, modifier page.
- The bakes and bake path are on the modifier (see 1.6). Each bake's node "may be deeply nested in the modifier node group. It can be none in some cases like missing linked data blocks." — API, NodesModifierBake page.
- A baked data-block reference records its library: lib_name "Used when the data block is not local to the current .blend file but is linked from some library". — API — https://docs.blender.org/api/5.3/bpy.types.NodesModifierDataBlock.html
- All read 2026-10-11.
- Step: when only the node group is linked, the modifier is local. So its input values (file paths, images, fonts, objects) and its bake folder path are recorded in the linking file, and a `//` path there is relative to the linking file (by the general rule in 2.1). The sources do not walk through this case.

### 2.8 How missing or moved files are reached again — documented

- Relocate: "This can be used to either fix a broken linked library (e.g. because the file was moved or renamed)". — link/append page.
- Save As: "Remap Relative: This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location." — https://docs.blender.org/manual/en/5.3/files/blend/open_save.html
- Both read 2026-10-11.

**Not covered.**
- Whether the linking file stores copies of the dependency paths (2.5).
- How geometry nodes resolve relative import paths and bake paths that sit inside linked data.
- Whether packing a linked setup brings its outside files along.
- Whether Essentials node groups are linked from a bundled file.
- What `Library.parent` holds.

*(Note added by the hand tests, 2026-10-18. Every line in this list stays true of the **sources**. Each was afterwards observed on the Creator's Mac, in the section "Observed by hand test on the Creator's Mac — 2026-10-18" below: what the linking file keeps of the paths its linked data uses, and how a relative import path inside linked data is reached, 5.3 list entry 18; where a bake made through a linked group goes, entry 22; whether packing a linked setup brings its outside files, entry 19; what `Library.parent` holds, entry 20. Whether Essentials node groups are linked from a bundled file is observed under question 1, entry 26. Read this list as "the documentation does not say", not as "nobody knows".)*

**Where I looked.**
- Manual searches across all of manual_5.3: "indirect", "relative path(s)", "make paths", "library file", "linked librar", "linked data", "relative to the library", "library's", "placeholder", "Essentials", "import method", "reuse data", "project_root", and the words "Windows", "macOS", "Linux", "different drive".
- Manual pages read: link/append, linked libraries index, packed data, open/save, file paths, File menu (topbar), Asset Browser, asset library preferences, save/load preferences, file path preferences, asset library introduction, data-blocks.
- API pages read: Library, LibraryWeakReference, PackedFile, ID, BlendDataLibraries, BlendImportContext, `bpy.path`, `bpy.utils`, `bpy.ops.file`, `bpy.ops.wm` (link and append), the file paths gotchas page, ShaderNodeScript, RenderEngine. I also searched the API for "indirect" and for "relative to the library".
- Release notes: core, assets, python_api, pipeline_io, geometry_nodes.
- Cycles files: searched for "library" and read `osl.py` and `__init__.py` around the hits.
- Command line page: searched for "librar", "relative", "pack", "missing", "path". It has nothing on linking beyond a library-override resync switch.

**Outside the interface.** Linking can be driven from Python. The defaults differ, which is documented in the signatures; the consequence below is inferred.
- The link operator defaults to a relative library path: `bpy.ops.wm.link(…, relative_path=True, …)`, "Select the file relative to the blend file". — https://docs.blender.org/api/5.3/bpy.ops.wm.html
- The data-level call defaults the other way: `load(filepath, *, link=False, pack=False, relative=False, …)`, with "relative (bool) – When True the path is stored relative to the open blend file." — https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html
- Step: a script that links through `bpy.data.libraries.load` without asking for `relative` gets an absolute library path, unlike the interface.
- The preference "Default value for Relative Paths when loading external files such as images, sounds, and linked libraries." (https://docs.blender.org/manual/en/5.3/editors/preferences/save_load.html) is not said to affect Python calls.
- Scripts reading paths on linked data must pass the library themselves: "When the data block may be from an external blend-file pass the library argument from the bpy.types.ID." (gotchas page).
- The sources do not say whether loading or path resolution differs in background mode.
- All read 2026-10-11.

**Brought up to date by hand test, 2026-10-18 — observed, on the Creator's Mac only.** Linking, packing the library, a chain of libraries, both ways of linking from Python, opening with a library missing, and baking through a linked group were all done and read from scripts in a Blender started from the terminal with no window (entries 18 to 22). `bpy.data.libraries.load` with no `relative` stored an absolute path, and with `relative=True`, like the link command's own default, a relative one (entry 21). With the library gone, Blender printed warnings naming it and the missing group, kept the group as missing, and exited with code 0 (entry 18). Writing into a linked node's Path from a script was accepted, while the window draws that field greyed out (entries 18 and 23). The two window tests, entries 23 and 24, were run by opening Blender's window from the terminal and taking Blender's own screenshots, with nothing clicked.

**Operating system.** Not in the sources. The only drive remark found ("except when on a different drive on Windows") belongs to the OBJ and FBX exporters' path options, not to linking.

**Brought up to date by hand test, 2026-10-18 — observed.** Everything observed below was run on **the Creator's Mac, macOS 26.6.2, on its APFS volume as the Mac came**, and holds for that machine only; letter case there is entry 7, under question 1. **Nothing was run on Linux or Windows in 5.3**: no such machine was available to this sitting. That is untried, not untestable — see the equipment section of `Preparation/Blender Research/Method.md` — and a Windows answer must name its edition, since every Windows machine this project has read reported a Server edition. One result turns on where files sit: a file using Smooth by Angle records Blender's own bundled file by a path relative to itself, reaching into the application folder of the Blender that saved it (entry 26, under question 1).

### Observed by hand test on the Creator's Mac — 2026-10-18

**LABEL: observed.**

Same machine, same date, same Blender, same settings and work folders as the observed section under question 1: **the Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83)**, `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`. The set-ups, the scripts and the exact commands are in `Audits/Blender Research/Node setups/Hand tests — 5.3.md`, under the entry numbers used below, which are that list's own. Nothing here was run on Linux or Windows.

Entries 18 to 24 share one shape. Two folders sit side by side in a work folder. `lib/` holds `lib.blend`, a three-vertex `model.obj` (with `mtllib model.mtl` and `usemtl red`), `model.mtl` and a four-by-four `tex.png`; inside `lib.blend` a node group `LibSetup` (kept with a fake user) is Import OBJ, Path `//model.obj` → Realize Instances → Join Geometry → Group Output, with an Image Texture node holding `tex.png`, whose path is `//tex.png`, connected to nothing. `shot/` holds `use.blend`, which was saved first and then linked `LibSetup` with `bpy.data.libraries.load(<lib.blend>, link=True, relative=True)` and put it on the factory cube through a Geometry Nodes modifier `GN`. Building the two files printed:

```
BLENDER: 5.3.0 Beta STEP: lib WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3
Info: Saved as "lib.blend"
Info: Saved "lib.blend"
saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend  LibSetup Path: '//model.obj'  image: '//tex.png'
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta STEP: shot WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3
Info: Saved as "use.blend"
Info: Saved "use.blend"
saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend  libraries: [('lib.blend', '//../lib/lib.blend')]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

#### Which folder a `//` inside a linked group is measured from, and what the linking file records (5.3 list entry 18)

`use.blend` was then opened four times, each in a fresh Blender with no window and the file named on the command line: with no `model.obj` beside it; with a **different**, four-vertex `model.obj` put beside it; with `lib/model.obj` moved away and the four-vertex copy still beside `use.blend`; and with the whole `lib` folder moved away (each move undone afterwards). Each time a script read the cube's vertex count; the libraries recorded, with path, parent, archive flag and version; the node groups with their library, whether indirectly linked and whether missing, and the Path; every image with its library and state, then its pixels and its path made absolute against its own library; `bpy.utils.blend_paths` five ways (absolute, with `local` off and on — the first two lines — then no arguments, `local=True` and `absolute=True`); `bpy.data.file_path_map` without and with `include_libraries=True`; and Report Missing Files. Last, it wrote `//written-from-linking-file.obj` into the linked node's Path and read the vertex count again; the file was not saved, so that write does not carry into the next open. The four opens, uncut:

```
=== STEP 3: open shot/use.blend with NO model.obj beside it ===
00:00.327  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.330  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x86dd87620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x86dd87620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj', '//../lib/lib.blend']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../lib/lib.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x86ad53820> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x86b382da0> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x86dd87620> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x86ad53820> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x86b382da0> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0x86dd87620> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
00:00.366  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'.
00:00.366  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 4: put a different model.obj (4 vertices) beside use.blend, open again ===
00:00.327  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.331  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xb833b7620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xb833b7620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj', '//../lib/lib.blend']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../lib/lib.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xb8829e020> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb871edc20> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xb833b7620> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xb8829e020> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb871edc20> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xb833b7620> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
00:00.365  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'.
00:00.365  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 5: same, but with lib/model.obj moved away (shot/model.obj still there) ===
00:00.325  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.328  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.332  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj'.
00:00.332  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj'
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xbe90b9820> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xbe90b9820> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj', '//../lib/lib.blend']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../lib/lib.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xbee368020> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xbee28a6a0> -> {'//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xbe90b9820> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xbee368020> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xbee28a6a0> -> {'//../lib/lib.blend', '//model.obj'}
    <bpy_struct, Library("lib.blend") at 0xbe90b9820> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
00:00.361  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'.
00:00.361  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP 6: same, but with the whole lib folder moved away ===
00:00.327  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.331  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.331  reports          | WARNING Unable to open '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend': No such file or directory
00:00.331  reports          | Cannot find lib '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend'
00:00.332  reports          | LIB: NodeTree: 'LibSetup' missing from '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', parent '<direct>'
00:00.332  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend
VERTICES: 8
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x88df8cc20> indirect: False missing: True
     Path socket unreadable: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../lib/lib.blend']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//../lib/lib.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend']
--- bpy.data.file_path_map() ---
    <bpy_struct, Library("lib.blend") at 0x88df8cc20> -> {'//../lib/lib.blend'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, NodeTree("LibSetup") at 0x891485808> -> {'//../lib/lib.blend'}
    <bpy_struct, Library("lib.blend") at 0x88df8cc20> -> {'//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend' not found, from local data-block 'LIlib.blend'
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write refused: KeyError 'bpy_prop_collection[key]: key "Import OBJ" not found'
DONE

Blender quit
exit code: 0
```

What came back: with the library in place the count was 3 whether or not a four-vertex `model.obj` sat beside `use.blend`; with `lib/model.obj` gone and the decoy still beside `use.blend` it was 0, with an `ERROR` naming `lib/model.obj`. The linking file records the library as `//../lib/lib.blend`, relative to itself, with no parent. The node group is linked directly, the image indirectly, both from `lib.blend`; the image loaded, and its `//tex.png` made absolute against its library points into `lib/`. `blend_paths` with `local=True` dropped the image and the `.obj` and kept `lib.blend`; with no arguments it gave the image and the `.obj` as `//tex.png` and `//model.obj`, as written in the library, and the library as `//../lib/lib.blend`. `file_path_map` gave each linked data-block its own path, and with `include_libraries=True` added the library's path to each. Report Missing Files named the linked `.obj` as "from linked data-block 'NTLibSetup' (from library …lib.blend)" when it was gone. Writing a new value into the linked node's Path from Python was accepted, and Blender then looked for it in `lib/`. With the whole `lib` folder gone, Blender printed four lines naming the missing library and the missing group, the group was marked missing and its nodes were not there to read, the only path left anywhere was the library's own, Report Missing Files named `lib.blend`, the exit code was 0, and the count was 8.

The 8 in the last open is this script's own reading of the cube with its group gone; a reader setting it beside another version's figure should first check that both read the same object the same way (the list named above, under *Controls in 5.2.2*).

#### What Pack Linked Libraries carries, and what it leaves behind (5.3 list entry 19)

`use.blend` from entry 18 — with the four-vertex decoy `model.obj` from entry 18's second open still beside it — was opened in a Blender with no window, Pack Linked Libraries was run (`bpy.ops.file.pack_libraries()`), the library, group and image states were printed, the result was saved as `packed_use.blend` beside it, and the states printed again. Then `packed_use.blend` was opened three times, each time read with entry 18's report: with everything in place; with `lib/lib.blend` alone moved away; and with the whole `lib` folder moved away (each move undone afterwards). The whole run, uncut:

```
=== STEP A: open shot/use.blend, Pack Linked Libraries, save as packed_use.blend ===
00:00.324  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.328  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend
--- before packing ---
   library: lib.blend '//../lib/lib.blend'
   node group library: <bpy_struct, Library("lib.blend") at 0xc08fc3620>
   operator returned: {'FINISHED'}
--- after packing ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xc08fc3620> is_linked_packed: False
   image: tex.png library: <bpy_struct, Library("lib.blend") at 0xc08fc3620> is_linked_packed: False packed_file: False
Info: Saved as "packed_use.blend"
saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend
--- after saving ---
   library: lib.blend '//../lib/lib.blend' is_archive=False archive_parent_library=None packed_file=bpy.data.libraries['lib.blend']...PackedFile
   node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xc08fc3620> is_linked_packed: False
DONE

Blender quit
=== STEP B: open packed_use.blend with everything still in place ===
00:00.327  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend"
00:00.330  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xa65f64020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xa65f64020> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xa61dcf020> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xa65f58e20> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xa61dcf020> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xa65f58e20> -> {'//model.obj', '//../lib/lib.blend'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
00:00.363  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'.
00:00.363  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP C: open packed_use.blend with lib.blend alone moved away (model.obj still in lib/) ===
00:00.326  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend"
00:00.330  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend
VERTICES: 3
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0x91094b620> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0x91094b620> indirect: True has_data: False size: (4, 4) packed: False
      after asking for its pixels: count = 64 has_data: True size: (4, 4)
      bpy.path.abspath(i.filepath, library=i.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0x90c943820> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x90f366da0> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0x90c943820> -> {'//../lib/lib.blend', '//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0x90f366da0> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Info: No missing files
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
00:00.363  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'.
00:00.363  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
=== STEP D: open packed_use.blend with the whole lib folder moved away ===
00:00.328  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend"
00:00.332  reports          | Read packed library: '//../lib/lib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.336  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj'.
00:00.336  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj'
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/packed_use.blend
VERTICES: 0
--- libraries recorded in this file ---
    lib.blend filepath='//../lib/lib.blend' parent=None is_archive=False version=(5, 3, 29)
--- node groups ---
    LibSetup library: <bpy_struct, Library("lib.blend") at 0xb4ddd8020> indirect: False missing: False
     Path socket: '//model.obj'
--- images ---
    Render Result '' library: None indirect: False has_data: False size: (0, 0) packed: False
    Viewer Node '' library: None indirect: False has_data: False size: (256, 256) packed: False
    tex.png '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xb4ddd8020> indirect: True has_data: False size: (0, 0) packed: False
      after asking for its pixels: count = 0 has_data: False size: (0, 0)
      bpy.path.abspath(i.filepath, library=i.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
--- bpy.utils.blend_paths ---
   local=False: ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj']
   local=True : ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
   blend_paths(): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//tex.png', '//model.obj']
   blend_paths(local=True): ['//../../../../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']
   blend_paths(absolute=True): ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj']
--- bpy.data.file_path_map() ---
    <bpy_struct, Image("tex.png") at 0xb49708820> -> {'//tex.png'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb4e18c020> -> {'//model.obj'}
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, Image("tex.png") at 0xb49708820> -> {'//tex.png', '//../lib/lib.blend'}
    <bpy_struct, GeometryNodeTree("LibSetup") at 0xb4e18c020> -> {'//../lib/lib.blend', '//model.obj'}
--- Report Missing Files, run in this file ---
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/tex.png' not found, from linked data-block 'IMtex.png' (from library '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend')
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/model.obj' not found, from linked data-block 'NTLibSetup' (from library '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend')
   operator returned: {'FINISHED'}
--- can the linking file write to the linked node's Path socket, and which folder is the new value measured from? ---
    write accepted; value now: '//written-from-linking-file.obj'
00:00.338  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'.
00:00.338  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/written-from-linking-file.obj'
    vertices after the write: 0
DONE

Blender quit
```

What came back: after packing, the library carried a packed file and kept its path `//../lib/lib.blend`; the group and the image stayed linked from it and neither reported itself as linked-packed; the image itself was not packed. On opening, Blender read the library as "packed library". With `lib.blend` gone and `lib/model.obj` and `lib/tex.png` still on disk, the count was 3 and the image loaded. With the whole `lib` folder gone, the count was 0, the image had no pixels, and Report Missing Files named `lib/tex.png` and `lib/model.obj` as missing, each from its linked data-block in `lib.blend` — while the group itself was present and not missing. Pack Linked Libraries carried the library file and not the files its data points at.

#### What a linking file records when its library links onward (5.3 list entry 20)

Three sibling folders were built, each file saved before anything was linked into it: `deep/deep.blend` with a group `DeepSetup` (Import OBJ, Path `//model.obj` → Realize Instances → Group Output) and a **three**-vertex `model.obj` beside it; `mid/mid.blend`, which linked `DeepSetup` with `relative=True` and wrapped it in a local group `MidSetup`, with a **four**-vertex decoy `model.obj` beside it; and `top/top.blend`, which linked `MidSetup` with `relative=True` onto the cube, with a **five**-vertex decoy beside it. Then `top.blend` was opened twice in a fresh Blender with no window — once as built, once with `deep/model.obj` moved away — and a script read the vertex count, each library's path and its parent's path, each group's library, whether indirect and whether missing, `blend_paths(absolute=True)`, `file_path_map(include_libraries=True)` and Report Missing Files. The whole run, uncut:

```
BLENDER: 5.3.0 Beta STEP: deep WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3
Info: Saved as "deep.blend"
Info: Saved "deep.blend"
saved deep.blend; DeepSetup Path: '//model.obj'
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta STEP: mid WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3
Info: Saved as "mid.blend"
Info: Saved "mid.blend"
saved mid.blend; libraries: [('deep.blend', '//../deep/deep.blend')]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta STEP: top WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3
Info: Saved as "top.blend"
Info: Saved "top.blend"
saved top.blend; libraries: [('deep.blend', '//../deep/deep.blend'), ('mid.blend', '//../mid/mid.blend')]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
=== STEP 4: open top/top.blend and report ===
00:00.326  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/top/top.blend"
00:00.329  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.332  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/deep.blend', '//../deep/deep.blend', parent '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend'
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/top/top.blend
VERTICES: 3
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/deep.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0xa89bd0720> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0xa89bd03a0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0xa85edea08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0xa85edd820> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Info: No missing files
   operator returned: {'FINISHED'}
DONE

Blender quit
=== STEP 5: same, with deep/model.obj moved away ===
00:00.329  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/top/top.blend"
00:00.333  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend', '//../mid/mid.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.335  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/deep.blend', '//../deep/deep.blend', parent '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend'
00:00.340  io.obj           | ERROR Cannot read from OBJ file:'/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/model.obj'.
00:00.340  reports          | ERROR OBJ Import: Cannot open file '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/model.obj'
BLENDER: 5.3.0 Beta OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/top/top.blend
VERTICES: 0
--- libraries recorded in top.blend ---
    deep.blend filepath='//../deep/deep.blend' parent='//../mid/mid.blend'
    mid.blend filepath='//../mid/mid.blend' parent=None
--- node groups in top.blend ---
    DeepSetup library='//../deep/deep.blend' indirect: True missing: False
    MidSetup library='//../mid/mid.blend' indirect: False missing: False
--- bpy.utils.blend_paths(absolute=True) ---
    ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/model.obj', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/deep.blend', '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend']
--- bpy.data.file_path_map(include_libraries=True) ---
    <bpy_struct, GeometryNodeTree("DeepSetup") at 0xb1a437820> -> {'//../deep/deep.blend', '//model.obj'}
    <bpy_struct, GeometryNodeTree("MidSetup") at 0xb1a4374a0> -> {'//../mid/mid.blend'}
    <bpy_struct, Library("deep.blend") at 0xb1e9d4c08> -> {'//../deep/deep.blend'}
    <bpy_struct, Library("mid.blend") at 0xb1949ea20> -> {'//../mid/mid.blend'}
--- Report Missing Files ---
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/model.obj' not found, from linked data-block 'NTDeepSetup' (from library '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/deep/deep.blend')
   operator returned: {'FINISHED'}
DONE

Blender quit
```

What came back: 3 vertices, then 0 with `deep/model.obj` gone and an `ERROR` naming it; neither decoy was read. `top.blend` records two libraries: `mid.blend` as `//../mid/mid.blend` with no parent, and `deep.blend` as `//../deep/deep.blend` with `mid.blend` as its parent; Blender's loading lines name `mid.blend` as the parent too. `DeepSetup` is linked indirectly and `MidSetup` directly. The listings carry `deep/model.obj` and both libraries, and Report Missing Files named the missing `.obj` from `DeepSetup` in `deep.blend`. One limit of this layout: the three folders are siblings, so `//../deep/deep.blend` names the same file whether it is measured from `mid/` or from `top/`, and this run does not show which.

#### Whether the library path is recorded relative or absolute (5.3 list entry 21)

A `lib/lib.blend` holding an empty group `LibSetup` (fake user) was made, then linked into a new file in a sibling `shot/` folder four ways, each in its own Blender with no window and each new file saved before linking: `bpy.data.libraries.load(…, link=True, relative=True)`; the same with `relative=False`; the same with no `relative` given; and `bpy.ops.wm.link` with only `filepath`, `directory` and `filename` given, so that its own default for `relative_path` applied. Each file was saved and the stored library path printed. Last, `bpy.data.libraries.load` was opened on three files without linking anything, and its `libraries` attribute printed: `lib.blend`, which links nothing, and entry 20's `mid.blend` and `top.blend`, which do. The whole run, uncut:

```
BLENDER: 5.3.0 Beta MODE: makelib WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3
Info: Saved as "lib.blend"
Info: Saved "lib.blend"
saved: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3/lib/lib.blend
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta MODE: rel WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3
--- bpy.data.libraries.load(link=True, relative=True) ---
Info: Saved as "rel.blend"
Info: Saved "rel.blend"
   stored library filepath: '//../lib/lib.blend'
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta MODE: abs WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3
--- bpy.data.libraries.load(link=True, relative=False) ---
Info: Saved as "abs.blend"
Info: Saved "abs.blend"
   stored library filepath: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3/lib/lib.blend'
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta MODE: dflt WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3
--- bpy.data.libraries.load with no relative given at all ---
Info: Saved as "dflt.blend"
Info: Saved "dflt.blend"
   stored library filepath: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3/lib/lib.blend'
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta MODE: oplink WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3
--- bpy.ops.wm.link, with its own default for relative_path ---
Info: Saved as "oplink.blend"
   relative_path default in the operator: True
   operator returned: {'FINISHED'}
Info: Saved "oplink.blend"
   libraries now: [('lib.blend', '//../lib/lib.blend')]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta MODE: libraries WORKDIR: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/21/5.3
--- what the libraries attribute of load() offers about the library's own libraries ---
   reading <workdir>/lib/lib.blend -> df.libraries: []
   reading /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/mid/mid.blend -> df.libraries: [bpy_lib_library(filepath='//../deep/deep.blend', is_archive=False)]
   reading /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/20/5.3/top/top.blend -> df.libraries: [bpy_lib_library(filepath='//../mid/mid.blend', is_archive=False)]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What came back: `relative=True` stored `//../lib/lib.blend`; `relative=False` and no `relative` at all both stored the absolute path; the link command, whose own default for `relative_path` is True, stored `//../lib/lib.blend`. The `libraries` attribute gave `[]` for `lib.blend`, the path `//../deep/deep.blend` for `mid.blend`, and for `top.blend` only `//../mid/mid.blend` — not `deep.blend`, although `top.blend`, once opened, records both libraries (entry 20).

#### What a linked group holding a Bake node gives the modifier, and where that bake goes (5.3 list entry 22)

A `lib/bakelib.blend` was saved holding a group `BakeLibSetup` (fake user): Group Input → Bake → Group Output, the Bake node given one Geometry item as in entry 14. A `shot/bakeuse.blend` was saved, then linked that group with `relative=True` onto the factory cube, renamed `Target`, through a Geometry Nodes modifier `GN`, and saved again. Then `bakeuse.blend` was opened in a fresh Blender with no window and a script printed the group's library, the modifier's bake settings and its bakes, and the files beside the `.blend`; baked once with Bake Target left as the file held it and listed every file under the work folder; deleted that bake (`bpy.ops.object.geometry_node_bake_delete_single`) and listed again; then set the modifier's Bake Target to Disk with Bake Path left as it was, baked again, and listed again. The whole run, uncut:

```
BLENDER: 5.3.0 Beta STEP: lib
Info: Saved as "bakelib.blend"
Info: Saved "bakelib.blend"
saved bakelib.blend
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
BLENDER: 5.3.0 Beta STEP: shot
Info: Saved as "bakeuse.blend"
Info: Saved "bakeuse.blend"
saved bakeuse.blend; libraries: [('bakelib.blend', '//../lib/bakelib.blend')]
DONE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
00:00.322  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/22/5.3/shot/bakeuse.blend"
00:00.325  reports          | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/22/5.3/lib/bakelib.blend', '//../lib/bakelib.blend', parent '<direct>'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
BLENDER: 5.3.0 Beta STEP: bake
OPEN FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/22/5.3/shot/bakeuse.blend
node group: BakeLibSetup library: //../lib/bakelib.blend
modifier bake_target: PACKED
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=831801478 bake_target=INHERIT directory='' bake_mode=STILL use_custom_path=False node=Bake
files beside the .blend before baking: ['bakeuse.blend', 'bakeuse.blend1']
=== first bake: Bake Target left as the file holds it ===
bake operator returned: {'FINISHED'}
modifier bake_target: PACKED
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=831801478 bake_target=INHERIT directory='' bake_mode=STILL use_custom_path=False node=Bake
4 files under the work folder:
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
delete-bake operator returned: {'FINISHED'}
4 files under the work folder:
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
=== second bake: modifier's Bake Target set to Disk, Bake Path left as it is ===
modifier bake_target: DISK
modifier bake_directory: ''
number of bake entries on the modifier: 1
   bake entry: bake_id=831801478 bake_target=INHERIT directory='' bake_mode=STILL use_custom_path=False node=Bake
Info: Bake directory of object Target, modifier GN is empty, setting default path
bake operator returned: {'FINISHED'}
modifier bake_target: DISK
modifier bake_directory: '//blendcache_bakeuse/Target_GN'
number of bake entries on the modifier: 1
   bake entry: bake_id=831801478 bake_target=INHERIT directory='' bake_mode=STILL use_custom_path=False node=Bake
6 files under the work folder:
   file on disk: <workdir>/lib/bakelib.blend
   file on disk: <workdir>/lib/bakelib.blend1
   file on disk: <workdir>/shot/bakeuse.blend
   file on disk: <workdir>/shot/bakeuse.blend1
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/831801478/blobs/00001_00000.blob
   file on disk: <workdir>/shot/blendcache_bakeuse/Target_GN/831801478/meta/00001_00000.json
DONE

Blender quit
```

What came back: the modifier using the linked group held one bake, from the Bake node inside that group, with Bake Target `PACKED` on the modifier and `INHERIT` on the bake. Baked with Bake Target left at `PACKED`, the bake command finished and **no file appeared** anywhere under the work folder, and the Bake Path stayed empty. With Bake Target set to Disk, Blender printed that the path was empty and it was setting a default one, filled the modifier's Bake Path in as `//blendcache_bakeuse/Target_GN`, and wrote the bake into `shot/blendcache_bakeuse/…`, **beside the linking file**. The library's folder gained nothing.

#### What a linked group's Path field and the modifier show in a window (5.3 list entry 23)

`use.blend` from entry 18, with everything in place, was opened **with** Blender's window from the terminal (this sitting's own preferences, window in front, as in entry 13's second form), and driven by scripts that took Blender's own screenshots. Nothing was clicked. It took three runs, all recorded.

**First form.** The script spread the linked group's nodes 400 units apart, showed `LibSetup` in a pinned geometry node editor and ran View All, took a screenshot; turned the area into the Properties editor on the Modifier tab and took a second; set the modifier's `open_manage_panel` switch and took a third (each real screenshot preceded by a throwaway one and a redraw). Blender printed:

```
00:00.686  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.689  blend.readfile   | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0x990879820> indirect: False missing: False
moving the linked group's nodes apart was accepted
    Group Input NodeGroupInput (0.0, 0.0)
    Group Output NodeGroupOutput (400.0, 0.0)
    Import OBJ GeometryNodeImportOBJ (800.0, 0.0)
    Realize Instances GeometryNodeRealizeInstances (1200.0, 0.0)
    Join Geometry GeometryNodeJoinGeometry (1600.0, 0.0)
    Image Texture GeometryNodeImageTexture (2000.0, 0.0)
view_all returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3/throwaway-1.png exists: True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3/linked-nodes.png exists: True
Path socket on the linked Import OBJ node holds: '//model.obj'
Path socket: NodeSocketStringFilePath hide_value: False enabled: True is_linked: False
the node group's own library_weak_reference: None
modifier: GN type: NODES
   bake_target: PACKED bake_directory: ''
   number of bake entries: 0
   open_manage_panel: False
   open_bake_panel: False
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3/throwaway-2.png exists: True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3/modifier-panel.png exists: True
open_manage_panel set to: True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3/modifier-manage-open.png exists: True
switches after drawing: open_manage_panel=True open_bake_panel=False
00:12.381  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/tmp/quit.blend"

Blender quit
```

Its node-editor screenshot shows the six nodes in a row, small. The Group sidebar beside them is drawn greyed out — its Name `LibSetup`, Color Tag, Node Width, the Usage boxes, and the plus and minus buttons beside the group's sockets — and the editor's header shows the group with a chain-link icon. At this size the Import OBJ node's Path field holds `//model.obj` in grey; its button could not be read. The second screenshot still showed the node editor (taken straight after the area changed type, as in entry 17). The third shows the Properties editor: the modifier `GN`, its group field reading `LibSetup` greyed out with a chain-link icon at its right, and the **Manage** section open, holding two collapsed sections, **Bake** and **Named Attributes**.

**Second form.** To read the button, the script also selected the linked Import OBJ node alone and ran View Selected, and forced a redraw with Blender's own `wm.redraw_timer` (its parameters read from the build first) before each screenshot; it also opened the Bake section. Blender printed:

```
00:00.658  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.662  blend.readfile   | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
node group: LibSetup library: <bpy_struct, Library("lib.blend") at 0xc88f68620> indirect: False missing: False
moving the linked group's nodes apart was accepted
view_all returned: {'FINISHED'}
redraw_timer parameters: ['type', 'iterations', 'time_limit'] types: ['DRAW', 'DRAW_SWAP', 'DRAW_WIN', 'DRAW_WIN_SWAP', 'ANIM_STEP', 'ANIM_PLAY', 'UNDO']
Warning: 1 × Draw window and swap: 5.9645 ms, average: 5.96454204 ms
redraw_timer returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3-second/linked-nodes.png exists: True
selecting the linked Import OBJ node was accepted
view_selected returned: {'FINISHED'}
redraw_timer parameters: ['type', 'iterations', 'time_limit'] types: ['DRAW', 'DRAW_SWAP', 'DRAW_WIN', 'DRAW_WIN_SWAP', 'ANIM_STEP', 'ANIM_PLAY', 'UNDO']
Warning: 1 × Draw window and swap: 5.5980 ms, average: 5.59800002 ms
redraw_timer returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3-second/linked-import-node.png exists: True
Path socket on the linked Import OBJ node holds: '//model.obj'
modifier: GN bake entries: 0 bake_target: PACKED bake_directory: ''
redraw_timer parameters: ['type', 'iterations', 'time_limit'] types: ['DRAW', 'DRAW_SWAP', 'DRAW_WIN', 'DRAW_WIN_SWAP', 'ANIM_STEP', 'ANIM_PLAY', 'UNDO']
Warning: 1 × Draw window and swap: 4.5316 ms, average: 4.53162496 ms
redraw_timer returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3-second/modifier-panel.png exists: True
switches set to: open_manage_panel=True open_bake_panel=True
redraw_timer parameters: ['type', 'iterations', 'time_limit'] types: ['DRAW', 'DRAW_SWAP', 'DRAW_WIN', 'DRAW_WIN_SWAP', 'ANIM_STEP', 'ANIM_PLAY', 'UNDO']
Warning: 1 × Draw window and swap: 4.4055 ms, average: 4.40554181 ms
redraw_timer returned: {'FINISHED'}
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3-second/modifier-manage-bake-open.png exists: True
switches after drawing: open_manage_panel=True open_bake_panel=True
00:13.859  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/tmp/quit.blend"

Blender quit
```

Every screenshot in this form shows the window as it was one step earlier: the file named `linked-import-node.png` shows the View All picture, the file named `modifier-panel.png` shows the close view of the Import OBJ node, and the last file shows that close view again; the Properties editor never appears. The close view shows the linked Import OBJ node with its title **Import OBJ**, its **Instances** output, and the Path row: a text field holding **`//model.obj` in grey**, a socket on the node's left edge level with it, and at the field's right end a **dimmed button whose icon, read from a close crop, is a folder with a bent arrow across it** — where entry 13 drew the same node in a local group with white text and a plain, bright folder.

**Third form.** Only the modifier: the script turned the area into the Properties editor on the Modifier tab, set both switches, waited on plain redraws, and took two screenshots one after the other. Blender printed:

```
00:00.604  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.607  blend.readfile   | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
modifier: GN group: LibSetup library: //../lib/lib.blend bake entries: 0 bake_target: PACKED bake_directory: ''
switches set to: open_manage_panel=True open_bake_panel=True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3-third/modifier-1.png exists: True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/23/5.3-third/modifier-2.png exists: True
switches after drawing: open_manage_panel=True open_bake_panel=True
00:06.261  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/tmp/quit.blend"

Blender quit
```

The second screenshot shows the modifier `GN` with its group field `LibSetup` greyed out and a chain-link icon, and **Manage** open, holding **Bake** open, with **Bake Target** reading **Packed** and **Bake Path** an **empty** field with a folder button — neither drawn greyed out — then **Named Attributes**, collapsed. The modifier held no bakes (`bake entries: 0`), the group having no Bake node.

What came back, in all: moving and selecting the linked group's nodes from a script were accepted. In the window, the linked group's Path field is drawn grey with a dimmed button, and the group's sidebar settings are drawn greyed out. The modifier that uses the linked group shows its Manage section with Bake and Named Attributes inside it, and its own Bake Target and Bake Path. All of this was drawn on macOS only.

#### What the Image Editor shows for the linked group's image (5.3 list entry 24)

`use.blend` from entry 18 was opened **with** Blender's window as in entry 23, and a script turned the largest area into an Image Editor showing the linked group's image `tex.png`, opened the editor's sidebar, set its tab to Image, took a throwaway screenshot, and then two screenshots with a redraw between. Nothing was clicked. Blender printed:

```
00:00.596  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/use.blend"
00:00.599  blend.readfile   | Read library: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/lib/lib.blend', '//../lib/lib.blend', parent '<direct>'
image: tex.png filepath: '//tex.png' library: <bpy_struct, Library("lib.blend") at 0xa42b74020> indirect: True
bpy.path.abspath(img.filepath, library=img.library) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/18/5.3/shot/../lib/tex.png
sidebar tab before: 'Tool'
sidebar tab set to: 'Image'
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/24/5.3/throwaway.png exists: True
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/24/5.3/image-editor.png exists: True
image has_data: True size: (4, 4)
screenshot returned: {'FINISHED'} written to /private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/work/24/5.3/image-editor-2.png exists: True
00:07.891  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/Node setups/tmp/quit.blend"

Blender quit
```

**What the screenshots show** (the two real ones are byte for byte the same size, and the second was read). The editor's header names the image `tex.png` with a chain-link icon beside it. The sidebar's **Image** tab (read from a close crop of the screenshot) shows **Source: Single Image**, dimmed; then the file path field holding **`//tex.png`**, with a dimmed button at its left, and at its right a dimmed folder button and a reload button that is not dimmed; below it the line **"4 × 4,  RGBA float, SRGB8_A8"**; then Color Space sRGB, Alpha Straight, View as Render and Seam Margin 8, all dimmed. In the middle of the editor the image is drawn at its own size as a tiny red square. The field shows the path as the library holds it, `//tex.png`; the window does not show which folder it is measured from. The script's own reading, `bpy.path.abspath` against the image's library, points into `lib/`, and the image has its 4 × 4 pixels. This was drawn on macOS only.

---

## Things a person at a computer could settle — pruned 2026-10-18

The nine checks first listed here were all run, on the Creator's Mac on 2026-10-18, gathered with the earlier versions' tests into `Audits/Blender Research/Node setups/Hand tests — 5.3.md`. Their results are written into this document under "Observed by hand test on the Creator's Mac — 2026-10-18" in each answer: the relative import path and the image inside a linked group, and the script listing of paths, 5.3 list entry 18 (with the Image Editor's view of the image in entry 24); the bake path with a linked group, entry 22; the External Data tools, entries 10 and 15; bake defaults, entries 14 and 17; packing a linked setup, entry 19; the Essentials node group, entry 26; bake path templates, entry 16.

What is still open, and why:

1. **Every one of them, on Linux and on Windows.** Only the Creator's Mac was available to this sitting. This matters most for letter case (entry 7), for when a changed file is read again (entry 6), and for anything drawn in a window (entries 13, 17, 23, 24).
2. **Whether dragging a supported file into the node editor makes a configured import node.** No script can make a real drag happen, on any machine.
3. **Whether letter case behaves otherwise on a case-sensitive volume on the same Mac.** Making one is a change to the machine rather than a test on it.
4. **Which file a chained library's relative path is measured from.** In entry 20 the three folders are siblings, so `//../deep/deep.blend` names the same file measured from the middle file or from the top one, and the run does not show which. A layout with the middle file at a different depth from the top one would.

---

Quoted manual text: The Blender 5.3 Manual by the Blender Documentation Team, licensed under CC-BY-SA v4.0. Quoted Cycles lines are from the Cycles files supplied in `Blender Sources/cycles_5.3`.

2026-10-17 — Claude Opus 5.5 (Claude Code) — Node setups, Blender 5.3

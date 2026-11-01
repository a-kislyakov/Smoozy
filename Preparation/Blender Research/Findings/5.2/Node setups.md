# Node setups — Blender 5.2

**Sources consulted:** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), Cycles' own files (`Blender Sources/cycles_5.2`) — all read on **2026-07-30**. Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`) — read on **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing a reader should know, in the interest of the line above being worth something. The tool I run in places a project's `CLAUDE.md` into my context automatically, before I take any action. So its text was in front of me without my opening it. I did not act on it: I did not open `Preparation/Groundwork/Start Here.md` or anything else it points to, and I opened nothing under `Preparation/`, `Audits/` or `App/`. That `CLAUDE.md` carries no statements about how Blender behaves — it tells Blender researchers to stop reading it and follow their brief alone, which is what I did. Nothing in this document came from it.

I ran no helper agents. Everything below I searched and read myself.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

**LABEL: documented** (for each kind of file listed below), with two clearly marked **inferred** points at the end of the answer.

The sources name four separate ways a geometry node setup reaches a file. I list them with the words that carry each claim.

### A. Files named by a path typed on the node itself — the six Import nodes

The manual has a whole category of nodes whose only job is to read an outside file:

> "Import Nodes
>
> Nodes used to import data from external file formats."
>
> "CSV (.csv)
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

— Blender manual, *Import Nodes*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/index.html — read 2026-07-30

Each of the six carries a **Path** input. The exact words, one page each:

> "Path
>
> The path to the .obj file to be imported."
— https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/obj.html

> "Path
>
> The path to the .ply file to be imported."
— https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/ply.html

> "Path
>
> The path to the .stl file to be imported."
— https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/stl.html

> "Path
>
> The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter."
— https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/csv.html

> "Path
>
> The path to the txt file."
>
> "Currently, only files with the .txt extension are supported."
— https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/txt.html

> "Path
>
> The path to the VDB file to be imported."
— https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/vdb.html

All read 2026-07-30.

The manual also says these nodes get made for you by dropping a file on the editor, and that the dropped file's path is what the node is set to:

> "Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor.
>
> When dropping a supported file into a Geometry Node tree, the corresponding import node is added automatically and configured with the file path."

— Blender manual, *Import Nodes*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/index.html — read 2026-07-30

**From a picture.** I opened `_images/node-types_GeometryNodeImportOBJ.webp`, the picture on the *Import OBJ Node* page (https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/obj.html), read 2026-07-30. Visible in it: a node headed "Import OBJ", one output row "Instances", and one row labelled "Path" holding an empty text field with a folder icon at its right-hand end, and a round socket dot on the node's left edge level with that row. **LABEL for the picture: documented** — that is what the picture shows.

*(Note added by the citation check, 2026-08-07: I opened the same picture and it shows exactly this, round socket dot and all. Worth setting beside it: hand test list entry 14, later in this document, reports the socket dots as **rectangular** in the running 5.2. Both can be true — a picture in the manual and the program as it draws itself today are different things — but a reader meeting the two a hundred lines apart will wonder, so it is said here.)*

### B. Files reached through a data-block the node points at

**Images.** Three geometry nodes take an image:

> "The Image node provides access to a image file which allows you to conveniently enter and switch images for multiple nodes in the tree."
>
> "Image Data-Block
>
> The data-block selector to select an existing image or open an image from the file browser."
— Blender manual, *Image Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/constant/image.html

> "The Image Texture node is used to add an image file as a texture."
— Blender manual, *Image Texture Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/texture/image.html

> "The Image Info node gets information from image and animation."
— Blender manual, *Image Info Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/scene/image_info.html

All read 2026-07-30.

What an image data-block can be pointed at is more than one file, and more than one kind of file:

> "filepath
>
> Image/Movie file name (default "", never None, blend relative // prefix supported)"

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

— Python API, *Image*, https://docs.blender.org/api/5.2/bpy.types.Image.html — read 2026-07-30

**Fonts.** Two geometry nodes take a font:

> "The Font node outputs a font data-block that can be used by other nodes that require a font input."
>
> "By default, Blender's built-in font is selected."
— Blender manual, *Font Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/constant/font.html

> "Font
>
> Font glyph used to generate the curve."
— Blender manual, *String to Curves Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/utilities/text/string_to_curves.html

Both read 2026-07-30. A font data-block carries a file path:

> "class bpy.types.VectorFont(ID)
>
> Vector font for Text objects
>
> filepath
>
> (default "", never None, blend relative // prefix supported)"

— Python API, *VectorFont*, https://docs.blender.org/api/5.2/bpy.types.VectorFont.html — read 2026-07-30

I opened `_images/node-types_GeometryNodeInputFont.webp`, the picture on the *Font Node* manual page, read 2026-07-30. Visible in it: a node headed "Font" with a data-block selector row — an "F" icon with a dropdown arrow, a truncated name field reading "Bf…", then a shield icon, a folder icon and an X. **LABEL for the picture: documented** — that is what the picture shows.

**Sounds.** One geometry node takes a sound:

> "The Sample Sound Frequencies node retrieves the amplitude of a frequency range from a sound data-block at a given point in time."
>
> "This can be used to drive animations, procedural effects, or geometry based on the intensity of specific frequency bands in an audio file."
>
> "Sound
>
> The sound data-block to sample."

— Blender manual, *Sample Sound Frequencies Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/utilities/sound/sample_sound_frequencies.html — read 2026-07-30

> "class bpy.types.Sound(ID)
>
> Sound data-block referencing an external or packed sound file"
>
> "filepath
>
> Sound sample file used by this Sound data-block (default "", never None, blend relative // prefix supported)"

— Python API, *Sound*, https://docs.blender.org/api/5.2/bpy.types.Sound.html — read 2026-07-30

The release notes state this node and its socket are new in 5.2, and say a sound can be opened from inside the node tree:

> "There is a new **Sample Sound Frequencies** node. It allows controlling animations and simulations with imported sound."
>
> "The node also uses the new Sound socket type. A sound can be opened directly in Geometry Nodes."

— Release notes, *Blender 5.2 LTS: Geometry Nodes*, https://developer.blender.org/docs/release_notes/5.2/geometry_nodes/ — read 2026-07-30

I opened `_images/node-types_GeometryNodeSampleSoundFrequencies.webp`, the picture on that manual page, read 2026-07-30. Visible in it: a node headed "Sample Sound Frequencies", an "Amplitude" output, and a first row holding a musical-note icon with a dropdown arrow, then a folder icon and a wide button reading "Open"; below it rows "Time", "All Channels", "Channel", "Low", "High" and a collapsed "FFT" section. **LABEL for the picture: documented** — that is what the picture shows.

### C. Files the setup writes and then reads back — bake data on disk

> "Baking allows saving and loading intermediate geometries."
>
> "The data format used to store geometry data is not considered to be an import/export format. Volume objects, however, are saved using the OpenVDB file format which can be used interoperably."
>
> "Data can be baked using two methods:
>
> Bake Node – used to bake any portion of the node tree.
>
> Simulation Zone Baking – used to bake animations where the result of one geometry state can influence the next state."
>
> "Data is not stored to disk until the Bake operation is ran."
>
> "Blend-files must be saved to a disk before data can be baked."

— Blender manual, *Baking* (geometry nodes), https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/baking.html — read 2026-07-30

Where it goes, and the choice between disk and packed:

> "Bake Target
>
> Specifies where the baked data should be stored.
>
> Inherit from Modifier: The setting is copied from the modifier that contains the bake node. This is the default.
>
> Packed: The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk: The baked data is stored in a separate directory on disk."
>
> "Custom Path
>
> Specify a path where the baked data should be stored manually.
>
> Bake Path
>
> Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

— Blender manual, *Bake Node*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-07-30

The same two settings on the modifier that holds the node group:

> "bake_directory
>
> Location on disk where the bake data is stored (default "", never None, blend relative // prefix supported)"
>
> "bake_target
>
> Where to store the baked data (default 'PACKED')
>
> PACKED Packed – Pack the baked data into the .blend file.
>
> DISK Disk – Store the baked data in a directory on disk."

— Python API, *NodesModifier*, https://docs.blender.org/api/5.2/bpy.types.NodesModifier.html — read 2026-07-30

And per individual bake:

> "directory
>
> Location on disk where the bake data is stored (default "", never None, blend relative // prefix supported)"
>
> "bake_target
>
> Where to store the baked data (default 'INHERIT')
>
> INHERIT Inherit from Modifier – Use setting from the modifier."

— Python API, *NodesModifierBake*, https://docs.blender.org/api/5.2/bpy.types.NodesModifierBake.html — read 2026-07-30

Simulation zones bake to the same place, and the manual names the render-farm case in so many words:

> "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order."

— Blender manual, *Simulation Zone*, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/simulation/simulation_zone.html — read 2026-07-30

### D. Other `.blend` files — through a node group that is not local

The node group a setup uses need not live in the same file:

> "This section lists all the node groups, both those in the current blend-file and those Linked or Appended from another blend-file."

— Blender manual, *Group* (geometry nodes), https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/group.html — read 2026-07-30

The 5.2 release notes carry a worked example in which two of the recorded libraries are node `.blend` files — one sitting under an `assets/nodes` folder, one under a `blender/.../datafiles/assets/nodes` folder. What those folders are is not stated on the page, so I record the paths as they stand:

> "bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)"
>
> "bpy_lib.libraries(filepath='//../../../../../local/blender/linux/5.0/datafiles/assets/nodes/procedural_hair_node_assets.blend', is_archive=True)"

— Release notes, *Blender 5.2 LTS: Python API*, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30

This is question 2's territory; I mention it here only because it is one of the file kinds a node setup can make a file point at.

### Two inferred points

**Inferred 1 — the Import nodes' Path can be computed rather than typed.** The step: the manual lists Path under "Inputs", not under "Properties", for all six Import nodes; the picture of the Import OBJ node shows a socket dot on the node's left edge level with the Path row; the manual elsewhere says of sockets generally that they are what links connect. From that I take it that the Path can be driven by a link from other nodes rather than being a fixed string in the file. The sources do not say this outright anywhere I found. A reader has to supply the step, so: **inferred**.

**Inferred 2 — the Import nodes' Path understands the `//` blend-relative prefix.** The step: the API lists a socket type

> "class bpy.types.NodeSocketStringFilePath(NodeSocketStandard)
>
> String socket of a node
>
> default_value
>
> (default "", never None, blend relative // prefix supported)"

— Python API, *NodeSocketStringFilePath*, https://docs.blender.org/api/5.2/bpy.types.NodeSocketStringFilePath.html — read 2026-07-30

and the picture of the Import OBJ node shows the Path row drawn with a file-browser folder icon, which is how the manual describes file-path fields elsewhere. Nothing in the sources says the Import nodes' Path input is of this socket type. So: **inferred**, and weakly so. The same socket exists on a node group's own interface —

> "class bpy.types.NodeTreeInterfaceSocketStringFilePath(NodeTreeInterfaceSocket)
>
> String socket of a node
>
> default_value
>
> Input value used for unconnected socket (default "", never None, blend relative // prefix supported)"

— Python API, *NodeTreeInterfaceSocketStringFilePath*, https://docs.blender.org/api/5.2/bpy.types.NodeTreeInterfaceSocketStringFilePath.html — read 2026-07-30

— which means a node group can expose a file path as one of its inputs, to be filled in wherever the group is used.

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer, and are no longer open questions about Blender: which path forms the Path accepts and whether `//` works, list entry 3; what happens when the file is missing, list entries 3 and 4; whether these paths take part in Make Paths Relative, Report Missing Files and packing, list entry 5; where a bake goes when the path is left empty, list entry 6; whether the path can be computed upstream instead of typed, list entry 13. Read this list as "the documentation does not say", not as "nobody knows".)*

- Whether the Import nodes' paths are counted as "external files" by Blender's own path housekeeping — Make Paths Relative, Make Paths Absolute, Report Missing Files, Find Missing Files, Pack Resources. Those operators are described as acting on "all paths to external files" without saying what that set contains; no source I found names node paths as being in or out of it.
- What happens when an Import node's path points at nothing, or at a file of the wrong kind. The sources do not say.
- Whether an Import node re-reads its file on every evaluation, caches it, or reads it once on file load. Not stated.
- Whether the file a node path names is stored anywhere in the `.blend` other than on the node.
- Whether an image, font or sound used only from inside a geometry node tree can be packed like other images, fonts and sounds. The packing pages talk about resources generally, not about node use specifically.
- The bake data's own file names and layout inside the bake directory. The manual gives only the directory and says the format "is not considered to be an import/export format".
- The Cycles texture cache. Blender's preferences carry a "Texture Cache — Directory to store .tx texture cache files", and the 5.2 Python API notes add an `EXPAND_CACHES` path-walking flag for "cache files, like texture cache files associated with images". (I did not record the addresses of those two quotes; nothing in the answer above rests on them.) I found nothing tying this to geometry node setups specifically, so I leave it out of the answer above.
- A `NodeSocketText` type exists (a socket carrying a Text data-block, which can have a `filepath`), but I found no geometry node documented as using it, so I do not claim it as a route.

### Outside the interface

The sources do not say that any of the above behaves differently when Blender is driven from outside its own window. Three things I did find that bear on it, all quoted exactly:

- Dropping a file into the node editor to create an Import node is plainly a person-at-a-window action; the manual describes no other way to reach it. Whether the same node can be made and pointed at a file from a script is not addressed on that page. **This line is thinner than it should be.** I opened the Python API pages for three of the six Import node types — `GeometryNodeImportCSV`, `GeometryNodeImportOBJ`, `GeometryNodeImportVDB` — and record nothing from them anywhere in this answer, and I did not open the other three. So the question was not searched out; it rests on one manual page saying nothing either way.
- Baking is gated on the file being on disk: "Blend-files must be saved to a disk before data can be baked." (*Baking*, address above.) The manual's baking entry point is an operator over a selection: "Bake Geometry Node — Bakes the simulations and bake nodes in all modifiers for the selected objects." (Manual; I did not record which page.) The API lists per-bake operators `bpy.ops.object.geometry_node_bake_single`, `geometry_node_bake_delete_single`, `geometry_node_bake_pack_single`, `geometry_node_bake_unpack_single` (https://docs.blender.org/api/5.2/bpy.ops.object.html, read 2026-07-30). I found no command-line switch for baking geometry nodes.
- The command-line documentation has a switch that speaks about evaluation in background mode: "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options). Scripts requiring evaluated data then need to explicitly ensure that an evaluated depsgraph is available (e.g. by calling depsgraph = context.evaluated_depsgraph_get()). NOTE: this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode." (Blender manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.) This says dependency graphs are built and evaluated on file load in background mode by default today, and that this will stop. Whether a geometry node setup — and so its file reads — is evaluated as part of that is not stated on this page; connecting the two would be an inference I am not making.

**Brought up to date by hand test, 2026-08-06 — observed.** A Blender started with no window does read an import node's file when it opens the `.blend`, and does read it again at render time; the rendered picture was the same with `--disable-depsgraph-on-file-load` in force as without it. Every route in this answer was built, set and read from a script in a Blender with no window at all, on a machine with no display server, so nothing here needs a person clicking. The one exception is dragging a file into the node editor to make an import node: no script can make a real drag happen, so it cannot be tested at all and sits in the "Cannot be run" section of `Audits/Blender Research/Node setups/Hand tests.md`. See "Observed by hand test" below, list entries 4 and 12. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers: a Blender started with no window reads an import node's file when it opens the `.blend`, and again at render time, and the picture was the same with `--disable-depsgraph-on-file-load` in force as without it. So this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so one thing could also be looked at there that no terminal run can settle: the Path row is drawn as a text field holding the path, with a folder button at its right-hand end and a socket dot on the node's left edge level with it, and the value it shows is the one a script reads back. See "Observed by hand test on the Creator's Mac" below, list entries 4, 12 and 14. Dragging a file into the node editor to make an import node still cannot be tested at all, on either machine.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back was built, changed and read entirely from a script in a Blender with no window at all, on a machine with no display server, so nothing in it needs a person clicking. It also turns up something anyone driving Blender from outside should know: asking for the evaluated geometry again does not by itself make an import node read its file again. The setup has to be marked for a fresh run, **and** the file's modification time has to have changed by a whole second. A script that writes a file and immediately asks for the geometry can therefore be handed the old contents, with nothing said about it. Observed on Linux in all four versions; untested on macOS, because the rented machine was given up before the Mac half could be run, and Windows was not tried at all. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Operating system

The sources do not say. Nothing on any of the pages above distinguishes Windows, macOS or Linux for these nodes, these paths or this baking. The one operating-system remark I found anywhere near file paths is in the Python API's path helpers: "bpy.path.basename(path) — Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." (https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30). Blender's requirements page (https://www.blender.org/download/requirements/, read 2026-08-01) does not name 5.2, so I do not read it as speaking for this version; the only line on it that names a range including 5.2 is "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later", which is about hardware, not about node setups.

**Brought up to date by hand test, 2026-08-06 — observed.** On Linux a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, and an image path behaves the same way — see "Observed by hand test" below, list entry 11. That is the one place these tests found an operating system could make a difference, and it has been settled for Linux only. Nothing here has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed. This is the one place the two machines disagreed.** The same test on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — came out the other way: `//MODEL.OBJ` and `//Model.Obj` both found `model.obj`, an image path in upper case loaded, and `bpy.path.resolve_ncase` handed the upper-case name straight back instead of correcting it to the file that exists. So **a path is matched letter for letter on Linux and is not on macOS**, on the ordinary file system each machine came with. The two results are kept apart and never merged into one. Windows is still untested, because no Windows machine has run it. Everything else in this answer was run on both machines and agreed on both. See "Observed by hand test on the Creator's Mac" below, list entry 11.

**Brought up to date a third time by hand test, 2026-08-07 — observed.** The test the citation check sent back found a second place where an operating system could make a difference, and it could only be run on one of them. Whether an import node reads its file again turns on the file's modification time, and on the *whole second* of it: a change inside the same whole second is not noticed, a change that crosses one is, in either direction. That was settled on the rented Linux machine's ordinary file system, in all four versions, across eighty runs with no exception. Whether macOS keeps and compares that stamp the same way is **untested** — the rented machine was given up before the Mac half could be run — and no Windows machine has run it. So this holds for Linux and is untested on both of the others. See "Observed by hand test — the test the citation check sent back" below, list entry 18.

### Where I looked

- Listed the whole geometry-nodes tree of the manual (`modeling/geometry_nodes`) and read its category index, which lists 361 node entries. Searched that index for `image`, `font`, `sound`, `import`, `bake`, `file`, `text`.
- Searched the whole geometry-nodes manual tree, case-insensitively, for: `import`, `file`, `external`, `relative`, `dependenc`, `pack`, `background`, `command line`. Read every hit that was not a false positive (`profile`, `Pack UV Islands`, `Set Grid Background`, `background lighting`).
- Read in full: the Import Nodes index and all six Import node pages; Image Node; Image Texture Node; Image Info Node; Font Node; String to Curves Node; Sample Sound Frequencies Node and the Sound Utility Nodes index; Baking; Bake Node; Simulation Zone; Group; the Geometry Nodes Modifier page.
- Opened the manual pictures `node-types_GeometryNodeImportOBJ.webp`, `node-types_GeometryNodeInputFont.webp`, `node-types_GeometryNodeSampleSoundFrequencies.webp`.
- Searched the whole API documentation for `bake_directory`, `bake_target`, `FILE_PATH`, `NodeSocketStringFilePath`. Listed every `NodeSocket*` and `NodeTreeInterfaceSocket*` type to see which data-block kinds a node can carry, and read the Font, Image, Sound, Text, String and StringFilePath ones. Read `bpy.types.Image`, `bpy.types.VectorFont`, `bpy.types.Sound`, `bpy.types.Text`, `bpy.types.NodesModifier`, `bpy.types.NodesModifierBake`, `bpy.types.GeometryNodeImportCSV`, `…ImportOBJ`, `…ImportVDB`.
- Read the 5.2 release notes for Geometry Nodes, Core, Pipeline & I/O, Python API and Assets in full.
- Searched Cycles' own files (`cycles_5.2`) for `geometry node`, `geometry_node` and `filepath`. The only geometry-node hits are compile-time switches in `source/kernel/svm/radial_tiling_shared.h` about sharing one texture function between Cycles and geometry nodes; the `filepath` hits are in `maketx.py` and `operators.py`, which are about converting textures to a Cycles texture cache. Nothing there speaks to what files a geometry node setup depends on. **Ruled out.**
- Ruled out the requirements page for this question: it does not name 5.2 and says nothing about nodes.

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

**LABEL: documented** for the parts marked below; **inferred** for the two steps I mark as such; **not in the sources** for one part I name at the end.

Reading of the question: I read "the files that setup uses" as the outside files the linked node setup depends on — images, fonts, sounds, imported data files, and further `.blend` files. If instead it meant only the one `.blend` the setup itself came from, the answer is the first section below on its own.

Scope: the question says "a node setup", not "a geometry node setup". Most of what follows is about linking itself and holds whatever kind of node group is linked, because the words quoted describe data-blocks and libraries rather than node types. Two stretches are about geometry nodes in particular and say so: *What stays local to the linking file*, which is about the Geometry Nodes modifier, and the baked-data material inside it.

### What is recorded: a library entry naming the other `.blend`

**Documented.**

> "class bpy.types.Library(ID)
>
> External .blend file from which data is linked"
>
> "filepath
>
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"

— Python API, *Library*, https://docs.blender.org/api/5.2/bpy.types.Library.html — read 2026-07-30

> "library
>
> Library file the data-block is linked from (readonly)"

— Python API, *ID*, https://docs.blender.org/api/5.2/bpy.types.ID.html — read 2026-07-30

The path can be relative or absolute, and that is a choice made at link time:

> "Relative Path
>
> Reference the external blend-file using a relative path rather than an absolute one."

— Blender manual, *Link & Append*, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — read 2026-07-30

What "relative" means, in the manual's own words:

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
>
> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."
>
> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

— Blender manual, *Relative Paths*, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30

And where a person can see it:

> "Linked data-blocks are indicated with a chain icon in the Outliner. They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."

— Blender manual, *Link & Append*, address above — read 2026-07-30

Linking means the data keeps being read from the other file rather than copied:

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."
>
> "Linked data-blocks are initially not editable."

— Blender manual, *Link & Append*, address above — read 2026-07-30

The library entry holds more than that path. The same API page lists a version, an archive flag and a packed file on it — quoted under *Libraries that are packed into the linking file* below.

### What the setup's own dependencies become in the linking file: indirectly linked data

**Documented that Blender keeps a layer of indirectly linked data; inferred that a linked node setup's own dependencies are what fills it.** The sources name the layer:

> "is_library_indirect
>
> Is this ID block linked indirectly (default False, readonly)"

— Python API, *ID*, address above — read 2026-07-30

That indirect layer is named again from the other side, in the options for copying data in rather than linking it:

> "Localize All
>
> Also copy all indirectly linked data, instead of maintaining the links."

— Blender manual, *Link & Append*, address above — read 2026-07-30

> "use_recursive (bool) – Localize All, Localize all appended data, including those indirectly linked from other libraries (optional)"

— Python API, *bpy.ops.wm.append*, https://docs.blender.org/api/5.2/bpy.ops.wm.html — read 2026-07-30

And the Asset Browser page states what "dependencies" covers when a thing is brought in:

> ""Dependencies" in this case means everything the asset refers to."

— Blender manual, *Asset Browser*, https://docs.blender.org/manual/en/5.2/editors/asset_browser.html — read 2026-07-30

**Inferred, and it is the step that joins this section to the question.** None of the words above mentions a node setup, and none says what makes a data-block indirectly linked. That the images, fonts, sounds and imported files a linked node setup uses are what arrives in the linking file as indirectly linked data is a step I supply, from the sentence about dependencies meaning everything the asset refers to. It is a short step, and both quotes point the same way; it is still a step, and the section rests on it.

(That page also states the ordinary way a node setup arrives from a library: "Geometry nodes can also be dragged onto objects to add a Geometry Nodes Modifier." Same page, same date.)

### If the setup's library itself links from a third `.blend`: the paths recorded are absolute, or relative to the library — never to the linking file

**Documented.** This is the clearest statement in the sources on how the chain is written down:

> "A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file. The output libraries attribute is always None."
>
> "Packed linked data-blocks are not linkable and are not listed in the input libraries attribute."

— Python API, *BlendDataLibraries.load*, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html — read 2026-07-30

The 5.2 release notes introduce this and print an actual result, in which every recorded path happens to begin with `//` — the relative prefix. That is one file's list rather than a rule; the sentence above allows either form. The words:

> "Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"
>
> "bpy_lib.libraries(filepath='//../../../assets/scripts/update_bake_path.blend', is_archive=False)
>
> bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)
>
> bpy_lib.libraries(filepath='//../../../assets/sets/space_background-ice_belt/space_background-ice_belt.blend', is_archive=False)
>
> bpy_lib.libraries(filepath='//../../../assets/props/singularity/singularity.blend', is_archive=False)
>
> bpy_lib.libraries(filepath='//../../../../../local/blender/linux/5.0/datafiles/assets/nodes/procedural_hair_node_assets.blend', is_archive=True)"

— Release notes, *Blender 5.2 LTS: Python API*, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30

### How a relative path carried by linked data is turned into a real path

**Documented for the helper; inferred for Blender's own resolution.**

The path helper takes the library into account explicitly:

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the "//" prefix."
>
> "start (str | bytes | None) – Relative to this path, when not set the current filename is used.
>
> library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

— Python API, *Path Utilities (bpy.path)*, https://docs.blender.org/api/5.2/bpy.path.html — read 2026-07-30

The same distinction shows up in the manual's path template variables:

> "blend_name_lib:
>
> Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from.
>
> blend_dir_lib:
>
> Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."

— Blender manual, *File Paths*, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30

**Inferred step 1:** from these two — a helper whose whole purpose is to substitute the library's path for the open file's when a path came from a library, and a pair of template variables that exist precisely to name the library's file and directory instead of the open one's — I take it that a `//` path carried by a linked data-block is resolved against the library `.blend` it came from, not against the file doing the linking. The sources do not say that in a single sentence. A reader has to supply the step, so: **inferred**. It is a short step and both sources point the same way, but it is a step.

### Whether the linking file records those outside file paths itself

**Inferred, and I want to be careful here.**

Two pieces of evidence, quoted first.

> "SKIP_LINKED:
>
> Skip Linked.
>
> Skip paths of linked IDs."

— Python API, *File Path Foreach Flag Items*, https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html — read 2026-07-30

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them."
>
> "include_libraries (bool) – Include library file paths of linked data. False by default."

— Python API, *BlendData*, https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30

What these two establish, taken apart rather than together.

**Documented:** the library `.blend` paths are a separate thing from the rest, and `file_path_map` leaves them out unless asked for — the source says "False by default" in so many words.

**Inferred:** that with a linked node setup loaded, a script running in the linking file can walk the paths of the linked data-blocks — the image files, font files and so on that the setup uses. The step: a flag exists for skipping the paths of linked IDs, so not skipping them must be what happens when it is not set. That flag list gives no defaults, so the step is mine — unlike the `include_libraries` default just above, which the source states.

**Inferred step 2:** that the linking file on disk does *not* store copies of those outside paths, and holds only the library reference. The step: linked data is described as read from the source file and re-read on reload ("changes made there will be reflected in the current file the next time it is reloaded"), it is not editable in the linking file, and a `//` path on it resolves against the library rather than the linking file. Together these only make sense if the paths belong to the library file. But no source I found states outright what is or is not written into the linking file on disk, and the two API items above describe what is visible in memory once everything is loaded, which is a different thing. So: **inferred**, and I would not lean hard on it.

### What happens when the library cannot be found

**Documented.**

> "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

— Blender manual, *Link & Append* (Broken Libraries), address above — read 2026-07-30

The two repair paths named there are Reload ("immediately update the current blend-file with the latest version of the linked data-blocks, without having to reopen the file") and Relocate ("It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed)"). Both are described as Outliner context-menu actions.

The manual also warns about cost:

> "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them. Once the data-blocks are loaded, however, memory usage is the same."

— Blender manual, *Link & Append* (Compression & Memory Use), address above — read 2026-07-30

And about direction:

> "In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links."

— Same page, same date.

### Libraries that are packed into the linking file

**Documented.** Instead of pointing at the other `.blend`, the linking file can carry it:

> "Pack Linked Libraries
>
> Mark all linked library files in the current blend-file as packed. Actual packing will happen on the next save of the blend-file."
>
> "Unpack Linked Libraries
>
> Unpack all used linked library files from this blend-file."

— Blender manual, *Packed Data*, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html — read 2026-07-30

> "Pack Linked Libraries
>
> Pack data-blocks that are linked from an external blend-file into the current one."

— Blender manual, *Topbar* (File ‣ External Data), https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30

Packed linked data is recorded as an "archive" library:

> "archive_libraries
>
> Archive libraries of packed IDs, generated (and owned) by this source library (default None, readonly)"
>
> "archive_parent_library
>
> Source library from which this archive of packed IDs was generated (readonly)"
>
> "is_archive
>
> This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library. (default False, readonly)"
>
> "packed_file
>
> (readonly)"
>
> "version
>
> Version of Blender the library .blend was saved with (array of 3 items, in [0, inf], default (0, 0, 0), readonly)"

— Python API, *Library*, address above — read 2026-07-30

That `is_archive` flag is the same one shown in the release-note listing above, where the Blender-installation node asset file came back with `is_archive=True`.

### What stays local to the linking file even though the setup is linked

**Documented for where the settings live; inferred for the consequence.**

The values fed into the linked node group belong to the modifier in the linking file:

> "Inputs
>
> A list of the node group's inputs which can have unique values even if the group is shared among multiple modifiers."

— Blender manual, *Geometry Nodes Modifier*, https://docs.blender.org/manual/en/5.2/modeling/modifiers/geometry_nodes.html — read 2026-07-30

So does the bake location:

> "Bake Path
>
> Location on disk where the baked data for Simulation Zones and Bake Nodes are stored."

— Same page, same date. And in the API, `bake_directory` and `bake_target` are properties of `NodesModifier` (quoted in full under question 1).

**Inferred:** because a node group's interface can carry Image, Font, Sound and file-path inputs (`NodeTreeInterfaceSocketImage`, `…SocketFont`, `…SocketSound`, `…SocketStringFilePath` — all listed at https://docs.blender.org/api/5.2/bpy.types.NodeTreeInterfaceSocket.html, read 2026-07-30), and because the modifier holds per-modifier values for a group's inputs, a linked node setup can end up reading files chosen in the linking file, whose paths would then be the linking file's own. The sources do not join these up; the step is mine.

Baked data can name data-blocks that live in a library, and records the library by name:

> "lib_name
>
> Used when the data block is not local to the current .blend file but is linked from some library (default "", readonly, never None)"
>
> "id_name
>
> Name that is mapped to the referenced data-block (default "", readonly, never None)"

— Python API, *NodesModifierDataBlock*, https://docs.blender.org/api/5.2/bpy.types.NodesModifierDataBlock.html — read 2026-07-30

Which the manual exposes as a panel:

> "Baked geometries that reference other data-blocks such as materials are listed here. This panel allows changing these references after the data has been baked."
>
> "Currently only material data-blocks are supported."

— Blender manual, *Baking* (Data-Block References), https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/baking.html — read 2026-07-30

The 5.2 release notes record a change here:

> "Added support for geonodes packed bakes in nodes modifiers"

— Release notes, *Blender 5.2 LTS: Core* (Library Override), https://developer.blender.org/docs/release_notes/5.2/core/ — read 2026-07-30

### Not covered
*(Note added by the citation check, 2026-08-07. Everything in this list remains a true statement about what the **sources** say, which is what the list is for. Several of the things it names were afterwards settled by the hand tests printed later in this answer: which folder a `//` inside a linked setup is measured from, and what the linking file records, list entry 7; what packing a library carries and what it leaves behind, list entry 8; what is recorded when a library links onward, list entry 9; whether the library path is stored relative or absolute, list entry 10. Read this list as "the documentation does not say", not as "nobody knows".)*

- **Not in the sources:** whether the paths held on the six Import nodes inside a *linked* node setup are resolved against the library `.blend` or against the linking file. The `bpy.path.abspath` and template-variable evidence above concerns "path properties on library-linked data-blocks"; nothing states whether a path sitting in a socket of a linked node tree counts as such. See "where I looked" below for the searches behind this.
- What the linking file writes to disk, as against what is in memory once linked data has loaded. No source I found describes the on-disk record directly.
- Whether the bake data of a linked node setup can be shared between files, or is always per-modifier.
- Whether "Make Paths Relative" and "Make Paths Absolute" touch linked data's paths at all (linked data is stated to be non-editable, but the operators are described as acting on "all paths to external files").
- Library Overrides. I read only the index entry for them and did not open the override pages, so I make no claim about what an override can change about a linked node setup's file references.
- Anything about the order in which libraries are loaded, or what happens when two libraries name the same outside file by different paths.
- Whether a shader or compositor node group linked from another file differs in any of this from a geometry one. I searched for linking rather than for node kinds, so nothing here was looked for separately for those two.

### Outside the interface

Partly documented, and better here than for question 1.

Linking has a scripting route that takes the same relative-path choice: `bpy.ops.wm.link(..., relative_path=True, ...)` — "relative_path (bool) – Relative Path, Select the file relative to the blend file (optional)" (Python API, *bpy.ops.wm*, https://docs.blender.org/api/5.2/bpy.ops.wm.html, read 2026-07-30). And `bpy.data.libraries.load(filepath, *, link=False, pack=False, relative=False, ...)` — "relative (bool) – When True the path is stored relative to the open blend file." and "pack (bool) – If True, and link is also True, pack linked data-blocks into the current blend-file." (Python API, *BlendDataLibraries*, address above, read 2026-07-30). The release-note example of reading a file's library list is itself a console session, so that reading works outside the window.

Three repair actions — Reload Library, Relocate Library, Relocate Linked ID — are documented as Outliner context-menu items. For Reload the API also lists `Library.reload()` ("Reload this library and all its linked data-blocks", *Library*, address above). For the two Relocate actions I found no non-interface equivalent.

One command-line switch bears directly on linking: "--disable-liboverride-auto-resync — Do not perform library override automatic resync when loading a new blend-file. NOTE: this is an alternative way to get the same effect as when setting the No Override Auto Resync User Preferences Debug option." (Blender manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.)

Beyond these, the sources do not say whether linking behaves differently outside the interface. In particular, nothing says what a headless Blender does when a library is missing — the "placeholder data-blocks" text does not mention a warning, a message, or a non-zero exit.

**Brought up to date by hand test, 2026-08-06 — observed.** All of it holds with no window. Linking, packing the library, reading what the linking file records, resolving a `//` path belonging to linked data, and creating placeholders for a library that cannot be found were every one of them done and read from a script in a Blender started with `-b`, on a machine with no display server at all. With the library gone, Blender printed a warning naming it, left a placeholder in its place and exited with code 0. See "Observed by hand test" below, list entries 7, 8, 9 and 10. Observed on Linux; untested on macOS, and Windows was not tried at all.

**Brought up to date again by hand test, 2026-08-07 — observed.** Every one of those tests was run again on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card** — and gave the same answers, so this holds on **macOS as well as Linux**. Windows is still untested, because no Windows machine has run it. The Mac has a screen, so two things could be looked at there that no terminal run can settle. The Path field on a linked group's Import node is drawn **greyed out** in the window, in all four versions — while a script writing to that same socket is accepted, which the Linux runs showed. And the modifier for a linked group holding no Bake node shows no Bake panel at all, only a collapsed section headed Manage; that section could not be opened without a click, so what it holds is untested. What a linked group that *does* hold a Bake node gives the modifier was answered instead with no window at all. See "Observed by hand test on the Creator's Mac" below, list entries 7, 8, 9, 10, 15, 16 and 17.

### Operating system

The sources do not say. Nothing on the Link & Append, Relative Paths, Packed Data or Library pages distinguishes operating systems. The release-note example happens to show a Linux path (`.../local/blender/linux/5.0/datafiles/...`), which tells you the person who ran it was on Linux and nothing more. The only operating-system remark near this subject is the `bpy.path.basename` note quoted under question 1: "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility."

**Brought up to date by hand test, 2026-08-06 — observed.** Everything below was run on Linux only, where a path is matched letter for letter — see answer 1's observed section, list entry 11. Nothing has been run on macOS yet, and no Windows machine has run it, so the answer holds for Linux and is untested on both of the others.

**Brought up to date again by hand test, 2026-08-07 — observed.** The letter-case test was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2** — as well, and came out the other way: there the upper-case forms found the file. See answer 1's Mac observed section, list entry 11. So a path is matched letter for letter on Linux and is not on macOS. Everything else here was run on both machines and agreed on both. No Windows machine has run it, so Windows is untested.

### Where I looked

- Listed `manual_5.2/files` and read in full: `linked_libraries/index.txt`, `linked_libraries/link_append.txt`, `blend/packed_data.txt`, `blend/open_save.txt` (Relative Paths section), `file_paths.txt`.
- Read the File ‣ External Data and Clean Up sections of `interface/window_system/topbar.txt`, and the Import Settings and Using Assets sections of `editors/asset_browser.txt`.
- Searched the whole manual, case-insensitively, for: `ndirect` (to catch "indirect", "indirectly"), `Relative Path`, `Report Missing Files`, `Find Missing Files`, `Make Paths Absolute`, `Make Paths Relative`, `archive`, `node group` within `files/`, `Import Method`, `Blender File` (Outliner display mode). Followed every hit that was about linking rather than about lighting (`indirect` matches a great many Cycles and EEVEE lighting pages, all ruled out on sight).
- Searched the whole API documentation for `library` in `bpy.path`, and read `bpy.types.Library`, `bpy.types.ID` (library, library_weak_reference, is_library_indirect, name_full), `bpy.types.BlendDataLibraries`, `bpy.types.BlendData` (`file_path_foreach`, `file_path_map`), `bpy.types.BlendDataPathMeta`, `bpy_types_enum_items/file_path_foreach_flag_items`, `bpy.types.NodesModifierDataBlock`, `bpy.ops.wm.link` and `bpy.ops.wm.append`, `bpy.ops.file` (`find_missing_files`, `report_missing_files`, `make_paths_relative`), `bpy.types.NodeTreeInterfaceSocket` and the interface socket types for Image, Font, Sound, Text and StringFilePath.
- Read the 5.2 release notes for Python API (Path Iterator, All IDs Iterator, API Improvements to `libraries.load()`), Core (Library Override), Assets and Geometry Nodes in full.
- For the "not in the sources" answer above about Import node paths inside a linked setup: searched the whole geometry-nodes manual tree for `relative`, `external`, `librar`, `linked`, `pack` and `dependenc`; searched the whole API documentation for `NodeSocketStringFilePath` and every use of the phrase "blend relative // prefix supported" on node-related types; read the six Import node pages and the Group page in full. The Import pages say only "The path to the … file to be imported" and never mention libraries, relative paths or linking. Nothing anywhere joins node sockets to library path resolution.
- Searched Cycles' own files (`cycles_5.2`) for `geometry node`, `geometry_node` and `filepath`. Nothing about linking or libraries. **Ruled out.**
- Ruled out the requirements page: it does not name 5.2 and says nothing about linking.

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

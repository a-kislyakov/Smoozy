# File output nodes — Blender 5.3 findings

- **Sources consulted:** the Blender 5.3 manual (plain-text copy, with its pictures), the Blender 5.3 Python API documentation (plain-text copy) and the Blender 5.3 release notes, all as taken on 2026-10-11; Cycles' own files for 5.3, as taken on 2026-10-16. These snapshot dates are the dates read throughout.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Notes on the line above, for honesty:

- When this session began, the tool I run in placed some text in front of me without my opening anything: the short text of the project's `CLAUDE.md` (a signpost telling Blender researchers to ignore it and follow their brief), a one-line-per-entry index of the user's saved notes, and the titles of recent project changes. I did not open any of these files. None of that text says anything about how File Output nodes behave.
- Inside `Blender Sources/` I also read the folder's own note, "About this folder.md". I did not follow its link out of the folder.
- I used only the 5.3 folders. I did not open the folders for other Blender versions, or the `requirements` folder.
- To look at two manual pictures, I made converted or enlarged copies of them in the scratch folder my brief names. I ran nothing else. I did not run Blender.

Labels used: **documented** (the source says it outright), **inferred** (follows from what a source says; the step is written out), **observed** (from examining Cycles' own files), **not in the sources**.

---

## Observed by hand test

*Added after the research above, by the hand-test sitting of 2026-10-18. The researcher's answers above each block are left as written; a block headed **Observed by hand test** is what a running Blender did, and it is labelled **observed**.*

Everything under such a heading was got by running **Blender 5.3.0 Beta**, built 2026-10-16, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — on **2026-10-18**, by Claude Opus 5.5 (Claude Code). No other machine ran any of it: there was no Linux machine, no Windows machine and no other graphics card in this sitting, so **every observed result here is the Mac's alone, and says nothing about Linux or Windows.**

Unless a block says it was run with Blender's window open, Blender was started from the Mac's Terminal with no window, in this form:

`TMPDIR="<work folder>/tmp" BLENDER_USER_RESOURCES="<work folder>/userres" "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python <script>`

so it used a settings folder and a temporary folder of its own, made beforehand and confirmed in use before the first test. The scripts built the scene from Blender's own starting scene, rendered with Cycles on the processor at one sample, no denoising, 64 × 64 pixels unless the block says otherwise. The compositor tree was made by script and handed to the scene through `scene.compositing_node_group`, holding a Render Layers node, the tree's own Group Output node fed from it, and the File Output nodes the test asked for. Unless a block says otherwise, a File Output node's File Name was set to `file_name`, its Media Type to Image and its format to PNG. Every script and every command typed is written out in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`, with the entry numbers used below; what follows stands on its own without it. The machine's output is quoted word for word; where a listing is long, the part that carries the answer is kept and the cut is said.

---

## 1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?

**Answer.**

- **What it writes: images.** Documented. The manual says the node "writes one or more images to disk".
- **Two ways of storing them.** Documented. The manual lists two Media Types for the node:
  - Image: each input becomes its own image file.
  - Multi-Layer EXR: all inputs go into one multi-layer OpenEXR file.
- **Which picture formats.** For formats, the manual points to its "Images Settings". The node's format settings are the same kind of settings object as the render output's (`ImageFormatSettings`, which the API lists as used by both `CompositorNodeOutputFile.format` and `RenderSettings.image_settings`). That object's format list is: AVIF, JPEG, OpenEXR, PNG, WebP, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw, TIFF, OpenEXR MultiLayer and FFmpeg Video.
  - That this is the list for the shared settings object is **documented**.
  - That the node offers every item on it is only **inferred**, and not confirmed. The sources never list the node's own menu. Video is dealt with in question 11.
- **How many outputs.** The node starts with no inputs, and inputs are added by hand. Documented. **Not in the sources:** any maximum number. The API's index for the active output is given the range "in [0, inf]". That is a range for an index, not a stated limit, so I draw nothing from it.
- **What each output carries.** Each input stands for one image to be written. Each one has a name, which is used to build the file name. In Multi-Layer EXR mode each input becomes one layer of the single file. All of this is documented. The full list of settings each output carries is in question 2.
- **The node's picture.** The manual's picture of the node is `_images/node-types_CompositorNodeOutputFile.webp`, on the File Output page. It shows a folder field reading "/tmp/" with a folder button, a name field reading "file_name", and one empty socket on the left. This is documented, in the sense that it is what the picture shows.

**Evidence.**

- "The File Output node writes one or more images to disk during rendering or compositing." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- Same page: "By default, the node has no input sockets. Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel."
- Same page: "Each input corresponds to an image that will be written to disk."
- Same page, Media Type: "Image: Saves each input as a separate image file. Each input can use its own image format settings." and "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."
- Same page: "The name of each input socket is used to construct the final file name." and "When using Multi-Layer EXR, each input is stored as a separate layer within the file."
- Same page: "See Images Settings for details on supported formats and options."
- `ImageFormatSettings`, References section: lists "CompositorNodeOutputFile.format", "NodeCompositorFileOutputItem.format" and "RenderSettings.image_settings" — API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html, read 2026-10-11.
- The format list ("AVIF … TIFF … OPEN_EXR_MULTILAYER … FFMPEG: FFmpeg Video.") — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/image_type_all_items.html, read 2026-10-11.
- `active_item_index`: "Index of the active item (in [0, inf], default 0)" — API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html, read 2026-10-11.
- Picture `_images/node-types_CompositorNodeOutputFile.webp` (folder field shows "/tmp/", name field shows "file_name"). Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.

**Not covered.**

- Any upper limit on the number of outputs.
- Which formats the node's own format menu actually offers.
- Whether an output whose input is not connected is written at all. The manual says "Each connected input is saved", which suggests only connected ones are, but it does not say so outright.

**Where I looked.** I searched the whole 5.3 manual for "file output". It appears on seven pages: the File Output page, the Output nodes index, the node types index, File Paths, Rendering Animations, the Output properties page, and the general index. I searched the API for "CompositorNodeOutputFile", "OutputFile" and "FileOutput", and read the node, item, item-collection, settings and format-list pages. I searched the release notes for "file output", "output file", "fileoutput", "OutputFile" and "file_output", which found one entry (see question 2). I searched the API change log for the node and its items, and found no entries.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, the panel lists the outputs with a plus and a minus (block below). The original note from the sources follows. The API documents script functions for setting up outputs without clicking: `file_output_items.new(socket_type, name)` ("Add an item at the end"), `remove`, `clear` and `move`, plus the operators `node.file_output_item_add`, `node.file_output_item_remove` and `node.file_output_item_move` (https://docs.blender.org/api/5.3/bpy.types.NodeCompositorFileOutputItems.html and https://docs.blender.org/api/5.3/bpy.ops.node.html). The sources do not say whether what gets written differs when Blender is started from the command line or driven by a script.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say anything about the node and operating systems. The only statement anywhere about formats differing between systems belongs to the command-line render-format option: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). The sources do not say whether this also applies to the node.

**Observed by hand test (2026-10-18, entry 1).** A File Output node was made by script in a compositor tree (`tree.nodes.new('CompositorNodeOutputFile')`) and read back without rendering. It has nothing on its output side, it starts with one blank input socket and no named ones, and the output-slot types of older Blenders are not there:

```
NODE HAS OUTPUT SOCKETS: []
NODE INPUT SOCKETS AT START: [('', 'NodeSocketVirtual')]
TYPE NodeOutputFileSlotFile present=False
TYPE NodeOutputFileSlotLayer present=False
TYPE NodeCompositorFileOutputItem present=True
```

**A node made this way starts on the multi-layer OpenEXR format, not on the scene's PNG**, so left alone it writes one multi-layer file rather than separate pictures:

```
A NEW NODE STARTS ON: directory='/tmp/' file_name='{blend_name}' file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE use_file_extension=True save_as_render=True
THE SCENE STARTS ON: file_format=PNG media_type=IMAGE
```

**No ceiling was reached on how many outputs a node takes.** Outputs of the Colour kind were added one at a time, named `o0` upwards, on a third node:

```
OUTPUTS ADDED WITHOUT REFUSAL: 2000
INPUT SOCKETS ON THE NODE NOW: 2001
```

— two thousand outputs plus the blank socket, with nothing refused. That establishes at least two thousand and says nothing above it. An output asked for with an empty name was made and kept the empty name: `AN OUTPUT ASKED FOR WITH AN EMPTY NAME IS CALLED: ''`. Which kinds an output can be made with is under question 12; which formats the node will take, under question 9.

**Observed by hand test, from Blender's own window (2026-10-18, entry 17).** In the sidebar's Node tab, outputs are listed in an **Images** panel with a plus, a minus and up and down arrows beside the list, and the kind of thing the selected output carries is drawn as a **Socket Type** menu below it (reading Color). The full description of the panel, and how the window was driven, is under question 2.

---

## 2. Which settings belong to an individual output, and which belong to the node as a whole?

**Answer.**

**Settings of one output.** Documented. The API lists these for each output:

- its name;
- "override node format" (off by default);
- its own format settings, used when the override is on;
- its own "save as render" switch (off by default);
- its data type (default Float);
- a vector size (2 to 4);
- a read-only colour, which is the socket's colour in the editor.

The manual describes the same per-output override, which is set in the "Item Format" panel.

**Settings of the whole node.** Documented. The node has:

- Directory;
- File Name;
- the Node Format panel, which holds the Media Type and the default format for all outputs;
- its own "save as render" switch (on by default);
- File Extensions (on by default);
- which output is selected (active).

The general node settings every node has also apply, including mute (see question 8).

**Where the Media Type sits.** The manual lists Media Type among the node's own properties. In the API, the media type is a field of the format settings object (`media_type`), and the node's `format` is such an object. That the node's media type is node-wide is **documented**.

Each output's own `format` is the same kind of object, so it too has a media-type field. **Not in the sources:** what that field does on a single output, if anything.

**Format overrides in Multi-Layer EXR.** The manual describes per-output format overrides only under "Image" ("When saving individual images…"). For Multi-Layer EXR it says only "See OpenEXR for EXR-specific settings." From this I **infer** that per-output overrides are an Image-mode feature. The manual does not say outright that they are unavailable in Multi-Layer EXR.

**Default file name: the sources disagree.**

- The release notes say the node's default file name is `{blend_name}`.
- The API page gives the file name's default as an empty string.
- The manual's picture shows "file_name" in that field.

The sources do not settle which of these a newly added node shows.

**Evidence.**

- Per output: "override_node_format — Use a different format instead of the node format for this file (default False)"; "save_as_render — Apply render part of display transform when saving byte image (default False)"; "socket_type (default 'FLOAT')"; "vector_socket_dimensions — Dimensions of the vector socket (in [2, 4], default 0)"; "name (default “”, never None)"; "format (readonly)"; "color — Color of the corresponding socket type in the node editor (… readonly)". API, https://docs.blender.org/api/5.3/bpy.types.NodeCompositorFileOutputItem.html, read 2026-10-11.
- Node: "directory — The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)"; "file_name — The base name of the file. Other information might be included in the final file name depending on the node options (default “”, never None, Supports template expressions)"; "save_as_render — Apply render part of display transform when saving byte image (default True)"; "use_file_extension — Add the file format extensions to the saved file name (eg: filename + .jpg) (default True)"; "format (readonly)". API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html, read 2026-10-11.
- "media_type — The type of media to save (default 'IMAGE')" with values ‘IMAGE’, ‘MULTI_LAYER_IMAGE’, ‘VIDEO’. API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html, read 2026-10-11.
- "Image: Controls the default format and encoding for all outputs. To override settings for an individual output, enable Override Node Format in the Images/Layers panel." Also: "Multi-Layer EXR: See OpenEXR for EXR-specific settings." Also: "When saving individual images, each output can be renamed and will be written to disk using either the global Node Format settings or its own overridden settings." Also, Item Format: "When Override Node Format is enabled, this panel controls the format and encoding options for the selected output image." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- "The default file name of the **File Output** node is now `{blend_name}`, so rendered images will be named after the current blend file's name, instead of a hardcoded string. This is just the initial default value, the name can still be changed as usual." Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.
- Picture `_images/node-types_CompositorNodeOutputFile.webp` (file name field shows "file_name"). Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.

**Not covered.**

- When an output's own "save as render" is off, does the node's switch apply to it? The sources do not say.
- What an output's own media-type field does.
- What a new node's File Name actually shows.

**Where I looked.** The File Output manual page and its picture. The API pages for the node, the item, the item collection and the format settings. The 5.3 compositor release notes. The API change log, which has no entry for these.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The panel read with Blender's window open is described in the blocks below; a node added through the window starts with the scene's output path as its folder where a node made by script starts with `/tmp/`. The original note from the sources follows. Every setting named here appears as a property in the Python API, so a script can read and set it. That the properties exist is documented. The sources do not say whether any of them behaves differently outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 1).** **The full list of settings, from the running program**, with what it publishes as each one's default and description. On the node (the general settings every node has are left out):

```
    file_output_items | COLLECTION | read-only=True | default=- | description=''
    active_item_index | INT | read-only=False | default=0 | description='Index of the active item'
    directory | STRING | read-only=False | default='' | description='The directory where the image will be written'
    file_name | STRING | read-only=False | default='' | description='The base name of the file. Other information might be included in the final file name depending on the node options'
    format | POINTER | read-only=True | default=- | description=''
    save_as_render | BOOLEAN | read-only=False | default=True | description='Apply render part of display transform when saving byte image'
    use_file_extension | BOOLEAN | read-only=False | default=True | description='Add the file format extensions to the saved file name (eg: filename + .jpg)'
```

On one output:

```
    name | STRING | read-only=False | default='' | description=''
    socket_type | ENUM | read-only=False | default='FLOAT' | description=''
    vector_socket_dimensions | INT | read-only=False | default=0 | description='Dimensions of the vector socket'
    color | FLOAT | read-only=True | default=- | description='Color of the corresponding socket type in the node editor'
    override_node_format | BOOLEAN | read-only=False | default=False | description='Use a different format instead of the node format for this file'
    save_as_render | BOOLEAN | read-only=False | default=False | description='Apply render part of display transform when saving byte image'
    format | POINTER | read-only=True | default=- | description=''
```

**What a new node actually starts with is not what those published defaults say**, which settles the disagreement this answer found between the release notes, the API page and the manual's picture — for a node made by script. The program publishes an empty default for both path settings, but a node made with `tree.nodes.new('CompositorNodeOutputFile')` read back with a folder of `/tmp/` and a File Name of `{blend_name}`, the value the release notes give:

```
A NEW NODE STARTS ON: directory='/tmp/' file_name='{blend_name}' file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE use_file_extension=True save_as_render=True
DEFAULTS PUBLISHED FOR THE SETTINGS: directory='' file_name=''
```

The same holds for an output's own display-transform switch: published as off, it read back on for an output made with `file_output_items.new('RGBA', "Image")`, whose own format started on separate pictures in PNG:

```
ITEM DEFAULTS: override_node_format=False save_as_render=True item format=IMAGE/PNG
```

What a node added through Blender's own window shows is entry 17. What the starting File Name writes as is in the next block.

**Observed by hand test (2026-10-18, entry 2).** **The starting File Name `{blend_name}` is replaced by the blend file's name when the node writes — and by the word `Unsaved` when the file has never been saved.** A node made by script was given its own folder and one output `img`, and its File Name was left exactly as the node was made; a still render was started with `bpy.ops.render.render(write_still=True)`. Blender's own lines, first in a file never saved, then in the same set-up saved first as `deftest.blend`:

```
CASE a: node starts with file_name='{blend_name}'
CASE a: rendering with file_name='{blend_name}'
00:00.486  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/File output nodes/runs/e02/unsaved/a/Unsavedimg.png'
```

```
BLEND FILE: '/private/tmp/claude-501/smoozy-hand-tests-5.3/File output nodes/runs/e02/saved/deftest.blend'
CASE a: rendering with file_name='{blend_name}'
00:00.485  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/File output nodes/runs/e02/saved/a/deftestimg.png'
```

No error or warning was printed either time, and the template text stayed in the setting (`rendering with file_name='{blend_name}'`) — it was replaced only in the written name.

**Observed by hand test (2026-10-18, entry 5) — an output's own format on a Multi-Layer EXR node.** An output's own format can be set on a node set to Multi-Layer EXR, and it reads back as set, but it changed nothing written: with Override Node Format on and the output's own format set to separate pictures in PNG, the output was still written as a layer of the one EXR file, and no PNG was made (quoted under question 6). So the output's own media-type field took a value different from the node's, and the node's won.

**Observed by hand test (2026-10-18, entry 6) — which display-transform switch wins when the node and an output disagree.** Eight still renders, one node each with one output, every combination of the node's `save_as_render`, the output's own `save_as_render`, and the output's Override Node Format (with the output's own format set to PNG when on); the first pixel of each written picture read back. The scene's view transform was AgX:

```
SCENE VIEW TRANSFORM: AgX
    node save_as_render=False  output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.051, 0.051, 0.051, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=False  output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=False  output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.051, 0.051, 0.051, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=False  output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=True   output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.043, 0.043, 0.043, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=True   output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=True   output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.043, 0.043, 0.043, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
    node save_as_render=True   output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)   (node format PNG 16-bit, output format PNG 8-bit)
```

**While Override Node Format is off, only the node's switch changes the file; while it is on, only the output's own does** — which settles this answer's first open item: with the override off, the output's own switch is not used at all. The two halves are not on one scale, and the last part of each line shows why: the node's format had become 16-bit PNG when it was switched from the multi-layer OpenEXR it starts on, while the output's own format was 8-bit PNG.

**Observed by hand test, from Blender's own window (2026-10-18, entry 17).** Blender 5.3 was started from the Mac's Terminal **with its window open** on a saved file whose compositor tree held a Render Layers node, an Invert node and the Group Output node, with `--factory-startup --enable-event-simulate --window-geometry 0 0 1600 1000` — the last but one so that the real mouse and keyboard were ignored and only the script's own simulated ones reached Blender. A script closed the first-start splash with a simulated Escape, turned the large area into the compositor with its sidebar open, added a File Output node with Blender's own add-node command (`bpy.ops.node.add_node(type='CompositorNodeOutputFile', use_transform=False)`), gave it three outputs (two asked to be called `clash`, one `one`) linked from Render Layers, set its folder and File Name to `<W>/runs/w17/out/` (the work folder written `<W>`) and `shot_`, Image and PNG, hid the compositor's asset shelf to make room, opened the sidebar on its Node tab, and photographed the whole window with Blender's own screenshot command; menus and panels were then opened with simulated clicks, one picture after each. The pictures are described here; they were not kept.

**A node added through the window starts with the scene's own output path as its folder**, and `{blend_name}` as its File Name — not the `/tmp/` that a node made by script starts with:

```
ADD NODE THROUGH THE WINDOW RETURNED {'FINISHED'} ; ACTIVE NODE: CompositorNodeOutputFile File Output
A NODE ADDED THROUGH THE WINDOW STARTS ON: directory='<W>/runs/w17/scene/scene_' file_name='{blend_name}' media_type=MULTI_LAYER_IMAGE file_format=OPEN_EXR_MULTILAYER
```

(the saved file's scene output path was `<W>/runs/w17/scene/scene_`).

**What the sidebar's Node tab draws, with the File Output node selected.** A **Node** panel: Name, Label, Color, a **Show Options** checkbox (ticked), a **Mute** checkbox, and Propagate (reading All Messages). A **Properties** panel: the folder field with a folder button, the File Name field with a folder button, neither carrying a written label; then two buttons side by side, **Image** and **Multi-Layer EXR**, with no third; then a **Node Format** panel holding Save as Render (ticked), File Format (reading `PNG (.png)`), Color (BW, RGB, RGBA), Color Depth (8, 16), Compression (15%) and a closed Color Management panel; then an **Images** panel holding the list of outputs — `clash`, `clash_001`, `one` — with a plus, a minus and up and down arrows beside it, and below the list, belonging to the selected output, **Socket Type** (reading Color) and an **Override Node Format** checkbox (drawn cut short as `Override Node For...`); then an **Output Paths** panel, closed when the sidebar first opens, with a lone `+` drawn under its header; then a closed **Custom Properties** panel. Opened by a simulated click on its header, Output Paths holds a **File Extensions** checkbox, ticked, then one row per output, each a small picture icon and the finished path with its middle left out by Blender for width — `/private/tmp/claud...ut/shot_clash.png`, then `/private/tmp/claud...ot_clash_001.png` (the third row lay below the picture's edge).

**Two tooltips were read by resting the simulated mouse on a control.** On File Format: `File format to save the rendered images as: PNG (.png)` over `Output image in PNG format.` On the Override Node Format checkbox no tooltip had appeared after about 2.5 seconds, so its words were not read.

**The node's own face** carries a header with its label, the folder field and the File Name field one above the other with a folder button each, one row per output showing its name alone (`clash`, `clash_001`, `one`), and a last row holding a `+`.

---

## 3. Does each output carry its own folder as well as its own name?

**Answer.** No separate folder setting per output is documented. There is one Directory, and it belongs to the node. An output can still put its file in a subfolder, through its name (see question 4).

- That there is no per-output folder setting is **inferred**. The step: the API page for an output lists every one of its properties (name, override node format, format, save as render, data type, vector size, colour), and none of them is a folder. The manual lists Directory only among the node's own properties.
- That an output's name can create subfolders is **documented**, for Image mode.

**Evidence.**

- Node property "Directory — The base output directory." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- Properties of `NodeCompositorFileOutputItem`: "color", "format", "name", "override_node_format", "save_as_render", "socket_type", "vector_socket_dimensions" (no directory or folder property). API, https://docs.blender.org/api/5.3/bpy.types.NodeCompositorFileOutputItem.html, read 2026-10-11.
- "When the Media Type is set to Image, input names are also used to construct subdirectories" — same manual page.

**Not covered.** Whether a folder written into an output's name can be absolute, or can step upwards out of the node's Directory. The sources say nothing about either.

**Where I looked.** The File Output manual page. The API pages for the node and the output. I also searched the manual for other mentions of "file output"; there are no others about folders.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The original note from the sources follows. The node's `directory` and each output's `name` are both script-settable properties (documented). The sources do not say whether behaviour differs outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 3).** **An output has no folder of its own, and its name cannot climb out of the node's folder.** One node in Image mode, with its folder set to `<work folder>/runs/e03/out/base` and its File Name `file_name`, was given five outputs. A name reaching upwards was changed before the node kept it, and a name starting with a slash was kept:

```
ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
```

A still render wrote both inside the node's folder (the work folder written `<W>`):

```
00:00.489  render           | Saved: '<W>/runs/e03/out/base/file_name_./escape.png'
00:00.490  render           | Saved: '<W>/runs/e03/out/base/file_name/absolute.png'
```

and nothing landed in `out` itself or anywhere above `base`. So the node's folder is a floor the outputs' names cannot climb out of, and a leading slash does not make a name's path absolute — it is joined under the node's folder, after the File Name.

---

---

## 4. Can a folder be expressed inside an output's name rather than separately?

**Answer.** Yes, when the Media Type is Image. This is **documented**:

- a forward slash or a backslash in an output's name is read as a folder path;
- any missing folders are created when the files are written.

**Not in the sources:** what a slash in an output's name does in Multi-Layer EXR mode. The manual ties the folder behaviour to Image mode only.

**Ambiguity.** "An output's name" most likely means the name of one output (one input socket), and that is how I have answered. A second reading is the node's own File Name field. For that field the sources say only that it is "The base name of the file" and that it supports template expressions. They do not say whether a slash in it makes a folder.

**Evidence.** All from the manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11:

- "When the Media Type is set to Image, input names are also used to construct subdirectories:"
- "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy."
- "Any missing directories are created automatically during output."
- "This allows organizing outputs into folders directly from the node setup, for example: passes/diffuse or masks/object_01."

**Not covered.**

- Multi-Layer EXR mode.
- A slash in the node's File Name.
- Whether template expressions such as `{scene_name}` work inside an output's name. The API marks the node's directory and file name as "Supports template expressions", but its entry for an output's name carries no such note.

**Where I looked.** The File Output manual page, and File Paths (https://docs.blender.org/manual/en/5.3/files/file_paths.html). The API pages for the node and the output.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The original note from the sources follows. An output's name is a script-settable property (documented). The sources do not say whether the folder behaviour differs outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. On that Mac a backslash in an output's name made a folder exactly as a forward slash did; on Windows, whose own folder separator is the backslash, it was not tried. The original note from the sources follows. The manual accepts both slash characters: "(/ or \)". It says nothing else about operating systems here.

**Observed by hand test (2026-10-18, entry 2).** **A template inside an output's own name is replaced**, which this answer left open. A node with File Name `file_name` and one output asked to be called `{scene_name}_img` kept the name as typed (`CASE c: output asked for '{scene_name}_img', node kept '{scene_name}_img'`) and wrote, from a still render in Blender's starting scene (called `Scene`):

```
00:00.497  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/File output nodes/runs/e02/unsaved/c/file_nameScene_img.png'
```

The same name came out with the file saved first. **A slash in an output's name still makes a folder when the File Name is left at its starting `{blend_name}`**: an output named `sub/img` under the untouched File Name wrote

```
00:00.492  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/File output nodes/runs/e02/unsaved/b/Unsavedsub/img.png'
```

in a file never saved, and `.../saved/b/deftestsub/img.png` with the file saved as `deftest.blend` — the File Name joined to the part before the slash, then the folder.

**Observed by hand test (2026-10-18, entry 3).** **Folders are made from a name in Image mode, several levels deep, and a backslash does the same as a forward slash.** Four nodes in one tree, one still render. Node (a): Image mode, folder `<W>/runs/e03/out/base`, File Name `file_name`, five outputs; the names were kept as typed, apart from the one reaching upwards (under question 3):

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
```

Node (b): one output only, named `sub\shot`, folder `.../out/base2`. Node (c): Multi-Layer EXR, one output `passes/diffuse`, folder `.../out/ml/`. Node (d): Image mode, File Name `sub/test`, one output `img`, folder `.../out/fn`. Blender's own lines, and what was on disk afterwards:

```
00:00.489  render           | Saved: '<W>/runs/e03/out/base/file_name_./escape.png'
00:00.490  render           | Saved: '<W>/runs/e03/out/base/file_name/absolute.png'
00:00.490  render           | Saved: '<W>/runs/e03/out/fn/sub/testimg.png'
00:00.491  render           | Saved: '<W>/runs/e03/out/base/file_namea/b/c/deep.png'
00:00.492  render           | Saved: '<W>/runs/e03/out/base/file_namesub/shot.png'
00:00.493  render           | Saved: '<W>/runs/e03/out/base/file_namesub/shot.png'
00:00.493  render           | Saved: '<W>/runs/e03/out/ml/file_name.exr'
00:00.494  render           | Saved: '<W>/runs/e03/out/base2/file_namesub/shot.png'
00:00.495  render           | Saved: '<W>/runs/e03/scene/scene_.png'
RESULT [5.3.0 Beta] separators in names, all four nodes, folder <W>/runs/e03/out
    out/base/file_name/absolute.png  3183 bytes
    out/base/file_name_./escape.png  3183 bytes
    out/base/file_namea/b/c/deep.png  3183 bytes
    out/base/file_namesub/shot.png  3183 bytes
    out/base2/file_namesub/shot.png  3183 bytes
    out/fn/sub/testimg.png  3183 bytes
    out/ml/file_name.exr  67351 bytes
```

Three things this shows. **The node's File Name is joined to the front of the name before it is split**, so `sub/shot` under `file_name` makes a folder called `file_namesub`. **A backslash makes a folder on this Mac exactly as a forward slash does** — node (b), whose only output is `sub\shot`, wrote `base2/file_namesub/shot.png`; and on node (a), `sub/shot` and `sub\shot` both went to the one path `base/file_namesub/shot.png`, which Blender reported saving twice, with no warning, and which is one file afterwards. **A slash in the node's own File Name makes a folder too**: `sub/test` with output `img` wrote `fn/sub/testimg.png`.

**In Multi-Layer EXR mode the slash is kept in the layer's name and no folder is made.** Node (c) left the one file `ml/file_name.exr`, with no `passes` folder, whose header (read from the file by the test script) holds:

```
EXR ml/file_name.exr PARTS AND CHANNELS: [('passes/diffuse', ['passes/diffuse.A', 'passes/diffuse.B', 'passes/diffuse.G', 'passes/diffuse.R'])]
```

— one part named `passes/diffuse`, its four channels named `passes/diffuse.A`, `.B`, `.G` and `.R`. Nothing was reported to the terminal in any of these cases. What happens when a folder cannot be made is under questions 5 and 15.

---

## 5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?

**Answer.**

- **Relative to the Blender file.** Documented. The node's Directory supports the `//` prefix, meaning "relative to the Blender file".
- **Template expressions.** Documented. The Directory supports template expressions. Among the variables available are the Blender file's own folder (`{blend_dir}`), and for linked data the library file's folder (`{blend_dir_lib}`). The 5.3 release notes add `{project_root}` and custom project variables for path templates.
- **Folders unrelated to the Blender file.** The manual's picture of the node shows "/tmp/" in the Directory field, which is a full path with no connection to the Blender file. This is **documented** in the sense that the picture shows it. It is **inferred** that any full (absolute) folder path can be entered. The step: the field is shown holding one. No sentence states this outright.
- **Network folders.** **Not in the sources** for this node. Three nearby statements exist, and none settles it:
  - The render Output panel's "Primitive Render Farm" hint says to "Set up a shared directory over a network file system". That hint is about the render output, not this node.
  - The File Browser's "Volumes" list is described as "Drives and network mounts." The node's picture shows a folder button beside the Directory field. That this button opens the File Browser, and so could reach network mounts, is a chain of guesses the sources do not confirm.
  - Network shares on Windows are mentioned only for extension repositories, which is unrelated.

**Evidence.**

- "directory — The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)". API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html, read 2026-10-11.
- "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node." Also: "blend_dir: Path up to (but not including) the currently open blend file." Also: "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." Manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html, read 2026-10-11.
- "`{project_root}` and custom project variables for path templates." Also "each project has a root directory". Release notes, https://developer.blender.org/docs/release_notes/5.3/core/, read 2026-10-11.
- "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file." and "when entering the path into a text field, use a double slash prefix (//) to make it so." Manual, https://docs.blender.org/manual/en/5.3/files/blend/open_save.html, read 2026-10-11.
- Picture `_images/node-types_CompositorNodeOutputFile.webp` (folder field shows "/tmp/"). Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- "Set up a shared directory over a network file system." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11.
- Volumes: "Drives and network mounts." Manual, https://docs.blender.org/manual/en/5.3/editors/file_browser.html, read 2026-10-11.

**Not covered.**

- Network paths in any form: mapped drives, mounted shares, or Windows share paths.
- What happens with an empty Directory.
- What a `//` path does in a Blender file that has never been saved. The manual's Relative Paths note says "You cannot use relative paths into a new untitled blend-file", but in the context of linking files, not of this node.
- How `{project_root}` behaves in this node. The release note covers path templates in general.

**Where I looked.**

- I searched the whole manual for these whole words: "network", "UNC", "nfs", "smb", "samba", "mounted", "network drive", "network share", "shared directory", "file server". The hits were the render-farm hint, the File Browser's Volumes line, an extension-repository line about Windows network shares, the deploying-Blender page (a read-only system repository that "could be located on a read-only network drive"), and unrelated uses of "network" (node networks, internet settings).
- I searched the manual for "project_root", "project variable" and "blender_project", and found nothing.
- I read the File Paths page, the Relative Paths section, and the node's API page.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The original note from the sources follows. The Directory is a script-settable property (documented). The command-line option `-o` "Set the render path and file name". The sources do not say it changes File Output folders (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). Template variables are filled in "at time of use (e.g. while rendering)". The sources do not say whether this differs for command-line renders.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. On that Mac an empty folder setting aimed the write at `/`, which its ordinary user may not write to; a network folder was not tried on any machine. The original note from the sources follows. The sources do not say how folder paths behave on each operating system for this node. The File Browser page mentions system folders "such as the home directory in Linux or the “Documents” folder in Windows", which says nothing about this node.

**Observed by hand test (2026-10-18, entry 4).** Blender was started from the folder `<W>/runs/e04/cwd` (the work folder written `<W>`), so that anything written relative to the starting folder would land there. One node per case, File Name `fo<case>_`, one output `made`, a still render with `bpy.ops.render.render(write_still=True)`; after each, the whole work folder, `/tmp` and `/` were searched for the file. Cases (a) to (e) ran in a file never saved; the file was then saved as `<W>/runs/e04/out/held.blend` for (f) to (i). Every render returned `{'FINISHED'}`, and the scene's own picture was saved every time.

**A folder unrelated to the Blender file, and one that does not exist yet, are both accepted; the missing one is made** — case (a):

```
00:00.476  render           | Saved: '<W>/runs/e04/out/does/not/exist/yet/foa_made.png'
FOLDER <W>/runs/e04/out/does/not/exist/yet EXISTS AFTERWARDS: True
```

**`//` is relative to the saved Blender file** — case (f), file saved at `<W>/runs/e04/out/held.blend`:

```
00:00.513  render           | Saved: '<W>/runs/e04/out/relative/fof_made.png'
```

**In a file never saved, `//relative/` wrote relative to the folder Blender was started from**, with no error — case (b):

```
00:00.485  render           | Saved: 'relative/fob_made.png'
RESULT [5.3.0 Beta] node folder set to the text //relative/ with the file never saved (blend file '')
    <W>/runs/e04/cwd/relative/fob_made.png  3183 bytes
```

**An empty folder setting tried to write at the top of the disk, `/`, and failed loudly**, saved or not — cases (c) and (g):

```
00:00.491  image.write      | ERROR OpenImageIO write failed: Could not open file "/foc_made.png"
00:00.491  reports          | ERROR Render error (Undefined error: 0) cannot save: '/foc_made.png'
```

```
00:00.519  image.write      | ERROR OpenImageIO write failed: Could not open file "/fog_made.png"
00:00.519  reports          | ERROR Render error (Undefined error: 0) cannot save: '/fog_made.png'
```

and no such file was found anywhere searched. The failure here is the Mac's ordinary user not being allowed to write at `/`; what an account that is allowed to would get was not tried.

**A folder that cannot be written to (mode 500) fails loudly and writes nothing** — case (d):

```
00:00.497  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e04/out/locked/fod_made.png"
00:00.497  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e04/out/locked/fod_made.png'
```

**`{project_root}` with no project made is a template error, and the node skips its file** — cases (e), never saved, and (h), saved; the same single line both times:

```
00:00.501  reports          | ERROR Invalid path template in File Output node. Skipping writing file.
```

**`{blend_dir}` names the saved file's folder** — case (i), `{blend_dir}/viatemplate/`:

```
00:00.531  render           | Saved: '<W>/runs/e04/out/viatemplate/foi_made.png'
```

How `{project_root}` behaves when a project does exist was not tried; see *Cannot be run* in the hand-test list. Network folders were not tried and could not be: the Mac has no network share, and making one would change its system settings.

---

## 6. Can several outputs be gathered into one file rather than written separately?

**Answer.** Yes. With the Media Type set to Multi-Layer EXR, all of the node's inputs are saved together in one multi-layer OpenEXR file, each input as its own layer. This is **documented**.

Because the Media Type is set once for the whole node (question 2), I **infer** that one node cannot put some outputs into a shared file and write others separately. It is all one file or all separate files. The step: Media Type is a node property that "Determines how multiple inputs are stored". To get a mix, separate nodes would be needed. That too is inferred, not stated.

**Evidence.**

- "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- Same page: "When using Multi-Layer EXR, each input is stored as a separate layer within the file."
- Same page: "Media Type — Determines how multiple inputs are stored:".

**Not covered.**

- What each layer inside the file is named. The sources do not say it is the input's name.
- Whether outputs from different nodes can be gathered into one file.

**Where I looked.** The File Output manual page. The API pages for the node and the format settings. The OpenEXR part of the Supported Image Formats page (https://docs.blender.org/manual/en/5.3/files/media/image_formats.html).

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The original note from the sources follows. The media type is a script-settable field of the node's format settings (`media_type`, value ‘MULTI_LAYER_IMAGE’; documented). The sources do not say whether behaviour differs outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say. The command-line page lists "OPEN_EXR_MULTILAYER" among formats "not available on all systems", but only for the render-format option (see question 1).

**Observed by hand test (2026-10-18, entry 5).** **The layers inside the one file are named after the outputs, and the file is named from the node's File Name alone.** A node set to Multi-Layer EXR with three outputs `first`, `second` and `third`, all fed the render picture, folder `<W>/runs/e05/a/` (the work folder written `<W>`), File Name `file_name`, still render:

```
00:00.416  render           | Saved: '<W>/runs/e05/a/file_name.exr'
RESULT [5.3.0 Beta] three outputs, Multi-Layer EXR, folder written with a trailing slash
    out/file_name.exr  202193 bytes
    EXR file_name.exr PARTS AND CHANNELS: [('first', ['first.A', 'first.B', 'first.G', 'first.R']), ('second', ['second.A', 'second.B', 'second.G', 'second.R']), ('third', ['third.A', 'third.B', 'third.G', 'third.R'])]
```

The last line is the file's own header, read by the test script: one part per output, named after it, each with four channels. The same file name came out with the folder written without a trailing slash (`<W>/runs/e05/b`): `Saved: '<W>/runs/e05/b/file_name.exr'`, the same three parts.

**An output set to its own format still goes into the one file.** With output `second` given Override Node Format and its own format set to PNG — both taken, `OUTPUT 'second': override_node_format=True, item format IMAGE/PNG; node format MULTI_LAYER_IMAGE/OPEN_EXR_MULTILAYER` — the render left one file, `c/file_name.exr`, holding the same three parts, and no PNG. This bears on the inference above that a node cannot mix shared and separate files: in this case it did not.

**Two Multi-Layer EXR nodes aimed at the same folder and File Name gave one file holding both nodes' outputs.** Two nodes, one output each, `from_node_one` and `from_node_two`:

```
00:00.495  render           | Saved: '<W>/runs/e05/d/file_name.exr'
00:00.495  render           | Saved: '<W>/runs/e05/d/file_name.exr'
RESULT [5.3.0 Beta] two Multi-Layer EXR nodes, same folder, same File Name, one output each
    out/file_name.exr  134967 bytes
    EXR file_name.exr PARTS AND CHANNELS: [('from_node_two', ['from_node_two.A', 'from_node_two.B', 'from_node_two.G', 'from_node_two.R']), ('from_node_one', ['from_node_one.A', 'from_node_one.B', 'from_node_one.G', 'from_node_one.R'])]
```

Blender reported saving the path twice; the one file left holds two parts, one from each node, the second-made node's first. Nothing was reported to the terminal in any of these cases. How the same thing goes in Image mode is under question 10.

---

## 7. What happens to a File Output node inside a group of nodes?

**Answer.** **Not in the sources.** Nothing in the manual, the API or the release notes says whether a File Output node inside a node group writes, writes differently, or is ignored.

Related statements, none of which settles the question:

- The general node-groups page advises against placing output nodes inside groups: "Avoid using output nodes such as Material Output inside node groups. These should be used on the top level node tree to improve re-usability of node groups." This is advice about reuse, not a statement of what happens.
- The compositor introduction says: "Nodes that are not connected to a Group Output are ignored during evaluation." Read literally, this would apply to any node, including a File Output node (which has no output sockets to connect). The sources do not say how this sentence relates to the File Output node, either at the top level or inside a group.
- The release notes say the compositor now runs as a stack of effects, each controlled by a node group. So in 5.3 the scene's compositor tree is itself a node group. Separately: "The compositor now renders scenes referenced inside node groups." That note is about scenes, not File Output nodes.

**Ambiguity.** "A group of nodes" most likely means a node group placed as a group node inside a compositor tree, and that is what I answered. Other readings:

- the node group that makes up one compositor effect;
- a Frame, which gathers nodes visually.

The sources do not answer any of these readings for the File Output node.

**Observed by hand test (2026-10-18, entry 7) — a node group placed as a group node.** **The node writes from inside a group, two groups deep as well, and once for each time the group is used.** A group was made holding a File Output node (folder `<W>/runs/e07/out/`, the work folder written `<W>`; File Name `g_`; one output `grouped`) fed from the group's own input, and the group node was fed the render picture at the top of the scene's tree. Four still renders, Blender's own lines between markers:

```
--- RENDER START: one File Output node inside a group, the group used once
00:00.516  render           | Saved: '<W>/runs/e07/out/g_grouped.png'
00:00.517  render           | Saved: '<W>/runs/e07/scene/scene_.png'
--- RENDER END: one File Output node inside a group, the group used once
--- RENDER START: the same group used twice in the tree
00:00.522  render           | Saved: '<W>/runs/e07/out/g_grouped.png'
00:00.523  render           | Saved: '<W>/runs/e07/out/g_grouped.png'
00:00.524  render           | Saved: '<W>/runs/e07/scene/scene_.png'
--- RENDER END: the same group used twice in the tree
--- RENDER START: the File Output node two groups deep
00:00.528  render           | Saved: '<W>/runs/e07/out/g_grouped.png'
00:00.529  render           | Saved: '<W>/runs/e07/scene/scene_.png'
--- RENDER END: the File Output node two groups deep
--- RENDER START: the outer group node muted
00:00.533  render           | Saved: '<W>/runs/e07/scene/scene_.png'
--- RENDER END: the outer group node muted
```

Each run left one file, `out/g_grouped.png`, except the last, which left nothing. **Used twice, the group's node wrote twice to the same path** — two `Saved:` lines — and one file was left, since both uses carry the same folder and name. **Muting the outer group node from outside stopped the writing.** Nothing else was reported to the terminal in any of the four.

**Observed by hand test (2026-10-18, entry 12) — the node group that makes up one compositor effect.** A File Output node in the tree of the second of two compositor effects wrote during a render, whichever effect was the active one, and wrote nothing once that effect's *Use effect during render* was off. The output is quoted under question 13.

**Evidence.**

- Advice quoted above — manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/groups.html, read 2026-10-11.
- "A compositor node tree is evaluated from the outputs backwards to determine which nodes contribute to the final result. Nodes that are not connected to a Group Output are ignored during evaluation." Manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html, read 2026-10-11.
- "The compositor now uses an effects stack instead of a single compositor node group." and "The compositor now renders scenes referenced inside node groups." Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.
- `SceneCompositorEffect.node_group`: "Node group that controls what this effect does". API, https://docs.blender.org/api/5.3/bpy.types.SceneCompositorEffect.html, read 2026-10-11.

**Not covered.** All of it.

**Where I looked.**

- Every manual page that contains "file output" (seven pages; see question 1). None mentions groups.
- The compositor's Group page (https://docs.blender.org/manual/en/5.3/compositing/types/groups.html), the general Node Groups page, and the compositor introduction, Compositor System and Using the Compositor pages.
- I searched the Node Groups page for "output node" and "viewer".
- I searched the 5.3 release notes for "node group", "output" and "compositor".
- The API pages for the node, the compositor node tree and the compositor effect.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The original note from the sources follows. The sources do not say.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

---

## 8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?

**Answer.**

**One whole node.** Every node has a mute setting, and the File Output node inherits it. That the setting exists is **documented**. The manual says muting a node "removes its contribution to the node tree". That a muted File Output node therefore writes nothing is **inferred**. The step: removing its contribution would include its writing. The sources never say this for this node. The Output panel speaks of "active File Output Node nodes", but does not define "active".

**One output within a node.** **Not in the sources** as a dedicated switch. An output's own settings (question 2) include no on/off switch. Three routes exist in the sources: muting an incoming link is **inferred** to work; disabling or hiding a socket is documented as a socket property, but its effect on writing is **not in the sources**; and deleting the output is removal, not switching off:

- Every node socket has an "enabled" setting and a "hide" setting. The sources do not say what disabling one of this node's sockets does to writing.
- A single link can be muted. A muted link "acts as though it’s no longer there", and the manual says "Each connected input is saved". From these I infer that muting the link into one output might stop that output being written. Not stated.
- Deleting the output removes it. That is removal, not switching off.

**Evidence.**

- `Node.mute`: "(default False)" — API, https://docs.blender.org/api/5.3/bpy.types.Node.html, read 2026-10-11. The File Output node's API page lists "Node.mute" among its inherited properties (https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html).
- "Muting a node removes its contribution to the node tree, and makes all links pass through it without change." and "A muted link acts as though it’s no longer there; this also means the input fields for specifying fixed values become visible again." Manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/editing.html, read 2026-10-11.
- Mute: "Temporarily disables the node without removing it. When possible, the node is bypassed and its inputs are passed directly to its outputs." Manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/node_editors.html, read 2026-10-11.
- "if compositing is enabled, active File Output Node nodes are used to save the result." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11.
- `NodeSocket.enabled`: "Enable the socket (default True)"; `hide`: "Hide the socket (default False)". API, https://docs.blender.org/api/5.3/bpy.types.NodeSocket.html, read 2026-10-11.
- "Each connected input is saved to the specified directory and filename". Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.

**Ruled out.** The 5.3 release notes, under Node Editor, list "Support for disabling outputs through switch nodes". It does not mention the File Output node, and nothing ties it to one. The compositor's Enable Output node concerns node-group outputs ("toggles the visibility and value of a socket for a Node Group outputs"), not File Output outputs.

**Not covered.**

- That a muted File Output node writes nothing.
- What disabling, hiding or unlinking one input does to writing.

**Where I looked.**

- I searched the whole manual for the whole words "mute", "muted" and "muting", leaving out sound, strip and animation-channel hits.
- The Node, NodeSocket, File Output node and output-item API pages.
- The Enable Output page (https://docs.blender.org/manual/en/5.3/compositing/types/output/enable_output.html).
- The release notes for "mute", "disabl" and "output".

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, the panel offers no per-output switch (block below). The original note from the sources follows. `mute` on the node and `enabled` / `hide` on sockets are script-settable properties (documented). The sources do not say whether muting behaves differently outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 8).** **Muting the node stops it writing**, which this answer had as inferred; **one output is switched off on its own by muting or removing its link**, and an output with nothing connected writes nothing at all — no empty file and no black one. One node, folder `<W>/runs/e08/out/` (the work folder written `<W>`), outputs `A` (fed the render picture) and `B` (fed the render through an Invert node), a fresh tree for each still render:

```
--- RENDER START: both outputs connected
00:00.482  render           | Saved: '<W>/runs/e08/out/file_nameA.png'
00:00.483  render           | Saved: '<W>/runs/e08/out/file_nameB.png'
--- RENDER START: the whole node muted (node.mute = True)
RESULT [5.3.0 Beta] the whole node muted (node.mute = True)
    (nothing written)
--- RENDER START: node unmuted, the link into output A muted (link.is_muted = True)
00:00.493  render           | Saved: '<W>/runs/e08/out/file_nameB.png'
--- RENDER START: the link into output A removed, the output itself left on the node
00:00.498  render           | Saved: '<W>/runs/e08/out/file_nameB.png'
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
```

(The scene's own picture was saved every time and those lines are cut, as are the `RENDER END` markers.) **The socket's own `enabled` and `hide` settings do not switch an output off.** With both outputs linked, A's socket set to `enabled = False` read back False and A was still written; A's socket set to `hide = True` read back False — the setting did not take on a linked socket — and A was still written:

```
SOCKET A enabled SET TO False, reads back: False
--- RENDER START: both linked, socket A's own 'enabled' set to False
00:00.503  render           | Saved: '<W>/runs/e08/out/file_nameA.png'
00:00.504  render           | Saved: '<W>/runs/e08/out/file_nameB.png'
SOCKET A hide SET TO True, reads back: False
--- RENDER START: both linked, socket A's own 'hide' set to True
00:00.509  render           | Saved: '<W>/runs/e08/out/file_nameA.png'
00:00.510  render           | Saved: '<W>/runs/e08/out/file_nameB.png'
```

Nothing was reported to the terminal in any of the six renders. 

**Observed by hand test, from Blender's own window (2026-10-18, entry 17).** **The panel offers no per-output on/off switch.** The Images list has a plus, a minus and up and down arrows; the only controls belonging to the selected output are Socket Type and Override Node Format (described under question 2). A simulated right-click on the list entry `clash` opened Blender's ordinary property menu and nothing else: a header reading `Active Item Index`, then Reset to Default Value, Copy to Selected (greyed out), Copy Data Path, Copy Full Data Path, Copy as New Driver, and Online Manual. So removing an output, or muting or removing its link as in the block above, is the way.

---

## 9. What restrictions, if any, apply to the kind of picture a File Output node can write?

**Answer.** Documented restrictions found:

1. **Two media types for the node: Image and Multi-Layer EXR.** Video is dealt with in question 11.
2. **Multi-Layer EXR means OpenEXR.** The single combined file is "a single multi-layer OpenEXR file".
3. **Image mode uses the general image settings,** and those carry per-format limits:
   - "RGBA is not available for all image formats";
   - "not all file formats support all color depths";
   - Float (Half) and Float (Full) are "Only supported for OpenEXR files";
   - the black-and-white colour mode is "only PNG, JPEG, TGA, TIF";
   - DPX and Cineon "only stores and converts the “visible” color range of values between 0.0 and 1.0".

   The Supported Image Formats table gives each format's bit depths and whether it has alpha.

In addition, one restriction is **inferred**. The compositor limitations page says that with the GPU compositor, "Images with resolution larger than 8k are not supported and are cropped/scaled." The step: a picture reaching a File Output node through the GPU compositor would be subject to this. The sources do not say so for this node.

**Evidence.**

- Media Type lines — manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- "Note, RGBA is not available for all image formats, check the list above for details." Also: "Note, not all file formats support all color depths." Also: "Float (Half): … Only supported for OpenEXR files." Also: "DPX as well as Cineon only stores and converts the “visible” color range of values between 0.0 and 1.0 (as a result of rendering or composite)." Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html, read 2026-10-11.
- "BW: BW. Images get saved in 8-bit grayscale (only PNG, JPEG, TGA, TIF)." API, https://docs.blender.org/api/5.3/bpy_types_enum_items/image_color_mode_items.html, read 2026-10-11.
- "General hardware limitations apply when using the GPU compositor." and "Images with resolution larger than 8k are not supported and are cropped/scaled." Manual, https://docs.blender.org/manual/en/5.3/compositing/limitations.html, read 2026-10-11.

**Not covered.**

- Whether the node's format menu is narrower than the general list.
- How an output's data type (Float, Vector, Colour) turns into channels in the written file.
- Whether the 8k limit applies to what the node writes.

**Where I looked.** The File Output page; Supported Image Formats; the render Output properties page; Compositor Limitations; Compositor System. The API pages for the format settings, the format list, the colour-mode list and the colour-depth list. The 5.3 release notes (pipeline and compositor pages).

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, the File Format menu could not be opened by the script, so whether the window offers exactly the formats the setting takes was not read. The original note from the sources follows. The sources do not say whether these limits differ outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The only system-related statements are these two:

- The command-line format option says OpenEXR, OpenEXR MultiLayer, FFmpeg, Cineon, DPX, JPEG 2000 and WebP are "Formats that can be compiled into Blender, not available on all systems". This is stated for the render-format option, not the node.
- The limitations page covers Windows GPU driver resets (TDR) during GPU compositing, which is not a limit on the kind of picture.

The sources do not say whether format availability for the node differs by system.

**Observed by hand test (2026-10-18, entry 1).** **The node's own format list is narrower than the shared one, and what it will take depends on the media type.** The format setting publishes sixteen values and three media types:

```
FILE FORMAT VALUES: ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
MEDIA TYPE VALUES: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
```

but on a File Output node set to Image it refused both the multi-layer value and the video value, and each refusal names the fourteen it will take:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in (\'AVIF\', \'JPEG\', \'OPEN_EXR\', \'PNG\', \'WEBP\', \'BMP\', \'CINEON\', \'DPX\', \'IRIS\', \'JPEG2000\', \'HDR\', \'TARGA\', \'TARGA_RAW\', \'TIFF\')')
WITH media_type='IMAGE', SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in (\'AVIF\', \'JPEG\', \'OPEN_EXR\', \'PNG\', \'WEBP\', \'BMP\', \'CINEON\', \'DPX\', \'IRIS\', \'JPEG2000\', \'HDR\', \'TARGA\', \'TARGA_RAW\', \'TIFF\')')
WITH media_type='IMAGE', SETTING file_format='PNG' -> accepted, reads back 'PNG'
```

Setting the media type moves the format with it: `SETTING media_type='IMAGE' -> accepted, reads back 'IMAGE', file_format now 'PNG'` and `SETTING media_type='MULTI_LAYER_IMAGE' -> accepted, reads back 'MULTI_LAYER_IMAGE', file_format now 'OPEN_EXR_MULTILAYER'`. The video media type is refused outright (quoted under question 11). So on Image the node takes exactly the fourteen still-picture formats, AVIF among them, and neither multi-layer OpenEXR nor video. This is one build — Blender's own 5.3.0 Beta for Apple silicon — and says nothing about another build. What the panel's own menu draws is entry 17.

**Observed by hand test, from Blender's own window (2026-10-18, entry 17).** The node's File Format menu read `PNG (.png)`, and its tooltip read `File format to save the rendered images as: PNG (.png)` over `Output image in PNG format.` **The menu itself could not be opened by the script**: a simulated click on it, a simulated press with no release, and a simulated Return key with the mouse resting on it each left the picture unchanged — while a simulated right-click elsewhere on the same panel did open its menu and was photographed. So which entries the menu draws, and in what order, was not read; what the setting takes is in the block above.

---

## 10. What happens when two outputs are aimed at the same name in the same place?

**Answer.** **Not in the sources.**

**Ambiguity.** "Two outputs" could mean two outputs of one node, or outputs of two different nodes. The sources answer neither.

Related facts, none of which settles it:

- The path is built as "{Directory}/{File Name}{Socket Name}{frame number}.{extension}". So two outputs collide only if every one of these parts matches. This is **inferred** from the pattern.
- Node sockets have a "Unique identifier for mapping sockets". The sources say nothing about whether output names must differ.
- "Overwrite existing files" is documented only for the render Output panel.

**Evidence.**

- "For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- `NodeSocket.identifier`: "Unique identifier for mapping sockets (default “”, readonly, never None)". API, https://docs.blender.org/api/5.3/bpy.types.NodeSocket.html, read 2026-10-11.
- `use_overwrite`: "Overwrite existing files while rendering (default True)" — on `RenderSettings`, not on the node. API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.

**Not covered.** All of it: whether names are made unique, whether one file overwrites the other, which one wins, and whether a warning appears.

**Where I looked.**

- I searched the compositing, render-output and media pages for "overwrit", "same name", "duplicate name", "unique name", "name collision", "clash" and "conflict". The only hits were the render Output panel's Overwrite setting, the File Output tip about not overwriting the main render result, and Menu Switch "conflict" (unrelated).
- I searched the node pages and compositing pages for "unique". Hits were node names, socket identifiers, the Format String node's own input names, and random/hash values; none about File Output.
- The output-item API page (its `name` property carries no uniqueness note) and the add-item operator.
- The release notes, which had nothing on this.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, the Output Paths panel showed two different paths and no warning. The original note from the sources follows. The sources do not say.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say. Whether names differing only in upper/lower case count as "the same name" on some systems is not addressed.

**Observed by hand test (2026-10-18, entry 9).** Folder `<W>/runs/e09/out/` (the work folder written `<W>`), File Name `file_name`, still renders unless said; one picture is the render, the other the render through an Invert node, told apart by the first pixel. The scene's own `Saved:` line after each render is cut below.

**Two outputs on one node cannot carry the same name — the second is renamed**, and both are written, each holding its own picture:

```
ONE NODE, TWO OUTPUTS BOTH ASKED FOR 'clash' -> node kept ['clash', 'clash_001']
00:00.477  render           | Saved: '<W>/runs/e09/out/file_nameclash.png'
00:00.478  render           | Saved: '<W>/runs/e09/out/file_nameclash_001.png'
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
```

**Two separate nodes aimed at the same file both write it, and the file left holds the picture of the node made first.** No warning or error was printed, and four renders in a row gave the same:

```
--- RENDER START: two separate File Output nodes, same folder, same output name (plain picture's node made first)
00:00.485  render           | Saved: '<W>/runs/e09/out/file_namesame.png'
00:00.486  render           | Saved: '<W>/runs/e09/out/file_namesame.png'
RESULT [5.3.0 Beta] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  3183 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
```

(each repeat also printed two `Saved:` lines for the one path). Making the two nodes in the opposite order swapped which picture survived:

```
RESULT [5.3.0 Beta] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  2987 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

In Multi-Layer EXR mode the same arrangement left one file holding both nodes' outputs as separate parts — quoted under question 6.

**A file already on disk is replaced silently, and the scene's Overwrite setting does not reach the node.** One node, one output `over`, rendered three times with 1.1 seconds between, the picture changed for the second render and changed back, with `render.use_overwrite = False`, for the third:

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1792292632.191
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1792292633.317
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1792292634.439
```

**The scene's Placeholders setting does not reach the node either.** A two-frame animation with `render.use_placeholder = True`, the scene's own output in the same folder, the folder listed by script at the start (`render_pre`) and end (`render_post`) of each frame:

```
    FOLDER AT render_pre OF FRAME 1: [('scene_0001.png', 0)]
00:02.774  render           | Saved: '<W>/runs/e09/out/file_nameover0001.png'
00:02.775  render           | Saved: '<W>/runs/e09/out/scene_0001.png'
    FOLDER AT render_post OF FRAME 1: [('file_nameover0001.png', 3183), ('scene_0001.png', 3826)]
    FOLDER AT render_pre OF FRAME 2: [('file_nameover0001.png', 3183), ('scene_0001.png', 3826), ('scene_0002.png', 0)]
00:02.780  render           | Saved: '<W>/runs/e09/out/file_nameover0002.png'
00:02.781  render           | Saved: '<W>/runs/e09/out/scene_0002.png'
```

A 0-byte placeholder stood for the scene's own file at the start of each frame; none stood for the node's. Whether the panel flags two outputs with one name is entry 17.

**Observed by hand test, from Blender's own window (2026-10-18, entry 17).** **The Output Paths panel flags nothing, because Blender renames the second output before a clash can happen.** With two outputs both asked to be called `clash`, the Images list showed `clash` and `clash_001`, and the Output Paths panel, opened, listed two different finished paths, `/private/tmp/claud...ut/shot_clash.png` and `/private/tmp/claud...ot_clash_001.png`, each beside a small picture icon. Nothing was coloured red and no warning mark appeared. How the window was driven is under question 2.

---

## 11. Can a File Output node write a video file?

**Answer.** The sources do not settle it.

- **Documented.** The manual's File Output page lists only two Media Types for the node, Image and Multi-Layer EXR. The render Output panel, by contrast, lists a third, Video ("Encodes each frame into a video container").
- **Documented.** The node's format is the same kind of settings object as the render output's. That object's media-type field allows ‘VIDEO’, and its format list includes "FFmpeg Video".
- **Not in the sources.** Whether the node can be set to video, and what it writes if it is.

**Evidence.**

- File Output Media Type lists "Image:" and "Multi-Layer EXR:" only — manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- Render Output Media Type: "Video: Encodes each frame into a video container. Encoding options can be found in the Encoding panel." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11.
- "media_type — The type of media to save (default 'IMAGE')", values ‘IMAGE’, ‘MULTI_LAYER_IMAGE’, ‘VIDEO’. API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html, read 2026-10-11.
- "FFMPEG: FFmpeg Video." API, https://docs.blender.org/api/5.3/bpy_types_enum_items/image_type_all_items.html, read 2026-10-11.

**Not covered.** Whether video is offered on the node, accepted by a script, or written.

**Where I looked.**

- The File Output page and its picture.
- The render Output page and Rendering Animations. My whole-manual search for "file output" found no mention of the node on the Supported Video Formats page.
- The API pages for the node and the format settings.
- I searched the release notes for "video" together with "output" and found nothing on this node.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, the panel offers no video choice (block below). The original note from the sources follows. A script can set the node's media-type field (it is a property of the format settings, documented). Whether it accepts ‘VIDEO’ on this node is not stated.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say for the node. FFmpeg appears among formats "not available on all systems" for the command-line format option (see question 1).

**Observed by hand test (2026-10-18, entry 1).** **The node refuses the video media type.** The shared format settings publish three media types, but a File Output node will not take the third, and the refusal names the two it will:

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in (\'IMAGE\', \'MULTI_LAYER_IMAGE\')')
```

The video format is refused too, on Image (quoted under question 9). What a render then writes is in the next block; what the panel offers is entry 17.

**Observed by hand test (2026-10-18, entry 10).** **Asked for video, the node stays on pictures and writes one picture per frame.** One node, one output `clip`, folder `<W>/runs/e10/out/` (the work folder written `<W>`); the media type and then the format were set to video, and a three-frame animation rendered with `bpy.ops.render.render(animation=True)`:

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in (\'IMAGE\', \'MULTI_LAYER_IMAGE\')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in (\'AVIF\', \'JPEG\', \'OPEN_EXR\', \'PNG\', \'WEBP\', \'BMP\', \'CINEON\', \'DPX\', \'IRIS\', \'JPEG2000\', \'HDR\', \'TARGA\', \'TARGA_RAW\', \'TIFF\')')
NODE BEFORE RENDER: media_type='IMAGE' file_format='PNG'
00:00.498  render           | Saved: '<W>/runs/e10/out/file_nameclip0001.png'
00:00.502  render           | Saved: '<W>/runs/e10/out/file_nameclip0002.png'
00:00.507  render           | Saved: '<W>/runs/e10/out/file_nameclip0003.png'
```

(the scene's own `Saved:` line after each is cut). So setting video from outside is refused, not silently taken, and no video file is written.

**Observed by hand test, from Blender's own window (2026-10-18, entry 17).** **The panel offers no video choice.** The Media Type is drawn on this node as two buttons, **Image** and **Multi-Layer EXR**, with no third (described under question 2). Whether the File Format menu holds FFmpeg Video was not read: the script could not open that menu (under question 9).

---

## 12. What can a File Output node be fed, and does it have to come from the render?

**Answer.**

**What it can be fed.** Inputs are made by dragging any other node's output onto the node, or by adding them by hand. This is **documented**. Each input has a data type:

- The API gives that type as "Node Socket Data Type Items", default Float. That is a general list of 24 types (Float, Integer, Boolean, Vector, Color, Rotation, Matrix, String, Image, Geometry and others). This is **documented**.
- The sources do not say which of these the File Output node accepts.
- The compositor's own data types are Float, Vector and Color. A vector output can be 2 to 4 wide. Both **documented**.

**Does it have to come from the render?** The manual says the node can "Save intermediate results from anywhere in the node tree". The compositor can take pictures from sources other than the render, such as the Image node. From these I **infer** that the input need not come from the render. The sources do not say so outright.

Also documented, and related:

- The default compositor tree in 5.3 has no Render Layers node. Its Group Input "Image" socket "contains the rendered image".
- A Render Layers node renders a view layer "from a scene", and its Scene property chooses which scene.

**Evidence.**

- "Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel." Also: "Save intermediate results from anywhere in the node tree." Also: "Export multiple render passes or processed outputs directly to files." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- `socket_type` "(default 'FLOAT')", type "Node Socket Data Type Items"; `vector_socket_dimensions` "(in [2, 4], default 0)". API, https://docs.blender.org/api/5.3/bpy.types.NodeCompositorFileOutputItem.html; list at https://docs.blender.org/api/5.3/bpy_types_enum_items/node_socket_data_type_items.html, read 2026-10-11.
- "Three types of data exist" — Float, Vector, Color. Manual, https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html, read 2026-10-11.
- "By default, the Image socket contains the rendered image, equivalent to the Combined render pass." Manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html, read 2026-10-11.
- "The default compositor node tree now uses a simple **Group Input** connected to a **Group Output** with no **Render Layers** node." Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.

**Not covered.**

- Which data types the node accepts.
- What is written for an input with nothing connected.
- Whether a File Output node fed only from non-render sources writes during a render.

**Where I looked.** The File Output page; the compositor introduction; Compositor System; Render Layers (https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html). The API pages for the output item and the socket-type list. The compositor release notes.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The original note from the sources follows. A script can add an output of a given data type (`file_output_items.new(socket_type, name)`, documented). The sources do not say whether what can be fed differs outside the interface.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 1).** **Of the twenty-four socket kinds an output publishes, the node will make three.** An output was asked for of every kind, on one node:

```
    socket_type values=['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE', 'FONT', 'SCENE', 'TEXT', 'MASK', 'SOUND', 'INT_VECTOR']
OUTPUTS THE NODE ACCEPTED: [('k_FLOAT', 'FLOAT'), ('k_VECTOR', 'VECTOR'), ('k_RGBA', 'RGBA')]
```

Each of the other twenty-one was refused with the same words; the first and last of them:

```
ADDING AN OUTPUT OF KIND INT RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
ADDING AN OUTPUT OF KIND INT_VECTOR RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
```

(the nineteen lines between, for BOOLEAN, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK and SOUND, read the same and are cut). So Float, Vector and Colour are the only kinds a written output can have. What each writes is in the next block.

**Observed by hand test (2026-10-18, entry 11).** Still renders, folders under `<W>/runs/e11/` (the work folder written `<W>`), File Name `file_name`, PNG; for every file written, the colour type and depth were read from the file's own header and the first pixel read back. The node's format was 16-bit PNG in every case, having been switched from the multi-layer OpenEXR it starts on.

**A plain colour writes nothing; a picture does.** One node, output `constant` fed by an RGB colour node and output `picture` fed by the render through an Invert node; both links reported valid, no error was printed, and only the second was written:

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE; links valid: True True
00:00.478  render           | Saved: '<W>/runs/e11/a/file_namepicture.png'
RESULT [5.3.0 Beta] one output fed by a colour node, one by an image
    out/file_namepicture.png  2987 bytes
    file_namepicture.png: PNG colour type 6 (RGBA), 16 bits per channel, pixel 0 = (0.544, 0.544, 0.544, 1.0)
```

The same with a single value: a Float output fed by a Value node wrote nothing, while a Float output beside it fed the render picture was written:

```
00:00.459  render           | Saved: '<W>/runs/e11/d/file_namepicture_fed.png'
RESULT [5.3.0 Beta] two Float outputs, one fed by a Value node, one by the render picture
    out/file_namepicture_fed.png  2727 bytes
    file_namepicture_fed.png: PNG colour type 6 (RGBA), 16 bits per channel, pixel 0 = (0.004, 0.004, 0.004, 1.0)
```

**It does not have to come from the render.** A node fed only by an Image node holding an 8 × 8 picture made inside the file (every pixel set to 0.2, 0.6, 0.9), with nothing from the render reaching it, wrote its file during the render:

```
00:00.589  render           | Saved: '<W>/runs/e11/c/file_namefrom_image_node.png'
    file_namefrom_image_node.png: PNG colour type 6 (RGBA), 16 bits per channel, pixel 0 = (0.097, 0.308, 0.538, 1.0)
```

and the inverted picture in the first case came through an Invert node rather than straight from the render.

**All three output kinds write the render picture, and all three write a four-channel file.** One node with one output of each kind the node will make, all fed the render picture:

```
OUTPUTS THE NODE ACCEPTED: ['FLOAT', 'VECTOR', 'RGBA']
LINKS THE TREE ACTUALLY HOLDS INTO THE NODE: ['k_FLOAT', 'k_RGBA', 'k_VECTOR']
00:00.581  render           | Saved: '<W>/runs/e11/b/file_namek_VECTOR.png'
00:00.582  render           | Saved: '<W>/runs/e11/b/file_namek_RGBA.png'
00:00.583  render           | Saved: '<W>/runs/e11/b/file_namek_FLOAT.png'
RESULT [5.3.0 Beta] one written output of every kind the node offers, all fed the render picture
    out/file_namek_FLOAT.png  2727 bytes
    out/file_namek_RGBA.png  3183 bytes
    out/file_namek_VECTOR.png  3183 bytes
    file_namek_FLOAT.png: PNG colour type 6 (RGBA), 16 bits per channel, pixel 0 = (0.004, 0.004, 0.004, 1.0)
    file_namek_RGBA.png: PNG colour type 6 (RGBA), 16 bits per channel, pixel 0 = (0.043, 0.043, 0.043, 1.0)
    file_namek_VECTOR.png: PNG colour type 6 (RGBA), 16 bits per channel, pixel 0 = (0.043, 0.043, 0.043, 1.0)
```

The Float file is smaller and darker at its first pixel than the Colour and Vector files, which match each other; all three are four-channel 16-bit PNGs. (The scene's own `Saved:` line after each render is cut throughout.)

---

## 13. What causes a File Output node to write, and when during a render does it write each file?

**Answer.**

**What causes it to write.** Documented in parts:

- The node writes "during rendering or compositing".
- It can "Automatically save the final image after a render."
- For the render to use the compositor at all, the Compositing switch in Post Processing must be on.
- With the render Output panel's own saving turned off, "if compositing is enabled, active File Output Node nodes are used to save the result". The word "active" is not defined.
- Still renders "must be saved manually from the Image Editor or written using a File Output Node". From this I **infer** that the node also writes on a single-frame render, not only for animations.
- In 5.3 the scene's compositor is a stack of effects, each with a "Use effect during render" switch. That the switch exists is documented. That an effect switched off for render would not write its File Output nodes is **inferred**. Not stated.

**When during a render.** The sources say only these things:

- Each file gets the frame number, "to create a sequence". So writing is per frame (documented).
- Compositing follows rendering: Post Processing "Renders the output … and then applies the Composite node tree on all images". For multi-view, "all views are rendered and then composited" when working "from the user interface", and "The compositing of a view is completed before the remaining views start to be composited." From these I **infer** that a frame's File Output files are written after that frame has been rendered.
- **Not in the sources:**
  - the exact moment of writing;
  - the order among a node's outputs, or among several nodes;
  - the order relative to the render output file;
  - whether files are written outside a render, for example by the interactive or viewport compositor.

  The release notes say the interactive compositor "is executed after rendering if the viewer is visible", but not whether that writes files.

**A tension in the sources.** The compositor introduction says "Nodes that are not connected to a Group Output are ignored during evaluation." A File Output node has no output sockets. Taken literally, that sentence would mean the node never runs, which conflicts with the File Output page. The sources do not resolve this.

**Evidence.**

- "The File Output node writes one or more images to disk during rendering or compositing." Also: "Automatically save the final image after a render." Also: "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node." Also: "For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11.
- "If it is disabled, frames will only be written if a Compositor File Output node is configured." Manual, https://docs.blender.org/manual/en/5.3/render/output/animation.html, read 2026-10-11.
- "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output’s Post Processing properties." Also: "Nodes that are not connected to a Group Output are ignored during evaluation." Manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html, read 2026-10-11.
- "Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html, read 2026-10-11.
- "The compositing of a view is completed before the remaining views start to be composited." Also: "By default, when compositing and rendering from the user interface all views are rendered and then composited." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html, read 2026-10-11.
- `use_compositing`: "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)". API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- `enable_for_render`: "Use effect during render (default False)"; `enable_for_preview`: "Use effect during preview (default False)". API, https://docs.blender.org/api/5.3/bpy.types.SceneCompositorEffect.html, read 2026-10-11.
- "Rendering no longer writes to viewer images, however, the interactive compositor is executed after rendering if the viewer is visible." Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.

**Not covered.** As listed under "Not in the sources" above.

**Where I looked.**

- The File Output, render Output, Rendering Animations, Post Processing, compositor introduction, Compositor System and Stereoscopy usage pages.
- The app-handlers API page: `render_write` is "on writing a render frame (directly after the frame is written)", and `composite_pre` / `composite_post` are "on a compositing background job". Neither ties to File Output files.
- I searched the manual for "pipeline", "per frame", "is written", "are written" and "after each frame".
- The 5.3 release notes for "compositor", "output" and "write".

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Most hand tests under this question ran with Blender started from a terminal with no window, and wrote as quoted below. With Blender's own window open, one frame rendered as F12 renders it wrote too, with no frame number in the names, as a still render from a script did; and nothing was written after the render, on a compositor change, or by the viewport's own compositor. The original note from the sources follows.

- The multi-view sentence is stated only for "from the user interface". The sources do not say what order applies from the command line.
- The command-line render pages say nothing about compositing or File Output nodes. The options `-o`, `-F` and `-x` set "the render path", "the render format" and the extension; the sources do not say they affect the node.
- The `use_compositing` switch and the effects' `enable_for_render` are script-settable (documented).

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 12).** The work folder is written `<W>`. In every render below the scene's own picture was saved as well, and its lines are cut except where the order matters.

**It writes when Blender is started from a terminal with no window**, on one frame and on a whole animation. A file saved as `fo_scene.blend` (frames 1–3; node folder `//out/`, File Name `file_name`, output `beauty`; the scene's own output `//out/scene_`), rendered with `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b fo_scene.blend -f 1` and then with `-a` in place of `-f 1`:

```
--- blender -b fo_scene.blend -f 1
exit status: 0
00:00.483  render           | Saved: '<W>/runs/e12/out/file_namebeauty0001.png'
00:00.484  render           | Saved: '<W>/runs/e12/out/scene_0001.png'
--- blender -b fo_scene.blend -a
exit status: 0
00:00.478  render           | Saved: '<W>/runs/e12/out/file_namebeauty0001.png'
00:00.478  render           | Saved: '<W>/runs/e12/out/scene_0001.png'
00:00.483  render           | Saved: '<W>/runs/e12/out/file_namebeauty0002.png'
00:00.483  render           | Saved: '<W>/runs/e12/out/scene_0002.png'
00:00.487  render           | Saved: '<W>/runs/e12/out/file_namebeauty0003.png'
00:00.488  render           | Saved: '<W>/runs/e12/out/scene_0003.png'
```

**A still render writes too, and the frame number is added only on an animation render**, with `####` placing it either way. Two outputs, `plain` and `hash####here`, rendered from a script with `bpy.ops.render.render(animation=False)` — the scene's own picture not asked to be saved — and then as an animation over frame 1 alone:

```
00:00.483  render           | Saved: '<W>/runs/e12/inside/a/file_nameplain.png'
00:00.484  render           | Saved: '<W>/runs/e12/inside/a/file_namehash0001here.png'
RESULT [5.3.0 Beta] render.render(animation=False)
00:00.489  render           | Saved: '<W>/runs/e12/inside/b/file_namehash0001here.png'
00:00.490  render           | Saved: '<W>/runs/e12/inside/b/file_nameplain0001.png'
RESULT [5.3.0 Beta] render.render(animation=True), frame range 1 to 1
```

**Each file is written as its frame finishes, just before the scene's own file for that frame.** Four frames, the node's files and the scene's in one folder; Blender's lines, then each file's last-written time read back:

```
00:00.495  render           | Saved: '<W>/runs/e12/inside/c/file_namenode0001.png'
00:00.496  render           | Saved: '<W>/runs/e12/inside/c/scene_0001.png'
00:00.500  render           | Saved: '<W>/runs/e12/inside/c/file_namenode0002.png'
00:00.501  render           | Saved: '<W>/runs/e12/inside/c/scene_0002.png'
00:00.504  render           | Saved: '<W>/runs/e12/inside/c/file_namenode0003.png'
00:00.505  render           | Saved: '<W>/runs/e12/inside/c/scene_0003.png'
00:00.509  render           | Saved: '<W>/runs/e12/inside/c/file_namenode0004.png'
00:00.509  render           | Saved: '<W>/runs/e12/inside/c/scene_0004.png'
FILES IN THE ORDER THEY WERE LAST WRITTEN (seconds since the epoch):
    1792292758.199919  out/file_namenode0001.png
    1792292758.200571  out/scene_0001.png
    1792292758.204673  out/file_namenode0002.png
    1792292758.205416  out/scene_0002.png
    1792292758.208918  out/file_namenode0003.png
    1792292758.209647  out/scene_0003.png
    1792292758.213079  out/file_namenode0004.png
    1792292758.213740  out/scene_0004.png
```

Nothing waited for the end of the run. These frames took a few thousandths of a second each, so the order is what this shows, not any timing.

**Switching Compositing off stops it; having no Group Output node in the tree does not** — so the manual's sentence that nodes not connected to a Group Output are ignored does not hold for this node:

```
RESULT [5.3.0 Beta] the same tree with render.use_compositing = False
    (nothing written)
00:00.517  render           | Saved: '<W>/runs/e12/inside/e/file_namepicture.png'
RESULT [5.3.0 Beta] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  3183 bytes
```

**The compositor effect's *Use effect during render* switch decides it, and a File Output node in any effect of the stack that is on for render writes, whichever effect is the active one.** The scene's tree is held by an effect; with that effect's `enable_for_render` off, nothing was written. Then the File Output node was taken out of the first effect's tree and put in a second tree, held by a second effect made with `scene.compositor_effects.new('Second effect')`; the second tree had no Group Output node:

```
EFFECTS: [('Scene Effect', 'render=False', 'active=True', 'Comp')]
RESULT [5.3.0 Beta] the scene's compositor effect with enable_for_render = False
    (nothing written)
EFFECTS: [('Scene Effect', 'render=True', 'active=False', 'Comp'), ('Second effect', 'render=True', 'active=True', 'Second')]
00:00.525  render           | Saved: '<W>/runs/e12/inside/g/second_picture.png'
RESULT [5.3.0 Beta] the File Output node in the second of two compositor effects
EFFECTS: [('Scene Effect', 'render=True', 'active=False', 'Comp'), ('Second effect', 'render=False', 'active=True', 'Second')]
RESULT [5.3.0 Beta] the same, the second effect with enable_for_render = False
    (nothing written)
EFFECTS: [('Scene Effect', 'render=True', 'active=True', 'Comp'), ('Second effect', 'render=True', 'active=False', 'Second')]
00:00.532  render           | Saved: '<W>/runs/e12/inside/g/second_picture.png'
RESULT [5.3.0 Beta] the same, both effects on for render, the first effect made the active one
```

**What a stopped render leaves behind: whole files for every finished frame, as pairs, and nothing for the frame in flight.** A file of 60 frames at 960 × 540 and 512 samples (`fo_heavy.blend`, node folder `//out/`) was rendered with `-a` and stopped from the same command after a fixed pause of 12 seconds — once with an interrupt, the polite stop that Ctrl-C sends, and once with a hard kill that gives Blender no chance to tidy up. With the interrupt:

```
blender's exit code: 1
the last lines blender printed:
00:11.174  render           | Fra: 6 | Mem: 1M | Sample 0/512
00:11.195  render           | Fra: 6 | Remaining: 00:10.68 | Mem: 19M | Sample 1/512
00:13.174  render           | Fra: 6 | Remaining: 00:10.80 | Mem: 19M | Sample 80/512
Sent an internal break event. Press ^C again to kill Blender
00:13.176  render           | Fra: 6 | Remaining: 00:10.80 | Mem: 19M | Cancelled
Blender quit
what is on disk afterwards:
    file_namebeauty0001.png  346733 bytes
    file_namebeauty0002.png  346733 bytes
    file_namebeauty0003.png  346733 bytes
    file_namebeauty0004.png  346733 bytes
    file_namebeauty0005.png  346733 bytes
    scene_0001.png  295260 bytes
    scene_0002.png  295260 bytes
    scene_0003.png  295260 bytes
    scene_0004.png  295260 bytes
    scene_0005.png  295260 bytes
```

With the hard kill, `blender's exit code: 137`, the last line printed `00:11.172  render           | Fra: 6 | Remaining: 00:10.88 | Mem: 19M | Sample 1/512`, and the same ten files at the same sizes were left. Both times frame 6, the one being rendered, left nothing — no part-written file, no empty file — and the node's files and the scene's came in matching pairs. (A first try at this, with a lighter 60-frame file, finished all 60 frames in about four seconds, before the stop arrived, and is recorded in the hand-test list as a run that did not bite.)

What a render started from Blender's own window does, and whether anything writes with no render, are entry 18.

**Observed by hand test, from Blender's own window (2026-10-18, entry 18).** Blender 5.3 was started from the Mac's Terminal with its window open (driven as described under question 2) on a saved file whose compositor tree held Render Layers, an Invert node and the Group Output node; the script added a File Output node with folder `<W>/runs/w18/out/` (the work folder written `<W>`), File Name `shot_`, Image, PNG, output `one` fed by Render Layers and output `two` fed by the Invert node, and turned the large area into the compositor, so that the compositor stayed in view. It then started one render the way F12 does, `bpy.ops.render.render('INVOKE_DEFAULT')` from the window — the preference for showing renders was `RENDER DISPLAY: WINDOW`, and a second, render window opened — and listed the node's folder at each step.

**One frame rendered from the window writes**, with no frame number in the names, and without the render being saved by hand:

```
FOLDER before the render: (empty)
render.render('INVOKE_DEFAULT') RETURNED {'RUNNING_MODAL'} at 1792294141.787
00:06.662  render           | Saved: '<W>/runs/w18/out/shot_two.png'
00:06.663  render           | Saved: '<W>/runs/w18/out/shot_one.png'
render_complete handler: render finished at 1792294141.833
FOLDER 8 seconds after the render was started: ['shot_one.png  3253 bytes  last written 1792294141.833', 'shot_two.png  3057 bytes  last written 1792294141.832']
```

**Nothing was written again after the render with the compositor in view, nor when a compositor value was changed with no render, nor in about thirty seconds of the 3D viewport's own compositor with no render.** About six seconds after the render, with nothing touched; then eight seconds after the Invert node's Factor was changed to 0.5; then the large area was turned back into a 3D viewport, its shading set to Material Preview and its compositor to Always, and left about thirty seconds:

```
FOLDER about 6 seconds later, nothing touched: ['shot_one.png  3253 bytes  last written 1792294141.833', 'shot_two.png  3057 bytes  last written 1792294141.832']
INVERT FACTOR CHANGED TO 0.5 at 1792294155.769, no render started
FOLDER 8 seconds after the compositor change, no render: ['shot_one.png  3253 bytes  last written 1792294141.833', 'shot_two.png  3057 bytes  last written 1792294141.832']
3D VIEW SHADING: MATERIAL viewport compositor: ALWAYS at 1792294163.770
FOLDER about 30 seconds of viewport compositing, no render: ['shot_one.png  3253 bytes  last written 1792294141.833', 'shot_two.png  3057 bytes  last written 1792294141.832']
```

The two files kept the times they were written by the render throughout. So "during rendering or compositing" did not extend, in this test, to the compositor run after a render, to a compositor change, or to the viewport.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**Answer.** **Not in the sources.**

The settings themselves are documented:

- the node's `mute`;
- the scene's `use_compositing` (the Compositing switch);
- the scene's compositor effects, each with `enable_for_render`, `is_active` and its `node_group`.

No source says whether any of these can be keyframed or driven, or whether a render takes notice of a change between frames.

Related facts, none of which settles it:

- The manual says, in general, "Blender lets you animate almost any property". It does not name these settings.
- The API pages do not mark individual properties as animatable or not. The API has a general check, `Property.is_animatable` ("Property is animatable through RNA"), which can only be read in a running Blender.
- **Observed** in Cycles' own files: Cycles' Post Processing panel draws the Compositing switch with keyframe markers ("decorators") turned off. So in that panel no keyframe marker shows beside the switch either way. That proves nothing about whether the switch can be animated.
- The manual's picture of the Post Processing panel (`_images/render_output_properties_post-processing_panel.png`) shows no marker beside "Compositing" or "Sequencer", and a dot beside "Dither". Its layout (the two checkboxes side by side, no "Pipeline" heading) differs from the panel Cycles' file draws (a "Pipeline" column). I draw no conclusion from the picture.

**Evidence.**

- `Node.mute` "(default False)" — https://docs.blender.org/api/5.3/bpy.types.Node.html, read 2026-10-11.
- `use_compositing` description (see question 13) — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- `compositor_effects`: "Compositor effects for this scene (default None, readonly)"; `compositing_node_group`: "Returns and sets the node group of the active scene compositor effect. Creates a new effect if none exists". https://docs.blender.org/api/5.3/bpy.types.Scene.html, read 2026-10-11.
- `enable_for_render`: "Use effect during render (default False)"; `is_active`: "This effect is active (default False)". https://docs.blender.org/api/5.3/bpy.types.SceneCompositorEffect.html, read 2026-10-11.
- `is_animatable`: "Property is animatable through RNA (default False, readonly)". https://docs.blender.org/api/5.3/bpy.types.Property.html, read 2026-10-11.
- "Blender lets you animate almost any property". Manual, https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/introduction.html, read 2026-10-11.
- "Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated." Manual, https://docs.blender.org/manual/en/5.3/interface/controls/buttons/decorators.html, read 2026-10-11.
- Cycles' own files, `ui.py`, class `CYCLES_PT_post_processing`, lines 1190–1206. Exact lines: `layout.use_property_decorate = False` (line 1199), `col = layout.column(align=True, heading="Pipeline")`, `col.prop(rd, "use_compositing")` (line 1203), `col.prop(rd, "use_sequencer")` (line 1204). Cycles 5.3 files, read 2026-10-16.
- Picture `_images/render_output_properties_post-processing_panel.png` (Post Processing panel showing no keyframe markers beside Compositing or Sequencer). Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html, read 2026-10-11.

**Not covered.** Whether any of these settings can change between frames, and whether a running render would act on such a change.

**Where I looked.**

- I searched the whole manual for "animatable", "can be animated", "cannot be animated", "not animatable" and "almost any". The hits were the decorators page, the F-curve introduction, and unrelated pages.
- I searched the API for "animatable". It appears only on the property-definition pages (`bpy.props`, `Property` and the typed property pages), never on the settings named here.
- The Node, RenderSettings, Scene, SceneCompositorEffect and SceneCompositorEffects pages.
- The 5.3 compositor release notes.
- Cycles' `ui.py`, searched for "use_compositing".

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. The two saved files rendered again with the window open wrote the same frames as with no window. The original note from the sources follows. The sources do not say. All of these settings are script-settable (documented). Whether a script that changes them between frames during a command-line render has any effect is not stated.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 13).** One script, `Render ‣ Lock Interface` on throughout, a fresh tree for each part: Render Layers feeding the scene's Group Output node and a File Output node (folder `<W>/runs/e13/5.3.0/out/`, the work folder written `<W>`; File Name `file_name`; one output `img`). Frames 1 to 4, or 1 to 6 where a heading says so. A heading with nothing under it but `(no File Output file)` means the node wrote nothing; the scene's own picture was written on every frame of every run, and those lines are cut.

**Which switches Blender will animate at all.** Asked for each property's own flag, then asked to insert a keyframe and add a driver:

```
RESULT [5.3.0 Beta] is_animatable, as Blender reports it
    Node.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    NodeLink.mute -> no such property in this version
    NodeLink.is_muted -> is_animatable=True  is_readonly=False  type=BOOLEAN
    RenderSettings.use_compositing -> is_animatable=False  is_readonly=False  type=BOOLEAN
    Scene.use_nodes -> is_animatable=True  is_readonly=False  type=BOOLEAN
    Scene.node_tree -> no such property in this version
    Scene.compositing_node_group -> is_animatable=False  is_readonly=False  type=POINTER
    CompositorNodeOutputFile.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    CompositorNodeGroup.mute -> is_animatable=False  is_readonly=False  type=BOOLEAN
    SceneCompositorEffect.enable_for_render -> is_animatable=True  is_readonly=False  type=BOOLEAN
    SceneCompositorEffect.is_active -> is_animatable=False  is_readonly=False  type=BOOLEAN
    SceneCompositorEffect.node_group -> is_animatable=False  is_readonly=False  type=POINTER
    SceneCompositorEffect.enable_for_preview -> is_animatable=True  is_readonly=False  type=BOOLEAN
RESULT [5.3.0 Beta] what keyframe_insert and driver_add return
    node.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable
    link.keyframe_insert(data_path="is_muted", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() could not make path to "is_muted"
    scene.render.keyframe_insert(data_path="use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "use_compositing" not animatable
    scene.keyframe_insert(data_path="render.use_compositing", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "render.use_compositing" not animatable
    scene.keyframe_insert(data_path="use_nodes", frame=1) -> returned True
    scene.keyframe_insert(data_path="compositing_node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "compositing_node_group" not animatable
    node.driver_add("mute") -> raised TypeError: bpy_struct.driver_add(): property "mute" not animatable
    link.driver_add("is_muted") -> raised TypeError: bpy_struct.driver_add(): could not make path to "is_muted"
    scene.render.driver_add("use_compositing") -> raised TypeError: bpy_struct.driver_add(): property "use_compositing" not animatable
    scene.driver_add("use_nodes") -> returned bpy.data.scenes['Scene']...FCurve
    effect.keyframe_insert(data_path="enable_for_render", frame=1) -> returned True
    effect.keyframe_insert(data_path="is_active", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "is_active" not animatable
    effect.keyframe_insert(data_path="node_group", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "node_group" not animatable
    effect.driver_add("enable_for_render") -> returned bpy.data.scenes['Scene']...FCurve
RESULT [5.3.0 Beta] where the curves ended up
    fcurves on the compositor node tree: 0 []; drivers: 0 []
    fcurves on the scene: 2 ['use_nodes', 'compositor_effects["Scene Effect"].enable_for_render']; drivers: 2 ['use_nodes', 'compositor_effects["Scene Effect"].enable_for_render']
```

So the node's mute, the Compositing switch and the scene's compositing tree refuse a keyframe and a driver; a link's mute reports itself animatable but no key or driver could be put on it; `scene.use_nodes` takes both; and **the compositor effect's *Use effect during render* switch takes both**.

**Each switch set for a whole render.** The node muted, Compositing off, the link feeding the node muted, and the effect's render switch off each stopped the node writing; `scene.use_nodes` would not even take the value False, and the node wrote:

```
RESULT [5.3.0 Beta] (i) nothing switched off
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
RESULT [5.3.0 Beta] (ii) the node muted for the whole render
    (no File Output file)
RESULT [5.3.0 Beta] (iii) Compositing switched off for the whole render
    (no File Output file)
    after setting scene.use_nodes = False it reads back as True
RESULT [5.3.0 Beta] (iv) scene.use_nodes switched off for the whole render
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
RESULT [5.3.0 Beta] (v) the link feeding the node muted for the whole render
    (no File Output file)
RESULT [5.3.0 Beta] (vi) the compositor effect's enable_for_render off for the whole render
    (no File Output file)
```

Every touch of `scene.use_nodes` printed a warning of this form: `<W>/scripts/e13_switches.py:109: DeprecationWarning: 'Scene.use_nodes' is expected to be removed in Blender 6.0`; setting the scene's compositing tree printed the same for `'Scene.compositing_node_group'`.

**A keyframe on the effect's render switch changes the writing from one frame to the next, on the frame it is keyed.** Keyed on at frame 1 and off at frame 3, keys set to Constant, frames 1 to 4 rendered as one animation; then the value read back at each frame. A keyframe on `scene.use_nodes` the same way does nothing, because the second key was recorded as 1:

```
        compositor_effects["Scene Effect"].enable_for_render [0]  frame 1 = 1, frame 3 = 0
RESULT [5.3.0 Beta] the effect's enable_for_render keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png, out/file_nameimg0002.png
RESULT [5.3.0 Beta] what enable_for_render read at each frame
    frame 1 -> enable_for_render = True
    frame 2 -> enable_for_render = True
    frame 3 -> enable_for_render = False
    frame 4 -> enable_for_render = False
        use_nodes [0]  frame 1 = 1, frame 3 = 1
RESULT [5.3.0 Beta] scene.use_nodes keyframed on at frame 1 and off at frame 3, frames 1 to 4 rendered as one animation
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
```

**A per-frame script handler changes it too, one frame late — the Compositing switch included.** A `frame_change_pre` handler switched each thing off from frame 3:

```
    handler ran: frame 3, node.mute set to True
    handler ran: frame 4, node.mute set to True
RESULT [5.3.0 Beta] (A) frame_change_pre sets node.mute on from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
    handler ran: frame 3, render.use_compositing set to False
    handler ran: frame 4, render.use_compositing set to False
RESULT [5.3.0 Beta] (B) frame_change_pre sets render.use_compositing off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
    handler ran: frame 3, scene.use_nodes set to False, reads back True
    handler ran: frame 4, scene.use_nodes set to False, reads back True
RESULT [5.3.0 Beta] (C) frame_change_pre sets scene.use_nodes off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png
    handler ran: frame 3, link.is_muted set to True
    handler ran: frame 4, link.is_muted set to True
RESULT [5.3.0 Beta] (D) frame_change_pre sets the link's mute on from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
    handler ran: frame 3, effect.enable_for_render set to False
    handler ran: frame 4, effect.enable_for_render set to False
RESULT [5.3.0 Beta] (E) frame_change_pre sets the effect's enable_for_render off from frame 3
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png
```

The same five with a `render_pre` handler instead gave the same counts: three files for (A), (B), (D) and (E), four for (C). Stretched to frames 1 to 6, with each thing switched off on frames 2 and 5 only:

```
RESULT [5.3.0 Beta] (G) handler mutes the node on frames 2 and 5 only, frames 1 to 6 rendered
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.3.0 Beta] (H) handler switches scene.use_nodes off on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0003.png, out/file_nameimg0004.png, out/file_nameimg0005.png, out/file_nameimg0006.png
RESULT [5.3.0 Beta] (J) handler mutes the link on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.3.0 Beta] (K) handler switches the effect's enable_for_render off on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
RESULT [5.3.0 Beta] (L) handler switches render.use_compositing off on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png
```

(each preceded by `handler ran: frame 2, …` and `handler ran: frame 5, …`). A change made by a handler on frame 2 left frame 2's file written and frame 3's missing: it lands on the frame after the one it was made on. A keyframe, above, landed on its own frame.

**A driver on the effect's render switch, from a saved file, with and without automatic script running.** A driver with the expression `frame < 3` was added to the effect's render switch and the file saved; it read False at frame 3 (`frame 3 -> the effect's enable_for_render = False`). The same driver on `scene.use_nodes` read True at frame 3. Each saved file was rendered from the Mac's Terminal, with `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender"` and the arguments shown; the node's own `Saved:` lines:

```
--- command: blender -b driven-enable_for_render.blend -a
00:00.485  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
00:00.490  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0002.png'
--- command: blender -b -y driven-enable_for_render.blend -a
00:00.483  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
00:00.488  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0002.png'
--- command: blender -b driven-use_nodes.blend -a
00:00.458  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
00:00.464  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0002.png'
00:00.468  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0003.png'
00:00.472  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0004.png'
--- command: blender -b -y driven-use_nodes.blend -a
00:00.485  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
00:00.490  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0002.png'
00:00.494  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0003.png'
00:00.498  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0004.png'
```

So the driver on the effect's render switch stopped the writing from frame 3 with automatic script running left off, as well as with `-y`.

**The node inside a group.** Frames 1 to 6, the File Output node inside a group fed from Render Layers: (8i) not muted, six files; (8ii) the group node muted for the whole render, `(no File Output file)`; (8iii) a handler muting the group node on frames 2 and 5, `out/file_nameimg0001.png, out/file_nameimg0002.png, out/file_nameimg0004.png, out/file_nameimg0005.png`. The group node's mute refused a key: `group.keyframe_insert(data_path="mute", frame=1) -> raised TypeError: bpy_struct.keyframe_insert() property "mute" not animatable`.

**Swapping which tree the scene composites with.** Frames 1 to 6. The scene set to the tree holding the node: six files. Set to a tree with no File Output node: `(no File Output file)`. Set to no tree at all — which left the effect in place with no tree, `after setting compositing_node_group = None it reads None; effects [('Scene Effect', None)]` — `(no File Output file)`. A handler swapping to the writer-less tree on frames 2 and 5 only, with a `render_write` handler printing each finished frame:

```
    handler ran: frame 1, scene.compositing_node_group set to Comp
    render_write: frame 1 finished
    handler ran: frame 2, scene.compositing_node_group set to NoWriter
    render_write: frame 2 finished
    render_write: frame 3 finished
    render_write: frame 4 finished
    render_write: frame 5 finished
    render_write: frame 6 finished
    handler ran: frame 1, scene.compositing_node_group set to Comp
RESULT [5.3.0 Beta] (9iv) handler swaps to the writer-less tree on frames 2 and 5 only
    out/file_nameimg0001.png, out/file_nameimg0002.png
```

The handler printed for frames 1 and 2 and not again until the render ended, while `render_write` printed for all six frames; two files were written. No error or traceback appeared.

**The tree's own output node.** Frames 1 to 6, the Group Output node muted throughout, muted by a handler on frames 2 and 5, and removed: six files every time.

**Saved files rendered from the command line.** A file whose effect's render switch was keyed `frame 1 = 1, frame 2 = 0, frame 3 = 1, frame 5 = 0, frame 6 = 1` (Constant), and a plain file rendered with a handler script named on the command line, `-b plain.blend --python <W>/scripts/e13_flip.py -a`, which mutes the node on frames 2 and 5:

```
--- command: blender -b keyed.blend -a
00:00.476  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
00:00.484  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0003.png'
00:00.488  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0004.png'
00:00.494  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0006.png'
--- command: blender -b plain.blend --python <W>/scripts/e13_flip.py -a
    handler registered
00:00.465  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
    handler ran: frame 2, node.mute set to True
00:00.470  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0002.png'
00:00.478  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0004.png'
    handler ran: frame 5, node.mute set to True
00:00.482  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0005.png'
```

The keyed file wrote every frame except the two keyed off, 2 and 5; the handler file wrote every frame except 3 and 6, the ones after the two it muted. Whether a render with Blender's window open agrees, and whether the window offers a keyframe on these switches, are entry 19.

**Observed by hand test, from Blender's own window (2026-10-18, entry 19).** Blender 5.3 was started with its window open and driven by script as described under question 2, and photographed with Blender's own screenshot command; the pictures are described here and were not kept.

**Whether a keyframe mark sits beside the switches.** The Output tab's Post Processing panel starts closed, so the script took the two panel classes out and put them back without their "start closed" option — `CYCLES_PT_post_processing put back with bl_options = set()` and `RENDER_PT_post_processing put back with bl_options = set()` — and scrolled the tab to its bottom. With the engine set to Cycles, the panel shows a **Pipeline** heading with **Compositing** and **Sequencer** checkboxes, both ticked, and a **Dither** slider reading 1.00, and **no keyframe mark beside any of the three**. With the engine set to EEVEE (`BLENDER_EEVEE`), the same three controls, **a small round mark at the right-hand edge of the Dither row, and none beside Compositing or Sequencer**. The compositor's sidebar Node panel, with a File Output node selected, lists Name, Label, Color, Show Options, Mute and Propagate, with no mark beside any row. **The compositor effects are drawn on a Properties tab of their own**, published as `COMPOSITOR` and showing the tooltip `Compositor & Effects` over `Procedural scene post-processing.` when the mouse rests on its icon; it holds an **Add Scene Effect** button, then the effect `Scene Effect` as a row with two toggle icons — a screen and a camera, both lit — a menu arrow and an X, and under it the effect's tree, `Comp`. **No keyframe mark is drawn beside the two toggles.**

**A render started with the window open writes the same frames as one started with no window.** The two saved files from the command-line part of entry 13 were rendered again with the window open and `Render ‣ Lock Interface` on, by `bpy.ops.render.render(animation=True)` from inside the window:

```
RENDERING <W>/runs/e13/5.3.0/keyed.blend frames 1 to 6 with the window open
00:05.133  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
00:05.143  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0003.png'
00:05.148  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0004.png'
00:05.156  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0006.png'
render.render(animation=True) RETURNED {'FINISHED'}
```

```
    handler registered
RENDERING <W>/runs/e13/5.3.0/plain.blend frames 1 to 6 with the window open
00:05.125  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0001.png'
    handler ran: frame 2, node.mute set to True
00:05.131  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0002.png'
00:05.140  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0004.png'
    handler ran: frame 5, node.mute set to True
00:05.144  render           | Saved: '<W>/runs/e13/5.3.0/out/file_nameimg0005.png'
render.render(animation=True) RETURNED {'FINISHED'}
```

These are the frames the same two files wrote with no window.

---

## 15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?

**Answer.** **Not in the sources.** No source describes a File Output write failing, or what happens to:

- the unfinished file;
- the other files for that frame;
- the render;
- the messages reported.

The nearest statements concern a different situation, a render with no output method at all: "If no valid output method is available, rendering operations will report errors." For template errors in the render output path, "rendering an animation will fail with an error message". Neither covers a single file failing while others succeed.

**Evidence.**

- "If no valid output method is available, rendering operations will report errors." Manual, https://docs.blender.org/manual/en/5.3/render/output/animation.html, read 2026-10-11.
- "if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors." Manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html, read 2026-10-11.

**Not covered.** All four parts of the question.

**Where I looked.**

- I searched the whole manual, the whole API documentation and all 5.3 release notes for: "cannot write", "can't write", "could not write", "unable to write", "failed to write", "fail to write", "write error", "error writing", "cannot save", "could not save", "unable to save", "failed to save", "disk full", "no space", "permission denied", "write permission", "read-only", "not writable". No hit concerns writing render or File Output files.
- I searched the release notes, and the render-output, compositing, media and command-line manual pages, for "fail", "error", "abort", "cancel", "incomplete", "partial" and "corrupt". The only relevant hits are the two quoted above.
- The app-handlers page has `render_cancel` and `composite_cancel` but says nothing about write failures.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question but one ran with no window: there, a failed write printed an error line naming the file, the render carried on, and the exit status was 0. With Blender's own window open (entry 20), the same kind of failure printed the `OpenImageIO write failed` line but no `Render error … cannot save` line, the Info editor and the reports list stayed empty, and a broken template was shown in red on the node. The original note from the sources follows. The sources do not say.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The reason printed in brackets for a refused write read `(Undefined error: 0)` on that Mac; what another system prints was not tried here. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 14).** **One file that cannot be written is left out, every other file for the frame is written, the render carries on to the end, a line naming the file is printed, and the exit status is 0.** Each case was a saved file `t.blend` in its own folder under `<W>/runs/e14/` (the work folder written `<W>`), built by script: one File Output node with File Name `shot` and outputs `aaa`, `bbb` and `ccc`, all fed the render picture, PNG, 64 × 64, Cycles on the processor at one sample; the scene's own output in a separate `scene` folder, so the two can be told apart. Each was rendered from the Mac's Terminal with `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b t.blend -f 1` (or `-a` where said), followed by `echo "exit status: $?"`; afterwards every file and folder left was listed with its size.

**The control**, nothing blocked: `exit status: 0`, and

```
00:00.487  render           | Saved: '<W>/runs/e14/e2-5.3/out/shotbbb0001.png'
00:00.488  render           | Saved: '<W>/runs/e14/e2-5.3/out/shotccc0001.png'
00:00.489  render           | Saved: '<W>/runs/e14/e2-5.3/out/shotaaa0001.png'
00:00.490  render           | Saved: '<W>/runs/e14/e2-5.3/scene/f0001.png'
```

— three files of 3,260 bytes and the scene's file of 3,903. The node names its files from the File Name glued to the output's name, then the frame number.

**One output of three blocked by a folder standing where its file goes** (a folder made beforehand at `out/shotbbb0001.png`): `exit status: 0`, and

```
00:00.459  render           | Saved: '<W>/runs/e14/e3-5.3/out/shotaaa0001.png'
00:00.459  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e14/e3-5.3/out/shotbbb0001.png"
00:00.459  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e14/e3-5.3/out/shotbbb0001.png'
00:00.460  render           | Saved: '<W>/runs/e14/e3-5.3/out/shotccc0001.png'
00:00.461  render           | Saved: '<W>/runs/e14/e3-5.3/scene/f0001.png'
```

`shotaaa0001.png` and `shotccc0001.png` were left at 3,260 bytes, the same as the control; the blocking folder was still an empty folder; the scene's file was written.

**The same block across a five-frame animation** (`-a`): `exit status: 0`, and a count taken from the log read `Saved: lines 19; ERROR lines 2; last frame reported: Fra: 5` — the two `ERROR` lines being the same two as above, for frame 1, once. On disk: `shotaaa0001.png` to `0005`, `shotccc0001.png` to `0005`, `shotbbb0002.png` to `0005` (each 3,260 bytes), the blocking folder `shotbbb0001.png`, and the scene's five files. So only frame 1's `bbb` was lost and the render went on writing `bbb` for frames 2 to 5.

**A whole node aimed at a folder that refuses writing, with a second node aimed at a writable one.** The read-only folder was a small disk image attached read-only at `<W>/runs/e14/e6a-5.3/ro-out` (`touch` into it answered `touch: <W>/runs/e14/e6a-5.3/ro-out/canary: Read-only file system`); the second node, outputs `ddd` and `eee`, wrote to `good`. `exit status: 0`, and

```
00:00.459  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e14/e6a-5.3/ro-out/shotccc0001.png"
00:00.459  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e14/e6a-5.3/ro-out/shotccc0001.png'
00:00.486  render           | Saved: '<W>/runs/e14/e6a-5.3/good/shotddd0001.png'
00:00.487  render           | Saved: '<W>/runs/e14/e6a-5.3/good/shoteee0001.png'
00:00.488  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e14/e6a-5.3/ro-out/shotaaa0001.png"
00:00.488  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e14/e6a-5.3/ro-out/shotaaa0001.png'
00:00.488  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e14/e6a-5.3/ro-out/shotbbb0001.png"
00:00.488  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e14/e6a-5.3/ro-out/shotbbb0001.png'
00:00.489  render           | Saved: '<W>/runs/e14/e6a-5.3/scene/f0001.png'
```

The read-only folder was empty afterwards; the second node's two files were written at 3,261 bytes; the scene's file was written. **One node failing does not stop another.**

**One output of three inside a read-only subfolder.** Outputs `aaa`, `sub/bbb`, `ccc`, with a second read-only disk image attached at `out/shotsub` (the folder the name `sub/bbb` writes into under File Name `shot`); `touch` into it answered `Read-only file system`, while the node's own folder took a new file. `exit status: 0`, and

```
00:00.442  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e14/e6b-5.3/out/shotsub/bbb0001.png"
00:00.442  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e14/e6b-5.3/out/shotsub/bbb0001.png'
00:00.470  render           | Saved: '<W>/runs/e14/e6b-5.3/out/shotccc0001.png'
00:00.470  render           | Saved: '<W>/runs/e14/e6b-5.3/out/shotaaa0001.png'
00:00.471  render           | Saved: '<W>/runs/e14/e6b-5.3/scene/f0001.png'
```

with `shotsub` empty afterwards. (The same outputs with no `sub` folder beforehand, nothing blocked: Blender made the folder and wrote `Saved: '<W>/runs/e14/e5-5.3/out/shotsub/bbb0001.png'` beside the other two.)

**A second node aimed inside a path where a plain file stands** (a 0-byte file `notadir`, the node's folder `notadir/inside/`): `exit status: 0`, and

```
00:00.493  render           | Saved: '<W>/runs/e14/e8-5.3/out/shotbbb0001.png'
00:00.494  render           | Saved: '<W>/runs/e14/e8-5.3/out/shotccc0001.png'
00:00.494  image            | ERROR Couldn't create directory for file <W>/runs/e14/e8-5.3/notadir/inside/shotddd0001.png: Not a directory
00:00.494  reports          | ERROR Render error (Not a directory) cannot save: '<W>/runs/e14/e8-5.3/notadir/inside/shotddd0001.png'
00:00.495  image            | ERROR Couldn't create directory for file <W>/runs/e14/e8-5.3/notadir/inside/shoteee0001.png: Not a directory
00:00.495  reports          | ERROR Render error (Not a directory) cannot save: '<W>/runs/e14/e8-5.3/notadir/inside/shoteee0001.png'
00:00.495  render           | Saved: '<W>/runs/e14/e8-5.3/out/shotaaa0001.png'
00:00.496  render           | Saved: '<W>/runs/e14/e8-5.3/scene/f0001.png'
```

`notadir` was still a 0-byte plain file afterwards.

**Multi-Layer EXR.** In this mode the node writes one file per frame, `shot0001.exr`, so there are no other files from that node for the frame. With a folder standing at that name: `exit status: 0`, nothing left for the node, the scene's file written:

```
00:00.426  image.openexr    | ERROR IMB_exr_write_end: Is a directory.
00:00.426  reports          | ERROR Error writing render result, Is a directory (see console)
00:00.426  reports          | ERROR Render error (Is a directory) cannot save: '<W>/runs/e14/e9-5.3/out/shot0001.exr'
00:00.487  render           | Saved: '<W>/runs/e14/e9-5.3/scene/f0001.png'
```

**Storage running out part-way through the write.** Three outputs as OpenEXR, 32-bit, no compression, 2000 × 2000, aimed at a 12 MB disk image filled to leave 1500 KB free, the scene's own output on the ordinary disk. The image was made with `hdiutil create -size 12m -fs HFS+ -volname fon53hfs`, attached inside the work folder, and `diskutil info` on it read `File System Personality:   HFS+` and `Type (Bundle):             hfs`; it was filled with `dd if=/dev/zero of=<mount>/filler bs=1024 count=<available minus 1500>`, after which `df -k` read 1500 available. `exit status: 0`, and

```
00:00.570  render           | Fra: 1 | Compositing
00:00.626  image.openexr    | ERROR imb_exr_write_channels: Failed to write pixel data to image file "<W>/runs/e14/e7/mnt-hfs/out/shotbbb0001.exr". No space left on device.
00:00.626  render           | Saved: '<W>/runs/e14/e7/mnt-hfs/out/shotbbb0001.exr'
00:00.627  image.openexr    | ERROR IMB_exr_write_end: No space left on device.
00:00.627  reports          | ERROR Error writing render result, No space left on device (see console)
00:00.627  reports          | ERROR Render error (No space left on device) cannot save: '<W>/runs/e14/e7/mnt-hfs/out/shotccc0001.exr'
00:00.627  image.openexr    | ERROR IMB_exr_write_end: No space left on device.
00:00.627  reports          | ERROR Error writing render result, No space left on device (see console)
00:00.627  reports          | ERROR Render error (No space left on device) cannot save: '<W>/runs/e14/e7/mnt-hfs/out/shotaaa0001.exr'
00:00.920  render           | Saved: '<W>/runs/e14/e7/hfs-case/scene/f0001.png'
```

On the volume afterwards, one part-written file and nothing else, and the volume full:

```
-rw-r--r--@ 1 artist     staff  1536000 Oct 18 06:11 shotbbb0001.exr
/dev/disk4s1       12248 12248         0   100%       5 4294967274    0%   <W>/runs/e14/e7/mnt-hfs
```

The same on a 12 MB **APFS** image (`File System Personality:   APFS`, `Type (Bundle):             apfs`), which read 1492 KB available after filling: `exit status: 0`, and

```
00:00.553  render           | Fra: 1 | Compositing
00:00.614  image.openexr    | ERROR imb_exr_write_channels: Failed to write pixel data to image file "<W>/runs/e14/e7/mnt-apfs/out/shotaaa0001.exr". No space left on device.
00:00.614  render           | Saved: '<W>/runs/e14/e7/mnt-apfs/out/shotaaa0001.exr'
00:00.775  image.openexr    | ERROR imb_exr_write_channels: Failed to write pixel data to image file "<W>/runs/e14/e7/mnt-apfs/out/shotbbb0001.exr". No space left on device.
00:00.776  render           | Saved: '<W>/runs/e14/e7/mnt-apfs/out/shotbbb0001.exr'
00:00.776  image.openexr    | ERROR IMB_exr_write_end: No space left on device.
00:00.776  reports          | ERROR Error writing render result, No space left on device (see console)
00:00.776  reports          | ERROR Render error (No space left on device) cannot save: '<W>/runs/e14/e7/mnt-apfs/out/shotccc0001.exr'
00:01.029  render           | Saved: '<W>/runs/e14/e7/apfs-case/scene/f0001.png'
```

leaving two part-written files, and `df -k` reading space left although the writes had failed for want of it:

```
-rw-r--r--@ 1 artist     staff  442728 Oct 18 06:12 shotaaa0001.exr
-rw-r--r--  1 artist     staff   69992 Oct 18 06:12 shotbbb0001.exr
/dev/disk6s1       12248 10916       984    92%       4  9840    0%   <W>/runs/e14/e7/mnt-apfs
```

On both file systems: **Blender printed a `Saved:` line for each part-written file**; the outputs that could not be started left no file at all, not even an empty one; the scene's own file on the ordinary disk was written in full (2,306,861 bytes); the render finished; the exit status was 0. The same scene on the ordinary disk wrote three complete files — 64,032,747 bytes each from a blend file whose path was 96 characters long, and 64,032,767 bytes each from one whose path was 116 characters long — so a complete file here is about 64 MB, against the 1.5 MB and smaller part-written ones, and its exact size moves with the length of the blend file's path.

**A broken path template.** With a valid `{blend_name}_` as the File Name, the files came out `t_aaa0001.png`, `t_bbb0001.png` and `t_ccc0001.png` from `t.blend`; with `{blend_name}_aaa` as one output's name under File Name `shot`, `shott_aaa0001.png` — so templates are live in both places. With the unclosed `{blend_name_bbb` as one output's name, and a second node (outputs `ddd`, `eee`, folder `good`) beside it, over five frames: `exit status: 0`, the count from the log read `Saved: lines 25; ERROR lines 5; last frame reported: Fra: 5`, `aaa` and `ccc` and the second node's two outputs were written for all five frames, nothing was written for `bbb`, and this line came once per frame, five in all:

```
00:00.423  reports          | ERROR Invalid path template in File Output node. Skipping writing file.
```

With the unclosed `{blend_name_` as the node's File Name instead — which every one of its outputs goes through — the count read `Saved: lines 15; ERROR lines 15; last frame reported: Fra: 5`: the same line three times per frame, fifteen in all; none of that node's files was written; the second node wrote both its outputs on all five frames, and the scene's five files were written. `exit status: 0`. The render was neither refused nor stopped.

**What a script driving the render is handed.** On a scene whose `bbb` file was blocked by a folder — its name read first from an unblocked render through the same route, since `bpy.ops.render.render()` names the files with no frame number — `bpy.ops.render.render(animation=False, write_still=True)` returned `{'FINISHED'}` and raised nothing:

```
BLOCKED shotbbb.png WITH A FOLDER OF THE SAME NAME
VS_RENDER_BEGIN
00:00.509  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/e14/e11b-5.3.0/out/shotbbb.png"
00:00.509  reports          | ERROR Render error (Undefined error: 0) cannot save: '<W>/runs/e14/e11b-5.3.0/out/shotbbb.png'
00:00.510  render           | Saved: '<W>/runs/e14/e11b-5.3.0/out/shotccc.png'
00:00.511  render           | Saved: '<W>/runs/e14/e11b-5.3.0/out/shotaaa.png'
00:00.511  render           | Saved: '<W>/runs/e14/e11b-5.3.0/scene/f.png'
VS_RENDER_RETURN {'FINISHED'}
VS_RENDER_END
REPORTS LIST RIGHT AFTER THE BLOCKED RENDER: []
```

**What the node and the handler list carry.** The node's full property list:

```
THE NODE'S PROPERTIES: ['rna_type', 'type', 'location', 'location_absolute', 'width', 'height', 'dimensions', 'name', 'label', 'inputs', 'outputs', 'panel_states', 'internal_links', 'parent', 'warning_propagation', 'use_custom_color', 'color', 'color_tag', 'select', 'show_options', 'show_preview', 'hide', 'mute', 'show_texture', 'bl_idname', 'bl_label', 'bl_description', 'bl_icon', 'bl_static_type', 'bl_width_default', 'bl_width_min', 'bl_width_max', 'bl_height_default', 'bl_height_min', 'bl_height_max', 'file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render', 'use_file_extension']
THE HANDLER LIST: ['animation_playback_post', 'animation_playback_pre', 'annotation_post', 'annotation_pre', 'blend_import_post', 'blend_import_pre', 'composite_cancel', 'composite_post', 'composite_pre', 'count', 'depsgraph_update_post', 'depsgraph_update_pre', 'exit_pre', 'frame_change_post', 'frame_change_pre', 'index', 'load_factory_preferences_post', 'load_factory_startup_post', 'load_post', 'load_post_fail', 'load_pre', 'n_fields', 'n_sequence_fields', 'n_unnamed_fields', 'object_bake_cancel', 'object_bake_complete', 'object_bake_pre', 'redo_post', 'redo_pre', 'render_cancel', 'render_complete', 'render_init', 'render_post', 'render_pre', 'render_stats', 'render_write', 'save_post', 'save_post_fail', 'save_pre', 'translation_update_post', 'undo_post', 'undo_pre', 'version_update', 'xr_session_start_pre']
```

There is no property reporting whether a write succeeded or naming the last path written, and there is a failure hook for loading and for saving a blend file but none for writing a render frame. **The reports list exists but stays empty with no window, and a control shows its emptiness carries no information** — it was empty after the blocked render (above), after a successful save, and after an open that certainly failed:

```
PROBE_WM_HAS_REPORTS True
VS0_AT_START []
VS0_AFTER_A_SUCCESSFUL_SAVE []
VS0_OPEN_RAISED RuntimeError Error: Cannot read file "/definitely/no/such/file.blend": No such file or directory
VS0_AFTER_A_FAILED_OPEN []
```

**The exit status does report a blend file Blender could not open.** Started on a blend file that does not exist, and on an empty 0-byte file named `empty.blend`:

```
--- blender -b <W>/runs/e14/e13b-5.3/missing.blend -f 1
exit status: 1
00:00.325  reports          | ERROR Cannot read file "<W>/runs/e14/e13b-5.3/missing.blend": No such file or directory
--- blender -b <W>/runs/e14/e13b-5.3/empty.blend -f 1
exit status: 1
00:00.302  reports          | ERROR File format is not supported in file "<W>/runs/e14/e13b-5.3/empty.blend"
```

So a script watching only the exit status sees the difference between a job that never started and one that ran, and no difference between every file written and one file not written. The same with a blend file Blender is not allowed to read was not tried: making one needs a change of file permissions that the tool this sitting ran in refused. A network folder taken away part-way, a file locked by another program, and Windows were not tried — see *Cannot be run* in the hand-test list. What Blender's window shows when a write fails is entry 20.

**Observed by hand test, from Blender's own window (2026-10-18, entry 20).** Blender 5.3 was started with its window open and driven by script as described under question 2. The scene's compositor tree was given two File Output nodes, both aimed at `<W>/runs/w20b/out/` (the work folder written `<W>`): one named `Broken template` with the unclosed `{blend_name_` as its File Name and one output `aaa`, and one named `Blocked output` with File Name `shot_` and outputs `bbb` and `ccc`, where a folder had been made beforehand at `shot_bbb.png`. The broken node was selected, the sidebar opened on its Node tab, the window photographed; one render was started the way F12 does, `bpy.ops.render.render('INVOKE_DEFAULT')`; eight seconds later the window was photographed again, then the large area turned into an Info editor and photographed. The pictures are described here and were not kept.

**The broken template is marked in red before any render**: on the node's face the File Name field reading `{blend_name_` has a red background, and in the sidebar, once drawn, the same field is red. **Neither node carries any warning mark on its header, before or after the render.** What Blender printed to the terminal during the render:

```
render.render('INVOKE_DEFAULT') RETURNED {'RUNNING_MODAL'}
00:10.121  reports          | ERROR Invalid path template in File Output node. Skipping writing file.
00:10.153  image.write      | ERROR OpenImageIO write failed: Could not open file "<W>/runs/w20b/out/shot_bbb.png"
00:10.154  render           | Saved: '<W>/runs/w20b/out/shot_ccc.png'
FOLDER AFTER: ['shot_bbb.png', 'shot_ccc.png']
```

— **with no `Render error … cannot save` line for the blocked file**, which every run with no window printed. In the picture taken eight seconds after the render started, the status bar at the foot of the window showed `Rendering...`, a progress bar reading `0%` and a cancel button. **The Info editor was empty**, and the reports list read from the script was empty too: `REPORTS LIST WITH THE WINDOW OPEN: []`. So in this test nothing in the window named the file that could not be written; the broken template was marked on the node itself. (A first run of the same script photographed the sidebar before it had redrawn on its Node tab; its terminal lines were the same three, in the order invalid template, `Saved:` for `shot_ccc.png`, write failed for `shot_bbb.png`, and its Info editor was empty as well.)

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**Answer.** **Not in the sources.** No source says whether a brought-in scene's File Output nodes write, are skipped, or behave differently.

Related statements, none of which settles it:

- Post Processing says that with the Sequencer switch on, scene strips "will also be rendered as part of the pipeline", and "If Compositing is also enabled, the Scene strip will be the output of the Compositor." From this I **infer** that a brought-in scene's compositor can run during the main render. The sentence does not say whose Compositing switch counts (the main scene's or the brought-in scene's). It also says nothing about File Output nodes.
- "Scene strips do not render individual Render Passes; only the Combined render pass will be used." The sources do not say how this affects File Output nodes in the brought-in scene that are fed render passes.
- The sequencer's own "Sequence Render Image" and "Sequence Render Animation" "don’t render Scene Strips" by default, and use the preview's shading mode instead. From this I **infer** that the brought-in scene's compositor does not run in those two operations by default. Not stated for File Output nodes.
- The API calls the render cache setting one that "affects indirectly rendered scenes", which shows the API recognises such scenes. It says nothing about File Output there.

**Evidence.**

- "Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor." Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html, read 2026-10-11.
- "Scene strips are a way to insert the render output of another scene into your sequence." Also: "Scene strips do not render individual Render Passes; only the Combined render pass will be used." Manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/scene.html, read 2026-10-11.
- "Sequence Render Image and Sequence Render Animation don’t render the final video by default – specifically, they don’t render Scene Strips, instead using the preview’s shading mode (which is initially Solid)." Manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/sequencer/navigating.html, read 2026-10-11.
- `use_render_cache`: "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)". API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- `SceneStrip`: "Sequence strip using the rendered image of a scene". API, https://docs.blender.org/api/5.3/bpy.types.SceneStrip.html, read 2026-10-11.

**Not covered.**

- Whether the brought-in scene's File Output nodes write, and with which settings.
- Whose Compositing switch applies.
- Whether showing or playing the strip in the editor (including the scene-strip thumbnails added in 5.3) runs them.

**Where I looked.**

- I searched the whole manual for "scene strip". It appears on 15 pages, all of which I checked for the relevant passages: the Scene Strip page, Post Processing, Sequencer Scene, the scene introduction, the strips introduction and index, sequencer navigation, the preview header and sidebar, the cache sidebar, editing, selecting, storyboarding, Alpha Over, workspaces, and dope-sheet overlays.
- I searched the video-editing and sequencer-editor pages for "composit". The hits were compositor strips and modifiers, which are unrelated.
- The SceneStrip and RenderSettings API pages; I searched the API for "indirectly rendered".
- I searched the 5.3 sequencer, rendering and index release notes for "scene".

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, `Outer`'s render wrote the same file, and showing or playing the strip in the sequencer wrote nothing. The original note from the sources follows. The sources do not say. The settings involved (`use_sequencer`, `use_compositing`, `SceneStrip.scene`) are script-settable (documented).

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 15).** **The file every test was built from**, by a script, one copy per variant in its own folder under `<W>/runs/e15/` (the work folder written `<W>`), so that no part of a name can be ambiguous about which scene it came from:

| | the brought-in scene | the containing scene |
|---|---|---|
| name | `Inner` | `Outer` |
| camera | `InnerCam`, plus a second camera `OverrideCam` | `OuterCam` |
| frames per second | 24 | 30 |
| resolution | 320 × 180 | 640 × 360 |
| frame range | 1 to 5 | 21 to 25 |
| its own render output | `//watch_innerown/inner_own_` | `//watch_outer/outer_` |

`Outer`'s sequencer holds one Scene strip of `Inner` on channel 1, placed at outer frame 21 and five frames long, so outer frame 21 lines up with inner frame 1. `Inner`'s compositor tree holds a Render Layers node feeding a File Output node named `FO_inner` (folder `//watch/`, File Name `inner_`, one output `Img`) and the tree's Group Output node. `Outer` has no compositor tree unless a variant gives it one. Both scenes have Compositing and Sequencer on unless a variant says otherwise; both render with Cycles on the processor at one sample. Every render was started from the Mac's Terminal with `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b t.blend` and the arguments shown.

**The control.** Rendering `Inner` on its own, `-S Inner -a`, `exit status 0`, put `watch/inner_Img0001.png` to `inner_Img0005.png` on disk, beside `watch_innerown/inner_own_0001.png` to `0005`. So an empty `watch` folder below can only mean that variant's own change stopped the writing.

**The brought-in scene's File Output node writes.** Rendering `Outer`'s animation, `-S Outer -a`, `exit status 0`, left one file in `watch`, `inner_Img.png`, written five times, with no frame number in its name; `Outer`'s own render was written as `outer_0021.png` to `outer_0025.png`. Blender's own lines for the first two frames:

```
00:00.423  render           | Fra: 1 | Compositing
00:00.491  render           | Saved: '<W>/runs/e15/plain/watch/inner_Img.png'
00:00.502  render           | Saved: '<W>/runs/e15/plain/watch_outer/outer_0021.png'
00:00.506  render           | Fra: 2 | Compositing
00:00.512  render           | Saved: '<W>/runs/e15/plain/watch/inner_Img.png'
00:00.521  render           | Saved: '<W>/runs/e15/plain/watch_outer/outer_0022.png'
```

**What stops it, and what does not**, each variant rendered with `-S Outer -a`; the count is of the inner node's own `Saved:` lines, and in every row `Outer`'s own five frames were written, with `exit status 0` and no error line:

| what was changed | the brought-in scene's node |
|---|---|
| nothing | 5 writes, `watch/inner_Img.png` |
| the **brought-in** scene's Compositing switch off | **nothing** |
| the **containing** scene's Compositing switch off | 5 writes, `watch/inner_Img.png` |
| both Compositing switches off | **nothing** |
| the containing scene's **Sequencer** switch off | **nothing** |
| the strip muted | **nothing** |
| the strip's **Input** set to Sequencer, with a colour strip in `Inner`'s own sequencer | **nothing** |
| the brought-in scene two levels down, behind a scene `Middle` (frames 11–15) | **nothing** |
| the brought-in tree left with no Group Output node | 5 writes, `watch/inner_Img.png` |
| `use_sequencer_override_scene_strip` on | 5 writes, `watch/inner_Img.png` |
| a File Output node added to the **containing** scene as well (folder `//watch_outerfo/`) | 5 writes, `watch/inner_Img.png`; **the containing scene's own node wrote nothing at all** — no `watch_outerfo` folder |

So **it is the brought-in scene's own Compositing switch that governs, and the containing scene's has no effect** — all four combinations were run. **Two levels down was run with a control**: the very same file rendered with `-S Middle -a` did write `watch/inner_Img.png` five times, beside `watch_middle/middle_0011.png` to `0015`.

**How often it writes: once for every frame of the strip that is rendered.** Trimming the strip to three frames: 3 writes. The containing scene's frame step set to 2: 3 writes. The strip's length set to ten frames over the five-frame scene, `Outer` running 21–30 (the build reported the strip as `('Inner strip', 'Inner', 1, 21, 31, 'on', 'CAMERA')`): 10 writes. Two strips of the same scene on channels 1 and 2, both at frame 21: 10 writes, two per frame, one file. Two strips at frames 21 and 26, `Outer` running 21–30: 10 writes, one file; nothing in the name told the two strips apart. Every one of these left the single file `watch/inner_Img.png`; for example, with two strips at once:

```
00:00.490  render           | Saved: '<W>/runs/e15/two_same/watch/inner_Img.png'
00:00.502  render           | Saved: '<W>/runs/e15/two_same/watch/inner_Img.png'
00:00.524  render           | Saved: '<W>/runs/e15/two_same/watch/inner_Img.png'
00:00.534  render           | Saved: '<W>/runs/e15/two_same/watch/inner_Img.png'
```

(the first four of ten lines).

**A single frame writes too.** `-S Outer -f 21` wrote `Saved: '<W>/runs/e15/plain_f21/watch/inner_Img.png'` once. From a script on the file with `-S Outer` — `bpy.ops.render.render(animation=False, write_still=True)` after setting the current frame to 21 wrote `watch/inner_Img.png` and `watch_outer/outer_.png` and returned `{'FINISHED'}`; `bpy.ops.render.render(animation=False)`, with `write_still` left off, at frame 21 wrote `watch/inner_Img.png` alone — the containing scene's picture was not saved and the inner node's file still was — and returned `{'FINISHED'}`. **With the current frame left where the file had it, frame 1, outside the strip**, the still render wrote only `watch_outer/outer_.png` and nothing from the inner node:

```
    ACTIVE SCENE: Outer current frame before: 1
    00:00.395  render           | Saved: '<W>/runs/e15/plain_ops_left/watch_outer/outer_.png'
    RETURNED {'FINISHED'}
```

`bpy.ops.render.render(animation=True)` gave the same files as `-a`.

**A render pass is produced.** With the Mist pass on in `Inner` and fed to a second output `Mist`, both `watch/inner_Img.png` and `watch/inner_Mist.png` were written on every frame, ten writes in all — the documented limit that scene strips use only the Combined pass did not stop the brought-in scene making a pass for its own node.

**One tree shared by both scenes.** With `Outer`'s compositor set to the very same tree `Inner` uses, the node still wrote once per frame: 5 writes, one file.

**Moving the current frame with no render writes nothing.** From a script on the file with `-S Outer` and no window, the current frame was set to each of frames 21–25, three times over: `WALKED frames 21-25 three times, no render; current frame 25`, and no file was written anywhere.

**The sequencer's own render command does nothing with no window.** From the same kind of script:

```
    bpy.ops.render.opengl({'animation': True, 'sequencer': True}) RETURNED {'CANCELLED'}
    bpy.ops.render.opengl({'animation': False, 'sequencer': True}) RETURNED {'CANCELLED'}
```

and no file was written. What the same arrangement does with Blender's window open is entry 21.

**Observed by hand test, from Blender's own window (2026-10-18, entry 21).** Copies of the plain file above were opened in Blender 5.3 with its window open, driven by script as described under question 2; the window's scene was set to `Outer`. The pictures are described here and were not kept.

**A render of `Outer` started from inside the window writes the same file as from a terminal**: `bpy.ops.render.render(animation=True)` from the window, with `Render ‣ Lock Interface` on:

```
00:22.631  render           | Saved: '<W>/runs/w21/watch/inner_Img.png'
00:22.653  render           | Saved: '<W>/runs/w21/watch/inner_Img.png'
00:22.673  render           | Saved: '<W>/runs/w21/watch/inner_Img.png'
00:22.693  render           | Saved: '<W>/runs/w21/watch/inner_Img.png'
00:22.713  render           | Saved: '<W>/runs/w21/watch/inner_Img.png'
render.render(animation=True) OF Outer WITH THE WINDOW RETURNED {'FINISHED'}
```

**Showing the strip in the sequencer, rendered, and moving the playhead, writes nothing.** In this version the Video Sequencer shows the workspace's own sequencer scene, so the script set it to `Outer` (`WORKSPACE SEQUENCER SCENE SET TO: Outer`), turned the large area into the sequencer's Preview, and set the scene-strip display shading to Rendered (`scene strip display shading: RENDERED`). The preview then showed the brought-in scene's picture — the cube, shaded — inside an orange frame. The playhead was set to each of frames 21 to 25, three times over, with the area redrawn after each: `WALKED THE PLAYHEAD OVER FRAMES: [21, 22, 23, 24, 25, 21, 22, 23, 24, 25, 21, 22, 23, 24, 25]`, then `WATCH FOLDER after the playhead walk with the preview showing rendered scene strips: (no files)`. (A first run, made before the sequencer scene was set, showed a black preview; it wrote nothing either, but it showed nothing to write.) **The timeline's strip thumbnails write nothing either**: with the area turned into the sequencer's timeline and its thumbnails on, the strip was drawn as `Inner strip | Inner | ViewLayer` with small pictures of the cube along it, and about five seconds later `WATCH FOLDER after about 5 seconds of the timeline showing strip thumbnails: (no files)`.

**The sequencer's own render command, with the window and the sequencer showing `Outer`, rendered `Outer`'s own frames and nothing from the brought-in scene's node:**

```
00:07.129  render           | Saved: '<W>/runs/w21c/watch_outer/outer_0021.png'
00:07.130  render           | Saved: '<W>/runs/w21c/watch_outer/outer_0022.png'
00:07.131  render           | Saved: '<W>/runs/w21c/watch_outer/outer_0023.png'
00:07.134  render           | Saved: '<W>/runs/w21c/watch_outer/outer_0024.png'
00:07.142  render           | Saved: '<W>/runs/w21c/watch_outer/outer_0025.png'
bpy.ops.render.opengl({'animation': True, 'sequencer': True}) WITH THE WINDOW RETURNED {'FINISHED'}
bpy.ops.render.opengl({'animation': False, 'sequencer': True}) WITH THE WINDOW RETURNED {'FINISHED'}
WATCH FOLDER after the sequencer's own render command, with the sequencer showing Outer: (no files)
```

Run before the sequencer scene was set, the same two commands with the window returned `{'CANCELLED'}` and wrote nothing at all.

---

## 17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

**Answer.** **Not in the sources** for this case.

The general naming rule is **documented**:

- The path is "{Directory}/{File Name}{Socket Name}{frame number}.{extension}".
- `#` characters in the File Name or an output's name place the frame number there, padded with zeros.
- The extension is added when File Extensions is on (on by default).
- The Directory and File Name may contain template variables. These include `{blend_name}`, `{scene_name}` ("Name of the current scene"), `{camera_name}`, `{fps}`, `{resolution_x}`/`{resolution_y}` and `{node_name}`.

What the sources do not say for a brought-in scene:

- which scene counts as "the current scene" for `{scene_name}`, `{camera_name}`, `{fps}` and the resolution: the brought-in scene, or the scene doing the bringing in;
- which frame number goes into the name: the brought-in scene's own frame, or the main scene's frame.

**Ambiguity.** "Called" could mean the file name alone, or the full path. The general rule above covers both. Neither is answered for this case.

**Evidence.**

- "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}." Also: "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters." Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html, read 2026-10-11.
- "Available only in render output paths (including in the compositor File Output node):" followed by fps, resolution_x / resolution_y, "scene_name: Name of the current scene." and "camera_name: Name of the current render camera." Also: "node_name: The name of the node that the path property is on." Manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html, read 2026-10-11.
- "The default file name of the **File Output** node is now `{blend_name}`…". Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.
- `use_file_extension` "(default True)". API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html, read 2026-10-11.

**Not covered.** Every part that is specific to a scene brought in through the sequencer.

**Where I looked.**

- The same scene-strip searches as in question 16.
- The File Paths page.
- I searched the release notes for "template", "path" and "scene".
- The SceneStrip API page. It has a content trim, a camera override and a view-layer choice, but says nothing about output names or frame numbers seen by the brought-in scene.

**Outside the interface.** **Brought up to date by hand test, 2026-10-18: observed.** Every hand test under this question ran with Blender started from a terminal with no window and driven by a script or a command line, so the observed answers below hold when Blender is driven from outside its own window. With the window open, the same name was written, and the Output Paths panel shows it without a frame number. The original note from the sources follows. The sources do not say. The `-o` command-line option sets the render path. The sources do not say it changes File Output names.

**Operating system.** **Brought up to date by hand test, 2026-10-18: observed.** Observed on one machine only: the Creator's Mac, macOS 26.6.2 on an Apple M1 Pro. No Linux or Windows machine ran any of it in this sitting, so the observed answers below cover macOS alone and leave Linux and Windows open; see *Cannot be run* in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`. The original note from the sources follows. The sources do not say.

**Observed by hand test (2026-10-18, entry 2) — the ordinary case, in a scene rendered directly.** A node whose File Name was `{scene_name}_{node_name}_`, named `FO_d`, with one output `img`, in Blender's starting scene (called `Scene`), wrote from a still render

```
00:00.502  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/File output nodes/runs/e02/unsaved/d/Scene_FO_d_img.png'
```

and the same name with the file saved first. The starting File Name `{blend_name}` came out as the blend file's name, or as `Unsaved` in a file never saved (quoted under question 2). A still render put no frame number in any of these names. The case this question asks about — a scene brought in through another scene's sequencer — is the next block.

**Observed by hand test (2026-10-18, entry 16) — a scene brought in through another scene's sequencer.** The file is the one described under question 16 (`Inner`, 24 fps, 320 × 180, frames 1–5, camera `InnerCam`, brought into `Outer`, 30 fps, 640 × 360, frames 21–25, camera `OuterCam`, by a Scene strip at outer frame 21; `Inner`'s node `FO_inner`, folder `//watch/`, File Name `inner_`, output `Img`), one copy per variant under `<W>/runs/e15/` (the work folder written `<W>`), each rendered from the Mac's Terminal with `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b t.blend -S Outer -a` unless said; `exit status 0` and no error line every time.

**What the file is called.** In the plain arrangement, one file, `watch/inner_Img.png`, written over five times, with no frame number in the name.

**Which frame number: the brought-in scene's own.** With the File Name set to `inner_####_`:

```
00:00.476  render           | Saved: '<W>/runs/e15/hash/watch/inner_0001_Img.png'
00:00.486  render           | Saved: '<W>/runs/e15/hash/watch_outer/outer_0021.png'
00:00.497  render           | Saved: '<W>/runs/e15/hash/watch/inner_0002_Img.png'
00:00.506  render           | Saved: '<W>/runs/e15/hash/watch_outer/outer_0022.png'
```

and so on to `inner_0005_Img.png` beside `outer_0025.png`. One frame, `-S Outer -f 21`, gave `inner_0001_Img.png` beside `outer_0021.png`. Without `####` there is no frame number at all, and one file is written over.

**Every naming variable takes its value from the brought-in scene.** With the File Name `{scene_name}-{camera_name}-{fps}-{resolution_x}x{resolution_y}-{node_name}-{blend_name}_`:

```
00:00.487  render           | Saved: '<W>/runs/e15/templates/watch/Inner-InnerCam-24.0-320x180-FO_inner-t_Img.png'
```

— `Inner`, not `Outer`; `InnerCam`, not `OuterCam`; `24.0`, not `30`; `320x180`, not `640x360`; the node's own name; and `t`, the open blend file's name.

**The strip's own camera setting does not reach the name.** With the strip's camera set to `OverrideCam` and the File Name `{camera_name}_`, the file came out `Saved: '<W>/runs/e15/camera_override/watch/InnerCam_Img.png'`.

**`{blend_name}` and `//` name the open file, not a linked library.** `Inner` was saved in its own file, `<W>/runs/e16/libdir/library.blend`, with the node's folder `//watch/` and File Name `{blend_name}_`, and **linked** into `<W>/runs/e16/maindir/mainfile.blend`, which holds `Outer` and the strip (`LINKED SCENE: Inner library: <W>/runs/e16/libdir/library.blend`). Rendering `mainfile.blend` with `-S Outer -a`:

```
00:00.480  render           | Saved: '<W>/runs/e16/maindir/watch/mainfile_Img.png'
00:00.481  render           | Saved: '<W>/runs/e16/maindir/watch_outer/outer_0021.png'
```

and nothing was written beside the library.

**The node's own format decides the extension**: set to JPEG, with both scenes' own output on PNG, `watch/inner_Img.jpg` beside `watch_outer/outer_0021.png` and the rest. **Multi-Layer EXR through a strip**: one file, `watch/inner_.exr` — the output's name `Img` is not in it, and there is no frame number. **The command line's `-o` does not reach the node**: with `-S Outer -o <W>/runs/e15/plain_o/override/override_ -a`, the containing scene's render moved to `override/override_0021.png` to `0025`, and the node still wrote `watch/inner_Img.png`. **Nothing in the name tells two strips apart** (under question 16). What the node's Output Paths panel shows is entry 21.

**Observed by hand test, from Blender's own window (2026-10-18, entry 21).** The same file opened with Blender's window open (driven as described under question 2), `Outer`'s animation rendered from inside the window wrote `watch/inner_Img.png` five times — the same name as from a terminal (quoted under question 16). **What the node's Output Paths panel shows.** The compositor was pinned to `Inner`'s tree with `FO_inner` selected, the sidebar opened on its Node tab and the Output Paths panel opened by a simulated click on its header. The panel shows a ticked **File Extensions** checkbox and one row, a picture icon and `/private/tmp/claud...tch/inner_Img.png` — the middle left out by Blender for width — that is, the name with no frame number, the name the node wrote through the strip; rendered on its own the node wrote `inner_Img0001.png` to `inner_Img0005.png` (under question 16). The row was the same, pixel for pixel, with `Outer` and then `Inner` as the window's scene.

---

## Things a running Blender 5.3 could settle

*Pruned on 2026-10-18 by the hand-test sitting.* The twelve checks that stood here were run on the Creator's Mac on 2026-10-18, together with every test the four earlier versions carry an observed result for, and the results are written into the questions above as observed findings. The numbered tests, with every script and command, are in `Audits/Blender Research/File output nodes/Hand tests — 5.3.md`; which of them answered which check: the starting folder and name (1) — entries 1, 2 and 17, under question 2; the number of outputs (2) — entry 1, under question 1; video (3) — entries 1, 10 and 17, under question 11; a folder in a name (5) — entry 3, under question 4; the same name in the same place (6) — entries 9 and 17, under question 10; muting (7) — entry 8, under question 8; groups (8) — entries 7 and 12, under question 7; changing between frames (9) — entries 13 and 19, under question 14; one file failing (10) — entries 14 and 20, under question 15; when files are written (11) — entries 12 and 18, under question 13; a brought-in scene (12) — entries 15, 16 and 21, under questions 16 and 17.

**Only these are still open.**

1. **A network folder (Q5, Q15)** — writing to one, and one taken away part-way through a render. Not run: the Mac has no network share, and making one would change its system settings; there was no second machine.
2. **Linux and Windows (every question).** No such machine was available in this sitting, so every observed answer above covers macOS on an Apple M1 Pro alone.
3. **What the node's File Format menu draws (Q9, Q11).** The script could not open that menu with Blender's simulated events; what the setting accepts is settled from a script.
4. **The words of the Override Node Format tooltip (Q2).** No tooltip appeared on it within about 2.5 seconds of the simulated mouse resting there.
5. **`{project_root}` in a file that has a project (Q5).** Tried only with no project made, where it is a template error.
6. **A blend file Blender is not allowed to read (Q15).** Making one needs a change of file permissions, which the tool the sitting ran in refused; a missing and an empty blend file were run instead.
7. **A file held open and locked by another program (Q15).** macOS does not enforce that kind of lock against an unrelated program.

---

2026-10-17 — Claude Opus 5.5 (Claude Code) — File output nodes, Blender 5.3

# File output nodes — Blender 5.1

**Sources consulted, and when.** Blender 5.1 manual (plain-text snapshot) and its `_images` folder; Blender 5.1 Python API documentation (plain-text snapshot); Blender 5.1 release notes; Cycles' own files for 5.1. All four were read on 2026-08-02, from a snapshot taken on 2026-07-30. The date read given against every finding below is 2026-07-30, the snapshot date.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

No helper agents were used. Every finding below is my own reading.

---

## Observed by hand test

Everything under this heading was settled by running Blender 5.1.2 on the Creator's own computer — one MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2 — on **2026-08-02**. Blender was started from a terminal with no window, in the form

`"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup --python /tmp/run.py`

and driven through its Python support. The exact scripts, and which test number each result came from, are in `Audits/Blender Research/File output nodes/Hand tests.md`. The machine's own output is quoted below word for word; where a listing is long, the part that carries the answer is kept and the rest is said to be omitted.

A few of the blocks below are marked **from Blender's own window** instead. Those were run by the Creator on the same computer on the same day, by opening a prepared file, reading the node's panel, and rendering from the window. They are the tests that needed eyes on the screen.

---

## Observed by hand test on a second machine

On **2026-08-06** the same tests were run again on a second computer: **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; its screen is shared with other work. Blender 5.1.2 there is the Linux x64 build from Blender's own download server, at `/opt/blender/blender-5.1.2-linux-x64/blender`, and it was started the same way:

`/opt/blender/blender-5.1.2-linux-x64/blender -b --factory-startup --python run.py`

with `BLENDER_USER_RESOURCES` pointed at a settings folder of its own, and with `DISPLAY` and `XAUTHORITY` unset so that the display's own `Authorization required` noise stayed out of the output. The scripts were the Mac's, word for word, with only the folder they write into changed. Two of them needed Blender's window, which was opened on that machine's own display and photographed with Blender's own screenshot command. The entries and the full output are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

Every question below therefore carries results from two machines. **A Mac result and a Linux result are two results and are never merged.** Where they agreed, both are named and the agreement is said plainly; where the machines differed, both are given and neither is chosen.

**These blocks update the two standing lines for every question they touch.** *Driven from outside*: every result not marked as coming from the window was produced with Blender driven from outside its own interface, with no window open, so each holds in that case; where a window result differs from the terminal one, both are given. That is now true on both machines. *Operating system*: every answer that carries both an *Observed by hand test* block and an *on the rented Linux machine* block below is settled on **macOS 26.5.2 on an Apple M1 Pro and on Ubuntu 22.04.5 LTS on this rented machine**, and on nothing else. **Windows was not tested at all**: no Windows machine has run it. Both hand-test lists record it under *Cannot be run*.

---

## 1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?

**Answer.** It writes images to disk. Each connected input becomes one written image, and by default the node starts with no inputs at all — you add them. The sources do not state any maximum number of outputs.

What the node writes, and the naming pattern:

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

How inputs come to exist, and what each one is:

> "By default, the node has no input sockets. Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel."

> "Each input corresponds to an image that will be written to disk."

The Images/Layers panel is described as listing them:

> "Lists all input sockets of the node. The name of each input socket is used to construct the final file name."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

On how many. The API gives no upper bound. The index of the active item is described as:

> "active_item_index
>
> Index of the active item (in [0, inf], default 0)"

and the collection of items offers `new`, `remove`, `clear` and `move` with no stated limit:

> "Collection of file output items"

- Source: Python API, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html and https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItems.html
- Date read: 2026-07-30
- Label for "no maximum is stated": **not in the sources**. I searched the manual and the API text for `maximum`, `limit`, `max sockets`, `MAX_`, and read both `NodeCompositorFileOutputItem` and `NodeCompositorFileOutputItems` pages in full. Neither states a cap. `[0, inf]` is the documented range of an index property, which is not the same as a documented statement about how many outputs the node supports.

On what each output carries. Each output has a data type, given as a property on the item:

> "socket_type
>
> (default 'FLOAT')
>
> Type:
>
> Literal[Node Socket Data Type Items]"

> "vector_socket_dimensions
>
> Dimensions of the vector socket (in [2, 4], default 0)"

- Source: Python API, https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html
- Date read: 2026-07-30
- Label: **documented**

The picture on the manual page shows the node in its starting state: a header reading "File Output", a directory field containing `/tmp/`, a file-name field containing `file_name`, and a single blank socket on the left edge with no label.

- Source: manual image `_images/node-types_CompositorNodeOutputFile.webp`, shown on https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** Which members of the "Node Socket Data Type Items" list the File Output node actually offers. That enum is the general node-socket list — its twenty-three values are FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK and SOUND, at https://docs.blender.org/api/5.1/bpy_types_enum_items/node_socket_data_type_items.html — shared with other node types.

*Changed by the citation check, 2026-08-02: this list gave fifteen values and stopped at Texture. The cited page carries twenty-three. The eight that were missing have been added.*
 Nothing in the sources says which of them a File Output item may be set to, and I will not guess. Also not covered: what happens to a non-image data type (a string, a matrix) when the node tries to write it.

**Outside the interface.** The sources do not say. The node's inputs are reachable as properties and through the operators `bpy.ops.node.file_output_item_add`, `file_output_item_move` and `file_output_item_remove` (https://docs.blender.org/api/5.1/bpy.ops.node.html), which implies they can be built without a person clicking — but the two ways of adding an input that the manual names (dragging a link, using the sidebar panel) are both interface actions.

**Operating system.** The sources do not say. Nothing on the page distinguishes operating systems.


**Observed by hand test (2026-08-02).** **The node has nothing on its output side** — `NODE HAS OUTPUT SOCKETS: []` — and the old pair of slot collections is gone: `TYPE NodeOutputFileSlotFile present=False`, `TYPE NodeOutputFileSlotLayer present=False`, `TYPE NodeCompositorFileOutputItem present=True`.

**A new node does not start on the same format as the scene.** `A NEW NODE STARTS ON: file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE` while `THE SCENE STARTS ON: file_format=PNG media_type=IMAGE`. A File Output node added and left alone therefore writes one multi-layer OpenEXR file rather than separate pictures.

**Which socket kinds an output may be set to**, which this answer left open, read from the running program:

```
    socket_type | ENUM | description='' values=['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE', 'FONT', 'SCENE', 'TEXT', 'MASK', 'SOUND']
```

That is what the setting will accept; what such an output actually writes is a different question, and question 12 below carries the part of it that was tested.

**No ceiling was reached on how many outputs a node takes**: `OUTPUTS ADDED WITHOUT REFUSAL: 2000` and `INPUT SOCKETS ON THE NODE NOW: 2001` — two thousand outputs plus the blank socket — with nothing refused. That establishes at least two thousand and says nothing above it. (Hand tests 1 and 14.)


**Observed by hand test, from Blender's own window (2026-08-02).** Outputs are added and removed with a plus and a minus beside the list in the **Images** panel, with a pair of arrows for reordering, and the kind of thing an output carries is drawn as a **Socket Type** dropdown below the list. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed with the Mac on every part.** `NODE HAS OUTPUT SOCKETS: []`; `TYPE NodeOutputFileSlotFile present=False`, `TYPE NodeOutputFileSlotLayer present=False`, `TYPE NodeCompositorFileOutputItem present=True`; and `A NEW NODE STARTS ON: file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE` while `THE SCENE STARTS ON: file_format=PNG media_type=IMAGE`. The same twenty-three socket kinds were published:

```
    socket_type | ENUM | description='' values=['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE', 'FONT', 'SCENE', 'TEXT', 'MASK', 'SOUND']
```

and the same ceiling was not reached: `OUTPUTS ADDED WITHOUT REFUSAL: 2000`, `INPUT SOCKETS ON THE NODE NOW: 2001`, with nothing refused. **Which of those twenty-three the node will actually accept was tested on Linux for the first time; it is three of them, and that is under question 12.**

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 1, 14 and 21.)

---

## 2. Which settings belong to an individual output, and which belong to the node as a whole?

**Answer.** The folder, the base file name, the media type and the default format belong to the node. The name, the format override and the per-output format belong to the individual output.

Node-wide, from the manual:

> "Directory
>
> The base output directory.
>
> File Name
>
> The base name of the file. The final filename may also include frame numbers or layer names depending on the settings.
>
> Media Type
>
> Determines how multiple inputs are stored"

and the node's format panel:

> "Image: Controls the default format and encoding for all outputs. To override settings for an individual output, enable Override Node Format in the Images/Layers panel."

Per-output, from the manual:

> "When saving individual images, each output can be renamed and will be written to disk using either the global Node Format settings or its own overridden settings."

> "Override Node Format
>
> Allows the selected output to use a custom format instead of the global Node Format.
>
> Item Format
>
> When Override Node Format is enabled, this panel controls the format and encoding options for the selected output image."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

The API lists the same split precisely. On the node: `directory`, `file_name`, `format`, `save_as_render`, `active_item_index`, `file_output_items`.

> "directory
>
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name
>
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

> "save_as_render
>
> Apply render part of display transform when saving byte image (default False)"

- Source: Python API, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html
- Date read: 2026-07-30
- Label: **documented**

On the individual output: `name`, `format`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions`, and a read-only `color`.

> "override_node_format
>
> Use a different format instead of the node format for this file (default False)"

> "save_as_render
>
> Apply render part of display transform when saving byte image (default False)"

> "color
>
> Color of the corresponding socket type in the node editor (array of 4 items, in [0, inf], default (0.0, 0.0, 0.0, 0.0), readonly)"

- Source: Python API, https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html
- Date read: 2026-07-30
- Label: **documented**

Note that `save_as_render` appears on **both** — the node has one and each output has one. Both are described with the identical sentence quoted above.

There is also a read-only display panel that belongs to the node as a whole:

> "Output Paths ... Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** What happens when the node-level `save_as_render` and an output-level `save_as_render` disagree, and whether the output-level one only takes effect when Override Node Format is on. The sources describe both properties but never their interaction. Also not covered: which of the format settings inside `ImageFormatSettings` are actually exposed on a File Output node's panels.

**Outside the interface.** Every setting named above exists as a property in the Python API, so all of them are reachable without the sidebar. The panel names (Node Format, Images/Layers, Item Format, Output Paths) are interface groupings only. Label for that last sentence: **inferred** — the manual gives the panels a "Panel: Sidebar ‣ Node ‣ Properties ‣ …" reference, and the API lists the same settings as plain properties with no panel; a reader must join those two facts.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **The full list of settings, from the running program.** On the node: `file_output_items`, `active_item_index`, `directory`, `file_name`, `format` (read-only) and `save_as_render`. On one written output: `name`, `socket_type`, `vector_socket_dimensions`, `color`, `override_node_format`, `save_as_render` and `format`. That matches the answer above; there is no `use_file_extension` on the node in this version.

**Which display-transform setting wins, which this answer left as not covered.** Eight combinations, one at a time, with the written pixel read back:

```
    node save_as_render=False  output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.051, 0.051, 0.051, 1.0)
    node save_as_render=False  output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.051, 0.051, 0.051, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.043, 0.043, 0.043, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.043, 0.043, 0.043, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
```

While Override Node Format is off, only the node's `save_as_render` changes the file; while it is on, only the output's own does. So the output-level one does depend on the override being on, which is what this answer could not settle. (The values in the two halves are not on the same scale, because the node's format block and the output's own were not left carrying identical settings; what the test shows is which flag moves the value.) (Hand tests 1 and 16.)


**Observed by hand test, from Blender's own window (2026-08-02).** **Every control the panel draws, with a File Output node selected and the sidebar open on the Node tab.** Under **Node**: Reset Node, Name, Label, Color, **Show Options** and **Mute**. Under **Properties**, belonging to the node: the folder field with a folder-browse button, the file-name field, then a two-button choice reading **Image** | **Multi-Layer EXR**, then a **Node Format** panel holding Save as Render, File Format, Color, Color Depth, Compression and a Color Management panel (Follow Scene or Override; Override adds Display, View, Look, Exposure, Gamma, Use Curves with its curve editor, and Use White Balance with Temperature and Tint). Then an **Images** panel holding the list of outputs. Belonging to the selected output: **Socket Type**, **Override Node Format**, and an **Item Format** panel repeating the same format and colour-management controls. Then an **Output Paths** panel listing the finished path of each output. Finally **Custom Properties**.

Neither the folder field nor the file-name field carries a visible label beside it; the manual's names Directory and File Name are not drawn on the panel.

**The Override Node Format tooltip reads:** "Use a different format instead of the node format for this file." That is word for word what the API reference gives, so the two agree. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux printed the same list of settings and the same eight pixel values.**

```
    node save_as_render=False  output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.051, 0.051, 0.051, 1.0)
    node save_as_render=False  output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.051, 0.051, 0.051, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.043, 0.043, 0.043, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.043, 0.043, 0.043, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
```

Those are the Mac's eight values exactly. Colour handling is one of the things most likely to differ between machines, and here it does not.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 1 and 16.)


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **What the node draws on its own face.** The node's body carries a header with the node's label, then two fields one above the other — the folder, and the base file name — then one row for each written output showing the output's name alone, and finally a row holding a `+`, which is the blank socket a new input is dragged onto. With outputs called `one` and `two` the rows read `one` and `two`. Neither of the two fields carries a written label beside it. The Creator's Mac screenshots were of the sidebar rather than the node's face, so this is the Linux picture and nothing more. (Linux entry 17.)

---

## 3. Does each output carry its own folder as well as its own name?

**Answer.** No — there is one folder for the whole node. Each output carries only a name. But a folder can be written inside that name (see question 4).

The folder is a node-level property, given once:

> "Directory
>
> The base output directory."

The item has a name and no folder property of its own. Its full list of properties is `color`, `format`, `name`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions` — there is no directory or path property among them.

> "name
>
> (default "", never None)"

- Sources: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html ; Python API, https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html
- Date read: 2026-07-30
- Label: **documented** for "one Directory on the node" and for the item's property list. **Inferred** for "therefore no separate per-output folder field": the step is that the API page lists an item's properties exhaustively and none of them is a folder, so a per-output folder field does not exist as a separate setting.

**Not covered.** Whether the per-output name is combined with the node Directory in any way other than the documented pattern `{Directory}/{File Name}{Socket Name}{frame number}.{extension}`.

**Outside the interface.** The sources do not say; the properties are the same ones either way.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Confirmed by running it, with one addition.** A name can put folders under the node's own folder, and a name reaching *upwards* is stopped: `../escape` was not kept as typed — `ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'` — and the file landed inside the node's folder as `base/file_name_./escape.png`, not outside it. A name beginning with a slash is joined under the node's folder too, as `base/file_name/absolute.png`. So the folder on the node is a floor the outputs cannot climb out of. (Hand test 4.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux stopped the upward climb in exactly the same way.** `ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'`, and the file landed inside the node's folder as `out/base/file_name_./escape.png`, not outside it. A name beginning with a slash was joined under the node's folder too, as `out/base/file_name/absolute.png`.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed — including the part most likely to differ, a name trying to climb out of its folder. Windows is untested. (Linux entry 4.)

---

## 4. Can a folder be expressed inside an output's name rather than separately?

**Answer.** Yes, when the Media Type is Image. A slash in an output's name is read as a folder structure, and missing folders are made.

> "When the Media Type is set to Image, input names are also used to construct subdirectories:
>
> If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output.
>
> This allows organizing outputs into folders directly from the node setup, for example: passes/diffuse or masks/object_01."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

The release notes state this as a change:

> "The **File Output** node now allows specifying directories in item names. This was possible in older versions of Blender and the functionality is now restored. (blender/blender@e19dff1ef8)"

- Source: release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** What happens if the Media Type is Multi-Layer EXR and a name contains a slash — the manual conditions the whole behaviour on "When the Media Type is set to Image" and says nothing about the other case. Also not covered: whether a name may reach *upwards* out of the base directory (`../`), and what happens if a folder cannot be created (no permission, read-only volume).

**Outside the interface.** The sources do not say. The wording "created automatically during output" ties folder creation to the moment of writing, not to anything a person does in the interface — but that is my reading, not a statement about running Blender headless.

**Operating system.** Partly answered by the sources: the manual names both slash characters, "/ or \", as triggering the behaviour. It does not say whether one, the other, or both work on each operating system. Label for that: **documented** that both characters are listed; **not in the sources** for any per-operating-system difference. That gap has since been closed by hand on two machines — see the observed blocks below.


**Observed by hand test (2026-08-02).** **Confirmed by running it, and the limits this answer left open are now known.** With the node pointed at `/tmp/fo/out/base`:

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
RESULT [5.1.2] separators in output names, Image mode, node folder /tmp/fo/out/base
    base/file_name/absolute.png  830 bytes
    base/file_name_./escape.png  830 bytes
    base/file_namea/b/c/deep.png  830 bytes
    base/file_namesub/shot.png  830 bytes
```

Three things follow. Folders are made, several levels deep where asked, and the node's File Name is joined to the front of the name before the split, so `sub/shot` under a File Name of `file_name` gives a folder called `file_namesub`. **A name cannot climb upwards**: `..` was turned into `_.` before the node kept it. **A leading slash does not make the path absolute**; it is joined under the node's folder.

**A backslash does the same thing as a forward slash.** A node whose only output was named `sub\shot`, pointed at a separate empty folder, produced

```
RESULT [5.1.2] one output only, named with a backslash, node folder /tmp/fo/out/base2
    base2/file_namesub/shot.png  830 bytes
```

**In Multi-Layer EXR mode the slash is kept in the layer's name and no folder is made** — the case the manual's "When the Media Type is set to Image" condition leaves open. One file was written, and its header text contains `'passes/diffuse.A', 'passes/diffuse.B', 'passes/diffuse.G', 'passes/diffuse.R'`:

```
RESULT [5.1.2] a separator in an output name, Multi-Layer EXR mode, node folder /tmp/fo/out/base/
    base/file_name.exr  10131 bytes
```

Nothing was reported to the terminal in any of these cases. Folder-creation failure is covered under question 5. (Hand tests 4 and 5.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux kept the same names and made the same folders.**

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
RESULT [5.1.2] separators in output names, Image mode, node folder /root/smoozy/File output nodes/work/fo/out/base
    out/base/file_name/absolute.png  811 bytes
    out/base/file_name_./escape.png  811 bytes
    out/base/file_namea/b/c/deep.png  811 bytes
    out/base/file_namesub/shot.png  811 bytes
```

A node whose only output was named with a backslash wrote `out/base2/file_namesub/shot.png`. In Multi-Layer EXR mode the slash was kept in the layer's name and no folder was made: the written file's header text carries `'passes/diffuse.A', 'passes/diffuse.B', 'passes/diffuse.G', 'passes/diffuse.R'`.

**This is the sharpest operating-system question on the sheet, and both machines answer it the same way.** A backslash is a folder separator on Windows and an ordinary character in a file name on macOS and Linux; Blender treats it as a folder separator on both machines tested. Windows itself is untested.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 4 and 5.)

---

## 5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?

**Answer.** The folder is free text with two documented conveniences: a `//` prefix meaning "beside the blend file", and template expressions such as `{blend_dir}`. Folders unrelated to the blend file are shown in the manual's own picture of the node. Network folders are not mentioned anywhere in connection with this node.

The property itself:

> "directory
>
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

- Source: Python API, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html
- Date read: 2026-07-30
- Label: **documented**

On `//`:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

and a limit on it:

> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

- Source: manual, https://docs.blender.org/manual/en/5.1/files/blend/open_save.html
- Date read: 2026-07-30
- Label: **documented**

On template expressions, which the manual says apply to this node by name:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

The variables available:

> "Available in all paths that support template expressions:
>
> blend_name: The currently open blend file's name (without the .blend).
>
> blend_dir: Path up to (but not including) the currently open blend file.
>
> blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from.
>
> blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."

> "Available only in render output paths (including in the compositor File Output node):
>
> fps: … resolution_x / resolution_y: … scene_name: Name of the current scene. camera_name: Name of the current render camera."

> "Available only in path properties on a node:
>
> node_name: The name of the node that the path property is on."

And a failure mode that matters for unattended use:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

- Source: manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html
- Date read: 2026-07-30
- Label: **documented**

On folders unrelated to the blend file. The manual's picture of the node shows the directory field holding `/tmp/`, an absolute path with no relation to any blend file.

- Source: manual image `_images/node-types_CompositorNodeOutputFile.webp`, shown on https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented** that the picture shows `/tmp/` in that field. **Inferred** for "so absolute paths outside the blend file's folder are accepted": the step is that the field's shown value is such a path, and the documentation gives no rule excluding them.

On network folders: **not in the sources.** Where I looked. I searched the whole 5.1 manual and the whole 5.1 API text for `UNC`, `SMB`, `NFS`, `mapped drive`, `shared folder`, `network share`, `network drive`, `network path`, `network file system`, `remote path`. The only hits anywhere near this subject are three, and none is about the File Output node:

- `manual_5.1/advanced/deploying_blender.txt` mentions a "read-only network drive" as a place to install Blender.
- `manual_5.1/advanced/extensions/creating_repository/static_repository.txt` mentions `file://HOST/share/path/…` "for network shares on Windows", about extension repositories.
- The render Output panel's hint: "Set up a shared directory over a network file system." — this is under "Primitive Render Farm" and concerns the scene's own render output path plus the Overwrite and Placeholders settings, not the File Output node (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html).

I also read the File Output page, the File Paths page and the Relative Paths section in full. None mentions networks.

**Not covered.** Whether a network location works, what syntax it would take, and what happens on failure to reach it. Also not covered: whether the folder field accepts environment variables or a `~` home shortcut — I searched for both and found nothing tied to this node.

**Outside the interface.** Partly answered, and it matters here: the `//` prefix depends on the blend file having been saved, and template errors are documented to stop an animation render outright. The sources do not say anything further about how the node's folder behaves in a headless run. Note also that the command-line switch `-o, --render-output <path>` sets "the render path and file name" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html) — the sources never say it touches File Output node paths, and I make no claim that it does.

**Operating system.** The sources do not say. The example path shown in the node picture is Unix-shaped (`/tmp/`), and the input-name rule accepts both `/` and `\`, but no page states a per-operating-system rule for the Directory field.


**Observed by hand test (2026-08-02).** **A folder unrelated to where the Blender file sits is accepted**, which settles the part this answer had as inferred: every test on this sheet wrote into `/tmp/fo/out`, which has nothing to do with the file's own folder, and did so with the file unsaved.

**A folder that does not exist is made**, with no error printed:

```
FOLDER /tmp/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [5.1.2] node folder that did not exist
    out/does/not/exist/yet/file_namemade.png  830 bytes
```

**The `//` form works and is relative to the saved file:**

```
RESULT [5.1.2] node folder set to the text //relative/ with the file saved at /tmp/fo/out/held.blend
    out/held.blend  96688 bytes
    out/relative/file_namemade.png  842 bytes
```

**A folder that cannot be written to fails loudly and writes nothing:**

```
00:01.555  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/fo/out/locked/file_namemade.png"
00:01.555  reports          | ERROR Render error (Undefined error: 0) cannot save: '/tmp/fo/out/locked/file_namemade.png'
RESULT [5.1.2] node folder that cannot be written to (mode 500)
    (nothing written)
```

Network folders were not tested and could not be — see *Cannot be run* in the hand-test list. (Hand test 11.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts, and this test had to be run twice there.**

A folder that does not exist is made:

```
FOLDER /root/smoozy/File output nodes/work/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [5.1.2] node folder that did not exist
    out/does/not/exist/yet/file_namemade.png  811 bytes
```

The `//` form is relative to the saved file:

```
RESULT [5.1.2] node folder set to the text //relative/ with the file saved at /root/smoozy/File output nodes/work/fo/out/held.blend
    out/held.blend  96733 bytes
    out/relative/file_namemade.png  854 bytes
```

**Why the unwritable-folder test was run twice.** The account running Blender on that machine is the root user, and root walks through file permissions. Run as plain root, the folder set to permissions 500 was written to anyway:

```
RESULT [5.1.2] node folder that cannot be written to (mode 500)
    out/locked/file_namemade.png  854 bytes
```

Run again with that power dropped for the one command, which is the real answer, it failed loudly and wrote nothing, as on the Mac:

```
00:02.854  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/smoozy/File output nodes/work/fo/out/locked/file_namemade.png"
00:02.854  reports          | ERROR Render error (Success) cannot save: '/root/smoozy/File output nodes/work/fo/out/locked/file_namemade.png'
RESULT [5.1.2] node folder that cannot be written to (mode 500)
    (nothing written)
```

**The words differ between the machines and the behaviour does not.** The Mac said `Render error (Undefined error: 0)`; Linux says `Render error (Success)`. The words in the brackets are the operating system's own description of what went wrong, and the two systems describe it differently. What Blender did — refuse loudly and write nothing — is the same on both.

Network folders still could not be tested on either machine — see *Cannot be run* in both hand-test lists.

**Operating system:** the parts of this answer that could be tested are now settled on macOS and on Linux, and they agreed on what happens, differing only in the wording of the failure message. Windows is untested, and network folders are untested everywhere. (Linux entry 11.)

---

## 6. Can several outputs be gathered into one file rather than written separately?

**Answer.** Yes, by setting the node's Media Type to Multi-Layer EXR.

> "Media Type
>
> Determines how multiple inputs are stored:
>
> Image:
>
> Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

> "When using Multi-Layer EXR, each input is stored as a separate layer within the file."

> "Multi-Layer EXR: See OpenEXR for EXR-specific settings."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

Background on the format:

> "An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

- Source: manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** How the single file is named when Multi-Layer EXR is used — the documented pattern `{Directory}/{File Name}{Socket Name}{frame number}.{extension}` includes a socket name, which cannot apply unchanged when all sockets share one file. The manual does not resolve this. Also not covered: whether the per-output Override Node Format has any effect in this mode (the manual attaches per-output formats to the Image mode only).

**Outside the interface.** The sources do not say. The setting is the `media_type` value of the node's `format` (https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html), so it is reachable as a property.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Both things this answer left open are now settled. The single file takes its name from the node's File Name alone**, with no output name in it — it was `out/file_name.exr` whether the folder was written with a trailing slash or without one. **The layers are named after the outputs**: three outputs named `first`, `second` and `third` gave

```
RESULT [5.1.2] three outputs, Multi-Layer EXR mode
    out/file_name.exr  14944 bytes
```

whose header text carries `'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R'`, then the same for `second` and `third`, each with its own `'name', 'string', 'first'` and so on (the rest of the header text is omitted).

**The media type is what switches the node between the two ways of writing.** With the media type on Image, the node refused the multi-layer format outright:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

(Hand tests 5 and 6.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts.**

```
RESULT [5.1.2] three outputs, Multi-Layer EXR mode
    out/file_name.exr  14944 bytes
```

whose header text carries `'first.A', 'first.B', 'first.G', 'first.R'`, then the same for `second` and `third` (the rest omitted). The single file was `out/file_name.exr` whether the folder was written with a trailing slash or without one. And with the media type on Image the node refused the multi-layer format outright, in the same words as on the Mac.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 5 and 6.)

---

## 7. What happens to a File Output node inside a group of nodes?

**Answer.** **Not in the sources.** Nothing in the 5.1 manual, API documentation, release notes or the Cycles files states what a File Output node does when it sits inside a node group.

Where I looked, and what I ruled out:

- The File Output page itself (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), read in full. The word "group" does not appear on it.
- The compositor's Group page (https://docs.blender.org/manual/en/5.1/compositing/types/groups.html), read in full. It covers Group Input, Group Output and node groups generally, and never mentions File Output.
- The general node Groups page (https://docs.blender.org/manual/en/5.1/interface/controls/nodes/groups.html). The nearest statement is generic advice naming a different node: "Avoid using nodes output nodes such as Material Output inside node groups. These should be used on the top level node tree to improve re-usability of node groups." It does not name the File Output node, and it is advice rather than a statement of behaviour.
- The Compositor System page (https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html), read in full. It has an "Output" section, but that section is about which single output is *displayed*: "The compositor only supports a single active output target, that is, only one of the Group Output nodes or Viewer nodes in the node tree will be considered active and the rest will be ignored. In particular, the compositor searches the Active Node Tree Context and falls back to the Root Node Tree Context if no active output was found in the active node tree context." It names Group Output and Viewer nodes only. Extending that rule to File Output nodes would be a guess, and I am not making it.
- The 5.1 release notes for the compositor (https://developer.blender.org/docs/release_notes/5.1/compositor/), read in full — two File Output entries, neither about groups.
- Searches across the whole manual and API text for `File Output` (5 files, all listed above plus the index pages), and for `OutputFile` in the API.

**Not covered.** Everything about this: whether the node writes at all from inside a group, whether it writes once per use of the group, and whether a group used more than once produces colliding files.

**Outside the interface.** Unknown, since the base behaviour is undocumented.

**Operating system.** Unknown, for the same reason.


**Observed by hand test (2026-08-02).** **It writes, once, wherever it sits.** A File Output node placed inside a node group wrote its file; using the same group twice still produced one file; and putting that group inside a second group made no difference:

```
RESULT [5.1.2] one File Output node inside a group, the group used once
    out/g_grouped.png  830 bytes
RESULT [5.1.2] the same group used twice in the tree
    out/g_grouped.png  830 bytes
RESULT [5.1.2] the File Output node two groups deep
    out/g_grouped.png  830 bytes
```

Muting the group node from outside stopped it:

```
RESULT [5.1.2] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases. Note what the second case does not settle: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like. (Hand test 10.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [5.1.2] one File Output node inside a group, the group used once
    out/g_grouped.png  811 bytes
RESULT [5.1.2] the same group used twice in the tree
    out/g_grouped.png  811 bytes
RESULT [5.1.2] the File Output node two groups deep
    out/g_grouped.png  811 bytes
RESULT [5.1.2] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases, as on the Mac. The same caution applies: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 10.)

---

## 8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?

**Answer, node level.** There is a Mute for any node, including this one.

> "Mute
>
> Reference
>
> Menu: Node ‣ Show/Hide ‣ Mute
>
> Shortcut: M
>
> Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

- Source: manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html
- Date read: 2026-07-30
- Label: **documented** that Mute exists and what it does in general terms.

That a muted File Output node writes no files is **inferred**, not documented. The step: the manual says muting "removes its contribution to the node tree", and the File Output node's contribution is writing files; therefore a muted one writes nothing. The manual never says this about the File Output node, and "makes all links pass through it without change" is written for nodes that have outputs, which this node does not ("This node has no output sockets"). A reader has to supply the connection, so the label is inferred.

The corresponding property is `Node.mute`, listed among the node's inherited properties (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html).

**Answer, single output within a node.** **Not in the sources.** There is no documented per-output on/off switch. The full property list for one output is `color`, `format`, `name`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions` (https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html) — no mute, enabled, active or skip among them. The Images/Layers panel section of the manual describes renaming and format override only, no switch.

Where I looked: the File Output manual page in full; `NodeCompositorFileOutputItem` and `NodeCompositorFileOutputItems` in full; `bpy.ops.node` for File Output operators, which yields only `file_output_item_add`, `file_output_item_move` and `file_output_item_remove` (add, move, remove — no toggle) at https://docs.blender.org/api/5.1/bpy.ops.node.html; the node editing page for anything socket-level, which offers "Mute Links" for links rather than sockets.

Two nearby things that are documented and adjacent, but do not answer it:

- Mute Links: "A muted link acts as though it's no longer there; this also means the input fields" … "the links on its output side will be muted too." (https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html). This mutes a link, not an output of this node.
- The File Output page says "Each **connected** input is saved to the specified directory and filename". That an unconnected input is therefore not written is **inferred** — the word "connected" carries it, but the manual never states the negative case.

There is a separate node called Enable Output in the compositor, but its documented job is different and unrelated to files: "The Enable Output node toggles the visibility and value of a socket for a Node Group outputs." (https://docs.blender.org/manual/en/5.1/compositing/types/output/enable_output.html). It concerns node-group output sockets, not File Output node outputs.

**Not covered.** Whether removing a link, muting a link, or muting the node has the same effect on writing; and whether a muted File Output node still creates its folders.

**Outside the interface.** `Node.mute` is a property, so it can be set without the M shortcut. Whether the effect on file writing is the same in a headless render: the sources do not say.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Muting the node does stop it writing**, which this answer had as inferred. Starting from two connected outputs that both wrote:

```
RESULT [5.1.2] both outputs connected
    out/file_nameA.png  830 bytes
    out/file_nameB.png  818 bytes
RESULT [5.1.2] the whole node muted (node.mute = True)
    (nothing written)
```

**One output can be switched off on its own, through its link**, and an unconnected output writes nothing at all — not an empty file, not a black one, which is the negative case this answer said the manual never states:

```
RESULT [5.1.2] node unmuted, the link into output A muted (link.is_muted = True)
    out/file_nameB.png  818 bytes
RESULT [5.1.2] the link into output A removed, the output itself left on the node
    out/file_nameB.png  818 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
```

Muting the link and removing it give the same result. The socket's own `enabled` setting was `True` throughout and was not what did this. **A muted node does not create its folders either** — nothing appeared on disk at all. Whether the panel offers a per-output switch of its own is test 17 on the Creator's half. (Hand test 9.)


**Observed by hand test, from Blender's own window (2026-08-02).** **There is no per-output on/off control on the panel.** The Images list offers a plus, a minus and a pair of reorder arrows; right-clicking an entry gives Blender's ordinary property menu — Reset to Default Value, Copy to Selected, Copy Data Path, Copy Full Data Path, Copy as New Driver, Online Manual — and nothing that switches an output off. The only per-output controls are Socket Type, Override Node Format and the format settings under it. So removing an output, or cutting or muting its link as under the test above, is the way. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [5.1.2] both outputs connected
    out/file_nameA.png  811 bytes
    out/file_nameB.png  824 bytes
RESULT [5.1.2] the whole node muted (node.mute = True)
    (nothing written)
RESULT [5.1.2] node unmuted, the link into output A muted (link.is_muted = True)
    out/file_nameB.png  824 bytes
RESULT [5.1.2] the link into output A removed, the output itself left on the node
    out/file_nameB.png  824 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
SOCKET 'enabled' ON OUTPUT A: True
```

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 9 and 10.)

---

## 9. What restrictions, if any, apply to the kind of picture a File Output node can write?

**Answer.** The restrictions are those of the chosen image format, plus the split by Media Type.

By Media Type:

> "Image: Saves each input as a separate image file. Each input can use its own image format settings."
>
> "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

> "Image: Controls the default format and encoding for all outputs. To override settings for an individual output, enable Override Node Format in the Images/Layers panel. See Saving Images for details on supported formats and options."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

The manual sends the reader to Saving Images for the format restrictions. There, the internally supported formats are listed with their channel depth, alpha support, metadata support and DPI support: JPEG (8bit, no alpha, metadata, DPI, `.jpg .jpeg`); OpenEXR (float 16, 32bit, alpha, metadata, DPI, `.exr`); PNG (8, 16bit, alpha, metadata, DPI, `.png`); WebP (8bit, alpha, metadata, no DPI, `.webp`); AVIF (8, 10, 12bit, alpha, no metadata, no DPI, `.avif`); BMP (8bit, alpha, no metadata, DPI, `.bmp`); Cineon (8, 10, 12, 16bit, `.cin`); DPX (8, 10, 12, 16bit, `.dpx`); Iris (8, 16bit, `.sgi .rgb .bw`); JPEG 2000 (8, 12, 16bit, `.jp2 .jp2 .j2c`); Radiance HDR (float, `.hdr`); Targa and Targa Raw (8bit, `.tga`); TIFF (8, 16bit, DPI, `.tif .tiff`).

Explicit restrictions stated on that page:

> "Note, RGBA is not available for all image formats, check the list above for details."

> "Note, not all file formats support all color depths."

> "Float (Half): 16-bit floating point per channel. Offers high dynamic range with lower memory and storage usage. Only supported for OpenEXR files."

> "Float (Full): 32-bit floating point per channel. Provides the highest precision and dynamic range. Only supported for OpenEXR files."

> "Internally, Blender only operates in either 8-bit or 32-bit. Images with higher than 8-bit precision (e.g., 10-bit, 12-bit, 16-bit) are converted to 32-bit float when loaded into Blender."

> "DPX as well as Cineon only stores and converts the 'visible' color range of values between 0.0 and 1.0 (as a result of rendering or composite)."

> "Not all formats support metadata, and some may only support a limited subset of fields. When using formats that do not support metadata, render information is not preserved in the saved file."

> "Note, Cineon, DPX, OpenEXR, and Radiance HDR image types default to being saved in a linear color space."

- Source: manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html
- Date read: 2026-07-30
- Label: **documented**

The full set of format values the underlying property can take is AVIF, JPEG, OPEN_EXR, PNG, WEBP, BMP, CINEON, DPX, IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, OPEN_EXR_MULTILAYER, FFMPEG (https://docs.blender.org/api/5.1/bpy_types_enum_items/image_type_all_items.html), and the property is described as:

> "file_format
>
> File format to save the rendered images as (default 'PNG')"

- Source: Python API, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html
- Date read: 2026-07-30
- Label: **documented**

Metadata is relevant here and changed in 5.1:

> "The **Image** and **File Output** nodes now read and write custom meta-data. (blender/blender@2dec178560)"

- Source: release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** Which of the sixteen format values are actually offered in the node's Node Format panel — the enum is shared with the scene's own render output settings, and the sources never state the node's subset. Also not covered: any restriction on image *size* from this node. One loose end I could not settle: the Compositor System page says compositor data types are "all of which are stored in half precision formats", while the compositor sidebar's Precision setting says "Auto: Use full precision for final renders, half precision otherwise". I do not know which governs what a File Output node writes, and the sources do not join them.

**Outside the interface.** The sources do not say for this node. The command line can override the scene's render format with `-F, --render-format` (valid options listed as "TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF" plus "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP", https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). Nothing says this reaches File Output nodes, and I do not claim it does.

**Operating system.** Partly answered, and this is a real difference: the command-line page states that OPEN_EXR, OPEN_EXR_MULTILAYER, FFMPEG, CINEON, DPX, JP2 and WEBP are "Formats that can be compiled into Blender, not available on all systems". Which systems is not stated. Label: **documented** that availability is not universal; **not in the sources** for which systems lack which.


**Observed by hand test (2026-08-02).** **The node's own format list is narrower than the shared one, and what it will take depends on the media type** — the subset this answer said the sources never state. The setting's published list of values is

```
FILE FORMAT VALUES: ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
```

but with the node on Image media type it refused both the multi-layer value and the video value, and each refusal names what it will take:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

So on Image media type the node offers fourteen still-picture formats and neither the multi-layer nor the video value. What the panel draws is test 17 on the Creator's half. (Hand tests 1, 5 and 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The File Format menu on the node draws exactly the fourteen formats the setting will take, and no more.** In the order shown: AVIF (.avif), JPEG (.jpg), OpenEXR (.exr), PNG (.png), WebP (.webp), then a divider, then Bitmap (.bmp), Cineon (.cin), DPX (.dpx), Iris (.rgb), JPEG 2000 (.jp2), Radiance HDR (.hdr), Targa (.tga), Targa Raw (.tga), TIFF (.tif) — fourteen in all. **OpenEXR MultiLayer and FFmpeg Video are both absent from it**, which is the same as what the setting refused. So what the panel offers and what the setting accepts agree, and the multi-layer choice is made with the Image / Multi-Layer EXR buttons above rather than from this menu. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **The Linux build of this version offers exactly the same formats as the Mac build.** The setting's published list:

```
FILE FORMAT VALUES: ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
MEDIA TYPE VALUES: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
```

and, with the node on Image media type, the same two refusals naming the same fourteen:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

**This narrows the build question without closing it.** The sources say some formats are compiled in and "not available on all systems". Two builds of this version have now been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same fourteen, **AVIF among them on both**. That is two builds, not all builds.

**Operating system:** the format list is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entries 1, 5 and 12.)

---

## 10. What happens when two outputs are aimed at the same name in the same place?

**Answer.** **Not in the sources.** Nothing in the 5.1 manual, API documentation, release notes or Cycles files says what happens when two File Output outputs resolve to the same file path.

Where I looked:

- The File Output page in full (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html). It documents the Output Paths panel, which "Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly" — a way to *see* a collision, but it says nothing about what the collision does.
- `NodeCompositorFileOutputItem` and `NodeCompositorFileOutputItems` in full. There is no uniqueness constraint on `name`; the property is documented only as `name (default "", never None)`. `NodeCompositorFileOutputItems.new(socket_type, name)` states no rule about duplicates.
- The render Output panel (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html) has "Overwrite existing files when rendering" and a placeholder option, and `RenderSettings.use_overwrite` ("Overwrite existing files while rendering (default True)") and `RenderSettings.use_placeholder` exist in the API (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html). These are scene render settings. No equivalent appears on the File Output node or its items, and nothing says the scene ones apply to it.
- Searches across the whole manual and API text for `overwrite`, `duplicate`, `unique name`, `name conflict`, `conflicting`, `collide`, `clash`, `same file name`, `written twice`, `last one wins`. No hit relates to File Output outputs.

**Not covered.** Everything: whether one file overwrites the other, whether a name is made unique, whether an error or warning appears, and whether the order of the outputs decides which survives.

**Outside the interface.** Unknown. Worth flagging that if the answer is "a warning in the interface", that warning may not be visible in a headless run — but the sources do not say there is a warning at all.

**Operating system.** Unknown.


**Observed by hand test (2026-08-02).** **Two outputs on one node cannot carry the same name — the second is renamed.** Both were asked to be called `clash`; the node kept `['clash', 'clash_001']`, and two files were written, each holding its own picture:

```
RESULT [5.1.2] two outputs on one node, both asked to be called 'clash'
    out/file_nameclash.png  830 bytes
    out/file_nameclash_001.png  818 bytes
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
```

**Two separate nodes aimed at the same file give one file, and it holds the first node's picture.** No warning or error was printed, and four renders in a row gave the same result:

```
RESULT [5.1.2] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  830 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
```

Building the two nodes in the opposite order swapped which picture survived, so it follows the order the nodes were made in:

```
RESULT [5.1.2] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  818 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

**A file already on disk is replaced silently, and the scene's Overwrite setting does not reach the node:**

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785689699.296
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1785689699.633
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785689699.972
```

**Placeholders do not reach it either.** A two-frame animation with `render.use_placeholder` on produced the node's files and the scene's files and no placeholder for the node. (Hand tests 7 and 8.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The Output Paths panel flags nothing, because there is nothing to flag — Blender renames the second output before a clash can happen.** With two outputs both asked to be called `clash`, the Images list shows `clash` and `clash_001`, and the Output Paths panel lists two different finished paths, ending `shot_clash.png` and `shot_clash_001.png`. Nothing is coloured red and no warning mark appears. (Hand test 18, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same answers, pixel value for pixel value.**

```
ONE NODE, TWO OUTPUTS BOTH ASKED FOR 'clash' -> node kept ['clash', 'clash_001']
RESULT [5.1.2] two outputs on one node, both asked to be called 'clash'
    out/file_nameclash.png  811 bytes
    out/file_nameclash_001.png  824 bytes
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
RESULT [5.1.2] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  811 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
RESULT [5.1.2] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  824 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

A file already on disk was replaced silently, and switching the scene's Overwrite off changed nothing:

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785995984.648
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1785995984.930
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785995985.140
```

Placeholders did not reach the node there either.

**Which of two writes to one name survives is exactly the kind of thing that could differ between operating systems, and it does not.** Both machines keep the first-made node's picture, four repeats in a row give the same answer on both, and reversing the order of making the nodes reverses the answer on both.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 7 and 8.)


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **The Output Paths panel behaves the same way there, and flags nothing.** It is folded shut when the sidebar is first opened; unfolded, it lists one row for each written output — a small picture icon and the finished path. With the two outputs both asked to be called `clash`, the list of outputs shows `clash` and `clash_001`, and the two rows read

```
/root/smoozy/File...t/shot_clash.png
/root/smoozy/File...ot_clash_001.png
```

with the middle of each path left out by Blender itself, because the panel is narrower than the path. **Nothing is coloured red and no warning mark of any kind appears** — the same as the Creator found on the Mac. There is no File Extensions checkbox in this version's panel; that arrives in 5.2.

*How that picture was got, and why it can be trusted.* That machine's display is covered by a desktop lock screen. Blender does receive a button press through it but never learns where the pointer is, so an ordinary pointer click never operates any control. Clicks aimed at a bare screen position were tried and did nothing, and were abandoned. The panel was unfolded instead by a click sent **inside Blender's own process**, which carries its own position with it and does not go through the display's pointer at all. That this route really works was proved by clicking a checkbox and reading the setting behind it back: `show_options=True` before, `show_options=False` after, and `True` again when it was clicked back. Blender's own screenshot photographs only its own window, so the picture shows Blender's own state and nothing else. (Linux entry 18.)

---

## 11. Can a File Output node write a video file?

**Answer.** The manual describes only two Media Types for this node, Image and Multi-Layer EXR. It does not describe Video for this node. A third Media Type, Video, is documented for the *scene's* Output panel, and the underlying property type does carry a VIDEO value — but no source states that a File Output node can be set to it.

What the File Output page offers:

> "Media Type
>
> Determines how multiple inputs are stored:
>
> Image:
>
> Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

That list has two entries. The page describes the Node Format panel in the same two branches: "This panel controls file format and encoding options, depending on the Media Type: Image: … Multi-Layer EXR: …". No mention of video, encoding, container or codec anywhere on the page.

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented** that only two Media Types are described for this node.

What the scene's Output panel offers, for contrast, is three:

> "Media Type
>
> The type of media to save.
>
> Image: Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file.
>
> Video: Encodes each frame into a video container. Encoding options can be found in the Encoding panel."

- Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30
- Label: **documented**

The property behind both is the same type. The File Output node's `format` is an `ImageFormatSettings` (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html), and that type carries:

> "media_type
>
> The type of media to save (default 'IMAGE')
>
> Type:
>
> Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']"

and its `file_format` enum includes `FFMPEG: FFmpeg Video`.

- Sources: Python API, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html and https://docs.blender.org/api/5.1/bpy_types_enum_items/image_type_all_items.html
- Date read: 2026-07-30
- Label: **documented** that the shared property type has three values including VIDEO.

Joining these: the node's format settings are of a type that has a VIDEO value, while the node's own documentation describes only two. Whether the node restricts the choice is **not in the sources**. I searched the whole manual and API text for `video`, `FFMPEG`, `encoding`, `container` and `codec` in connection with the File Output node and found nothing on its page or in its API entries. Concluding either "yes it can" or "no it cannot" would be a guess, and I am not making it.

**Not covered.** The direct question. Also not covered: what a File Output node would do with a video format across a frame range, given that the node's documented naming pattern appends a frame number per frame.

**Outside the interface.** If the answer is that the node's interface simply does not offer Video, a script setting `media_type = 'VIDEO'` on the node's format might not be blocked in the same way. The sources do not say. This is exactly the kind of gap that only a hands-on check settles — see the list at the end.

**Operating system.** The command-line page states FFMPEG is among the formats "that can be compiled into Blender, not available on all systems" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). Beyond that, the sources do not say.


**Observed by hand test (2026-08-02).** **No. The node refuses the video value**, which settles the outside-the-interface case this answer named. The shared format block carries three media types, but this node will not take the third:

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

The refusal names only Image and Multi-Layer EXR — the same two the manual describes for this node. The node stayed on PNG and a three-frame animation produced three pictures:

```
RESULT [5.1.2] node set to video, animation over frames 1 to 3
    out/file_nameclip0001.png  828 bytes
    out/file_nameclip0002.png  828 bytes
    out/file_nameclip0003.png  828 bytes
```

What the panel offers on screen is test 17 on the Creator's half; the setting behind it will not take the value. (Hand test 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The panel offers no video choice.** Media Type is drawn on this node as two buttons, **Image** and **Multi-Layer EXR**, not as a menu with a third entry — and the File Format menu under it holds no FFmpeg Video (listed under question 9). So the panel and the setting agree, and they agree with the manual describing only two media types for this node: video cannot be chosen here, by hand or from outside. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux refused it too, in the same words.**

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
RESULT [5.1.2] node set to video, animation over frames 1 to 3
    out/file_nameclip0001.png  809 bytes
    out/file_nameclip0002.png  809 bytes
    out/file_nameclip0003.png  809 bytes
```

The refusal names only Image and Multi-Layer EXR there too.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entry 12.)

---

## 12. What can a File Output node be fed, and does it have to come from the render?

**Answer.** It is **documented** that the node can be fed from anywhere in the compositor node tree. It is **inferred** that the input does not have to come from the render; the inference is set out below.

> "Inputs can be added by dragging an output from another node onto the blank socket area"

> "This node can be used to:
>
> Automatically save the final image after a render.
>
> Save intermediate results from anywhere in the node tree.
>
> Export multiple render passes or processed outputs directly to files."

> "Use the File Output node to save raw passes or intermediate steps without overwriting the main render result. Combine with the Cryptomatte Node or other utility passes to export custom data for compositing pipelines."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

Sources other than the render exist as compositor input nodes. The manual's list of Input Nodes is: Constant, Group Input, Bokeh Image, Image, Image Info, Image Coordinates, Mask, Movie Clip, Sequencer Strip Info, and Scene (https://docs.blender.org/manual/en/5.1/compositing/index.html and https://docs.blender.org/manual/en/5.1/compositing/types/input/index.html). The Scene group holds Render Layers, Scene Time and Time Curve, which are separate pages under https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/ rather than entries on either list above. Of these, the Image node is described as:

*Changed by the citation check, 2026-08-02: the contents page cited here lists the input nodes down to "Scene" but does not enumerate the Scene group's three members. The address for those three has been given separately, and Constant moved to the front to match the order on the page.*


> "The Image node injects any image format that is supported by Blender."

and only the Render Layers node is described as coming from a render:

> "Renders a View Layer and reads its Passes into the compositing node graph."

- Sources: manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/image.html and https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html
- Date read: 2026-07-30
- Label: **documented** for what those nodes are. **Inferred** for "so a File Output can be fed from a non-render source": the step is joining "Save intermediate results from anywhere in the node tree" with the existence of input nodes that are not renders.

On the kinds of data. The compositor's own data model is documented:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats: Float … Vector … Color"

and there is an implicit-conversion table between Float, Vector and Color (Float→Vector as `f => Vector(f, f, f, 0)`, Color→Float as `(r, g, b, a) => Luminance(r, g, b)`, and so on).

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** Whether a File Output input that receives a "dimensionless single value" rather than an image writes a file at all, and what such a file would contain. The compositor page says a node input expecting an image treats a single value as covering the whole space ("if node inputs that expect an image are given a single value, the single value will be assumed to cover the whole compositing space"), but it does not say the File Output node is such a node. Also not covered: whether the compositor can run, and therefore whether these nodes write, when no render is performed at all.

**Outside the interface.** The sources do not say. Note the one documented interface-only detail nearby: the Render Layers node has a "button next to the dropdown" that "re-renders it immediately", which is plainly an interface control.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It has to be fed a picture. A plain colour writes nothing** — the "dimensionless single value" case this answer left open. One node with two outputs, one fed by a colour node and one fed by the render through an Invert node, wrote only the second, with no error and with the link reported valid:

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [5.1.2] one output fed by a colour node, one by an image
    out/file_namepicture.png  818 bytes
```

**It does not have to come from the render**, which this answer had as inferred: the picture that was written came through an Invert node rather than straight from the render, and every test on this sheet that used a modified picture wrote normally. (Hand test 13.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux did the same**, and the thing this answer and the second-pass document both left open was settled there for the first time.

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [5.1.2] one output fed by a colour node, one by an image
    out/file_namepicture.png  824 bytes
```

**Which of the twenty-three socket kinds the node will actually accept.** An output was asked for of every one of them. **Only three could be made at all:**

```
OUTPUTS THE NODE ACCEPTED: [('k_FLOAT', 'FLOAT'), ('k_VECTOR', 'VECTOR'), ('k_RGBA', 'RGBA')]
```

Each of the other twenty was refused with the same message, for example:

```
ADDING AN OUTPUT OF KIND MATRIX RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
ADDING AN OUTPUT OF KIND MASK RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
```

All three that were made took the render picture, and all three wrote a file:

```
LINKS THE TREE ACTUALLY HOLDS INTO THE NODE: ['k_FLOAT', 'k_RGBA', 'k_VECTOR']
RESULT [5.1.2] one written output of every kind the node offers, all fed the render picture
    out/file_namek_FLOAT.png  762 bytes
    out/file_namek_RGBA.png  811 bytes
    out/file_namek_VECTOR.png  811 bytes
```

The Float file is smaller than the other two; the Vector and Colour files are the same size as each other. What this does **not** settle: what a Float or a Vector output writes when it is fed something other than a picture, and what channels are actually inside each written file — they were not read back.

**Operating system:** the colour-versus-picture part is now settled on macOS and on Linux, and they agreed; which socket kinds the node accepts is settled on Linux alone. Windows is untested. (Linux entries 13 and 21.)

---

## 13. What causes a File Output node to write, and when during a render does it write each file?

**Answer, what causes it.** The node writes during rendering or compositing. For a render to use the compositing node tree, the compositing pipeline must be enabled.

> "The File Output node writes one or more images to disk during rendering or compositing."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

The compositor only takes part in a render if the pipeline option is on:

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

and in the API:

> "use_compositing
>
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

- Sources: manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html and https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html ; Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- Date read: 2026-07-30
- Label: **documented**

That a File Output node therefore writes nothing when Compositing is switched off is **inferred**: the step is that the node writes "during rendering or compositing", and the pipeline option decides whether the compositing node tree runs during a render at all.

Per frame, the documented naming is a per-frame one:

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence."

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."

- Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30
- Label: **documented**

**Answer, when during a render.** **Not in the sources.** No page states the point within a frame's render, or within an animation, at which a File Output node writes its files — before or after the scene's own output file, after each frame or at the end, before or after denoising, and so on.

Where I looked: the File Output page in full; the compositor Introduction and Compositor System pages in full; Post Processing in full; the render Output panel and the Rendering Animations page (whose nearest statement is about the scene's own output, not this node: "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered"); the command-line rendering pages. Searches across the compositing and render sections for `after each frame`, `per frame`, `end of the render`, `written at`, `flushed`, `when rendering completes` returned nothing relevant.

**Not covered.** Ordering and timing, as above. Also not covered: whether the node writes during interactive or viewport compositing as opposed to a final render. The Compositor System page distinguishes the two cases for image size — "For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size" — and the sidebar's Precision setting distinguishes them again — "Auto: Use full precision for final renders, half precision otherwise" (https://docs.blender.org/manual/en/5.1/compositing/sidebar.html) — but neither says anything about whether files are written in the viewport case. The File Output page's phrase "during rendering or compositing" hints that it might, and I am deliberately not turning that hint into a claim.

**Outside the interface.** The pipeline switch is a scene property (`RenderSettings.use_compositing`), so it carries into a headless render; that much is **inferred** from it being a scene setting rather than an interface state. Everything else about timing is unknown. This is the answer that most needs settling by hand, because a render driven from outside the window is exactly where "when did the file appear" matters.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes when Blender is run from a terminal with no window** — the case this answer said most needed settling. From a saved file:

```
--- blender -b /tmp/fo_scene.blend -f 1
    /tmp/fo/out/file_namebeauty0001.png
    /tmp/fo/out/scene_0001.png
--- blender -b /tmp/fo_scene.blend -a
    /tmp/fo/out/file_namebeauty0001.png
    /tmp/fo/out/file_namebeauty0002.png
    /tmp/fo/out/file_namebeauty0003.png
    /tmp/fo/out/scene_0001.png
    /tmp/fo/out/scene_0002.png
    /tmp/fo/out/scene_0003.png
```

**The frame number is added only on an animation render, and `####` forces it either way:**

```
RESULT [5.1.2] render.render(animation=False)
    out/file_namehash0001here.png  828 bytes
    out/file_nameplain.png  828 bytes
RESULT [5.1.2] render.render(animation=True), frame range 1 to 1
    out/file_namehash0001here.png  828 bytes
    out/file_nameplain0001.png  828 bytes
```

A single frame from the command line (`-f 1`) does gain the number, so the command line behaves as an animation render here.

**Each file is written as its frame finishes, just before the scene's own file for that frame.** Over a four-frame animation with both writing into one folder:

```
    1785690077.335  out/file_namenode0001.png
    1785690077.337  out/scene_0001.png
    1785690078.636  out/file_namenode0002.png
    1785690078.639  out/scene_0002.png
    1785690079.989  out/file_namenode0003.png
    1785690079.991  out/scene_0003.png
    1785690081.286  out/file_namenode0004.png
    1785690081.288  out/scene_0004.png
```

Frame 1's files carry a time about four seconds before frame 4's, so nothing waited for the end of the run; and within each frame the node's file is a couple of thousandths of a second ahead of the scene's.

**Switching compositing off stops it; having no Group Output node in the tree does not:**

```
RESULT [5.1.2] the same tree with render.use_compositing = False
    (nothing written)
RESULT [5.1.2] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  818 bytes
```

The first of those confirms the inference above about the compositing pipeline switch. Whether a single frame rendered from the window writes, and whether viewport compositing writes, are tests 19 and 20 on the Creator's half. (Hand tests 2, 3, 13 and 15.)


**Observed by hand test, from Blender's own window (2026-08-02).** **One frame rendered from the window does write.** Pressing F12 once and closing the render window without saving the render left four files, one for each connected output of the two nodes in the file:

```
shot_clash.png
shot_clash_001.png
shot_one.png
shot_two.png
```

So a File Output node does not need the render to be saved by hand. Note that none of the four names carries a frame number, which matches the rule above: a single-frame render from the window is not an animation render.

**The viewport compositor writes nothing.** With compositing switched on in the 3D viewport and left running for about thirty seconds with no render started, the folder stayed empty. So "during rendering or compositing" does not extend to the viewport. (Hand tests 19 and 20, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on every part, and one thing the Mac could not reach was settled here.**

**It writes when Blender is run from a terminal with no window**, from a saved file. `-f 1` and `-a` both wrote, and the node's file was named before the scene's own for the same frame:

```
--- blender -b fo_scene.blend -f 1  (Blender 5.1)
00:01.026  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0001.png'
00:01.027  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
--- blender -b fo_scene.blend -a  (Blender 5.1)
00:15.882  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0001.png'
00:15.882  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
00:15.952  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0002.png'
00:15.953  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0002.png'
00:16.022  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0003.png'
00:16.022  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0003.png'
```

**The frame number is added only on an animation render, with `####` forcing it either way:**

```
RESULT [5.1.2] render.render(animation=False)
    out/file_namehash0001here.png  809 bytes
    out/file_nameplain.png  809 bytes
RESULT [5.1.2] render.render(animation=True), frame range 1 to 1
    out/file_namehash0001here.png  809 bytes
    out/file_nameplain0001.png  809 bytes
```

**Each file is written as its frame finishes, just before the scene's own file for that frame:**

```
FILES IN THE ORDER THEY WERE LAST WRITTEN (seconds since the epoch):
    1785996160.466  out/file_namenode0001.png
    1785996160.472  out/scene_0001.png
    1785996161.860  out/file_namenode0002.png
    1785996161.864  out/scene_0002.png
    1785996162.823  out/file_namenode0003.png
    1785996162.833  out/scene_0003.png
    1785996164.020  out/file_namenode0004.png
    1785996164.028  out/scene_0004.png
```

That machine is shared with other work, so no claim is made about how long any of it took; what the test shows is the **order**, and the order is the same on both machines.

**Switching compositing off stops it; having no Group Output node in the tree does not:**

```
RESULT [5.1.2] the same tree with render.use_compositing = False
    (nothing written)
RESULT [5.1.2] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  824 bytes
```

**What a stopped render leaves behind, which was never tested on the Mac.** A sixty-frame animation was started from the command line and stopped part way, twice: once with an interrupt — the polite stop, what pressing Ctrl-C does — and once with a hard kill that gives Blender no chance to tidy up. **Both left whole files for every frame that had finished, and nothing at all for the frame that was in flight.** No part-written file, no empty file, no file of the wrong size, and the node's files and the scene's files always in matching pairs. Stopped with an interrupt it left twenty-two pairs, frames 0001 to 0022, every node file 8544 bytes and every scene file 10924 bytes, with frame 23 — the one being rendered — leaving nothing. Stopped with a hard kill it left seventeen pairs, again all whole, with frame 18 leaving nothing. The two counts differ only because the machine got through different numbers of frames before the stop arrived; that machine is shared, so the counts are not a measure of its speed.

Whether a single frame rendered from the window writes, and whether viewport compositing writes, were **not** run on Linux — those two need the machine's screen to themselves, and it is shared. They are answered for macOS by the Creator's run above, and are written under *Cannot be run* in the Linux hand-test list.

**Operating system:** the terminal half of this answer is now settled on macOS and on Linux, and they agreed; what a stopped render leaves behind is settled on Linux alone. The window half is settled on macOS alone. Windows is untested throughout. (Linux entries 2, 3, 13, 15 and 22.)

---

## What is still left for a person with Blender open

The list that stood here has been worked through, and nothing on it is outstanding. The tests a terminal could reach were run on 2026-08-02 and the four that needed eyes on the window were run by the Creator the same day; both kinds are written into the answers above as observed findings, and the numbered tests are in `Audits/Blender Research/File output nodes/Hand tests.md`.

**Then a second machine arrived.** On 2026-08-06 the whole list was run again on a rented Linux machine, and two behaviours the Mac could never reach were run for the first time: which kinds of input socket the node will actually accept and what each writes, and what a stopped render leaves behind. Those results are in the answers above too, marked as coming from that machine, and the entries are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`. Linux agreed with the Mac on every test both machines ran; the one place their words differ is the operating system's own description of a failed write, recorded under question 5.

**Cannot be run at all.** Four things, and the reasons are written out in the *Cannot be run* sections of both hand-test lists:

- **A folder reached over a network**, in any version. Neither machine has a network store to point at, and setting one up on the rented machine would mean installing something and changing a system setting.
- **Anything about Windows.** No Windows machine has run it. macOS and Linux are both covered; Windows is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **A Blender built by somebody else.** Two builds of this version have been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same file formats. That is two builds, not all builds.
- **A frame rendered from Blender's own window on Linux, and viewport compositing on Linux.** That machine's screen is shared with several other topics at once, and those two tests would have to hold it for their whole length. They are answered for macOS by the Creator's run, and are on the *Needs the machine to itself* list for later.

2026-08-02 — Opus 5 (Claude Code) — File output nodes, Blender 5.1

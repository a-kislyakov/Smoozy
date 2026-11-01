# File output nodes — Blender 5.2

**Sources consulted, and when.** Blender's 5.2 manual (plain-text copy plus one picture from its `_images` folder), Blender's 5.2 Python API documentation, and the Blender 5.2 release notes — all from the snapshot taken 2026-07-30, and all read by me on 2026-08-02. I also searched Cycles' own files for this version and found nothing on this subject.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note on how I looked: the manual and the API documentation were searched whole, by word, before any file was opened. Where an answer is "not in the sources", the words I searched for are listed with it.

## The pages I quote, and their addresses

Every quotation below comes from one of these. I name the page in each answer; the address is here. All were read on 2026-08-02, from the 2026-07-30 snapshot.

**Manual**

- File Output page — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html
- Picture on that page — `_images/node-types_CompositorNodeOutputFile.webp`, shown on the File Output page above
- Render Output panel — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Rendering Animations — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Post Processing panel — https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html
- File Paths (path templates) — https://docs.blender.org/manual/en/5.2/files/file_paths.html
- Opening and Saving Files (relative paths) — https://docs.blender.org/manual/en/5.2/files/blend/open_save.html
- Supported Image Formats / Images Settings — https://docs.blender.org/manual/en/5.2/files/media/image_formats.html
- Node Groups — https://docs.blender.org/manual/en/5.2/interface/controls/nodes/groups.html
- Editing Nodes (mute) — https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html
- Compositor System — https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html
- Compositing Introduction — https://docs.blender.org/manual/en/5.2/compositing/introduction.html
- Using The Compositor — https://docs.blender.org/manual/en/5.2/compositing/usage.html
- Output Nodes index — https://docs.blender.org/manual/en/5.2/compositing/types/output/index.html
- Enable Output Node — https://docs.blender.org/manual/en/5.2/compositing/types/output/enable_output.html
- Render Layers Node — https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html
- Command Line Arguments — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html

**Python API documentation**

- CompositorNodeOutputFile — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html
- NodeCompositorFileOutputItem — https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html
- NodeCompositorFileOutputItems — https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItems.html
- ImageFormatSettings — https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html
- RenderSettings — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Node — https://docs.blender.org/api/5.2/bpy.types.Node.html
- Application Handlers — https://docs.blender.org/api/5.2/bpy.app.handlers.html
- Node Socket Data Type Items — https://docs.blender.org/api/5.2/bpy_types_enum_items/node_socket_data_type_items.html
- Change Log — https://docs.blender.org/api/5.2/change_log.html

**Release notes**

- Compositor — https://developer.blender.org/docs/release_notes/5.2/compositor/
- Bugfixes — https://developer.blender.org/docs/release_notes/5.2/bugfixes/

**Cycles' own files** — searched, and silent on this subject. See the note at the end.

---

## Observed by hand test

Everything under this heading was settled by running Blender 5.2.0 LTS on the Creator's own computer — one MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2 — on **2026-08-02**. Blender was started from a terminal with no window, in the form

`"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup --python /tmp/run.py`

and driven through its Python support. The exact scripts, and which test number each result came from, are in `Audits/Blender Research/File output nodes/Hand tests.md`. The machine's own output is quoted below word for word; where a listing is long, the part that carries the answer is kept and the rest is said to be omitted.

A few of the blocks below are marked **from Blender's own window** instead. Those were run by the Creator on the same computer on the same day, by opening a prepared file, reading the node's panel, and rendering from the window. They are the tests that needed eyes on the screen.

---

## Observed by hand test on a second machine

On **2026-08-06** the same tests were run again on a second computer: **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; its screen is shared with other work. Blender 5.2.0 LTS there is the Linux x64 build from Blender's own download server, at `/opt/blender/blender-5.2.0-linux-x64/blender`, and it was started the same way:

`/opt/blender/blender-5.2.0-linux-x64/blender -b --factory-startup --python run.py`

with `BLENDER_USER_RESOURCES` pointed at a settings folder of its own, and with `DISPLAY` and `XAUTHORITY` unset so that the display's own `Authorization required` noise stayed out of the output. The scripts were the Mac's, word for word, with only the folder they write into changed. Two of them needed Blender's window, which was opened on that machine's own display and photographed with Blender's own screenshot command. The entries and the full output are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

Every question below therefore carries results from two machines. **A Mac result and a Linux result are two results and are never merged.** Where they agreed, both are named and the agreement is said plainly; where the machines differed, both are given and neither is chosen.

**These blocks update the two standing lines for every question they touch.** *Driven from outside*: every result not marked as coming from the window was produced with Blender driven from outside its own interface, with no window open, so each holds in that case; where a window result differs from the terminal one, both are given. That is now true on both machines. *Operating system*: every answer that carries both an *Observed by hand test* block and an *on the rented Linux machine* block below is settled on **macOS 26.5.2 on an Apple M1 Pro and on Ubuntu 22.04.5 LTS on this rented machine**, and on nothing else. **Windows was not tested at all**: no Windows machine has run it. Both hand-test lists record it under *Cannot be run*.

---

## 1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?

**Answer.** It writes images to disk. It starts with no named input sockets; inputs are added by the user, and each input corresponds to an image written separately or to a layer in one multi-layer file. The sources state no maximum number.

The manual's File Output page says:

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

On how many, and how they appear:

> "By default, the node has no input sockets. Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel."

> "Each input corresponds to an image that will be written to disk."

The picture on that page (`_images/node-types_CompositorNodeOutputFile.webp`, shown at the top of the File Output page) shows the node as it ships: a folder field reading `/tmp/`, a name field reading `file_name`, and a single hollow (unconnected) socket on the left — no named inputs. That much is **documented**, since the picture is part of the manual; anything about how the node behaves that I might read into the picture would be inferred, and I have not done so.

What each output carries depends on the node's Media Type, still on the File Output page:

> "Image: Saves each input as a separate image file. Each input can use its own image format settings."

> "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

> "When saving individual images, each output can be renamed and will be written to disk using either the global Node Format settings or its own overridden settings. When using Multi-Layer EXR, each input is stored as a separate layer within the file."

The API documentation's NodeCompositorFileOutputItem page gives each output a declared kind:

> "socket_type ... (default 'FLOAT')"

and the NodeCompositorFileOutputItems page shows the collection of outputs is open-ended — it offers `new`, `remove`, `clear` and `move` — while the CompositorNodeOutputFile page gives:

> "active_item_index — Index of the active item (in [0, inf], default 0)"

**Label:** documented (what it writes, how outputs are added, what each carries). **Inferred** for "no stated maximum": the sources place no upper bound on the index and state no limit, so I take it that none is documented — that is not the same as there being none.

**Not covered.** No number is given for the largest workable count. Nothing says what happens if an added input is left unconnected — whether it is skipped or written as something empty. The default socket kind is stated as Float, but the sources do not say what an output set to a non-image kind actually produces in a file.

**Outside the interface.** The sources do not say. Nothing on this page distinguishes a person clicking from Blender being driven from outside. The API documentation exposes the node's folder, name, format, extension flag and its list of outputs as ordinary readable and writable properties, and the collection can be added to, removed from, cleared and reordered — so the node can be built and altered without a person clicking. Whether it *writes* the same way in that situation is question 13.

**Operating system.** The sources do not say. Nothing on the File Output page or in the API entries names an operating system. See question 9 for the one place the sources do tie file formats to the build.


**Observed by hand test (2026-08-02).** **The node has nothing on its output side** — `NODE HAS OUTPUT SOCKETS: []` — and the old pair of slot collections is gone: `TYPE NodeOutputFileSlotFile present=False`, `TYPE NodeOutputFileSlotLayer present=False`, `TYPE NodeCompositorFileOutputItem present=True`.

**A new node does not start on the same format as the scene**, which this answer said was not stated anywhere. `A NEW NODE STARTS ON: file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE` while `THE SCENE STARTS ON: file_format=PNG media_type=IMAGE`. A File Output node added and left alone therefore writes one multi-layer OpenEXR file rather than separate pictures.

**Which socket kinds an output may be set to**, read from the running program:

```
    socket_type | ENUM | description='' values=['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE', 'FONT', 'SCENE', 'TEXT', 'MASK', 'SOUND', 'INT_VECTOR']
```

**No ceiling was reached on how many outputs a node takes**: `OUTPUTS ADDED WITHOUT REFUSAL: 2000` and `INPUT SOCKETS ON THE NODE NOW: 2001` — two thousand outputs plus the blank socket — with nothing refused. That establishes at least two thousand and says nothing above it. (Hand tests 1 and 14.)


**Observed by hand test, from Blender's own window (2026-08-02).** Outputs are added and removed with a plus and a minus beside the list in the **Images** panel, with a pair of arrows for reordering, and the kind of thing an output carries is drawn as a **Socket Type** dropdown below the list. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed with the Mac on every part.** `NODE HAS OUTPUT SOCKETS: []`; `TYPE NodeOutputFileSlotFile present=False`, `TYPE NodeOutputFileSlotLayer present=False`, `TYPE NodeCompositorFileOutputItem present=True`; and `A NEW NODE STARTS ON: file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE` while `THE SCENE STARTS ON: file_format=PNG media_type=IMAGE`. The same twenty-four socket kinds were published:

```
    socket_type | ENUM | description='' values=['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE', 'FONT', 'SCENE', 'TEXT', 'MASK', 'SOUND', 'INT_VECTOR']
```

and the same ceiling was not reached: `OUTPUTS ADDED WITHOUT REFUSAL: 2000`, `INPUT SOCKETS ON THE NODE NOW: 2001`, with nothing refused. **Which of those twenty-four the node will actually accept was tested on Linux for the first time; it is three of them, and that is under question 12.**

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 1, 14 and 21.)

---

## 2. Which settings belong to an individual output, and which belong to the node as a whole?

**Answer.** The API documentation lists them plainly.

Belonging to the node as a whole (from `CompositorNodeOutputFile`):

> "directory — The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name — The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

> "format — (readonly) Type: ImageFormatSettings | None"

> "save_as_render — Apply render part of display transform when saving byte image (default False)"

> "use_file_extension — Add the file format extensions to the saved file name (eg: filename + .jpg) (default False)"

> "active_item_index — Index of the active item (in [0, inf], default 0)"

> "file_output_items — (default None, readonly)"

Belonging to an individual output (from `NodeCompositorFileOutputItem`):

> "color — Color of the corresponding socket type in the node editor (array of 4 items, in [0, inf], default (0.0, 0.0, 0.0, 0.0), readonly)"

> "format — (readonly) Type: ImageFormatSettings | None"

> "name — (default "", never None)"

> "override_node_format — Use a different format instead of the node format for this file (default False)"

> "save_as_render — Apply render part of display transform when saving byte image (default False)"

> "socket_type — (default 'FLOAT')"

> "vector_socket_dimensions — Dimensions of the vector socket (in [2, 4], default 0)"

The manual's File Output page describes the same split in words:

> "Node Format ... Image: Controls the default format and encoding for all outputs. To override settings for an individual output, enable Override Node Format in the Images/Layers panel."

> "Override Node Format — Allows the selected output to use a custom format instead of the global Node Format."

> "Item Format — When Override Node Format is enabled, this panel controls the format and encoding options for the selected output image."

The folder and the base name are listed by the manual under the node's own Properties, and there is no folder or base name on an individual output.

**Label:** documented.

**Not covered.** One thing I cannot settle: the manual lists Media Type among the node's own properties, but the API documentation puts `media_type` on the format settings block — and each individual output has its own format settings block too. So whether an individual output could be given a *different* media type from the node is not settled by the sources; the manual reads as though media type is node-wide. Also, `save_as_render` appears at both levels in the API documentation but is not mentioned on the manual's File Output page at all, so which one wins when they disagree is not stated.

**Outside the interface.** Every one of these is a named, readable property in the API documentation, so the same split holds when the node is built from outside. Two of them (`format` on the node, `format` on the item) are marked read-only, meaning the block itself cannot be swapped, though the sources do not say its contents cannot be changed. The sources do not say anything differs.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **The full list of settings, from the running program**, which confirms the answer above and settles one thing it could not. On the node: `file_output_items`, `active_item_index`, `directory`, `file_name`, `format` (read-only), `save_as_render`, and `use_file_extension`. On one written output: `name`, `socket_type`, `vector_socket_dimensions`, `color`, `override_node_format`, `save_as_render` and `format`.

**An output cannot be given a different media type from the node** — the thing this answer said the sources did not settle. With the node's media type on Image, its format setting refuses the multi-layer value outright, and the media type is what decides which formats are offered at all (quoted in full under question 9). The two are not independent settings that could disagree.

**Which display-transform setting wins when the node and an output disagree.** Eight combinations, one at a time, with the written pixel read back:

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

While Override Node Format is off, only the node's `save_as_render` changes the file; while it is on, only the output's own does. (The values in the two halves are not on the same scale, because the node's format block and the output's own were not left carrying identical settings; what the test shows is which flag moves the value.) (Hand tests 1 and 16.)


**Observed by hand test, from Blender's own window (2026-08-02).** **Every control the panel draws, with a File Output node selected and the sidebar open on the Node tab.** Under **Node**: Reset Nodes, Name, Label, Color, **Show Options**, **Mute** and **Propagate**. Under **Properties**, belonging to the node: the folder field with a folder-browse button, the file-name field, then a two-button choice reading **Image** | **Multi-Layer EXR**, then a **Node Format** panel holding Save as Render, File Format, Color, Color Depth, Compression and a Color Management panel (Follow Scene or Override; Override adds Display, View, Look, Exposure, Gamma, Use Curves with its curve editor, and Use White Balance with Temperature and Tint). Then an **Images** panel holding the list of outputs. Belonging to the selected output: **Socket Type**, **Override Node Format**, and an **Item Format** panel repeating the same format and colour-management controls. Then an **Output Paths** panel listing the finished path of each output, with a **File Extensions** checkbox above the list. Finally **Custom Properties**.

Neither the folder field nor the file-name field carries a visible label beside it; the manual's names Directory and File Name are not drawn on the panel.

**The Override Node Format tooltip reads:** "Use a different format instead of the node format for this file." That is word for word what the API reference gives, so the two agree. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux printed the same list of settings, including this version's new one, and the same eight pixel values.** On the node: `file_output_items`, `active_item_index`, `directory`, `file_name`, `format` (read-only), `save_as_render`, and

```
    use_file_extension | BOOLEAN | read-only=False | description='Add the file format extensions to the saved file name (eg: filename + .jpg)'
```

On one written output: `name`, `socket_type`, `vector_socket_dimensions`, `color`, `override_node_format`, `save_as_render` and `format`. The display-transform test:

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


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **What the node draws on its own face.** The node's body carries a header with the node's label, then two fields one above the other — the folder, and the base file name — then one row for each written output showing the output's name alone, and finally a row holding a `+`, which is the blank socket a new input is dragged onto. With outputs called `one` and `two` the rows read `one` and `two`. Neither of the two fields carries a written label beside it, which agrees with what the Creator found on the Mac's sidebar.

The sidebar's **Node** panel in this version also carries **Show Options**, **Mute** and **Propagate**, which the Creator's Mac reading names too. Turning Show Options off takes the two path fields off the node's face, leaving the header and the output rows. The Creator's Mac screenshots were of the sidebar rather than the node's face, so the rest of this is the Linux picture and nothing more. (Linux entry 17.)

---

## 3. Does each output carry its own folder as well as its own name?

**Answer.** No. An output carries a name only. The folder is a single setting on the node.

The API documentation's CompositorNodeOutputFile page lists exactly one folder, and it is on the node:

> "directory — The directory where the image will be written"

The list of properties on an individual output — the NodeCompositorFileOutputItem page, quoted in full under question 2 — contains `name` but nothing resembling a folder.

The manual's File Output page agrees:

> "Directory — The base output directory."

> "Images/Layers ... Lists all input sockets of the node. The name of each input socket is used to construct the final file name."

**Label:** documented for "the node has one folder and an output has a name"; **inferred** for the flat "no" — the step is that the sources list an individual output's settings exhaustively and no folder is among them, so an output has none of its own. A reader has to accept that the list is exhaustive.

**Not covered.** This says nothing about folders reached *through* a name — see question 4, where a name can in fact carry folder structure.

**Outside the interface.** Same answer: the API documentation is where the split is stated, and it is not a description of the interface.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Confirmed by running it, with one addition.** A name can put folders under the node's own folder, and a name reaching *upwards* is stopped: `../escape` was not kept as typed — `ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'` — and the file landed inside the node's folder as `base/file_name_./escape.png`, not outside it. A name beginning with a slash is joined under the node's folder too, as `base/file_name/absolute.png`. So the folder on the node is a floor the outputs cannot climb out of. (Hand test 4.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux stopped the upward climb in exactly the same way.** `ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'`, and the file landed inside the node's folder as `out/base/file_name_./escape.png`, not outside it. A name beginning with a slash was joined under the node's folder too, as `out/base/file_name/absolute.png`.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed — including the part most likely to differ, a name trying to climb out of its folder. Windows is untested. (Linux entry 4.)

---

## 4. Can a folder be expressed inside an output's name rather than separately?

**Answer.** Yes, when the node is set to write separate images. The manual's File Output page states it directly:

> "When the Media Type is set to Image, input names are also used to construct subdirectories:"

> "If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy."

> "Any missing directories are created automatically during output."

> "This allows organizing outputs into folders directly from the node setup, for example: passes/diffuse or masks/object_01."

**Label:** documented.

**Not covered.** The condition attached is "When the Media Type is set to Image" — the sources do not say what a slash in a name does when the node is set to Multi-Layer EXR (where names become layers inside one file, not separate files). Nothing says whether a name may climb *upwards* out of the folder, or whether an absolute path in a name is honoured. Nothing says what happens if the folders cannot be created.

**Outside the interface.** The sources do not say. The statement is about how the name is interpreted at output time, not about the interface, but nothing states it explicitly either way.

**Operating system.** Both slash characters are named — forward slash and backslash — and the manual does not tie either to a particular system. It does not say whether both work everywhere or whether each is meant for its own system. That gap has since been closed by hand on two machines — see the observed blocks below.


**Observed by hand test (2026-08-02).** **Confirmed by running it, and all four things this answer left open are now known.** With the node pointed at `/tmp/fo/out/base`:

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
RESULT [5.2.0 LTS] separators in output names, Image mode, node folder /tmp/fo/out/base
    base/file_name/absolute.png  902 bytes
    base/file_name_./escape.png  902 bytes
    base/file_namea/b/c/deep.png  902 bytes
    base/file_namesub/shot.png  902 bytes
```

Folders are made, several levels deep where asked, and the node's File Name is joined to the front of the name before the split, so `sub/shot` under a File Name of `file_name` gives a folder called `file_namesub`. **A name cannot climb upwards**: `..` was turned into `_.` before the node kept it. **A leading slash does not make the path absolute.** **A backslash does the same thing as a forward slash** — a node whose only output was named `sub\shot` produced `base2/file_namesub/shot.png`.

**In Multi-Layer EXR mode the slash is kept in the layer's name and no folder is made** — the case the manual's "When the Media Type is set to Image" condition leaves open. One file was written, and its header text contains `'passes/diffuse.A', 'passes/diffuse.B', 'passes/diffuse.G', 'passes/diffuse.R'`:

```
RESULT [5.2.0 LTS] a separator in an output name, Multi-Layer EXR mode, node folder /tmp/fo/out/base/
    base/file_name.exr  10158 bytes
```

Nothing was reported to the terminal in any of these cases. What happens when a folder cannot be made is under question 5. (Hand tests 4 and 5.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux kept the same names and made the same folders.**

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '_./escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
RESULT [5.2.0 LTS] separators in output names, Image mode, node folder /root/smoozy/File output nodes/work/fo/out/base
    out/base/file_name/absolute.png  906 bytes
    out/base/file_name_./escape.png  906 bytes
    out/base/file_namea/b/c/deep.png  906 bytes
    out/base/file_namesub/shot.png  906 bytes
```

A node whose only output was named with a backslash wrote `out/base2/file_namesub/shot.png`. In Multi-Layer EXR mode the slash was kept in the layer's name and no folder was made:

```
RESULT [5.2.0 LTS] a separator in an output name, Multi-Layer EXR mode, node folder /root/smoozy/File output nodes/work/fo/out/base/
    out/base/file_name.exr  10158 bytes
```

whose header text carries `'passes/diffuse.A', 'passes/diffuse.B', 'passes/diffuse.G', 'passes/diffuse.R'`.

**This is the sharpest operating-system question on the sheet, and both machines answer it the same way.** A backslash is a folder separator on Windows and an ordinary character in a file name on macOS and Linux; Blender treats it as a folder separator on both machines tested. Windows itself is untested.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 4 and 5.)

---

## 5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?

**Answer, in two parts.**

*Folders unrelated to where the Blender file sits: inferred as possible, but not fully established.* The folder is a text path, and the API documentation explicitly supports a blend-relative form marked by a prefix. Reading that relative form as optional, and therefore reading an unprefixed path as able to point elsewhere, is a step beyond the quoted words. From the API documentation's CompositorNodeOutputFile page:

> "directory — The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

The manual's Opening and Saving Files page explains that prefix generally:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

The folder can also be written using substituted values, and the manual's File Paths page says the File Output node is one of only two places where that works:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

The variables available there include ones that name places outside the current file's folder:

> "blend_dir: Path up to (but not including) the currently open blend file."

and one that exists only on nodes:

> "Available only in path properties on a node: node_name: The name of the node that the path property is on."

Bad substitutions stop the path being used:

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

*Folders reached over a network: not in the sources, for this node.* I searched the whole manual and the whole API documentation for: UNC, network path, network drive, network share, mapped drive, samba, SMB, NFS, and a doubled backslash. Nothing connects any of these to the File Output node, or to output paths generally. The only nearby mentions are: the File Browser's side list ("Drives and network mounts."), a note about installing Blender itself on a "read-only network drive", an extensions-repository example using `file://HOST/share/...` "for network shares on Windows", and one render-farm hint on the render Output panel which says to "Set up a shared directory over a network file system." — that hint is about the scene's own render output path and its Overwrite and Placeholders settings, not about the File Output node.

**Label:** documented for blend-relative folders and substituted values; **inferred** for folders unrelated to the Blender file; **not in the sources** for network-reached folders.

**Not covered.** Nothing states whether the folder must already exist (the automatic-creation sentence quoted in question 4 is attached to slashes inside output *names*, not to the folder field). Nothing states what happens when the folder cannot be written to. Nothing states a maximum path length, or which characters are rejected.

**Outside the interface.** The relative prefix is described as a text convention as well as a checkbox — "when entering the path into a text field, use a double slash prefix (//)" — so it is not tied to the file browser. The one caution that is tied to state rather than interface is that a relative path needs the Blender file to have been saved. The substitution feature is described as happening "internally at time of use (e.g. while rendering)", not at the moment of typing, which is the same whoever set the value.

**Operating system.** The sources do not say for this node. The only operating system named anywhere near this subject is Windows, in the extensions-repository example, which is not about File Output.


**Observed by hand test (2026-08-02).** **A folder unrelated to where the Blender file sits is accepted**, which settles the part this answer had as inferred: every test on this sheet wrote into `/tmp/fo/out`, which has nothing to do with the file's own folder, and did so with the file unsaved.

**A folder that does not exist is made**, which this answer said nothing stated:

```
FOLDER /tmp/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [5.2.0 LTS] node folder that did not exist
    out/does/not/exist/yet/file_namemade.png  902 bytes
```

**The `//` form works and is relative to the saved file:**

```
RESULT [5.2.0 LTS] node folder set to the text //relative/ with the file saved at /tmp/fo/out/held.blend
    out/held.blend  97277 bytes
    out/relative/file_namemade.png  914 bytes
```

**A folder that cannot be written to fails loudly and writes nothing** — the other thing this answer said was not stated:

```
00:01.634  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/fo/out/locked/file_namemade.png"
00:01.634  reports          | ERROR Render error (Undefined error: 0) cannot save: '/tmp/fo/out/locked/file_namemade.png'
RESULT [5.2.0 LTS] node folder that cannot be written to (mode 500)
    (nothing written)
```

Network folders were not tested and could not be — see *Cannot be run* in the hand-test list. (Hand test 11.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts, and this test had to be run twice there.**

A folder that does not exist is made:

```
FOLDER /root/smoozy/File output nodes/work/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [5.2.0 LTS] node folder that did not exist
    out/does/not/exist/yet/file_namemade.png  906 bytes
```

The `//` form is relative to the saved file:

```
RESULT [5.2.0 LTS] node folder set to the text //relative/ with the file saved at /root/smoozy/File output nodes/work/fo/out/held.blend
    out/held.blend  97316 bytes
    out/relative/file_namemade.png  949 bytes
```

**Why the unwritable-folder test was run twice.** The account running Blender on that machine is the root user, and root walks through file permissions. Run as plain root, the folder set to permissions 500 was written to anyway:

```
RESULT [5.2.0 LTS] node folder that cannot be written to (mode 500)
    out/locked/file_namemade.png  949 bytes
```

Run again with that power dropped for the one command, which is the real answer, it failed loudly and wrote nothing, as on the Mac:

```
00:02.479  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/smoozy/File output nodes/work/fo/out/locked/file_namemade.png"
00:02.479  reports          | ERROR Render error (Success) cannot save: '/root/smoozy/File output nodes/work/fo/out/locked/file_namemade.png'
RESULT [5.2.0 LTS] node folder that cannot be written to (mode 500)
    (nothing written)
```

**The words differ between the machines and the behaviour does not.** The Mac said `Render error (Undefined error: 0)`; Linux says `Render error (Success)`. The words in the brackets are the operating system's own description of what went wrong, and the two systems describe it differently. What Blender did — refuse loudly and write nothing — is the same on both.

Network folders still could not be tested on either machine — see *Cannot be run* in both hand-test lists.

**Operating system:** this is the question the answer above calls a real operating-system boundary. The parts of it that could be tested are now settled on macOS and on Linux, and they agreed on what happens, differing only in the wording of the failure message. Windows is untested, and network folders are untested everywhere. (Linux entry 11.)

---

## 6. Can several outputs be gathered into one file rather than written separately?

**Answer.** Yes — that is what the node's Multi-Layer EXR media type does. From the manual's File Output page:

> "Media Type — Determines how multiple inputs are stored:"

> "Image: Saves each input as a separate image file. Each input can use its own image format settings."

> "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

> "When using Multi-Layer EXR, each input is stored as a separate layer within the file."

The manual's Supported Image Formats page supports it:

> "An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

The 5.2 release notes record a fix in this area:

> "Rendering File Output Anamorph with Mulit-Layer EXR cannot save file"

(listed among the issues fixed in 5.2).

**Label:** documented.

**Not covered.** The sources describe only this format as a way to combine outputs; they do not establish that no other way exists. Nothing says whether the per-output override of format still applies when everything goes into one file; the manual's Node Format section says only "Multi-Layer EXR: See OpenEXR for EXR-specific settings", which reads as node-wide. Nothing says what the layers inside the file are called beyond "each input is stored as a separate layer".

**Outside the interface.** The setting is an ordinary named value on the format block — the API documentation's ImageFormatSettings page gives "media_type — The type of media to save (default 'IMAGE')" — so it can be set from outside. The sources do not say the behaviour differs.

**Operating system.** The sources do not say directly, but see question 9: the multi-layer OpenEXR format is one of the formats the manual lists as possibly absent from a given build.


**Observed by hand test (2026-08-02).** **The layers are named after the outputs**, which this answer said was not stated. Three outputs named `first`, `second` and `third` gave one file:

```
RESULT [5.2.0 LTS] three outputs, Multi-Layer EXR mode
    out/file_name.exr  14994 bytes
```

whose header text carries `'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R'`, then the same for `second` and `third`, each with its own `'name', 'string', 'first'` and so on (the rest of the header text is omitted).

**The single file takes its name from the node's File Name alone**, with no output name in it — it was `out/file_name.exr` whether the folder was written with a trailing slash or without one.

**The media type is what switches the node between the two ways of writing.** With the media type on Image, the node refused the multi-layer format outright:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

(Hand tests 5 and 6.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts.**

```
RESULT [5.2.0 LTS] three outputs, Multi-Layer EXR mode
    out/file_name.exr  14994 bytes
```

whose header text carries `'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R'`, then the same for `second` and `third` (the rest omitted). The single file was `out/file_name.exr` whether the folder was written with a trailing slash or without one. And with the media type on Image the node refused the multi-layer format outright, in the same words as on the Mac.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 5 and 6.)

---

## 7. What happens to a File Output node inside a group of nodes?

**Answer: not in the sources.** Nothing states what a File Output node does when it sits inside a node group.

Where I looked, and what I found instead:

- The manual's node-group page carries a general caution that names a different node, not this one: "Avoid using output nodes such as Material Output inside node groups. These should be used on the top level node tree to improve re-usability of node groups." and "Use Group Output to pass data out of a node group." That is advice about output nodes in general, with a shader example. It does not say what happens if you ignore it, and it does not name the File Output node.
- The manual's compositor system page describes a rule about which output *is used*, but it is about a different set of nodes: "The compositor only supports a single active output target, that is, only one of the Group Output nodes or Viewer nodes in the node tree will be considered active and the rest will be ignored. In particular, the compositor searches the Active Node Tree Context and falls back to the Root Node Tree Context if no active output was found in the active node tree context. The active node tree context is the node tree of an expanded node group, that is, when the user selects a node group node and edits its underlying tree, while the root node tree context is the top level node tree without any expanded node groups." File Output nodes are not named in that rule, so I will not extend it to them.
- The File Output page itself says nothing about groups.
- Searches run: "File Output" across the whole manual (seven files, all read); "node group" across the whole compositing section and across the release notes; "group" on the File Output page.

**Label:** not in the sources.

**Not covered.** Everything about this question, including the related and equally unanswered one of what happens to a File Output node inside a compositor node group used as a Video Sequencer strip or modifier — the manual describes those as using "compositor node groups" and says "Both compositor strips and modifiers use compositor node groups" and "The first node group output socket must use the Color type", but says nothing about file writing from within them.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes, once, wherever it sits.** A File Output node placed inside a node group wrote its file; using the same group twice still produced one file; and putting that group inside a second group made no difference:

```
RESULT [5.2.0 LTS] one File Output node inside a group, the group used once
    out/g_grouped.png  902 bytes
RESULT [5.2.0 LTS] the same group used twice in the tree
    out/g_grouped.png  902 bytes
RESULT [5.2.0 LTS] the File Output node two groups deep
    out/g_grouped.png  902 bytes
```

Muting the group node from outside stopped it:

```
RESULT [5.2.0 LTS] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases. Note what the second case does not settle: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like. (Hand test 10.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [5.2.0 LTS] one File Output node inside a group, the group used once
    out/g_grouped.png  906 bytes
RESULT [5.2.0 LTS] the same group used twice in the tree
    out/g_grouped.png  906 bytes
RESULT [5.2.0 LTS] the File Output node two groups deep
    out/g_grouped.png  906 bytes
RESULT [5.2.0 LTS] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases, as on the Mac. The same caution applies: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 10.)

---

## 8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?

**Answer, in two parts.**

*A whole node: it can be muted, but the sources do not establish that muting stops its file writing.* The manual's Editing Nodes page:

> "Mute — Reference Menu: Node ‣ Show/Hide ‣ Mute Shortcut: M"

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

The API documentation confirms this exists on every node: `Node.mute` is listed on the Node page with "(default False)", and appears among the inherited properties on the CompositorNodeOutputFile page.

The manual's Render Output panel page also speaks of File Output nodes being active or not:

> "Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

Separately, the same Editing Nodes page says a node's incoming link can be cut off without touching the node:

> "A muted link acts as though it's no longer there; this also means the input fields for specifying fixed values become visible again."

*One output within a node: not in the sources.* The API documentation lists an individual output's settings in full (quoted under question 2) and there is no on/off among them — only a colour, a format block, a name, the override flag, the display-transform flag, a socket kind and a vector dimension count. The manual's Images/Layers section describes renaming and format override, nothing else. Searches run for this: "mute", "enable", "disable", "toggle", "active" on the File Output page and in the two API entries for the node and its items; "Enable Output" across the compositing section — that node exists but is about node *group* outputs ("The Enable Output node toggles the visibility and value of a socket for a Node Group outputs"), not about File Output.

**Label:** documented that the whole node has a mute setting; **not in the sources** that muting stops its file writing, or that a single output within a node has its own switch.

**Not covered.** The manual says muting "removes its contribution to the node tree" — it does not say in so many words that a muted File Output node writes nothing, and I will not make that step for you; it is the obvious reading but it is a step. Likewise, "active File Output Node nodes" is not defined anywhere I could find, so what makes one active or inactive is unstated. Whether removing an output from the list is the only way to stop it being written is unstated. What an unconnected input does is unstated.

**Outside the interface.** Muting is a named property on every node, so it can be set from outside as well as with the M key. The sources do not say it behaves differently.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Muting the node does stop it writing** — the step this answer declined to take:

```
RESULT [5.2.0 LTS] both outputs connected
    out/file_nameA.png  902 bytes
    out/file_nameB.png  865 bytes
RESULT [5.2.0 LTS] the whole node muted (node.mute = True)
    (nothing written)
```

**One output can be switched off on its own, through its link**, and an unconnected output writes nothing at all — not an empty file, not a black one:

```
RESULT [5.2.0 LTS] node unmuted, the link into output A muted (link.is_muted = True)
    out/file_nameB.png  865 bytes
RESULT [5.2.0 LTS] the link into output A removed, the output itself left on the node
    out/file_nameB.png  865 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
```

Muting the link and removing it give the same result, so removing an output from the list is not the only way to stop it being written. The socket's own `enabled` setting was `True` throughout and was not what did this. Whether the panel offers a per-output switch of its own is test 17 on the Creator's half. (Hand test 9.)


**Observed by hand test, from Blender's own window (2026-08-02).** **There is no per-output on/off control on the panel.** The Images list offers a plus, a minus and a pair of reorder arrows; right-clicking an entry gives Blender's ordinary property menu — Reset to Default Value, Copy to Selected, Copy Data Path, Copy Full Data Path, Copy as New Driver, Online Manual — and nothing that switches an output off. The only per-output controls are Socket Type, Override Node Format and the format settings under it. So removing an output, or cutting or muting its link as under the test above, is the way. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [5.2.0 LTS] both outputs connected
    out/file_nameA.png  906 bytes
    out/file_nameB.png  862 bytes
RESULT [5.2.0 LTS] the whole node muted (node.mute = True)
    (nothing written)
RESULT [5.2.0 LTS] node unmuted, the link into output A muted (link.is_muted = True)
    out/file_nameB.png  862 bytes
RESULT [5.2.0 LTS] the link into output A removed, the output itself left on the node
    out/file_nameB.png  862 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
SOCKET 'enabled' ON OUTPUT A: True
```

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 9 and 10.)

---

## 9. What restrictions, if any, apply to the kind of picture a File Output node can write?

**Answer.** The node's manual page does not give a separate list of formats. It points at Blender's general image settings:

> "Node Format ... Image: Controls the default format and encoding for all outputs. ... See Images Settings for details on supported formats and options."

> "Multi-Layer EXR: See OpenEXR for EXR-specific settings."

The general list it points to is the manual's Supported Image Formats page:

> "This is the list of image file formats supported internally by Blender:"

and the formats named in that table are JPEG, OpenEXR, PNG, WebP, AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF, each with its channel depths, whether it carries alpha, whether it carries metadata, whether it carries a pixel-density value, and its file extensions. Restrictions the manual names alongside it:

> "Note, RGBA is not available for all image formats, check the list above for details."

> "Note, not all file formats support all color depths."

> "Float (Half): ... Only supported for OpenEXR files."

> "Float (Full): ... Only supported for OpenEXR files."

> "Internally, Blender only operates in either 8-bit or 32-bit. Images with higher than 8-bit precision (e.g., 10-bit, 12-bit, 16-bit) are converted to 32-bit float when loaded into Blender."

> "Not all formats support metadata, and some may only support a limited subset of fields."

The extension is added or not by a node setting of its own, new in this version. From the API documentation's CompositorNodeOutputFile page:

> "use_file_extension — Add the file format extensions to the saved file name (eg: filename + .jpg) (default False)"

The API Change Log page lists it under "5.1 to 5.2", "bpy.types.CompositorNodeOutputFile", "Added": "bpy.types.CompositorNodeOutputFile.use_file_extension". And the 5.2 release notes for the compositor record why it is separate:

> "File Output node now has a **File Extension** checkbox that is independent of the 'File Extension' checkbox in the scene render settings."

**Label:** documented, with one inferred step: the manual's File Output page does not itself list formats, it refers to the general Images Settings page, so applying that page's list and its caveats to this node is a step the manual invites but does not spell out.

**Not covered.** The general list is Blender's, not the node's — nothing states that the node offers every entry on it, or excludes any. Nothing says which format the node starts on (the underlying format block's own default is stated as PNG, but that is the shared format block, not a statement about the node).

**Outside the interface.** The format is an ordinary named setting, so it can be chosen from outside.

**Operating system.** This is the one place the sources tie formats to the build. The manual's Command Line Arguments page lists formats in two groups:

> "-F, --render-format <format> Set the render format. Valid options are: TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF."

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

So the sources do state that OpenEXR, multi-layer OpenEXR, the video machinery, Cineon, DPX, JPEG 2000 and WebP may be absent depending on how Blender was built. That statement is made about the render format argument, not about the File Output node; extending it to the node is an inference, and I flag it as one.


**Observed by hand test (2026-08-02).** **The node's own format list is narrower than the general one, and what it will take depends on the media type** — which this answer said nothing stated. The setting's published list of values is

```
FILE FORMAT VALUES: ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
```

but with the node on Image media type it refused both the multi-layer value and the video value, and each refusal names what it will take:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

So on Image media type the node offers exactly the fourteen still-picture formats the manual's table lists, and neither the multi-layer nor the video value. **A new node starts on the multi-layer value**, not on the format block's own default of PNG. What the panel draws is test 17 on the Creator's half. (Hand tests 1, 5 and 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The File Format menu on the node draws exactly the fourteen formats the setting will take, and no more.** In the order shown: AVIF (.avif), JPEG (.jpg), OpenEXR (.exr), PNG (.png), WebP (.webp), then a divider, then Bitmap (.bmp), Cineon (.cin), DPX (.dpx), Iris (.rgb), JPEG 2000 (.jp2), Radiance HDR (.hdr), Targa (.tga), Targa Raw (.tga), TIFF (.tif) — fourteen in all. **OpenEXR MultiLayer and FFmpeg Video are both absent from it**, which is the same as what the setting refused. So what the panel offers and what the setting accepts agree, and the multi-layer choice is made with the Image / Multi-Layer EXR buttons above rather than from this menu.

**The File Extensions checkbox is drawn inside the Output Paths panel**, above the list of finished paths — the node's own copy of that setting, which this version added. (Hand test 17, run by the Creator.)


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

`use_file_extension` is on the node there too, with the same description.

**This narrows the build question without closing it.** The sources say some formats are compiled in and "not available on all systems". Two builds of this version have now been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same fourteen, AVIF among them on both. That is two builds, not all builds.

**Operating system:** the format list is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entries 1, 5 and 12.)

---

## 10. What happens when two outputs are aimed at the same name in the same place?

**Answer: not in the sources.** Nothing describes a collision between two outputs writing to the same final path — neither two outputs on one node, nor two nodes, nor a node colliding with the scene's own render output.

Where I looked:

- The whole File Output page. It describes how the final name is built ("{Directory}/{File Name}{Socket Name}{frame number}.{extension}") and offers a panel to check the result — "Output Paths ... Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly." — but says nothing about two of them matching.
- Searched the whole compositing section for "overwrite": one match, and it is the File Output page's own tip — "Use the File Output node to save raw passes or intermediate steps without overwriting the main render result." That is about not disturbing the main render result, not about two outputs meeting on one path.

  *Changed by the citation check, 2026-08-02: this line said "no matches at all". There is one, quoted above. It does not bear on a collision, so the answer stands, but the record was wrong.*
- Searched the two API entries for the node and its items for "overwrite": no matches.
- Searched the whole manual for "unique name", "duplicate name", "already exists" in this context: nothing about output sockets or file paths.
- The nearest thing in the sources is a pair of settings on the *scene's* render output, not on the node. From the API documentation's RenderSettings page: "use_overwrite — Overwrite existing files while rendering (default True)" and "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)", matching the Overwrite and Placeholders entries on the manual's Render Output panel page. Nothing states that these apply to File Output nodes, and I will not assume it.
- Nothing states whether two outputs on one node may even carry the same name.

**Label:** not in the sources.

**Not covered.** All of it: which write wins, whether an error or warning is raised, whether the Output Paths panel flags the clash, and whether the scene's Overwrite setting reaches the node at all.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Two outputs on one node cannot carry the same name — the second is renamed**, which settles the last line of this answer. Both were asked to be called `clash`; the node kept `['clash', 'clash_001']`, and two files were written, each holding its own picture:

```
RESULT [5.2.0 LTS] two outputs on one node, both asked to be called 'clash'
    out/file_nameclash.png  902 bytes
    out/file_nameclash_001.png  865 bytes
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
```

**Two separate nodes aimed at the same file give one file, and it holds the first node's picture.** No warning or error was printed, and four renders in a row gave the same result:

```
RESULT [5.2.0 LTS] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  902 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
```

Building the two nodes in the opposite order swapped which picture survived, so it follows the order the nodes were made in:

```
RESULT [5.2.0 LTS] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  865 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

**A file already on disk is replaced silently, and the scene's Overwrite setting does not reach the node:**

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785689725.588
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1785689725.933
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785689726.285
```

**Placeholders do not reach it either.** A two-frame animation with `render.use_placeholder` on produced the node's files and the scene's files and no placeholder for the node. (Hand tests 7 and 8.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The Output Paths panel flags nothing, because there is nothing to flag — Blender renames the second output before a clash can happen.** With two outputs both asked to be called `clash`, the Images list shows `clash` and `clash_001`, and the Output Paths panel lists two different finished paths, ending `shot_clash.png` and `shot_clash_001.png`. Nothing is coloured red and no warning mark appears. (Hand test 18, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same answers, pixel value for pixel value.**

```
ONE NODE, TWO OUTPUTS BOTH ASKED FOR 'clash' -> node kept ['clash', 'clash_001']
RESULT [5.2.0 LTS] two outputs on one node, both asked to be called 'clash'
    out/file_nameclash.png  906 bytes
    out/file_nameclash_001.png  862 bytes
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
RESULT [5.2.0 LTS] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  906 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
RESULT [5.2.0 LTS] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  862 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

A file already on disk was replaced silently, and switching the scene's Overwrite off changed nothing:

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785996005.559
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1785996005.645
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785996005.742
```

Placeholders did not reach the node there either.

**Which of two writes to one name survives is exactly the kind of thing that could differ between operating systems, and it does not.** Both machines keep the first-made node's picture, four repeats in a row give the same answer on both, and reversing the order of making the nodes reverses the answer on both.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 7 and 8.)


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **The Output Paths panel behaves the same way there, and flags nothing.** It is folded shut when the sidebar is first opened; unfolded, it shows a **File Extensions** checkbox, ticked, and then one row for each written output — a small picture icon and the finished path. With the two outputs both asked to be called `clash`, the list of outputs shows `clash` and `clash_001`, and the two rows read

```
/root/smoozy/File...t/shot_clash.png
/root/smoozy/File...ot_clash_001.png
```

with the middle of each path left out by Blender itself, because the panel is narrower than the path. **Nothing is coloured red and no warning mark of any kind appears** — the same as the Creator found on the Mac.

*How that picture was got, and why it can be trusted.* That machine's display is covered by a desktop lock screen. Blender does receive a button press through it but never learns where the pointer is, so an ordinary pointer click never operates any control. Clicks aimed at a bare screen position were tried and did nothing, and were abandoned. The panel was unfolded instead by a click sent **inside Blender's own process**, which carries its own position with it and does not go through the display's pointer at all. That this route really works was proved by clicking a checkbox and reading the setting behind it back: `show_options=True` before, `show_options=False` after, and `True` again when it was clicked back. Blender's own screenshot photographs only its own window, so the picture shows Blender's own state and nothing else. (Linux entry 18.)

---

## 11. Can a File Output node write a video file?

**Answer.** On the manual's File Output page, the node's Media Type is documented with two choices, and video is not one of them:

> "Media Type — Determines how multiple inputs are stored:"

> "Image: Saves each input as a separate image file. Each input can use its own image format settings."

> "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

The node's Node Format panel is likewise described for two cases only:

> "This panel controls file format and encoding options, depending on the Media Type: Image: ... Multi-Layer EXR: ..."

By contrast, the manual's Render Output panel page documents three choices for the same-named setting, including:

> "Video: Encodes each frame into a video container. Encoding options can be found in the Encoding panel."

And the shared format block on the API documentation's ImageFormatSettings page carries all three values:

> "media_type — The type of media to save (default 'IMAGE') Type: Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']"

So: the node's own documentation offers only images and multi-layer OpenEXR. The node has a shared image-format settings block, and that block's documented media-type property can hold a video value; applying that general property description to what this node will accept or do is an inference.

**Label:** documented that the node's Media Type is described with two choices and video is not among them; **inferred** that its settings block may accept the documented video value. I am deliberately *not* labelling "it cannot write video" as documented — the manual's silence about a third choice on this node does not amount to a statement that the node refuses it.

**Not covered.** What the node does if the video value is set on it from outside is not stated anywhere. Whether the interface even offers the choice on this node is not shown — the picture of the node on the manual page shows only the folder and name fields, with the media type living in the sidebar, which no picture in the sources shows. This is the single most worthwhile thing on this sheet to settle by hand; see the list at the end.

**Outside the interface.** The gap named just above is exactly an outside-the-interface question, and the sources do not answer it.

**Operating system.** The manual's Command Line Arguments page lists `FFMPEG` among "Formats that can be compiled into Blender, not available on all systems", so the video machinery is not guaranteed present in every build. That statement is about the render format argument, not about this node.


**Observed by hand test (2026-08-02).** **No. The node refuses the video value** — the thing this answer called the single most worthwhile item to settle by hand. The shared format block carries three media types, but this node will not take the third:

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

The refusal names only Image and Multi-Layer EXR — the same two the manual describes for this node. So the manual's silence about a third choice matches what the node does. The node stayed on PNG and a three-frame animation produced three pictures:

```
RESULT [5.2.0 LTS] node set to video, animation over frames 1 to 3
    out/file_nameclip0001.png  900 bytes
    out/file_nameclip0002.png  900 bytes
    out/file_nameclip0003.png  900 bytes
```

This is the outside-the-interface case this answer named: setting the value from outside is refused, not silently accepted. What the panel offers on screen is test 17 on the Creator's half. (Hand test 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The panel offers no video choice.** Media Type is drawn on this node as two buttons, **Image** and **Multi-Layer EXR**, not as a menu with a third entry — and the File Format menu under it holds no FFmpeg Video (listed under question 9). So the panel and the setting agree, and they agree with the manual describing only two media types for this node: video cannot be chosen here, by hand or from outside. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux refused it too, in the same words.**

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
RESULT [5.2.0 LTS] node set to video, animation over frames 1 to 3
    out/file_nameclip0001.png  904 bytes
    out/file_nameclip0002.png  904 bytes
    out/file_nameclip0003.png  904 bytes
```

The refusal names only Image and Multi-Layer EXR there too — the same two the manual describes for this node.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entry 12.)

---

## 12. What can a File Output node be fed, and does it have to come from the render?

**Answer.** No, it does not have to come from the render. The manual's File Output page says so, listing what the node is for:

> "Automatically save the final image after a render."

> "Save intermediate results from anywhere in the node tree."

> "Export multiple render passes or processed outputs directly to files."

> "Use the File Output node to save raw passes or intermediate steps without overwriting the main render result."

> "Combine with the Cryptomatte Node or other utility passes to export custom data for compositing pipelines."

Inputs are made by connecting something to the node:

> "Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel."

> "Each input corresponds to an image that will be written to disk."

The compositor's own material that is *not* the render includes, among the input nodes the manual documents under https://docs.blender.org/manual/en/5.2/compositing/types/input/, an Image node, a Movie Clip node, a Mask node, a Bokeh Image node, a Blank Image node, a String To Image node and a Group Input node, alongside the Render Layers node, which is the render one and whose page says:

> "Renders a View Layer and reads its Passes into the compositing node graph."

The manual's Compositor System page also states what kind of data travels in the compositor at all:

> "Compositing nodes operate on data that is either an image or a dimensionless single value."

> "Three types of data exist, all of which are stored in half precision formats: Float ... Vector ... Color"

and that mismatched kinds are converted on the way in:

> "In case a node input is given data of type other than its own type, the following implicit conversions are performed:"

**Label:** documented for "it does not have to come from the render" and for how inputs are made; **inferred** for the list of things that can feed it — the sources document those input nodes and document that any node's output can be dragged onto the node's blank socket area, and I have joined the two.

**Not covered.** The sources do not enumerate which node outputs are acceptable to a File Output input, nor what happens if a single value rather than an image is connected. An individual output's declared kind is stated as defaulting to Float, and the compositor now carries many more kinds than images and numbers (the 5.2 release notes add object, matrix, rotation, string and font sockets to the compositor), but nothing says which of those a File Output input will accept or what it would write.

**Outside the interface.** Dragging onto the blank socket area is an interface act, but the same list of outputs can be added, removed, cleared and reordered as a named collection, and links are ordinary node links, so the sources give no reason to think inputs cannot be built from outside. Nothing states this explicitly.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It has to be fed a picture. A plain colour writes nothing** — one of the two things this answer said were not covered. One node with two outputs, one fed by a colour node and one fed by the render through an Invert node, wrote only the second, with no error and with the link reported valid:

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [5.2.0 LTS] one output fed by a colour node, one by an image
    out/file_namepicture.png  865 bytes
```

**It does not have to come from the render**, confirmed: the picture that was written came through an Invert node rather than straight from the render, and every test on this sheet that used a modified picture wrote normally. (Hand test 13.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux did the same**, and the other thing this answer said was not covered was settled there for the first time.

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [5.2.0 LTS] one output fed by a colour node, one by an image
    out/file_namepicture.png  862 bytes
```

**Which of the twenty-four socket kinds a File Output input will accept** — which this answer said nothing states, and which was never tested on the Mac. An output was asked for of every one of them. **Only three could be made at all:**

```
OUTPUTS THE NODE ACCEPTED: [('k_FLOAT', 'FLOAT'), ('k_VECTOR', 'VECTOR'), ('k_RGBA', 'RGBA')]
```

Each of the other twenty-one was refused with the same message, for example:

```
ADDING AN OUTPUT OF KIND MATRIX RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
ADDING AN OUTPUT OF KIND INT_VECTOR RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
```

So the object, matrix, rotation, string and font sockets this version's release notes add to the compositor cannot be made into a written output on this node at all. All three that were made took the render picture, and all three wrote a file:

```
LINKS THE TREE ACTUALLY HOLDS INTO THE NODE: ['k_FLOAT', 'k_RGBA', 'k_VECTOR']
RESULT [5.2.0 LTS] one written output of every kind the node offers, all fed the render picture
    out/file_namek_FLOAT.png  768 bytes
    out/file_namek_RGBA.png  906 bytes
    out/file_namek_VECTOR.png  906 bytes
```

The Float file is smaller than the other two; the Vector and Colour files are the same size as each other. What this does **not** settle: what a Float or a Vector output writes when it is fed something other than a picture, and what channels are actually inside each written file — they were not read back.

**Operating system:** the colour-versus-picture part is now settled on macOS and on Linux, and they agreed; which socket kinds the node accepts is settled on Linux alone. Windows is untested. (Linux entries 13 and 21.)

---

## 13. What causes a File Output node to write, and when during a render does it write each file?

**Answer, in two parts.**

*What causes it to write: partly documented.* The manual's File Output page states the occasion broadly:

> "The File Output node writes one or more images to disk during rendering or compositing."

The compositor must be switched into the pipeline at all — the manual's Compositing Introduction page, and then its Post Processing panel page:

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

> "Pipeline – Compositing — Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

And the scene's own output saving can be turned off, in which case the render leans on these nodes instead — the first two from the manual's Render Output panel page, the third from its Rendering Animations page:

> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

> "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node."

> "Blender creates a file for each frame of the animation when Save Output is enabled. If it is disabled, frames will only be written if a Compositor File Output node is configured."

The matching setting on the API documentation's RenderSettings page is "save_output — Write frames to disk for animation renders (default True)".

*When during a render each file is written: not in the sources.* Nothing states the moment. Where I looked: the whole File Output page; the render Output and Rendering Animations pages (which give the "during rendering" framing above and nothing finer); the compositor system page, whose only timing statements are about evaluation, not writing — "Socket values are only available after the node tree has been evaluated. Nodes that do not contribute to the final output are generally not evaluated and therefore have no inspection data available."; the 5.2 release notes for the compositor, which describe when the *interactive* compositor runs but never mention file writing; and the API documentation's Application Handlers page, where the nearest entry is "render_write — on writing a render frame (directly after the frame is written)", which is about a render frame, not about this node, and I will not stretch it. Searches run: "during rendering", "after the render", "written", "writes", "per frame", "when rendering", across the compositing section and the render output pages.

**Label:** partly documented for the broad conditions under which writing occurs; **not in the sources** for the complete trigger or for when during a render each file is written.

**Not covered.** Whether the node writes on a single-frame render as well as an animation render is not stated in so many words — the "Still images must be saved manually ... or written using a File Output Node" sentence implies it, but that is an implication, not a statement. Whether it writes during interactive or viewport compositing, as opposed to a final render, is not stated; the node page says "during rendering or compositing" without defining the second. Whether writing happens before, during or after the frame is handed on, whether it is ordered against the scene's own output write, and what happens on a cancelled render, are all unstated.

**Outside the interface.** This matters most here and the sources are thin. Nothing on any of these pages says whether a File Output node writes when Blender is run without its window. The manual's Command Line Arguments page documents rendering that way — "-b, --background Run in background (often used for UI-less rendering)", "-a, --render-anim Render frames from start to end (inclusive)", "-f, --render-frame <frame> Render frame <frame> and save it" — and documents the scene's own output path argument "-o, --render-output <path>", but says nothing about compositor File Output nodes. So: not in the sources.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes when Blender is run without its window** — the case this answer recorded as not in the sources. From a saved file:

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

So `-b` with `-f` and with `-a` both write, alongside the scene's own output.

**A single-frame render does write** — the implication this answer would not turn into a statement — and the frame number is added only on an animation render, with `####` forcing it either way:

```
RESULT [5.2.0 LTS] render.render(animation=False)
    out/file_namehash0001here.png  900 bytes
    out/file_nameplain.png  900 bytes
RESULT [5.2.0 LTS] render.render(animation=True), frame range 1 to 1
    out/file_namehash0001here.png  900 bytes
    out/file_nameplain0001.png  900 bytes
```

**Each file is written as its frame finishes, just before the scene's own file for that frame.** Over a four-frame animation with both writing into one folder:

```
    1785690083.593  out/file_namenode0001.png
    1785690083.595  out/scene_0001.png
    1785690084.812  out/file_namenode0002.png
    1785690084.815  out/scene_0002.png
    1785690086.168  out/file_namenode0003.png
    1785690086.170  out/scene_0003.png
    1785690087.504  out/file_namenode0004.png
    1785690087.506  out/scene_0004.png
```

Frame 1's files carry a time about four seconds before frame 4's, so nothing waited for the end of the run; and within each frame the node's file is a couple of thousandths of a second ahead of the scene's.

**Switching compositing off stops it; having no Group Output node in the tree does not:**

```
RESULT [5.2.0 LTS] the same tree with render.use_compositing = False
    (nothing written)
RESULT [5.2.0 LTS] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  865 bytes
```

Whether a single frame rendered from the window writes, and whether viewport compositing writes, are tests 19 and 20 on the Creator's half. What happens to a part-finished render was not tested on this machine; it was tested later on the rented Linux machine, and the result is in the block below. (Hand tests 2, 3, 13 and 15.)


**Observed by hand test, from Blender's own window (2026-08-02).** **One frame rendered from the window does write.** Pressing F12 once and closing the render window without saving the render left four files, one for each connected output of the two nodes in the file:

```
shot_clash.png
shot_clash_001.png
shot_one.png
shot_two.png
```

So a File Output node does not need the render to be saved by hand. Note that none of the four names carries a frame number, which matches the rule above: a single-frame render from the window is not an animation render.

**The viewport compositor writes nothing.** With compositing switched on in the 3D viewport and left running for about thirty seconds with no render started, the folder stayed empty. So "during rendering or compositing" does not extend to the viewport. (Hand tests 19 and 20, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on every part, and the one thing this answer said had not been tested was settled here.**

**It writes when Blender is run without its window**, from a saved file:

```
--- blender -b fo_scene.blend -f 1  (Blender 5.2)
00:01.758  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0001.png'
00:01.758  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
--- blender -b fo_scene.blend -a  (Blender 5.2)
00:02.212  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0001.png'
00:02.213  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
00:02.380  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0002.png'
00:02.381  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0002.png'
00:02.537  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0003.png'
00:02.538  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0003.png'
```

**A single-frame render does write**, and the frame number is added only on an animation render, with `####` forcing it either way:

```
RESULT [5.2.0 LTS] render.render(animation=False)
    out/file_namehash0001here.png  904 bytes
    out/file_nameplain.png  904 bytes
RESULT [5.2.0 LTS] render.render(animation=True), frame range 1 to 1
    out/file_namehash0001here.png  904 bytes
    out/file_nameplain0001.png  904 bytes
```

**Each file is written as its frame finishes, just before the scene's own file for that frame:**

```
FILES IN THE ORDER THEY WERE LAST WRITTEN (seconds since the epoch):
    1785996168.524  out/file_namenode0001.png
    1785996168.533  out/scene_0001.png
    1785996169.796  out/file_namenode0002.png
    1785996169.798  out/scene_0002.png
    1785996171.053  out/file_namenode0003.png
    1785996171.059  out/scene_0003.png
    1785996172.149  out/file_namenode0004.png
    1785996172.152  out/scene_0004.png
```

That machine is shared with other work, so no claim is made about how long any of it took; what the test shows is the **order**, and the order is the same on both machines.

**Switching compositing off stops it; having no Group Output node in the tree does not:**

```
RESULT [5.2.0 LTS] the same tree with render.use_compositing = False
    (nothing written)
RESULT [5.2.0 LTS] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  862 bytes
```

**What a stopped render leaves behind** — the thing this answer records as untested, and which the Mac never reached. A sixty-frame animation was started from the command line and stopped part way, twice: once with an interrupt — the polite stop, what pressing Ctrl-C does — and once with a hard kill that gives Blender no chance to tidy up. **Both left whole files for every frame that had finished, and nothing at all for the frame that was in flight.** No part-written file, no empty file, no file of the wrong size, and the node's files and the scene's files always in matching pairs. Stopped with an interrupt:

```
blender's exit code: 1
the last ten lines blender printed:
00:07.973  render           | Fra: 21 | Rendering 100 / 256 samples
...
00:08.117  render           | Fra: 21 | Rendering 256 / 256 samples

Blender quit
what is on disk afterwards:
    file_namebeauty0001.png  10854 bytes
    ... (twenty in all, 0001 to 0020, every one 10854 bytes)
    scene_0001.png  10961 bytes
    ... (twenty in all, 0001 to 0020, every one 10961 bytes)
```

Twenty pairs, all whole, and frame 21 — the one being rendered when the stop arrived — left nothing. Stopped with a hard kill (`blender's exit code: 137`) it left twenty pairs again, all whole, with frame 21 leaving nothing. The counts differ between versions only because the machine got through different numbers of frames before the stop arrived; that machine is shared, so the counts are not a measure of its speed.

Whether a single frame rendered from the window writes, and whether viewport compositing writes, were **not** run on Linux — those two need the machine's screen to themselves, and it is shared. They are answered for macOS by the Creator's run above, and are written under *Cannot be run* in the Linux hand-test list.

**Operating system:** the terminal half of this answer is now settled on macOS and on Linux, and they agreed; what a stopped render leaves behind is settled on Linux alone. The window half is settled on macOS alone. Windows is untested throughout. (Linux entries 2, 3, 13, 15 and 22.)

---

## Cycles' own files

I searched the whole of the supplied Cycles folder for "file_output", "OutputFile", "file output" and "compositor". The only hits for any of them were four lines in `ui.py` naming compositor *performance* panels that Cycles borrows from Blender's render properties (`CYCLES_RENDER_PT_performance_compositor` and a denoise child of it, at lines 842 to 849, plus their registration at lines 2617 to 2618, and an import at line 14). Nothing in Cycles' files concerns the File Output node, its settings, or its writing. So nothing above is labelled **observed** on the strength of Cycles. (The observed findings this document does carry came later, from hand tests run in a running Blender on 2026-08-02; they are marked as such.)

---

## What is still left for a person with Blender open

The list that stood here has been worked through, and nothing on it is outstanding. The tests a terminal could reach were run on 2026-08-02 and the four that needed eyes on the window were run by the Creator the same day; both kinds are written into the answers above as observed findings, and the numbered tests are in `Audits/Blender Research/File output nodes/Hand tests.md`.

**Then a second machine arrived.** On 2026-08-06 the whole list was run again on a rented Linux machine, and two behaviours the Mac could never reach were run for the first time: which kinds of input socket the node will actually accept and what each writes, and what a stopped render leaves behind. Those results are in the answers above too, marked as coming from that machine, and the entries are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`. Linux agreed with the Mac on every test both machines ran; the one place their words differ is the operating system's own description of a failed write, recorded under question 5.

**Cannot be run at all.** Four things, and the reasons are written out in the *Cannot be run* sections of both hand-test lists:

- **A folder reached over a network**, in any version. Neither machine has a network store to point at, and setting one up on the rented machine would mean installing something and changing a system setting.
- **Anything about Windows.** No Windows machine has run it. macOS and Linux are both covered; Windows is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **A Blender built by somebody else.** Two builds of this version have been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same file formats. That is two builds, not all builds.
- **A frame rendered from Blender's own window on Linux, and viewport compositing on Linux.** That machine's screen is shared with several other topics at once, and those two tests would have to hold it for their whole length. They are answered for macOS by the Creator's run, and are on the *Needs the machine to itself* list for later.

2026-08-02 — Opus 5 (Claude Code) — File output nodes, Blender 5.2

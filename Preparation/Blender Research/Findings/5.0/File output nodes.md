# File output nodes — Blender 5.0

**Sources consulted, and the date I read them.**
Blender 5.0 manual (plain-text snapshot), Blender 5.0 Python API documentation (plain-text snapshot), Blender 5.0 release notes (plain files), one manual image, and the Cycles files supplied. All read on 2026-07-30 (the snapshot date given).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

**A note on the Cycles files.** I searched all of `cycles_5.0` for `file_output`, `OutputFile`, `file output` and `compositor`. The only hits are four lines in `ui.py` that pull in compositor *performance* panels from Blender's own render properties. Nothing in Cycles' files describes the File Output node. So nothing below is labelled **observed** on the strength of Cycles. (The observed findings this document does carry came later, from hand tests run in a running Blender on 2026-08-02; they are marked as such.)

**A note on wording.** The sheet says "outputs" of a File Output node. The node has no output sockets at all; the things it writes are its *input* sockets. I have read "output" throughout as "one of the things the node writes", which is what the manual itself calls an output in its Images/Layers and Output Paths panels. Where that reading matters I say so.

---

## Observed by hand test

Everything under this heading was settled by running Blender 5.0.1 on the Creator's own computer — one MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2 — on **2026-08-02**. Blender was started from a terminal with no window, in the form

`"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup --python /tmp/run.py`

and driven through its Python support. The exact scripts, and which test number each result came from, are in `Audits/Blender Research/File output nodes/Hand tests.md`. The machine's own output is quoted below word for word; where a listing is long, the part that carries the answer is kept and the rest is said to be omitted.

A few of the blocks below are marked **from Blender's own window** instead. Those were run by the Creator on the same computer on the same day, by opening a prepared file, reading the node's panel, and rendering from the window. They are the tests that needed eyes on the screen.

---

## Observed by hand test on a second machine

On **2026-08-06** the same tests were run again on a second computer: **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; its screen is shared with other work. Blender 5.0.1 there is the Linux x64 build from Blender's own download server, at `/opt/blender/blender-5.0.1-linux-x64/blender`, and it was started the same way:

`/opt/blender/blender-5.0.1-linux-x64/blender -b --factory-startup --python run.py`

with `BLENDER_USER_RESOURCES` pointed at a settings folder of its own, and with `DISPLAY` and `XAUTHORITY` unset so that the display's own `Authorization required` noise stayed out of the output. The scripts were the Mac's, word for word, with only the folder they write into changed. Two of them needed Blender's window, which was opened on that machine's own display and photographed with Blender's own screenshot command. The entries and the full output are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

Every question below therefore carries results from two machines. **A Mac result and a Linux result are two results and are never merged.** Where they agreed, both are named and the agreement is said plainly; where the machines differed, both are given and neither is chosen.

**These blocks update the two standing lines for every question they touch.** *Driven from outside*: every result not marked as coming from the window was produced with Blender driven from outside its own interface, with no window open, so each holds in that case; where a window result differs from the terminal one, both are given. That is now true on both machines. *Operating system*: every answer that carries both an *Observed by hand test* block and an *on the rented Linux machine* block below is settled on **macOS 26.5.2 on an Apple M1 Pro and on Ubuntu 22.04.5 LTS on this rented machine**, and on nothing else. **Windows was not tested at all**: no Windows machine has run it. Both hand-test lists record it under *Cannot be run*.

---

## 1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?

**Label: documented**, except the count, which is **not in the sources**.

**What it writes.** The manual states:

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

and lists what it is for:

> "Automatically save the final image after a render.
> Save intermediate results from anywhere in the node tree.
> Export multiple render passes or processed outputs directly to files."

**Where the outputs come from and what each carries.**

> "By default, the node has no input sockets. Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel."

> "Each input corresponds to an image that will be written to disk."

> "Lists all input sockets of the node. The name of each input socket is used to construct the final file name."

And the node has nothing coming out of it:

> "Outputs
> This node has no output sockets."

**Two ways of being written.** The node-wide Media Type decides whether each output becomes its own file or a layer in one file:

> "Media Type
> Determines how multiple inputs are stored:
> Image:
> Saves each input as a separate image file. Each input can use its own image format settings.
> Multi-Layer EXR:
> Saves all inputs together in a single multi-layer OpenEXR file."

> "When using Multi-Layer EXR, each input is stored as a separate layer within the file."

**What each output carries, in data terms.** Each output is one item, and each item has a data type:

> "socket_type
> Type:
> enum in Node Socket Data Type Items, default 'FLOAT'"

> "vector_socket_dimensions
> Dimensions of the vector socket
> Type:
> int in [2, 4], default 0"

**How many.** The outputs are a collection with add, remove, clear and move:

> "new(socket_type, name)
> Add an item at the end"

> "active_item_index
> Index of the active item
> Type:
> int in [0, inf], default 0"

No maximum count is given anywhere. I searched the manual page, both API pages, and the release notes for any limit; there is none.

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItem.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItems.html
- Date read: 2026-07-30

I also opened the manual's picture of the node, `_images/node-types_CompositorNodeOutputFile.webp` (shown on the File Output Node manual page). It shows a node headed "File Output" with two text fields — one containing `/tmp/` with a folder-browse button beside it, one containing `file_name` — and a single hollow socket on its left edge, with no sockets on its right edge. **Documented** (it is part of the manual). I take from it only what is visible: the node's own body carries the two path fields, it starts with an empty socket, and it has nothing on its output side.

**Not covered.** No upper limit on the number of outputs. Nothing about what happens if an output is added but left unconnected — the manual says "Each *connected* input is saved", but does not say what an unconnected one does. Nothing about how the socket name is sanitised if it contains characters a file system rejects.

**Driven from outside.** The naming pattern includes a frame number, and the release notes state a difference that depends on how Blender is driven: "The **File Output** node now only appends the frame number to the image name when doing an animation render. Rendering through the command line always does an animation render, so no change of behavior is expected. Rendering using the `render.render()` operator when `animation=False` will see the change of behavior, but setting that to True and providing `start_frame` and `end_frame` should restore the old behavior. Finally, adding `####` manually in the name should substitute the frame number even in single renders." (Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/, read 2026-07-30.) Everything else in this answer is stated without reference to how Blender is driven. Adding inputs by dragging a link is an interface action; the API offers `file_output_items.new()` instead.

**Operating system.** The sources do not say. Nothing in the File Output pages distinguishes operating systems.


**Observed by hand test (2026-08-02).** **The node has nothing on its output side** — `NODE HAS OUTPUT SOCKETS: []` — and the old pair of slot collections is gone: the running program reports `TYPE NodeOutputFileSlotFile present=False`, `TYPE NodeOutputFileSlotLayer present=False`, `TYPE NodeCompositorFileOutputItem present=True`.

**A new node does not start on the same format as the scene.** `A NEW NODE STARTS ON: file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE` while `THE SCENE STARTS ON: file_format=PNG media_type=IMAGE`. So a File Output node added and left alone writes one multi-layer OpenEXR file, not separate pictures.

**No ceiling was reached on how many outputs a node takes**: `OUTPUTS ADDED WITHOUT REFUSAL: 2000` and `INPUT SOCKETS ON THE NODE NOW: 2001` — two thousand outputs plus the blank socket — with nothing refused. That establishes at least two thousand and says nothing above it.

**Three outputs really do become one file, and the output's name becomes the layer's name.** With three outputs named `first`, `second` and `third` and the node in Multi-Layer EXR mode:

```
RESULT [5.0.1] three outputs, Multi-Layer EXR mode
    out/file_name.exr  14944 bytes
```

and the readable text at the start of that file contains `'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R'`, then the same for `second` and for `third`, each with its own `'name', 'string', 'first'` and so on (the rest of the header text is omitted). (Hand tests 1, 6 and 14.)


**Observed by hand test, from Blender's own window (2026-08-02).** Outputs are added and removed with a plus and a minus beside the list in the **Images** panel, with a pair of arrows for reordering, and the kind of thing an output carries is drawn as a **Socket Type** dropdown below the list. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed with the Mac on every part, and filled in one thing this version's record was missing.**

The same three checks came back the same: `NODE HAS OUTPUT SOCKETS: []`; `TYPE NodeOutputFileSlotFile present=False`, `TYPE NodeOutputFileSlotLayer present=False`, `TYPE NodeCompositorFileOutputItem present=True`; and `A NEW NODE STARTS ON: file_format=OPEN_EXR_MULTILAYER media_type=MULTI_LAYER_IMAGE` while `THE SCENE STARTS ON: file_format=PNG media_type=IMAGE`. So a node added and left alone writes one multi-layer OpenEXR file here too.

**Which socket kinds an output may be set to**, which this version's findings never recorded, read from the running program:

```
    socket_type | ENUM | description='' values=['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE']
```

Eighteen values. That is what the setting publishes; what the node will actually accept is far narrower, and is under question 12.

**No ceiling was reached on how many outputs a node takes** there either: `OUTPUTS ADDED WITHOUT REFUSAL: 2000` and `INPUT SOCKETS ON THE NODE NOW: 2001`, with nothing refused.

**Three outputs really do become one file**, with the output's name as the layer's name:

```
RESULT [5.0.1] three outputs, Multi-Layer EXR mode
    out/file_name.exr  14944 bytes
```

whose header text carries `'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R'`, then the same for `second` and `third` (the rest omitted).

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 1, 6 and 14.)

---

## 2. Which settings belong to an individual output, and which belong to the node as a whole?

**Label: documented.**

**Belonging to the node as a whole.** The manual lists the node's own properties:

> "Directory
> The base output directory.
> File Name
> The base name of the file. The final filename may also include frame numbers or layer names depending on the settings.
> Media Type
> Determines how multiple inputs are stored:"

and a node-wide format panel:

> "Node Format
> Reference
> Panel:
> Sidebar ‣ Node ‣ Properties ‣ Node Format
> This panel controls file format and encoding options, depending on the Media Type:
> Image: Controls the default format and encoding for all outputs."

and a node-wide read-out:

> "Output Paths
> ...
> Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly."

The API confirms which properties sit on the node:

> "directory
> The directory where the image will be written"
> "file_name
> The base name of the file. Other information might be included in the final file name depending on the node options"
> "format
> Type:
> ImageFormatSettings, (readonly)"
> "save_as_render
> Apply render part of display transform when saving byte image
> Type:
> boolean, default False"
> "active_item_index"
> "file_output_items"

**Belonging to an individual output.** The manual:

> "When saving individual images, each output can be renamed and will be written to disk using either the global Node Format settings or its own overridden settings."

> "Override Node Format
> Allows the selected output to use a custom format instead of the global Node Format.
> Item Format
> When Override Node Format is enabled, this panel controls the format and encoding options for the selected output image."

The API lists an item's complete set of properties:

> "color
> Color of the corresponding socket type in the node editor ... (readonly)
> format
> Type:
> ImageFormatSettings, (readonly)
> name
> Type:
> string, default "", (never None)
> override_node_format
> Use a different format instead of the node format for this file
> Type:
> boolean, default False
> save_as_render
> Apply render part of display transform when saving byte image
> Type:
> boolean, default False
> socket_type
> Type:
> enum in Node Socket Data Type Items, default 'FLOAT'
> vector_socket_dimensions
> Dimensions of the vector socket
> Type:
> int in [2, 4], default 0"

So: folder, base file name, media type and the fallback format are the node's; name, whether to override the format, the overriding format itself, colour-management-on-save, data type and vector width are the individual output's. `save_as_render` exists in both places.

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItem.html
- Date read: 2026-07-30

**Not covered.** The Media Type setting lives inside `ImageFormatSettings`, and each item has its own `ImageFormatSettings` (Python API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html, read 2026-07-30, which lists `media_type` as an enum in `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`). Whether an individual output's media type can therefore be set independently of the node's is **not** settled by the sources — the manual describes Media Type only as a node-wide choice. I have not guessed either way. Also not covered: which of the node's two `save_as_render` settings wins when an output overrides the node format.

**Driven from outside.** The split holds in the API too: the node-wide settings are properties of `CompositorNodeOutputFile`, the per-output ones are properties of `NodeCompositorFileOutputItem`. The Node Format, Item Format and Output Paths *panels* are interface things; the underlying properties are reachable without the interface. The sources do not say whether the Output Paths read-out has any equivalent outside the interface.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **The full list of settings, from the running program.** On the node: `file_output_items`, `active_item_index` ("Index of the active item"), `directory` ("The directory where the image will be written"), `file_name` ("The base name of the file. Other information might be included in the final file name depending on the node options"), `format` (read-only), and `save_as_render`. On one written output: `name`, `socket_type`, `vector_socket_dimensions`, `color`, `override_node_format` ("Use a different format instead of the node format for this file"), `save_as_render`, and `format`. That matches the answer above exactly; there is no `use_file_extension` on the node in this version.

**Which display-transform setting wins, which the sources left open.** Eight combinations, one at a time, with the written pixel read back:

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

While Override Node Format is off, only the node's `save_as_render` changes the file. While it is on, only the output's own `save_as_render` changes it. (The values in the two halves are not on the same scale, because the node's format block and the output's own were not left carrying identical settings; what the test shows is which flag moves the value, not what the two halves have in common.) (Hand tests 1 and 16.)


**Observed by hand test, from Blender's own window (2026-08-02).** **Every control the panel draws, with a File Output node selected and the sidebar open on the Node tab.** Under **Node**: Reset Node, Name, Label and Color. Under **Properties**, belonging to the node: the folder field with a folder-browse button, the file-name field, then a two-button choice reading **Image** | **Multi-Layer EXR**, then a **Node Format** panel holding Save as Render, File Format, Color, Color Depth, Compression and a Color Management panel (Follow Scene or Override; Override adds Display, View, Look, Exposure, Gamma, Use Curves with its curve editor, and Use White Balance with Temperature and Tint). Then an **Images** panel holding the list of outputs. Belonging to the selected output: **Socket Type**, **Override Node Format**, and an **Item Format** panel repeating the same format and colour-management controls. Then an **Output Paths** panel listing the finished path of each output. Finally **Custom Properties**.

Neither the folder field nor the file-name field carries a visible label beside it; the manual's names Directory and File Name are not drawn on the panel.

**The Override Node Format tooltip reads:** "Use a different format instead of the node format for this file." That is word for word what the API reference gives, so the two agree. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux printed the same list of settings and the same eight pixel values.** On the node: `file_output_items`, `active_item_index`, `directory`, `file_name`, `format` (read-only) and `save_as_render`; on one written output: `name`, `socket_type`, `vector_socket_dimensions`, `color`, `override_node_format`, `save_as_render` and `format`. There is no `use_file_extension` on the node in this version there either. The display-transform test:

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


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **What the node draws on its own face.** The node's body carries a header with the node's label, then two fields one above the other — the folder, and the base file name — then one row for each written output showing the output's name alone, and finally a row holding a `+`, which is the blank socket a new input is dragged onto. With outputs called `one` and `two` the rows read `one` and `two`. Neither of the two fields carries a written label beside it, which agrees with what the Creator found on the Mac's sidebar. The Creator's Mac screenshots were of the sidebar rather than the node's face, so the rest of this is the Linux picture and nothing more. (Linux entry 17.)

---

## 3. Does each output carry its own folder as well as its own name?

**Label: inferred.**

The claim: no. In Blender 5.0 the folder belongs to the node, and an individual output carries only a name.

The step I took. The API page for `NodeCompositorFileOutputItem` lists that type's properties in full — `color`, `format`, `name`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions` — and none of them is a folder, directory or path. The only directory property in the whole node is on the node itself:

> "directory
> The directory where the image will be written"

The manual describes the same division, calling Directory "The base output directory" and describing an output's own setting as a name only: "each output can be renamed", and "The name of each input socket is used to construct the final file name."

The release notes make the change explicit, including what was taken away:

> "- File Output node now takes `directory` and `file_name` as two different inputs
>   - Removed:
>     - `bpy.types.CompositorNodeOutputFile.file_slots`
>     - `bpy.types.CompositorNodeOutputFile.layer_slots`
>     - `bpy.types.CompositorNodeOutputFile.base_path`
>   - Added:
>     - `bpy.types.CompositorNodeOutputFile.directory`
>     - `bpy.types.CompositorNodeOutputFile.file_name`
>     - `bpy.types.CompositorNodeOutputFile.file_output_items`"

and

> "# Giving input sockets a custom name in 4.5:
> file_output_node = ...
> file_output_node.file_slots[0].path = "my_custom_socket_name"
>
> # In 5.0:
> file_output_node = ...
> file_output_node.file_output_items[0].name = "my_custom_socket_name""

A reader must supply one thing to get from those words to my claim — that the API page lists every property an item has — which is why this is inferred and not documented.

- Exact words quoted above.
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItem.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/
- Date read: 2026-07-30

**Not covered.** This settles that there is no *separate* folder setting per output. It does not settle whether a folder can be smuggled into the name — that is question 4, and the sources do not answer it.

**Driven from outside.** The same properties are what the API exposes, so the division is the same when Blender is driven from outside. No difference is stated.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Confirmed, and in this version there is no way to smuggle a folder into the name either.** Every separator asked for in an output's name was replaced before the node kept it:

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub_shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub_shot_001'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a_b_c_deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '.._escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '_absolute'
```

So an output carries a name and nothing else, and the name cannot reach out of the node's folder. (Hand test 4.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux replaced every separator in the same way.**

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub_shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub_shot_001'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a_b_c_deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '.._escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '_absolute'
```

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 4.)

---

## 4. Can a folder be expressed inside an output's name rather than separately?

**Label: not in the sources.**

The nearest the sources get is the naming pattern, which shows where the socket name sits in the finished path:

> "For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

The socket name lands in the file-name part, after the folder separator. But nothing states what happens if the name itself contains a separator — whether it makes a subfolder, is rejected, or is written literally.

**Where I looked.**

- The whole File Output Node manual page (https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html) — every panel description, including Images/Layers and Output Paths.
- `NodeCompositorFileOutputItem` (https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItem.html) — the `name` property carries no path-related annotation (it reads only "string, default "", (never None)"), whereas the node's `directory` and `file_name` both carry them ("blend relative // prefix supported", "Supports template expressions"). The absence of any path note on `name` is not a statement, so I am not treating it as one.

  *Changed by the citation check, 2026-08-02: this line said `name` "carries no annotation at all". The API page does annotate it — "(default "", never None)" — so the claim is narrowed to path-related annotations. The conclusion drawn from the absence is unchanged.*
- `CompositorNodeOutputFile` and `NodeCompositorFileOutputItems` (https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItems.html).
- The File Paths manual page in full (https://docs.blender.org/manual/en/5.0/files/file_paths.html) — path templates, variables, format specifiers, escape sequences, errors. It names only "the render output path in Scene properties and the output paths in the compositor's File Output node" as supporting templates, and says nothing about separators.
- Release notes for the compositor, the Python API and the core (https://developer.blender.org/docs/release_notes/5.0/compositor/, https://developer.blender.org/docs/release_notes/5.0/python_api/, https://developer.blender.org/docs/release_notes/5.0/core/).
- Whole-manual searches for: "file output" (five files, all listed and read), "subdirector", "sub-director", "subfolder", "collision", "conflict", "same name", "unique name", "duplicate".

- Date read: 2026-07-30

**Not covered.** Everything about this question. It is not settled in either direction.

**Driven from outside.** The sources do not say. Note that if it were possible, the difference between typing a name in the interface and setting `file_output_items[n].name` from outside would matter, and the sources address neither.

**Operating system.** The sources do not say. This is a question where operating systems plausibly differ, since path separators differ, and the sources are silent on it.


**Observed by hand test (2026-08-02).** **No, not in this version.** With the node pointed at `/tmp/fo/out/base` and five outputs named with separators, every separator was turned into an underscore in the name the node kept (quoted under question 3), and every file landed flat in the node's own folder — no subfolder was made anywhere:

```
RESULT [5.0.1] separators in output names, Image mode, node folder /tmp/fo/out/base
    base/file_name.._escape.png  757 bytes
    base/file_name_absolute.png  757 bytes
    base/file_namea_b_c_deep.png  757 bytes
    base/file_namesub_shot.png  757 bytes
    base/file_namesub_shot_001.png  757 bytes
```

A node whose only output was named with a backslash gave the same treatment: `A NODE WHOSE ONLY OUTPUT IS NAMED 'sub\\shot' -> node kept 'sub_shot'`, and the file was `base2/file_namesub_shot.png`.

**In Multi-Layer EXR mode the same replacement happens**, so the layer inside the file is called `passes_diffuse`: the written `.exr` header text contains `'passes_diffuse.A', 'passes_diffuse.B', 'passes_diffuse.G', 'passes_diffuse.R'`. Nothing was reported to the terminal in any of these cases. (Hand tests 4 and 5.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux did the same, and everything landed flat in the node's own folder there too.** The names the node kept are quoted under question 3. The files:

```
RESULT [5.0.1] separators in output names, Image mode, node folder /root/smoozy/File output nodes/work/fo/out/base
    out/base/file_name.._escape.png  738 bytes
    out/base/file_name_absolute.png  738 bytes
    out/base/file_namea_b_c_deep.png  738 bytes
    out/base/file_namesub_shot.png  738 bytes
    out/base/file_namesub_shot_001.png  738 bytes
```

A node whose only output was named with a backslash got the same treatment — `A NODE WHOSE ONLY OUTPUT IS NAMED 'sub\\shot' -> node kept 'sub_shot'` — and wrote `out/base2/file_namesub_shot.png`. In Multi-Layer EXR mode the same replacement happened, so the layer was called `passes_diffuse`: the written file's header text carries `'passes_diffuse.A', 'passes_diffuse.B', 'passes_diffuse.G', 'passes_diffuse.R'`.

**This is the sharpest operating-system question on the sheet, and both machines answer it the same way.** A backslash is a folder separator on Windows and an ordinary character in a file name on macOS and Linux; in this version Blender replaces it, along with the forward slash, on both machines tested. Windows itself is untested.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 4 and 5.)

---

## 5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?

**Label: documented** for folders relative to the blend file and for folders built from path templates. **Inferred** for folders unrelated to the blend file. **Not in the sources** for folders reached over a network.

**Folders relative to the blend file, and templates.** The API annotates the node's directory property:

> "directory
> The directory where the image will be written
> Type:
> string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

The manual explains the prefix:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

And the templates:

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

The variables available include ones that build a folder from where the blend file sits, and ones that are specific to a node:

> "blend_name:
> The currently open blend file's name (without the .blend).
> blend_dir:
> Path up to (but not including) the currently open blend file."

> "Available only in path properties on a node:
> node_name:
> The name of the node that the path property is on."

A malformed template stops the path being used:

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

**Folders unrelated to the blend file.** The manual's picture of the node, `_images/node-types_CompositorNodeOutputFile.webp` on the File Output Node page, shows the Directory field containing `/tmp/` — an absolute path with no relation to any blend file, and a folder-browse button beside it. The step: from "the manual's own illustration of the node shows an absolute path in that field" I conclude absolute paths unrelated to the blend file are accepted. That is a step beyond what the picture shows, so: **inferred**.

**Folders over a network.** Nothing in the sources says whether the File Output node's directory may be a network location.

**Where I looked for the network part.** Whole-source searches for "UNC", "network path", "network drive", "network file system", "shared directory", and backslash-backslash server prefixes across the 5.0 manual. Three hits, none about this node: a read-only network drive in the Blender deployment page; a Freestyle Python example; and one hint on the scene render Output panel, which concerns the *scene* render output path, not the node —

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

I am not carrying that across to the File Output node. Disable Overwrite and Placeholders are properties of `RenderSettings` (`use_overwrite`, `use_placeholder`), not of the node, and nothing says the node honours them.

- Exact words quoted above.
- Python API, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html
- Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html
- Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Manual image, `_images/node-types_CompositorNodeOutputFile.webp`, on https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Date read: 2026-07-30

**Not covered.** Whether a network folder works, and what happens if it is unreachable. Whether Blender creates the folder if it does not exist. Whether the Overwrite and Placeholder settings on the scene apply to files this node writes. What happens when a folder cannot be written to.

**Driven from outside.** The `//` prefix and the templates are properties of the path string, not of the interface, so they hold when the path is set from outside. One thing is stated to differ: relative paths need a saved blend file. The template-error behaviour is described in terms of a red field and a tooltip, which are interface things; the release note on templates gives the same variables without reference to the interface.

**Operating system.** The sources do not say. The illustrated default `/tmp/` is a path shape used on some operating systems and not others, but the sources make no claim about that.


**Observed by hand test (2026-08-02).** **A folder unrelated to where the Blender file sits is accepted**, which settles the part this answer had as inferred: every test on this sheet wrote into `/tmp/fo/out`, which has nothing to do with the file's own folder, and did so with the file unsaved.

**A folder that does not exist is made.** Pointed at `/tmp/fo/out/does/not/exist/yet`, with no error printed:

```
FOLDER /tmp/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [5.0.1] node folder that did not exist
    out/does/not/exist/yet/file_namemade.png  757 bytes
```

**The `//` form works and is relative to the saved file.** With the file saved at `/tmp/fo/out/held.blend` and the folder set to the text `//relative/`:

```
RESULT [5.0.1] node folder set to the text //relative/ with the file saved at /tmp/fo/out/held.blend
    out/held.blend  95709 bytes
    out/relative/file_namemade.png  769 bytes
```

**A folder that cannot be written to fails loudly and writes nothing:**

```
00:01.423  image.write      | ERROR OpenImageIO write failed: Could not open file "/tmp/fo/out/locked/file_namemade.png"
00:01.423  reports          | ERROR Render error (Undefined error: 0) cannot save: '/tmp/fo/out/locked/file_namemade.png'
RESULT [5.0.1] node folder that cannot be written to (mode 500)
    (nothing written)
```

Network folders were not tested and could not be — see *Cannot be run* in the hand-test list. (Hand test 11.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts, and this test had to be run twice there.**

A folder that does not exist is made:

```
FOLDER /root/smoozy/File output nodes/work/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [5.0.1] node folder that did not exist
    out/does/not/exist/yet/file_namemade.png  738 bytes
```

The `//` form is relative to the saved file:

```
RESULT [5.0.1] node folder set to the text //relative/ with the file saved at /root/smoozy/File output nodes/work/fo/out/held.blend
    out/held.blend  95705 bytes
    out/relative/file_namemade.png  781 bytes
```

**Why the unwritable-folder test was run twice.** The account running Blender on that machine is the root user, and root walks through file permissions. Run as plain root, the folder set to permissions 500 was written to anyway:

```
RESULT [5.0.1] node folder that cannot be written to (mode 500)
    out/locked/file_namemade.png  781 bytes
```

Run again with that power dropped for the one command, which is the real answer, it failed loudly and wrote nothing, as on the Mac:

```
00:02.634  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/smoozy/File output nodes/work/fo/out/locked/file_namemade.png"
00:02.634  reports          | ERROR Render error (Success) cannot save: '/root/smoozy/File output nodes/work/fo/out/locked/file_namemade.png'
RESULT [5.0.1] node folder that cannot be written to (mode 500)
    (nothing written)
```

**The words differ between the machines and the behaviour does not.** The Mac said `Render error (Undefined error: 0)`; Linux says `Render error (Success)`. The words in the brackets are the operating system's own description of what went wrong, and the two systems describe it differently. What Blender did — refuse loudly and write nothing — is the same on both.

Network folders still could not be tested on either machine — see *Cannot be run* in both hand-test lists.

**Operating system:** the parts of this answer that could be tested are now settled on macOS and on Linux, and they agreed on what happens, differing only in the wording of the failure message. Windows is untested, and network folders are untested everywhere. (Linux entry 11.)

---

## 6. Can several outputs be gathered into one file rather than written separately?

**Label: documented.**

Yes, as layers of one OpenEXR file.

> "Media Type
> Determines how multiple inputs are stored:
> Image:
> Saves each input as a separate image file. Each input can use its own image format settings.
> Multi-Layer EXR:
> Saves all inputs together in a single multi-layer OpenEXR file."

> "When using Multi-Layer EXR, each input is stored as a separate layer within the file."

And the format panel changes accordingly:

> "Multi-Layer EXR: See OpenEXR for EXR-specific settings."

The manual's OpenEXR notes say why this format can do it:

> "An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

The API's shared media-type enum names the same choice:

> "media_type
> The type of media to save
> Type:
> enum in ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'], default 'IMAGE'"

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Manual, https://docs.blender.org/manual/en/5.0/files/media/image_formats.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html
- Date read: 2026-07-30

**Not covered.** Whether some outputs can be gathered while others stay separate on the same node — Media Type reads as one choice for the whole node. What the layers are named inside the file, beyond the manual saying the socket name is used to build the file name. Whether per-output format overrides do anything in Multi-Layer EXR mode; the manual attaches the override to "When saving individual images", which suggests it does not, but does not say.

**Driven from outside.** The setting is `format.media_type` on the node, reachable without the interface. No difference stated.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Confirmed by running it**, and two things the sources left open are now settled. **The layers are named after the outputs** — three outputs named `first`, `second` and `third` produced one file whose header text carries `'first.A' … 'second.A' … 'third.A'` (quoted under question 1). **The single file takes its name from the node's File Name alone**, with no output name in it: it was `out/file_name.exr` whether the folder was written with a trailing slash or without one.

**The two settings are not independent**: with the media type on Image, the node refused the multi-layer format outright —

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

— so the media type is what switches the node between the two ways of writing, and the format list follows it. (Hand tests 5 and 6.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts.** Three outputs became one file whose layers are named after them (quoted under question 1). The single file took its name from the node's File Name alone — `out/file_name.exr` whether the folder was written with a trailing slash or without one. And with the media type on Image, the node refused the multi-layer format outright, in the same words:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 5 and 6.)

---

## 7. What happens to a File Output node inside a group of nodes?

**Label: not in the sources.**

Nothing in the 5.0 sources describes a File Output node placed inside a node group.

**Where I looked, and what I ruled out.**

- The whole File Output Node manual page (https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html). It never mentions groups.
- The Node Groups manual page (https://docs.blender.org/manual/en/5.0/interface/controls/nodes/groups.html). It carries general advice about output nodes, but names a different node: "Avoid using nodes output nodes such as Material Output inside node groups. These should be used on the top level node tree to improve re-usability of node groups." and "Use Group Output to pass data out of a node group." That is guidance for a shading output node, and it says "avoid", not what happens. I am not stretching it to cover File Output.
- The Compositor System manual page (https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html), which is the page that does describe how the compositor picks output nodes inside and outside groups: "The compositor only supports a single active output target, that is, only one of the Group Output nodes or Viewer nodes in the node tree will be considered active and the rest will be ignored. In particular, the compositor searches the Active Node Tree Context and falls back to the Root Node Tree Context if no active output was found in the active node tree context. The active node tree context is the node tree of an expanded node group, that is, when the user selects a node group node and edits its underlying tree, while the root node tree context is the top level node tree without any expanded node groups." That rule is stated for Group Output and Viewer nodes only. File Output is not named in it, and I will not assume it is covered.
- The compositor release notes (https://developer.blender.org/docs/release_notes/5.0/compositor/), including the whole File Output redesign entry, and the Python API release notes (https://developer.blender.org/docs/release_notes/5.0/python_api/).
- Whole-manual searches for "file output" — it appears in only five files, all read: the genindex, two contents listings, the node page itself, and the File Paths page.

- Date read: 2026-07-30

**Not covered.** Everything. In particular: whether a File Output node inside a group writes at all; whether it writes once per use of the group or once in total; whether the `{node_name}` template variable resolves to the node's name inside the group or to the group node's name.

**Driven from outside.** Not settled, since nothing is settled. Worth noting that 5.0 makes the compositing tree itself a node group — "The compositing node tree is now its own data block, making it reusable across blend files." (Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/) and the scene points at it through `compositing_node_group` (Python API, https://docs.blender.org/api/5.0/bpy.types.Scene.html, "compositing_node_group / Compositor Nodes / Type: NodeTree"), so "inside a group" is now the ordinary case for the top level too. The sources do not draw out what that means for File Output.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes, once, wherever it sits.** A File Output node placed inside a node group wrote its file; using the same group twice still produced one file; and putting that group inside a second group made no difference:

```
RESULT [5.0.1] one File Output node inside a group, the group used once
    out/g_grouped.png  757 bytes
RESULT [5.0.1] the same group used twice in the tree
    out/g_grouped.png  757 bytes
RESULT [5.0.1] the File Output node two groups deep
    out/g_grouped.png  757 bytes
```

Muting the group node from outside stopped it:

```
RESULT [5.0.1] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases. Note what the second case does not settle: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like. (Hand test 10.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [5.0.1] one File Output node inside a group, the group used once
    out/g_grouped.png  738 bytes
RESULT [5.0.1] the same group used twice in the tree
    out/g_grouped.png  738 bytes
RESULT [5.0.1] the File Output node two groups deep
    out/g_grouped.png  738 bytes
RESULT [5.0.1] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases, as on the Mac. The same caution applies: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 10.)

---

## 8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?

**Label: not in the sources** for whether either can be switched off on its own. **Documented** only for the existence of the generic node mute control and the scene-wide compositing control.

**The node as a whole.** Every node can be muted. The manual:

> "Mute
> Reference
> Menu:
> Node ‣ Show/Hide ‣ Mute
> Shortcut:
> M
> Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

and the property exists on every node:

> "mute
> Type:
> boolean, default False"

Those words do not say whether muting stops a File Output node's file-writing side effect. The generic mute control is therefore a lead for a test, not an answer to whether this node can be switched off on its own.

There is also a switch that turns off all compositing at once, which is not the same thing:

> "use_compositing
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
> Type:
> boolean, default True"

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

**An individual output.** `NodeCompositorFileOutputItem` has no enable, mute, active or skip property — its full list is `color`, `format`, `name`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions`. The generic socket type does carry switches —

> "enabled
> Enable the socket
> Type:
> boolean, default False"
> "hide
> Hide the socket
> Type:
> boolean, default False"

— but nothing in the sources says that turning either of those off stops a File Output node writing that output's file, so I will not claim it does.

The one thing the manual ties writing to is connection: "Each **connected** input is saved to the specified directory and filename". This makes disconnecting an input a lead for a test. It does not state what an unconnected input does, so it does not establish a way to switch one output off.

I also checked the Enable Output Node, because its name suggests it. It is a different thing: "The Enable Output node toggles the visibility and value of a socket for a Node Group outputs." It acts on node *group* outputs, not on File Output.

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/enable_output.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.Node.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeSocket.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItem.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**Not covered.** Whether muting actually stops the writing. Whether a disabled or hidden socket writes. Whether an unconnected output produces an empty file, a black file, or nothing.

**Driven from outside.** `mute` is a plain property, so it can be set without the interface; the manual describes it through a menu item and a keyboard shortcut, which are interface routes to the same property. `use_compositing` is likewise a plain property. Nothing states a difference.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Muting the node does stop it writing**, which this answer had as unresolved. Starting from two connected outputs that both wrote:

```
RESULT [5.0.1] both outputs connected
    out/file_nameA.png  757 bytes
    out/file_nameB.png  745 bytes
RESULT [5.0.1] the whole node muted (node.mute = True)
    (nothing written)
```

**One output can be switched off on its own, through its link.** Muting only the link feeding output A, and then removing that link while leaving the output on the node, both left B alone:

```
RESULT [5.0.1] node unmuted, the link into output A muted (link.is_muted = True)
    out/file_nameB.png  745 bytes
RESULT [5.0.1] the link into output A removed, the output itself left on the node
    out/file_nameB.png  745 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
```

So an unconnected output writes nothing at all — not an empty file, not a black one. The socket's own `enabled` setting was `True` throughout and was not what did this. Whether the panel offers a per-output switch of its own is test 17 on the Creator's half. (Hand test 9.)


**Observed by hand test, from Blender's own window (2026-08-02).** **There is no per-output on/off control on the panel.** The Images list offers a plus, a minus and a pair of reorder arrows; right-clicking an entry gives Blender's ordinary property menu — Reset to Default Value, Copy to Selected, Copy Data Path, Copy Full Data Path, Copy as New Driver, Online Manual — and nothing that switches an output off. The only per-output controls are Socket Type, Override Node Format and the format settings under it. So removing an output, or cutting or muting its link as under the test above, is the way. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [5.0.1] both outputs connected
    out/file_nameA.png  738 bytes
    out/file_nameB.png  751 bytes
RESULT [5.0.1] the whole node muted (node.mute = True)
    (nothing written)
RESULT [5.0.1] node unmuted, the link into output A muted (link.is_muted = True)
    out/file_nameB.png  751 bytes
RESULT [5.0.1] the link into output A removed, the output itself left on the node
    out/file_nameB.png  751 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B', '']
SOCKET 'enabled' ON OUTPUT A: True
```

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 9 and 10.)

---

## 9. What restrictions, if any, apply to the kind of picture a File Output node can write?

**Label: documented** for the listed picture and file restrictions. **Not in the sources** for whether video can be written.

**Media types described for the node.** The manual's File Output page describes two Media Types — Image and Multi-Layer EXR — quoted in full in answer 6. The underlying enum has three values, `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`, and the scene's own Output panel documents all three:

> "Media Type
> The type of media to save.
> Image: ...
> Multi-Layer EXR: ...
> Video:
> Encodes each frame into a video container. Encoding options can be found in the Encoding panel."

The node's page omits Video, but omission does not establish that the node cannot write it, especially because the underlying property permits the value. Video is unresolved; see question 11.

**Which file formats.** The node's page defers to the general format documentation: "See Saving Images for details on supported formats and options." The API's format list is:

> "file_format
> File format to save the rendered images as
> Type:
> enum in Image Type All Items, default 'PNG'"

and that enum names: JPEG (.jpg), OpenEXR (.exr), PNG (.png), WebP (.webp), Bitmap (.bmp), Cineon (.cin), DPX (.dpx), Iris (.rgb), JPEG 2000 (.jp2), Radiance HDR (.hdr), Targa (.tga), Targa Raw (.tga), TIFF (.tif), OpenEXR MultiLayer (.exr), FFmpeg Video.

**Restrictions on channels and depth.** The manual states these plainly:

> "Note, RGBA is not available for all image formats, check the list above for details."

> "Note, not all file formats support all color depths."

> "Float (Half):
> 16-bit floating point per channel. Offers high dynamic range with lower memory and storage usage. Only supported for OpenEXR files.
> Float (Full):
> 32-bit floating point per channel. Provides the highest precision and dynamic range. Only supported for OpenEXR files."

> "Internally, Blender only operates in either 8-bit or 32-bit. Images with higher than 8-bit precision (e.g., 10-bit, 12-bit, 16-bit) are converted to 32-bit float when loaded into Blender."

> "DPX as well as Cineon only stores and converts the "visible" color range of values between 0.0 and 1.0 (as a result of rendering or composite)."

> "Note, Cineon, DPX, OpenEXR, and Radiance HDR image types default to being saved in a linear color space."

The manual's format table gives, per format, the channel depth, whether alpha is carried, whether metadata is carried, whether DPI is carried, and the extensions — for example JPEG at 8 bit with no alpha, OpenEXR at float 16 and 32 bit with alpha, PNG at 8 and 16 bit with alpha, WebP at 8 bit with alpha and no DPI.

**A restriction on the picture data itself, upstream of the file.** The compositor's own data is described as:

> "Three types of data exist, all of which are stored in half precision formats:"

and precision is a compositor setting:

> "Precision GPU
> The precision of compositor intermediate result.
> Auto:
> Use full precision for final renders, half precision otherwise.
> Full:
> Use full precision for final renders and viewport."

**A restriction on size and position.** The node is an output node, so its area is fixed to the render size:

> "The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size."

and the release notes state a change that follows from it:

> "- The **File Output**, **Map UV**, **Corner Pin**, **Plane Track Deform**, and **Bokeh Blur** nodes now take translation into consideration and relevant inputs are clipped if translated."

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Manual, https://docs.blender.org/manual/en/5.0/files/media/image_formats.html
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/sidebar.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/
- Date read: 2026-07-30

**Not covered.** Whether every format in the Image Type All Items enum is actually offered on this node — the node's page does not list them, it points at the general page, and I have not verified the node's own list. Whether the file extension is always added, or whether the scene's "Saving – File Extensions" setting (`use_file_extension`, a `RenderSettings` property) governs the node. Whether stereo or multi-view output works from this node — `views_format` and `stereo_3d_format` exist on `ImageFormatSettings`, but nothing ties them to this node. Any maximum image size.

**Driven from outside.** The format restrictions are properties of the file formats and of `ImageFormatSettings`, so they hold regardless of how Blender is driven. The one item that names a difference is compositor precision: "Use full precision for final renders, half precision otherwise" — a final render is what a machine-driven render does.

**Operating system.** The sources do not say for the formats.


**Observed by hand test (2026-08-02).** **The node's own format list is narrower than the shared one, and what it will take depends on the media type.** The setting's published list of values is

```
FILE FORMAT VALUES: ['JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
```

but with the node on Image media type it refused both the multi-layer value and the video value, and each refusal names what it will take:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

So on Image media type the node offers thirteen still-picture formats and neither the multi-layer nor the video value. What the panel draws is test 17 on the Creator's half. (Hand tests 1, 5 and 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The File Format menu on the node draws exactly the thirteen formats the setting will take, and no more.** In the order shown: JPEG (.jpg), OpenEXR (.exr), PNG (.png), WebP (.webp), then a divider, then Bitmap (.bmp), Cineon (.cin), DPX (.dpx), Iris (.rgb), JPEG 2000 (.jp2), Radiance HDR (.hdr), Targa (.tga), Targa Raw (.tga), TIFF (.tif) — thirteen in all. **OpenEXR MultiLayer and FFmpeg Video are both absent from it**, which is the same as what the setting refused. So what the panel offers and what the setting accepts agree, and the multi-layer choice is made with the Image / Multi-Layer EXR buttons above rather than from this menu. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **The Linux build of this version offers exactly the same formats as the Mac build.** The setting's published list:

```
FILE FORMAT VALUES: ['JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
MEDIA TYPE VALUES: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
```

and, with the node on Image media type, the same two refusals naming the same thirteen:

```
WITH media_type='IMAGE', SETTING file_format='OPEN_EXR_MULTILAYER' RAISED: TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

**This narrows the build question without closing it.** The sources say some formats are compiled in and "not available on all systems". Two builds of this version have now been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same thirteen. That is two builds, not all builds. Note also that **AVIF is absent from both**, and appears only from 5.1 onwards.

**Operating system:** the format list is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entries 1, 5 and 12.)

---

## 10. What happens when two outputs are aimed at the same name in the same place?

**Label: not in the sources.**

Nothing in the 5.0 sources says what happens when two outputs of a File Output node — or two File Output nodes — resolve to the same file path. Not whether one overwrites the other, not whether a number is appended, not whether it is an error, not what order they are written in.

**Where I looked.**

- The whole File Output Node manual page (https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html). The Output Paths panel is the only place that touches finished paths: "Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly." It shows the paths; it does not say what a clash does.
- `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, `NodeCompositorFileOutputItems` (all three API pages listed above). No uniqueness rule, no error, no note on the `name` property.
- The scene render Output panel (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html), which does document an overwrite rule — "Overwrite existing files when rendering" and "Create empty placeholder frames while rendering" — but for the *scene* render output path. The matching properties `use_overwrite` and `use_placeholder` live on `RenderSettings` (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html), not on the node, and nothing says the node consults them.
- The File Paths manual page (https://docs.blender.org/manual/en/5.0/files/file_paths.html). It covers template errors only.
- Release notes: compositor, Python API, core, pipeline & I/O, bugfixes (https://developer.blender.org/docs/release_notes/5.0/).
- Whole-source searches across manual, API and release notes for: "clobber", "already exists", "existing file", "same name", "unique name", "duplicate", "conflict", "collision", "atomic", "partial file", "temporary file", "flush", "write order". The only name-clash rule I found anywhere is Blender's rule for naming *data-blocks* — "When Blender has to name a new data-block, or rename an existing one, it will check for name collisions" (https://docs.blender.org/manual/en/5.0/files/data_blocks.html) — which is about data inside the blend file, not about files on disk, and I am not carrying it across.

- Date read: 2026-07-30

**Not covered.** All of it. This is a good candidate for the hands-on list at the end.

**Driven from outside.** Not settled. Note that the interface has the Output Paths panel, which shows the finished paths and would let a person see a clash coming; nothing equivalent is documented for use from outside.

**Operating system.** The sources do not say. Behaviour when two writes race for one path is the sort of thing that can differ by operating system, and the sources make no claim either way.


**Observed by hand test (2026-08-02).** **Two outputs on one node cannot carry the same name — the second is renamed.** Both were asked to be called `clash`; the node kept `['clash', 'clash_001']`, and two files were written, each holding its own picture:

```
RESULT [5.0.1] two outputs on one node, both asked to be called 'clash'
    out/file_nameclash.png  757 bytes
    out/file_nameclash_001.png  745 bytes
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
```

**Two separate nodes aimed at the same file give one file, and it holds the first node's picture.** No warning or error was printed:

```
RESULT [5.0.1] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  757 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
```

Building the two nodes in the opposite order swapped which picture survived, so it follows the order the nodes were made in:

```
RESULT [5.0.1] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  745 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

**A file already on disk is replaced silently, and the scene's Overwrite setting does not reach the node:**

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785689674.929
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1785689675.270
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785689675.622
```

**Placeholders do not reach it either.** A two-frame animation with `render.use_placeholder` on produced the node's files and the scene's files and no placeholder for the node. (Hand tests 7 and 8.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The Output Paths panel flags nothing, because there is nothing to flag — Blender renames the second output before a clash can happen.** With two outputs both asked to be called `clash`, the Images list shows `clash` and `clash_001`, and the Output Paths panel lists two different finished paths, ending `shot_clash.png` and `shot_clash_001.png`. Nothing is coloured red and no warning mark appears. (Hand test 18, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same answers, pixel value for pixel value.**

```
ONE NODE, TWO OUTPUTS BOTH ASKED FOR 'clash' -> node kept ['clash', 'clash_001']
RESULT [5.0.1] two outputs on one node, both asked to be called 'clash'
    out/file_nameclash.png  738 bytes
    out/file_nameclash_001.png  751 bytes
    PIXEL 0 OF file_nameclash.png = (0.043, 0.043, 0.043, 1.0)
    PIXEL 0 OF file_nameclash_001.png = (0.544, 0.544, 0.544, 1.0)
RESULT [5.0.1] two separate File Output nodes, same folder, same output name
    out/file_namesame.png  738 bytes
    PIXEL 0 OF file_namesame.png = (0.043, 0.043, 0.043, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('file_namesame.png', (0.043, 0.043, 0.043, 1.0))]
RESULT [5.0.1] the two nodes made in the opposite order (inverted picture's node made first)
    out/file_namesame.png  751 bytes
    PIXEL 0 OF file_namesame.png = (0.544, 0.544, 0.544, 1.0)
```

A file already on disk was replaced silently, and switching the scene's Overwrite off changed nothing:

```
    PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785995956.032
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.544, 0.544, 0.544, 1.0)   last written at 1785995956.542
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.043, 0.043, 0.043, 1.0)   last written at 1785995956.745
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

**Label: not in the sources.**

The sources do not settle whether the node can write video. Its manual page describes two media types and omits video, while the underlying property permits a video value.

The File Output Node manual page lists Media Type with two entries:

> "Media Type
> Determines how multiple inputs are stored:
> Image:
> Saves each input as a separate image file. Each input can use its own image format settings.
> Multi-Layer EXR:
> Saves all inputs together in a single multi-layer OpenEXR file."

The scene's Output panel documents the same setting with three entries — the same two, plus:

> "Video:
> Encodes each frame into a video container. Encoding options can be found in the Encoding panel."

The underlying property allows video, since node and item both hold an `ImageFormatSettings`, and that type's `media_type` is "enum in ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'], default 'IMAGE'", while `file_format` is "enum in Image Type All Items", an enum that includes "FFMPEG: FFmpeg Video." What the property permits and what the node does with it are different claims. The sources do not say what the node does with this value.

The node's naming pattern is described as one file per frame — "with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}". That does not settle what happens when the video value is set.

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html
- Date read: 2026-07-30

**Where I looked.** The File Output Node manual page, the scene Output panel, and `ImageFormatSettings`, including its `media_type` and `file_format` entries. The first omits video for this node; the second describes video for scene output; the API permits video values but does not describe what this node does with them. These sources pull in different directions and do not settle the behaviour.

**Not covered.** Whether the node can write video; what happens if `format.media_type` or `format.file_format` is set to a video value; whether the node has any encoding settings; whether audio is possible.

**Driven from outside.** The sources do not say what happens when the video value is set from outside. This is the unresolved case described above.

**Operating system.** The sources do not say. Video encoding in general is documented with a note that codecs and containers vary in compatibility — "Some containers and codecs are not compatible with each other" — but nothing operating-system-specific for this node.


**Observed by hand test (2026-08-02).** **No. The node refuses the video value.** The shared format block does carry three media types, but this node will not take the third:

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
```

The refusal names only Image and Multi-Layer EXR — the same two the manual describes for this node. The node stayed on PNG and a three-frame animation produced three pictures:

```
RESULT [5.0.1] node set to video, animation over frames 1 to 3
    out/file_nameclip0001.png  755 bytes
    out/file_nameclip0002.png  755 bytes
    out/file_nameclip0003.png  755 bytes
```

This is the outside-the-interface case this answer said the sources did not cover: setting the value from outside is refused, not silently accepted. What the panel offers on screen is test 17 on the Creator's half. (Hand test 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The panel offers no video choice.** Media Type is drawn on this node as two buttons, **Image** and **Multi-Layer EXR**, not as a menu with a third entry — and the File Format menu under it holds no FFmpeg Video (listed under question 9). So the panel and the setting agree, and they agree with the manual describing only two media types for this node: video cannot be chosen here, by hand or from outside. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux refused it too, in the same words.**

```
SETTING media_type='VIDEO' RAISED: TypeError('bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')')
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')')
RESULT [5.0.1] node set to video, animation over frames 1 to 3
    out/file_nameclip0001.png  736 bytes
    out/file_nameclip0002.png  736 bytes
    out/file_nameclip0003.png  736 bytes
```

The refusal names only Image and Multi-Layer EXR there too — the same two the manual describes for this node.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entry 12.)

---

## 12. What can a File Output node be fed, and does it have to come from the render?

**Label: documented.**

**What it can be fed.** The manual says it can save intermediate results from anywhere in the node tree and describes links from other nodes:

> "Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel."

> "Save intermediate results from anywhere in the node tree."

> "Combine with the Cryptomatte Node or other utility passes to export custom data for compositing pipelines."

Each input has a declared data type:

> "socket_type
> Type:
> enum in Node Socket Data Type Items, default 'FLOAT'"
> "vector_socket_dimensions
> Dimensions of the vector socket
> Type:
> int in [2, 4], default 0"

and the redesign note says the types are fixed once made:

> "- The **File Output** node was redesigned for a better user experience.
>   - It now supports dragging links to create a new input.
>   - The final paths of the images are displayed in the UI to remove guess work.
>   - The base path is now split into a Directory and a File Name.
>   - Socket types are not set on the node and do not change.
>   - Other UI improvements."

The compositor as a whole carries three data types, and converts between them:

> "Three types of data exist, all of which are stored in half precision formats:
> Float ... Vector ... Color"

> "In case a node input is given data of type other than its own type, the following implicit conversions are performed:"

**It does not have to come from the render.** The compositor's input nodes include sources that have nothing to do with rendering:

> "Input nodes produce information from a data source. For instance, an input can be:
> Taken directly from the active camera in a selected scene.
> A static image.
> A movie clip (such as an image sequence or video).
> A color or value."

The listed input nodes are Constant, Group Input Node, Bokeh Image Node, Image Node, Image Info Node, Image Coordinates Node, Mask Node, Movie Clip Node, and Scene. And the manual's overview says the same: "you use nodes to import the image into Blender, change it, optionally merge it with other images, and finally, save it."

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/input/index.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.NodeCompositorFileOutputItem.html
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/
- Date read: 2026-07-30

**Not covered.** `socket_type` is annotated with the shared Node Socket Data Type Items enum, which lists every socket type Blender has, including Geometry, Object, Collection, Material and Closure. That enum is shared across many node systems. Nothing states which of its values a File Output node actually accepts, and I am not claiming it accepts all of them. The compositor's own data types are stated to be three — Float, Vector, Color — which suggests a narrower set, but that is my reading, not a statement about this node. Also not covered: whether the compositor runs at all when nothing renders, and therefore whether a File Output node fed only from an image file writes without a render happening.

**Driven from outside.** Dragging a link to make an input is an interface action; `file_output_items.new(socket_type, name)` is the equivalent from outside. No difference is stated in what may be fed in.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It has to be fed a picture. A plain colour writes nothing.** One node with two outputs, one fed by a colour node and one fed by the render through an Invert node, wrote only the second — with no error and with the link reported valid:

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [5.0.1] one output fed by a colour node, one by an image
    out/file_namepicture.png  745 bytes
```

That settles the "dimensionless single value" case this answer left open: nothing is written for it. It does not have to come from the render, but it does have to be an image — the picture that was written came through an Invert node rather than straight from the render. (Hand test 13.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux did the same**, and one thing this answer left open was settled there for the first time.

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [5.0.1] one output fed by a colour node, one by an image
    out/file_namepicture.png  751 bytes
```

**Which of the socket kinds the node will actually accept**, which this answer said the sources do not state and which was never tested on the Mac. An output was asked for of every one of the eighteen kinds the setting publishes. **Only three could be made at all:**

```
OUTPUTS THE NODE ACCEPTED: [('k_FLOAT', 'FLOAT'), ('k_VECTOR', 'VECTOR'), ('k_RGBA', 'RGBA')]
```

Each of the other fifteen was refused with the same message, for example:

```
ADDING AN OUTPUT OF KIND MATRIX RAISED: RuntimeError('Error: Unable to create item with this socket type\n')
```

All three that were made took the render picture, and all three wrote a file:

```
LINKS THE TREE ACTUALLY HOLDS INTO THE NODE: ['k_FLOAT', 'k_RGBA', 'k_VECTOR']
RESULT [5.0.1] one written output of every kind the node offers, all fed the render picture
    out/file_namek_FLOAT.png  689 bytes
    out/file_namek_RGBA.png  738 bytes
    out/file_namek_VECTOR.png  738 bytes
```

The Float file is smaller than the other two; the Vector and Colour files are the same size as each other. What this does **not** settle: what a Float or a Vector output writes when it is fed something other than a picture, and what channels are actually inside each written file — they were not read back.

**Operating system:** the colour-versus-picture part is now settled on macOS and on Linux, and they agreed; which socket kinds the node accepts is settled on Linux alone. Windows is untested. (Linux entries 13 and 21.)

---

## 13. What causes a File Output node to write, and when during a render does it write each file?

**Label: documented** for what causes it. **Not in the sources** for when during a render.

**What causes it.** Three things, each stated.

First, a render or a composite:

> "The File Output node writes one or more images to disk during rendering or compositing."

Second, the input must be connected:

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence."

Third, compositing must be switched on for the scene:

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

> "Pipeline – Compositing
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

> "use_compositing
> Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
> Type:
> boolean, default True"

And a scene needs a compositing node group assigned:

> "compositing_node_group
> Compositor Nodes
> Type:
> NodeTree"

**Whether it is a single render or an animation render changes the file name.** The release notes:

> "- The **File Output** node now only appends the frame number to the image name when doing an animation render. Rendering through the command line always does an animation render, so no change of behavior is expected. Rendering using the `render.render()` operator when `animation=False` will see the change of behavior, but setting that to True and providing `start_frame` and `end_frame` should restore the old behavior. Finally, adding `####` manually in the name should substitute the frame number even in single renders."

The matching bug-fix entry names the same change: "Compositor File Output: Optional frame numbering". The operator's own parameters match the note:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
> "animation (boolean, (optional)) – Animation, Render files from the animation range of this scene"
> "frame_start (int in [-inf, inf], (optional)) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render"

**When during a render.** Not stated. The manual says "during rendering or compositing" and nothing more precise. It does not say whether the file is written before or after the main render result is saved, whether it is written per frame as each frame finishes or at the end of a run, whether it is written before or after the Video Sequencer stage, or whether the file is complete and closed at any particular moment.

**Where I looked for the timing.** The File Output Node page in full; the Post Processing page in full (https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html); the Compositor System page in full, including its Output section; the scene Output properties page in full; the command-line rendering page and the command-line arguments page (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html); `bpy.ops.render` (https://docs.blender.org/api/5.0/bpy.ops.render.html), where `render.render` is marked "Undocumented, consider contributing."; the compositor and bugfix release notes. Whole-source searches for "write order", "after each frame", "flush", "atomic", "partial file", "temporary file".

- Exact words quoted above.
- Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html
- Manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Python API, https://docs.blender.org/api/5.0/bpy.types.Scene.html
- Python API, https://docs.blender.org/api/5.0/bpy.ops.render.html
- Release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/
- Release notes, https://developer.blender.org/docs/release_notes/5.0/bugfixes/
- Date read: 2026-07-30

**Not covered.** The timing, as above. Whether the node writes during viewport compositing as well as during a final render — the Compositor System page distinguishes "viewport compositing" from "final render compositing" for image size, but says nothing about File Output writing in the viewport. Whether the node writes when the compositor is run as a Video Sequencer strip modifier — 5.0 added one ("A new **Compositor** sequencer strip modifier was added.", release notes, compositor; and the manual's Compositor Modifier section), and nothing says what File Output does inside it. Whether the node writes when the render is cancelled part way.

**Driven from outside.** This is the one answer where the sources speak directly to it, and it is a real difference. The release note is explicit: "Rendering through the command line always does an animation render, so no change of behavior is expected. Rendering using the `render.render()` operator when `animation=False` will see the change of behavior". So a machine driving Blender through `render.render()` with `animation=False` gets file names without frame numbers unless `####` is put in the name by hand. Driving Blender from the command line does not hit this.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes when Blender is run from a terminal with no window.** From a saved file:

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

Note that `-f 1` — a single frame from the command line — still gave `file_namebeauty0001.png` with a frame number, which is what the release note quoted above says should happen.

**The frame-number rule the release note describes is confirmed.** Driven through the render operator:

```
RESULT [5.0.1] render.render(animation=False)
    out/file_namehash0001here.png  755 bytes
    out/file_nameplain.png  755 bytes
RESULT [5.0.1] render.render(animation=True), frame range 1 to 1
    out/file_namehash0001here.png  755 bytes
    out/file_nameplain0001.png  755 bytes
```

`plain` gains a frame number only on the animation render; `hash####here` carries one either way.

**Each file is written as its frame finishes, just before the scene's own file for that frame.** Over a four-frame animation with both writing into one folder, the times a file was last written were:

```
    1785690006.996  out/file_namenode0001.png
    1785690006.998  out/scene_0001.png
    1785690008.186  out/file_namenode0002.png
    1785690008.189  out/scene_0002.png
    1785690009.425  out/file_namenode0003.png
    1785690009.428  out/scene_0003.png
    1785690010.682  out/file_namenode0004.png
    1785690010.685  out/scene_0004.png
```

Frame 1's files carry a time nearly four seconds before frame 4's, so nothing waited for the end of the run.

**Switching compositing off stops it; having no Group Output node in the tree does not:**

```
RESULT [5.0.1] the same tree with render.use_compositing = False
    (nothing written)
RESULT [5.0.1] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  745 bytes
```

Whether a single frame rendered from the window writes, and whether viewport compositing writes, are tests 19 and 20 on the Creator's half. (Hand tests 2, 3, 13 and 15.)


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

**It writes when Blender is run from a terminal with no window**, from a saved file:

```
--- blender -b fo_scene.blend -f 1  (Blender 5.0)
00:01.069  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0001.png'
00:01.069  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
--- blender -b fo_scene.blend -a  (Blender 5.0)
00:00.967  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0001.png'
00:00.967  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
00:01.079  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0002.png'
00:01.079  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0002.png'
00:01.146  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/file_namebeauty0003.png'
00:01.146  render           | Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0003.png'
```

`-f 1` gave a name carrying a frame number there too, which is what the release note quoted above says should happen.

**The frame-number rule the release note describes holds on Linux as well:**

```
RESULT [5.0.1] render.render(animation=False)
    out/file_namehash0001here.png  736 bytes
    out/file_nameplain.png  736 bytes
RESULT [5.0.1] render.render(animation=True), frame range 1 to 1
    out/file_namehash0001here.png  736 bytes
    out/file_nameplain0001.png  736 bytes
```

**Each file is written as its frame finishes, just before the scene's own file for that frame:**

```
FILES IN THE ORDER THEY WERE LAST WRITTEN (seconds since the epoch):
    1785996150.337  out/file_namenode0001.png
    1785996150.345  out/scene_0001.png
    1785996151.833  out/file_namenode0002.png
    1785996151.836  out/scene_0002.png
    1785996153.206  out/file_namenode0003.png
    1785996153.215  out/scene_0003.png
    1785996154.728  out/file_namenode0004.png
    1785996154.735  out/scene_0004.png
```

That machine is shared with other work, so no claim is made about how long any of it took; what the test shows is the **order**, and the order is the same on both machines.

**Switching compositing off stops it; having no Group Output node in the tree does not:**

```
RESULT [5.0.1] the same tree with render.use_compositing = False
    (nothing written)
RESULT [5.0.1] compositing back on, but the tree's terminal output node removed
    out/file_namepicture.png  751 bytes
```

**What a stopped render leaves behind, which was never tested on the Mac.** A sixty-frame animation was started from the command line and stopped part way, twice: once with an interrupt — the polite stop, what pressing Ctrl-C does — and once with a hard kill that gives Blender no chance to tidy up. **Both left whole files for every frame that had finished, and nothing at all for the frame that was in flight.** No part-written file, no empty file, no file of the wrong size, and the node's files and the scene's files always in matching pairs. Stopped with an interrupt it left thirty-four pairs, frames 0001 to 0034, every node file 8471 bytes and every scene file 10851 bytes, with frame 35 — the one being rendered — leaving nothing. Stopped with a hard kill it left twenty-seven pairs, again all whole, with frame 28 leaving nothing. The two counts differ only because the machine got through different numbers of frames before the stop arrived; that machine is shared, so the counts are not a measure of its speed.

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

2026-08-02 — Opus 5 (Claude Code) — File output nodes, Blender 5.0

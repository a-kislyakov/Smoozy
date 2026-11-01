# File output nodes — Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, and Blender 4.5 release notes, all as plain-text copies taken on 2026-07-30, read by me on 2026-08-02. Two manual images were opened (listed where used). Cycles' own files (the 4.5 add-on folder supplied) were searched and contain nothing about this topic.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## A note that applies to the whole sheet

The 4.5 manual devotes one short page to this node. It describes the node's two front-facing settings and says an image is written on rendering. Almost everything else about the node — its list of outputs, the per-output settings, what happens on a clash, when exactly a file lands — is described only in the Python API reference, which describes *settings that exist*, not *behaviour*. So several answers below are "not in the sources", and several others are "inferred" from a setting's name and description rather than from a sentence about behaviour. I have kept those two apart carefully.

Addresses used repeatedly below:

- Manual, File Output Node: https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html
- Manual, File Paths (path templates): https://docs.blender.org/manual/en/4.5/files/file_paths.html
- Manual, Output panel: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- API, CompositorNodeOutputFile: https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html
- API, NodeOutputFileSlotFile: https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html
- API, NodeOutputFileSlotLayer: https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotLayer.html
- API, CompositorNodeOutputFileFileSlots: https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFileFileSlots.html
- API, ImageFormatSettings: https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html
- API, Image Type Items: https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html
- API, bpy.ops.node: https://docs.blender.org/api/4.5/bpy.ops.node.html

---

## Observed by hand test

Everything under this heading was settled by running Blender 4.5.12 LTS on the Creator's own computer — one MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2 — on **2026-08-02**. Blender was started from a terminal with no window, in the form

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup --python /tmp/run.py`

and driven through its Python support. The exact scripts, and which test number each result came from, are in `Audits/Blender Research/File output nodes/Hand tests.md`. The machine's own output is quoted below word for word; where a listing is long, the part that carries the answer is kept and the rest is said to be omitted.

A few of the blocks below are marked **from Blender's own window** instead. Those were run by the Creator on the same computer on the same day, by opening a prepared file, reading the node's panel, and rendering from the window. They are the tests that needed eyes on the screen.

---

## Observed by hand test on a second machine

On **2026-08-06** the same tests were run again on a second computer: **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; its screen is shared with other work. Blender 4.5.12 LTS there is the Linux x64 build from Blender's own download server, at `/opt/blender/blender-4.5.12-linux-x64/blender`, and it was started the same way:

`/opt/blender/blender-4.5.12-linux-x64/blender -b --factory-startup --python run.py`

with `BLENDER_USER_RESOURCES` pointed at a settings folder of its own, and with `DISPLAY` and `XAUTHORITY` unset so that the display's own `Authorization required` noise stayed out of the output. The scripts were the Mac's, word for word, with only the folder they write into changed. Two of them needed Blender's window, which was opened on that machine's own display and photographed with Blender's own screenshot command. The entries and the full output are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

Every question below therefore carries results from two machines. **A Mac result and a Linux result are two results and are never merged.** Where they agreed, both are named and the agreement is said plainly; where the machines differed, both are given and neither is chosen.

**These blocks update the two standing lines for every question they touch.** *Driven from outside*: every result not marked as coming from the window was produced with Blender driven from outside its own interface, with no window open, so each holds in that case; where a window result differs from the terminal one, both are given. That is now true on both machines. *Operating system*: every answer that carries both an *Observed by hand test* block and an *on the rented Linux machine* block below is settled on **macOS 26.5.2 on an Apple M1 Pro and on Ubuntu 22.04.5 LTS on this rented machine**, and on nothing else. **Windows was not tested at all**: no Windows machine has run it. Both hand-test lists record it under *Cannot be run*.

---

## 1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?

**Label: documented** (what it writes, what an output carries) **/ not in the sources** (any limit on how many).

**What it writes.** The node writes image files.

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "This node can be used as a way to automatically save the image after a render; In addition, since this node can be hooked in anywhere in the node tree, it can also save intermediate images automatically."

Source: manual, File Output Node page, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30.

The API states the node's purpose in one line:

> "Write image file to disk"

Source: API, CompositorNodeOutputFile, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

**How many outputs.** The node's inputs are called *slots*, and they are a collection that can be added to, removed from, cleared and reordered:

> "Collection of File Output node slots"
> "new(name) — Add a file slot to this node"
> "remove(socket) — Remove a file slot from this node"
> "clear() — Remove all file slots from this node"
> "move(from_index, to_index) — Move a file slot to another position"

Source: API, CompositorNodeOutputFileFileSlots, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFileFileSlots.html — read 2026-07-30. (The same four calls, with identical wording, appear on CompositorNodeOutputFileLayerSlots.)

There is a matching interface command:

> "bpy.ops.node.output_file_add_socket(*, file_path='') — Add a new input to a file output node"

Source: API, bpy.ops.node, https://docs.blender.org/api/4.5/bpy.ops.node.html — read 2026-07-30.

No maximum number is stated anywhere I searched. I searched the manual and the API text for "file output", "output file", "slot", "file_slots", "layer_slots" and "output_file_add_socket"; nothing gives a count or a ceiling.

**What each output carries.** The node has two parallel collections of slots, and each collection's member type is described:

> "file_slots — Type: CompositorNodeOutputFileFileSlots bpy_prop_collection of NodeOutputFileSlotFile, (readonly)"
> "layer_slots — Type: CompositorNodeOutputFileLayerSlots bpy_prop_collection of NodeOutputFileSlotLayer, (readonly)"

Source: API, CompositorNodeOutputFile, address above — read 2026-07-30.

> "Single layer file slot of the file output node" (NodeOutputFileSlotFile)

> "Multilayer slot of the file output node" (NodeOutputFileSlotLayer)

Sources: API, NodeOutputFileSlotFile and NodeOutputFileSlotLayer, addresses above — read 2026-07-30.

The thing flowing into a slot is an image:

> "Image — The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual, File Output Node page — read 2026-07-30.

I also opened the manual's picture of the node, `_images/node-types_CompositorNodeOutputFile.webp` (shown on the File Output Node page). **Documented from the image:** the node shows a header "File Output", a field labelled "Base Path:" containing `/tmp/` with a folder-browse button beside it, and one input socket labelled "Image" with the greyed word "WebP" to its right. That is a node with a single input slot in its default drawing, with the file format shown as a label next to the socket.

**Not covered.** No upper limit on slot count. Nothing about what happens if a slot is left unconnected (except one release-note bugfix line, quoted under question 12). Nothing about whether the two slot collections can both hold entries at once.

**Outside the interface.** The sources do not say. The slot collection and its `new`/`remove`/`clear`/`move` calls are Python API entries, which is evidence they can be driven without a person clicking, but the documentation does not state that the node behaves the same when Blender is driven from outside.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** Three things were settled. **The node has nothing on its output side**, confirming the manual's sentence from the running program: `NODE HAS OUTPUT SOCKETS: []`. **Both slot collections exist side by side** — the running 4.5 reports `TYPE NodeOutputFileSlotFile present=True`, `TYPE NodeOutputFileSlotLayer present=True`, and `TYPE NodeCompositorFileOutputItem present=False`. **No ceiling was reached on how many outputs a node takes**: adding outputs in a loop gave `OUTPUTS ADDED WITHOUT REFUSAL: 2000` and `INPUT SOCKETS ON THE NODE NOW: 2000`, with nothing refused. That establishes at least two thousand and says nothing about any higher limit. (Hand tests 1, 6 and 14.)


**Observed by hand test, from Blender's own window (2026-08-02).** Outputs are added from the panel with a button labelled **Add Input**, and the list of them sits below it with a pair of arrows for reordering. There is no control on the panel for what kind of thing an output carries. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **The same three things, and Linux agreed with the Mac on every one.** The running program there printed `NODE HAS OUTPUT SOCKETS: []`; `TYPE NodeOutputFileSlotFile present=True`, `TYPE NodeOutputFileSlotLayer present=True`, `TYPE NodeCompositorFileOutputItem present=False`; and, for how many outputs a node will take:

```
OUTPUTS ADDED WITHOUT REFUSAL: 2000
INPUT SOCKETS ON THE NODE NOW: 2000
```

**This version has no setting for what kind of thing an output carries at all**, which was checked directly on Linux: `SOCKET TYPES THE OUTPUT LIST OFFERS: []` and `4.5 HAS NO SOCKET TYPE SETTING ON A WRITTEN OUTPUT - NOTHING TO TRY`. So the question of what each kind writes, which the later versions raise, does not arise here.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 1, 14 and 21.)

---

## 2. Which settings belong to an individual output, and which belong to the node as a whole?

**Label: documented.**

**Belonging to the node as a whole** (properties of `CompositorNodeOutputFile`):

> "active_input_index — Active input index in details view list — Type: int in [-inf, inf], default 0"
> "base_path — Base output path for the image — Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)"
> "format — Type: ImageFormatSettings, (readonly)"
> "save_as_render — Apply render part of display transform when saving byte image — Type: boolean, default False"

Source: API, CompositorNodeOutputFile, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

**Belonging to an individual output** (properties of `NodeOutputFileSlotFile`):

> "format — Type: ImageFormatSettings, (readonly)"
> "path — Subpath used for this slot — Type: string, default "", (never None, Supports template expressions)"
> "save_as_render — Apply render part of display transform when saving byte image — Type: boolean, default False"
> "use_node_format — Type: boolean, default False"

Source: API, NodeOutputFileSlotFile, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html — read 2026-07-30.

The API lists one property specific to an individual output in the multilayer collection:

> "name — OpenEXR layer name used for this slot — Type: string, default "", (never None)"

Source: API, NodeOutputFileSlotLayer, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotLayer.html — read 2026-07-30.

So: the folder and the whole-node format live on the node; the per-output name/subpath, per-output format, per-output `save_as_render` setting, and a switch called `use_node_format` live on the output.

The manual splits it the same way but names only two settings, both on the node:

> "Base Path — Unlike the render output filepath, this node uses a base directory and an image name, by default the output path is composed of: {base path}/{file name}{frame number}.{extension}."

> "File Format — Label that shows the selected file format."

> "Note — More options can be set in the Sidebar region."

Source: manual, File Output Node page — read 2026-07-30.

**Not covered.** `use_node_format` carries no description in the API text — the source gives its name, its type and its default (`False`) and nothing more. I am not going to state what it does. The manual does not enumerate the sidebar options at all, so the correspondence between the API property names above and the labels a person sees is not established by these sources.

**Outside the interface.** The sources do not say. All the settings above are exposed as Python API properties, which is the same list either way, but no statement covers behaviour.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **The full list of settings, from the running program.** On the node itself: `base_path` ("Base output path for the image"), `active_input_index`, `format` (read-only), `save_as_render` ("Apply render part of display transform when saving byte image"), `file_slots` and `layer_slots`. On one written output of the single-image kind: `use_node_format`, `save_as_render`, `format`, and `path` ("Subpath used for this slot"). On one of the multi-layer kind: `name` ("OpenEXR layer name used for this slot"). The running program gives `use_node_format` no description either — it printed `use_node_format | BOOLEAN | description=''` — so what it is called on screen is still open, and is test 17 on the Creator's half.

**Which display-transform setting wins.** With eight combinations tried one at a time and the written pixel read back:

```
    node save_as_render=False  output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
```

While an output uses the node's format (`use_node_format` on), only the node's `save_as_render` changes the file. While it uses its own format (`use_node_format` off), only the output's own `save_as_render` changes it. (Hand tests 1 and 16.)


**Observed by hand test, from Blender's own window (2026-08-02).** **Every control the panel draws, with a File Output node selected and the sidebar open on the Node tab.** Under **Node**: Reset Node, Name, Label, Color. Under **Properties**, belonging to the node: `Base Path:` with a folder-browse button, **Save as Render**, **File Format**, **Color** (BW / RGB / RGBA), **Color Depth** (8 / 16), **Compression**, and **Color Management** (Follow Scene or Override; Override adds Display Device, View, Look, Exposure, Gamma, Use Curves and Use White Balance). Then **Add Input** and the list of outputs. Belonging to the selected output: **File Subpath:** with a cross to remove it, then **Format:** with a checkbox labelled **Use Node Format**, and beneath it the same Save as Render, File Format, Color, Color Depth, Compression and Color Management controls again.

So the setting the API reference calls `path` is drawn as **File Subpath**, and `use_node_format` is drawn as **Use Node Format**. There is **no Media Type control** anywhere on this node's panel, which agrees with the running program having no such setting.

**The Use Node Format checkbox has no tooltip at all** — nothing appears on hovering it. The API reference gives it no description either, and so does the running program. So what this control means is stated nowhere in Blender itself in this version; the only description of it anywhere is the one Blender 5.0 and later put on its replacement, **Override Node Format**: "Use a different format instead of the node format for this file." Note that the two are opposites — ticking Use Node Format is the same as leaving Override Node Format unticked. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux printed the same list of settings, with the same descriptions, and the same eight pixel values.** The node's own settings came back as `base_path`, `active_input_index`, `format`, `save_as_render`, `file_slots` and `layer_slots`; a written output of the single-image kind as `use_node_format` (with `description=''` there too), `save_as_render`, `format` and `path`; and one of the multi-layer kind as `name`. The display-transform test:

```
    node save_as_render=False  output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=False  output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=False  ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=False  output uses own format=True   ->  pixel 0 = (0.247, 0.247, 0.247, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=False  ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
    node save_as_render=True   output save_as_render=True   output uses own format=True   ->  pixel 0 = (0.227, 0.227, 0.227, 1.0)
```

Those are the Mac's eight values exactly. Colour handling is one of the things most likely to differ between machines, and here it does not.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 1 and 16.)


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **What the node draws on its own face.** The node's body carries a header with the node's label, a line reading `Base Path:`, the folder field with a folder-browse button beside it, and then one row for each written output — the output's name on the left and its file format in grey on the right. With two outputs called `clash` and `clash_001` the rows read `clash    PNG` and `clash_001    PNG`. There is no row on the node itself for adding an output. The Creator's Mac screenshots were of the sidebar rather than the node's face, so there is nothing here to compare against; this is the Linux picture and nothing more. (Linux entry 17.)

---

## 3. Does each output carry its own folder as well as its own name?

**Label: inferred.**

What the sources say outright: the node carries one folder, and each output carries a *subpath*.

> "base_path — Base output path for the image"

> "path — Subpath used for this slot"

Sources: API, CompositorNodeOutputFile and NodeOutputFileSlotFile, addresses above — read 2026-07-30.

> "Unlike the render output filepath, this node uses a base directory and an image name, by default the output path is composed of: {base path}/{file name}{frame number}.{extension}."

Source: manual, File Output Node page — read 2026-07-30.

**The step I took.** The folder property (`base_path`) exists once, on the node; there is no folder property on a slot. The only path-like thing a slot has is `path`, described as a *sub*path. From that I infer: each output does **not** carry its own independent folder; it carries a name (or sub-path) that sits underneath the node's single folder. A reader has to supply the assumption that "subpath" means "beneath the base path" — the sources never say so in a sentence. That is why this is inferred, not documented.

**Not covered.** Whether a slot's subpath can be made to escape the base folder (for example by starting with a slash or with `..`). Nothing in the sources addresses that.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say. Note that the folder separator shown in the manual's formula is `/`, and no per-platform remark accompanies it.


**Observed by hand test (2026-08-02).** **An output's name can reach outside the node's folder.** With the node pointed at `/tmp/fo/out/base` and one output named `../escape`, the file landed one level up, outside that folder:

```
RESULT [4.5.12 LTS] separators in output names, Image mode, node folder /tmp/fo/out/base
    base/a/b/c/deep0001.png  694 bytes
    base/absolute0001.png  694 bytes
    base/sub/shot0001.png  694 bytes
    escape0001.png  694 bytes
```

(The listing is relative to `/tmp/fo`, so `escape0001.png` sits beside `base`, not inside it.) A name beginning with a slash did not escape: `/absolute` was joined under the node's folder as `base/absolute0001.png`. So the inference above stands for the ordinary case, and there is one exception to it — a name climbing upwards does leave the folder. (Hand test 4.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux did the same, including the escape.** With the node pointed at `.../fo/out/base` and the same five names:

```
RESULT [4.5.12 LTS] separators in output names, Image mode, node folder /root/smoozy/File output nodes/work/fo/out/base
    out/base/a/b/c/deep0001.png  694 bytes
    out/base/absolute0001.png  694 bytes
    out/base/sub/shot0001.png  694 bytes
    out/escape0001.png  694 bytes
```

(The listing is relative to the folder above `out`, so `out/escape0001.png` sits beside `base`, not inside it.) A name beginning with a slash was joined under the node's folder, as on the Mac.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed — including the part most likely to differ, a name climbing out of its folder. Windows is untested. (Linux entry 4.)

---

## 4. Can a folder be expressed inside an output's name rather than separately?

**Label: not in the sources.**

The sources never state whether writing a separator inside a slot's name creates or targets a folder.

**Where I looked.** Manual File Output Node page (the whole page, quoted above — it names only Base Path and File Format). Manual File Paths page in full. Manual Output panel page in full. API pages for `CompositorNodeOutputFile`, `NodeOutputFileSlotFile`, `NodeOutputFileSlotLayer`, both slot-collection types, and `ImageFormatSettings`. Searches across the whole manual and the whole API text for: "subpath", "sub path", "file output", "output file", "base path", "directory", "folder", "File Subpath", "Add Input". The only two hits for "subpath" in the entire API and manual are the two I already quoted:

> "path — Subpath used for this slot" (API, NodeOutputFileSlotFile)

> "file_path (string, (optional, never None)) – File Path, Subpath of the output file" (API, bpy.ops.node, entry for `bpy.ops.node.output_file_add_socket`)

Both read 2026-07-30. Neither says what a separator inside that string does.

The word "subpath" (rather than "name" or "filename") is suggestive, and the manual's formula `{base path}/{file name}{frame number}.{extension}` shows the two being joined with a separator. But going from "it is called a subpath" to "you may put a folder in it and Blender will create/use that folder" requires the reader to supply the whole claim. I will not label that inferred; it is a guess, and the honest answer is that the sources are silent. This is on the hand-check list at the end.

**Not covered.** Everything about this question.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say. Whether both `/` and `\` work is an operating-system question the sources never touch; it has since been settled by hand on two machines — see the observed blocks below.


**Observed by hand test (2026-08-02).** **Yes, in this version, and both separators work.** With the node pointed at `/tmp/fo/out/base`, the names asked for were kept exactly as typed —

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '../escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
```

— and a folder was made for each forward slash, several levels deep where asked (`base/sub/shot0001.png`, `base/a/b/c/deep0001.png`; full listing quoted under question 3). A backslash does the same thing: a node whose only output was named `sub\shot`, pointed at a separate empty folder, produced

```
RESULT [4.5.12 LTS] one output only, named with a backslash, node folder /tmp/fo/out/base2
    base2/sub/shot0001.png  694 bytes
```

Missing folders were made without complaint, and nothing was reported to the terminal in any of these cases. (Hand test 4.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux kept the same names and made the same folders.**

```
ASKED FOR OUTPUT NAMED 'sub/shot' -> node kept 'sub/shot'
ASKED FOR OUTPUT NAMED 'sub\\shot' -> node kept 'sub\\shot'
ASKED FOR OUTPUT NAMED 'a/b/c/deep' -> node kept 'a/b/c/deep'
ASKED FOR OUTPUT NAMED '../escape' -> node kept '../escape'
ASKED FOR OUTPUT NAMED '/absolute' -> node kept '/absolute'
```

and a node whose only output was named with a backslash gave

```
RESULT [4.5.12 LTS] one output only, named with a backslash, node folder /root/smoozy/File output nodes/work/fo/out/base2
    out/base2/sub/shot0001.png  694 bytes
```

**This is the sharpest operating-system question on the sheet, and both machines answer it the same way.** A backslash is a folder separator on Windows and an ordinary character in a file name on macOS and Linux; Blender treats it as a folder separator on both machines tested. Windows itself is untested.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 4.)

---

## 5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?

**Label: documented** (relative-to-the-blend-file form is supported) **/ inferred** (the rest) **/ not in the sources** (network folders specifically).

**Documented.** The node's folder setting accepts the blend-file-relative prefix:

> "base_path — Base output path for the image — Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

Source: API, CompositorNodeOutputFile, address above — read 2026-07-30.

And the manual ties the node's folder setting to the render output path:

> "Besides being split into two settings, in all other respects, this setting is treated the same as the render output path."

Source: manual, File Output Node page — read 2026-07-30.

The render output path is described this way:

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

Source: manual, Output panel, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

And relative paths in general:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

Source: manual, Save & Load, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30.

**Inferred: folders unrelated to where the blend-file sits.** The step: the `//` prefix is described as an *option* that *makes* a path relative. From that opt-in form I infer that a folder without the prefix can be taken as given — that is, the node can be pointed at a folder that has nothing to do with the blend-file's location. The sources never write that sentence about the File Output node.

**Not in the sources: network folders.** No page states that the File Output node can write to a network location. The nearest thing in the whole manual is a hint attached to the *render output* path, not to this node:

> "An easy way to get multiple machines to share the rendering workload is to: Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory."

Source: manual, Output panel, address above — read 2026-07-30.

That is about the render output path and the Overwrite/Placeholders options, which belong to the Output panel, not to this node. Chaining it to the File Output node via "in all other respects, this setting is treated the same as the render output path" would be stretching a sentence about *path expansion* into a claim about *network file systems*, so I am not making that claim.

**Where I looked for network folders.** Whole-manual searches for: "UNC", "network path", "network drive", "network file system", "shared directory", "smb://", "\\\\", "mapped drive", "SMB", "NFS". The only relevant hit is the render-farm hint quoted above.

**Not covered.** Whether the node creates a folder that does not yet exist. Whether an unreachable or read-only folder produces an error, a warning, or silence. Anything specific about network locations.

**Outside the interface.** The sources do not say for this node. Related and documented: the command line can set a render output path, and the manual's example uses a `//` relative path in background mode —

> "blender -b animation.blend -o //render_ -F PNG -x 1 -a"

Source: manual, Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. That is the *render output* path, not the node's, so it does not settle the question.

**Operating system.** The sources do not say. There is one nearby operating-system statement, about formats rather than folders, quoted under question 9.


**Observed by hand test (2026-08-02).** **A folder unrelated to where the Blender file sits is accepted**, which settles the part this answer had as inferred: every test above wrote into `/tmp/fo/out`, which has nothing to do with the file's own folder, and did so with the file unsaved.

**A folder that does not exist is made.** Pointed at `/tmp/fo/out/does/not/exist/yet`, the render printed no error and afterwards:

```
FOLDER /tmp/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [4.5.12 LTS] node folder that did not exist
    out/does/not/exist/yet/made0001.png  694 bytes
```

**The `//` form works and is relative to the saved file.** With the file saved at `/tmp/fo/out/held.blend` and the folder set to the text `//relative/`:

```
RESULT [4.5.12 LTS] node folder set to the text //relative/ with the file saved at /tmp/fo/out/held.blend
    out/held.blend  463699 bytes
    out/relative/made0001.png  706 bytes
```

**A folder that cannot be written to fails loudly and writes nothing.** With the folder set to one whose permissions were 500:

```
/tmp/fo/out/locked/made0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '/tmp/fo/out/locked/made0001.png'
RESULT [4.5.12 LTS] node folder that cannot be written to (mode 500)
    (nothing written)
```

Network folders were not tested and could not be — see *Cannot be run* in the hand-test list. (Hand test 11.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on all three parts, and this test had to be run twice there.**

A folder that does not exist is made:

```
FOLDER /root/smoozy/File output nodes/work/fo/out/does/not/exist/yet EXISTS AFTERWARDS: True
RESULT [4.5.12 LTS] node folder that did not exist
    out/does/not/exist/yet/made0001.png  694 bytes
```

The `//` form is relative to the saved file:

```
RESULT [4.5.12 LTS] node folder set to the text //relative/ with the file saved at /root/smoozy/File output nodes/work/fo/out/held.blend
    out/held.blend  463699 bytes
    out/relative/made0001.png  737 bytes
```

**Why the unwritable-folder test was run twice.** The account running Blender on that machine is the root user, and root walks through file permissions. Run as plain root, the folder set to permissions 500 was written to anyway:

```
RESULT [4.5.12 LTS] node folder that cannot be written to (mode 500)
    out/locked/made0001.png  737 bytes
```

Run again with that power dropped for the one command, which is the real answer, it failed loudly and wrote nothing, as on the Mac:

```
/root/smoozy/File output nodes/work/fo/out/locked/made0001.png: Permission denied
Error: Render error (Invalid argument) cannot save: '/root/smoozy/File output nodes/work/fo/out/locked/made0001.png'
RESULT [4.5.12 LTS] node folder that cannot be written to (mode 500)
    (nothing written)
```

**The words differ between the machines and the behaviour does not.** The Mac said `Render error (Permission denied)`; Linux says `Render error (Invalid argument)`. The words in the brackets are the operating system's own description of what went wrong, and the two systems describe it differently. What Blender did — refuse loudly and write nothing — is the same on both.

Network folders still could not be tested on either machine — see *Cannot be run* in both hand-test lists.

**Operating system:** the parts of this answer that could be tested are now settled on macOS and on Linux, and they agreed on what happens, differing only in the wording of the failure message. Windows is untested, and network folders are untested everywhere. (Linux entry 11.)

---

## 6. Can several outputs be gathered into one file rather than written separately?

**Label: inferred.**

The node carries a second, parallel collection of slots whose members are described as multilayer slots holding an OpenEXR layer name:

> "layer_slots — Type: CompositorNodeOutputFileLayerSlots bpy_prop_collection of NodeOutputFileSlotLayer, (readonly)"

Source: API, CompositorNodeOutputFile — read 2026-07-30.

> "Multilayer slot of the file output node"
> "name — OpenEXR layer name used for this slot — Type: string, default "", (never None)"

Source: API, NodeOutputFileSlotLayer, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotLayer.html — read 2026-07-30.

The format list the node draws on contains a multilayer entry:

> "OPEN_EXR_MULTILAYER: OpenEXR MultiLayer. Output image in multilayer OpenEXR format."

Source: API, Image Type Items, https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html — read 2026-07-30.

And the format of a multilayer OpenEXR file is described:

> "An OpenEXR file can store multiple layers and passes."

Source: manual, Supported Graphics Formats, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30.

A 4.5 release note describes the format setting on this node as now carrying a kind:

> "Image Format Settings in the Render Output, File Output, and Image Saving now has a Media Type enum that specifies the general type of the file format, be it Image, Multi-Layer EXR, or Video."

Source: release notes, Pipeline & I/O, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read 2026-07-30.

**The step I took.** The node has one set of slots whose members are "single layer file slot"s carrying a subpath each, and a second set whose members are "multilayer slot"s carrying an OpenEXR layer name each. An OpenEXR file holds multiple layers. From that I infer: yes — when the node is set to the multilayer OpenEXR kind, its several inputs become several layers inside one file rather than several files. The sources never state this in a sentence; I am joining "multilayer slot", "OpenEXR layer name used for this slot" and "an OpenEXR file can store multiple layers" together.

**Not covered.** How the node is switched between the two modes (I did not find that stated — presumably by the format setting, but that is another unwritten step). Whether the per-slot format and `use_node_format` settings still apply in multilayer mode. Whether gathering into one file is possible for any format other than multilayer OpenEXR — nothing in the sources suggests it is.

**Outside the interface.** The sources do not say.

**Operating system.** Partly relevant: OpenEXR multilayer is listed among formats that are compiled in rather than guaranteed — see question 9 for the quote.


**Observed by hand test (2026-08-02).** **Confirmed, and the two collections turn out to work differently from the inference above.** Switching the node's format to `OPEN_EXR_MULTILAYER` did not move anything between the collections by itself — the running program reported `4.5 COLLECTIONS AFTER SWITCHING FORMAT: file_slots=0 layer_slots=0`. Adding two entries to `layer_slots` then gave `4.5 AFTER ADDING TWO LAYER SLOTS: file_slots=2 layer_slots=2 sockets=['passes/diffuse', 'plainlayer']` — the two collections are two views of the same list of inputs, not two separate lists.

**Three outputs became one file with three layers.** With three inputs named `first`, `second` and `third`:

```
RESULT [4.5.12 LTS] three outputs, Multi-Layer EXR mode
    out/0001.exr  1502 bytes
```

and the readable text at the start of that file contains

```
'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R', 'second.A', 'second.B', 'second.G', 'second.R', 'third.A', 'third.B', 'third.G', 'third.R'
```

(the rest of the header text is omitted). So the output's name becomes the layer's name, and a name with a slash in it is kept whole as a layer name rather than making a folder — with an output called `passes/diffuse` the channels came out as `'passes/diffuse.A', 'passes/diffuse.B', 'passes/diffuse.G', 'passes/diffuse.R'`.

**In this mode the node's folder setting is used as the start of the file name, not only as a folder.** Pointed at `/tmp/fo/out/` the file was `out/0001.exr`; pointed at `/tmp/fo/out` with no trailing slash it was `out0001.exr` — a file beside the folder, not in it. The written name carries no output name at all, only the frame number. (Hand tests 5 and 6.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same answers to all three parts.** The two collections behaved the same way — `4.5 COLLECTIONS AFTER SWITCHING FORMAT: file_slots=0 layer_slots=0`, then `4.5 AFTER ADDING TWO LAYER SLOTS: file_slots=2 layer_slots=2 sockets=['passes/diffuse', 'plainlayer']`. Three outputs became one file:

```
RESULT [4.5.12 LTS] three outputs, Multi-Layer EXR mode
    out/0001.exr  1497 bytes
```

whose header text carries `'channels', 'chlist', 'first.A', 'first.B', 'first.G', 'first.R', 'second.A', 'second.B', 'second.G', 'second.R', 'third.A', 'third.B', 'third.G', 'third.R'` (the rest omitted). An output named `passes/diffuse` was kept whole as a layer name rather than making a folder, giving `'passes/diffuse.A'` and its fellows. And the trailing slash still decides where the file lands: pointed at the folder with a trailing slash it wrote `out/base/0001.exr`; pointed at it without one it wrote `out0001.exr`, a file beside the folder.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 5 and 6.)

---

## 7. What happens to a File Output node inside a group of nodes?

**Label: not in the sources.**

Nothing in the 4.5 manual or API says what a File Output node does inside a node group.

**Where I looked.** The manual's Node Groups page in full (https://docs.blender.org/manual/en/4.5/interface/controls/nodes/groups.html). The manual's Compositor System page in full. The File Output Node page. Whole-manual and whole-API searches for "file output", "output file", "node group", "group node", "Group Output", "Make Group", "Ungroup", "node tree context".

Two nearby statements exist, and neither names this node:

> "Avoid using nodes output nodes such as Material Output or Composite inside node groups. These should be used on the top level node tree to improve re-usability of node groups."

Source: manual, Node Groups, address above — read 2026-07-30. It names Material Output and Composite. It does not name File Output, and it is advice ("avoid"), not a statement of what happens.

> "The compositor only supports a single active output target, that is, only one of the Composite nodes or Viewer nodes in the node tree will be considered active and the rest will be ignored. In particular, the compositor searches the Active Node Tree Context and falls back to the Root Node Tree Context if no active output was found in the active node tree context. The active node tree context is the node tree of an expanded node group, that is, when the user selects a node group node and edits its underlying tree, while the root node tree context is the top level node tree without any expanded node groups."

Source: manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30. This is explicitly about Composite and Viewer nodes — the ones that compete to be the single active output target. A File Output node is not one of those, so this passage does not settle its behaviour inside a group either way, and I am not going to extend it.

**Not covered.** Everything the question asks. This is on the hand-check list.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes, once, wherever it sits.** A File Output node placed inside a node group wrote its file; using the same group twice in the tree still produced one file; and putting that group inside a second group made no difference:

```
RESULT [4.5.12 LTS] one File Output node inside a group, the group used once
    out/grouped0001.png  694 bytes
RESULT [4.5.12 LTS] the same group used twice in the tree
    out/grouped0001.png  694 bytes
RESULT [4.5.12 LTS] the File Output node two groups deep
    out/grouped0001.png  694 bytes
```

Muting the group node from outside stopped it:

```
RESULT [4.5.12 LTS] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases. Note what the second case does not settle: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like. (Hand test 10.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [4.5.12 LTS] one File Output node inside a group, the group used once
    out/grouped0001.png  694 bytes
RESULT [4.5.12 LTS] the same group used twice in the tree
    out/grouped0001.png  694 bytes
RESULT [4.5.12 LTS] the File Output node two groups deep
    out/grouped0001.png  694 bytes
RESULT [4.5.12 LTS] the outer group node muted
    (nothing written)
```

Nothing was reported to the terminal in any of the four cases, as on the Mac. The same caution applies: both uses of the group carry the same folder and the same output name, so one file is what a collision would also look like.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 10.)

---

## 8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?

**Label: inferred** (whole node) **/ not in the sources** (single output).

**Whole node.** Nodes in general can be muted, and this node is a node:

> "Muting a node removes its contribution to the node tree, and makes all links pass through it without change. Links will appear red as an indicator of passing through the muted node."

Source: manual, Node Editing, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html — read 2026-07-30. The same page gives the menu entry "Node ‣ Toggle Node Mute" and the shortcut M.

The property exists on every node, including this one — `Node.mute` is listed among the inherited properties on the CompositorNodeOutputFile page, and its own entry reads:

> "mute — Type: boolean, default False"

Source: API, Node, https://docs.blender.org/api/4.5/bpy.types.Node.html — read 2026-07-30. (The API text gives `mute` no description sentence; only the type and default.)

**The step I took.** "Muting removes a node's contribution to the node tree" plus "this node's contribution is writing a file" gives "a muted File Output node does not write". The manual never says that about this node. Note also that the second half of the mute sentence — "makes all links pass through it without change" — cannot apply here, because the manual states plainly:

> "Outputs — This node has no output sockets."

Source: manual, File Output Node page — read 2026-07-30. So mute's documented description only half fits this node, which is another reason to treat the conclusion as inferred rather than documented.

**A single output within a node.** Not in the sources. The per-output settings are exactly four (`format`, `path`, `save_as_render`, `use_node_format` — quoted in full under question 2) and none of them is an on/off switch for writing.

**Where I looked.** API pages for `NodeOutputFileSlotFile`, `NodeOutputFileSlotLayer`, `CompositorNodeOutputFile`, both slot-collection types. Whole-API searches for "use_node_format", "enabled", "mute", "hide", "disable", "skip". The generic socket type does carry switches —

> "enabled — Enable the socket — Type: boolean, default False"
> "hide — Hide the socket — Type: boolean, default False"

Source: API, NodeSocket, https://docs.blender.org/api/4.5/bpy.types.NodeSocket.html — read 2026-07-30.

— but these are properties of *any* socket in *any* node editor, described in terms of the socket's own display and enablement, and nothing connects them to whether a File Output slot writes a file. Reading them as "switch off one output" would be me supplying the whole claim. The honest answer is that the sources do not cover it. Removing a slot outright is documented ("remove(socket) — Remove a file slot from this node"), but removing is not switching off.

**Not covered.** Whether a muted node's files are simply not written or are written stale. Whether an unconnected slot writes anything. Whether there is a per-output switch in the sidebar that the manual never lists.

**Outside the interface.** The sources do not say. `mute` is a Python API property, so it can be set without clicking; whether muting has the same effect on a render driven from outside is not stated.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Muting the node does stop it writing**, which the sources left as a step. Starting from two connected outputs that both wrote:

```
RESULT [4.5.12 LTS] both outputs connected
    out/A0001.png  694 bytes
    out/B0001.png  600 bytes
RESULT [4.5.12 LTS] the whole node muted (node.mute = True)
    (nothing written)
```

**One output can be switched off on its own, through its link.** Muting only the link feeding output A, and then removing that link while leaving the output on the node, both left B alone:

```
RESULT [4.5.12 LTS] node unmuted, the link into output A muted (link.is_muted = True)
    out/B0001.png  600 bytes
RESULT [4.5.12 LTS] the link into output A removed, the output itself left on the node
    out/B0001.png  600 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B']
```

A disconnected output writes nothing at all — not an empty file, not a black one. The socket's own `enabled` setting was `True` throughout and was not what did this. Whether the panel offers a per-output switch of its own is test 17 on the Creator's half. (Hand test 9.)


**Observed by hand test, from Blender's own window (2026-08-02).** **There is no per-output on/off control on the panel.** The list of outputs offers only a pair of reorder arrows; right-clicking an entry gives Blender's ordinary property menu — Insert Keyframe, Add Driver, Open Drivers Editor, Add to Keying Set, Remove from Keying Set, Reset to Default Value, Copy to Selected, Copy Data Path, Copy Full Data Path, Copy as New Driver, Online Manual — and nothing that switches an output off. The only per-output controls are File Subpath, Use Node Format and the format settings under it. So cutting or muting the link, as under the test above, is the way. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same four results.**

```
RESULT [4.5.12 LTS] both outputs connected
    out/A0001.png  694 bytes
    out/B0001.png  597 bytes
RESULT [4.5.12 LTS] the whole node muted (node.mute = True)
    (nothing written)
RESULT [4.5.12 LTS] node unmuted, the link into output A muted (link.is_muted = True)
    out/B0001.png  597 bytes
RESULT [4.5.12 LTS] the link into output A removed, the output itself left on the node
    out/B0001.png  597 bytes
OUTPUTS STILL ON THE NODE: ['A', 'B']
SOCKET 'enabled' ON OUTPUT A: True
```

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 9 and 10.)

---

## 9. What restrictions, if any, apply to the kind of picture a File Output node can write?

**This question is ambiguous** and I answer the reading I think is meant: *which file formats, depths and channel arrangements are available*. The other reading — *what image content is acceptable* (resolution, transparency, colour space of the incoming image) — is addressed at the end of this answer where the sources allow.

**Label: documented** (the format list and per-format limits) **/ inferred** (that the node draws on that list).

**Inferred: the node's format list is the general image-format list.** The node's format setting is of the general image-format type:

> "format — Type: ImageFormatSettings, (readonly)"

Source: API, CompositorNodeOutputFile — read 2026-07-30. And that type's format choice is:

> "file_format — File format to save the rendered images as — Type: enum in Image Type Items, default 'PNG'"

Source: API, ImageFormatSettings, https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html — read 2026-07-30. The ImageFormatSettings page confirms this node is one of its users, listing under References: "CompositorNodeOutputFile.format" and "NodeOutputFileSlotFile.format".

The step: the node's format setting is of the shared type, therefore the node offers that type's format list. The sources do not say the node offers the *whole* list, only that it uses the type.

**Documented: the list itself.**

> "Image — BMP … IRIS … PNG … JPEG … JPEG2000 … TARGA … TARGA_RAW … CINEON … DPX … OPEN_EXR_MULTILAYER … OPEN_EXR … HDR … TIFF … WEBP …
> Movie — FFMPEG: FFmpeg Video."

Source: API, Image Type Items, https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html — read 2026-07-30. (Quoted in condensed form; each entry on that page carries its own label and a one-line description such as "Output image in PNG format.")

**Documented: per-format limits.** The manual gives a table of channel depth, alpha, metadata and DPI support per format, under the heading:

> "This is the list of image file formats supported internally by Blender:"

and the manual notes:

> "Note, RGBA is not available for all image formats, check the list above for details."

> "Internally, Blender only operates in either 8-bit or 32-bit."

> "Float (Half): 16-bit floating point per channel. … Only supported for OpenEXR files."

Source: manual, Supported Graphics Formats, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30. The table there gives, for example, JPEG as 8-bit with no alpha, and OpenEXR as float 16/32-bit with alpha.

**Documented nearby / inferred for this node: some render formats are not on every build.**

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Source: manual, Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. That sentence sits under the command line's `-F, --render-format` option, so applying the build limit to the File Output node depends on the earlier inference that the node draws on the same format list.

**The other reading (what image content is acceptable).** Almost nothing. The manual says the node's input is an image and that images are saved on rendering (quoted under question 1). The compositor's own data page says images in the compositor are stored in half precision:

> "Three types of data exist, all of which are stored in half precision formats"

Source: manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30. And it describes output nodes' size:

> "The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size."

Same page and date. That names the Viewer node as its example; whether the File Output node is an "output node" for this purpose is not stated, so I am not claiming it fixes the written file's size.

**Not covered.** Whether the node's format list is trimmed relative to the general list. Whether the node warns or fails when a format cannot hold what is fed to it (for example alpha into JPEG). What size the written image is.

**Outside the interface.** Documented for the command-line render-format option: some formats are "not available on all systems". Applying that limit to this node is inferred, as above. Beyond that, the sources do not say.

**Operating system.** Inferred in part for this node: OpenEXR, multilayer OpenEXR, FFmpeg, Cineon, DPX, JPEG 2000 and WebP are described for the command-line render-format option as compiled-in and "not available on all systems". The sources do not say whether that limit carries over to this node or which systems have which formats.


**Observed by hand test (2026-08-02).** **The node's own format list is narrower than the shared list, and does not include video.** The setting's published list of values, read from the running program, is

```
FILE FORMAT VALUES: ['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']
```

but the list the node will actually take is shorter. Setting the node's format to the video value was refused, and the refusal names what it will take:

```
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP')')
```

That is the shared list with `FFMPEG` removed and nothing else changed. A new node starts on `PNG` (`A NEW NODE STARTS ON: file_format=PNG media_type=(no such setting)`), the same as the scene's own output. What the panel draws is test 17 on the Creator's half. (Hand tests 1 and 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The File Format menu on the node draws exactly the fourteen formats the setting will take, and no more.** In order as shown, in two columns: BMP, Iris, PNG, JPEG, JPEG 2000, Targa, Targa Raw / Cineon, DPX, OpenEXR MultiLayer, OpenEXR, Radiance HDR, TIFF, WebP. That is the same fourteen the refusal message named, so what the panel offers and what the setting accepts agree. **AVIF is not among them** in this version. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **The Linux build of this version offers exactly the same formats as the Mac build.** The setting's published list:

```
FILE FORMAT VALUES: ['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']
```

and the refusal, which names the shorter list the node will actually take:

```
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP')')
```

A new node starts on PNG there too: `A NEW NODE STARTS ON: file_format=PNG media_type=(no such setting)`.

**This narrows the build question without closing it.** The sources say some formats are compiled in and "not available on all systems". Two builds of this version have now been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same fourteen. That is two builds, not all builds.

**Operating system:** the format list is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entries 1 and 12.)

---

## 10. What happens when two outputs are aimed at the same name in the same place?

**Label: not in the sources.**

Nothing in the manual, the API documentation or the release notes describes a clash between two outputs writing to the same file.

**Where I looked.** File Output Node page in full. File Paths page in full. Output panel page in full (it is where Overwrite and Placeholders are documented, and they are described only for the render output: "Overwrite — Overwrite existing files when rendering." and "Placeholders — Create empty placeholder frames while rendering." — manual, Output panel, read 2026-07-30; nothing there mentions this node). Rendering Animations page in full. API pages for the node, both slot types and both slot collections. Whole-manual and whole-API searches for: "overwrite", "placeholder", "already exists", "existing file", "collision", "conflict", "clash", "duplicate", "same file name", "unique". None of the hits concerns two File Output slots.

The word "unique" appears in the manual only for a node's *name* —

> "Name — A unique node identifier inside this node tree."

Source: manual, Node Sidebar, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/sidebar.html — read 2026-07-30 — which is about node names, not file names, and does not bear on this.

**Not covered.** Everything the question asks: within one node, across two nodes, and against a file already on disk. This is on the hand-check list.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **Two outputs on one node cannot carry the same name — the second is renamed.** Both were asked to be called `clash`; the node kept `['clash', 'clash_001']`, and two files were written, each holding its own picture (the two pictures differ, and the pixel value tells them apart):

```
RESULT [4.5.12 LTS] two outputs on one node, both asked to be called 'clash'
    out/clash0001.png  694 bytes
    out/clash_0010001.png  600 bytes
    PIXEL 0 OF clash0001.png = (0.227, 0.227, 0.227, 1.0)
    PIXEL 0 OF clash_0010001.png = (0.761, 0.761, 0.761, 1.0)
```

**Two separate nodes aimed at the same file give one file, and it holds the first node's picture.** No warning or error was printed:

```
RESULT [4.5.12 LTS] two separate File Output nodes, same folder, same output name
    out/same0001.png  694 bytes
    PIXEL 0 OF same0001.png = (0.227, 0.227, 0.227, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('same0001.png', (0.227, 0.227, 0.227, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('same0001.png', (0.227, 0.227, 0.227, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('same0001.png', (0.227, 0.227, 0.227, 1.0))]
```

Building the two nodes in the opposite order swapped which picture survived, so it follows the order the nodes were made in:

```
RESULT [4.5.12 LTS] the two nodes made in the opposite order (inverted picture's node made first)
    out/same0001.png  600 bytes
    PIXEL 0 OF same0001.png = (0.761, 0.761, 0.761, 1.0)
```

**A file already on disk is replaced silently, and the scene's Overwrite setting does not reach the node.** Rendering the same frame again with a different picture changed both the content and the time written; switching `render.use_overwrite` off changed nothing about that:

```
    PIXEL 0 = (0.227, 0.227, 0.227, 1.0)   last written at 1785689653.695
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.761, 0.761, 0.761, 1.0)   last written at 1785689654.024
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.227, 0.227, 0.227, 1.0)   last written at 1785689654.373
```

**Placeholders do not reach it either.** A two-frame animation with `render.use_placeholder` on produced the node's files and the scene's files and no placeholder for the node. (Hand tests 7 and 8.)


**Observed by hand test, from Blender's own window (2026-08-02).** **There is nothing for Blender to flag, because it renames the second output before a clash can happen.** With two outputs both asked to be called `clash`, the panel's list shows `clash` and `clash_001`. **This version has no Output Paths panel**; the sidebar ends at the per-output format controls, so there is no read-out of finished paths to check before rendering. (Hand test 18, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux gave the same answers, pixel value for pixel value.**

```
ONE NODE, TWO OUTPUTS BOTH ASKED FOR 'clash' -> node kept ['clash', 'clash_001']
RESULT [4.5.12 LTS] two outputs on one node, both asked to be called 'clash'
    out/clash0001.png  694 bytes
    out/clash_0010001.png  597 bytes
    PIXEL 0 OF clash0001.png = (0.227, 0.227, 0.227, 1.0)
    PIXEL 0 OF clash_0010001.png = (0.761, 0.761, 0.761, 1.0)
RESULT [4.5.12 LTS] two separate File Output nodes, same folder, same output name
    out/same0001.png  694 bytes
    PIXEL 0 OF same0001.png = (0.227, 0.227, 0.227, 1.0)
    REPEAT 2 OF THE TWO-NODE CASE: [('same0001.png', (0.227, 0.227, 0.227, 1.0))]
    REPEAT 3 OF THE TWO-NODE CASE: [('same0001.png', (0.227, 0.227, 0.227, 1.0))]
    REPEAT 4 OF THE TWO-NODE CASE: [('same0001.png', (0.227, 0.227, 0.227, 1.0))]
RESULT [4.5.12 LTS] the two nodes made in the opposite order (inverted picture's node made first)
    out/same0001.png  597 bytes
    PIXEL 0 OF same0001.png = (0.761, 0.761, 0.761, 1.0)
```

A file already on disk was replaced silently, and switching the scene's Overwrite off changed nothing:

```
    PIXEL 0 = (0.227, 0.227, 0.227, 1.0)   last written at 1785995933.173
    AFTER A SECOND RENDER WITH A DIFFERENT PICTURE: PIXEL 0 = (0.761, 0.761, 0.761, 1.0)   last written at 1785995933.383
    AFTER A THIRD RENDER WITH render.use_overwrite = False: PIXEL 0 = (0.227, 0.227, 0.227, 1.0)   last written at 1785995933.624
```

Placeholders did not reach the node there either.

**Which of two writes to one name survives is exactly the kind of thing that could differ between operating systems, and it does not.** Both machines keep the first-made node's picture, four repeats in a row give the same answer on both, and reversing the order of making the nodes reverses the answer on both.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entries 7 and 8.)


**Observed by hand test, from Blender's own window on the rented Linux machine (2026-08-06).** **This version has no Output Paths panel there either**, which agrees with the Mac. With the node's other sidebar panel folded shut, the whole of the Node tab is visible in the picture and holds exactly two panels — **Node** and **Properties** — and nothing else. There is no read-out of finished paths to check before rendering.

*How that picture was got, and why it can be trusted.* That machine's display is covered by a desktop lock screen. Blender does receive a button press through it but never learns where the pointer is, so an ordinary pointer click never operates any control. Clicks aimed at a bare screen position were tried and did nothing, and were abandoned. The panel was folded shut instead by a click sent **inside Blender's own process**, which carries its own position with it and does not go through the display's pointer at all. That this route really works was proved by clicking a checkbox and reading the setting behind it back: `show_options=True` before, `show_options=False` after, and `True` again when it was clicked back. Blender's own screenshot photographs only its own window, so the picture shows Blender's own state and nothing else. (Linux entry 18.)

---

## 11. Can a File Output node write a video file?

**Label: inferred** (a video format appears in the list the node's format setting draws on) **/ not in the sources** (that it actually produces a working video file).

Two things are documented. First, the format list contains a Movie group with one entry:

> "Movie — FFMPEG: FFmpeg Video."

Source: API, Image Type Items, https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html — read 2026-07-30.

Second, a 4.5 release note describes the format settings *of this node* as carrying a kind that includes video:

> "Image Format Settings in the Render Output, File Output, and Image Saving now has a Media Type enum that specifies the general type of the file format, be it Image, Multi-Layer EXR, or Video."

Source: release notes, Pipeline & I/O, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read 2026-07-30.

**The step I took.** The node's format setting is of type ImageFormatSettings; that type's `file_format` choice comes from Image Type Items; Image Type Items includes FFmpeg Video; and the release note names File Output among the places whose format settings gained a kind that can be Video. From those I infer a video format can be selected on this node. What no source states is what the node then *does* — a video file is not written frame by frame the way an image is, and nothing in these sources describes this node opening, appending to or closing a video file.

**An honest discrepancy I must report.** The release note above says the 4.5 Image Format Settings "now has a Media Type enum". The 4.5 API page for `ImageFormatSettings` does **not** list any `media_type` property — I searched the entire 4.5 API text for "media_type" and "media type" and got no hits at all, and the same searches over the whole 4.5 manual also returned nothing. I cannot reconcile these two 4.5 sources, and I am not going to guess which is right. Both are quoted above as they stand.

**Where else I looked.** File Output Node page (it says only "This node writes out an image…"). Manual pages Rendering Animations and Output panel — both discuss video, but only for the render output path and the Encoding panel, never for this node. Searches over the whole manual and API for "movie", "video", "FFMPEG", "container", "codec" cross-referenced against "file output": no page connects them.

**Not covered.** Whether a video format is actually offered in this node's interface; whether the node can produce a playable video; what happens across an animation range if a video format is chosen. This is on the hand-check list.

**Outside the interface.** The sources do not say.

**Operating system.** Documented in part: FFmpeg is on the list of formats "not available on all systems" (quoted under question 9), so a build without it would not offer video at all.


**Observed by hand test (2026-08-02).** **No. The node refuses the video value outright in this version.** There is no media-type setting on it at all — the running program reports `A NEW NODE STARTS ON: file_format=PNG media_type=(no such setting)` — which settles the discrepancy recorded above between the 4.5 release note and the 4.5 API reference: the release note's Media Type is not present on this node in Blender 4.5.12. Setting the node's format to the video value was refused:

```
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP')')
```

The node stayed on PNG, and a three-frame animation produced three pictures:

```
RESULT [4.5.12 LTS] node set to video, animation over frames 1 to 3
    out/clip0001.png  692 bytes
    out/clip0002.png  692 bytes
    out/clip0003.png  692 bytes
```

Whether the panel offers Video as a choice on screen is test 17 on the Creator's half; the setting behind it will not take the value. (Hand test 12.)


**Observed by hand test, from Blender's own window (2026-08-02).** **The panel offers no video choice either.** There is no Media Type control on this node at all, and the File Format menu (listed under question 9) has fourteen entries with no FFmpeg Video among them. So the panel and the setting agree: video cannot be chosen on this node in this version, by hand or from outside. (Hand test 17, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux refused it too, in the same words.** There is no media-type setting on the node there either — `A NEW NODE STARTS ON: file_format=PNG media_type=(no such setting)` — and:

```
SETTING file_format='FFMPEG' RAISED: TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP')')
RESULT [4.5.12 LTS] node set to video, animation over frames 1 to 3
    out/clip0001.png  692 bytes
    out/clip0002.png  692 bytes
    out/clip0003.png  692 bytes
```

So the discrepancy this answer records between the 4.5 release note and the 4.5 API reference is settled the same way on both machines: the release note's Media Type is not present on this node in Blender 4.5.12, on either.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested, and so is any build made by somebody else. (Linux entry 12.)

---

## 12. What can a File Output node be fed, and does it have to come from the render?

**Label: documented** (what it is fed, and that it need not come from the render).

The node's input is an image:

> "Inputs — Image — The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual, File Output Node page — read 2026-07-30. The picture of the node (`_images/node-types_CompositorNodeOutputFile.webp`, opened by me) shows a single input socket labelled "Image".

It does not have to come from the render:

> "This node can be used as a way to automatically save the image after a render; In addition, since this node can be hooked in anywhere in the node tree, it can also save intermediate images automatically."

Source: manual, File Output Node page — read 2026-07-30.

And the compositor's inputs are not limited to the render:

> "Input nodes produce information from a data source. For instance, an input can be: Taken directly from the active camera in a selected scene. A static image. A movie clip (such as an image sequence or video). A color or value."

Source: manual, Input Nodes, https://docs.blender.org/manual/en/4.5/compositing/types/input/index.html — read 2026-07-30. That page lists Bokeh Image, Image, Image Info, Image Coordinates, Mask, Movie Clip, Texture and Scene nodes.

> "To process your image, you use nodes to import the image into Blender, change it, optionally merge it with other images, and finally, save it."

Source: manual, Compositing Introduction, https://docs.blender.org/manual/en/4.5/compositing/introduction.html — read 2026-07-30.

On what happens if something other than an image arrives, the compositor's general rule applies:

> "if node inputs that expect an image are given a single value, the single value will be assumed to cover the whole compositing space"

and the implicit conversion table gives, for example, "Float → Color: f => Color(f, f, f, 1)".

Source: manual, Compositor System, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30. That is a statement about compositor nodes in general, not about this node specifically.

One release note bears on an unconnected input:

> "File output node crash in eevee when in preview mode and without connections."

Source: release notes, Fixes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — read 2026-07-30. It is listed as a fix, so it reports a fault that was corrected; it does not say what an unconnected input now does.

**Not covered.** The exact socket type of the input (colour, image, or otherwise) is not stated on the node's page. Whether feeding it from a source unrelated to the render still requires a render to be running is a separate matter — see question 13.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It has to be fed a picture. A plain colour writes nothing.** One node with two outputs, one fed by a colour node and one fed by the render through an Invert node, wrote only the second — with no error and with the link reported valid:

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [4.5.12 LTS] one output fed by a colour node, one by an image
    out/picture0001.png  600 bytes
```

**It does not have to come from the render**, but it does have to be an image: the picture that was written came through an Invert node rather than straight from the render, and every test on this sheet that used a modified picture wrote normally. (Hand test 13.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux did the same.**

```
OUTPUT 'constant' IS FED BY A COLOUR NODE; OUTPUT 'picture' BY THE RENDER THROUGH AN INVERT NODE
RESULT [4.5.12 LTS] one output fed by a colour node, one by an image
    out/picture0001.png  597 bytes
```

Nothing was written for the output fed by a plain colour, and no error was printed.

**Operating system:** this answer is now settled on macOS and on Linux, and they agreed. Windows is untested. (Linux entry 13.)

---

## 13. What causes a File Output node to write, and when during a render does it write each file?

**Label: documented** (that rendering causes it) **/ not in the sources** (the moment within a render).

**What causes it.** Rendering:

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

Source: manual, File Output Node page — read 2026-07-30.

The compositor must be switched on for any of it to run:

> "Access the Compositor and activate nodes for compositing by clicking the Use Nodes checkbox in the header… After clicking Use Nodes the Compositor is enabled, however, it can also be disabled in the Post Processing."

Source: manual, Compositing Introduction, address above — read 2026-07-30.

> "Compositing — Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Composite Output node."

Source: manual, Post Processing, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30.

There is one further condition stated about the compositor as a whole, which I quote because it bears directly on whether this node ever runs, while noting it is written about other nodes:

> "The compositor searches for the active Composite node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether."

Source: manual, Compositor System, address above — read 2026-07-30. Whether "the compositor doesn't run" means a File Output node in that tree writes nothing is a step the source does not take, so I do not take it either.

**When during a render.** Not in the sources. No page states whether the file lands before, with, or after the main render output for that frame, nor whether it lands per frame or at the end of a sequence beyond the general "An entire sequence of images will be saved, when an animation is rendered."

**Where I looked.** File Output Node page. Post Processing page. Rendering Animations page in full (it describes frames being "automatically saved directly out to a drive after being rendered", but that is about the render output, not this node). Compositor System page in full. Compositing Limitations page in full. API pages for the node and its slots. Searches across manual and API for "write", "written", "on rendering", "after render", "per frame", "frame written". The closest thing anywhere is a Python handler, which is about the render output rather than this node:

> "bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written)"

Source: API, bpy.app.handlers, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30.

**Not covered.** The ordering within a frame. Whether a single-frame render writes at all without an explicit "save" step (for the render output, that is governed by a separate flag: "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)" — API, bpy.ops.render, https://docs.blender.org/api/4.5/bpy.ops.render.html, read 2026-07-30 — but nothing says whether that flag affects this node). Whether the node writes during viewport compositing.

**Outside the interface.** The sources do not say for this node. The render itself can plainly be driven from outside — the manual documents `blender -b`, `-a`, `-f`, `-o` and `-F` on the Command Line Arguments page — but no source states that a File Output node writes under those conditions. Given how much depends on this, it is first on the hand-check list.

**Operating system.** The sources do not say.


**Observed by hand test (2026-08-02).** **It writes when Blender is run from a terminal with no window** — the case this answer said mattered most. From a saved file, one frame and then the whole range:

```
--- blender -b /tmp/fo_scene.blend -f 1
Saved: '/tmp/fo/out/beauty0001.png'
Saved: '/tmp/fo/out/scene_0001.png'
--- blender -b /tmp/fo_scene.blend -a
Saved: '/tmp/fo/out/beauty0001.png'
Saved: '/tmp/fo/out/scene_0001.png'
Saved: '/tmp/fo/out/beauty0002.png'
Saved: '/tmp/fo/out/scene_0002.png'
Saved: '/tmp/fo/out/beauty0003.png'
Saved: '/tmp/fo/out/scene_0003.png'
```

The terminal names each written file, and names the node's file before the scene's own for the same frame.

**The frame number is always added in this version.** A single-frame render and an animation render gave the same names, and `####` inside a name is honoured in both:

```
RESULT [4.5.12 LTS] render.render(animation=False)
    out/hash0001here.png  692 bytes
    out/plain0001.png  692 bytes
RESULT [4.5.12 LTS] render.render(animation=True), frame range 1 to 1
    out/hash0001here.png  692 bytes
    out/plain0001.png  692 bytes
```

**Each file is written as its frame finishes, just before the scene's own file for that frame.** Over a four-frame animation with both writing into one folder, the times a file was last written were:

```
    1785690001.191  out/node0001.png
    1785690001.193  out/scene_0001.png
    1785690002.452  out/node0002.png
    1785690002.455  out/scene_0002.png
    1785690003.800  out/node0003.png
    1785690003.803  out/scene_0003.png
    1785690005.118  out/node0004.png
    1785690005.120  out/scene_0004.png
```

Frame 1's files carry a time about four seconds before frame 4's, so nothing waited for the end of the run; and within each frame the node's file is a couple of thousandths of a second ahead of the scene's.

**Switching compositing off stops it; having no Composite node in the tree does not.**

```
RESULT [4.5.12 LTS] the same tree with render.use_compositing = False
    (nothing written)
RESULT [4.5.12 LTS] compositing back on, but the tree's terminal output node removed
    out/picture0001.png  600 bytes
```

That last one matters: the quoted sentence about the compositor not running without an active Composite or Viewer node does not stop this node writing. Whether a single frame rendered from the window writes, and whether viewport compositing writes, are tests 19 and 20 on the Creator's half. (Hand tests 2, 3, 13 and 15.)


**Observed by hand test, from Blender's own window (2026-08-02).** **One frame rendered from the window does write.** Pressing F12 once and closing the render window without saving the render left four files, one for each connected output of the two nodes in the file:

```
clash0001.png
clash_0010001.png
one0001.png
two0001.png
```

So a File Output node does not need the render to be saved by hand, and the frame number is added here as it is everywhere else in this version.

**The viewport compositor writes nothing.** With compositing switched on in the 3D viewport and left running for about thirty seconds with no render started, the folder stayed empty. So "during rendering or compositing" does not extend to the viewport. (Hand tests 19 and 20, run by the Creator.)


**Observed by hand test on the rented Linux machine (2026-08-06).** **Linux agreed on every part, and one thing the Mac could not reach was settled here.**

**It writes when Blender is run from a terminal with no window**, from a saved file:

```
--- blender -b fo_scene.blend -f 1  (Blender 4.5)
Saved: '/root/smoozy/File output nodes/work/fo/out/beauty0001.png'
Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
--- blender -b fo_scene.blend -a  (Blender 4.5)
Saved: '/root/smoozy/File output nodes/work/fo/out/beauty0001.png'
Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0001.png'
Saved: '/root/smoozy/File output nodes/work/fo/out/beauty0002.png'
Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0002.png'
Saved: '/root/smoozy/File output nodes/work/fo/out/beauty0003.png'
Saved: '/root/smoozy/File output nodes/work/fo/out/scene_0003.png'
```

**The frame number is always added in this version**, on Linux as on the Mac:

```
RESULT [4.5.12 LTS] render.render(animation=False)
    out/hash0001here.png  692 bytes
    out/plain0001.png  692 bytes
RESULT [4.5.12 LTS] render.render(animation=True), frame range 1 to 1
    out/hash0001here.png  692 bytes
    out/plain0001.png  692 bytes
```

**Each file is written as its frame finishes, just before the scene's own file for that frame:**

```
FILES IN THE ORDER THEY WERE LAST WRITTEN (seconds since the epoch):
    1785996139.991  out/node0001.png
    1785996139.999  out/scene_0001.png
    1785996141.219  out/node0002.png
    1785996141.228  out/scene_0002.png
    1785996142.464  out/node0003.png
    1785996142.473  out/scene_0003.png
    1785996143.773  out/node0004.png
    1785996143.783  out/scene_0004.png
```

That machine is shared with other work, so no claim is made about how long any of it took; what the test shows is the **order**, and the order is the same on both machines.

**Switching compositing off stops it; having no Composite node in the tree does not:**

```
RESULT [4.5.12 LTS] the same tree with render.use_compositing = False
    (nothing written)
RESULT [4.5.12 LTS] compositing back on, but the tree's terminal output node removed
    out/picture0001.png  597 bytes
```

**What a stopped render leaves behind, which was never tested on the Mac.** A sixty-frame animation was started from the command line and stopped part way, twice: once with an interrupt — the polite stop, what pressing Ctrl-C does — and once with a hard kill that gives Blender no chance to tidy up. **Both left whole files for every frame that had finished, and nothing at all for the frame that was in flight.** No part-written file, no empty file, no file of the wrong size, and the node's files and the scene's files always in matching pairs. Stopped with an interrupt, this version printed:

```
Sent an internal break event. Press ^C again to kill Blender
```

*(Corrected 2026-08-06 by the runner. This originally said the newer versions did not print that line. The check of these findings interrupted a 5.2 render and got it too, and sent the question back rather than resolving it. Run again on the same machine in all four versions — a 200-frame render at 256 by 256, interrupted after twelve seconds — every one printed it exactly once, word for word:*

```
4.5: Sent an internal break event. Press ^C again to kill Blender
5.0: Sent an internal break event. Press ^C again to kill Blender
5.1: Sent an internal break event. Press ^C again to kill Blender
5.2: Sent an internal break event. Press ^C again to kill Blender
```

*So the line is not particular to this version. Every version prints it when a render is interrupted politely. The frame counts differed — 15, 16, 14 and 14 — because the machine is shared and got through different numbers before the stop arrived.)*

and left five pairs, frames 0001 to 0005, every one of them the full 11025 bytes, with frame 6 — the one being rendered — leaving nothing. Stopped with a hard kill it left thirty-one pairs, again all whole, with frame 32 leaving nothing. The two counts differ only because the machine got through different numbers of frames before the stop arrived; that machine is shared, so the counts are not a measure of its speed.

Whether a single frame rendered from the window writes, and whether viewport compositing writes, were **not** run on Linux — those two need the machine's screen to themselves, and it is shared. They are answered for macOS by the Creator's run above, and are written under *Cannot be run* in the Linux hand-test list.

**Operating system:** the terminal half of this answer is now settled on macOS and on Linux, and they agreed; what a stopped render leaves behind is settled on Linux alone. The window half is settled on macOS alone. Windows is untested throughout. (Linux entries 2, 3, 13, 15 and 22.)

---

## What is still left for a person with Blender open

The list that stood here has been worked through, and nothing on it is outstanding. The tests a terminal could reach were run on 2026-08-02 and the four that needed eyes on the window were run by the Creator the same day; both kinds are written into the answers above as observed findings, and the numbered tests are in `Audits/Blender Research/File output nodes/Hand tests.md`.

**Then a second machine arrived.** On 2026-08-06 the whole list was run again on a rented Linux machine, and two behaviours the Mac could never reach were run for the first time: what every kind of input socket produces in a written file, and what a stopped render leaves behind. Those results are in the answers above too, marked as coming from that machine, and the entries are in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`. Linux agreed with the Mac on every test both machines ran; the one place their words differ is the operating system's own description of a failed write, recorded under question 5.

**Cannot be run at all.** Four things, and the reasons are written out in the *Cannot be run* sections of both hand-test lists:

- **A folder reached over a network**, in any version. Neither machine has a network store to point at, and setting one up on the rented machine would mean installing something and changing a system setting.
- **Anything about Windows.** No Windows machine has run it. macOS and Linux are both covered; Windows is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **A Blender built by somebody else.** Two builds of this version have been tried — Apple silicon macOS and Linux x64, both from Blender's own download server — and they offer the same file formats. That is two builds, not all builds.
- **A frame rendered from Blender's own window on Linux, and viewport compositing on Linux.** That machine's screen is shared with several other topics at once, and those two tests would have to hold it for their whole length. They are answered for macOS by the Creator's run, and are on the *Needs the machine to itself* list for later.

2026-08-02 — Opus 5 (Claude Code) — File output nodes, Blender 4.5

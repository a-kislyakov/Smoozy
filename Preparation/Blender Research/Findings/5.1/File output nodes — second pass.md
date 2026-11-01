# File output nodes — Blender 5.1 — second pass

Sources consulted: the Blender 5.1 manual pages for File Output, the compositor system, compositor introduction, Group Output, node groups, node editing and node-editor controls, file paths and relative paths, the File Browser, image formats, render output, post-processing, animation output, and command-line rendering; the Blender 5.1 Python API pages for `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, `NodeCompositorFileOutputItems`, `ImageFormatSettings`, `Node`, and the relevant socket/image-format enumerations; Blender 5.1 compositor and bug-fix release notes. The complete supplied manual, API, release-note, and Cycles trees were searched for the terms recorded under each question. No relevant Cycles source result was found. Snapshot/date read: **2026-07-30**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## A note added after this document was written: the answers below have been tested by hand on two machines

The *Operating system* lines under each question are statements about **the sources** — about what Blender's own documentation does and does not qualify by operating system. They are unchanged and still true. What has changed is that the answers themselves have since been tested by hand:

- on **the Creator's Mac** — one MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2 — on 2026-08-02, and
- on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH** — on 2026-08-06.

**The two machines agreed on every test both of them ran.** The one place their words differ is the operating system's own description of a failed write, under question 5. Every result, from both machines, is written into this version's first findings document, `Preparation/Blender Research/Findings/5.1/File output nodes.md`, which is where the answers they complete live. The tests are listed in `Audits/Blender Research/File output nodes/Hand tests.md` and `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

So, for the questions below: **anything settled by those tests is settled on macOS and on Linux and on nothing else. Windows was not tested at all.**

---

## 1

**LABEL: documented, with the numerical maximum not in the sources.**

The node writes images. The manual says, exactly: “The File Output node writes one or more images to disk during rendering or compositing,” “Each connected input is saved to the specified directory and filename,” and “Each input corresponds to an image that will be written to disk.” It starts with no actual item sockets, and items can be added through the node: “By default, the node has no input sockets. Inputs can be added by dragging an output from another node onto the blank socket area, or by adding new inputs in the Images/Layers panel.” ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.)

In the separate-image mode, each item/input produces its own image file. In Multi-Layer EXR mode, each item/input carries one image stored as one layer in the shared EXR: “When using Multi-Layer EXR, each input is stored as a separate layer within the file.” (Same source and date.) The API calls the items a “Collection of file output items” and provides `new(socket_type, name)` to “Add an item at the end.” ([NodeCompositorFileOutputItems](https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItems.html), snapshot 2026-07-30.)

The sources say “one or more,” but give no numeric maximum and do not call the collection unlimited. The defensible answer is therefore: dynamically addable multiple outputs, with no numerical cap established by the supplied documentation.

**Search record:** Searched the full manual, API, release-note, and Cycles copies for `File Output`, `CompositorNodeOutputFile`, `file_output_items`, `one or more`, `multiple inputs`, `maximum`, `limit`, `unlimited`, `input sockets`, and `connected input`. Opened the File Output manual page and the three File Output API pages. The generic limit results concerned unrelated Blender features; the File Output sources had no count/range.

**NOT-COVERED:** A hard maximum number of items, or resource/performance limits at very high item counts, is not stated.

**Outside the interface.** The collection can be constructed through the Python API, so creating/naming items does not require the UI; the sources do not state a different item count when rendering in background mode.

**Operating system.** The item model and lack of a documented numeric cap are not qualified by operating system; practical resource limits are not covered.

## 2

**LABEL: documented.**

Node-wide settings are **Directory**, **File Name**, **Media Type**, and **Node Format**. The manual calls Directory “The base output directory,” File Name “The base name of the file,” and says Media Type “Determines how multiple inputs are stored.” Node Format “controls file format and encoding options” and, in Image mode, is “the default format and encoding for all outputs.” ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.) The node API also exposes `directory`, `file_name`, `format`, and `save_as_render`; the last is described as “Apply render part of display transform when saving byte image.” ([CompositorNodeOutputFile](https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html), snapshot 2026-07-30.)

An individual item/output has its **name/socket name**, socket data type (and vector dimensions when applicable), **Override Node Format**, item format, and item `save_as_render`. The item API lists `name`, `socket_type`, `vector_socket_dimensions`, `override_node_format`, `format`, and `save_as_render`; `override_node_format` means “Use a different format instead of the node format for this file.” ([NodeCompositorFileOutputItem](https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html), snapshot 2026-07-30.) The manual limits the format override description to separate-image output: “When saving individual images, each output can be renamed and will be written to disk using either the global Node Format settings or its own overridden settings.” ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.)

**Search record:** Searched for `Directory`, `File Name`, `Media Type`, `Node Format`, `Item Format`, `override_node_format`, `save_as_render`, `format`, `socket_type`, and `vector_socket_dimensions`. Opened the File Output manual page, `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, `NodeCompositorFileOutputItems`, and `ImageFormatSettings`. Generic render-output settings were ruled out unless the File Output page or API attached them to this node.

**NOT-COVERED:** The sources do not explain why `save_as_render` appears at both node and item level, or the exact precedence when an item override is enabled.

**Outside the interface.** All listed node/item properties are exposed in the Python API. The sources state no background-rendering difference.

**Operating system.** These setting ownership rules are not qualified by operating system.

## 3

**LABEL: inferred from the node and item property model.**

*Changed by the citation check, 2026-08-02: this was labelled documented. The quoted words give one Directory on the node and an item property list with no folder in it; getting from there to “an output has no folder of its own” needs the reader to accept that the item API page lists an item's properties exhaustively. That is a reasoning step, so the label is inferred. The 5.1 first findings document and the 5.2 second pass both label the same conclusion inferred.*

There is one node-wide Directory: “The base output directory.” The individual item API has no `directory` or folder property; its properties are `name`, format override/format, `save_as_render`, socket type, vector dimensions, and a read-only color. ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html) and [NodeCompositorFileOutputItem](https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html), snapshot 2026-07-30.) Thus an output does **not** have a second, separately stored folder field of its own. It does have its own name, and that name can encode subdirectories, as covered in question 4.

**Search record:** Searched the full supplied documentation for `output folder`, `item directory`, `slot path`, `file slot`, `layer slot`, `directory`, `folder`, `name`, and the File Output API property names. Opened the File Output node and item API pages and compared their complete property lists. No per-item directory field or separately labelled per-item folder was found.

**NOT-COVERED:** The sources do not describe a hidden or non-RNA per-item folder mechanism; none is documented.

**Outside the interface.** The same ownership is visible in the API: `directory` belongs to the node, not `NodeCompositorFileOutputItem`.

**Operating system.** The ownership of the field is platform-independent in the supplied API; path syntax itself is addressed separately.

## 4

**LABEL: documented.**

Yes. In Image media mode, the item/input name can contain directory separators. The manual says, exactly: “If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy,” followed by “Any missing directories are created automatically during output.” It gives `passes/diffuse` and `masks/object_01` as examples. ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.) The 5.1 release note independently states: “The **File Output** node now allows specifying directories in item names.” ([Blender 5.1 Compositor release notes](https://developer.blender.org/docs/release_notes/5.1/compositor/), snapshot 2026-07-30.)

This is a directory hierarchy embedded in the item name beneath the node’s base Directory; it is not a separate per-item folder setting.

**Search record:** Searched for `directory`, `directories`, `hierarchy`, `slash`, `backslash`, `item names`, `subdirectory`, `passes/diffuse`, and `masks/object_01`. Opened the File Output manual and compositor release notes. The general file-path/template pages were checked and ruled out as unnecessary for this specific item-name feature.

**NOT-COVERED:** The sources do not specify handling of `..`, absolute-looking item names, drive letters, repeated separators, or illegal/reserved path characters.

**Outside the interface.** Item names are writable through the Python API; the sources do not state that slash interpretation changes outside the UI.

**Operating system.** The manual explicitly accepts both `/` and `\\` in an item name, but does not document platform-specific normalization or invalid-character rules. That gap has since been closed by hand: both characters make a folder, and `..` is turned into `_.` before the node keeps the name, on macOS and on Linux alike. Windows is untested. The output is in this version's first findings document, under question 4.

## 5

**LABEL: partly documented; network targets are not in the File Output sources.**

The node accepts a base output directory. The API says: “The directory where the image will be written” and explicitly notes “blend relative // prefix supported.” ([CompositorNodeOutputFile](https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html), snapshot 2026-07-30.) The relative-path manual explains that Blender stores a partial path relative to the directory of the blend file, and: “when entering the path into a text field, use a double slash prefix (//) to make it so.” ([Relative Paths](https://docs.blender.org/manual/en/5.1/files/blend/open_save.html#relative-paths), snapshot 2026-07-30.)

The File Output manual’s supplied node image shows `/tmp/` in the Directory field. That picture is evidence of an absolute Unix directory that is not expressed relative to the blend file. ([File Output Node image](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.) Therefore the sources establish both blend-relative directories and at least an absolute directory unrelated to the blend-file location.

For network storage, the general Render Output manual says a render farm can “Set up a shared directory over a network file system,” but that passage is about the scene Render Output panel, not the compositor File Output node. ([Output](https://docs.blender.org/manual/en/5.1/render/output/properties/output.html), snapshot 2026-07-30.) I cannot transfer that claim to File Output without an explicit statement. The File Output page/API do not name UNC, SMB, NFS, a mapped drive, or a mounted network share.

**Search record:** Searched the full copies for `absolute path`, `relative path`, `// prefix`, `network`, `network file system`, `shared directory`, `network share`, `UNC`, `SMB`, `NFS`, `mapped drive`, `remote`, `server`, `drive`, and `filesystem`. Opened File Output, its node image, its API page, File Paths, Relative Paths, File Browser, Render Output, command-line render, and command-line arguments. The network-render-farm passage was ruled out as direct File Output evidence because it governs the scene render-output field.

**NOT-COVERED:** Whether File Output accepts every OS-accessible/mounted folder, native UNC paths, SMB/NFS URLs, authentication, permission failures, or unavailable network targets is not stated.

**Outside the interface.** The API can set the `directory` string and background rendering is documented, but network behavior and error reporting outside the UI are not stated.

**Operating system.** The `/tmp/` example is Unix-specific; the sources do not document Windows/macOS absolute or network-path spellings for this node. Since then, a folder unrelated to the blend file, a folder that does not yet exist, the `//` form and a folder that cannot be written to have all been tried by hand on macOS and on Linux, and the two machines agreed on what happens, differing only in the wording of the failure message. Windows is untested, and a network folder is untested on both. The output is in this version's first findings document, under question 5.

## 6

**LABEL: documented.**

Yes. Set Media Type to Multi-Layer EXR. The manual says: “Saves all inputs together in a single multi-layer OpenEXR file,” and “When using Multi-Layer EXR, each input is stored as a separate layer within the file.” ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.)

This is the documented gathering mechanism. It is specifically a multilayer OpenEXR file, not a general container for arbitrary separate-image formats.

**Search record:** Searched for `multiple inputs`, `single file`, `one file`, `multi-layer`, `multilayer`, `OpenEXR`, `layers`, and `gather`. Opened File Output, Image Formats/OpenEXR, `ImageFormatSettings`, and the image-type enumeration. Other multi-file or video-container results were ruled out because they were not File Output modes.

**NOT-COVERED:** The sources do not state a maximum layer count, duplicate-layer-name handling, or compatibility details for consumers of the resulting EXR.

**Outside the interface.** The node’s `format` is exposed through the API; no different background-render behavior is documented.

**Operating system.** The multilayer EXR behavior is not qualified by operating system.

## 7

**LABEL: not in the sources.**

The supplied sources do not specifically say what a File Output node does when placed inside a compositor node group. General node documentation says node groups can contain nodes and says: “Avoid using nodes output nodes such as Material Output inside node groups. These should be used on the top level node tree to improve re-usability of node groups.” ([Node Groups](https://docs.blender.org/manual/en/5.1/interface/controls/nodes/groups.html), snapshot 2026-07-30.) That is advice, not File Output behavior.

The compositor-system page discusses active Group Output and Viewer targets and expanded-group/root contexts, but does not mention File Output in that rule. It says the compositor searches the active group context and root context for an active Group Output or Viewer. ([Compositor System](https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html#output), snapshot 2026-07-30.) The Group Output page says Group Output defines what leaves the group, but again does not state whether an internal File Output writes, is pruned, or writes once per group instance. ([Group Output](https://docs.blender.org/manual/en/5.1/compositing/types/output/group_output.html), snapshot 2026-07-30.)

**Search record:** Searched the whole manual/API/release-note corpus for `File Output group`, `group File Output`, `inside group`, `nested File Output`, `output node group`, `active node tree context`, `root node tree context`, `side effect`, `evaluate group`, and `group instance`. Opened Node Groups, Compositor System, Group Output, Group Input, generic Node, and File Output. The Material Output warning and Viewer/Group Output selection rules were ruled out as direct evidence of File Output execution.

**NOT-COVERED:** Whether an internal File Output writes at all, whether unused branches are evaluated, and whether one group definition used multiple times causes one write or one write per instance are not stated.

**Outside the interface.** The sources do not state different group behavior for background/command-line/Python-driven evaluation.

**Operating system.** No operating-system qualification is given because the underlying group behavior itself is not documented.

## 8

**LABEL: documented for node/link controls; the effect on File Output writing is inferred.**

A File Output node inherits `Node.mute`; the API lists `Node.mute` among its inherited properties, and the generic node API exposes `mute` as a Boolean. ([CompositorNodeOutputFile](https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html) and [Node](https://docs.blender.org/api/5.1/bpy.types.Node.html), snapshot 2026-07-30.) The manual says: “Muting a node removes its contribution to the node tree,” and another node-editor page says it “Temporarily disables the node without removing it.” ([Editing Nodes](https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html#mute) and [Node Editors](https://docs.blender.org/manual/en/5.1/interface/controls/nodes/node_editors.html#mute), snapshot 2026-07-30.)

**Inference:** Applying that generic definition to File Output means the node can be switched off as a node. The sources do not explicitly say “muting File Output prevents file creation,” and its lack of output sockets means the usual bypass wording (“inputs are passed directly to its outputs”) does not explain this side effect.

No per-item `mute` or `enabled` property appears in the complete `NodeCompositorFileOutputItem` property list. Individual **links** can be muted: “A muted link acts as though it’s no longer there.” ([Editing Nodes](https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html#mute-links), snapshot 2026-07-30.) Combined with “Each connected input is saved,” this supports the inference that muting/disconnecting one item’s input link can suppress that item while leaving other connected inputs active. It is a link-level method, not a dedicated per-output switch.

**Search record:** Searched for `mute`, `muted`, `disable`, `disabled`, `enabled`, `bypass`, `switch off`, `skip output`, `item mute`, and `link mute`. Opened File Output, `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, generic `Node`, Editing Nodes, and Node Editors. The Switch node was ruled out because it selects data flow rather than documenting File Output enablement.

**NOT-COVERED:** The sources do not directly confirm the filesystem effect of muting the node or one input link, or whether an unlinked existing item writes a default/blank image.

**Outside the interface.** `Node.mute` is writable through the Python API; link creation/removal is also scriptable. The file-writing consequence outside the UI is not separately stated.

**Operating system.** Muting/link semantics are not qualified by OS; file-side effects remain undocumented on all systems.

## 9

**LABEL: documented for supported image encodings and compositor data; output-size application is inferred.**

The File Output node writes **images**, either separate images or a multilayer EXR. Its Image mode uses Blender’s supported still-image formats. The supported-graphics table lists JPEG, OpenEXR, PNG, WebP, AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw, and TIFF, with their channel-depth, alpha, metadata, DPI, and extension capabilities. ([Supported Graphics Formats](https://docs.blender.org/manual/en/5.1/files/media/image_formats.html), snapshot 2026-07-30.) The same page says the save color choices are BW, RGB, and RGBA and warns: “RGBA is not available for all image formats.” It also says: “not all file formats support all color depths.”

The data fed to compositor nodes can be Float, Vector, or Color. The compositor-system page says: “Three types of data exist, all of which are stored in half precision formats,” then defines Float, Vector, and Color. ([Compositor System](https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html#data), snapshot 2026-07-30.) The File Output item API has a `socket_type` and `vector_socket_dimensions`; the 5.1 bug-fix notes also explicitly mention “File output node vector socket dimensions.” ([NodeCompositorFileOutputItem](https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html) and [Blender 5.1 bug fixes](https://developer.blender.org/docs/release_notes/5.1/bugfixes/), snapshot 2026-07-30.) Thus the node is not documented as accepting only conventional RGBA beauty images; scalar/vector/color compositor results can be represented by its input items, subject to the selected image encoding’s channels/depth.

**Inference about size:** the compositor manual says output nodes “like the Viewer node” use an operation domain “with an identity transformation and the same size as the final compositor output,” which for final-render compositing is the scene render size. File Output is an output node, so this general rule points to scene-render-size files during final rendering. The page does not name File Output in that sentence, so the application is inferred, not quoted as File Output-specific fact. ([Compositor System](https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html#determining-operation-domain), snapshot 2026-07-30.)

**Search record:** Searched for `File Output resolution`, `size`, `dimensions`, `domain`, `picture`, `image type`, `format`, `channel`, `depth`, `float`, `vector`, `color`, `RGBA`, and `render size`. Opened File Output, Image Formats, Compositor System, the item API, `ImageFormatSettings`, image-type enumeration, and the relevant 5.1 bug-fix note. Results about video codecs and unrelated image operators were ruled out.

**NOT-COVERED:** The sources do not directly state File Output’s behavior for zero-sized, very large, transformed, non-finite, negative, or out-of-gamut pixel data; nor do they directly name File Output in the final-output-domain rule.

**Outside the interface.** Format and item socket settings are exposed through the API; no different image-type or size restrictions are documented for background rendering.

**Operating system.** Supported encoding capabilities are presented without OS qualifications. Codec/build availability differences are not described for this node.

## 10

**LABEL: not in the sources.**

No supplied File Output source says which output wins, whether later writes overwrite earlier ones, whether Blender reports a collision, or whether names are disambiguated when two items resolve to the same final path.

The Render Output panel has an “Overwrite” option — “Overwrite existing files when rendering” — but it belongs to the scene render output, not to the File Output node, and no File Output page/API property links that option to node-item collisions. ([Output](https://docs.blender.org/manual/en/5.1/render/output/properties/output.html), snapshot 2026-07-30.) It is therefore not evidence for this question.

**Search record:** Searched the complete supplied trees for `same name`, `same file`, `same path`, `duplicate name`, `duplicate path`, `overwrite`, `collision`, `conflict`, `clobber`, `existing file`, `replace`, `unique name`, and `disambiguate`, both generally and near `File Output`/`CompositorNodeOutputFile`. Opened File Output, Output Paths, the item/node API pages, Render Output, and Rendering Animations. The only relevant-looking overwrite text was the separate scene Render Output option and was ruled out.

**NOT-COVERED:** Collision order, overwrite policy, warnings/errors, atomicity, and differences between two items, two File Output nodes, and a pre-existing file are not stated.

**Outside the interface.** The sources do not state whether command-line/background/Python-triggered renders handle collisions differently.

**Operating system.** Case sensitivity, Unicode normalization, reserved names, and filesystem overwrite semantics are not covered.

## 11

**LABEL: inferred from the File Output mode list; a generic API ambiguity is noted.**

The File Output manual gives two Media Type choices and describes both: **Image** (“Saves each input as a separate image file”) and **Multi-Layer EXR** (“Saves all inputs together in a single multi-layer OpenEXR file”). It does not list Video for this node. ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.) From that node-specific, apparently exhaustive mode list, the supported conclusion is that File Output writes still image files/image sequences or one multilayer EXR per frame, **not a video file**.

There is an ambiguity in the generic `ImageFormatSettings` type used by many Blender owners: its `media_type` enumeration includes `IMAGE`, `MULTI_LAYER_IMAGE`, and `VIDEO`, and its all-image-type enumeration includes `FFMPEG`. ([ImageFormatSettings](https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html) and [Image Type All Items](https://docs.blender.org/api/5.1/bpy_types_enum_items/image_type_all_items.html), snapshot 2026-07-30.) Those are generic type capabilities, while the File Output manual exposes only two modes. The API does not say that `VIDEO`/`FFMPEG` is valid for `CompositorNodeOutputFile.format`, so it cannot establish node video support.

For contrast, the compositor introduction says saving a whole composition as a movie uses an AVI or QuickTime render format and the Animation command, while the render-output manual expressly describes its own Video media type. ([Introduction](https://docs.blender.org/manual/en/5.1/compositing/introduction.html#saving-your-composite-image) and [Output](https://docs.blender.org/manual/en/5.1/render/output/properties/output.html), snapshot 2026-07-30.) Those are scene render-output workflows, not File Output functionality.

**Search record:** Searched for `video`, `movie`, `FFmpeg`, `AVI`, `MOV`, `MPEG`, `QuickTime`, `container`, `codec`, `Media Type`, and `image sequence`. Opened File Output, compositor Introduction, Render Output, Rendering Animations, `ImageFormatSettings`, and the image-type enumeration. Generic render-output video controls were ruled out as File Output controls.

**NOT-COVERED:** The sources do not explicitly say “File Output cannot write video,” nor do they explain what happens if a script attempts to assign the generic VIDEO/FFMPEG enum through the node’s format object.

**Outside the interface.** The UI-documented modes exclude video; the API’s generic enum creates an unresolved scripting edge case that needs a hand test.

**Operating system.** The node-specific mode list has no OS qualification; generic video codec availability by OS/build is outside what these sources establish for File Output.

## 12

**LABEL: documented for what the sources say; inferred for the conclusion.**

*Changed by the citation check, 2026-08-02: this was labelled documented throughout. The quotes carry that the compositor accepts non-render sources and that the node can save intermediate results from anywhere in the tree. Joining those two to conclude that a File Output node itself may be fed from a non-render source is a step no quoted sentence takes, so that part is inferred. The 5.1 first findings document labels the same conclusion inferred.*

The input does not have to come from a Render Layers node. The File Output manual says inputs can be added “by dragging an output from another node onto the blank socket area,” and lists as a use: “Save intermediate results from anywhere in the node tree.” It also says the node can “Export multiple render passes or processed outputs directly to files.” ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.)

The compositor itself can start from stills, sequences, and movie frames. Its introduction says “The term Image may refer to a single picture, a picture in a numbered sequence of images, or a frame of a movie clip,” and “The Compositor processes one image at a time, no matter what kind of input you provide.” ([Introduction](https://docs.blender.org/manual/en/5.1/compositing/introduction.html), snapshot 2026-07-30.) Together with the Float/Vector/Color compositor data model and the item socket-type API described under question 9, this establishes that File Output can receive compositor results from imported media, generators/constants, processed branches, or render passes; render origin is not required.

**Search record:** Searched for `fed`, `input`, `connected input`, `another node`, `anywhere in the node tree`, `intermediate`, `render pass`, `Render Layers`, `Image node`, `Movie Clip`, `single value`, `Float`, `Vector`, and `Color`. Opened File Output, compositor Introduction and System, the item API, Image input, Movie Clip input, and Render Layers input pages. Render-only restrictions on Cryptomatte/other specialized nodes were ruled out as unrelated to File Output itself.

**NOT-COVERED:** The sources do not provide a File Output-specific exhaustive whitelist of socket types or explain how every non-color socket is mapped into every file encoding.

**Outside the interface.** Node links and File Output items can be built by Python, and the source need not be a render in that case either; the sources state no background-mode difference.

**Operating system.** Input provenance and compositor data types are not qualified by OS.

## 13

**LABEL: documented for broad triggers; precise scheduling is inferred/not fully covered.**

The direct statement is: “The File Output node writes one or more images to disk during rendering or compositing.” It can “Automatically save the final image after a render,” and “Each connected input is saved.” ([File Output Node](https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html), snapshot 2026-07-30.) Thus evaluation of the compositor as part of a render, or compositor evaluation in another supported context, is what causes connected items to write; merely having an unconnected item is not documented as sufficient.

For render ordering, the post-processing manual says its panel controls processing “after rendering,” and the Group Output page says the compositor result is sent “to image output after rendering.” ([Post Processing](https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html) and [Group Output](https://docs.blender.org/manual/en/5.1/compositing/types/output/group_output.html), snapshot 2026-07-30.) Rendering Animations says frames “are automatically saved directly out to a drive after being rendered” and “Blender creates a file for each frame of the animation.” ([Rendering Animations](https://docs.blender.org/manual/en/5.1/render/output/animation.html), snapshot 2026-07-30.)

**Inference:** In a normal final render with compositing enabled, the render engine finishes a frame, the compositor evaluates that frame, and File Output writes that frame’s connected items during that compositing step, before the animation proceeds to completion. The File Output page’s frame-number rule — each connected input gets a frame number appended, unless `####` places it explicitly — is consistent with one set of files per evaluated frame. The sources do not give a lower-level timestamp such as before/after the main scene output is written.

The compositor must be enabled for the render pipeline to use its tree: “To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled.” ([Introduction](https://docs.blender.org/manual/en/5.1/compositing/introduction.html), snapshot 2026-07-30.) Command-line `-f` renders and saves a selected frame, while `-a` renders the animation with saved settings. ([Rendering From The Command Line](https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html), snapshot 2026-07-30.)

**Search record:** Searched for `during rendering`, `during compositing`, `after render`, `after rendering`, `each frame`, `every frame`, `per frame`, `render animation`, `single frame`, `write`, `save`, `evaluate`, `update`, `compositing pipeline`, and `background render`. Opened File Output, compositor Introduction, Compositor System, Group Output, Post Processing, Render Output Introduction, Rendering Animations, command-line render, and command-line arguments. General main-output save timing was used only to support the frame-order inference, not treated as a File Output-specific scheduling guarantee.

**NOT-COVERED:** The sources do not precisely enumerate which interactive compositor changes trigger disk writes, whether viewport-compositor evaluation writes, the ordering among several File Output nodes/items, or whether File Output happens before or after the main render-output file is committed.

**Outside the interface.** Background rendering is documented: `-b` renders without a UI, `-f` renders one frame, and `-a` renders the animation. With the saved compositor pipeline enabled, the sources imply the same File Output evaluation, but do not state a File Output-specific background exception.

**Operating system.** Render/compositor ordering is not qualified by OS. Filesystem latency, locking, and failure timing are not covered.

## What is still left for a person with Blender open

The list that stood here has been worked through. The tests a terminal could reach were run on 2026-08-02, and the four needing eyes on Blender's window were run by the Creator the same day; the results are recorded as observed findings in this version's first findings document, `Preparation/Blender Research/Findings/5.1/File output nodes.md`, which is where the answers they complete live. The numbered tests are in `Audits/Blender Research/File output nodes/Hand tests.md`.

**The one item left open here has since been run.** On 2026-08-06 the whole list was run again on a rented Linux machine, and **what a File Output output does with each kind of data it will accept** (questions 9 and 12) was tried there for the first time. An output was asked for of every one of the twenty-three socket kinds the setting publishes, and only three could be made at all — Float, Vector and Colour. All three took the render picture and all three wrote a file, with the Float file smaller than the other two. The output is in the first findings document, under question 12, and the entry is number 21 in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

**What is still not settled about it.** What a Float or a Vector output writes when it is fed something other than a picture, and what channels are actually inside each written file — the files were listed and sized, not read back. That part remains open.

**Cannot be run at all.** Four things, written out in the *Cannot be run* sections of both hand-test lists: a folder reached over a network, because neither machine has a network store to point at; anything about Windows, because no Windows machine has run it; a Blender built by somebody else, since only two builds of this version have been tried; and, on the rented machine alone, a frame rendered from Blender's own window and viewport compositing, because that machine's screen is shared with several other topics at once. Those last two are answered for macOS by the Creator's run. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2026-08-02 — GPT-5.6 Sol (Codex) — File output nodes, Blender 5.1, second pass

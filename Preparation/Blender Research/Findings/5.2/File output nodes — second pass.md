# File output nodes — Blender 5.2, second pass

Sources consulted: the supplied Blender 5.2 Manual snapshot, Blender 5.2 Python API snapshot, Blender 5.2 release-notes snapshot, and Cycles 5.2 snapshot. The principal pages opened are linked under each question. The entire supplied Manual, API, release-notes, and Cycles copies were searched before any absence finding. Snapshot date and date read: **2026-07-30**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Terminology: I read “output” primarily as one file-output item/input socket, because that is the sense used by the File Output node's Images/Layers panel. Where “output” could instead mean an outgoing node socket, I state that result too.

---

## A note added after this document was written: the answers below have been tested by hand on two machines

The *Operating system* lines under each question are statements about **the sources** — about what Blender's own documentation does and does not qualify by operating system. They are unchanged and still true. What has changed is that the answers themselves have since been tested by hand:

- on **the Creator's Mac** — one MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2 — on 2026-08-02, and
- on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH** — on 2026-08-06.

**The two machines agreed on every test both of them ran.** The one place their words differ is the operating system's own description of a failed write, under question 5. Every result, from both machines, is written into this version's first findings document, `Preparation/Blender Research/Findings/5.2/File output nodes.md`, which is where the answers they complete live. The tests are listed in `Audits/Blender Research/File output nodes/Hand tests.md` and `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

So, for the questions below: **anything settled by those tests is settled on macOS and on Linux and on nothing else. Windows was not tested at all.**

---

## 1

**LABEL: documented, with a bounded inference about the maximum.**

The node writes images. It can write final images, raw passes, intermediate results, or processed outputs. The manual says, exactly, “The File Output node writes one or more images to disk during rendering or compositing,” and “Each input corresponds to an image that will be written to disk.” It also says the node begins with no inputs and that inputs can be added either by linking another node into the blank socket area or through Images/Layers. Each connected input is one file-output item and carries the corresponding image. In Image mode the items become separate image files; in Multi-Layer EXR mode they become layers of one EXR. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30).

The API calls these a “Collection of file output items” and provides `new(socket_type, name)`, `remove`, `clear`, and `move`. It does not publish a finite maximum. Thus the documented count is zero initially and one or more after items are added; the inference is that the practical count is not given a small fixed cap by this interface, but the sources do not establish an absolute maximum. [NodeCompositorFileOutputItems](https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItems.html) (snapshot 2026-07-30).

Under the alternate reading—outgoing sockets on the node—the answer is zero: “This node has no output sockets.” [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30).

**Search record.** Searched the complete Manual, API, release notes, and Cycles snapshots for `file output`, `output file`, `CompositorNodeOutputFile`, `file_output_items`, `file_slots`, `layer_slots`, `input socket`, `output socket`, `one or more`, `raw passes`, and `intermediate`. Opened the File Output manual page, the three File Output API pages, the node-socket-data-type enum, the compositor introduction, and the 5.2 compositor release notes. The generic socket enum was ruled out as proof of every type the compositor File Output node will functionally accept; it is broader than this node-specific manual page. Cycles had no node implementation or additional limit relevant to this question.

**NOT-COVERED:** No supplied source gives a hard maximum item count or a complete node-specific table mapping every accepted socket type to stored image channels.

**Outside the interface.** The same item collection is exposed to Python through `file_output_items`; background rendering is documented, so the answer does not depend on operating Blender's window. The sources do not state a different item limit outside the interface.

**Operating system.** The sources identify no operating-system difference in item count or what each item represents.

## 2

**LABEL: documented.**

Per-item settings are:

- the item/socket `name`;
- its `socket_type`, and `vector_socket_dimensions` for vector sockets;
- `override_node_format`;
- the item's own `format` when override is used; and
- item-level `save_as_render`.

The manual says an output “can be renamed” and can use “either the global Node Format settings or its own overridden settings.” It also says, “When Override Node Format is enabled, this panel controls the format and encoding options for the selected output image.” [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30). The per-item property list is documented on [NodeCompositorFileOutputItem](https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html) (snapshot 2026-07-30).

Node-wide settings are:

- `directory`;
- `file_name`;
- Node Format, including media type and the global image/EXR format and encoding settings;
- node-level `save_as_render`;
- `use_file_extension`; and
- the active-item index and the collection/order of items.

The manual calls Directory “The base output directory,” File Name “The base name of the file,” and says Node Format controls the default format and encoding for all Image-mode outputs. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30). The node property list is documented on [CompositorNodeOutputFile](https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html), while the nested format properties, including `media_type`, are on [ImageFormatSettings](https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html) (snapshot 2026-07-30). The 5.2 release note additionally says the node's File Extension checkbox is independent of the scene checkbox. [Blender 5.2 Compositor release notes](https://developer.blender.org/docs/release_notes/5.2/compositor/) (snapshot 2026-07-30).

**Search record.** Searched all four supplied corpora for `directory`, `file_name`, `base_path`, `path`, `media type`, `node format`, `item format`, `override node format`, `save_as_render`, `use_file_extension`, `format`, `active item`, `file_output_items`, `file_slots`, and `layer_slots`. Opened the File Output manual page; CompositorNodeOutputFile, NodeCompositorFileOutputItem, NodeCompositorFileOutputItems, and ImageFormatSettings API pages; and the compositor release notes. Old `file_slots`/`layer_slots` terms did not describe the 5.2 API and were ruled out. Cycles contained no additional File Output settings.

**NOT-COVERED:** The sources refer onward to Image Settings for the format-dependent choices; this answer does not reproduce every codec, channel, depth, quality, compression, color-management, or multiview subsetting. The sources also do not explain why `save_as_render` appears at both node and item levels in every media/override combination.

**Outside the interface.** These settings are documented as Python properties, so they can be inspected and changed without using Blender's window. No different headless setting ownership is documented.

**Operating system.** No operating-system-specific ownership difference is documented; only path syntax and available encoders/filesystems may be platform-dependent, which does not move a setting between item and node.

## 3

**LABEL: inferred from the documented property split.**

No separate folder property belongs to an individual item. Directory is node-wide, while an item has a name and optional format override. The reasoning step is: the manual defines one node-level “base output directory”; the exhaustive item API lists `name`, format controls, `save_as_render`, and socket metadata but no directory/path field; therefore an item does not carry a separately editable folder setting. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html), [CompositorNodeOutputFile](https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html), and [NodeCompositorFileOutputItem](https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html) (snapshot 2026-07-30).

An item can nevertheless create a per-item subdirectory hierarchy by putting separators in its name; that is name syntax, not a second folder field (see question 4).

**Search record.** Searched all supplied corpora for `individual output`, `per output`, `each output`, `folder`, `directory`, `path`, `slot path`, `base_path`, `name`, and `subdirectory`. Opened the File Output manual and node/item API pages. The Output Paths panel was ruled out as an editable per-item folder: the manual describes it as displaying calculated final paths. The supplied node image was opened; it shows one Directory and one File Name field on the node, but no item row because the depicted default node has no inputs. Cycles and release notes added no per-item directory property.

**NOT-COVERED:** The sources do not explicitly phrase the conclusion as “items have no folder”; it follows from the documented node/item property lists.

**Outside the interface.** Python exposes the same split—`directory` on the node and no directory property on NodeCompositorFileOutputItem—so there is no documented external-only per-item folder field.

**Operating system.** No OS changes where the field belongs. Separator interpretation is addressed in question 4.

## 4

**LABEL: documented.**

Yes. In Image mode, an item's/input's name can express folders. The manual says exactly: “If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy.” It immediately adds, “Any missing directories are created automatically during output.” The examples are `passes/diffuse` and `masks/object_01`. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30).

**Search record.** Searched all supplied corpora for `slash`, `backslash`, `separator`, `directory hierarchy`, `subdirectory`, `missing directories`, `input name`, `socket name`, and `folder`. Opened the File Output manual, File Output item API, and File Paths manual pages. The API item page confirms the name property but does not itself describe separators. The rule is expressly limited by the manual to Media Type = Image; Multi-Layer EXR uses names as layers instead. Release notes and Cycles had no contrary or additional rule.

**NOT-COVERED:** The sources do not say how absolute-looking item names, `..`, leading separators, reserved characters, or collisions between a file name and directory name are sanitized or rejected.

**Outside the interface.** Setting the item `name` through Python should feed the same documented naming rule; the sources state no external-driver exception.

**Operating system.** The manual explicitly accepts both `/` and `\\` as hierarchy characters. It does not document further OS-specific normalization or invalid-character rules. That gap has since been closed by hand: both characters make a folder, `..` is turned into `_.` before the node keeps the name, and a leading slash does not make the path absolute — on macOS and on Linux alike. Windows is untested. The output is in this version's first findings document, under question 4.

## 5

**LABEL: documented for local path forms; not in the sources for a network-reached directory.**

*Changed by the citation check, 2026-08-02: the network part was labelled inferred, and rested on the step "once a network share is exposed to the operating system as a writable filesystem directory, it is a directory path and can be supplied to the File Output node's Directory field". The quoted words carry that the field takes a directory path, and that the scene render output can be pointed at a network file system. They carry nothing about this node and a network location, and the bridging step is an assumption about operating systems that no cited source states. The step has been removed and the label changed. The 5.2 first findings document reaches the same "not in the sources" on the same citations.*

The node can use its base Directory field. The API says exactly: “The directory where the image will be written,” and documents support for the blend-relative `//` prefix and template expressions. [CompositorNodeOutputFile](https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html) (snapshot 2026-07-30). The File Paths page defines `//` paths and variables including `{blend_dir}` and `{blend_name}`, specifically noting that templates are supported in compositor File Output paths. [File Paths](https://docs.blender.org/manual/en/5.2/files/file_paths.html) (snapshot 2026-07-30).

The manual's supplied File Output node image shows `/tmp/` in the Directory field. That is direct visual documentation that the field accepts an absolute filesystem path form rather than only a path next to the blend-file. Image consulted: `node-types_CompositorNodeOutputFile.webp`, embedded on [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30).

For network storage, the render-output manual expressly documents the workflow step “Set up a shared directory over a network file system.” [Output](https://docs.blender.org/manual/en/5.2/render/output/properties/output.html) (snapshot 2026-07-30). It does not attach that sentence specifically to the File Output node, and I am not carrying it across: it is about the scene's own render output path and the Overwrite and Placeholders settings beside it. Nothing in the supplied sources says whether the File Output node's Directory may be a network location. The sources also do not establish direct URL schemes, an unmounted share, credentials, or any UNC spelling.

**Search record.** Searched all supplied corpora for `absolute path`, `relative path`, `blend relative`, `// prefix`, `network`, `network file system`, `shared directory`, `UNC`, `SMB`, `NFS`, `mounted`, `drive letter`, `external`, `template expressions`, `blend_dir`, and `directory browser`. Opened the File Output manual and its image, CompositorNodeOutputFile API, File Paths manual, render Output manual, command-line rendering manual, and bpy.path API. `UNC`, `SMB`, and direct network URL searches produced no File Output rule. The command-line page's `/project/renders/...` and the render Output shared-filesystem workflow concern scene render output, so they were used only as supporting path context, not as direct proof of File Output's network protocol handling. Cycles and release notes contained no extra path transport rule.

**NOT-COVERED:** Direct UNC/SMB/NFS syntax, authentication, permissions, disconnected shares, atomicity, latency, and whether a missing network root is created are not in the sources. Only missing subdirectories created beneath a valid Image-mode path are documented.

**Outside the interface.** The Directory is a Python string property and background rendering is documented, including rendering remotely through SSH. The source does not document a semantic difference outside Blender's window; the external process still needs filesystem access and permissions. [Rendering From the Command Line](https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html) (snapshot 2026-07-30).

**Operating system.** This is the question with a real OS boundary: absolute path spelling, drive letters, mounts, UNC handling, permissions, and available network filesystems differ by OS. The Blender sources document `//` as blend-relative but do not provide an all-OS matrix for external/network paths. Two of the four corners of that boundary have since been filled in by hand: a folder unrelated to the blend file, a folder that does not yet exist, the `//` form and a folder that cannot be written to were all tried on macOS and on Linux, and the two machines agreed on what happens — differing only in the wording of the failure message, which is the operating system's own words rather than Blender's. Windows is untested, and a network folder is untested on both. The output is in this version's first findings document, under question 5.

## 6

**LABEL: documented.**

Yes. Set Media Type to Multi-Layer EXR. The manual says exactly, “Saves all inputs together in a single multi-layer OpenEXR file.” Each input is then “stored as a separate layer within the file.” In Image mode, by contrast, each input is a separate image file. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30). The node's format object documents `media_type` values including `MULTI_LAYER_IMAGE`. [ImageFormatSettings](https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html) (snapshot 2026-07-30).

**Search record.** Searched all supplied corpora for `multi-layer`, `multilayer`, `Multi-Layer EXR`, `single file`, `all inputs together`, `separate layer`, `layer_slots`, `media_type`, and `OPEN_EXR_MULTILAYER`. Opened the File Output manual, ImageFormatSettings API, render Output manual, image-formats manual, compositor release notes, and relevant bugfix entry. The scene render-cache and ordinary render-output multilayer settings were ruled out as different output mechanisms. Cycles did not add a File Output aggregation mode.

**NOT-COVERED:** The sources do not state how duplicate layer names are resolved, the maximum layer count, or the exact channel mapping for every socket type.

**Outside the interface.** `ImageFormatSettings.media_type` is exposed in Python, so aggregation is not UI-only. No headless difference is documented.

**Operating system.** No OS-specific difference in the single multi-layer EXR behavior is documented; filesystem/path constraints still apply.

## 7

**LABEL: not in the sources.**

The supplied sources do not say what Blender 5.2 actually does with a File Output node placed inside a node group—whether it executes once, once per group instance, is rejected, is ignored, or has some other rule.

The generic node-group manual says, “Only sockets connected to this node will be available as outputs on the group itself,” and advises, “Avoid using output nodes such as Material Output inside node groups.” It does not name File Output or define its execution behavior. [Node Groups](https://docs.blender.org/manual/en/5.2/interface/controls/nodes/groups.html) (snapshot 2026-07-30). The compositor-specific Groups page describes Group Input and Group Output but supplies no File Output exception. [Compositor Groups](https://docs.blender.org/manual/en/5.2/compositing/types/groups.html) (snapshot 2026-07-30).

**Search record.** Searched the complete Manual, API, release notes, and Cycles copies for `File Output group`, `group File Output`, `output node group`, `inside node groups`, `top level node tree`, `side effect`, `group instance`, `execute`, `nested`, `Make Group`, and `Ungroup`. Opened the File Output manual, generic Node Groups manual, compositor Groups manual, node-editing grouping section, CompositorNodeOutputFile API, compositor release notes, and user-interface release notes. The generic recommendation to keep output nodes top-level was ruled out as an answer to actual runtime behavior. Release-note grouping changes concern socket conversion/proxies, not File Output execution. Cycles had no relevant compositor group rule.

**NOT-COVERED:** All runtime behavior of a File Output node inside a group, including multiple instances and nested groups, is absent from the supplied sources.

**Outside the interface.** The sources do not say whether creating or instancing such a group through Python/background mode changes or preserves whatever the UI behavior is.

**Operating system.** The sources do not identify an OS difference, but they also do not establish the base behavior on any OS.

## 8

**LABEL: documented for node and link muting; inferred for suppression of one file item.**

One File Output node can be switched off by muting it. The generic node manual says, exactly, “Temporarily disables the node without removing it.” The editing page says, “Muting a node removes its contribution to the node tree.” [Node Editors](https://docs.blender.org/manual/en/5.2/interface/controls/nodes/node_editors.html) and [Editing Nodes](https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html) (snapshot 2026-07-30). File Output inherits `Node.mute`. [CompositorNodeOutputFile](https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html) and [Node](https://docs.blender.org/api/5.2/bpy.types.Node.html) (snapshot 2026-07-30).

For one item, the API item has no `enabled` or `mute` property. A single incoming link can, however, be muted. The manual says a muted link “acts as though it’s no longer there”; the API says `NodeLink.is_muted` means “Link is muted and can be ignored.” [Editing Nodes](https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html) and [NodeLink](https://docs.blender.org/api/5.2/bpy.types.NodeLink.html) (snapshot 2026-07-30). The File Output manual says each **connected** input is saved. The reasoning step is: muting just that link makes that item unconnected for evaluation, so that is the documented mechanism closest to switching off one item. It is an inference—not an explicit File Output guarantee—that no file at all is emitted for that item rather than an empty/default image.

Removing an item is also available through `file_output_items.remove`, but that deletes the configured item rather than merely switching it off. [NodeCompositorFileOutputItems](https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItems.html) (snapshot 2026-07-30).

**Search record.** Searched all supplied corpora for `mute`, `muted`, `disable`, `disabled`, `switch off`, `enabled`, `active output`, `skip file`, `node.mute`, `is_muted`, and `remove item`. Opened the File Output manual; Node Editors and Editing Nodes manuals; CompositorNodeOutputFile, Node, NodeLink, NodeCompositorFileOutputItem, and NodeCompositorFileOutputItems API pages; render Output manual; and compositor/rendering release notes. The scene-wide Save Output checkbox was ruled out because it controls ordinary render output and explicitly relies on active File Output nodes as an alternative. The item API has no per-item enable flag. Cycles supplied no extra switch.

**NOT-COVERED:** The sources never explicitly say “muting a File Output node prevents its writes,” nor do they explicitly state the on-disk result of muting only one incoming link.

**Outside the interface.** Python exposes `Node.mute` and `NodeLink.is_muted`, so both controls are available externally. No background-mode difference is documented.

**Operating system.** No OS-specific muting behavior is documented.

## 10

**LABEL: not in the sources.**

The supplied sources do not state what wins, whether one write overwrites the other, whether Blender renames one, whether it reports an error, or whether execution order determines the result when two File Output items resolve to the same path and name.

The File Output manual explains the constructed path and has an Output Paths display, but gives no collision policy. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30). `RenderSettings.use_overwrite` is documented as “Overwrite existing files while rendering,” but neither that API page nor the render Output manual says it governs collisions between File Output items in the same compositor execution. [RenderSettings](https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html) and [Output](https://docs.blender.org/manual/en/5.2/render/output/properties/output.html) (snapshot 2026-07-30). I therefore did not transfer the scene-output overwrite rule to this node.

**Search record.** Searched the complete Manual, API, release-notes, and Cycles snapshots for `same file`, `same name`, `same path`, `same filename`, `same directory`, `duplicate`, `collision`, `conflict`, `overwrite`, `overwritten`, `existing file`, `already exists`, `execution order`, and `race`. Opened the File Output manual and image; CompositorNodeOutputFile and item API pages; RenderSettings API; render Output and animation manuals; compositor and bugfix release notes. The ordinary render-output Overwrite/Placeholders feature and unrelated unpack/save overwrite rules were ruled out because they do not specify two File Output items targeting one resolved file. Cycles search hits about film-buffer overwrite were ruled out as renderer-internal pass writes, not compositor filesystem collisions.

**NOT-COVERED:** The entire same-execution collision policy, including image-mode collisions, collisions between different File Output nodes, multilayer duplicate layer names, errors, warnings, and determinism.

**Outside the interface.** The sources do not state whether externally initiated/background renders change collision handling.

**Operating system.** The sources do not state whether filesystem semantics or case sensitivity change the outcome; even the base collision rule is absent.

## 12

**LABEL: documented, with an explicit boundary on accepted socket types.**

It does not have to come from the render. Inputs can be created “by dragging an output from another node onto the blank socket area,” and the node can “Save intermediate results from anywhere in the node tree.” The manual also lists raw passes, processed outputs, and final images as uses. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30).

The compositor introduction says composition nodes can import an image, alter it, merge it, and save it, and that “The Compositor processes one image at a time, no matter what kind of input you provide.” Thus the source may be a render result/pass, an imported still, image sequence, movie frame, or any intermediate compositor result that Blender accepts into the dynamically created File Output input. [Compositing Introduction](https://docs.blender.org/manual/en/5.2/compositing/introduction.html) (snapshot 2026-07-30).

The API makes item socket type explicit through `socket_type` and `vector_socket_dimensions`, but its type annotation points to the general Node Socket Data Type enum rather than a node-specific accepted-types list. [NodeCompositorFileOutputItem](https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html) and [Node Socket Data Type Items](https://docs.blender.org/api/5.2/bpy_types_enum_items/node_socket_data_type_items.html) (snapshot 2026-07-30).

**Search record.** Searched all supplied corpora for `fed`, `input`, `connected input`, `dragging an output`, `anywhere in the node tree`, `render pass`, `raw pass`, `intermediate`, `processed output`, `image input`, `movie clip`, `sequence`, `socket_type`, `value`, `vector`, `color`, and `integer vector`. Opened the File Output manual, compositor introduction, item and item-collection API, node-socket enum, relevant input-node manual pages, and 5.2 compositor release notes. The broad generic socket enum and release-note statement that the compositor supports additional socket families were ruled out as proof that File Output meaningfully serializes every generic type. Cycles added no input restriction.

**NOT-COVERED:** The sources do not provide a definitive File Output-specific list of every accepted socket data type or explain conversion/channel behavior for non-image values, vectors, matrices, strings, objects, and other newer compositor types.

**Outside the interface.** Python can create an item with `new(socket_type, name)` and connect node sockets. The sources document no requirement for the source to be a UI render result in background mode.

**Operating system.** No OS-specific restriction on input provenance is documented.

## 13

**LABEL: documented for triggers and per-frame output; inferred for the precise render stage.**

The direct rule is: “The File Output node writes one or more images to disk during rendering or compositing.” Each connected input is saved, and a frame number is appended to make a sequence. [File Output Node](https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html) (snapshot 2026-07-30).

For a scene render to use the node tree, Compositing Pipeline must be enabled. The API describes that switch as “Process the render result through the compositing pipeline.” [Compositing Introduction](https://docs.blender.org/manual/en/5.2/compositing/introduction.html) and [RenderSettings](https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html) (snapshot 2026-07-30). The Post Processing manual says that panel controls processing “after rendering” and describes Compositing as applying the node tree to images. [Post Processing](https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html) (snapshot 2026-07-30).

For animation, the manual says frames are saved “directly to disk as they are rendered” and, when ordinary Save Output is disabled, “frames will only be written if a Compositor File Output node is configured.” [Rendering Animations](https://docs.blender.org/manual/en/5.2/render/output/animation.html) (snapshot 2026-07-30). The bounded inference is that, for each rendered frame, connected File Output items write during that frame's compositor/post-processing execution, after the rendered image needed by the compositor exists. This is per completed frame/image, not evidence of tile-by-tile or sample-by-sample writes.

The node can also write when compositing is run on non-render inputs; the File Output page expressly says “rendering or compositing.” The supplied sources do not enumerate every interactive-compositor event that performs a file write. The 5.2 release notes discuss interactive execution triggers generally, but do not say that every such execution writes File Output files. [Blender 5.2 Compositor release notes](https://developer.blender.org/docs/release_notes/5.2/compositor/) (snapshot 2026-07-30).

**Search record.** Searched all supplied corpora for `during rendering`, `during compositing`, `after rendering`, `post processing`, `compositing pipeline`, `active File Output`, `per frame`, `each frame`, `write frame`, `save frame`, `execute`, `execution trigger`, `interactive compositor`, `F12`, `render-frame`, and `background`. Opened the File Output manual, compositor introduction, Post Processing and Output manuals, Rendering Animations, command-line rendering and arguments pages, RenderSettings API, render operator API, 5.2 compositor/rendering release notes, and relevant bugfix entries. The scene's ordinary automatic Save Output timing was distinguished from File Output node writes. Interactive-compositor release-note triggers were ruled out as a complete file-writing trigger list. Cycles contained no compositor file-write scheduling rule.

**NOT-COVERED:** Exact ordering among several File Output items/nodes, whether files are opened early or only at final write, behavior on cancellation or partial renders, interaction with render borders/tiles, and which interactive preview changes cause disk writes. Two of those have since been settled by hand rather than from the sources: the order of writing within a frame, on both machines, and what a cancelled render leaves behind, on the rented Linux machine — see question 13 of this version's first findings document.

**Outside the interface.** Background rendering is explicitly documented (`-b`, `-f`, `-a`), and Python exposes render operations and the compositing switch. A background render using the saved node tree therefore follows the documented render/compositing path; the sources state no different File Output timing. [Rendering From the Command Line](https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html) and [Command Line Arguments](https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html) (snapshot 2026-07-30).

**Operating system.** No OS-specific trigger or compositor-stage timing difference is documented. Actual successful writes still depend on that OS's path and filesystem behavior.

## What is still left for a person with Blender open

The list that stood here has been worked through. The tests a terminal could reach were run on 2026-08-02, and the four needing eyes on Blender's window were run by the Creator the same day; the results are recorded as observed findings in this version's first findings document, `Preparation/Blender Research/Findings/5.2/File output nodes.md`, which is where the answers they complete live. The numbered tests are in `Audits/Blender Research/File output nodes/Hand tests.md`. Only these are still open.

**Both items left open here have since been run.** On 2026-08-06 the whole list was run again on a rented Linux machine, and these two were tried there for the first time. Both results are in the first findings document, and the entries are numbers 21 and 22 in `Audits/Blender Research/File output nodes/Hand tests — Linux.md`.

1. **What each socket kind produces in a written file** (item 5 of the old list). An output was asked for of every one of the twenty-four socket kinds the setting publishes, and only three could be made at all — Float, Vector and Colour. Every other kind was refused with `Error: Unable to create item with this socket type`. All three that could be made took the render picture and all three wrote a file, with the Float file smaller than the other two. The output is under question 12 of the first findings document. **Still not settled:** what a Float or a Vector output writes when fed something other than a picture, and what channels are actually inside each written file — the files were listed and sized, not read back.

2. **What a cancelled render leaves behind** (item 6 of the old list). A sixty-frame animation was started from the command line and stopped part way, twice — once with an interrupt, once with a hard kill. Both left whole files for every frame that had finished and nothing at all for the frame that was in flight: no part-written file, no empty file, no file of the wrong size. The output is under question 13 of the first findings document. This one is settled on Linux alone; it was never run on the Mac.

**Cannot be run at all.** Four things, written out in the *Cannot be run* sections of both hand-test lists: a folder reached over a network, because neither machine has a network store to point at; anything about Windows, because no Windows machine has run it; a Blender built by somebody else, since only two builds of this version have been tried; and, on the rented machine alone, a frame rendered from Blender's own window and viewport compositing, because that machine's screen is shared with several other topics at once. Those last two are answered for macOS by the Creator's run. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2026-08-02 — GPT-5.6 Sol (Codex) — File output nodes, Blender 5.2, second pass

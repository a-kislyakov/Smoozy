# Writing — Blender 5.3 findings

- **Sources consulted:** the Blender 5.3 manual (text copy and its `_images` pictures, taken 2026-10-11), the Blender 5.3 Python API documentation (taken 2026-10-11), the Blender 5.3 release notes (taken 2026-10-11), and Cycles' own add-on files for 5.3 (taken 2026-10-16). I also read `Blender Sources/About this folder.md`. Research done 2026-10-16.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One honest note on the line above. The tool I ran in put two things into my starting context without my opening them: the text of this project's `CLAUDE.md` (a short signpost naming another file) and a list of one-line titles of the user's saved notes. I did not open, search or follow either, and I did not open the file the signpost names. No finding below rests on them; every claim below is tied to a quote from `Blender Sources/`.

Nothing was run. No Blender program is supplied in `Blender Sources/`, so every "observed" label below means I read a Cycles file, not that I executed anything.

*Added 2026-10-17 by the hand tests.* The researcher's two paragraphs above describe the researcher's own answers. Each question now also ends with a block headed **Observed — hand tests on the Creator's Mac, 2026-10-17**, which records Blender 5.3 actually run, and each standing line has a sentence added saying what those runs settled. The hand-test list at the end has been pruned to what is still open.

Pictures looked at: `render_output_properties_output_panel.png`, `files_file_paths-invalid_path_template_tooltip.png`, `node-types_CompositorNodeOutputFile.webp` (converted to PNG in a scratch folder outside the project, only to view it).

---

## 1. Can where a render writes be settled at the moment the render is started, without changing the file?

**Answer.** Yes, for the scene's main render output, when the render is started from the command line. The `-o` (`--render-output`) option sets the render path at launch. It must come after the blend-file and before the option that starts the render, because options run in order. The sources describe no step in which `-o` saves the blend-file, so the stored file on disk is left as it was. That last point is inferred.

A second, separate mechanism: a stored path can contain template expressions such as `{blend_name}`. These are replaced when the path is used, not when it is typed. So the place a render writes can depend on values at render time, while the stored text stays the same.

From Python, the render operator takes no path. A script would have to set the scene's path first (inferred, see below).

**Label.** Documented that `-o` sets the render path at launch, that order matters, and that path templating replaces variables at render time. Inferred that the blend-file on disk is not changed. Inferred for the Python route.

**Quotes.**

> "-o, --render-output <path> … Set the render path and file name. Use // at the start of the path to render relative to the blend-file."
> "You can use path templating features such as {blend_name} in the path."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "Arguments are executed in the order they are given."
> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."
> "blender --background test.blend --render-output /tmp --render-frame 1 … …works as expected."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "Always position -f or -a as the last arguments."

https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html — read 2026-10-11

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

https://docs.blender.org/manual/en/5.3/files/file_paths.html — read 2026-10-11

The Python render operator's full signature, with no path parameter:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11

The scene path is an ordinary text property, not marked read-only:

> "filepath … Directory/name to save animations, # characters define the position and padding of frame numbers (default “//”, never None, blend relative // prefix supported, Supports template expressions)"

https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11

The preference for a default output folder applies only to new scenes, so it does not settle where an existing file writes:

> "render_output_directory … The default directory for rendering output, for new scenes"

https://docs.blender.org/api/5.3/bpy.types.PreferencesFilePaths.html — read 2026-10-11

**Steps behind the inferences.**
- *Blend-file unchanged:* the manual says loading the blend-file "overwrites the render output that was set". So `-o` changes the setting in the copy Blender has loaded. No source says `-o` saves the file. So the stored file keeps its own path. Nothing says this outright.
- *Python route:* the operator has no path parameter, and `RenderSettings.filepath` is a writable text property. So a script would set `filepath` and then call the operator. No source shows this as a recipe.

**Not covered.** Whether `-o` also changes File Output node paths (no source says; see question 9). Which scene `-o` changes when a file has several scenes (`-S` "Set the active scene <name> for rendering" exists, but the link to `-o` is not stated). Inside the interface, the sources show no way to give a path at the moment of pressing Render. The only way there is to edit the Output panel field first, which changes the loaded scene. Whether that change is saved depends on saving the file (inferred). Release notes mention custom project variables for templates (question 3), but they do not say where those values are stored, so I cannot say whether they change the blend-file.

**Outside the interface.** `-o` is itself a command-line route and is shown with `-b` (background). Inside the interface no equivalent at render start is described. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac, below):* `-o` was run with no window in every entry, and settled what the sources left inferred — it set the active scene's stored output path for that run only, the blend-file on disk kept the same checksum, size and modified time, and the file reopened with its own path (entry 14). It reached only the active scene, or the scene `-S` had made active when `-S` came before it (entries 17 and 32), and did not reach a File Output node (entry 15). A template given to `-o` stays as typed in the stored path (entry 23). With Blender's window open, `-o` landed on the active scene the same way and the Output panel's path field showed it (entry 84).

**Operating system.** The sources do not say `-o` behaves differently on different systems. All examples use forward slashes (`/tmp`, `/project/renders/…`). Nothing is said about Windows-style paths with `-o`. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac. Linux and Windows were not run for 5.3.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 14 — does `-o` change the blend-file on disk?** At factory settings, no window, a file `test.blend` was saved with Workbench at 64 × 64, one frame, PNG, and the stored output path `…/work/e14/stored/stored_`, and a byte-for-byte copy kept beside it as `keep.blend`. Both were checksummed and the file's size and modified time read. After a two-second pause, the file was rendered with `-o` pointing somewhere else. Then the checksums, size and time were read again, the file reopened and its stored path printed, and the folder searched for pictures. The commands:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q15_make.py -- …/work/e14
cp …/work/e14/test.blend …/work/e14/keep.blend
shasum -a 256 …/work/e14/test.blend …/work/e14/keep.blend
stat -f '%N %z %m' …/work/e14/test.blend
sleep 2
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e14/test.blend -o …/work/e14/elsewhere/e_ -F PNG -x 1 -f 1
echo "exit code: $?"
shasum -a 256 …/work/e14/test.blend …/work/e14/keep.blend
stat -f '%N %z %m' …/work/e14/test.blend
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e14/test.blend --python q15_read.py
find …/work/e14 -name '*.png'
```

What came back, in full:

```
Info: Saved as "test.blend"
stored filepath: '…/work/e14/stored/stored_'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
dbbb1a4f55e9e059e025856105ef990d58f9b76f9e88e2b28e48d1489201eeea  …/work/e14/test.blend
dbbb1a4f55e9e059e025856105ef990d58f9b76f9e88e2b28e48d1489201eeea  …/work/e14/keep.blend
…/work/e14/test.blend 95813 1792228797
00:00.347  blend            | Read blend: "…/work/e14/test.blend"
00:00.352  render           | Rendering single frame
00:00.352  render           | Rendering frame 1
00:00.354  render           | Start rendering: Scene, ViewLayer
00:00.354  render           | Engine: Workbench
00:00.536  render           | Saved: '…/work/e14/elsewhere/e_0001.png'
00:00.536  render           | Time: 00:00.18 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
dbbb1a4f55e9e059e025856105ef990d58f9b76f9e88e2b28e48d1489201eeea  …/work/e14/test.blend
dbbb1a4f55e9e059e025856105ef990d58f9b76f9e88e2b28e48d1489201eeea  …/work/e14/keep.blend
…/work/e14/test.blend 95813 1792228797
00:00.352  blend            | Read blend: "…/work/e14/test.blend"
filepath after reopening: '…/work/e14/stored/stored_'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
…/work/e14/elsewhere/e_0001.png
```

The checksum, the size (95813 bytes) and the modified time (1792228797) of `test.blend` were the same before and after, the stored path read back as `…/work/e14/stored/stored_`, and the picture landed at `…/work/e14/elsewhere/e_0001.png`.

**Entry 15 — does `-o` reach a File Output node?** Written out in full under question 9. With a file whose scene output points at `…/scene_out/sc_` and whose File Output node points at `…/node_out/`, `-o …/cli_out/cli_ -f 1` wrote the scene's picture to `…/cli_out/cli_0001.png` and the node's file to its own stored folder, `…/node_out/`; the file's stored values read back unchanged afterwards.

**Entry 17 — which scene `-o` reaches.** In a file with two scenes, `Scene` (active) and `Second`, each with its own stored path, a search of everything a script can read after `-o /tmp/smoozy_q13_sentinel_out_` found the value in `bpy.data.scenes['Scene'].render.filepath` and in `sys.argv`, and nowhere else; `Second` kept its stored path. Written out in full under question 13.

**Entry 23 — a template and `#` marks given to `-o`.** `-o "/tmp/smoozy_q13_tmpl/{blend_name}_####"` was read back by a script as `'/tmp/smoozy_q13_tmpl/{blend_name}_####'`, and the path builder gave `'/tmp/smoozy_q13_tmpl/q13-5.3_0007.png'` for frame 7; `-o "//renders/out_####"` was read back as `'//renders/out_####'`. Written out in full under question 13.

**Entry 32 — `-S` and `-o` on a file with two scenes.** Written out in full under question 13. With `-S Second` before `-o`, the value landed on `Second` and the frame was written to the `-o` folder as `override_0001.png`, the log reading `Start rendering: Second, ViewLayer`. With `-o` before `-S Second`, the value landed on `Scene`, and `Second` rendered to its own stored path, `…/work/e32/stored/from_file_second_0001.png`. With no `-S`, the value landed on `Scene` and `Scene` rendered to the `-o` folder. In every case the other scene kept its stored path. When `Second` had no camera, the two launches that rendered it printed `ERROR No camera found in scene "Second"`, wrote nothing, and ended with exit code 0.

**Entry 84 — `-o` with Blender's window open.** Written out in full under question 13. Started with its window, the two-scene test file and `-o /tmp/smoozy_q13_sentinel_out_`, Blender's active scene held the sentinel, the second scene its own stored path, and the Output panel's path field in the window read `/tmp/smoozy_q13_sentinel_out_`.

---

## 2. How is a place to write recorded in a file — what parts does the recorded thing have?

**Answer.** There are two kinds of recorded place.

*The scene's render output.* One text value, `filepath`, holds folder and name together ("Directory/name"). The default is `//`. It accepts the `//` prefix and template expressions, and `#` marks the frame number. Next to it the scene records:
- whether to add a file extension (`use_file_extension`, default on)
- the format (`image_settings`, with `file_format` and `media_type`: image, multi-layer EXR or video)
- whether to overwrite (`use_overwrite`), whether to make placeholders (`use_placeholder`), and whether to write frames at all (`save_output`)

There is also a read-only `file_extension`. For multi-view, each view has a `file_suffix`.

*The compositor File Output node.* Two text values, `directory` and `file_name`, plus a list of items, each with its own `name`. It also has a node-wide `format`, a per-item format override, and its own `use_file_extension`. The manual gives the pattern `{Directory}/{File Name}{Socket Name}{frame number}.{extension}`.

The sources describe these as properties of data in the blend-file. They do not describe how the blend-file stores them byte by byte.

**Label.** Documented for the scene render output and File Output node properties. Inferred for the internal path length limit applying to `filepath`, and for background script access to API properties.

**Quotes.**

> "filepath … Directory/name to save animations, # characters define the position and padding of frame numbers (default “//”, never None, blend relative // prefix supported, Supports template expressions)"
> "use_file_extension … Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)"
> "file_extension … The file extension used for saving renders (default “”, readonly, never None)"
> "save_output … Write frames to disk for animation renders (default True)"
> "use_overwrite … Overwrite existing files while rendering (default True)"
> "use_placeholder … Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)"

https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11

> "media_type … The type of media to save (default 'IMAGE') … Literal[‘IMAGE’, ‘MULTI_LAYER_IMAGE’, ‘VIDEO’]"

https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html — read 2026-10-11

> "file_suffix … Suffix added to the render images for this view (default “”, never None)"

https://docs.blender.org/api/5.3/bpy.types.SceneRenderView.html — read 2026-10-11

> "directory … The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)"
> "file_name … The base name of the file. Other information might be included in the final file name depending on the node options (default “”, never None, Supports template expressions)"

https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html — read 2026-10-11

> "name … (default “”, never None)" and "override_node_format … Use a different format instead of the node format for this file (default False)"

https://docs.blender.org/api/5.3/bpy.types.NodeCompositorFileOutputItem.html — read 2026-10-11

> "For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11

Picture `node-types_CompositorNodeOutputFile.webp` on that page shows the node with two fields: one reading `/tmp/` with a folder button beside it, and one reading `file_name`. Picture `render_output_properties_output_panel.png` on https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (read 2026-10-11) shows a single path field reading `/tmp/`, with checkboxes Overwrite, Placeholders, File Extensions and Cache Result. Documented. The picture may be older than 5.3: the 5.3 release notes give a different default name for the node (see question 3).

Text encoding and length:

> "all strings in blend-files must be UTF-8, ASCII compatible." … "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user’s file system cannot be ignored."

https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html — read 2026-10-11

> "Fixed strings are used internally, and while it is not useful to list all limits, here are some common limits." … "directory:" 767, "file-name:" 255, "file-path:" 1023 *(Changed by citation check: quote corrected to match definition list in limits.txt without added slashes).*

https://docs.blender.org/manual/en/5.3/advanced/limits.html — read 2026-10-11

**Step behind the inference.** The limits page lists general internal string limits. It does not name the render output field. Applying "file-path: 1023" to `filepath` is my step, so that part is inferred.

**Not covered.** Byte-level storage in the blend-file. Whether the length limits count bytes or characters for these fields (the page notes multi-byte characters take more room). How the multi-view `file_suffix` is placed in the name.

**Outside the interface.** All the parts above are Python properties, so a script run in the background can read and set them. Documented as properties; the background-use statement is inferred from the fact that they are API properties. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* every part above was read and set by scripts with no window (entries 1, 2, 3, 5, 12). The stored path took 1023 characters at most, whether set by script (entry 12) or pasted into the field in Blender's window (entry 83). In the window, the Output panel carries a ticked Save Output checkbox in its header, then the path field, Saving, Media Type, File Format and the Image Sequence switches (entry 80).

**Operating system.** The sources do not say the recorded parts differ by system. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. On this Mac a path typed with backslashes was stored as typed and its backslashes acted as folder separators when written (entry 5).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 1 — what a fresh Blender records as the place to write, and which switches sit beside it.** Blender 5.3 was started with no window at factory settings, and a short script printed the scene's stored output path, the property's own built-in default, whether each output switch exists and its value, the file format, the media type, and the preference that seeds a new scene. The command:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q1.py
```

What came back, in full:

```
VERSION 5.3.0 Beta
startup filepath '/tmp/'
rna default     '//'
use_file_extension True True
file_extension True '.png'
use_overwrite True True
use_placeholder True False
use_render_cache True False
save_output True True
file_format PNG
media_type IMAGE
render_output_directory '//'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
```

Each switch line reads: its name, whether it exists, its value.

**Entry 2 — what a File Output node records, and what a fresh node holds.** At factory settings, a compositor node group was made, put on the scene, and one File Output node added to it; every property of the node was printed with its kind, whether it can be written, and the property's own built-in default. Then, in a second launch, a fresh node was made the same way and the values it actually starts with were printed. The commands:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q2.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q2b.py
```

What the first printed. It is long — forty-one properties — so the lines about drawing the node (its colour, size, position, label and the like) are left out; every line below is whole:

```
…/scripts/orig/q2.py:5: DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0
VERSION 5.3.0 Beta
scene has node_tree: False
scene has compositing_node_group: True
node type CompositorNodeOutputFile
  PROP active_item_index INT writable 0
  PROP directory STRING writable ''
  PROP file_name STRING writable ''
  PROP file_output_items COLLECTION readonly ''
  PROP format POINTER readonly ''
  PROP inputs COLLECTION readonly ''
  PROP name STRING writable ''
  PROP panel_states COLLECTION readonly ''
  PROP save_as_render BOOLEAN writable True
  PROP use_file_extension BOOLEAN writable True
inputs ['']
exit code: 0
```

The same deprecation warning was printed for each of the script's three uses of `compositing_node_group`. There is no `base_path`, no `file_slots` and no `layer_slots`.

What the second printed:

```
VERSION 5.3.0 Beta
new node directory: '/tmp/'
new node file_name: '{blend_name}'
new node use_file_extension: True
new node save_as_render: True
new node format media_type: MULTI_LAYER_IMAGE file_format: OPEN_EXR_MULTILAYER
new node items: []
new node inputs: ['']
exit code: 0
```

So the property's own default for `file_name` prints `''` while a node made with `nodes.new` starts with `'{blend_name}'`, and its folder field starts at `'/tmp/'`.


**Entries 3 and 5 — what a stored path with a trailing separator, an empty path, or a backslash does.** Written out in full under question 3. With the output path `'/tmp/'` the path builder gave `'/tmp/0007.png'`, and with `''` it gave `'0007.png'` (entry 3). A path typed with backslashes was **stored exactly as typed** — `'…/work/e05\\sep_back\\img_'` read back unchanged — while the path builder gave `'…/work/e05/sep_back/img_0001.png'` and the render wrote `…/work/e05/sep_back/img_0001.png`, inside a folder `sep_back`; a doubled forward slash was stored as typed and written as one (entry 5).

**Entry 12 — how much text the stored path holds.** A path 1268 characters long was set and read back as 1023 characters; a path 368 characters long was stored whole (written out in full under question 10).

**Entry 80 — the switch in the Output panel's header, and the switches in the panel.** In Blender's window at its factory start-up scene, the Properties editor was turned to its Output tab and photographed; in a second run, the simulated pointer was moved over the Properties editor and twelve simulated scroll-wheel steps sent, and the editor photographed again, to show the rest of the panel. Each run first printed Blender's own name and description for five of the panel's switches:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e80.py; the second time with SCROLL=1)
```

What the first run printed:

```
WIN version 5.3.0 Beta background False event simulation True
WIN setting save_output: name='Save Output' description='Write frames to disk for animation renders' value=True
WIN setting use_file_extension: name='File Extensions' description='Add the file format extensions to the rendered file name (eg: filename + .jpg)' value=True
WIN setting use_render_cache: name='Cache Result' description='Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)' value=False
WIN setting use_overwrite: name='Overwrite' description='Overwrite existing files while rendering' value=True
WIN setting use_placeholder: name='Placeholders' description="Create empty placeholder files while rendering frames (similar to Unix 'touch')" value=False
WIN properties editor now on: OUTPUT
WIN screenshot …/shots/e80_output_tab.png exists=True size=1162834 window=1512x862
WIN quitting
00:05.114  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

The second run printed the same five `WIN setting` lines, `WIN properties editor now on: OUTPUT` and two screenshot lines (`e80_output_tab.png` and `e80_output_tab_scrolled.png`).

What the two pictures show of the panel headed **Output**: a **ticked checkbox in the panel's own header**, beside the word "Output". Below it, top to bottom: the path field reading `/tmp/` with a folder button; **Saving** — `File Extensions` (ticked), `Cache Result` (unticked); **Media Type** `Image`; **File Format** `PNG (.png)`; **Color** `BW | RGB | RGBA` (RGBA chosen); **Color Depth** `8 | 16` (8 chosen); **Compression** `15%`; **Image Seque…** (the label cut off by the panel's width) — `Overwrite` (ticked), `Placeholders` (unticked); then the closed sub-panels **Color Management** and **Pixel Density**. The panels below are **Metadata** and **Post Processing**. No tooltip was photographed (see above).

**Entry 81 — what a File Output node shows on its face.** In Blender's window, a script made a compositor node group, put it on the scene, added a Render Layers node and a File Output node named `FileOut` with two inputs, `passes/diffuse` and `beauty`, its folder field `//out/` and its name field `shot`, linked Render Layers' Image to both inputs, saved the file as `…/work/e81/node.blend`, switched the window to the Compositing workspace, framed the nodes, opened the node editor's sidebar, and photographed the window. Three runs: the first as described; the second also with a simulated click on the sidebar's "Node" tab; the third instead setting the sidebar's tab to "Node" by script:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e81.py; three times)
```

What the third run printed (the first two printed the same lines, without the last two):

```
WIN version 5.3.0 Beta background False event simulation True
<string>:6: DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0
Info: Saved as "node.blend"
WIN saved …/work/e81/node.blend node inputs ['passes/diffuse', 'beauty', '']
WIN workspace now Layout
WIN node editor tree type CompositorNodeTree node tree Comp
WIN view_all -> {'FINISHED'}
WIN sidebar region width 561 active panel category Group
WIN screenshot …/shots/e81_node.png exists=True size=1323317 window=1512x862
WIN sidebar category set to Node
WIN screenshot …/shots/e81_node_tab.png exists=True size=1299689 window=1512x862
WIN quitting
00:09.149  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

What the pictures show. The node editor's header reads `Comp`, with a path above the nodes reading **Scene › Scene Effect › Comp**, and an asset shelf along its bottom with entries named Bloom, Chrom…, Film G…, Senso…, Vignette, Sepia, Split T…, Tune I…, Unsha… and Retime. The **File Output** node's face, top to bottom: a text field `//out/` with a folder button; a text field `shot` with a folder button; an input socket `passes/diffuse`; an input socket `beauty`; an empty socket with a `+`. The input named `passes/diffuse` keeps its slash on the face. The sidebar showed its **Group** tab (Name `Comp`, Description, Color Tag, Node Width `140`, Usage — Strip Modifier and Scene Effect unticked — and Group Sockets) in all three runs; neither the simulated click nor the script brought the node's own tab to the front — the script printed `WIN sidebar category set to Node` and the picture still shows the Group tab — so the node's own panels in the sidebar were not photographed.

**Entry 83 — very long text put into the File Path field.** In Blender's window at its factory start-up scene, with the Properties editor on its Output tab. A script put 1400 characters — `abcdefghij` repeated 140 times — on Blender's own clipboard, then, with the simulated pointer resting on the File Path field at (2759, 410), sent Command-V; read the field's value back; cleared the clipboard and sent Command-C over the field; read the clipboard. Then it set the path back to `/tmp/`, put the 1400 characters on the clipboard again, and sent a click on the field, Command-A, Command-V and Return, and read the field again. It was run twice. In the first run the click was sent as a double click, which Blender refused (`RuntimeError: Error: Value: only 'PRESS/RELEASE/NOTHING' are supported`), so the Command-A and Command-V that followed went to the field without the field being opened for editing; in the second run the click was a press and release:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e83.py and HOVER="field:2759,410"; twice)
```

What the second run printed, in full (the line about a double-click is the script's own label for the second way):

```
WIN version 5.3.0 Beta background False event simulation True
WIN clipboard holds 1400 characters
WIN before: field holds 5 characters; first 20 '/tmp/'; last 20 '/tmp/'; equals the first 5 of what went in: False
WIN after paste over the field: field holds 1023 characters; first 20 'abcdefghijabcdefghij'; last 20 'defghijabcdefghijabc'; equals the first 1023 of what went in: True
WIN screenshot …/shots/e83_after_paste_over.png exists=True size=1168879 window=1512x862
WIN after copy over the field: clipboard holds 1023 characters, equals the field: True
WIN clipboard holds 1400 characters
WIN screenshot …/shots/e83_editing.png exists=True size=1172629 window=1512x862
WIN after double-click, select all, paste, Return: field holds 1023 characters; first 20 'abcdefghijabcdefghij'; last 20 'defghijabcdefghijabc'; equals the first 1023 of what went in: True
WIN screenshot …/shots/e83_after_return.png exists=True size=1170681 window=1512x862
WIN quitting
00:14.118  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

The first run printed the same lines up to `WIN clipboard holds 1400 characters` the second time, then the refusal above, then `WIN after double-click, select all, paste, Return: field holds 1023 characters; …; equals the first 1023 of what went in: True`.

In the second run's picture taken just after the paste, the field is open for editing — a text cursor in it, still showing `/tmp/` as last drawn — and the status bar shows the editing keys `Confirm`, `Esc Cancel`, `⌘A Select All`, `⌘C Copy`, `⌘V Paste`. In the picture after Return, the field reads `abcdefghijabcd…efghijabcdefghi`, shortened for display, and the status bar shows `Pan View` and `Context Menu`; no message.

So both ways left 1023 characters in the field — exactly the first 1023 of the 1400 — and copying the field back out gave the same 1023.

---

## 3. What characters or marks can a recorded place to write carry that stand for something else rather than for themselves, and what does each of them do?

**Answer.**
- **`//` at the start** — the folder of the current blend-file.
- **`#`** — the frame number. The count of `#` sets the zero padding.
- **`{name}`** — a template expression, replaced at time of use. Available names:
  - in every path that supports templates: `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`
  - in render output paths only (including the File Output node): `fps`, `resolution_x`, `resolution_y`, `scene_name`, `camera_name`
  - in paths on a node only: `node_name`
  - added in 5.3 per the release notes: `{project_root}` and custom project variables
- **`:` inside braces, followed by `#` and `.`** — a format for a number, for example `{fps:###}`. Inside braces, `#` counts digits; it does not mean the frame number. This works only with number values.
- **`{{` and `}}`** — a literal `{` or `}`.
- **`%`** — replaced by a view's name. This appears only in the API description of `frame_path`.
- **`/` or `\` in a File Output node input name** — makes a folder level, when Media Type is Image.

**Label.** Documented for each mark listed. Inferred that marks in stored text apply however the render is started. The `%` mark is documented only in one API parameter description, and what exactly it produces is not shown.

**Quotes.**

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."
> "You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11

> "The # characters are replaced by the frame number, and used to define zero padding."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "Path templates substitute template expressions (written {abcd}, where “abcd” is a variable name) in a filepath when the path is used."
> "Format specifiers currently can only be used with variables that represent numerical values, not string values."
> "In all cases, the number of hash symbols (#) indicates the desired number of digits."
> "{{ translates to a single { in the final path." / "}} translates to a single } in the final path."
> "The expression {blend_name isn’t properly closed, which will result in an error."
> "if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

https://docs.blender.org/manual/en/5.3/files/file_paths.html — read 2026-10-11. The full list of variables named above is on the same page. Picture `files_file_paths-invalid_path_template_tooltip.png` shows the red path field and a tooltip reading "Invalid or incomplete template expression '{blend_name.png'." Documented.

> "`{project_root}` and custom project variables for path templates."

https://developer.blender.org/docs/release_notes/5.3/core/ — read 2026-10-11

> "name … The variable’s name. Must not start with a digit, and must contain only alphanumeric characters and underscores"

https://docs.blender.org/api/5.3/bpy.types.ProjectVariable.html — read 2026-10-11

> "view (str) – View, The name of the view to use to replace the “%” chars (optional, never None)"

https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (under `frame_path`) — read 2026-10-11

> "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy."

https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11

**Not covered.**
- What `#` does in the folder part of a path (the manual says "in the file name").
- What several separate runs of `#` do.
- Whether `%` is used during an ordinary render, or only by `frame_path`.
- Whether `~` or system environment variables are expanded in output paths. I searched for `~/` and found only unrelated uses.
- `<UDIM>` and `<UVTILE>` tokens are documented for image texture paths (https://docs.blender.org/manual/en/5.3/modeling/meshes/uv/workflows/udims.html), not for render output. I did not count them.
- The manual's page on templates does not yet list `project_root`. Only the release notes and API mention it.

**Outside the interface.** The marks belong to the stored text, so they apply however the render is started (inferred). The command-line help states that `//`, templates and `#` work in `-o` (documented). The red highlight and tooltip for template errors are interface-only. Outside, the documented effect is that an animation render "will fail with an error message". *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* every mark was run with no window. `//`, `#`, templates, `{{` and `}}`, `%` and number formats such as `{fps:###}` behaved as recorded (entries 3, 4, 6, 8, 9). `~`, `$HOME` and `%HOME%` were left as written; `${HOME}`, and `{project_root}` in a file belonging to no project, were refused as unknown template variables (entry 4). A broken template stopped a windowless render, exit code 1 (entry 11); in the window the path field was drawn red and the status bar showed the parse error (entry 82). With no saved file, `//frame_####` became the bare name `frame_0001.png` and the render refused to save it, with a window and without (entries 38, 85).

**Operating system.** The File Output node accepts both `/` and `\` as folder marks in input names (documented). The sources do not say whether `\` acts as a folder mark in the main render path on systems other than Windows, or say anything else system-specific about these marks. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. On this Mac a backslash in the scene's output path acted as a folder separator (entries 5, 13). Characters typed into the path reached the file name unchanged, while the same characters arriving through `{scene_name}` were replaced by `_` (entry 13).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 3 — fifteen recorded strings through the path builder, with File Extensions on and off.** At factory settings, no window, the scene was saved as `…/work/e03/dance.blend` so that `//` has a folder to mean, the engine set to Workbench and the format to PNG. Then the scene's output path was set to each of fifteen strings in turn, and `scene.render.frame_path(frame=7)` printed with `use_file_extension` on and then off; then, with the path `/tmp/out_%_`, `frame_path` was asked for three view names. Nothing was rendered. The command:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q9.py -- …/work/e03
```

What came back, in full:

```
Info: Saved as "dance.blend"
saved as: …/work/e03/dance.blend
scene name: Scene
ext=True  '/tmp/out_'                  -> '/tmp/out_0007.png'
ext=False '/tmp/out_'                  -> '/tmp/out_0007'
ext=True  '/tmp/out_##_x.png'          -> '/tmp/out_07_x.png'
ext=False '/tmp/out_##_x.png'          -> '/tmp/out_07_x.png'
ext=True  '/tmp/out_######'            -> '/tmp/out_000007.png'
ext=False '/tmp/out_######'            -> '/tmp/out_000007'
ext=True  '//out_'                     -> '…/work/e03/out_0007.png'
ext=False '//out_'                     -> '…/work/e03/out_0007'
ext=True  '//sub/out_'                 -> '…/work/e03/sub/out_0007.png'
ext=False '//sub/out_'                 -> '…/work/e03/sub/out_0007'
ext=True  '/tmp/{blend_name}_'         -> '/tmp/dance_0007.png'
ext=False '/tmp/{blend_name}_'         -> '/tmp/dance_0007'
ext=True  '/tmp/{scene_name}_'         -> '/tmp/Scene_0007.png'
ext=False '/tmp/{scene_name}_'         -> '/tmp/Scene_0007'
ext=True  '/tmp/{fps}_'                -> '/tmp/24.0_0007.png'
ext=False '/tmp/{fps}_'                -> '/tmp/24.0_0007'
ext=True  '/tmp/out_%_'                -> '/tmp/out_%_0007.png'
ext=False '/tmp/out_%_'                -> '/tmp/out_%_0007'
ext=True  '/tmp/out_%_##'              -> '/tmp/out_%_07.png'
ext=False '/tmp/out_%_##'              -> '/tmp/out_%_07'
ext=True  '/tmp/dir_##/out_'           -> '/tmp/dir_##/out_0007.png'
ext=False '/tmp/dir_##/out_'           -> '/tmp/dir_##/out_0007'
ext=True  '/tmp/out_.png'              -> '/tmp/out_.png0007.png'
ext=False '/tmp/out_.png'              -> '/tmp/out_.png0007'
ext=True  '/tmp/'                      -> '/tmp/0007.png'
ext=False '/tmp/'                      -> '/tmp/0007'
ext=True  ''                           -> '0007.png'
ext=False ''                           -> '0007'
ext=True  '/tmp/my_weird}}_{{path_'    -> '/tmp/my_weird}_{path_0007.png'
ext=False '/tmp/my_weird}}_{{path_'    -> '/tmp/my_weird}_{path_0007'
view=''     -> '/tmp/out_%_0007.png'
view='left' -> '/tmp/out_%_0007_L.png'
view='right' -> '/tmp/out_%_0007_R.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
```

**Entry 4 — ten more strings, through the same path builder.** The same set-up as entry 3, in a new folder, `…/work/e04/dance.blend`, with ten strings the documentation leaves open or describes only: a home-folder mark, three ways of naming an environment variable, `{project_root}` in a file that belongs to no project, two number formats, and two separate runs of `#`. The command:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q3extra.py -- …/work/e04
```

What came back, in full:

```
Info: Saved as "dance.blend"
saved as: …/work/e04/dance.blend
HOME in the environment: '/Users/artist'
'~/out_'                                         -> '~/out_0007.png'
'$HOME/out_'                                     -> '$HOME/out_0007.png'
'${HOME}/out_'                                   -> "RAISED RuntimeError: Error: Parse errors in path '${HOME}/out_':\n- Unknown variable referenced in template expression '{HOME}'.\n"
'%HOME%/out_'                                    -> '%HOME%/out_0007.png'
'/tmp/{project_root}/out_'                       -> "RAISED RuntimeError: Error: Parse errors in path '/tmp/{project_root}/out_':\n- Unknown variable referenced in template expression '{project_root}'.\n"
'{project_root}/out_'                            -> "RAISED RuntimeError: Error: Parse errors in path '{project_root}/out_':\n- Unknown variable referenced in template expression '{project_root}'.\n"
'/tmp/{camera_name}_{resolution_x}x{resolution_y}_' -> '/tmp/Camera_1920x1080_0007.png'
'/tmp/{fps:###}_'                                -> '/tmp/024_0007.png'
'/tmp/{fps:#.##}_'                               -> '/tmp/24.00_0007.png'
'/tmp/a_##_b_###_'                               -> '/tmp/a_##_b_007_.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
```

The blend-file was not part of any project; nothing was set up to give `{project_root}` a value, so these lines say what happens to it in a file with none.

**Entry 5 — what a backslash does in a stored path.** At factory settings, no window, Workbench at 64 × 64, one frame, PNG. The output path was set five ways — with forward slashes, with backslashes for the last two separators, with one backslash among forward slashes, with doubled forward slashes, and ending in a backslash — and for each the stored value, the path builder's answer and the file the render wrote were recorded. The command, and a listing of the folder afterwards:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q25sep.py -- …/work/e05
find …/work/e05 | sort
```

What came back, in full:

```
00:00.631  render           | Saved: '…/work/e05/sep_fwd/img_0001.png'
00:00.636  render           | Saved: '…/work/e05/sep_back/img_0001.png'
00:00.641  render           | Saved: '…/work/e05/sep_mix/sub/img_0001.png'
00:00.648  render           | Saved: '…/work/e05/sep_dbl/img_0001.png'
00:00.653  render           | Saved: '…/work/e05/sep_trail/0001.png'
os.sep = '/'
--- forward
    asked for: '…/work/e05/sep_fwd/img_'
    stored as: '…/work/e05/sep_fwd/img_'
    frame_path: '…/work/e05/sep_fwd/img_0001.png'
    returned: {'FINISHED'}
--- backslash
    asked for: '…/work/e05\\sep_back\\img_'
    stored as: '…/work/e05\\sep_back\\img_'
    frame_path: '…/work/e05/sep_back/img_0001.png'
    returned: {'FINISHED'}
--- mixed
    asked for: '…/work/e05/sep_mix\\sub/img_'
    stored as: '…/work/e05/sep_mix\\sub/img_'
    frame_path: '…/work/e05/sep_mix/sub/img_0001.png'
    returned: {'FINISHED'}
--- double
    asked for: '…/work/e05//sep_dbl//img_'
    stored as: '…/work/e05//sep_dbl//img_'
    frame_path: '…/work/e05/sep_dbl/img_0001.png'
    returned: {'FINISHED'}
--- trailing_back
    asked for: '…/work/e05/sep_trail\\'
    stored as: '…/work/e05/sep_trail\\'
    frame_path: '…/work/e05/sep_trail/0001.png'
    returned: {'FINISHED'}
abspath('//a\\b\\c.png'): 'a\\b\\c.png'
native_pathsep('a\\b\\c.png'): 'a/b/c.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
…/work/e05
…/work/e05/sep_back
…/work/e05/sep_back/img_0001.png
…/work/e05/sep_dbl
…/work/e05/sep_dbl/img_0001.png
…/work/e05/sep_fwd
…/work/e05/sep_fwd/img_0001.png
…/work/e05/sep_mix
…/work/e05/sep_mix/sub
…/work/e05/sep_mix/sub/img_0001.png
…/work/e05/sep_trail
…/work/e05/sep_trail/0001.png
```

(The doubled backslashes inside the quoted values are Python's way of printing one backslash.) The `abspath` line was asked in a session with no saved file.

**Entry 6 — a template whose value itself holds `#`.** At factory settings, no window, Workbench at 64 × 64, one frame, PNG. The scene was renamed `sc##ne` and the output path set to `…/work/e06/tmpl/{scene_name}_`; the path builder was asked for frame 7 and an animation rendered. What came back, in full, and the folder afterwards:

```
00:00.541  render           | Saved: '…/work/e06/tmpl/sc01ne_.png'
scene name is now: 'sc##ne'
path: '…/work/e06/tmpl/{scene_name}_'
frame_path: '…/work/e06/tmpl/sc07ne_.png'
returned: {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
…/work/e06
…/work/e06/tmpl
…/work/e06/tmpl/sc01ne_.png
```

**Entry 8, the parts about marks.** Of the seven paths entry 8 rendered (written out in full under question 8): the path `…/c_dir_hash/seq_##/img_` wrote `…/c_dir_hash/seq_##/img_0001.png` and `…/seq_##/img_0002.png`, in a folder named `seq_##`; and `…/f_percent/img_%_` wrote `…/f_percent/img_%_0001.png` and `…/img_%_0002.png`.

**Entry 9 — `%` in a multi-view render.** With multi-view on and each view saved to its own file, the path `…/with_percent/img_%_` wrote `img_%_0001_L.png` and `img_%_0001_R.png`; the path builder asked for the view `left` gave `…/img_%_0001_L.png` (written out in full under question 8).

**Entry 11 — a broken template expression, with no window.** At factory settings, no window, Workbench at 64 × 64, one frame, PNG. The output path was set to `…/work/e11/{blend_name.png` — an opening brace with no closing one — then the path builder was asked for frame 1, and an animation was rendered from Python inside a `try`. The command, and a listing afterwards:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q8.py -- …/work/e11
find …/work/e11 | sort
```

What came back, in full:

```
path set to: '…/work/e11/{blend_name.png'
frame_path raised: RuntimeError Error: Parse errors in path '…/work/e11/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.

Error: Parse errors in path '…/work/e11/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
render raised: RuntimeError Error: Parse errors in path '…/work/e11/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.

Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 1
…/work/e11
```

Nothing was written. The script caught the error and went on to its end, and Blender still ended with exit code 1.

**Entry 13 — awkward characters, typed into the path and arriving through a template.** The computer was first asked, with ordinary empty files, which of eight names it would refuse:

```
touch [co:lon] exit code: 0
touch [aster*isk] exit code: 0
touch [quest?ion] exit code: 0
touch [pi|pe] exit code: 0
touch [back\slash] exit code: 0
touch [кириллица] exit code: 0
touch [em😀ji] exit code: 0
touch [tra ilingspace ] exit code: 0
```

Then each was typed as the end of the render output path and one frame rendered with no window (written out in full under question 10). The backslash case: asked for `'…/chars/back\\slash_'`, stored as `'…/chars/back\\slash_'`, path builder `'…/chars/back/slash_0001.png'`, and the render wrote `…/chars/back/slash_0001.png` inside a new folder `back`. Every other name came out on disk exactly as typed.

Then, in a second launch, the scene was renamed `a:b*c?` and the output path set to `…/work/e13/v/tmpl/{scene_name}_`, so the same kind of characters reached the path through a template rather than typed. What came back, in full, and the folder afterwards:

```
00:00.519  render           | Saved: '…/work/e13/v/tmpl/a_b_c__0001.png'
scene name is now: 'a:b*c?'
path: '…/work/e13/v/tmpl/{scene_name}_'
frame_path: '…/work/e13/v/tmpl/a_b_c__0001.png'
returned: {'FINISHED'}
listing: ['a_b_c__0001.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
a_b_c__0001.png
```

So the scene name `a:b*c?` reached the file name as `a_b_c_`, while the same characters typed into the path itself reached the file name unchanged.

**Entry 38 — `//` at the start of the output path when no file has been saved.** Blender was started from a known folder, `…/work/e38/cwd`, at factory settings with no file opened — a session that has never been saved — and `-o "//frame_####"`. First a script printed what Blender makes of `//` there; then two renders were started from the command line, an animation of frames 1 to 2 and a single frame, Workbench, PNG; then everything under `…/work/e38` was listed:

```
cd …/work/e38/cwd
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -o "//frame_####" --python unsaved_probe.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -E BLENDER_WORKBENCH -o "//frame_####" -F PNG -s 1 -e 2 -a
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -E BLENDER_WORKBENCH -o "//frame_####" -F PNG -f 1
find …/work/e38 -type f | sort
```

What came back, in full:

```
bpy.data.filepath = ''
current folder = '…/work/e38/cwd'
scene filepath = '//frame_####'
abspath('//') = ''
frame_path(frame=1) = 'frame_0001.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

00:00.371  render           | Rendering animation (frames 1..2)
00:00.371  render           | Rendering frame 1
00:00.372  render           | Start rendering: Scene, ViewLayer
00:00.372  render           | Engine: Workbench
00:00.563  image            | ERROR Couldn't create directory for file frame_0001.png: No such file or directory
00:00.563  reports          | ERROR Render error (No such file or directory) cannot save: 'frame_0001.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.563  render           | Time: 00:00.19

exit code: 1
00:00.351  render           | Rendering single frame
00:00.351  render           | Rendering frame 1
00:00.351  render           | Start rendering: Scene, ViewLayer
00:00.351  render           | Engine: Workbench
00:00.560  image            | ERROR Couldn't create directory for file frame_0001.png: No such file or directory
00:00.560  reports          | ERROR Render error (No such file or directory) cannot save: 'frame_0001.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.560  render           | Time: 00:00.20

exit code: 1
```

The listing found no file. `ls` of `/Users/artist/frame_0001.png` and of `/frame_0001.png` afterwards both said `No such file or directory`.

**Entry 42 — a slash in a File Output input's name.** Written out in full under question 4. With the node writing PNG, an input named `passes/diffuse` and an empty name field wrote `…/nodebase/passes/diffuse0001.png`, in a new folder `passes`. With the node's format left as a fresh node has it (multi-layer OpenEXR), the same input wrote one file, `…/nodebase/0001.exr`, and no folder.

**Entry 82 — a broken template expression, seen in the window.** In Blender's window: Workbench, 320 × 240, frames 1 to 3, PNG, the output path set to `//out_{blend_name.png` — an opening brace with no closing one — and the file saved as `…/work/e82/broken.blend` so that `//` and `blend_name` have something to mean. The Properties editor was turned to its Output tab and photographed; then Ctrl-F12 was started as the menu starts it (`render.render`, `animation=True`, `use_viewport=True`, called as a key press calls it) from the 3D Viewport, the window photographed a second later, the Outliner turned into an Info editor, and the window photographed again:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e82.py)
```

What came back, in full:

```
WIN version 5.3.0 Beta background False event simulation True
Info: Saved as "broken.blend"
WIN saved …/work/e82/broken.blend path '//out_{blend_name.png'
WIN screenshot …/shots/e82_output_tab.png exists=True size=1140927 window=1512x862
WIN Ctrl-F12 -> {'RUNNING_MODAL'}
00:05.762  operator         | ERROR Parse errors in path '//out_{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
WIN screenshot …/shots/e82_after_ctrl_f12.png exists=True size=1183361 window=1512x862
WIN outliner area turned into INFO
WIN screenshot …/shots/e82_info.png exists=True size=1183290 window=1512x862
WIN out folder: ['broken.blend']
WIN quitting
00:09.687  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

What the pictures show. Not every change the script made was drawn by the time of the next picture: the first picture, taken two seconds after the switch to the Output tab, still shows the Object tab, and the last, taken a second and a half after the Outliner was turned into an Info editor, still shows the Outliner. In the second and third pictures the Output panel's path field reads `//out_{blend_name.png` and is **drawn in red**, and the status bar along the bottom of the window shows a red error mark and, word for word:

```
Parse errors in path '//out_{blend_name.png':- Invalid or incomplete template expression '{blend_name.png'.
```

Nothing was written: the folder held only `broken.blend`. No tooltip was photographed (see above), and the Info editor was not seen.

**Entry 85, case C — `//` with no saved file, in the window.** Written out in full under question 12. In a Blender with its window and no saved file, Ctrl-F12 with the output path `//frame_####` wrote nothing; the terminal printed `ERROR Couldn't create directory for file frame_0001.png: No such file or directory`, and the status bar and the Info editor showed, beside a red error mark, `Render error (No such file or directory) cannot save: 'frame_0001.png'`. Nothing was written into the folder Blender had been started from.

---

## 4. Does Blender make folders that are not already there when it writes?

**Answer.**
- **File Output node:** yes, for the sub-folders built from input names. The manual says missing folders are "created automatically during output". That sentence sits under the paragraph about input names that contain slashes, when Media Type is Image. Whether it also covers a missing base **Directory** is ambiguous: "Any missing directories" reads broadly, but its place on the page ties it to input-name folders.
- **Main scene render output** (Output panel or `-o`): the sources do not say.

**Label.** Documented for File Output node sub-folders. Not in the sources for the main render output.

**Quotes.**

> "When the Media Type is set to Image, input names are also used to construct subdirectories:" … "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy." … "Any missing directories are created automatically during output."

https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11

Related, but about other folders:

> "Temporary Files … The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead."

https://docs.blender.org/manual/en/5.3/editors/preferences/file_paths.html — read 2026-10-11. So Blender's temporary folder setting is not created when missing; Blender falls back instead. This is not about render output.

> "If the cache path folder does not exist, it will be created."

https://docs.blender.org/manual/en/5.3/modeling/modifiers/physics/ocean.html — read 2026-10-11. This is the Ocean modifier's bake cache, not render output. I ruled it out as an answer.

**Where I looked for the main render output.**
- Read in full: Output properties, command-line arguments, command-line rendering, rendering animations, render output introduction, file paths (templates), viewport render, the topbar Render menu, Cycles performance; the API pages for `RenderSettings`, `CompositorNodeOutputFile`, `bpy.ops.render`, `PreferencesFilePaths`.
- Searched the whole manual for: "create" or "created" next to directory, folder, sub-directory, sub-folder; "missing directory", "missing folder"; "mkdir"; "make directory", "make folder"; "does not exist", "doesn't exist", "do not exist", "not exist".
- Searched the render output pages, the compositor output node pages, the command-line pages, the Cycles render settings pages and the Image Editor pages for "director" and "folder".
- Searched the 5.3 release notes for "director", "folder", "path", "output".
- Searched the Cycles Python files for "directory", "makedirs", "mkdir".
- Hits were only the File Output node sentence, the Ocean cache, asset catalogs, bundle paths and other unrelated pages.

**Not covered.** The main render output case. Whether the File Output node's base Directory is created. What happens with a Multi-Layer EXR or Video media type in the File Output node. Folders for placeholders or the render cache.

**Outside the interface.** The sources do not say. Nothing found ties folder creation to how the render was started. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* a render with no window made every missing folder it was given — the scene's output folder one, two and three levels deep, from Python and from `-o` (entries 40, 45); a File Output node's own folder (entries 41, 61); the folder a node input's slash names (entries 42, 48); and the render cache folder (entry 49). Ctrl-F12 in Blender's window did the same (entry 85).

**Operating system.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 40 — does a render make a missing folder for the scene's own output path?** First from Python: at factory settings, no window, Workbench at 64 × 64, one frame, PNG, an animation rendered into `…/py/one_missing/img_` and then into `…/py/two_a/two_b/img_`, neither folder existing. Then from the command line alone, into `…/cli/a/b/frame_####`, frames 1 to 3, with `…/cli` not existing. The commands:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q3.py -- …/work/e40/py
find …/work/e40/py | sort
ls …/work/e40/cli
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -E BLENDER_WORKBENCH -o …/work/e40/cli/a/b/frame_#### -F PNG -s 1 -e 3 -a
find …/work/e40/cli | sort
```

What came back, in full:

```
00:00.723  render           | Saved: '…/work/e40/py/one_missing/img_0001.png'
00:00.731  render           | Saved: '…/work/e40/py/two_a/two_b/img_0001.png'
ONE LEVEL, path: …/work/e40/py/one_missing/img_
TWO LEVELS, path: …/work/e40/py/two_a/two_b/img_
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
…/work/e40/py
…/work/e40/py/one_missing
…/work/e40/py/one_missing/img_0001.png
…/work/e40/py/two_a
…/work/e40/py/two_a/two_b
…/work/e40/py/two_a/two_b/img_0001.png
ls: …/work/e40/cli: No such file or directory
00:00.348  render           | Rendering animation (frames 1..3)
00:00.348  render           | Rendering frame 1
00:00.348  render           | Start rendering: Scene, ViewLayer
00:00.348  render           | Engine: Workbench
00:00.637  render           | Saved: '…/work/e40/cli/a/b/frame_0001.png'
00:00.637  render           | Time: 00:00.28 (Saving: 00:00.16)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.637  render           | Rendering frame 2
00:00.637  render           | Start rendering: Scene, ViewLayer
00:00.637  render           | Engine: Workbench
00:00.759  render           | Saved: '…/work/e40/cli/a/b/frame_0002.png'
00:00.759  render           | Time: 00:00.12 (Saving: 00:00.10)
00:00.759  render           | Rendering frame 3
00:00.759  render           | Start rendering: Scene, ViewLayer
00:00.759  render           | Engine: Workbench
00:00.878  render           | Saved: '…/work/e40/cli/a/b/frame_0003.png'
00:00.878  render           | Time: 00:00.11 (Saving: 00:00.09)
exit code: 0
…/work/e40/cli
…/work/e40/cli/a
…/work/e40/cli/a/b
…/work/e40/cli/a/b/frame_0001.png
…/work/e40/cli/a/b/frame_0002.png
…/work/e40/cli/a/b/frame_0003.png
```

**Entry 41 — does a File Output node make its own missing folder?** At factory settings, no window, Workbench at 64 × 64, one frame: a File Output node with one input `Image`, its folder `…/node_a/node_b/` (two levels, neither existing), its name `nd_`, its format left as a fresh node has it; the scene's own output at `…/scene_out/sc_`. An animation rendered from Python:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q4.py -- …/work/e41
find …/work/e41 | sort
```

What came back, in full (the deprecation warnings for `Scene.compositing_node_group` left out):

```
00:00.473  render           | Saved: '…/work/e41/node_a/node_b/nd_0001.exr'
00:00.537  render           | Saved: '…/work/e41/scene_out/sc_0001.png'
node folder: …/work/e41/node_a/node_b
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
…/work/e41
…/work/e41/node_a
…/work/e41/node_a/node_b
…/work/e41/node_a/node_b/nd_0001.exr
…/work/e41/scene_out
…/work/e41/scene_out/sc_0001.png
```

**Entry 42 — does a slash inside a File Output input's name make folders?** At factory settings, no window, Workbench at 64 × 64, one frame: a File Output node whose one input is named `passes/diffuse`, its folder `…/nodebase/` (existing), its name field empty. Run twice: first with the node's format left as a fresh node has it, then with the node's format set to a still image first (`fo.format.media_type = 'IMAGE'`, `fo.format.file_format = 'PNG'`) and nothing else changed:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q5.py -- …/work/e42
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q5_png.py -- …/work/e42png
```

What came back, in full (deprecation warnings left out), the first:

```
00:00.465  render           | Saved: '…/work/e42/nodebase/0001.exr'
00:00.527  render           | Saved: '…/work/e42/scene_out/sc_0001.png'
item names: ['passes/diffuse']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
…/work/e42
…/work/e42/nodebase
…/work/e42/nodebase/0001.exr
…/work/e42/scene_out
…/work/e42/scene_out/sc_0001.png
```

and the second:

```
00:00.526  render           | Saved: '…/work/e42png/nodebase/passes/diffuse0001.png'
00:00.527  render           | Saved: '…/work/e42png/scene_out/sc_0001.png'
item names: ['passes/diffuse']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
…/work/e42png
…/work/e42png/nodebase
…/work/e42png/nodebase/passes
…/work/e42png/nodebase/passes/diffuse0001.png
…/work/e42png/scene_out
…/work/e42png/scene_out/sc_0001.png
```

**Entries 45 and 48 — folders made, as controls.** Written out in full under question 12. Under an ordinary writable folder, a render aimed at `open/a/b/c/f_` made all three missing levels and wrote frames 1 to 3 (entry 45); a File Output node whose input is named `passes/diffuse`, aimed at an existing writable folder, made a folder `n_passes` inside it and wrote `n_passes/diffuse0001.png` to `diffuse0003.png` (entry 48).

**Entry 49 — the render cache folder, missing under a writable folder.** Written out in full under question 12. With Cache Result on and the render cache folder set to `…/open/cachefolder`, which did not exist, the render made it and left `cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr` in it (2,468,413 bytes at the end).

**Entry 61 — a File Output node's folder made by the render.** Written out in full under question 11. A node aimed at `…/node/`, which did not exist (`node folder exists before the render: False`), had it made by the render, with `n_Image0001.png` to `n_Image0004.png` in it.

**Entry 85, case A — Ctrl-F12 into a missing folder, in the window.** Written out in full under question 12. With the output path `…/work/e85/missing/a/b/f_`, neither folder existing, Ctrl-F12 made both folders and wrote `f_0001.png` to `f_0003.png`; nothing appeared in the Info editor.

---

## 5. What does a write do when its folder is not there, when there is no room left, or when it cannot be written to?

**Answer.** Not in the sources. None of the three cases (missing folder, no room, no permission to write) is described for render output, File Output nodes or video output.

The documented render-output failures are different ones, so they do not answer this question:
- an animation render fails when the output path has template errors
- an error is reported when render output is turned off and no File Output node exists
- a video "might become unplayable" if a problem occurs while rendering

**Label.** Not in the sources.

**Where I looked.**
- The pages listed under question 4, plus the image formats and video formats pages, the File Output node page, the crash-log page, the Info editor page, and the Temporary Directory section of the directory layout page.
- Searched the whole manual, API docs, 5.3 release notes and Cycles Python files for:
  - "disk full", "out of disk", "out of space", "no space", "free space", "disk space", "drive space", "quota", "ENOSPC"
  - "permission", "permission denied", "access denied", "write access", "read-only", "read only", "write-protect", "not writable", "EACCES", "ENOENT"
  - "cannot write", "can't write", "unable to save", "unable to write", "could not save", "could not write", "failed to write", "failed to save", "error saving", "error writing", "write error", "I/O error"
  - "cannot create", "could not create", "unable to create", "failed to create", "cannot open", "could not open", "unable to open"
  - "network drive", "network share", "mounted"
- Searched the render output pages, the command-line rendering page, the compositor output node pages, the file paths page, the image and video format pages and the viewport render page for "error", "fail", "abort", "cancel", "stop". (The command-line arguments page I read in full instead.)
- Hits were: read-only extension repositories, a Windows note about running an external editor as administrator, OSL compile errors, Ocean cache and template errors. None are about a failed render write. I ruled them out.

**Not covered.** All three cases, for every way of starting a render.

**Outside the interface.** The sources do not say. The `--python-exit-code` option covers Python exceptions only ("Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line)", https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11). Nothing says a failed render write sets an exit code. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, a missing folder was made and nothing was said (entry 40). With no room left, the first frame was announced `Saved` while part-written, the second failed and the render stopped, exit code 1 (entry 44). Inside a read-only volume, the render stopped at frame 1, exit code 1 (entry 43). A broken template stopped the render, exit code 1 (entry 11). With Save Output off and nothing else to write, `-f` and `-a` wrote nothing, printed an error and ended with exit code 0 (entry 36). Each failure was printed on the terminal and, where Python started the render, raised as `RuntimeError`. In the window, the same failures appear in the status bar and the Info editor (entries 82, 85). A folder refused by its own permissions was not run for 5.3: the tool this sitting ran in refused `chmod`.

**Operating system.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. On this Mac the no-room failure named its cause (`No space left on device`) in every line (entry 44).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 11 — a broken template expression in the output path, with no window.** With the path `…/work/e11/{blend_name.png`, the path builder and an animation render started from Python both raised `RuntimeError` with the message `Error: Parse errors in path '…/work/e11/{blend_name.png':` and `- Invalid or incomplete template expression '{blend_name.png'.`, the same message was printed once more on its own, nothing was written, and Blender ended with exit code 1 although the script had caught the error. Written out in full under question 3.

**Entry 12 — a name longer than the computer accepts.** With a 300-character name, Blender printed `ERROR OpenImageIO write failed: Could not open file "…"` and `Error: Render error (Undefined error: 0) cannot save: '…'`, wrote nothing, and ended with exit code 1. Written out in full under question 10.

**Entry 36 — Save Output switched off, and nothing else to write.** First, at factory settings, no window, Workbench at 64 × 64, one frame, PNG: a script set `save_output` to False and rendered an animation from Python inside a `try`. Second, a file was saved with Workbench at 64 × 64, frames 1 to 2, PNG, the output path `…/work/e36/out/still_####` and `save_output` False, and that file rendered from the command line with `-f 1`, then with `-a`, the output folder listed after each:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q23.py -- …/work/e36/py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python save_off_make.py -- …/work/e36
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e36/save_off.blend -f 1
ls -la …/work/e36/out
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e36/save_off.blend -a
ls -la …/work/e36/out
```

What came back, in full:

```
has save_output: True
save_output set to False
Error: Render output disabled in Output properties and no active compositing File Output nodes
animation raised: RuntimeError Error: Render output disabled in Output properties and no active compositing File Output nodes
listing: no folder
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
Info: Saved as "save_off.blend"
saved; save_output = False ; filepath = '…/work/e36/out/still_####'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
=== -f 1
00:00.369  blend            | Read blend: "…/work/e36/save_off.blend"
00:00.374  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
ls: …/work/e36/out: No such file or directory
=== -a
00:00.407  blend            | Read blend: "…/work/e36/save_off.blend"
00:00.414  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
ls: …/work/e36/out: No such file or directory
```

**Entry 40 — a write whose folder is not there.** Written out in full under question 4. From Python and from the command line alone, a render aimed at a folder that did not exist — one level, two levels, and `cli/a/b` — made the missing folders, wrote every frame, printed no error and ended with exit code 0.

**Entry 44 — a write with no room left.** A two-megabyte disk image, made with `hdiutil` inside the working folder and mounted at `/Volumes/SMOOZYTINY` (1868 kilobytes free). At factory settings, no window: Workbench, 2000 × 2000, frames 1 to 3, uncompressed 32-bit OpenEXR, the output path `/Volumes/SMOOZYTINY/img_`, an animation rendered from Python inside a `try`. The volume was listed afterwards and detached:

```
hdiutil create -size 2m -fs HFS+ -volname SMOOZYTINY …/work/e44/tiny.dmg
hdiutil attach …/work/e44/tiny.dmg
df -k /Volumes/SMOOZYTINY
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q7.py
ls -la /Volumes/SMOOZYTINY
hdiutil detach /Volumes/SMOOZYTINY
```

What came back, in full:

```
/dev/disk4          	GUID_partition_scheme          	
/dev/disk4s1        	Apple_HFS                      	/Volumes/SMOOZYTINY
Filesystem   1024-blocks Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk4s1        2008  140      1868     7%       2 4294967277    0%   /Volumes/SMOOZYTINY
00:00.512  image.openexr    | ERROR imb_exr_write_channels: Failed to write pixel data to image file "/Volumes/SMOOZYTINY/img_0001.exr". No space left on device.
00:00.512  render           | Saved: '/Volumes/SMOOZYTINY/img_0001.exr'
00:00.553  image.openexr    | ERROR IMB_exr_write_end: No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/Volumes/SMOOZYTINY/img_0002.exr'
raised: RuntimeError Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/Volumes/SMOOZYTINY/img_0002.exr'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 1
total 3728
drwxr-xr-x  4 artist     staff      204 Oct 17 12:27 .
drwxr-xr-x  4 root       wheel      128 Oct 17 12:27 ..
drwx------+ 3 artist     staff      102 Oct 17 12:27 .fseventsd
-rw-r--r--@ 1 artist     staff  1908736 Oct 17 12:27 img_0001.exr
"disk4" ejected.
```

So: the first frame was announced `Saved` although the line above it says its pixel data could not be written, and a 1,908,736-byte `img_0001.exr` was left on the volume; frame 2 failed to save, frame 3 was never attempted, and Blender ended with exit code 1.

**Entry 43 — a write into a folder that exists and cannot be written to.** The folder `…/work/e43/ro` was a four-megabyte disk image mounted **read-only** there, so a write into it is refused because the volume is read-only. (The earlier versions' form of this test refused the write through the folder's own permissions instead; that could not be done in this sitting, and it is not run for 5.3.) A `touch` into the folder first proved the refusal. Then, at factory settings, no window, Workbench at 64 × 64, frames 1 to 3, PNG, an animation was rendered from Python into `…/ro/img_`; then a single frame from the command line alone into `…/ro/cli_`. The commands:

```
hdiutil create -size 4m -fs HFS+ -volname SMOOZYRO43 …/work/e43/ro.dmg
hdiutil attach -readonly -nobrowse -mountpoint …/work/e43/ro …/work/e43/ro.dmg
touch …/work/e43/ro/canary.txt ; echo "touch exit code: $?"
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q6.py -- …/work/e43
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -o …/work/e43/ro/cli_ -F PNG -x 1 -f 1
ls -la …/work/e43/ro
hdiutil detach …/work/e43/ro
```

What came back, in full:

```
created: …/work/e43/ro.dmg
/dev/disk4          	GUID_partition_scheme          	
/dev/disk4s1        	Apple_HFS                      	…/work/e43/ro
touch: …/work/e43/ro/canary.txt: Read-only file system
touch exit code: 1
00:00.639  image.write      | ERROR OpenImageIO write failed: Could not open file "…/work/e43/ro/img_0001.png"
Error: Render error (Undefined error: 0) cannot save: '…/work/e43/ro/img_0001.png'
raised: RuntimeError Error: Render error (Undefined error: 0) cannot save: '…/work/e43/ro/img_0001.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 1
00:00.370  render           | Rendering single frame
00:00.370  render           | Rendering frame 1
00:00.370  render           | Start rendering: Scene, ViewLayer
00:00.370  render           | Engine: EEVEE
00:00.530  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:01.043  render           | Fra: 1 | Rendering 25 / 64 samples
00:01.351  render           | Fra: 1 | Rendering 50 / 64 samples
00:01.517  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.649  image.write      | ERROR OpenImageIO write failed: Could not open file "…/work/e43/ro/cli_0001.png"
00:01.649  reports          | ERROR Render error (Undefined error: 0) cannot save: '…/work/e43/ro/cli_0001.png'
00:01.649  render           | Time: 00:01.27
exit code: 1
total 0
drwxr-xr-x  2 artist     staff   68 Oct 17 12:29 .
drwxr-xr-x@ 4 artist     wheel  128 Oct 17 12:29 ..
"disk4" ejected.
```

From Python the render stopped at frame 1 — frames 2 and 3 have no line — and raised `RuntimeError`; from the command line one frame was asked for and failed. Both ended with exit code 1 and the folder was empty. The command-line run used the factory engine, EEVEE, because it set none.

**Entries 45 and 46 — when the folder a write needs cannot be made.** Written out in full under question 12. Inside a read-only volume, the render stopped at frame 1 with `ERROR Couldn't create directory for file …: Read-only file system` and `Error: Render error (Read-only file system) cannot save: '…'`, raised `RuntimeError` to Python, and ended with exit code 1. Where an ordinary file stood in the folder's place, the same two lines read `File exists`, with exit code 1.

**Entry 82 — a broken template expression and Ctrl-F12, in the window.** Written out in full under question 3. With the output path `//out_{blend_name.png`, Ctrl-F12 wrote nothing; the terminal printed `ERROR Parse errors in path '//out_{blend_name.png':` and `- Invalid or incomplete template expression '{blend_name.png'.`, and the window's status bar showed the same words beside a red error mark; the path field was drawn in red.

**Entry 85 — what the window says.** Written out in full under question 12. With Ctrl-F12 aimed inside a read-only folder, the status bar and the Info editor both showed, beside a red error mark, `Render error (Read-only file system) cannot save: '…/locked/newfolder/f_0001.png'`.

---

## 6. What are the ways a render can be started, and what does each of them write?

**Answer.**

*From the interface, the Render menu:*
- **Render Image (F12).** Renders the current frame. The image is not saved automatically. Files are written only by active File Output nodes, or by the person saving from the Image Editor.
- **Render Animation (Ctrl-F12).** Renders the frame range. With Save Output on (the default), it writes frames, or one video file, to the output path. With Save Output off, it relies on File Output nodes, and reports an error if there are none.
- **Render Sequencer Image (Alt-F12) and Render Sequencer Animation (Ctrl-Alt-F12).** Render the Sequencer Scene with that scene's own output settings. That they write the same way as the two items above is inferred.
- **Render Audio.** Mixes the scene's sound to a sound file.

*From the 3D Viewport (Workbench and EEVEE only, not Cycles):*
- **Render Viewport Preview** (a still), **Render Playblast** (an animation), **Render Playblast on Keyframes.** These use the scene's output path and file format. Playblast on Keyframes writes every frame, repeating the last rendered one where there is no key. Both playblasts raise an error when render output is turned off in the Output panel.

*From the command line:*
- **`-f`** renders the given frame or frames and saves them. A frame list and a range are allowed.
- **`-a`** renders the frame range.

*From Python:*
- **`bpy.ops.render.render`.** With `animation=True` it renders the range. Otherwise it renders a still, saved to the output path only if `write_still=True`. The default is False.
- **`bpy.ops.render.opengl`** (viewport render). It has the same `animation` and `write_still` switches, plus `render_keyed_only` and `sequencer`.

*Extra writes that come with a render, when switched on:*
- Cache Result writes a multi-layer EXR to the Render Cache folder.
- Placeholders writes empty placeholder files.
- Cycles caches tiles to disk for high-resolution images.
- Cycles Auto Generate writes `.tx` texture files.

**Label.** Documented, except the sequencer items, which are inferred.

**Quotes.**

> "Render Image F12 … Render the active scene at the current frame." / "Render Animation Ctrl-F12 … Render the animation of the active scene." / "Render Sequencer Image Alt-F12 … If a Sequencer Scene exists that differs from the active scene, render that scene instead." / "Render Sequencer Animation Ctrl-Alt-F12 … Render the animation of the Sequencer Scene." / "Render Audio … Mix the scene’s audio to a sound file."

https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html — read 2026-10-11

> "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node."
> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."
> "Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."
> "Saving – Cache Result … Saves the rendered view layers and their passes to a multi-layer OpenEXR image." … "The image is stored in the Render Cache folder as specified in the File Paths Preferences."

https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11

> "The render and output settings of the Sequencer Scene are the same as for regular scenes."

https://docs.blender.org/manual/en/5.3/video_editing/sequencer_scene.html — read 2026-10-11

> "Viewport rendering only works for the Workbench and EEVEE render engines. It’s not supported for Cycles." … "Additionally, some output settings are used too:" followed by "Resolution", "Aspect", "Output path", "File format" … "The other frames are still written to the output, but will simply repeat the last-rendered frame." *(Changed by citation check: quote corrected to match the list in viewport_render.txt without added slashes).*

https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11

> "-a, --render-anim … Render frames from start to end (inclusive)." / "-f, --render-frame <frame> … Render frame <frame> and save it." / "A comma separated list of frames can also be used (no spaces)." / "A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"
> "bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)"

https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11

> "The File Output node writes one or more images to disk during rendering or compositing."

https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11

> "use_auto_generate_texture_cache … Automatically create tx files from image files when rendering, if the files do not exist or are outdated."

https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11

> "Tiles are cached to disk while rendering to save memory."

https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html — read 2026-10-11

**Step behind the inference.** The sequencer menu items render "that scene", and the Sequencer Scene's output settings "are the same as for regular scenes". From that I take it that they write as Render Image and Render Animation do. No source says so in so many words.

**Not covered.**
- Whether `-f` still writes when Save Output is off. `save_output` says "for animation renders".
- Whether a still from `-f` gets a frame number when the path has no `#` (see question 8).
- Whether File Output nodes write during viewport renders.
- Where Render Audio's file goes by default.
- Whether "during … compositing" includes the interactive compositor writing files.

**Outside the interface.** `-f`, `-a` and the Python operators work without the interface (the manual shows them with `-b`). F12, the Image Editor's manual save and the 3D Viewport menu items are interface routes. `bpy.ops.render.opengl` takes "a snapshot of the active viewport". Whether it works in background mode, where there is no viewport, is not stated. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, `-f`, `-a` and the Python call wrote as recorded (entries 8, 35, 50); the viewport render call was refused with no window (`Cannot use OpenGL render in background mode (no opengl context)`, entry 34) and worked with one (entry 79); with Save Output off, `-f` and `-a` wrote nothing, printed an error and ended with exit code 0 (entry 36). In Blender's window, F12 wrote nothing at the output path and Ctrl-F12 wrote the frames (entry 78); the Render menu holds Render Image, Render Animation, Render Audio..., View Render, View Animation and Lock Interface (entry 77).

**Operating system.** The sources do not say the routes differ by system, apart from the keyboard shortcuts, which are given only in one form. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. The menu's shortcuts were shown with the Command key, `⌘ F12`, on this Mac (entry 77).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 1 — the switch for writing frames at all.** Blender 5.3 at factory settings, no window, a script printing whether the scene's render settings carry `save_output` and its value. It printed `save_output True True` — the switch exists and is on. The full output is under question 2.

**Entry 10 — what an animation render writes when the output is a video, and how the video format is reached.** An animation of frames 1 to 5, with the media type set to Video, the format FFmpeg and the container Matroska, wrote one file, `mov_0001-0005.mkv`, and printed one `Video append frame N` line per frame and no `Saved:` line (written out in full under question 8). Then, in a fresh launch at factory settings with nothing set first, the format alone was set to `'FFMPEG'`:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python c1_ff.py
```

which printed, in full:

```
FF version 5.3.0 Beta
FF direct: REFUSED TypeError('bpy_struct: item.attr = val: enum "FFMPEG" not found in (\'AVIF\', \'JPEG\', \'OPEN_EXR\', \'PNG\', \'WEBP\', \'BMP\', \'CINEON\', \'DPX\', \'IRIS\', \'JPEG2000\', \'HDR\', \'TARGA\', \'TARGA_RAW\', \'TIFF\')')
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

So with the media type at its factory value, Image, `'FFMPEG'` is not among the formats offered; the run in entry 10 that set the media type to Video first was able to set it.

**Entry 34 — the viewport render call with no window.** At factory settings, no window, the engine set to Workbench (the script asks for EEVEE under the name `BLENDER_EEVEE_NEXT` and falls back to Workbench when that name is not offered, as here), 64 × 64, one frame, PNG. `bpy.ops.render.opengl` was called three ways:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q21.py -- …/work/e34
```

What came back, in full:

```
engine: BLENDER_WORKBENCH
--- calling bpy.ops.render.opengl(**{'write_still': True, 'view_context': True})
Error: Cannot use OpenGL render in background mode (no opengl context)
   raised: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
--- calling bpy.ops.render.opengl(**{'write_still': True, 'view_context': False})
Error: Cannot use OpenGL render in background mode (no opengl context)
   raised: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
--- calling bpy.ops.render.opengl(**{'animation': True, 'view_context': False})
Error: Cannot use OpenGL render in background mode (no opengl context)
   raised: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
listing: no folder
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
```

**Entry 35 — what `-f`, `-a` and the Python render call each write.** A file holding a scene output path and a File Output node (folder `…/work/e15png/node_out/`, name `nd_`, one input `Image`, writing PNG), one frame, copied once per route. Each copy was rendered with `-o` pointing the scene's output at a folder of its own: with `-f 1`; with `-a`; and with a Python script calling `bpy.ops.render.render` three times — `animation=False, write_still=False`, then `animation=False, write_still=True`, then `animation=True`. The node's folder, shared by all three, was listed with each file's time before the first and after each:

```
ls -lT …/work/e15png/node_out
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e35/r_f.blend -o …/work/e35/f_scene/s_ -F PNG -x 1 -f 1
ls -lT …/work/e15png/node_out
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e35/r_a.blend -o …/work/e35/a_scene/s_ -F PNG -x 1 -a
ls -lT …/work/e15png/node_out
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e35/r_py.blend -o …/work/e35/py_scene/s_ -F PNG -x 1 --python q22.py
ls -lT …/work/e15png/node_out
find …/work/e35 -type f -not -name '*.blend' | sort
```

What came back, in full:

```
total 8
-rw-r--r--@ 1 artist     wheel  3544 Oct 17 12:20:36 2026 nd_Image0001.png
00:00.343  blend            | Read blend: "…/work/e35/r_f.blend"
00:00.347  render           | Rendering single frame
00:00.348  render           | Rendering frame 1
00:00.348  render           | Start rendering: Scene, ViewLayer
00:00.348  render           | Engine: Workbench
00:00.452  render           | Executing compositor
00:00.452  render           | Fra: 1 | Compositing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.517  render           | Saved: '…/work/e15png/node_out/nd_Image0001.png'
00:00.518  render           | Saved: '…/work/e35/f_scene/s_0001.png'
00:00.518  render           | Time: 00:00.17 (Saving: 00:00.00)
exit code: 0
total 8
-rw-r--r--@ 1 artist     wheel  3540 Oct 17 12:25:01 2026 nd_Image0001.png
00:00.343  blend            | Read blend: "…/work/e35/r_a.blend"
00:00.348  render           | Rendering single frame
00:00.348  render           | Rendering frame 1
00:00.348  render           | Start rendering: Scene, ViewLayer
00:00.348  render           | Engine: Workbench
00:00.448  render           | Executing compositor
00:00.448  render           | Fra: 1 | Compositing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.510  render           | Saved: '…/work/e15png/node_out/nd_Image0001.png'
00:00.511  render           | Saved: '…/work/e35/a_scene/s_0001.png'
00:00.511  render           | Time: 00:00.16 (Saving: 00:00.00)
exit code: 0
total 8
-rw-r--r--@ 1 artist     wheel  3540 Oct 17 12:25:02 2026 nd_Image0001.png
00:00.357  blend            | Read blend: "…/work/e35/r_py.blend"
00:00.532  render           | Saved: '…/work/e15png/node_out/nd_Image.png'
00:00.538  render           | Saved: '…/work/e15png/node_out/nd_Image.png'
00:00.539  render           | Saved: '…/work/e35/py_scene/s_.png'
00:00.545  render           | Saved: '…/work/e15png/node_out/nd_Image0001.png'
00:00.545  render           | Saved: '…/work/e35/py_scene/s_0001.png'
--- animation=False, write_still=False
    {'FINISHED'}
--- animation=False, write_still=True
    {'FINISHED'}
--- animation=True
    {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
total 16
-rw-r--r--@ 1 artist     wheel  3541 Oct 17 12:25:03 2026 nd_Image.png
-rw-r--r--@ 1 artist     wheel  3541 Oct 17 12:25:03 2026 nd_Image0001.png
…/work/e35/a_scene/s_0001.png
…/work/e35/f_scene/s_0001.png
…/work/e35/py_scene/s_.png
…/work/e35/py_scene/s_0001.png
```

**Entry 36 — Save Output switched off.** Written out in full under question 5: from Python, an animation render with `save_output` False and no File Output node raised `RuntimeError` and wrote nothing; from the command line, a file saved with `save_output` False wrote nothing with `-f 1` or with `-a`, printed `ERROR Render output disabled in Output properties and no active compositing File Output nodes`, and ended with exit code 0.

**Entry 50 — a still from Python and a single frame from `-f`.** Written out in full under question 11. A still rendered from Python with no save switch left the empty output folder empty; the same with `write_still=True` wrote `still_.png`; `-f 1` from the command line wrote `cli_0001.png`.

**Entry 60 — the `.tx` files a render can write.** Written out in full under question 11. With the texture-cache switches at factory settings (`use_texture_cache` True, `use_auto_generate_texture_cache` False), a Cycles render using a PNG texture wrote nothing beside the texture. With `use_auto_generate_texture_cache` also on, it made a folder `blender_tx` beside the texture holding `smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx`; the preference `texture_cache_directory` was `''`.

**How entries 77 to 86 used Blender's window.** Blender 5.3 was started from the terminal **with** its window, on the Creator's screen, using a settings folder of its own with the splash screen switched off and Python tooltips switched on, and with `--enable-event-simulate`, which makes Blender take only simulated pointer and key events and ignore the real mouse and keyboard. A script then drove it on a fixed timetable — simulated pointer moves, clicks and keys at window coordinates, and Blender's own operators — took pictures of the window with Blender's own screenshot command (`bpy.ops.screen.screenshot`), and quit Blender. The pictures are 3024 × 1724 pixels and were looked at directly. Every script and coordinate is in `Audits/Blender Research/Writing/Hand tests — 5.3.md`.

**No tooltip appeared in any picture.** In entry 77 and in a separate trial, the simulated pointer rested on menu entries and on a field for 1.8 to 2.5 seconds; each picture shows the item highlighted under the pointer and no tooltip. So the words a tooltip would show were not photographed; where an entry needed them, Blender's own names and descriptions for the call or setting, as its Python interface reports them, were printed by a script instead. They are not a picture of a tooltip, and nothing here says a tooltip would show the same words.

**Entry 77 — what the Render menu holds, and which call each entry makes.** First the menu's own drawing code was run against a stand-in that prints each entry, the call it makes and every setting it gives that call, instead of drawing it. Then the menu was opened by a simulated click on "Render" in the top bar and photographed. Then, in a second launch with no window, a script printed Blender's own name and description for each call the menu makes:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_menu.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python opdesc.py -- render.render sound.mixdown render.view_show render.play_rendered_anim prop:RenderSettings.use_lock_interface
```

What the first printed:

```
WIN version 5.3.0 Beta event simulation True python tooltips True
MENU === TOPBAR_MT_render (label 'Render') ===
MENU entry: text='Render Image' calls bpy.ops.render.render  {'icon': 'RENDER_STILL'}
MENU       sets render.render.use_viewport = True
MENU entry: text='Render Animation' calls bpy.ops.render.render  {'icon': 'RENDER_ANIMATION'}
MENU       sets render.render.animation = True
MENU       sets render.render.use_viewport = True
MENU ----
MENU entry: text='Render Audio...' calls bpy.ops.sound.mixdown
MENU ----
MENU entry: text='View Render' calls bpy.ops.render.view_show
MENU entry: text='View Animation' calls bpy.ops.render.play_rendered_anim
MENU ----
MENU setting: RenderSettings.use_lock_interface text='Lock Interface'
WIN screenshot …/shots/e77_menu_open.png exists=True size=1136547 window=1512x862
WIN screenshot …/shots/e77_tip_render_image.png exists=True size=1137126 window=1512x862
WIN screenshot …/shots/e77_tip_render_animation.png exists=True size=1136547 window=1512x862
WIN screenshot …/shots/e77_tip_render_audio.png exists=True size=1137234 window=1512x862
WIN screenshot …/shots/e77_tip_view_render.png exists=True size=1136995 window=1512x862
WIN screenshot …/shots/e77_tip_view_animation.png exists=True size=1137085 window=1512x862
WIN screenshot …/shots/e77_tip_lock_interface.png exists=True size=1136547 window=1512x862
WIN quitting
00:20.389  blend            | Saved session recovery to "…/tmp/quit.blend"
exit code: 0
```

What the picture of the open menu (`e77_menu_open.png`) shows, top to bottom: **Render Image** `F12`, **Render Animation** `⌘ F12`, a line, **Render Audio...**, a line, **View Render** `F11`, **View Animation** `⌘ F11`, a line, and an unticked checkbox **Lock Interface**. The six later pictures show the same menu with the entry under the pointer highlighted in turn, and no tooltip.

What the second printed:

```
DESC bpy.ops.render.render: name='Render' description='(undocumented operator)'
DESC     property animation: name='Animation' description='Render files from the animation range of this scene' default=False
DESC     property write_still: name='Write Image' description='Save the rendered image to the output path (used only when animation is disabled)' default=False
DESC     property use_viewport: name='Use 3D Viewport' description='When inside a 3D viewport, use layers and camera of the viewport' default=False
DESC bpy.ops.sound.mixdown: name='Mixdown' description="Mix the scene's audio to a sound file"
DESC bpy.ops.render.view_show: name='Show/Hide Render View' description='Toggle show render view'
DESC bpy.ops.render.play_rendered_anim: name='Play Rendered Animation' description='Play back rendered frames/movies using an external player'
DESC property RenderSettings.use_lock_interface: name='Lock Interface' description='Lock interface during rendering in favor of giving more memory to the renderer'
```

No entry for rendering a Sequencer Scene was drawn or shown, in Blender's factory start-up file.

**Entry 78 — does F12 write a file, and Ctrl-F12; a still from a script with a window open; and Blender's temporary folder during a large F12.** In Blender's window (as described above), with Workbench, 320 × 240, frames 1 to 3, PNG, the output path `…/out/img_` in an empty folder. F12 and Ctrl-F12 were started the way entry 77 found the menu starts them — `render.render` with `use_viewport=True`, and with `animation=True` as well for Ctrl-F12 — called as a key press calls them (`'INVOKE_DEFAULT'`) from the 3D Viewport, and the folder listed a fixed five or six seconds later. Then the still script of entry 50 ran in the same window (Cycles, 8 samples). Then a large F12 — Cycles, 16 samples, adaptive sampling and denoising off, 3840 × 2160, tile size 512, Cache Result off — with Blender's temporary folder listed every second for thirty seconds:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e78.py)
```

What came back, in full apart from Cycles' progress lines:

```
WIN version 5.3.0 Beta background False event simulation True
WIN pressed F12 (render.render use_viewport=True) -> {'RUNNING_MODAL'}
WIN out folder after F12: []
WIN screenshot …/shots/e78_after_f12.png exists=True size=1063528 window=1512x862
WIN pressed Ctrl-F12 (render.render animation=True use_viewport=True) -> {'RUNNING_MODAL'}
00:10.591  render           | Saved: '…/work/e78/out/img_0001.png'
00:10.605  render           | Saved: '…/work/e78/out/img_0002.png'
00:10.615  render           | Saved: '…/work/e78/out/img_0003.png'
WIN out folder after Ctrl-F12: ['img_0001.png', 'img_0002.png', 'img_0003.png']
WIN screenshot …/shots/e78_after_ctrl_f12.png exists=True size=1063528 window=1512x862
WIN BEFORE: []
WIN AFTER render, no save switch: []
00:17.965  render           | Saved: '…/work/e78/still/still_.png'
WIN AFTER render with the save switch: ['still_.png']
WIN big F12: tempdir = …/tmp/blender_Jut6PO/  render_cache_directory = ''
WIN tempdir before: []
WIN pressed F12 (render.render use_viewport=True) -> {'RUNNING_MODAL'}
WIN tempdir during, 1 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 2 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 3 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 4 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 5 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 6 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 7 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 8 s after F12: ['cycles-tile-buffer-79564-32243420144-2-0.exr']  big folder: []
WIN tempdir during, 9 s after F12: []  big folder: []
WIN tempdir during, 10 s after F12: []  big folder: []
WIN tempdir during, 11 s after F12: []  big folder: []
WIN tempdir during, 12 s after F12: []  big folder: []
WIN tempdir during, 13 s after F12: []  big folder: []
WIN tempdir during, 14 s after F12: []  big folder: []
WIN tempdir during, 15 s after F12: []  big folder: []
WIN tempdir during, 16 s after F12: []  big folder: []
WIN tempdir during, 17 s after F12: []  big folder: []
WIN tempdir during, 18 s after F12: []  big folder: []
WIN tempdir during, 19 s after F12: []  big folder: []
WIN tempdir during, 20 s after F12: []  big folder: []
WIN tempdir during, 21 s after F12: []  big folder: []
WIN tempdir during, 22 s after F12: []  big folder: []
WIN tempdir during, 23 s after F12: []  big folder: []
WIN tempdir during, 24 s after F12: []  big folder: []
WIN tempdir during, 25 s after F12: []  big folder: []
WIN tempdir during, 26 s after F12: []  big folder: []
WIN tempdir during, 27 s after F12: []  big folder: []
WIN tempdir during, 28 s after F12: []  big folder: []
WIN tempdir during, 29 s after F12: []  big folder: []
WIN tempdir during, 30 s after F12: []  big folder: []
WIN screenshot …/shots/e78_big_f12.png exists=True size=1148375 window=1512x862
WIN tempdir at the end: []  big folder: []
WIN quitting
00:51.076  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

The pictures Blender took of its main window after F12 and after Ctrl-F12 are the same picture, byte for byte: the Layout workspace, with no render result in it. Blender's screenshot command photographs one window, the first, and nothing here says whether another window was open.

**Entry 79 — the 3D Viewport's own render entries, with a real window.** In Blender's window: Workbench, 320 × 240, frames 1 to 3, PNG, the output path `…/out/vp_` in an empty folder. The run started the still entry and the animation entry the way the View menu's own drawing code shows they start — `render.opengl` with no setting, and with `animation=True` — as a click starts them (`'INVOKE_DEFAULT'`), from the 3D Viewport, listing the folder a fixed four and six seconds later. Then three direct calls of `render.opengl` into a second folder, `…/direct/vp_`. The first run was made into an empty folder; it also tried to read the View menu's drawing code before the window had a 3D Viewport in context, which failed, and to open the menu by a simulated click, which did not open it. A second run read the drawing code inside the 3D Viewport and tried to open the menu with Blender's own menu-opening call; its folder already held the first run's three files, so only its first part is quoted. A third trial with a slower click did not open the menu either. Then, with no window, Blender's own name and description for `render.opengl` were printed:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e79.py; twice)
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python opdesc.py -- render.opengl
```

The first run, in full:

```
WIN version 5.3.0 Beta background False event simulation True
MENU === VIEW3D_MT_view (label 'View') ===
MENU setting: NoneType.show_region_toolbar text=None
MENU setting: NoneType.show_region_ui text=None
MENU setting: NoneType.show_region_tool_header text=None
MENU setting: NoneType.show_region_asset_shelf text=None
MENU setting: NoneType.show_region_hud text=None
MENU ----
MENU entry: text='Frame Selected' calls bpy.ops.view3d.view_selected
MENU drawing code raised AttributeError: 'NoneType' object has no attribute 'region_quadviews'
WIN screenshot …/shots/e79_view_menu.png exists=True size=1146494 window=1512x862
WIN menu still entry render.opengl({}) -> {'RUNNING_MODAL'}
WIN out after the still entry: []
WIN screenshot …/shots/e79_after_still.png exists=True size=1146494 window=1512x862
WIN menu animation entry render.opengl({'animation': True}) -> {'RUNNING_MODAL'}
00:12.145  render           | Saved: '…/work/e79/out/vp_0001.png'
00:12.158  render           | Saved: '…/work/e79/out/vp_0002.png'
00:12.171  render           | Saved: '…/work/e79/out/vp_0003.png'
WIN out after the animation entry: ['vp_0001.png', 'vp_0002.png', 'vp_0003.png']
WIN screenshot …/shots/e79_after_anim.png exists=True size=1146494 window=1512x862
00:19.129  render           | Saved: '…/work/e79/direct/vp_.png'
00:19.129  render           | OpenGL Render written to '…/work/e79/direct/vp_.png'
WIN direct render.opengl({'write_still': True, 'view_context': True}) -> {'FINISHED'}
00:19.138  render           | Saved: '…/work/e79/direct/vp_.png'
00:19.138  render           | OpenGL Render written to '…/work/e79/direct/vp_.png'
WIN direct render.opengl({'write_still': True, 'view_context': False}) -> {'FINISHED'}
00:19.150  render           | Saved: '…/work/e79/direct/vp_0001.png'
00:19.155  render           | Saved: '…/work/e79/direct/vp_0002.png'
00:19.157  render           | Saved: '…/work/e79/direct/vp_0003.png'
WIN direct render.opengl({'animation': True, 'view_context': False}) -> {'FINISHED'}
WIN direct folder: ['vp_.png', 'vp_0001.png', 'vp_0002.png', 'vp_0003.png']
WIN quitting
00:20.119  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

The second run, the menu's drawing code and the call that opens it (the remainder repeated the renders into a folder that already held `vp_0001.png` to `vp_0003.png`):

```
WIN version 5.3.0 Beta background False event simulation True
MENU === VIEW3D_MT_view (label 'View') ===
MENU setting: SpaceView3D.show_region_toolbar text=None
MENU setting: SpaceView3D.show_region_ui text=None
MENU setting: SpaceView3D.show_region_tool_header text=None
MENU setting: SpaceView3D.show_region_asset_shelf text=None
MENU setting: SpaceView3D.show_region_hud text=None
MENU ----
MENU entry: text='Frame Selected' calls bpy.ops.view3d.view_selected
MENU entry: text=None calls bpy.ops.view3d.view_all
MENU       sets view3d.view_all.center = False
MENU entry: text='Perspective/Orthographic' calls bpy.ops.view3d.view_persportho
MENU submenu: VIEW3D_MT_view_local text=None
MENU setting: SpaceView3D.show_viewer text='Viewer Node'
MENU ----
MENU submenu: VIEW3D_MT_view_cameras text='Cameras'
MENU ----
MENU submenu: VIEW3D_MT_view_viewpoint text=None
MENU submenu: VIEW3D_MT_view_navigation text=None
MENU submenu: VIEW3D_MT_view_align text=None
MENU ----
MENU layout.operator_context = 'INVOKE_REGION_WIN'
MENU submenu: VIEW3D_MT_view_regions text='View Regions'
MENU ----
MENU entry: text='Play Animation' calls bpy.ops.screen.animation_play
MENU ----
MENU entry: text='Render Viewport Preview' calls bpy.ops.render.opengl  {'icon': 'RENDER_STILL'}
MENU entry: text='Render Playblast' calls bpy.ops.render.opengl  {'icon': 'RENDER_ANIMATION'}
MENU       sets render.opengl.animation = True
MENU entry: text='Render Playblast on Keyframes' calls bpy.ops.render.opengl  {'icon': 'RENDER_ANIMATION'}
MENU       sets render.opengl.animation = True
MENU       sets render.opengl.render_keyed_only = True
MENU ----
MENU submenu: INFO_MT_area text=None
WIN call_menu VIEW3D_MT_view -> {'INTERFACE'}
```

None of the three pictures of the window shows the View menu open. The description lines:

```
DESC bpy.ops.render.opengl: name='Viewport Render' description='Take a snapshot of the active viewport'
DESC     property animation: name='Animation' description='Render files from the animation range of this scene' default=False
DESC     property render_keyed_only: name='Render Keyframes Only' description='Render only those frames where selected objects have a key in their animation data. Only used when rendering animation' default=False
DESC     property sequencer: name='Sequencer' description="Render using the sequencer's OpenGL display" default=False
DESC     property write_still: name='Write Image' description='Save the rendered image to the output path (used only when animation is disabled)' default=False
DESC     property view_context: name='View Context' description='Use the current 3D view for rendering, else use scene settings' default=True
```

So the View menu's drawing code holds three render entries — **Render Viewport Preview** (`render.opengl` with nothing set), **Render Playblast** (`animation = True`) and **Render Playblast on Keyframes** (`animation = True`, `render_keyed_only = True`). Started as a click starts it, the still entry wrote nothing into the empty output folder; the animation entry wrote `vp_0001.png` to `vp_0003.png`. Called directly with `write_still=True`, the viewport render wrote `vp_.png` and printed `OpenGL Render written to '…'`, with the 3D view's own context and without it; with `animation=True` it wrote three numbered frames.

**Entry 80 — the Save Output switch in the window.** Written out in full under question 2. The Output panel's header carries a ticked checkbox; Blender names the setting behind it `Save Output` and describes it as `Write frames to disk for animation renders`.

---

## 7. If a render can be set up so that it writes no files, what does it leave behind?

**Answer.** Yes, for a still. A still render (F12, or the Python operator with `write_still` left False), with no active File Output node, writes no image file. It leaves the picture as the **Render Result** inside Blender. That result is temporary: it is lost on the next render, on loading a new file, or on closing Blender, unless saved by hand.

An animation render cannot be set to write nothing through these settings. With Save Output off and no File Output node, an error is reported instead.

Things the sources say a render may put on disk even when no output file is written:
- **Cache Result**, only if turned on: an EXR in the Render Cache folder. It can be loaded back after restarting Blender.
- **Cycles tiles**, for high-resolution images: "cached to disk while rendering". The sources do not say where, or whether they are removed afterwards.
- **Cycles `.tx` files**, if Auto Generate is on. The manual says they go in a `blender_tx/` folder next to each image by default. The API says a blank setting means "the same directory as image files". The two pages differ.
- The **temporary folder** is used for run-time files "including render layers". The sources do not say which renders put files there or whether they are cleared.

**Label.** Documented that still renders are not saved, that the Render Result is temporary, and the animation error. The list of possible leftovers is documented item by item. Whether any of them is actually present after a no-output render is inferred or not stated, as marked.

**Quotes.**

> "When enabled, Blender behaves in the standard way: still renders (F12) are not saved automatically, while animation renders (Ctrl-F12) are written to the configured output path."
> "For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11

> "Render Result images and Compositor Viewer node results are temporary and are not automatically saved to disk when saving the blend-file or closing Blender." … "Failing to save these images before rendering again, loading a new file, or closing Blender will cause the image data to be lost."

https://docs.blender.org/manual/en/5.3/files/media/image_formats.html — read 2026-10-11

> "Note that Blender doesn’t create these cache files by default. You have to enable Cache Result in the scene’s Output options and then render it at least once."

https://docs.blender.org/manual/en/5.3/editors/image/editing.html — read 2026-10-11

> "Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory." … "By default, files are stored in the blender_tx/ folder next to the image file."

https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html — read 2026-10-11

> "texture_cache_directory … Leave blank to store tx files in the same directory as image files"

https://docs.blender.org/api/5.3/bpy.types.PreferencesFilePaths.html — read 2026-10-11

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html — read 2026-10-11

**Not covered.**
- Where the Cycles tile cache is written, and whether it is deleted when the render ends or is cancelled.
- Whether a plain still render puts anything in the temporary folder.
- Whether placeholders are made when Save Output is off.
- Whether a render run with `-b` keeps its Render Result anywhere after Blender exits (inferred: no, since the result is lost on closing Blender).
- Two things written on events, not by a render: a `quit.blend` written when the interface is quit, and crash logs. I did not count them.

**Outside the interface.** The documented "still renders are not saved automatically" refers to F12. From the command line, `-f` is described as "Render frame <frame> and save it". So the documented still-with-no-file route belongs to the interface, or to Python with `write_still` False. Whether `-f` can be made to write nothing is not stated. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, a still not asked to save wrote nothing and Blender held its Render Result (entries 33, 50); Cycles made one tile file in Blender's temporary folder for pictures larger than one tile and removed it before exiting (entries 37, 56 to 58); a `blender_tx` folder appeared beside a texture only with Auto Generate on (entry 60); on the graphics card Cycles also wrote compiled kernels into `~/.cache/cycles/kernels` in the user's home folder (entry 56). In the window, F12 wrote nothing at the output path (entry 78).

**Operating system.** The temporary folder is chosen differently by system: "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms)" (https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html). The command-line page instead names "TMPDIR … (UNIX Systems)" (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). The two pages disagree on the name for non-Windows systems. Nothing else here is said to differ by system. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. On this Mac every tile file appeared under the folder `TMPDIR` named, in a `blender_XXXXXX` folder of its own (entries 56 to 58, 72).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 1 — the switch for writing frames at all.** Blender 5.3 at factory settings, no window, a script printing whether the scene's render settings carry `save_output` and its value. It printed `save_output True True` — the switch exists and is on. The full output is under question 2.

**Entry 33 — what a windowless render that writes nothing leaves behind.** At factory settings, no window, Workbench at 64 × 64, one frame, PNG, Cache Result and Placeholders off, the output path pointed into an empty folder `should_stay_empty`. The script printed Blender's temporary folder and the images Blender held, rendered a still from Python with `write_still=False`, then printed the images again and listed the temporary folder and the output folder:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q20.py -- …/work/e33
find …/work/e33 | sort
```

What came back, in full:

```
temp dir: …/tmp/blender_HJfgGI/
images before: [('Render Result', 'RENDER_RESULT'), ('Viewer Node', 'COMPOSITING')]
returned: {'FINISHED'}
images after : [('Render Result', 'RENDER_RESULT', (0, 0)), ('Viewer Node', 'COMPOSITING', (256, 256))]
tempdir listing: []
output folder listing: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
…/work/e33
…/work/e33/should_stay_empty
```

The image list reads: name, kind, and, after the render, width and height as Blender reports them.

**Entry 36, first half — Save Output switched off, from Python.** Written out in full under question 5. With `save_output` set to False and no File Output node, an animation render started from Python raised `RuntimeError` with `Error: Render output disabled in Output properties and no active compositing File Output nodes` and wrote nothing.

**Entry 36, second half — Save Output switched off, from the command line.** A file saved with `save_output` False, rendered with `-f 1` and with `-a`: each printed `ERROR Render output disabled in Output properties and no active compositing File Output nodes`, wrote nothing, and ended with exit code 0. Written out in full under question 5.

**Entry 37 — does a Cycles render leave tile files behind?** At factory settings, no window: Cycles, one sample, denoising off, 4000 × 4000, one frame, PNG, tile size set to 512. The script listed Blender's temporary folder before the render, at the moment the frame was announced as written (from a `render_write` handler), and after:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q33.py -- …/work/e37
```

What came back, in full:

```
00:03.110  render           | Saved: '…/work/e37/cyc/img_0001.png'
tile_size: 2048
use_auto_tile: True
tile_size set to 512
temp dir: …/tmp/blender_wwC0rn/
tempdir before: []
   during render, tempdir: []
returned: {'FINISHED'}
tempdir after : []
output listing: ['img_0001.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
```

A listing taken at those three moments cannot see a file that is made and removed between them; entries 56 to 58 and 72 watch the temporary folder continuously instead.

**Entry 50 — a still from Python and a single frame from `-f`.** Written out in full under question 11. A still rendered from Python with no save switch left the empty output folder empty; the same with `write_still=True` wrote `still_.png`; `-f 1` from the command line wrote `cli_0001.png`.

**Entry 56 — what a large Cycles render writes outside its output folder.** Written out in full under question 11. At 8000 × 8000, on the processor and on the Mac's graphics, Cycles wrote one tile file into Blender's temporary folder during the render and removed it before Blender exited; at 1000 × 1000 it wrote none. The render on the graphics card also created four new files and changed the modified time of 83 others in Cycles' compiled-kernel folder in the user's home folder, `~/.cache/cycles/kernels/Apple_M1_Pro/`, which is outside both the temporary folder and the run's settings folder, and those remained afterwards.

**Entry 60 — the `.tx` files a render can write.** Written out in full under question 11. With the texture-cache switches at factory settings (`use_texture_cache` True, `use_auto_generate_texture_cache` False), a Cycles render using a PNG texture wrote nothing beside the texture. With `use_auto_generate_texture_cache` also on, it made a folder `blender_tx` beside the texture holding `smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx`; the preference `texture_cache_directory` was `''`.

**Entry 78 — in Blender's window: F12, Ctrl-F12, and a still from a script.** Written out in full under question 6. F12 left the empty output folder empty; Ctrl-F12 then wrote `img_0001.png` to `img_0003.png`. A still rendered from a script with no save switch left its empty folder empty, and the same with `write_still=True` wrote `still_.png`. During a large F12 (Cycles, 3840 × 2160, tile size 512, Cache Result off), Blender's temporary folder held one file, `cycles-tile-buffer-79564-32243420144-2-0.exr`, for the first eight one-second listings and was empty from the ninth to the end; the folder the large F12's output path pointed at stayed empty.

**Entry 79 — the 3D Viewport's still entry, with a real window.** Written out in full under question 6. Started as a click starts it (`render.opengl` with nothing set), the still viewport render wrote nothing into the empty output folder; the animation entry wrote three frames.

**Entry 80 — the Save Output switch in the window.** Written out in full under question 2. The Output panel's header carries a ticked checkbox; Blender names the setting behind it `Save Output` and describes it as `Write frames to disk for animation renders`.

---

## 8. How does the place a file records for writing become the name of a file on disk? Say where a frame's number appears in it, and what decides the end of the name.

**Answer.** For the scene's render output, the documented steps are:
1. A leading `//` becomes the blend-file's folder.
2. Template expressions are replaced when the path is used.
3. Each run of `#` becomes the frame number, padded with zeros to the count of `#`. It can sit anywhere in the file name.
4. If there is no `#`, four `#` are added to the end of the file name. So the frame number goes at the end of the name, padded to four digits.
5. If File Extensions is on (the default), the file format's extension is added. That extension ends the name.

Worked example from the manual: `//render_` becomes `//render_####`, and writes `//render_0001.png`.

The API's `frame_path` returns the full name to be written for a given frame. Cycles' own denoise tool uses it to find rendered frames on disk (observed).

For the File Output node, the pattern is `{Directory}/{File Name}{Socket Name}{frame number}.{extension}`. The frame number goes at the end unless `#` places it. With `#`, even a single render gets the frame number in its name.

**Label.** Documented for frame number padding, default suffix, and extension. Observed for Cycles using `frame_path`. Inferred that `frame_path` matches render output naming exactly, that File Extensions off leaves the name without an extension, and that a single render from the File Output node without `#` omits the frame number.

**Quotes.**

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."
> "Saving – File Extensions … Adds the correct file extensions per file type to the output files."

https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11

> "The # characters are replaced by the frame number, and used to define zero padding." / "animation_##_test.png becomes animation_01_test.png" / "test-######.png becomes test-000001.png" / "When the filename does not contain #, the suffix #### is added to the filename." / "//render_ becomes //render_####, writing frames as //render_0001.png"

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "use_file_extension … Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)"
> "frame_path(*, frame=-2147483648, preview=False, view='') … Return the absolute path to the filename to be written for a given frame"

https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."
> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."
> "File Extensions … Add the file format extensions to the saved file name (eg: filename + .jpg)."

https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11

Observed, from `cycles_5.3/operators.py`, class `CYCLES_OT_denoise_animation`, method `execute`, lines 70–91:

```
            # Denoise animation sequence with expanded frames matching
            # Blender render output file naming.
...
                filepath = scene.render.frame_path(frame=frame)
...
                if not os.path.isfile(filepath):
...
                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
```

What the file says: Cycles builds each frame's file name with `frame_path` and checks that the file exists. The comment says this matches "Blender render output file naming". Taking from this that `frame_path` gives exactly the name a render writes is one step beyond the file, so that part is inferred.

**Steps behind the inferences.**
- *File Extensions off:* the extension comes only from the File Extensions option. So with the option off, the name ends where the stored text, with the frame number put in, ends. No source says this outright.
- *Single renders from the File Output node:* the File Output page says `#` makes "a single render" carry the frame number. From that I take it that, without `#`, a single render from the node may not carry one. No source says so directly.

**Not covered.**
- Whether a still (`-f` or F12 with `write_still`) gets `####` added when the path has no `#`. The manual's "appended" sentence names animation renders; the command-line sentence is general.
- What happens when the stored name already ends in an extension and File Extensions is on. The `image_##_test.png` example suggests it is not doubled, but this is not stated.
- `#` in the folder part, several runs of `#`, and negative frame numbers.
- How a **video** file is named. No source describes it.
- Where a multi-view `file_suffix` goes in the name.

**Outside the interface.** The command-line help gives the same rules for `-o`, so they hold outside the interface (documented). `-x` "Set option to add the file extension to the end of the file" controls the extension from the command line (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html). *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* every rule was observed with no window (entries 3, 4, 6 to 10, 35, 70): a name with no `#` gets four digits at the end; a run of `#` sets the padding, and of two separate runs only the last is replaced; `#` in a folder name is left as written; a typed `.png` is not doubled in an animation, while a single frame from `-f` with `-o …/typed.png` was written `typed.png0001.png`; a single frame from `-f` with no `#` gets `0001`; a still from Python gets no number; frames −2 to −1 wrote one file, `img_0000.png`; a stereo frame's views end `_L` and `_R`; a video is named for its frame range.

**Operating system.** The sources do not say the naming differs by system. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entries 3 and 4 — the path builder.** Entry 3 put fifteen recorded strings through `scene.render.frame_path(frame=7)` with File Extensions on and off, and entry 4 ten more; both are written out in full under question 3. The lines that bear on where the frame number goes and how the name ends: `'/tmp/out_'` gave `'/tmp/out_0007.png'` with the switch on and `'/tmp/out_0007'` with it off; `'/tmp/out_##_x.png'` gave `'/tmp/out_07_x.png'` both ways; `'/tmp/out_######'` gave `'/tmp/out_000007.png'`; `'/tmp/out_.png'` gave `'/tmp/out_.png0007.png'`; `'/tmp/'` gave `'/tmp/0007.png'`; `''` gave `'0007.png'`; `'/tmp/dir_##/out_'` gave `'/tmp/dir_##/out_0007.png'`; and `'/tmp/a_##_b_###_'` gave `'/tmp/a_##_b_007_.png'`.

**Entry 6 — a template whose value holds `#`.** With the scene renamed `sc##ne` and the path `{scene_name}_`, the path builder gave `sc07ne_.png` for frame 7 and the render wrote `sc01ne_.png` for frame 1 (written out in full under question 3).

**Entry 7 — frame numbers past four digits, and below zero.** At factory settings, no window, Workbench at 64 × 64, PNG. Three animations: frames 99999 to 100001 with the path `img_`; frames 12344 to 12345 with `img_##`; frames −2 to −1 with `img_`. For each, the path builder was asked for the first and last frame, then the animation rendered. What came back, in full, and the folder afterwards:

```
00:00.524  render           | Saved: '…/work/e07/wide_none/img_99999.png'
00:00.530  render           | Saved: '…/work/e07/wide_none/img_100000.png'
00:00.537  render           | Saved: '…/work/e07/wide_none/img_100001.png'
00:00.545  render           | Saved: '…/work/e07/wide_two/img_12344.png'
00:00.553  render           | Saved: '…/work/e07/wide_two/img_12345.png'
00:00.561  render           | Saved: '…/work/e07/negative/img_0000.png'
--- wide_none '…/work/e07/wide_none/img_' frames 99999 to 100001
   frame_path(99999): '…/work/e07/wide_none/img_99999.png'
   frame_path(100001): '…/work/e07/wide_none/img_100001.png'
   returned: {'FINISHED'}
--- wide_two '…/work/e07/wide_two/img_##' frames 12344 to 12345
   frame_path(12344): '…/work/e07/wide_two/img_12344.png'
   frame_path(12345): '…/work/e07/wide_two/img_12345.png'
   returned: {'FINISHED'}
--- negative '…/work/e07/negative/img_' frames -2 to -1
   frame_path(-2): '…/work/e07/negative/img_-0002.png'
   frame_path(-1): '…/work/e07/negative/img_-0001.png'
   returned: {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
…/work/e07
…/work/e07/negative
…/work/e07/negative/img_0000.png
…/work/e07/wide_none
…/work/e07/wide_none/img_100000.png
…/work/e07/wide_none/img_100001.png
…/work/e07/wide_none/img_99999.png
…/work/e07/wide_two
…/work/e07/wide_two/img_12344.png
…/work/e07/wide_two/img_12345.png
```

For frames −2 to −1 the path builder gave `img_-0002.png` and `img_-0001.png`, and the render wrote one file, `img_0000.png`.

**Entry 8 — the names a render actually writes.** First, at factory settings, no window, Workbench at 64 × 64, PNG, frames 1 to 2, seven animations, each with its own output path, one of them with File Extensions off. Then two single frames from the command line: `-o …/still/still -F PNG -f 1`, and `-o …/still/typed.png -F PNG -x 1 -f 1`. The commands:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q10.py -- …/work/e08/anim
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -E BLENDER_WORKBENCH -o …/work/e08/still/still -F PNG -f 1
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -E BLENDER_WORKBENCH -o …/work/e08/still/typed.png -F PNG -x 1 -f 1
```

What the first printed, in full:

```
00:00.504  render           | Saved: '…/work/e08/anim/a_plain/img_0001.png'
00:00.509  render           | Saved: '…/work/e08/anim/a_plain/img_0002.png'
00:00.514  render           | Saved: '…/work/e08/anim/b_mid_hash/img_01_v2.png'
00:00.518  render           | Saved: '…/work/e08/anim/b_mid_hash/img_02_v2.png'
00:00.523  render           | Saved: '…/work/e08/anim/c_dir_hash/seq_##/img_0001.png'
00:00.529  render           | Saved: '…/work/e08/anim/c_dir_hash/seq_##/img_0002.png'
00:00.534  render           | Saved: '…/work/e08/anim/d_typed_ext/img_0001.png'
00:00.538  render           | Saved: '…/work/e08/anim/d_typed_ext/img_0002.png'
00:00.543  render           | Saved: '…/work/e08/anim/e_no_ext_sw/img_0001'
00:00.547  render           | Saved: '…/work/e08/anim/e_no_ext_sw/img_0002'
00:00.551  render           | Saved: '…/work/e08/anim/f_percent/img_%_0001.png'
00:00.556  render           | Saved: '…/work/e08/anim/f_percent/img_%_0002.png'
00:00.563  render           | Saved: '…/work/e08/anim/g_trailing/0001.png'
00:00.567  render           | Saved: '…/work/e08/anim/g_trailing/0002.png'
--- a_plain path: '…/work/e08/anim/a_plain/img_' use_file_extension: True
   returned: {'FINISHED'}
--- b_mid_hash path: '…/work/e08/anim/b_mid_hash/img_##_v2' use_file_extension: True
   returned: {'FINISHED'}
--- c_dir_hash path: '…/work/e08/anim/c_dir_hash/seq_##/img_' use_file_extension: True
   returned: {'FINISHED'}
--- d_typed_ext path: '…/work/e08/anim/d_typed_ext/img_####.png' use_file_extension: True
   returned: {'FINISHED'}
--- e_no_ext_sw path: '…/work/e08/anim/e_no_ext_sw/img_' use_file_extension: False
   returned: {'FINISHED'}
--- f_percent path: '…/work/e08/anim/f_percent/img_%_' use_file_extension: True
   returned: {'FINISHED'}
--- g_trailing path: '…/work/e08/anim/g_trailing/' use_file_extension: True
   returned: {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
```

The listing afterwards held exactly the fourteen files named in the `Saved:` lines. The two single frames printed, in full:

```
00:00.348  render           | Rendering single frame
00:00.348  render           | Rendering frame 1
00:00.349  render           | Start rendering: Scene, ViewLayer
00:00.349  render           | Engine: Workbench
00:00.646  render           | Saved: '…/work/e08/still/still0001.png'
00:00.646  render           | Time: 00:00.29 (Saving: 00:00.16)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
00:00.349  render           | Rendering single frame
00:00.349  render           | Rendering frame 1
00:00.349  render           | Start rendering: Scene, ViewLayer
00:00.349  render           | Engine: Workbench
00:00.652  render           | Saved: '…/work/e08/still/typed.png0001.png'
00:00.652  render           | Time: 00:00.30 (Saving: 00:00.16)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
```

and the folder held `still0001.png` and `typed.png0001.png`.

**Entry 9 — multi-view: where a view's ending lands.** At factory settings, no window, Workbench at 64 × 64, one frame, PNG, with multi-view on, the stereo set-up, and each view saved to a file of its own (`views_format = 'INDIVIDUAL'`). Three output paths — with `%`, without it, and with `%` and `###` — each asked of the path builder for no view, `left` and `right`, then rendered. What came back, in full, and the files afterwards:

```
00:00.539  render           | Saved: '…/work/e09/with_percent/img_%_0001_L.png'
00:00.540  render           | Saved: '…/work/e09/with_percent/img_%_0001_R.png'
00:00.554  render           | Saved: '…/work/e09/no_percent/img_0001_L.png'
00:00.554  render           | Saved: '…/work/e09/no_percent/img_0001_R.png'
00:00.566  render           | Saved: '…/work/e09/hash_percent/img_%_001_L.png'
00:00.567  render           | Saved: '…/work/e09/hash_percent/img_%_001_R.png'
views: [('left', '_L', '_L', True), ('right', '_R', '_R', True)]
--- with_percent '…/work/e09/with_percent/img_%_'
   frame_path view='' -> '…/work/e09/with_percent/img_%_0001.png'
   frame_path view='left' -> '…/work/e09/with_percent/img_%_0001_L.png'
   frame_path view='right' -> '…/work/e09/with_percent/img_%_0001_R.png'
   returned: {'FINISHED'}
--- no_percent '…/work/e09/no_percent/img_'
   frame_path view='' -> '…/work/e09/no_percent/img_0001.png'
   frame_path view='left' -> '…/work/e09/no_percent/img_0001_L.png'
   frame_path view='right' -> '…/work/e09/no_percent/img_0001_R.png'
   returned: {'FINISHED'}
--- hash_percent '…/work/e09/hash_percent/img_%_###'
   frame_path view='' -> '…/work/e09/hash_percent/img_%_001.png'
   frame_path view='left' -> '…/work/e09/hash_percent/img_%_001_L.png'
   frame_path view='right' -> '…/work/e09/hash_percent/img_%_001_R.png'
   returned: {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
…/work/e09/hash_percent/img_%_001_L.png
…/work/e09/hash_percent/img_%_001_R.png
…/work/e09/no_percent/img_0001_L.png
…/work/e09/no_percent/img_0001_R.png
…/work/e09/with_percent/img_%_0001_L.png
…/work/e09/with_percent/img_%_0001_R.png
```

The views line reads: name, file suffix, camera suffix, in use.

**Entry 10 — what a video file is called.** At factory settings, no window, Workbench at 64 × 64, frames 1 to 5. The media type was set to Video, the format to FFmpeg and the container to Matroska; two output paths, `…/plain/mov_` and `…/hashes/mov_##.mkv`, each asked of the path builder for frame 1 and then rendered. What came back, in full, and the files afterwards:

```
00:00.514  render           | Video append frame 1
00:00.520  render           | Video append frame 2
00:00.527  render           | Video append frame 3
00:00.531  render           | Video append frame 4
00:00.536  render           | Video append frame 5
00:00.543  render           | Video append frame 1
00:00.547  render           | Video append frame 2
00:00.551  render           | Video append frame 3
00:00.555  render           | Video append frame 4
00:00.560  render           | Video append frame 5
--- plain '…/work/e10/plain/mov_'
   frame_path: '…/work/e10/plain/mov_0001-0005.mkv'
   returned: {'FINISHED'}
--- hashes '…/work/e10/hashes/mov_##.mkv'
   frame_path: '…/work/e10/hashes/mov_##.mkv'
   returned: {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
…/work/e10/hashes/mov_##.mkv
…/work/e10/plain/mov_0001-0005.mkv
```

No `Saved:` line was printed for either video.

**Entry 35 — the name a File Output node gives a single render.** Written out in full under question 6. With the node's name field `nd_` and its one input `Image`: a single frame from `-f 1`, and an animation of one frame from `-a`, each wrote `nd_Image0001.png`; a still rendered from Python with `animation=False` wrote `nd_Image.png`, both with `write_still=False` and with `write_still=True`, and the scene's own still with `write_still=True` was written as `s_.png`; the Python animation wrote `nd_Image0001.png` and `s_0001.png`.

**Entry 38 — the name built from `//frame_####` when no file has been saved.** Written out in full under question 3. With no saved file, `bpy.path.abspath('//')` gave `''` and the path builder gave `'frame_0001.png'` for frame 1, with no folder in front. Both a single frame and an animation then stopped at frame 1 with `ERROR Couldn't create directory for file frame_0001.png: No such file or directory` and `ERROR Render error (No such file or directory) cannot save: 'frame_0001.png'`, wrote nothing, and ended with exit code 1.

**Entry 70 — the names of a stereo frame's two files.** Written out in full under question 14. With multi-view on and each view saved to its own file, the output path `…/frame_` wrote `frame_0001_L.exr` and `frame_0001_R.exr` for frame 1, and so on.

**Entry 81 — a finished path in the window.** Written out in full under question 2. The File Output node's own sidebar panels, where a finished file path would be listed, could not be brought up by script, so no finished path was seen in the window.

---

## 9. Can the place a file output node writes to be settled for a render — at the moment the render is started, and by writing it into the file from outside Blender?

The question is ambiguous on its second half. "From outside Blender" could mean (a) a program other than Blender editing the blend-file, or (b) Blender started and driven by something else, with a script that changes the stored value. I answer both.

**Answer.**

*At the moment the render is started:*
- No command-line option is described for the File Output node. `-o` is described only as setting "the render path and file name". Whether it reaches the node is not stated.
- The node's `directory` and `file_name` accept template expressions, replaced at time of use. So the actual place can depend on values at render time without editing the stored text. The values include blend-file name and folder, scene, camera, resolution, fps, node name, and in 5.3 project variables.
- In 5.3 the node's default file name is `{blend_name}`.
- A script given at launch with `--python` or `--python-expr` runs in order with the other options. `directory` and `file_name` are writable text properties. So a script placed before the render option could set them for that run (inferred).

*Writing it into the file from outside Blender:*
- (a) A program other than Blender editing the blend-file: not in the sources. The only related fact is that compressed blend-files use Zstandard and "can be compressed/decompressed with external tools". Nothing describes editing the contents.
- (b) Blender driven from outside: a background script can set the properties (as above). Saving the file afterwards would store them. That is inferred; no source gives it as a recipe.

**Label.** Documented for templates and the 5.3 default. Not in the sources for a command-line option for the node, and for editing by another program. Inferred for the script route.

**Quotes.**

> "directory … The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)"
> "file_name … The base name of the file. Other information might be included in the final file name depending on the node options (default “”, never None, Supports template expressions)"

https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html — read 2026-10-11

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node."
> "Available only in path properties on a node: node_name: The name of the node that the path property is on."

https://docs.blender.org/manual/en/5.3/files/file_paths.html — read 2026-10-11

> "The default file name of the **File Output** node is now `{blend_name}`, so rendered images will be named after the current blend file's name, instead of a hardcoded string. This is just the initial default value, the name can still be changed as usual."

https://developer.blender.org/docs/release_notes/5.3/compositor/ — read 2026-10-11

> "-o, --render-output <path> … Set the render path and file name." / "--python-expr <expression> … Run the given expression as a Python script." / "Arguments are executed in the order they are given."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "The used compression algorithm is Zstandard. It is not unique to Blender so files can be compressed/decompressed with external tools."

https://docs.blender.org/manual/en/5.3/files/blend/open_save.html — read 2026-10-11

**Step behind the inference.** The two properties are listed without "readonly", unlike `file_output_items` and `format`, which are marked "readonly". Options run in the order given. So a script placed before `-f` or `-a` could change them before the render. No source shows this being done.

**Where I looked for a node option at launch.** The full command-line arguments page and the command-line rendering page. I searched the whole manual, API docs and release notes for "render-output", "-o /", "-o //" and "File Output". Every `-o` mention speaks of "the render path". No page ties a launch option to the node.

**Not covered.** Whether `-o` affects the node. How another program could change the node's stored path inside a blend-file. How a script finds the node in 5.3: the compositor now uses "an effects stack" per the 5.3 compositor release notes. I did not pursue this, because it was not asked.

**Outside the interface.** Templates and the script route are both outside-the-interface routes (the script route is inferred). Inside the interface, the node's fields are edited by hand. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, `-o` did not reach a File Output node (entry 15); a script given at launch set the node's folder and name for that render, and a windowless Blender set them and saved the file (entry 16); `bpy.data.file_path_foreach` visited no node path (entry 16); a node made by script starts with the name `{blend_name}` and the folder `/tmp/` (entry 2). Editing a blend-file with a program other than Blender was not tried.

**Operating system.** The sources do not say this differs by system. The node accepts both `/` and `\` in input names as folder marks (question 3). *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 2 — what a File Output node records, and what a fresh one starts with.** At factory settings, no window, a compositor node group was made and put on the scene, and one File Output node added with `nodes.new("CompositorNodeOutputFile")`. Its properties include `directory` and `file_name`, both `STRING writable`, and `file_output_items` and `format`, both `readonly` (the full list is under question 2). A fresh node, made the same way in a second launch, printed:

```
new node directory: '/tmp/'
new node file_name: '{blend_name}'
new node use_file_extension: True
new node save_as_render: True
new node format media_type: MULTI_LAYER_IMAGE file_format: OPEN_EXR_MULTILAYER
new node items: []
new node inputs: ['']
```

**Entry 15 — does `-o` reach a File Output node?** At factory settings, no window, a file `node.blend` was saved holding a scene whose own output path is `…/scene_out/sc_` (Workbench, 64 × 64, one frame, PNG) and a File Output node named `FileOut`, wired from Render Layers, with one input named `Image`, folder `…/node_out/` and name `nd_`. It was rendered with `-o` pointing at a third folder, the folder listed, and the file reopened to print the node's stored values. This was done twice. The first time, the node's own format was left as it starts; the second time, the node's format was set to a still image first (`fo.format.media_type = 'IMAGE'` then `fo.format.file_format = 'PNG'`) and nothing else changed. The commands, the same both times with the folder `e15` or `e15png`:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q16_make.py -- …/work/e15
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e15/node.blend -o …/work/e15/cli_out/cli_ -F PNG -x 1 -f 1
echo "exit code: $?"
find …/work/e15 -type f -not -name '*.blend' | sort
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e15/node.blend --python q16_read.py
```

What the first came back with (the deprecation warnings for `Scene.compositing_node_group`, one per use, are left out):

```
Info: Saved as "node.blend"
node name: FileOut node folder: …/work/e15/node_out/
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
00:00.342  blend            | Read blend: "…/work/e15/node.blend"
00:00.347  render           | Rendering single frame
00:00.347  render           | Rendering frame 1
00:00.350  render           | Start rendering: Scene, ViewLayer
00:00.350  render           | Engine: Workbench
00:00.470  render           | Executing compositor
00:00.470  render           | Fra: 1 | Compositing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.474  render           | Saved: '…/work/e15/node_out/nd_0001.exr'
00:00.536  render           | Saved: '…/work/e15/cli_out/cli_0001.png'
00:00.536  render           | Time: 00:00.18 (Saving: 00:00.06)

Blender quit
exit code: 0
…/work/e15/cli_out/cli_0001.png
…/work/e15/node_out/nd_0001.exr
00:00.352  blend            | Read blend: "…/work/e15/node.blend"
node directory: '…/work/e15/node_out/' file_name: 'nd_'
scene filepath: '…/work/e15/scene_out/sc_'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

What the second came back with:

```
Info: Saved as "node.blend"
node name: FileOut node folder: …/work/e15png/node_out/
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
00:00.349  blend            | Read blend: "…/work/e15png/node.blend"
00:00.353  render           | Rendering single frame
00:00.353  render           | Rendering frame 1
00:00.356  render           | Start rendering: Scene, ViewLayer
00:00.356  render           | Engine: Workbench
00:00.467  render           | Executing compositor
00:00.467  render           | Fra: 1 | Compositing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.531  render           | Saved: '…/work/e15png/node_out/nd_Image0001.png'
00:00.532  render           | Saved: '…/work/e15png/cli_out/cli_0001.png'
00:00.532  render           | Time: 00:00.17 (Saving: 00:00.00)

Blender quit
exit code: 0
…/work/e15png/cli_out/cli_0001.png
…/work/e15png/node_out/nd_Image0001.png
00:00.343  blend            | Read blend: "…/work/e15png/node.blend"
node directory: '…/work/e15png/node_out/' file_name: 'nd_'
scene filepath: '…/work/e15png/scene_out/sc_'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Entry 16 — setting the node's place from outside, three ways.** Using the second `node.blend` above. Part one: Blender started with no window on that file, a one-line script given with `--python-expr` set the node's folder to `…/work/e16/from_cli/` and its name to `zz_`, and then `-f 1` rendered one frame. Part two: on a copy, `edit.blend`, the same kind of line set the folder to `…/work/e16/written_in/` and the name to `ww_` and saved the file, with no render; the file was then reopened and its node's values printed. Part three: the file was opened and `bpy.data.file_path_foreach` called with a function that records every path it is handed. The commands:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e15png/node.blend --python-expr "import bpy; sc=bpy.context.scene; t=sc.compositing_node_group; n=t.nodes['FileOut']; n.directory='…/work/e16/from_cli/'; n.file_name='zz_'; print('set to', n.directory, n.file_name)" -f 1
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e16/edit.blend --python-expr "import bpy; sc=bpy.context.scene; n=sc.compositing_node_group.nodes['FileOut']; n.directory='…/work/e16/written_in/'; n.file_name='ww_'; bpy.ops.wm.save_mainfile(); print('saved')"
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e16/edit.blend --python q16_read.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e15png/node.blend --python q19.py
find …/work/e16 -type f -not -name '*.blend' | sort
```

What came back (the deprecation warnings and the script lines Python echoes with them are left out):

```
00:00.347  blend            | Read blend: "…/work/e15png/node.blend"
00:00.352  render           | Rendering single frame
00:00.352  render           | Rendering frame 1
00:00.352  render           | Start rendering: Scene, ViewLayer
00:00.352  render           | Engine: Workbench
00:00.470  render           | Executing compositor
00:00.470  render           | Fra: 1 | Compositing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.533  render           | Saved: '…/work/e16/from_cli/zz_Image0001.png'
00:00.534  render           | Saved: '…/work/e15png/scene_out/sc_0001.png'
00:00.534  render           | Time: 00:00.18 (Saving: 00:00.00)
set to …/work/e16/from_cli/ zz_

Blender quit
exit code: 0
00:00.352  blend            | Read blend: "…/work/e16/edit.blend"
Info: Saved "edit.blend"
saved
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
00:00.351  blend            | Read blend: "…/work/e16/edit.blend"
node directory: '…/work/e16/written_in/' file_name: 'ww_'
scene filepath: '…/work/e15png/scene_out/sc_'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
00:00.343  blend            | Read blend: "…/work/e15png/node.blend"
has file_path_foreach: True
count: 0
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
…/work/e16/edit.blend1
…/work/e16/from_cli/zz_Image0001.png
```

The `.blend1` file is the backup Blender keeps of the version it saved over.

**Entry 41 — a File Output node's own missing folder.** Written out in full under question 4. A node aimed at `…/node_a/node_b/`, neither level existing, wrote `…/node_a/node_b/nd_0001.exr` and both folders were there afterwards.

**Entry 81 — a File Output node's fields in the window.** Written out in full under question 2. On the node's face its folder and its name are two plain text fields, each with a folder button, showing exactly what was stored (`//out/`, `shot`); each input is listed by its name. The node's own sidebar panels could not be brought up by script, so whether the window shows a finished file path was not seen.

---

## 10. Can the place a render records for writing carry characters that the computer it lands on will not accept, and what does a render do when it does?

**Answer.**

*Can it carry them?* Very likely yes, but this is inferred. The sources put no character limit on the render path or the File Output node fields, beyond these:
- template braces must be closed or doubled
- there is a general length limit
- names cannot contain the Null character

Paths may even hold text that is not valid UTF-8. Template expressions put names into the path: the blend-file's name, the scene name, the camera name, the node name. Data-block names are limited only to UTF-8 without the Null character, and to 255 bytes. The sources do not say whether Blender cleans those names when it puts them into a path.

A Python helper, `bpy.path.clean_name`, exists that replaces risky characters. No source says a render uses it.

*What does a render do then?* Not in the sources.

**Label.** Inferred for "can carry". Not in the sources for what a render does.

**Quotes.**

> "Data-block names have a fixed length of 255 bytes and are UTF-8 encoded." … "Data-block names cannot contain a Null character (U+0000)."

https://docs.blender.org/manual/en/5.3/files/data_blocks.html — read 2026-10-11

> "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user’s file system cannot be ignored."

https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html — read 2026-10-11

> "scene_name: Name of the current scene." / "camera_name: Name of the current render camera." / "blend_name: The currently open blend file’s name (without the .blend)."

https://docs.blender.org/manual/en/5.3/files/file_paths.html — read 2026-10-11

> "bpy.path.clean_name(name, *, replace='_') … Returns a name with characters replaced that may cause problems under various circumstances, such as writing to a file. All characters besides A-Z/a-z, 0-9 are replaced with “_” or the replace argument if defined."

https://docs.blender.org/api/5.3/bpy.path.html — read 2026-10-11

**Step behind the inference.** Names may hold any UTF-8 except Null. Templates put names into the path. No source says the names are cleaned. So a name holding, for example, a colon could reach the path. The sources never state this.

**Where I looked for what a render does.**
- Searched the whole manual, API docs, 5.3 release notes and Cycles files for: "invalid char", "illegal char", "forbidden char", "reserved char", "special char", "unsupported char", "not allowed in file/path", "characters are/is (not) allowed/permitted/replaced", "replaced with/by underscore", "clean_name", "ensure_filename", "sanitiz", "valid file name", "valid path", "backslash", "path separator", "colon", "unicode", "utf-8", "ascii".
- Hits were: the Special Characters node (text output, unrelated); asset catalog paths ("Invalid characters: :, \.", about catalog names, not render output); driver variable names; `bpy.path.clean_name`; the encoding page above.
- None describes a render meeting a name the system rejects.

**Not covered.** Which characters each system rejects (the sources do not list them). What a render does on meeting one. Whether template values are cleaned.

**Outside the interface.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, every awkward character typed into the path was stored and written on this Mac; a path set longer than 1023 characters was cut to 1023 (entry 12), as it was when pasted into the field in Blender's window (entry 83); a name too long for the computer made the render print `Render error (Undefined error: 0) cannot save` and end with exit code 1 (entry 12); values arriving through a template had `:`, `*` and `?` replaced by `_` (entry 13).

**Operating system.** The sources do not say which characters fail on which system. This is the point the question turns on, and it is not covered. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. macOS accepted every character tried, so this Mac cannot show what a render does when the computer refuses one; Windows, which refuses several, was not run.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 12 — names and paths longer than the published limits.** At factory settings, no window, Workbench at 64 × 64, one frame, PNG. The output path was set to a folder followed by a name of 300 letters `n`, then of 1200; for each, the stored length was read back, the path builder asked for frame 1, and an animation rendered from Python inside a `try`. The command:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python q25.py -- …/work/e12
```

What came back, in full. A run of the letter n is written `⟨945 × n⟩` and so on, with its count; in the second line, where the closing quote would be, Blender printed a zero byte, written `⟨zero byte⟩`; nothing else is changed. The line beginning `Error: Render error` for the 1200-letter name is printed as it came, ending in `... truncated` with no line break before the next line:

```
00:00.499  image.write      | ERROR OpenImageIO write failed: Could not open file "…/work/e12/long/⟨300 × n⟩0001.png"
00:00.506  image.write      | ERROR OpenImageIO write failed: Could not open file "…/work/e12/long/⟨945 × n⟩0001.png⟨zero byte⟩
--- 300 chars asked length: 368
    stored length: 368
    stored tail: '⟨40 × n⟩'
    frame_path length: 376
Error: Render error (Undefined error: 0) cannot save: '…/work/e12/long/⟨300 × n⟩0001.png'
    raised: RuntimeError Error: Render error (Undefined error: 0) cannot save: '…/work/e12/long/⟨300 × n⟩0001.png'

--- 1200 chars asked length: 1268
    stored length: 1023
    stored tail: '⟨40 × n⟩'
    frame_path length: 1021
Error: Render error (Undefined error: 0) cannot save: '…/work/e12/long/⟨877 × n⟩... truncated    raised: RuntimeError Error: Render error (Undefined error: 0) cannot save: '…/work/e12/long/⟨945 × n⟩0001.png'

listing: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 1
```

So 1268 characters asked for were stored as 1023; 368 were stored whole. Nothing was written for either.

**Entry 13 — characters this computer will and will not take.** First the computer itself, with ordinary empty files: every one of `co:lon`, `aster*isk`, `quest?ion`, `pi|pe`, `back\slash`, `кириллица`, `em😀ji` and `tra ilingspace ` was accepted by `touch` with exit code 0, and `ls -b` listed:

```
aster*isk
back\\slash
co:lon
em\360\237\230\200ji
pi|pe
quest?ion
tra ilingspace 
\320\272\320\270\321\200\320\270\320\273\320\273\320\270\321\206\320\260
```

Then, at factory settings, no window, Workbench at 64 × 64, one frame, PNG, each of nine names was typed as the end of the output path, the stored value and the path builder's answer printed, and one frame rendered. What came back, in full, and `ls -b` of the folder afterwards:

```
00:00.525  render           | Saved: '…/work/e13/v/chars/co:lon_0001.png'
00:00.531  render           | Saved: '…/work/e13/v/chars/aster*isk_0001.png'
00:00.538  render           | Saved: '…/work/e13/v/chars/quest?ion_0001.png'
00:00.545  render           | Saved: '…/work/e13/v/chars/pi|pe_0001.png'
00:00.552  render           | Saved: '…/work/e13/v/chars/back/slash_0001.png'
00:00.560  render           | Saved: '…/work/e13/v/chars/кириллица_0001.png'
00:00.566  render           | Saved: '…/work/e13/v/chars/em😀ji_0001.png'
00:00.571  render           | Saved: '…/work/e13/v/chars/trailing_space_ 0001.png'
00:00.577  render           | Saved: '…/work/e13/v/chars/dot._0001.png'
--- asked for: '…/work/e13/v/chars/co:lon_'
    stored as: '…/work/e13/v/chars/co:lon_'
    frame_path: '…/work/e13/v/chars/co:lon_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/aster*isk_'
    stored as: '…/work/e13/v/chars/aster*isk_'
    frame_path: '…/work/e13/v/chars/aster*isk_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/quest?ion_'
    stored as: '…/work/e13/v/chars/quest?ion_'
    frame_path: '…/work/e13/v/chars/quest?ion_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/pi|pe_'
    stored as: '…/work/e13/v/chars/pi|pe_'
    frame_path: '…/work/e13/v/chars/pi|pe_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/back\\slash_'
    stored as: '…/work/e13/v/chars/back\\slash_'
    frame_path: '…/work/e13/v/chars/back/slash_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/кириллица_'
    stored as: '…/work/e13/v/chars/кириллица_'
    frame_path: '…/work/e13/v/chars/кириллица_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/em😀ji_'
    stored as: '…/work/e13/v/chars/em😀ji_'
    frame_path: '…/work/e13/v/chars/em😀ji_0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/trailing_space_ '
    stored as: '…/work/e13/v/chars/trailing_space_ '
    frame_path: '…/work/e13/v/chars/trailing_space_ 0001.png'
    returned: {'FINISHED'}
--- asked for: '…/work/e13/v/chars/dot._'
    stored as: '…/work/e13/v/chars/dot._'
    frame_path: '…/work/e13/v/chars/dot._0001.png'
    returned: {'FINISHED'}
listing: ['aster*isk_0001.png', 'back', 'co:lon_0001.png', 'dot._0001.png', 'em😀ji_0001.png', 'pi|pe_0001.png', 'quest?ion_0001.png', 'trailing_space_ 0001.png', 'кириллица_0001.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
exit code: 0
aster*isk_0001.png
back
co:lon_0001.png
dot._0001.png
em\360\237\230\200ji_0001.png
pi|pe_0001.png
quest?ion_0001.png
trailing_space_ 0001.png
\320\272\320\270\321\200\320\270\320\273\320\273\320\270\321\206\320\260_0001.png
```

Then the scene was renamed `a:b*c?` and the path set to `…/tmpl/{scene_name}_`: the path builder gave `…/tmpl/a_b_c__0001.png` and the render wrote `a_b_c__0001.png` (written out in full under question 3).

**Entry 83 — 1400 characters pasted into the File Path field, in the window.** Written out in full under question 2. Pasted over the field, and pasted inside the field opened for editing and confirmed with Return, the field kept 1023 characters — the first 1023 of the 1400 — with no message shown.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**Answer.**

*In the output folder during an animation render:*
- Frames are "typically saved directly to disk as they are rendered". So finished frames are present while later ones are still rendering. The manual's crash advice relies on this.
- With **Placeholders** on (default off), empty placeholder files are created "while rendering frames".
- With **Overwrite** off, existing files are kept, and rendering can resume "where you left off".

*A video file during the run:* that the file exists during the run is inferred. The manual warns it "might become unplayable" if a problem occurs while rendering (more under question 15).

*Size of the picture:*
- Only Cycles is documented. It renders "high resolution images in tiles" of the Tile Size (default 2048, range 8 to 8192) and caches "tiles … to disk while rendering". The sources do not say where this disk cache is, so they do not say whether it appears in the output folder.
- Cycles' own interface code compares the picture's area with one tile's area. That suggests tiling starts only when the picture is larger than one tile (inferred).
- Nothing is documented about picture size for EEVEE or Workbench, or for placeholder files.

**Label.** Documented for frames being saved as rendered, placeholders, overwrite, and tiles cached to disk. Observed for Cycles' tile-size code. Inferred for when tiling starts and the video file existing. Not in the sources for where the tile cache sits.

**Quotes.**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."
> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory." / "Just disable the Overwrite option to start rendering where you left off."

https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11

> "Overwrite existing files when rendering." … "Create empty placeholder frames while rendering." … "Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory."

https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11. On that page both settings sit under headings that read "Image Sequence – Overwrite". The second describes placeholders. The picture `render_output_properties_output_panel.png` shows separate "Overwrite" and "Placeholders" checkboxes.

> "use_placeholder … Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)"

https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11

> "Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory." … "the default 2048 pixels provides a good balance for most scenes."

https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html — read 2026-10-11

Observed, `cycles_5.3/properties.py`, class holding Cycles scene settings, lines 1128–1138:

```
    use_auto_tile: BoolProperty(
        name="Auto Tile",
        description="Deprecated, tiling is always enabled",
        default=True,
    )
    tile_size: IntProperty(
        name="Tile Size",
        default=2048,
        description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
        min=8,
        max=8192,
    )
```

Observed, `cycles_5.3/ui.py`, path guiding panel `draw`, lines 405–420:

```
            # Calculation matches TileManager::compute_render_tile_size and
            # Session::get_effective_tile_size
            if cscene.tile_size < 128:
                tile_size = cscene.tile_size
            else:
                tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)
                tile_size = min(tile_size, 8192)
            tile_area = tile_size ** 2
            ...
            if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:
                layout.label(text="May work poorly with render tiling", icon='STATUS_INFO')
```

What the file says: the tile size Cycles uses is the setting rounded up to a multiple of 128, capped at 8192, when the setting is 128 or more. The panel shows a warning about "render tiling" when the picture's pixel area is larger than one tile's area. The stated reason this panel matches Cycles is the comment's "Calculation matches…".

**Steps behind the inferences.**
- *Tiling threshold:* the warning about render tiling appears only when the picture area is larger than one tile. From that I take it that a picture no larger than one tile is rendered as one tile, and so has nothing to cache tile by tile. The code shows only when a warning appears, not what the renderer does, so this is inferred.
- *Where the tile cache sits:* the temporary folder is used for run-time files "including render layers" (https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html). Linking the tile cache to that folder is a guess the sources do not support. I state no location.

**Not covered.**
- Whether a partly written image file is ever visible in the output folder before it is complete.
- Whether a placeholder is replaced in place or removed and re-made.
- Where Cycles' tile cache is written and how big it gets.
- What File Output node files look like during a render.
- The extra branch in the Cycles code for widths or heights over 8192.

**Outside the interface.** The sources do not say this differs. Placeholders and Overwrite are scene settings, so they apply to command-line renders too (inferred). *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* all observed with no window (entries 50 to 63, 74, 75), and for stills and F12 again with a window (entry 78): finished frames accumulate; nothing stands for the frame being rendered unless Placeholders is on, when a 0-byte file appears as that frame starts and is written in place; a frame's file is visible under its final name, growing, while it is written, and Blender holds it open for writing; Cycles' tile file sits in Blender's temporary folder for pictures larger than one tile; Cache Result rewrites one file per frame in the render cache folder; a single video file is present from the first frame and grows.

**Operating system.** The sources do not say. The placeholder text compares the behaviour to "Unix ‘touch’" only as a description. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. On this Mac Cycles' tile file appeared under `TMPDIR`.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 50 — what a still render puts at the output path, with no window.** At factory settings, no window: Cycles, 8 samples, 320 × 240, PNG, the output path `…/out/still_` in an empty folder. The script listed the folder, rendered a still from Python with no save switch, listed it, rendered again with `write_still=True`, and listed it. Then a single frame from the command line alone into a second empty folder:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e01_still.py -- …/work/e50/out
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -o …/work/e50/out2/cli_ -F PNG -f 1
ls …/work/e50/out2
```

What came back, in full apart from the command-line render's progress lines (`Fra: 1 | Rendering N / 64 samples`, one every few samples):

```
00:00.789  render           | Saved: '…/work/e50/out/still_.png'
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
00:00.364  render           | Rendering single frame
00:00.364  render           | Rendering frame 1
00:00.364  render           | Start rendering: Scene, ViewLayer
00:00.364  render           | Engine: EEVEE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:01.913  render           | Saved: '…/work/e50/out2/cli_0001.png'
00:01.913  render           | Time: 00:01.54 (Saving: 00:00.17)
exit code: 0
cli_0001.png
```

The command-line run set no engine, so it used the factory engine, EEVEE.

**How the folder was watched in entries 51 to 76.** A small program, not Blender, read the output folder and everything under it in a tight loop, about every two milliseconds, and printed a line each time a file appeared (`NEW`), changed size (`GROW`) or went (`GONE`), with the clock to the microsecond. It was started before Blender and stopped after Blender exited. Its log was then read by a second small program that prints, for each file, when it first appeared and at what size, how many growth readings it had, when it last changed, its final size, and whether any file changed size after a later file had first appeared. Both are printed in full in `Audits/Blender Research/Writing/Hand tests — 5.3.md`. The clock in these logs is the computer's own, in seconds.

**Entry 51 — what is in the output folder while an animation renders, Placeholders off.** At factory settings, no window: Cycles on the processor, 400 samples, 1920 × 1080, frames 1 to 4, PNG, Overwrite on, Placeholders off, into an empty folder, with the watcher on that folder:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e02_anim.py -- …/work/e51/out 0 1
```

Blender's own lines (its per-sample progress lines left out):

```
00:15.486  render           | Saved: '…/work/e51/out/f_0001.png'
00:30.840  render           | Saved: '…/work/e51/out/f_0002.png'
00:45.592  render           | Saved: '…/work/e51/out/f_0003.png'
00:58.620  render           | Saved: '…/work/e51/out/f_0004.png'
use_placeholder = False  use_overwrite = True
FINAL: [('f_0001.png', 1599935), ('f_0002.png', 1599935), ('f_0003.png', 1599935), ('f_0004.png', 1599935)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
```

The watcher's summary:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
f_0001.png                                           1792229596.476956          0     37 1792229596.612418      1599935 -
f_0002.png                                           1792229611.856035          0     36 1792229611.966699      1599935 -
f_0003.png                                           1792229626.621469          0     36 1792229626.719444      1599935 -
f_0004.png                                           1792229639.652084          0     36 1792229639.748332      1599935 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

and its raw lines at the start of the first frame's file and at the end of the last:

```
WATCH start 1792229580.283856 interval=0.002 dirs=…/work/e51/out
1792229596.476956 NEW   …/work/e51/out/f_0001.png 0
1792229596.516430 GROW  …/work/e51/out/f_0001.png 40960
1792229596.518834 GROW  …/work/e51/out/f_0001.png 65536
…
1792229596.609612 GROW  …/work/e51/out/f_0001.png 1548288
1792229596.612418 GROW  …/work/e51/out/f_0001.png 1599935
…
1792229639.745777 GROW  …/work/e51/out/f_0004.png 1572864
1792229639.748332 GROW  …/work/e51/out/f_0004.png 1599935
```

So for about fifteen seconds while each frame was rendered the folder held nothing for it; then its file appeared at 0 bytes and grew through 36 or 37 readings over about a tenth of a second to its final size, under its final name.

**Entry 52 — the same animation with Placeholders on and Overwrite off.** As entry 51, with `use_placeholder` True and `use_overwrite` False, into an empty folder:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e02_anim.py -- …/work/e52/out 1 0
```

Blender's own lines (per-sample progress left out):

```
00:17.556  render           | Saved: '…/work/e52/out/f_0001.png'
00:31.343  render           | Saved: '…/work/e52/out/f_0002.png'
00:43.623  render           | Saved: '…/work/e52/out/f_0003.png'
00:56.123  render           | Saved: '…/work/e52/out/f_0004.png'
use_placeholder = True  use_overwrite = False
FINAL: [('f_0001.png', 1599935), ('f_0002.png', 1599935), ('f_0003.png', 1599935), ('f_0004.png', 1599935)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
```

The watcher's summary:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
f_0001.png                                           1792229650.964294          0     35 1792229667.983079      1599935 -
f_0002.png                                           1792229667.985648          0     37 1792229681.770519      1599935 -
f_0003.png                                           1792229681.770519          0     36 1792229694.052082      1599935 -
f_0004.png                                           1792229694.052082          0     35 1792229706.551546      1599935 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

and its raw lines at the start and where frame 1 finishes:

```
WATCH start 1792229649.525458 interval=0.002 dirs=…/work/e52/out
1792229650.964294 NEW   …/work/e52/out/f_0001.png 0
1792229667.894978 GROW  …/work/e52/out/f_0001.png 40960
…
1792229667.983079 GROW  …/work/e52/out/f_0001.png 1599935
1792229667.985648 NEW   …/work/e52/out/f_0002.png 0
1792229681.677000 GROW  …/work/e52/out/f_0002.png 16384
```

So `f_0001.png` appeared at 0 bytes 1.44 seconds after the watcher started and stayed at 0 bytes for the seventeen seconds frame 1 was rendered, then grew in place; `f_0002.png` appeared at 0 bytes in the reading after `f_0001.png` reached its final size, and so on — one empty file at a time, never all four at the start.

**Entry 53 — what is left when that render is stopped part way.** As entry 52 — Placeholders on, Overwrite off, frames 1 to 4 — into an empty folder, with Blender started in the background and sent the ordinary terminate signal after a fixed 30 seconds, which is inside the third frame's render (each frame took about twelve seconds):

```
TMPDIR=… BLENDER_USER_RESOURCES=… "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e02_anim.py -- …/work/e53b/out 1 0 > …/blender.log 2>&1 &
B=$!; sleep 30; kill $B; wait $B; echo "blender exit code: $?"
ls -l …/work/e53b/out
```

What came back — the shell's own line about the stop, the exit code, the watcher's summary, the listing and Blender's own lines (per-sample progress left out):

```
…/scripts/run_e53.sh: line 7: 42026 Terminated: 15          TMPDIR=$W/tmp BLENDER_USER_RESOURCES=$W/res-5.3 "$B53" -b --factory-startup --python $W/scripts/cross/e02_anim.py -- $E/out 1 0 > $E/blender.log 2>&1
blender exit code: 143
file                                                 first_seen            at_size  grows last_change         final_bytes gone
f_0001.png                                           1792229786.329430          0     36 1792229798.709132      1599935 -
f_0002.png                                           1792229798.709132          0     36 1792229810.724646      1599935 -
f_0003.png                                           1792229810.727264          0      0 1792229810.727264            0 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
total 8448
-rw-r--r--@ 1 artist     wheel  1599935 Oct 17 12:36 f_0001.png
-rw-r--r--@ 1 artist     wheel  1599935 Oct 17 12:36 f_0002.png
-rw-r--r--@ 1 artist     wheel        0 Oct 17 12:36 f_0003.png
00:12.805  render           | Saved: '…/work/e53b/out/f_0001.png'
00:24.821  render           | Saved: '…/work/e53b/out/f_0002.png'
```

So the 0-byte file for the frame being rendered, `f_0003.png`, was left behind, with the two finished frames beside it, and frame 4 had no file.

**Entry 54 — whether one frame's file is visible while it is still being written.** At factory settings, no window: Workbench, 10000 × 10000, one frame, rendered as an animation, once each as PNG, as OpenEXR and as multi-layer OpenEXR (the media type set to Multi-Layer Image first), each into an empty folder watched as above:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e05_big.py -- …/work/e54/<format>/out <format>
```

What came back for each — the watcher's summary, Blender's own lines, the count of growth readings, and the watcher's first two and last raw lines:

```
=== PNG
exit code: 0
file                                                 first_seen            at_size  grows last_change         final_bytes gone
big_0001.png                                         1792229862.597840          0   1502 1792229866.544975     59514334 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
00:07.670  render           | Saved: '…/work/e54/PNG/out/big_0001.png'
media_type = IMAGE  file_format = PNG
returned: {'FINISHED'}
FINAL: [('big_0001.png', 59514334)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
1502
=== OPEN_EXR
exit code: 0
file                                                 first_seen            at_size  grows last_change         final_bytes gone
big_0001.exr                                         1792229871.151620          0     79 1792229871.737491      2703768 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
00:02.809  render           | Saved: '…/work/e54/OPEN_EXR/out/big_0001.exr'
media_type = IMAGE  file_format = OPEN_EXR
returned: {'FINISHED'}
FINAL: [('big_0001.exr', 2703768)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
79
=== OPEN_EXR_MULTILAYER
exit code: 0
file                                                 first_seen            at_size  grows last_change         final_bytes gone
big_0001.exr                                         1792229875.815112       4907     82 1792229876.377225      2703918 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
00:02.586  render           | Saved: '…/work/e54/OPEN_EXR_MULTILAYER/out/big_0001.exr'
media_type = MULTI_LAYER_IMAGE  file_format = OPEN_EXR_MULTILAYER
returned: {'FINISHED'}
FINAL: [('big_0001.exr', 2703918)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
82
--- raw, PNG
1792229862.597840 NEW   …/work/e54/PNG/out/big_0001.png 0
1792229862.672099 GROW  …/work/e54/PNG/out/big_0001.png 16384
…
1792229866.544975 GROW  …/work/e54/PNG/out/big_0001.png 59514334
--- raw, OPEN_EXR
1792229871.151620 NEW   …/work/e54/OPEN_EXR/out/big_0001.exr 0
1792229871.155893 GROW  …/work/e54/OPEN_EXR/out/big_0001.exr 4757
…
1792229871.737491 GROW  …/work/e54/OPEN_EXR/out/big_0001.exr 2703768
--- raw, OPEN_EXR_MULTILAYER
1792229875.815112 NEW   …/work/e54/OPEN_EXR_MULTILAYER/out/big_0001.exr 4907
1792229875.837554 GROW  …/work/e54/OPEN_EXR_MULTILAYER/out/big_0001.exr 50867
…
1792229876.377225 GROW  …/work/e54/OPEN_EXR_MULTILAYER/out/big_0001.exr 2703918
```

So in all three formats the file was visible under its final name while it was being written: the PNG appeared at 0 bytes and grew through 1502 readings over about four seconds to 59,514,334 bytes; the two OpenEXR files grew through 79 and 82 readings over about half a second. No other name appeared in the folder.

**Entry 55 — whether anything holds the file open while it is being written.** As entry 54, multi-layer OpenEXR, with a small program watching for the file; the first moment it existed, the program asked `lsof` which program had it open, then renamed it away and back:

```
holdcheck.py …/work/e55/out/big_0001.exr 300 &
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e05_big.py -- …/work/e55/out OPEN_EXR_MULTILAYER
```

What came back, in full — the watching program's lines, then the listing, then Blender's:

```
2.335 file seen, size 4907
COMMAND   PID      USER   FD   TYPE DEVICE SIZE/OFF      NODE NAME
Blender 42225 artist       4w   REG   1,18   263859 159685989 …/work/e55/out/big_0001.exr
lsof exit code: 0
rename: allowed
rename back: allowed
total 5288
-rw-r--r--@ 1 artist     wheel  2703918 Oct 17 12:38 big_0001.exr
00:02.559  render           | Saved: '…/work/e55/out/big_0001.exr'
media_type = MULTI_LAYER_IMAGE  file_format = OPEN_EXR_MULTILAYER
returned: {'FINISHED'}
FINAL: [('big_0001.exr', 2703918)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
blender exit code: 0
```

The `4w` in the `lsof` line means Blender held the file open for writing.

**How entries 56 to 59 watched a render.** One frame, `write_still=True`, 32 samples where the engine is Cycles, with Blender started directly in the background. Two instruments ran beside it. One listed every file Blender had open every half second with the system's `lsof`, keeping each distinct line once; the lines that are part of Blender itself or of the system (its program files, libraries, fonts, devices, its own log file's twin line and a system cache file under `/private/var/folders`) are left out below, and everything else it saw is kept. The other watched Blender's temporary folder — this sitting's own, `…/tmp`, set through `TMPDIR` — in a tight loop about every two milliseconds, as the folder watcher above. `quit.blend` in that folder was left there by two earlier trial runs with a window and is not part of these renders.

**Entry 56 — every file a rendering Blender has open, at a large picture and a small one.** Cycles on the processor, 32 samples, tile size at its shipped 2048, 8000 × 8000 and then 1000 × 1000:

```
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e56/big/out 8000 8000 2048 CYCLES &
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e56/small/out 1000 1000 2048 CYCLES &
```

At 8000 × 8000 — Blender's own lines, then what `lsof` saw, then the temporary folder:

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_PAQp3h/
tile_size = 2048  use_auto_tile = True
pid = 42310
04:06.778  render           | Saved: '…/work/e56/big/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 449
Blender 42310 artist     cwd    DIR   1,18       192           159686111 …/work/e56/big
Blender 42310 artist       1w   REG   1,18       191           159686118 …/work/e56/big/blender.log
Blender 42310 artist       2w   REG   1,18       191           159686118 …/work/e56/big/blender.log
Blender 42310 artist       4u   REG   1,18    118784           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18    208896           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18    294912           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18   5537792           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18   5627904           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18   5713920           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18   5799936           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  46247936           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  46333952           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  48369664           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  50126848           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  58159104           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  84578304           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  84664320           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  84754432           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  84840448           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18  87638016           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4r   REG   1,18  89572097           159686204 …/tmp/blender_PAQp3h/cycles-tile-buffer-42310-34612644848-0-0.exr
Blender 42310 artist       4u   REG   1,18   7917568           159687648 …/work/e56/big/out/t_.png
Blender 42310 artist       4u   REG   1,18  16793600           159687648 …/work/e56/big/out/t_.png
Blender 42310 artist       4u   REG   1,18  25415680           159687648 …/work/e56/big/out/t_.png
Blender 42310 artist       4u   REG   1,18  34095104           159687648 …/work/e56/big/out/t_.png
Blender 42310 artist       4u   REG   1,18  42987520           159687648 …/work/e56/big/out/t_.png
Blender 42310 artist       1w   REG   1,18       417           159686118 …/work/e56/big/blender.log
Blender 42310 artist       2w   REG   1,18       417           159686118 …/work/e56/big/blender.log
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792229943.881716      95581      0 1792229943.881716        95581 -
cycles-tile-buffer-42310-34612644848-0-0.exr         1792229950.485476          0     92 1792230043.349111     89572097 1792230187.884558
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

At 1000 × 1000, the same:

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_9GDONz/
tile_size = 2048  use_auto_tile = True
pid = 44302
00:03.547  render           | Saved: '…/work/e56/small/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 6
Blender 44302 artist     cwd    DIR   1,18       192           159687905 …/work/e56/small
Blender 44302 artist       1w   REG   1,18       191           159687908 …/work/e56/small/blender.log
Blender 44302 artist       2w   REG   1,18       191           159687908 …/work/e56/small/blender.log
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792230258.527579      95581      0 1792230258.527579        95581 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```
Then the same 8000 × 8000 render on the Mac's own graphics, by adding `--cycles-device METAL` after the script's arguments:

```
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e56/metal/out 8000 8000 2048 CYCLES --cycles-device METAL &
```

Blender's own lines, then what `lsof` saw — here also leaving out 83 lines naming Cycles' compiled-kernel files under `/Users/artist/.cache/cycles/kernels/Apple_M1_Pro/` and the system's graphics-library cache files under `/private/var/folders/…/C/org.blenderfoundation.blender/com.apple.metal/`, which are described below — then the temporary folder:

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_tE9utm/
tile_size = 2048  use_auto_tile = True
pid = 44349
03:15.936  render           | Saved: '…/work/e56/metal/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 364
Blender 44349 artist     cwd    DIR   1,18       192           159687944 …/work/e56/metal
Blender 44349 artist       1w   REG   1,18       191           159687949 …/work/e56/metal/blender.log
Blender 44349 artist       2w   REG   1,18       191           159687949 …/work/e56/metal/blender.log
Blender 44349 artist       9u   REG   1,18    118784           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18    208896           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18    294912           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18   5537792           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18   5627904           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18   5713920           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18   5799936           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  47366144           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  47456256           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  49557504           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  51355648           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  88080384           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  88170496           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  88256512           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18  88346624           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9r   REG   1,18  93076491           159688011 …/tmp/blender_tE9utm/cycles-tile-buffer-44349-42815802352-0-0.exr
Blender 44349 artist       9u   REG   1,18   5087232           159693229 …/work/e56/metal/out/t_.png
Blender 44349 artist       9u   REG   1,18  14094336           159693229 …/work/e56/metal/out/t_.png
Blender 44349 artist       9u   REG   1,18  23216128           159693229 …/work/e56/metal/out/t_.png
Blender 44349 artist       9u   REG   1,18  32239616           159693229 …/work/e56/metal/out/t_.png
Blender 44349 artist       9u   REG   1,18  41238528           159693229 …/work/e56/metal/out/t_.png
Blender 44349 artist       1w   REG   1,18       419           159687949 …/work/e56/metal/blender.log
Blender 44349 artist       2w   REG   1,18       419           159687949 …/work/e56/metal/blender.log
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792230267.332962      95581      0 1792230267.332962        95581 -
cycles-tile-buffer-44349-42815802352-0-0.exr         1792230280.285275          0     85 1792230317.572732     93076491 1792230460.377880
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

The 83 kernel-file lines were all opened for reading (`txt`). Afterwards, of the 514 files in that kernel folder, 83 had a modified time inside this run and 4 had been created during it:

```
Oct 17 12:44:41 2026 born, 1183968 bytes  ~/.cache/cycles/kernels/Apple_M1_Pro/integrator_shade_shadow/PSO_SPECIALIZED_SHADE/8348D6E0CE94339C2EBE87AF0102241C.bin
Oct 17 12:44:40 2026 born, 976352 bytes  ~/.cache/cycles/kernels/Apple_M1_Pro/integrator_shade_light_forward/PSO_SPECIALIZED_SHADE/B18C107EAC3743166867B4EB905BCE0F.bin
Oct 17 12:44:40 2026 born, 1057168 bytes  ~/.cache/cycles/kernels/Apple_M1_Pro/integrator_shade_background/PSO_SPECIALIZED_SHADE/B18C107EAC3743166867B4EB905BCE0F.bin
Oct 17 12:45:22 2026 born, 6093408 bytes  ~/.cache/cycles/kernels/Apple_M1_Pro/integrator_shade_surface/PSO_SPECIALIZED_SHADE/37B9841D64E179D28014A052E4D2F506.bin
```

This folder is in the user's home folder, not in Blender's temporary folder or its settings folder, so neither `TMPDIR` nor the run's own settings folder kept it out. The graphics-library file `libraries.data` was held open for reading and writing and was 704,708,608 bytes with a modified time of 12:44:39 afterwards.

So on the processor and on the graphics card alike, the 8000 × 8000 render wrote one tile file, `cycles-tile-buffer-<process number>-<number>-0-0.exr`, into Blender's temporary folder — 89,572,097 bytes on the processor and 93,076,491 on the graphics card — read it back, and removed it before Blender exited; the 1000 × 1000 render made no file there and `lsof` saw nothing of the kind.

**Entry 57 — the largest and a small tile size, and the switch Cycles calls deprecated.** As entry 56, Cycles on the processor, 8000 × 8000, three runs: tile size 8192; tile size 256; and tile size 2048 with `use_auto_tile` set to False after the tile size:

```
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e57/t8192/out 8000 8000 8192 CYCLES &
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e57/t256/out 8000 8000 256 CYCLES &
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e57/autooff/out 8000 8000 2048 CYCLES auto_tile_off &
```

Tile size 8192 — Blender's lines, `lsof`, the temporary folder:

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_UCDuLA/
tile_size = 8192  use_auto_tile = True
pid = 45614
03:44.752  render           | Saved: '…/work/e57/t8192/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 408
Blender 45614 artist     cwd    DIR   1,18       192           159693400 …/work/e57/t8192
Blender 45614 artist       1w   REG   1,18       191           159693403 …/work/e57/t8192/blender.log
Blender 45614 artist       2w   REG   1,18       191           159693403 …/work/e57/t8192/blender.log
Blender 45614 artist       4u   REG   1,18   7585792           159696001 …/work/e57/t8192/out/t_.png
Blender 45614 artist       4u   REG   1,18  16424960           159696001 …/work/e57/t8192/out/t_.png
Blender 45614 artist       4u   REG   1,18  25210880           159696001 …/work/e57/t8192/out/t_.png
Blender 45614 artist       4u   REG   1,18  34299904           159696001 …/work/e57/t8192/out/t_.png
Blender 45614 artist       4u   REG   1,18  43266048           159696001 …/work/e57/t8192/out/t_.png
Blender 45614 artist       1w   REG   1,18       419           159693403 …/work/e57/t8192/blender.log
Blender 45614 artist       2w   REG   1,18       419           159693403 …/work/e57/t8192/blender.log
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792230473.013084      95581      0 1792230473.013084        95581 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

Tile size 256 (the many `lsof` lines naming the tile file are counted, with the first and last kept):

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_MKQc0i/
tile_size = 256  use_auto_tile = True
pid = 46906
04:04.287  render           | Saved: '…/work/e57/t256/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 442
Blender 46906 artist     cwd    DIR   1,18       192           159696012 …/work/e57/t256
Blender 46906 artist       1w   REG   1,18       190           159696019 …/work/e57/t256/blender.log
Blender 46906 artist       2w   REG   1,18       190           159696019 …/work/e57/t256/blender.log
Blender 46906 artist       4u   REG   1,18     45056           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    131072           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    217088           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    307200           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    741376           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    831488           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    905216           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18    995328           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   1421312           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   1507328           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   1593344           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   1650688           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   1728512           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   2150400           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   2240512           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   2326528           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   2416640           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   2850816           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   2928640           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3014656           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3112960           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3551232           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3637248           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3715072           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3801088           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   4227072           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   4317184           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   4390912           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   4468736           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   4894720           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   4972544           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   5058560           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   5144576           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   5570560           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   5660672           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   5746688           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   5832704           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   6258688           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   6348800           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   6422528           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   6512640           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   7241728           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   7327744           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   7643136           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   8253440           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   8331264           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  10219520           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  10317824           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  10514432           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  11198464           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  11882496           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  12218368           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  12496896           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  15532032           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  15618048           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  16027648           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  16711680           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  17395712           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  17731584           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  18071552           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  18743296           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  18976768           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  22896640           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  22982656           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  23572480           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  24256512           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  24940544           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  25272320           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  25948160           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  26284032           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  26673152           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  30621696           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  30699520           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  31309824           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  32333824           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  32673792           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  33009664           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  33685504           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  34021376           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  34402304           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  38387712           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  38465536           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  38776832           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  39456768           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  40140800           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  40816640           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  41148416           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  41824256           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  42209280           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  46247936           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  46333952           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  46665728           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  47349760           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  48029696           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  48705536           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  49041408           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  49713152           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  50126848           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  54206464           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  54292480           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  54972416           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  55656448           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  56004608           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  56692736           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  57028608           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  57696256           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  58122240           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  58200064           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  62324736           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  62713856           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  63053824           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  63754240           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  64425984           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  64765952           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  65462272           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  65798144           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  66285568           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  70483968           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  70586368           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  71290880           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  71634944           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  72318976           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  72650752           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  72990720           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  73678848           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  74379264           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  74575872           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  78237696           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  78340096           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  78921728           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  79265792           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  79937536           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  80605184           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  80695296           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  82329600           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  82415616           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  82722816           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  82821120           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  83255296           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  83333120           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  83398656           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  83484672           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  83931136           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  84021248           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  84107264           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  84205568           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  84643840           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  84742144           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  84840448           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  85266432           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  85344256           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  85430272           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  85528576           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  85975040           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86052864           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86138880           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86216704           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86654976           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86753280           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86839296           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  86937600           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  87384064           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  87494656           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  87592960           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88031232           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88117248           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88195072           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88293376           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88727552           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88825856           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  88903680           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  89001984           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18  89477120           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4r   REG   1,18  89572097           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4u   REG   1,18   3936256           159711076 …/work/e57/t256/out/t_.png
Blender 46906 artist       4u   REG   1,18  12869632           159711076 …/work/e57/t256/out/t_.png
Blender 46906 artist       4u   REG   1,18  21798912           159711076 …/work/e57/t256/out/t_.png
Blender 46906 artist       4u   REG   1,18  30609408           159711076 …/work/e57/t256/out/t_.png
Blender 46906 artist       4u   REG   1,18  39190528           159711076 …/work/e57/t256/out/t_.png
Blender 46906 artist       4u   REG   1,18  48107520           159711076 …/work/e57/t256/out/t_.png
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792230700.453786      95581      0 1792230700.453786        95581 -
cycles-tile-buffer-46906-33431792624-0-0.exr         1792230701.854952      32768   1049 1792230799.593707     89572097 1792230941.644736
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
(lsof lines naming the tile file: 174; the first and last of them:)
Blender 46906 artist       4u   REG   1,18     45056           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
Blender 46906 artist       4r   REG   1,18  89572097           159696022 …/tmp/blender_MKQc0i/cycles-tile-buffer-46906-33431792624-0-0.exr
```

Tile size 2048 with `use_auto_tile` set to False (the same):

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_KGMEmg/
tile_size = 2048  use_auto_tile = False
pid = 48364
04:11.790  render           | Saved: '…/work/e57/autooff/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 453
Blender 48364 artist     cwd    DIR   1,18       192           159711113 …/work/e57/autooff
Blender 48364 artist       1w   REG   1,18       192           159711123 …/work/e57/autooff/blender.log
Blender 48364 artist       2w   REG   1,18       192           159711123 …/work/e57/autooff/blender.log
Blender 48364 artist       4u   REG   1,18    118784           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18    208896           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18    294912           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18   5537792           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18   5627904           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18   5713920           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18   5799936           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  46247936           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  46333952           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  48369664           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  50126848           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  79769600           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  84578304           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  84664320           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  84754432           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18  84840448           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4r   REG   1,18  89572097           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4u   REG   1,18   3174400           159747107 …/work/e57/autooff/out/t_.png
Blender 48364 artist       4u   REG   1,18  11952128           159747107 …/work/e57/autooff/out/t_.png
Blender 48364 artist       4u   REG   1,18  20779008           159747107 …/work/e57/autooff/out/t_.png
Blender 48364 artist       4u   REG   1,18  29523968           159747107 …/work/e57/autooff/out/t_.png
Blender 48364 artist       4u   REG   1,18  38707200           159747107 …/work/e57/autooff/out/t_.png
Blender 48364 artist       4u   REG   1,18  47493120           159747107 …/work/e57/autooff/out/t_.png
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792230947.156008      95581      0 1792230947.156008        95581 -
cycles-tile-buffer-48364-52355263472-0-0.exr         1792230952.956668      32768     75 1792231045.227303     89572097 1792231195.785555
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
(lsof lines naming the tile file: 17; the first and last of them:)
Blender 48364 artist       4u   REG   1,18    118784           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
Blender 48364 artist       4r   REG   1,18  89572097           159711141 …/tmp/blender_KGMEmg/cycles-tile-buffer-48364-52355263472-0-0.exr
```

So at tile size 8192 — one tile larger than the 8000 × 8000 picture — no tile file was written; at 256 one tile file was written and grew through 1049 readings; and with `use_auto_tile` set to False, which read back as `False`, the tile file was written as at 2048 in entry 56.

**Entry 58 — where tiling starts: one tile's area against one tile and a row.** As entry 56, Cycles on the processor, tile size 2048, at 2048 × 2048 and then at 2048 × 2049:

```
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e58/s2048/out 2048 2048 2048 CYCLES &
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e58/s2049/out 2048 2049 2048 CYCLES &
```

At 2048 × 2048:

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_OIdRnV/
tile_size = 2048  use_auto_tile = True
pid = 50942
00:21.677  render           | Saved: '…/work/e58/s2048/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 39
Blender 50942 artist     cwd    DIR   1,18       192           159747592 …/work/e58/s2048
Blender 50942 artist       1w   REG   1,18       191           159747595 …/work/e58/s2048/blender.log
Blender 50942 artist       2w   REG   1,18       191           159747595 …/work/e58/s2048/blender.log
Blender 50942 artist       4u   REG   1,18   2658304           159749541 …/work/e58/s2048/out/t_.png
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792231260.659658      95581      0 1792231260.659658        95581 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

At 2048 × 2049:

```
engine names offered: ['BLENDER_EEVEE']
engine = CYCLES
tempdir = …/tmp/blender_3XyYn5/
tile_size = 2048  use_auto_tile = True
pid = 51239
00:20.216  render           | Saved: '…/work/e58/s2049/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 36
Blender 51239 artist     cwd    DIR   1,18       192           159749592 …/work/e58/s2049
Blender 51239 artist       1w   REG   1,18       191           159749595 …/work/e58/s2049/blender.log
Blender 51239 artist       2w   REG   1,18       191           159749595 …/work/e58/s2049/blender.log
Blender 51239 artist       4u   REG   1,18   1703936           159749686 …/work/e58/s2049/out/t_.png
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792231284.754974      95581      0 1792231284.754974        95581 -
cycles-tile-buffer-51239-41609726960-0-0.exr         1792231294.623523          0     24 1792231294.823813      6169360 1792231305.227569
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

So the temporary-folder watcher saw a 6,169,360-byte tile file for 2048 × 2049, made and removed within the render, and none for 2048 × 2048; the half-second `lsof` sampler saw no tile file in either run.

**Entry 59 — what EEVEE and Workbench write while a render runs.** As entry 56, 8000 × 8000, with the engine set to EEVEE (the only EEVEE name 5.3 offers is `BLENDER_EEVEE`) and then to Workbench:

```
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e59/eevee/out 8000 8000 2048 EEVEE &
TMPDIR=…/tmp … "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python tiles.py -- …/work/e59/workbench/out 8000 8000 2048 BLENDER_WORKBENCH &
```

EEVEE:

```
engine names offered: ['BLENDER_EEVEE']
engine = BLENDER_EEVEE
tempdir = …/tmp/blender_e2Ujid/
tile_size = 2048  use_auto_tile = True
pid = 51450
00:18.811  render           | Saved: '…/work/e59/eevee/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 35
Blender 51450 artist     cwd    DIR   1,18       192           159749700 …/work/e59/eevee
Blender 51450 artist       1w   REG   1,18         0           159749708 …/work/e59/eevee/blender.log
Blender 51450 artist       2w   REG   1,18         0           159749708 …/work/e59/eevee/blender.log
Blender 51450 artist     txt    REG   1,18 704708608            33329691 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/32024/libraries.data
Blender 51450 artist     txt    REG   1,18 559218688              225717 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/16777235_530/functions.data
Blender 51450 artist       1w   REG   1,18       198           159749708 …/work/e59/eevee/blender.log
Blender 51450 artist       2w   REG   1,18       198           159749708 …/work/e59/eevee/blender.log
Blender 51450 artist       5u   REG   1,18 704708608            33329691 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/32024/libraries.data
Blender 51450 artist       6u   REG   1,18    205620           159749726 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/32024/libraries.list
Blender 51450 artist       7u   REG   1,18 559218688              225717 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/16777235_530/functions.data
Blender 51450 artist       8u   REG   1,18    318852           159749727 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/16777235_530/functions.list
Blender 51450 artist       4u   REG   1,18   3420160           159749801 …/work/e59/eevee/out/t_.png
Blender 51450 artist       4u   REG   1,18  10088448           159749801 …/work/e59/eevee/out/t_.png
Blender 51450 artist       4u   REG   1,18  17129472           159749801 …/work/e59/eevee/out/t_.png
Blender 51450 artist       4u   REG   1,18  24174592           159749801 …/work/e59/eevee/out/t_.png
Blender 51450 artist       4u   REG   1,18  31666176           159749801 …/work/e59/eevee/out/t_.png
Blender 51450 artist       1w   REG   1,18       426           159749708 …/work/e59/eevee/blender.log
Blender 51450 artist       2w   REG   1,18       426           159749708 …/work/e59/eevee/blender.log
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792231307.303151      95581      0 1792231307.303151        95581 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

Workbench:

```
engine names offered: ['BLENDER_EEVEE']
engine = BLENDER_WORKBENCH
tempdir = …/tmp/blender_Gf7Hb2/
tile_size = 2048  use_auto_tile = True
pid = 51767
00:05.343  render           | Saved: '…/work/e59/workbench/out/t_.png'
output folder: ['t_.png']
tempdir after: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

--- open files (not Blender's own, not the system's):
SAMPLES TAKEN: 10
Blender 51767 artist     cwd    DIR   1,18       192           159749843 …/work/e59/workbench
Blender 51767 artist       1w   REG   1,18         0           159749846 …/work/e59/workbench/blender.log
Blender 51767 artist       2w   REG   1,18         0           159749846 …/work/e59/workbench/blender.log
Blender 51767 artist       1w   REG   1,18       202           159749846 …/work/e59/workbench/blender.log
Blender 51767 artist       2w   REG   1,18       202           159749846 …/work/e59/workbench/blender.log
Blender 51767 artist     txt    REG   1,18 704708608            33329691 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/32024/libraries.data
Blender 51767 artist     txt    REG   1,18 559218688              225717 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/16777235_530/functions.data
Blender 51767 artist       5u   REG   1,18 704708608            33329691 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/32024/libraries.data
Blender 51767 artist       6u   REG   1,18    205620           159749861 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/32024/libraries.list
Blender 51767 artist       7u   REG   1,18 559218688              225717 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/16777235_530/functions.data
Blender 51767 artist       8u   REG   1,18    318852           159749862 /private/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/C/org.blenderfoundation.blender/com.apple.metal/16777235_530/functions.list
Blender 51767 artist       4u   REG   1,18   1335296           159749871 …/work/e59/workbench/out/t_.png
Blender 51767 artist       4u   REG   1,18   8949760           159749871 …/work/e59/workbench/out/t_.png
Blender 51767 artist       4u   REG   1,18  16859136           159749871 …/work/e59/workbench/out/t_.png
Blender 51767 artist       4u   REG   1,18  25415680           159749871 …/work/e59/workbench/out/t_.png
Blender 51767 artist       4u   REG   1,18  32804864           159749871 …/work/e59/workbench/out/t_.png
--- temporary folder:
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792231328.991474      95581      0 1792231328.991474        95581 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

Neither wrote anything to the temporary folder. Both held macOS's graphics-library cache files under `/private/var/folders/…/com.apple.metal/` open, and nothing else besides their own output picture and log.

**Entry 60 — whether a render writes a texture cache beside the pictures it reads.** At factory settings, no window: a 1024 × 1024 colour-grid picture made and saved as PNG by Blender itself into `…/tex`, loaded as an image texture into a new material on the cube, and a Cycles render at 4096 × 4096, 8 samples, saved as a still. Run A at factory settings; run B a script printing every render setting, Cycles scene setting and file-path preference whose name mentions texture or cache; run C as A with every such switch run B found turned on:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e11_tex.py -- …/work/e60/A off
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e11b_probe.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e11_tex.py -- …/work/e60/C on
find …/work/e60 -type f | sort
```

What came back, in full apart from Cycles' progress lines:

```
=== A, factory settings
…/scripts/new/e11_tex.py:22: DeprecationWarning: 'Material.use_nodes' is expected to be removed in Blender 6.0
  mat.use_nodes = True
00:42.016  render           | Saved: '…/work/e60/A/out/r_.png'
texture folder before the render: ['smoozytex.png']
texture folder after the render: ['smoozytex.png']
  under the texture folder: …/work/e60/A/tex/smoozytex.png 213596
output folder: ['r_.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
=== B, what 5.3 offers
VERSION 5.3.0 Beta
--- render settings whose name mentions texture, tx or cache ---
   use_render_cache BOOLEAN False
   use_texture_cache BOOLEAN True
   use_auto_generate_texture_cache BOOLEAN False
   use_compositor_frames_cache BOOLEAN False
--- cycles scene settings whose name mentions texture, tx or cache ---
   texture_resolution FLOAT 1.0
   texture_resolution_render FLOAT 1.0
   texture_limit ENUM 'OFF'
   texture_limit_render ENUM 'OFF'
   debug_use_texture_cache_eviction BOOLEAN True
   debug_texture_cache_preserve_unused INT 0
--- file path preferences whose name mentions texture, tx or cache ---
   texture_directory STRING '//'
   render_cache_directory STRING ''
   texture_cache_directory STRING ''
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
=== C, every switch on
…/scripts/new/e11_tex.py:22: DeprecationWarning: 'Material.use_nodes' is expected to be removed in Blender 6.0
  mat.use_nodes = True
00:40.529  render           | Saved: '…/work/e60/C/out/r_.png'
texture folder before the render: ['smoozytex.png']
set use_texture_cache = True
set use_auto_generate_texture_cache = True
texture folder after the render: ['blender_tx', 'smoozytex.png']
  under the texture folder: …/work/e60/C/tex/smoozytex.png 213596
  under the texture folder: …/work/e60/C/tex/blender_tx/smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx 535017
output folder: ['r_.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
…/work/e60/A/out/r_.png
…/work/e60/A/tex/smoozytex.png
…/work/e60/C/out/r_.png
…/work/e60/C/tex/blender_tx/smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx
…/work/e60/C/tex/smoozytex.png
```

So at factory settings nothing appeared beside the picture; with `use_texture_cache` and `use_auto_generate_texture_cache` on, a folder `blender_tx` was made beside the picture, holding `smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx` (535,017 bytes). The file-path preference `texture_cache_directory` read `''`.

**Entry 61 — what a File Output node's folder holds while a render is working.** At factory settings, no window: Cycles, 400 samples, 1920 × 1080, frames 1 to 4, PNG, the scene's output `…/out/f_`; a File Output node writing PNG (one input `Image`, name `n_`) aimed at `…/node/`, which did not exist. The folder above both, `…/work/e61`, was watched — which also caught the watcher's own log and Blender's log as they grew; those two are left out of the summary:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e12_node.py -- …/work/e61
```

Blender's own lines (per-sample progress and one deprecation warning left out):

```
00:15.706  render           | Saved: '…/work/e61/node/n_Image0001.png'
00:16.117  render           | Saved: '…/work/e61/out/f_0001.png'
00:33.904  render           | Saved: '…/work/e61/node/n_Image0002.png'
00:34.310  render           | Saved: '…/work/e61/out/f_0002.png'
00:53.651  render           | Saved: '…/work/e61/node/n_Image0003.png'
00:54.109  render           | Saved: '…/work/e61/out/f_0003.png'
01:13.247  render           | Saved: '…/work/e61/node/n_Image0004.png'
01:13.724  render           | Saved: '…/work/e61/out/f_0004.png'
node folder exists before the render: False
returned: {'FINISHED'}
FINAL out: ['f_0001.png', 'f_0002.png', 'f_0003.png', 'f_0004.png']
FINAL node: ['n_Image0001.png', 'n_Image0002.png', 'n_Image0003.png', 'n_Image0004.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
```

The watcher's summary of the picture files, and its first lines:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
n_Image0001.png                                      1792231516.555220          0     72 1792231517.026837      1712088 -
f_0001.png                                           1792231517.047980       4096     67 1792231517.435167      1599935 -
n_Image0002.png                                      1792231534.780673          0     68 1792231535.213079      1712088 -
f_0002.png                                           1792231535.241798          0     66 1792231535.628224      1599935 -
n_Image0003.png                                      1792231554.544251          0     65 1792231554.964205      1712088 -
f_0003.png                                           1792231554.985687          0     73 1792231555.421264      1599935 -
n_Image0004.png                                      1792231574.099398          0     66 1792231574.566534      1712088 -
f_0004.png                                           1792231574.579957          0     68 1792231575.042772      1599935 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
…
1792231500.470744 NEW   …/work/e61/watch.log 113
1792231500.856199 NEW   …/work/e61/blender.log 0
1792231516.555220 NEW   …/work/e61/node/n_Image0001.png 0
```

So the node's folder did not exist before the render and appeared with the node's first file; each frame's node file appeared at 0 bytes, grew, and finished about half a second before that frame's scene file appeared, and both were finished before the next frame's node file appeared.

**Entry 62 — whether the render cache file is there before the render finishes.** At factory settings, no window: Cycles, 400 samples, 1920 × 1080, two view layers, frames 1 to 3, PNG, Cache Result on, and the render cache folder set to an empty folder `…/cache`; the output folder and the cache folder watched together:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e13_cache.py -- …/work/e62 …/work/e62/cache
```

Blender's own lines (per-sample progress left out):

```
00:25.969  render           | Saved: '…/work/e62/out/f_0001.png'
00:50.140  render           | Saved: '…/work/e62/out/f_0002.png'
01:13.517  render           | Saved: '…/work/e62/out/f_0003.png'
VERSION 5.3.0 Beta
use_render_cache = True
render_cache_directory = '…/work/e62/cache'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: True
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: [('cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr', 132799543)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Caching exr file, 1920x1080, …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
Caching exr file, 1920x1080, …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
Caching exr file, 1920x1080, …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
exit code: 0
```

The watcher's summary — where the overlap check lists the cache file against the later frames because it is one file that changes during every frame:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 1792231629.730399          0    337 1792231678.853203    132799543 -
f_0001.png                                           1792231631.112505          0     62 1792231631.411715      1600138 -
f_0002.png                                           1792231655.488046          0     36 1792231655.582281      1600138 -
f_0003.png                                           1792231678.862082          0     36 1792231678.958889      1600138 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr changed at 1792231678.853203, after f_0001.png appeared at 1792231631.112505
  cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr changed at 1792231678.853203, after f_0002.png appeared at 1792231655.488046

```

and its raw lines just before each frame's own file appeared:

```
1792231631.051192 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 132010423
1792231631.057620 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 132799543
1792231631.112505 NEW   …/work/e62/out/f_0001.png 0
…
1792231655.473523 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 130367927
1792231655.476102 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 132799543
1792231655.488046 NEW   …/work/e62/out/f_0002.png 0
…
1792231678.848257 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 131719607
1792231678.853203 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 132799543
1792231678.862082 NEW   …/work/e62/out/f_0003.png 0
…
```

and where the cache file went back to a smaller size for the next frame:

```
1792231629.730399 NEW   …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 0
1792231655.383822 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 0
1792231678.744309 GROW  …/work/e62/cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr 205239
```

So one cache file, `cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr`, appeared about a second and a half before frame 1's picture, reached 132,799,543 bytes a few hundredths of a second before each frame's picture appeared, was written again from the start for each frame, and was still there at the end.

**Entry 63 — whether the output folder holds anything different at a very large picture size.** As entry 51 — Cycles, PNG, Placeholders off — but 8192 × 4608, 16 samples, frames 1 to 3:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e16_bigframes.py -- …/work/e63/out
```

Blender's own lines (per-sample progress left out), and the watcher's summary:

```
02:08.936  render           | Saved: '…/work/e63/out/f_0001.png'
04:13.732  render           | Saved: '…/work/e63/out/f_0002.png'
06:36.682  render           | Saved: '…/work/e63/out/f_0003.png'
FINAL: [('f_0001.png', 28099358), ('f_0002.png', 28099358), ('f_0003.png', 28099358)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
file                                                 first_seen            at_size  grows last_change         final_bytes gone
f_0001.png                                           1792231853.595088          0   1286 1792231860.877806     28099358 -
f_0002.png                                           1792231983.755514          0    567 1792231985.679327     28099358 -
f_0003.png                                           1792232126.692125          0    651 1792232128.628449     28099358 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

So, as at 1920 × 1080 in entry 51, nothing stood in the folder for a frame while it was rendered, and each frame's file appeared at 0 bytes and grew in place — here for about two to seven seconds rather than a tenth of a second. Nothing else appeared in the folder.

**Entry 72 — the tile file during a three-frame render.** Written out in full under question 14. At 4000 × 4000 with tile size 256, each frame wrote its own tile file into Blender's temporary folder — `cycles-tile-buffer-72403-39007243248-0-0.exr`, then `…-1-0.exr`, then `…-2-0.exr`, each 9,564,578 bytes — while it rendered, and each was gone before that frame's output file appeared.

**Entries 74 and 75 — a single video file during the run.** Written out in full under question 15. The video file was present under its final name from the first frame, at 0 bytes, and grew as frames were added — in steps of 262,144 bytes for H.264 in Matroska, and about half its final size by the half-way frame for lossless HUFFYUV.

**Entry 78 — the same with Blender's window open.** Written out in full under question 6. A still rendered from a script with no save switch put nothing at the output path; with `write_still=True` it wrote `still_.png`. During a large F12, Cycles' tile file stood in Blender's temporary folder for about eight seconds and was gone afterwards.

---

## 12. What does a render do when it must make a folder and cannot?

**Answer.** Not in the sources. The only documented folder-making for render output is the File Output node's "Any missing directories are created automatically during output" (question 4). Nothing says what happens if that fails.

**Label.** Not in the sources.

**Where I looked.**
- Everything listed under questions 4 and 5.
- Searched the whole manual, API docs, 5.3 release notes and Cycles Python files for: "cannot create", "can't create", "could not create", "couldn't create", "unable to create", "failed to create", "fail to make", "mkdir", "makedirs", "permission", "read-only", "access denied".
- Hits were about mesh editing, constraints, cloth, the Python API's internal errors ("couldn’t create bpy_struct object"), OSL query errors, and the Ocean cache. None is about a render failing to make a folder.

**Not covered.** The whole question, for both the main render output and the File Output node.

**Outside the interface.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, when the scene's output folder could not be made the render stopped at frame 1, printed `Couldn't create directory for file …: <reason>` and `Render error (<reason>) cannot save: '…'`, raised `RuntimeError` to Python and ended with exit code 1 (entries 45, 46); when a File Output node's folder, a node input's folder or the render cache folder could not be made, the render carried on, wrote the scene's frames, printed errors and ended with exit code 0 (entries 47 to 49). In Blender's window, the status bar and the Info editor showed the `Render error` line (entry 85). The folders that could not be made were inside a read-only volume, or blocked by an ordinary file; a folder refused by its own permissions was not run for 5.3 (the tool refused `chmod`).

**Operating system.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. On this Mac the read-only volume's refusal was named `Read-only file system` (entries 45, 47, 48).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**About entries 45, 47 and 48 — how the folder that cannot be written to was made.** In each, the folder that cannot be written to is a four-megabyte disk image, made with `hdiutil` inside the working folder and mounted **read-only** at that folder, so Blender's attempt to make a folder inside it is refused because the volume is read-only. A `touch` into it before each run printed `Read-only file system` with exit code 1. The earlier versions' form of these tests refused the write through the folder's own permissions (`chmod 500`); that could not be done in this sitting, so **what 5.3 does when a folder's own permissions refuse it is not run**, and nothing below answers it.

**Entry 45 — the scene's output folder cannot be made, one level and three levels, with a control.** At factory settings, no window, Workbench at 320 × 240, frames 1 to 3, PNG; a script set the output path from its argument, rendered an animation from Python inside a `try`, and said whether the folder existed afterwards. Three paths: `locked/newfolder/f_` and `locked/a/b/c/f_` inside the read-only folder, and `open/a/b/c/f_` inside an ordinary folder as the control:

```
hdiutil create -size 4m -fs HFS+ -volname SMOOZYRO45 …/work/e45/ro.dmg
hdiutil attach -readonly -nobrowse -mountpoint …/work/e45/locked …/work/e45/ro.dmg
touch …/work/e45/locked/canary.txt ; echo "touch exit code: $?"
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e18_locked.py -- …/work/e45/<path>      (for each of the three)
find …/work/e45 | sort
hdiutil detach …/work/e45/locked
```

What came back, in full:

```
created: …/work/e45/ro.dmg
/dev/disk4          	GUID_partition_scheme          	
/dev/disk4s1        	Apple_HFS                      	…/work/e45/locked
touch: …/work/e45/locked/canary.txt: Read-only file system
touch exit code: 1
=== locked/newfolder/f_
00:00.580  image            | ERROR Couldn't create directory for file …/work/e45/locked/newfolder/f_0001.png: Read-only file system
Error: Render error (Read-only file system) cannot save: '…/work/e45/locked/newfolder/f_0001.png'
operator raised: RuntimeError Error: Render error (Read-only file system) cannot save: '…/work/e45/locked/newfolder/f_0001.png'

does the folder exist now: False
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 1
=== locked/a/b/c/f_
00:00.504  image            | ERROR Couldn't create directory for file …/work/e45/locked/a/b/c/f_0001.png: Read-only file system
Error: Render error (Read-only file system) cannot save: '…/work/e45/locked/a/b/c/f_0001.png'
operator raised: RuntimeError Error: Render error (Read-only file system) cannot save: '…/work/e45/locked/a/b/c/f_0001.png'

does the folder exist now: False
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 1
=== open/a/b/c/f_
00:00.526  render           | Saved: '…/work/e45/open/a/b/c/f_0001.png'
00:00.541  render           | Saved: '…/work/e45/open/a/b/c/f_0002.png'
00:00.558  render           | Saved: '…/work/e45/open/a/b/c/f_0003.png'
operator returned: {'FINISHED'}
does the folder exist now: True
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
…/work/e45
…/work/e45/locked
…/work/e45/open
…/work/e45/open/a
…/work/e45/open/a/b
…/work/e45/open/a/b/c
…/work/e45/open/a/b/c/f_0001.png
…/work/e45/open/a/b/c/f_0002.png
…/work/e45/open/a/b/c/f_0003.png
…/work/e45/ro.dmg
"disk4" ejected.
```

**Entry 46 — a folder that cannot be made because an ordinary file stands in its place.** A file `blocker` holding one line of text; the output path `…/work/e46/blocker/f_`; the same script and scene as entry 45:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e18_locked.py -- …/work/e46/blocker/f_
ls -la …/work/e46
```

What came back, in full:

```
00:00.500  image            | ERROR Couldn't create directory for file …/work/e46/blocker/f_0001.png: File exists
Error: Render error (File exists) cannot save: '…/work/e46/blocker/f_0001.png'
operator raised: RuntimeError Error: Render error (File exists) cannot save: '…/work/e46/blocker/f_0001.png'

does the folder exist now: False
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 1
total 8
drwxr-xr-x@  3 artist     wheel    96 Oct 17 12:30 .
drwxr-xr-x@ 42 artist     wheel  1344 Oct 17 12:30 ..
-rw-r--r--@  1 artist     wheel    15 Oct 17 12:30 blocker
```

**Entry 47 — a File Output node's own folder cannot be made.** A scene with a File Output node writing PNG (one input `Image`, name `n_`) aimed at `…/locked/nodeout/` inside the read-only folder, and the scene's own output aimed at an ordinary folder `…/scene/s_`; Workbench, 320 × 240, frames 1 to 3; an animation rendered from Python:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e20_node_locked.py -- …/work/e47/scene …/work/e47/locked/nodeout
```

What came back, in full (one deprecation warning for `Scene.compositing_node_group` left out):

```
00:00.507  image            | ERROR Couldn't create directory for file …/work/e47/locked/nodeout/n_Image0001.png: Read-only file system
00:00.507  reports          | ERROR Render error (Read-only file system) cannot save: '…/work/e47/locked/nodeout/n_Image0001.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.540  render           | Saved: '…/work/e47/scene/s_0001.png'
00:00.547  image            | ERROR Couldn't create directory for file …/work/e47/locked/nodeout/n_Image0002.png: Read-only file system
00:00.547  reports          | ERROR Render error (Read-only file system) cannot save: '…/work/e47/locked/nodeout/n_Image0002.png'
00:00.552  render           | Saved: '…/work/e47/scene/s_0002.png'
00:00.560  image            | ERROR Couldn't create directory for file …/work/e47/locked/nodeout/n_Image0003.png: Read-only file system
00:00.560  reports          | ERROR Render error (Read-only file system) cannot save: '…/work/e47/locked/nodeout/n_Image0003.png'
00:00.565  render           | Saved: '…/work/e47/scene/s_0003.png'
VERSION 5.3.0 Beta
input names: ['Image']
node folder exists before: False
operator returned: {'FINISHED'}
node folder exists now: False
node folder holds: []
scene folder holds: ['s_0001.png', 's_0002.png', 's_0003.png']

exit code: 0
```

**Entry 48 — the folder an input's name asks for: made, and when it cannot be made.** The same scene, with the node's one input named `passes/diffuse`. First the control, the node's base folder an ordinary existing folder `…/nodeopen`; then the node's base folder the read-only folder `…/nodelocked`, which exists:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e20_node_locked.py -- …/work/e48/scene1 …/work/e48/nodeopen passes/diffuse
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e20_node_locked.py -- …/work/e48/scene2 …/work/e48/nodelocked passes/diffuse
find …/work/e48 | sort
```

What came back, in full (deprecation warnings left out):

```
=== control
00:00.521  render           | Saved: '…/work/e48/nodeopen/n_passes/diffuse0001.png'
00:00.525  render           | Saved: '…/work/e48/scene1/s_0001.png'
00:00.535  render           | Saved: '…/work/e48/nodeopen/n_passes/diffuse0002.png'
00:00.539  render           | Saved: '…/work/e48/scene1/s_0002.png'
00:00.548  render           | Saved: '…/work/e48/nodeopen/n_passes/diffuse0003.png'
00:00.552  render           | Saved: '…/work/e48/scene1/s_0003.png'
VERSION 5.3.0 Beta
input names: ['passes/diffuse']
node folder exists before: True
operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: ['n_passes/diffuse0001.png', 'n_passes/diffuse0002.png', 'n_passes/diffuse0003.png']
scene folder holds: ['s_0001.png', 's_0002.png', 's_0003.png']
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== locked base
00:00.633  image            | ERROR Couldn't create directory for file …/work/e48/nodelocked/n_passes/diffuse0001.png: Read-only file system
00:00.633  reports          | ERROR Render error (Read-only file system) cannot save: '…/work/e48/nodelocked/n_passes/diffuse0001.png'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.671  render           | Saved: '…/work/e48/scene2/s_0001.png'
00:00.694  image            | ERROR Couldn't create directory for file …/work/e48/nodelocked/n_passes/diffuse0002.png: Read-only file system
00:00.694  reports          | ERROR Render error (Read-only file system) cannot save: '…/work/e48/nodelocked/n_passes/diffuse0002.png'
00:00.699  render           | Saved: '…/work/e48/scene2/s_0002.png'
00:00.716  image            | ERROR Couldn't create directory for file …/work/e48/nodelocked/n_passes/diffuse0003.png: Read-only file system
00:00.716  reports          | ERROR Render error (Read-only file system) cannot save: '…/work/e48/nodelocked/n_passes/diffuse0003.png'
00:00.720  render           | Saved: '…/work/e48/scene2/s_0003.png'
VERSION 5.3.0 Beta
input names: ['passes/diffuse']
node folder exists before: True
operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: []
scene folder holds: ['s_0001.png', 's_0002.png', 's_0003.png']

exit code: 0
…/work/e48
…/work/e48/nodelocked
…/work/e48/nodeopen
…/work/e48/nodeopen/n_passes
…/work/e48/nodeopen/n_passes/diffuse0001.png
…/work/e48/nodeopen/n_passes/diffuse0002.png
…/work/e48/nodeopen/n_passes/diffuse0003.png
…/work/e48/ro.dmg
…/work/e48/scene1
…/work/e48/scene1/s_0001.png
…/work/e48/scene1/s_0002.png
…/work/e48/scene1/s_0003.png
…/work/e48/scene2
…/work/e48/scene2/s_0001.png
…/work/e48/scene2/s_0002.png
…/work/e48/scene2/s_0003.png
```

**Entry 49 — the render cache folder: made when missing, and when it cannot be made.** Cache Result on, two view layers, frames 1 to 3, PNG, the render cache folder set in the preferences by the script. First the cache folder pointed at `…/open/cachefolder`, missing under an ordinary writable folder, with Workbench at 320 × 240. Then pointed at `…/locked/cachefolder`, missing inside the read-only folder: run A with Cycles, 400 samples, 1920 × 1080, and run B with Workbench at 320 × 240. The commands:

```
hdiutil create -size 4m -fs HFS+ -volname SMOOZYRO49 …/work/e49/ro.dmg
hdiutil attach -readonly -nobrowse -mountpoint …/work/e49/locked …/work/e49/ro.dmg
touch …/work/e49/locked/canary.txt ; echo "touch exit code: $?"
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e13_cache.py -- …/work/e49/wC …/work/e49/open/cachefolder WB
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e13_cache.py -- …/work/e49/wA …/work/e49/locked/cachefolder
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e13_cache.py -- …/work/e49/wB …/work/e49/locked/cachefolder WB
find …/work/e49 | sort
hdiutil detach …/work/e49/locked
```

What came back, in full (the two times printed around run A are the shell's clock):

```
created: …/work/e49/ro.dmg
/dev/disk4          	GUID_partition_scheme          	
/dev/disk4s1        	Apple_HFS                      	…/work/e49/locked
touch: …/work/e49/locked/canary.txt: Read-only file system
touch exit code: 1
=== missing, parent writable
00:00.519  render           | Saved: '…/work/e49/wC/out/f_0001.png'
00:00.537  render           | Saved: '…/work/e49/wC/out/f_0002.png'
00:00.554  render           | Saved: '…/work/e49/wC/out/f_0003.png'
VERSION 5.3.0 Beta
use_render_cache = True
render_cache_directory = '…/work/e49/open/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: [('cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr', 2468413)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Caching exr file, 320x240, …/work/e49/open/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
Caching exr file, 320x240, …/work/e49/open/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
Caching exr file, 320x240, …/work/e49/open/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
exit code: 0
=== cannot be made, run A
12:30:24
00:30.954  image.openexr    | ERROR IMB_exr_write_end: No such file or directory.
00:30.955  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Caching exr file, 1920x1080, …/work/e49/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:31.210  render           | Saved: '…/work/e49/wA/out/f_0001.png'
01:01.020  image.openexr    | ERROR IMB_exr_write_end: No such file or directory.
01:01.020  reports          | ERROR Error writing render result, No such file or directory (see console)
Caching exr file, 1920x1080, …/work/e49/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
01:01.132  render           | Saved: '…/work/e49/wA/out/f_0002.png'
01:32.534  image.openexr    | ERROR IMB_exr_write_end: No such file or directory.
01:32.534  reports          | ERROR Error writing render result, No such file or directory (see console)
Caching exr file, 1920x1080, …/work/e49/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
01:32.643  render           | Saved: '…/work/e49/wA/out/f_0003.png'
VERSION 5.3.0 Beta
use_render_cache = True
render_cache_directory = '…/work/e49/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
exit code: 0
12:31:58
=== cannot be made, run B
00:00.638  image.openexr    | ERROR IMB_exr_write_end: No such file or directory.
00:00.638  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Caching exr file, 320x240, …/work/e49/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:00.705  render           | Saved: '…/work/e49/wB/out/f_0001.png'
00:00.722  image.openexr    | ERROR IMB_exr_write_end: No such file or directory.
00:00.722  reports          | ERROR Error writing render result, No such file or directory (see console)
Caching exr file, 320x240, …/work/e49/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:00.727  render           | Saved: '…/work/e49/wB/out/f_0002.png'
00:00.741  image.openexr    | ERROR IMB_exr_write_end: No such file or directory.
00:00.741  reports          | ERROR Error writing render result, No such file or directory (see console)
Caching exr file, 320x240, …/work/e49/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:00.746  render           | Saved: '…/work/e49/wB/out/f_0003.png'
VERSION 5.3.0 Beta
use_render_cache = True
render_cache_directory = '…/work/e49/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
exit code: 0
…/work/e49
…/work/e49/locked
…/work/e49/open
…/work/e49/open/cachefolder
…/work/e49/open/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
…/work/e49/ro.dmg
…/work/e49/wA
…/work/e49/wA/out
…/work/e49/wA/out/f_0001.png
…/work/e49/wA/out/f_0002.png
…/work/e49/wA/out/f_0003.png
…/work/e49/wB
…/work/e49/wB/out
…/work/e49/wB/out/f_0001.png
…/work/e49/wB/out/f_0002.png
…/work/e49/wB/out/f_0003.png
…/work/e49/wC
…/work/e49/wC/out
…/work/e49/wC/out/f_0001.png
…/work/e49/wC/out/f_0002.png
…/work/e49/wC/out/f_0003.png
"disk4" ejected.
```

**Entry 85 — what the window says when Ctrl-F12 meets a missing folder, a folder that cannot be made, and a file never saved.** Blender started with its window, with no file, from the folder `…/work/e85`; a four-megabyte disk image mounted **read-only** at `…/work/e85/locked` beforehand (a `touch` into it printed `Read-only file system`, exit code 1). Workbench, 320 × 240, frames 1 to 3, PNG. The script turned the Outliner into an Info editor, then three times set the output path, started Ctrl-F12 as the menu starts it, moved the simulated pointer, photographed the window four seconds later, and listed the folders: case A `…/work/e85/missing/a/b/f_` (missing, can be made); case B `…/work/e85/locked/newfolder/f_` (inside the read-only folder); case C `//frame_####` with the file never saved. A last picture was taken a second after case C's:

```
hdiutil create -size 4m -fs HFS+ -volname SMOOZYRO85 …/work/e85/ro.dmg
hdiutil attach -readonly -nobrowse -mountpoint …/work/e85/locked …/work/e85/ro.dmg
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --enable-event-simulate --python win_generic.py      (with STEPS=steps_e85.py, started in …/work/e85)
hdiutil detach …/work/e85/locked
```

What came back, in full:

```
WIN version 5.3.0 Beta background False event simulation True
WIN case A path '…/work/e85/missing/a/b/f_' Ctrl-F12 -> {'RUNNING_MODAL'} ; bpy.data.filepath ''
00:04.749  render           | Saved: '…/work/e85/missing/a/b/f_0001.png'
00:04.783  render           | Saved: '…/work/e85/missing/a/b/f_0002.png'
00:04.808  render           | Saved: '…/work/e85/missing/a/b/f_0003.png'
WIN screenshot …/shots/e85_case_A.png exists=True size=1122787 window=1512x862
WIN after case A, under missing: ['missing/a', 'missing/a/b', 'missing/a/b/f_0001.png', 'missing/a/b/f_0002.png', 'missing/a/b/f_0003.png']
WIN after case A, under locked: []
WIN after case A, current folder …/work/e85 holds frame files: []
WIN case B path '…/work/e85/locked/newfolder/f_' Ctrl-F12 -> {'RUNNING_MODAL'} ; bpy.data.filepath ''
00:10.135  image            | ERROR Couldn't create directory for file …/work/e85/locked/newfolder/f_0001.png: Read-only file system
00:10.146  operator         | ERROR Render error (Read-only file system) cannot save: '…/work/e85/locked/newfolder/f_0001.png'
WIN screenshot …/shots/e85_case_B.png exists=True size=1166535 window=1512x862
WIN after case B, under missing: ['missing/a', 'missing/a/b', 'missing/a/b/f_0001.png', 'missing/a/b/f_0002.png', 'missing/a/b/f_0003.png']
WIN after case B, under locked: []
WIN after case B, current folder …/work/e85 holds frame files: []
WIN case C path '//frame_####' Ctrl-F12 -> {'RUNNING_MODAL'} ; bpy.data.filepath ''
00:15.636  image            | ERROR Couldn't create directory for file frame_0001.png: No such file or directory
WIN screenshot …/shots/e85_case_C.png exists=True size=1076699 window=1512x862
WIN after case C, under missing: ['missing/a', 'missing/a/b', 'missing/a/b/f_0001.png', 'missing/a/b/f_0002.png', 'missing/a/b/f_0003.png']
WIN after case C, under locked: []
WIN after case C, current folder …/work/e85 holds frame files: []
WIN screenshot …/shots/e85_end.png exists=True size=1076704 window=1512x862
WIN quitting
00:22.644  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
```

What the pictures show, word for word where there are words:

- **Case A.** The status bar shows a progress bar reading `Rendering...` at `0%` and the frame counter at 2; the Info editor is empty. (The terminal's `Saved:` lines for all three frames came before this picture, so the picture was drawn before the window caught up.)
- **Case B.** The status bar, beside a red error mark: `Render error (Read-only file system) cannot save: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing/work/e85/locked/newfolder/f_0001.png'`. The Info editor, beside a red error mark, the same words.
- **Case C.** The status bar, beside a red error mark: `Render error (No such file or directory) cannot save: 'frame_0001.png'`. The Info editor shows case B's line and, below it, beside a red error mark, the same words as the status bar. The terminal printed only the first of its two lines for this case.
- The last picture shows the same as case C.

---

## 13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

**Answer.** Options are carried out in the order given. That decides what a script sees.
- **A setting that lands in the scene, such as `-o`.** A script placed after both the blend-file and the setting would see the new value in the scene's data. A script placed before the setting would see the file's own value. A script placed before the blend-file would see data that the file then replaces. These three are inferred from the documented order rule, from the documented fact that loading the blend-file overwrites a render output set earlier, and from the API's note that opening a file clears what a script set up before it.
- **A render option placed before the script.** The render has already run when the script starts (inferred from the documented `--render-frame` example).
- **Launch flags a script can read directly** (documented): `bpy.app.background` (`-b`), `bpy.app.factory_startup`, the `bpy.app.debug*` flags (`--debug` and `--debug-*`), `bpy.app.autoexec_override`, `bpy.app.online_access_override`.
- **Raw arguments:** anything after `--` is passed through unchanged, for scripts to read in `sys.argv`.
- **Cycles' own `--cycles-device` option** (given after `--`): Cycles reads it from `sys.argv` when the add-on registers and hands it to its compiled part. It does not store it in any scene or preference property in the Python file (observed). So whether a script reading the scene's device setting sees it is not settled.

**Label.** Documented for argument order, the `bpy.app` flags and `sys.argv`. Inferred for what a script sees in scene data. Observed for Cycles.

**Quotes.**

> "Arguments are executed in the order they are given. eg:" … "blender --background test.blend --render-frame 1 --render-output "/tmp"" … "…will not render to /tmp because --render-frame 1 renders before the output path is set." … "…will not render to /tmp because loading the blend-file overwrites the render output that was set."
> "-P, --python <filepath> … Run the given Python script file."
> "-- … End option processing, following arguments passed unchanged. Access via Python’s sys.argv."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

> "It is possible to run a script via the --python command line argument, before the blend file." … "Populating the driver name-space before the blend-file loads also doesn’t work since opening a file clears the name-space."
> "bpy.app.background … Boolean, True when blender is running without a user interface (started with -b)"
> "bpy.app.factory_startup … Boolean, True when blender is running with –factory-startup"
> "bpy.app.debug … Boolean, for debug info (started with --debug / --debug-* matching this attribute name)."
> "bpy.app.autoexec_override … The auto-execution set by the command line, None when the preference isn’t overridden (read-only)."
> "bpy.app.online_access_override … Boolean, true when internet access preference is overridden by the command line (read-only)."

https://docs.blender.org/api/5.3/bpy.app.html — read 2026-10-11

Observed, `cycles_5.3/engine.py`, functions `_configure_argument_parser` and `_parse_command_line`, lines 8–41:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
...
def _parse_command_line():
    import sys

    argv = sys.argv
    if "--" not in argv:
        return

    parser = _configure_argument_parser()
    args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])
...
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

And `cycles_5.3/__init__.py`, function `register`, line 146: `engine.init()`. `init()` in `engine.py` (line 44 onward) ends by calling `_parse_command_line()`.

What the files say: Cycles looks for `--` in `sys.argv` and reads what follows. It passes the device to `_cycles.set_device_override`, a compiled part not supplied here. It exits if that call returns false. Nothing in these lines writes to `scene.cycles.device` or to a preference. That `sys.argv` holds the whole command line, including the part before `--`, is inferred from the code searching for `--` inside it.

**Steps behind the inferences.**
- Options run in order.
- Loading the file replaces a render output set before it.
- `--render-frame` renders at its own position.
- The render output is a scene property (`RenderSettings.filepath`).
- So a script runs against whatever state the options before it have produced, and sees `-o` only if it comes after both the file and `-o`.
- None of the sources shows a script reading `-o`'s value.

**Not covered.**
- Whether `-s`, `-e`, `-j`, `-E`, `-F`, `-x`, `-t` and `-S` change stored scene values a script can read, or only affect the render. The sources do not say where each is kept.
- Whether `scene.render.filepath` shows the `-o` text with templates unexpanded (inferred yes, since templates "remain as-is in the path field").
- What `scene.cycles.device` reads after `--cycles-device`.

**Outside the interface.** This whole question is about command-line launches. The `bpy.app.background` flag is how a script tells it is running without the interface. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* every launch in entries 17 to 32 had no window; entry 84 repeated the `-o` launch with Blender's window open, and the script read the same value and the Output panel showed it.

**Operating system.** The sources do not say this differs by system. The only system-only options nearby (`-con`, `/?`, `-r`, `--qos`) are unrelated to this. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**The test file for entries 17 to 31.** Built by Blender 5.3 itself, at factory settings, no window: two scenes, `Scene` (the active one) and `Second`, with the stored output paths `/tmp/smoozy_q13/from_file_scene_` and `/tmp/smoozy_q13/from_file_second_`, and a text block named `probe` holding a small script. Nothing in these entries renders to those stored paths. The value handed to `-o` in most entries is `/tmp/smoozy_q13_sentinel_out_`, a string that appears nowhere else; it too is never written to. In every run Blender also printed its own closing lines, `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)` and `Blender quit`, with blank lines around them; those are left out of the blocks below.

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python make_file.py -- …/work/q13/q13-5.3.blend …/scripts/arch
```

printed `SAVED …/work/q13/q13-5.3.blend  blender 5.3.0 Beta  scenes ['Scene', 'Second']  texts ['probe']`.

**Entry 17 — where a value given by `-o` lands, found by searching rather than by guessing a name.** A script that assumes no property name walked every data-block in the file and everything reachable from it to a depth of five, reading only text and choice values, then the preferences the same way, then every attribute of `bpy.app`, then `sys.argv`, and printed every place whose value holds the sentinel. The command:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/q13/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P find_value.py -- /tmp/smoozy_q13_sentinel_out_
```

What came back. While it read every setting, Blender printed twenty lines of the form `bpy.rna | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'` (naming also `View3DShading`, `ToolSettings`, `FFmpegSettings`, `SpaceNodeEditor`, `SpaceDopeSheetEditor`, `SpreadsheetColumn` and `SpaceImageEditor`) and one deprecation warning for `Scene.compositing_node_group`; those are left out, and every other line is here:

```
00:00.383  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SEARCH scenes in file = ['Scene', 'Second']
SEARCH active scene = 'Scene'
SEARCH sentinel = '/tmp/smoozy_q13_sentinel_out_'
SEARCH structs visited = 2479 ; properties examined = 22072 ; text/enum values examined = 4531 ; bpy.app attributes examined = 73
SEARCH bpy.app attribute names = ['alembic', 'autoexec', 'autoexec_fail', 'autoexec_fail_message', 'autoexec_fail_quiet', 'autoexec_override', 'background', 'binary_path', 'build_branch', 'build_cflags', 'build_commit_date', 'build_commit_time', 'build_commit_timestamp', 'build_cxxflags', 'build_date', 'build_hash', 'build_linkflags', 'build_options', 'build_platform', 'build_system', 'build_time', 'build_type', 'cachedir', 'count', 'debug', 'debug_depsgraph', 'debug_depsgraph_build', 'debug_depsgraph_eval', 'debug_depsgraph_pretty', 'debug_depsgraph_tag', 'debug_depsgraph_time', 'debug_events', 'debug_freestyle', 'debug_handlers', 'debug_io', 'debug_python', 'debug_simdata', 'debug_value', 'debug_wm', 'driver_namespace', 'factory_startup', 'ffmpeg', 'handlers', 'help_text', 'icons', 'index', 'is_job_running', 'memory_usage_undo', 'module', 'n_fields', 'n_sequence_fields', 'n_unnamed_fields', 'ocio', 'oiio', 'online_access', 'online_access_override', 'opensubdiv', 'openvdb', 'portable', 'python_args', 'render_icon_size', 'render_preview_size', 'sdl', 'tempdir', 'timers', 'translations', 'usd', 'use_event_simulate', 'use_userpref_skip_save_on_exit', 'version', 'version_cycle', 'version_file', 'version_string']
HIT bpy.data.scenes['Scene'].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
HIT sys.argv[5] = '/tmp/smoozy_q13_sentinel_out_'
HIT sys.argv[9] = '/tmp/smoozy_q13_sentinel_out_'
SEARCH hits: data+preferences = 1 ; bpy.app = 0 ; sys.argv = 2
exit code: 0
```

So the value reached `bpy.data.scenes['Scene'].render.filepath` and nothing else outside `sys.argv`; the second scene in the same file did not take it.

**Entry 18 — whether a script sees `-o`, in five arrangements.** A script printing both scenes' stored output paths, the active scene and `sys.argv`, handed in five ways around the file and `-o`:

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P probe.py
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P probe.py -o /tmp/smoozy_q13_sentinel_out_
C   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -o /tmp/smoozy_q13_sentinel_out_ …/q13-5.3.blend -P probe.py
D   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -P probe.py …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_
E   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend --render-output /tmp/smoozy_q13_sentinel_out_ -P probe.py
```

What came back, in full apart from the `sys.argv` lines, which repeat each command:

```
--- A
00:00.384  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P active scene = 'Scene'

--- B
00:00.384  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13/from_file_scene_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P active scene = 'Scene'

--- C
00:00.390  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13/from_file_scene_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P active scene = 'Scene'

--- D
PROBE-P scene[Scene].render.filepath = '/tmp/'
PROBE-P active scene = 'Scene'
00:00.374  blend            | Read blend: "…/work/q13/q13-5.3.blend"

--- E
00:00.391  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P active scene = 'Scene'

```

In D the script ran before the file was loaded, and saw one scene holding the factory path `/tmp/`.

**Entry 19 — a script that ran before the file was loaded, and a `load_post` handler.** A script that prints the scenes, registers a handler to run after a file loads, and prints how many such handlers there are, handed in with `-o` before it and the file after it:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -o /tmp/smoozy_q13_sentinel_out_ -P handler.py …/q13-5.3.blend
```

What came back:

```
HANDLER before load: scenes = [('Scene', '/tmp/smoozy_q13_sentinel_out_')]
HANDLER handler registered, load_post now holds 3
00:00.419  blend            | Read blend: "…/work/q13/q13-5.3.blend"
HANDLER after load: scenes = [('Scene', '/tmp/smoozy_q13/from_file_scene_'), ('Second', '/tmp/smoozy_q13/from_file_second_')]

exit code: 0
```

**Entry 20 — which wins when both a script and `-o` set the path, and what the render writes.** A script that sets the output path to a value given in an environment variable (`…/from_script_`), and `-o` giving `…/from_flag_`, in both orders, each followed by one Workbench frame and a listing:

```
A   Q13_NEWPATH=…/work/e20/A/from_script_ "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P setpath.py -o …/work/e20/A/from_flag_ -E BLENDER_WORKBENCH -f 1
B   Q13_NEWPATH=…/work/e20/B/from_script_ "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o …/work/e20/B/from_flag_ -P setpath.py -E BLENDER_WORKBENCH -f 1
```

What came back:

```
--- A
00:00.441  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SETPATH before = '/tmp/smoozy_q13/from_file_scene_'
SETPATH after  = '…/work/e20/A/from_script_'
00:00.448  render           | Rendering single frame
00:00.448  render           | Rendering frame 1
00:00.451  render           | Start rendering: Scene, ViewLayer
00:00.451  render           | Engine: Workbench
00:00.797  render           | Saved: '…/work/e20/A/from_flag_0001.png'
00:00.797  render           | Time: 00:00.34 (Saving: 00:00.18)

from_flag_0001.png
--- B
00:00.392  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SETPATH before = '…/work/e20/B/from_flag_'
SETPATH after  = '…/work/e20/B/from_script_'
00:00.397  render           | Rendering single frame
00:00.397  render           | Rendering frame 1
00:00.402  render           | Start rendering: Scene, ViewLayer
00:00.402  render           | Engine: Workbench
00:00.785  render           | Saved: '…/work/e20/B/from_script_0001.png'
00:00.785  render           | Time: 00:00.38 (Saving: 00:00.18)

from_script_0001.png
```

**Entry 21 — the read-back guard end to end, and its exit code.** A script that compares the scene's output path with a value handed to it after `--`, and stops Blender with exit code 1 if they differ; run once with the script after `-o` and once before it:

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o …/work/e21/A/out_ -P guard.py -E BLENDER_WORKBENCH -f 1 -- …/work/e21/A/out_
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P guard.py -o …/work/e21/B/out_ -E BLENDER_WORKBENCH -f 1 -- …/work/e21/B/out_
```

What came back, with each run's exit code and folder listing:

```
--- A
00:00.395  blend            | Read blend: "…/work/q13/q13-5.3.blend"
GUARD expected = '…/work/e21/A/out_'
GUARD actual   = '…/work/e21/A/out_'
GUARD MATCH - job would proceed
00:00.401  render           | Rendering single frame
00:00.401  render           | Rendering frame 1
00:00.401  render           | Start rendering: Scene, ViewLayer
00:00.401  render           | Engine: Workbench
00:00.730  render           | Saved: '…/work/e21/A/out_0001.png'
00:00.730  render           | Time: 00:00.32 (Saving: 00:00.17)

exit code: 0
out_0001.png
--- B
00:00.389  blend            | Read blend: "…/work/q13/q13-5.3.blend"
GUARD expected = '…/work/e21/B/out_'
GUARD actual   = '/tmp/smoozy_q13/from_file_scene_'
GUARD MISMATCH - job stopped
exit code: 1
```

Run B's folder was empty.
**Entry 22 — what of the command line a script can read as text.** A script printing `sys.argv`, `bpy.app.python_args` and `bpy.app.binary_path`, run without and with `-- --my-setting 7` at the end:

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P argv.py
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P argv.py -- --my-setting 7
```

What came back:

```
--- A
00:00.382  blend            | Read blend: "…/work/q13/q13-5.3.blend"
ARGV sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/argv.py']
ARGV bpy.app.python_args = ('-I',)
ARGV bpy.app.binary_path = '/Applications/Blender 5.3b.app/Contents/MacOS/Blender'

--- B
00:00.539  blend            | Read blend: "…/work/q13/q13-5.3.blend"
ARGV sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/argv.py', '--', '--my-setting', '7']
ARGV bpy.app.python_args = ('-I',)
ARGV bpy.app.binary_path = '/Applications/Blender 5.3b.app/Contents/MacOS/Blender'

```

**Entry 23 — a path template and `#` marks handed to `-o`, as a script reads them.** A script printing the stored path, the path builder's answer for frame 7, and `bpy.path.abspath` of the stored path:

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o "/tmp/smoozy_q13_tmpl/{blend_name}_####" -P tmpl.py
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o "//renders/out_####" -P tmpl.py
```

Nothing was rendered. What came back:

```
--- A
00:00.588  blend            | Read blend: "…/work/q13/q13-5.3.blend"
TMPL filepath raw = '/tmp/smoozy_q13_tmpl/{blend_name}_####'
TMPL frame_path(frame=7) = '/tmp/smoozy_q13_tmpl/q13-5.3_0007.png'
TMPL abspath = '/tmp/smoozy_q13_tmpl/{blend_name}_####'

--- B
00:00.364  blend            | Read blend: "…/work/q13/q13-5.3.blend"
TMPL filepath raw = '//renders/out_####'
TMPL frame_path(frame=7) = '…/work/q13/renders/out_0007.png'
TMPL abspath = '…/work/q13/renders/out_####'

```

**Entry 24 — other command-line settings before and after the script.** A script printing the engine, frame range and step, format, extension switch, threads, output path and resolution; A with the settings before it, B after it, C with none:

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -E CYCLES -s 7 -e 9 -j 2 -F OPEN_EXR -x 0 -t 3 -P settings.py
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P settings.py -E CYCLES -s 7 -e 9 -j 2 -F OPEN_EXR -x 0 -t 3
C   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P settings.py
```

What came back:

```
--- A
00:00.349  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SET engine = 'CYCLES'
SET frame_start = 7  frame_end = 9  frame_step = 2
SET file_format = 'OPEN_EXR'
SET use_file_extension = False
SET threads = 3  threads_mode = 'FIXED'
SET filepath = '/tmp/smoozy_q13/from_file_scene_'
SET resolution = 1920 x 1080  percent 100

--- B
00:00.338  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SET engine = 'BLENDER_EEVEE'
SET frame_start = 1  frame_end = 250  frame_step = 1
SET file_format = 'PNG'
SET use_file_extension = True
SET threads = 3  threads_mode = 'FIXED'
SET filepath = '/tmp/smoozy_q13/from_file_scene_'
SET resolution = 1920 x 1080  percent 100

--- C
00:00.341  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SET engine = 'BLENDER_EEVEE'
SET frame_start = 1  frame_end = 250  frame_step = 1
SET file_format = 'PNG'
SET use_file_extension = True
SET threads = 8  threads_mode = 'AUTO'
SET filepath = '/tmp/smoozy_q13/from_file_scene_'
SET resolution = 1920 x 1080  percent 100

```

**Entry 25 — the five ways of handing a script in at one launch.** The same reading done by a script file (`-P`), a one-line expression (`--python-expr`), the text block `probe` inside the file (`--python-text probe`), a command a script registers (`--command`), and lines piped into Blender's own Python console (`--python-console`):

```
1   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P probe.py
2   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ --python-expr "import bpy; print('PROBE-EXPR scene[%s].render.filepath = %r' % (bpy.context.scene.name, bpy.context.scene.render.filepath))"
3   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ --python-text probe
4   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P cmdreg.py --command q13probe
5   echo "import bpy; print('PROBE-CONSOLE scene[%s].render.filepath = %r' % (bpy.context.scene.name, bpy.context.scene.render.filepath))" | "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ --python-console
```

What came back, in full:

```
--- way 1
00:00.396  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P active scene = 'Scene'
PROBE-P sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/probe.py']

--- way 2
00:00.368  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-EXPR scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'

--- way 3
00:00.366  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-TEXT scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-TEXT scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-TEXT sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '--python-text', 'probe']

--- way 4
PROBE-CMD command registered
PROBE-CMD scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-CMD scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-CMD args handed to the command = []
PROBE-CMD sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/cmdreg.py', '--command', 'q13probe']
--- way 5
00:00.380  blend            | Read blend: "…/work/q13/q13-5.3.blend"
Python 3.13.13 (main, Apr 25 2025, 12:39:20) [Clang 21.0.0 (clang-2100.1.1.101)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
(InteractiveConsole)
>>> PROBE-CONSOLE scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
>>> 
now exiting InteractiveConsole...

```

**Entry 26 — two scripts at one launch with `-o` between them.**

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P probe.py -o /tmp/smoozy_q13_sentinel_out_ -P probe2.py
```

What came back, in full:

```
00:00.371  blend            | Read blend: "…/work/q13/q13-5.3.blend"
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13/from_file_scene_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P active scene = 'Scene'
PROBE-P sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-P', '…/scripts/arch/probe.py', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/probe2.py']
PROBE-P2 scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-P2 scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-P2 active scene = 'Scene'
PROBE-P2 sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '-b', '--factory-startup', '…/work/q13/q13-5.3.blend', '-P', '…/scripts/arch/probe.py', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/probe2.py']

```

**Entry 27 — `-y` and `-Y`.** A script printing that it ran, the preference for running scripts in files, and `bpy.app.autoexec_fail`, with neither switch, with `-y` and with `-Y`:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P autoexec.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -y …/q13-5.3.blend -P autoexec.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -Y …/q13-5.3.blend -P autoexec.py
```

What came back:

```
--- none
00:00.360  blend            | Read blend: "…/work/q13/q13-5.3.blend"
AUTOEXEC script ran
AUTOEXEC preferences.filepaths.use_scripts_auto_execute = False
AUTOEXEC bpy.app.autoexec_fail = False

--- -y
00:00.355  blend            | Read blend: "…/work/q13/q13-5.3.blend"
AUTOEXEC script ran
AUTOEXEC preferences.filepaths.use_scripts_auto_execute = False
AUTOEXEC bpy.app.autoexec_fail = False

--- -Y
00:00.354  blend            | Read blend: "…/work/q13/q13-5.3.blend"
AUTOEXEC script ran
AUTOEXEC preferences.filepaths.use_scripts_auto_execute = False
AUTOEXEC bpy.app.autoexec_fail = False

```

**Entry 28 — `--cycles-device`, and whether it lands anywhere a script can read.** A script printing the scene's Cycles device and the Cycles preference for the compute device type, run with no Cycles argument, with `-- --cycles-device CPU`, and with `-- --cycles-device METAL` (`METAL` is how this Mac's graphics are reached); then the blind search of entry 17 looking for the text `METAL`, without and with the argument. The `sys.argv` lines the first script printed are left out:

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -E CYCLES -P cycdev.py
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -E CYCLES -P cycdev.py -- --cycles-device CPU
C   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -E CYCLES -P cycdev.py -- --cycles-device METAL
D   Q13_SENTINEL=METAL "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -E CYCLES -P find_value.py
E   Q13_SENTINEL=METAL "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -E CYCLES -P find_value.py -- --cycles-device METAL
```

What came back (the `bpy.rna` warnings and the `bpy.app` name list, as in entry 17, left out):

```
--- A
00:00.369  blend            | Read blend: "…/work/q13/q13-5.3.blend"
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'METAL'

--- B
00:00.371  blend            | Read blend: "…/work/q13/q13-5.3.blend"
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'METAL'

--- C
00:00.369  blend            | Read blend: "…/work/q13/q13-5.3.blend"
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'METAL'

--- D
00:00.362  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SEARCH scenes in file = ['Scene', 'Second']
SEARCH active scene = 'Scene'
SEARCH sentinel = 'METAL'
SEARCH structs visited = 2479 ; properties examined = 22072 ; text/enum values examined = 4531 ; bpy.app attributes examined = 73
HIT bpy.context.preferences.addons[5].preferences.compute_device_type = 'METAL'
HIT bpy.context.preferences.system.gpu_backend = 'METAL'
SEARCH hits: data+preferences = 2 ; bpy.app = 0 ; sys.argv = 0

--- E
00:00.360  blend            | Read blend: "…/work/q13/q13-5.3.blend"
SEARCH scenes in file = ['Scene', 'Second']
SEARCH active scene = 'Scene'
SEARCH sentinel = 'METAL'
SEARCH structs visited = 2479 ; properties examined = 22072 ; text/enum values examined = 4531 ; bpy.app attributes examined = 73
HIT bpy.context.preferences.addons[5].preferences.compute_device_type = 'METAL'
HIT bpy.context.preferences.system.gpu_backend = 'METAL'
HIT sys.argv[10] = 'METAL'
SEARCH hits: data+preferences = 2 ; bpy.app = 0 ; sys.argv = 1

```

**Entry 29 — whether add-ons are registered when a launch-time script runs.** A script printing the enabled add-ons, whether Cycles' modules are loaded, whether the scene has its Cycles settings, and the registered render engines; once after the file, once before it:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P addons.py
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -P addons.py …/q13-5.3.blend
```

What came back:

```
--- A after the file
00:00.369  blend            | Read blend: "…/work/q13/q13-5.3.blend"
ADDON enabled add-ons = ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
ADDON '_cycles' in sys.modules = True
ADDON 'cycles' in sys.modules = True
ADDON scene has cycles attr = True
ADDON registered render engines = ['CYCLES', 'HydraRenderEngine']

--- B before the file
ADDON enabled add-ons = ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
ADDON '_cycles' in sys.modules = True
ADDON 'cycles' in sys.modules = True
ADDON scene has cycles attr = True
ADDON registered render engines = ['CYCLES', 'HydraRenderEngine']
00:00.360  blend            | Read blend: "…/work/q13/q13-5.3.blend"

```

**Entry 30 — whether a script can write to the `bpy.app` flags.** `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P appwrite.py` printed:

```
00:00.361  blend            | Read blend: "…/work/q13/q13-5.3.blend"
APPWRITE bpy.app.debug write accepted, now True
APPWRITE bpy.app.background write refused: AttributeError: readonly attribute

```

**Entry 31 — whether the `bpy.app` flags report what the launch was given.** A script printing seven flags, run plain, with `--debug-python`, with `--offline-mode`, and without `--factory-startup` (with a settings folder holding no saved preferences):

```
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/q13-5.3.blend -P appflags.py
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --debug-python …/q13-5.3.blend -P appflags.py
C   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --offline-mode …/q13-5.3.blend -P appflags.py
D   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b …/q13-5.3.blend -P appflags.py
```

What came back, in full:

```
--- A
00:00.362  blend            | Read blend: "…/work/q13/q13-5.3.blend"
APPFLAG bpy.app.background = True
APPFLAG bpy.app.factory_startup = True
APPFLAG bpy.app.debug = False
APPFLAG bpy.app.debug_python = False
APPFLAG bpy.app.online_access = False
APPFLAG bpy.app.online_access_override = False
APPFLAG bpy.app.use_event_simulate = False

--- B
00:00.358  blend            | Read blend: "…/work/q13/q13-5.3.blend"
time bl_operators 0.0757
time bl_ui 0.0000
time keyingsets_builtins 0.0005
time nodeitems_builtins 0.0003
Python Script Load Time 0.0955
Warning, unregistered class: SEQUENCER_PT_custom_props(Panel)
Warning, unregistered class: NodeMenu(Menu)
Warning, unregistered class: NODE_MT_texture_node_input_base(Menu)
Warning, unregistered class: NODE_MT_texture_node_output_base(Menu)
Warning, unregistered class: NODE_MT_texture_node_color_base(Menu)
Warning, unregistered class: NODE_MT_texture_node_converter_base(Menu)
Warning, unregistered class: NODE_MT_texture_node_distort_base(Menu)
Warning, unregistered class: NODE_MT_texture_node_pattern_base(Menu)
Warning, unregistered class: NODE_MT_texture_node_texture_base(Menu)
Warning, unregistered class: SEQUENCER_MT_view_render(Menu)
bl_app_template_utils.reset('')
Extension version cache: no extensions, skipping cache data.
	addon_utils.enable bl_pkg
	addon_utils.enable io_scene_fbx
	addon_utils.enable io_curve_svg
	addon_utils.enable io_mesh_uv_layout
	addon_utils.enable io_anim_bvh
	addon_utils.enable io_scene_gltf2
	addon_utils.enable cycles
	addon_utils.enable pose_library
APPFLAG bpy.app.background = True
APPFLAG bpy.app.factory_startup = True
APPFLAG bpy.app.debug = False
APPFLAG bpy.app.debug_python = True
APPFLAG bpy.app.online_access = False
APPFLAG bpy.app.online_access_override = False
APPFLAG bpy.app.use_event_simulate = False
	addon_utils.disable bl_pkg
	addon_utils.disable io_scene_fbx
	addon_utils.disable io_curve_svg
	addon_utils.disable io_mesh_uv_layout
	addon_utils.disable io_anim_bvh
	addon_utils.disable io_scene_gltf2
	addon_utils.disable cycles
	addon_utils.disable pose_library

--- C
00:00.369  blend            | Read blend: "…/work/q13/q13-5.3.blend"
APPFLAG bpy.app.background = True
APPFLAG bpy.app.factory_startup = True
APPFLAG bpy.app.debug = False
APPFLAG bpy.app.debug_python = False
APPFLAG bpy.app.online_access = False
APPFLAG bpy.app.online_access_override = True
APPFLAG bpy.app.use_event_simulate = False

--- D
00:00.372  blend            | Read blend: "…/work/q13/q13-5.3.blend"
APPFLAG bpy.app.background = True
APPFLAG bpy.app.factory_startup = False
APPFLAG bpy.app.debug = False
APPFLAG bpy.app.debug_python = False
APPFLAG bpy.app.online_access = False
APPFLAG bpy.app.online_access_override = False
APPFLAG bpy.app.use_event_simulate = False

```

**Entry 32 — `-S` and `-o` on a file with two scenes.** A file of its own with two scenes, `Scene` (active) and `Second`, whose stored output paths are `…/work/e32/stored/from_file_scene_` and `…/work/e32/stored/from_file_second_`. Three launches, each printing both scenes' paths and the active scene with `probe.py`, then rendering one Workbench frame: A with `-S Second` before `-o`, B with `-S Second` after `-o`, C with no `-S`. It was run twice. The first file was made with `Second` holding nothing, and the two launches that render `Second` printed `ERROR No camera found in scene "Second"` and ended with exit code 0, writing nothing. The second file was the same with the first scene's camera also linked into `Second` and made its camera. The commands, the same both times with `two.blend` or `twocam.blend` and folders `A`, `B`, `C` or `A2`, `B2`, `C2`:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python make_two.py -- …/work/e32/two.blend …/work/e32/stored
A   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e32/two.blend -S Second -o …/work/e32/A/override_#### -P probe.py -E BLENDER_WORKBENCH -f 1
B   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e32/two.blend -o …/work/e32/B/override_#### -S Second -P probe.py -E BLENDER_WORKBENCH -f 1
C   "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup …/work/e32/two.blend -o …/work/e32/C/override_#### -P probe.py -E BLENDER_WORKBENCH -f 1
find …/work/e32/stored -type f
```

The first time (the `sys.argv` lines left out):

```

--- A: -S Second, then -o
00:00.372  blend            | Read blend: "…/work/e32/two.blend"
PROBE-P scene[Scene].render.filepath = '…/work/e32/stored/from_file_scene_'
PROBE-P scene[Second].render.filepath = '…/work/e32/A/override_####'
PROBE-P active scene = 'Second'
00:00.377  reports          | ERROR No camera found in scene "Second"
Scene switch for render: 'Second' in file: '…/work/e32/two.blend'

exit code: 0
--- B: -o, then -S Second
00:00.336  blend            | Read blend: "…/work/e32/two.blend"
PROBE-P scene[Scene].render.filepath = '…/work/e32/B/override_####'
PROBE-P scene[Second].render.filepath = '…/work/e32/stored/from_file_second_'
PROBE-P active scene = 'Second'
00:00.341  reports          | ERROR No camera found in scene "Second"
Scene switch for render: 'Second' in file: '…/work/e32/two.blend'

exit code: 0
--- C: no -S
00:00.384  blend            | Read blend: "…/work/e32/two.blend"
PROBE-P scene[Scene].render.filepath = '…/work/e32/C/override_####'
PROBE-P scene[Second].render.filepath = '…/work/e32/stored/from_file_second_'
PROBE-P active scene = 'Scene'
00:00.389  render           | Rendering single frame
00:00.389  render           | Rendering frame 1
00:00.389  render           | Start rendering: Scene, ViewLayer
00:00.389  render           | Engine: Workbench
00:00.727  render           | Saved: '…/work/e32/C/override_0001.png'
00:00.727  render           | Time: 00:00.33 (Saving: 00:00.17)

exit code: 0
override_0001.png
```

The second time:

```

--- A: -S Second, then -o
00:00.353  blend            | Read blend: "…/work/e32/twocam.blend"
PROBE-P scene[Scene].render.filepath = '…/work/e32/stored/from_file_scene_'
PROBE-P scene[Second].render.filepath = '…/work/e32/A2/override_####'
PROBE-P active scene = 'Second'
00:00.358  render           | Rendering single frame
00:00.358  render           | Rendering frame 1
00:00.358  render           | Start rendering: Second, ViewLayer
00:00.358  render           | Engine: Workbench
00:00.625  render           | Saved: '…/work/e32/A2/override_0001.png'
00:00.626  render           | Time: 00:00.26 (Saving: 00:00.14)
Scene switch for render: 'Second' in file: '…/work/e32/twocam.blend'

exit code: 0
override_0001.png
--- B: -o, then -S Second
00:00.355  blend            | Read blend: "…/work/e32/twocam.blend"
PROBE-P scene[Scene].render.filepath = '…/work/e32/B2/override_####'
PROBE-P scene[Second].render.filepath = '…/work/e32/stored/from_file_second_'
PROBE-P active scene = 'Second'
00:00.360  render           | Rendering single frame
00:00.360  render           | Rendering frame 1
00:00.360  render           | Start rendering: Second, ViewLayer
00:00.360  render           | Engine: Workbench
00:00.619  render           | Saved: '…/work/e32/stored/from_file_second_0001.png'
00:00.619  render           | Time: 00:00.25 (Saving: 00:00.12)
Scene switch for render: 'Second' in file: '…/work/e32/twocam.blend'

exit code: 0
--- C: no -S
00:00.360  blend            | Read blend: "…/work/e32/twocam.blend"
PROBE-P scene[Scene].render.filepath = '…/work/e32/C2/override_####'
PROBE-P scene[Second].render.filepath = '…/work/e32/stored/from_file_second_'
PROBE-P active scene = 'Scene'
00:00.364  render           | Rendering single frame
00:00.364  render           | Rendering frame 1
00:00.368  render           | Start rendering: Scene, ViewLayer
00:00.368  render           | Engine: Workbench
00:00.662  render           | Saved: '…/work/e32/C2/override_0001.png'
00:00.662  render           | Time: 00:00.29 (Saving: 00:00.16)

exit code: 0
override_0001.png
…/work/e32/stored/from_file_second_0001.png
```

**Entry 84 — the same launch with a real window open, and what the Output panel shows.** Blender 5.3 started **with** its window, using the window settings folder described under question 6 (splash screen off), given the two-scene test file, `-o /tmp/smoozy_q13_sentinel_out_` and a script that prints both scenes' paths, `sys.argv`, `bpy.app.background` and the number of windows, turns the Properties editor to its Output tab, takes Blender's own screenshot on a six-second timer, and quits:

```
Q13_SHOT=…/shots/e84_mac-win-5.3.png Q13_DELAY=6 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" …/work/q13/q13-5.3.blend -o /tmp/smoozy_q13_sentinel_out_ -P winprobe.py
```

What came back, in full:

```
00:00.587  blend            | Read blend: "…/work/q13/q13-5.3.blend"
WIN scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
WIN scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
WIN sys.argv = ['/Applications/Blender 5.3b.app/Contents/MacOS/Blender', '…/work/q13/q13-5.3.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '…/scripts/arch/winprobe.py']
WIN bpy.app.background = False
WIN windows open = 1
WIN properties editors switched to the Output tab at load: 1
00:06.999  blend            | Saved session recovery to "…/tmp/quit.blend"
WIN properties editors switched to the Output tab on the timer: 1
WIN Blender's own screenshot: …/shots/e84_mac-win-5.3.png exists=True size=1174852
Blender quit
exit code: 0
```

The picture shows the Properties editor on its Output tab, and the path field at the top of the **Output** panel reads `/tmp/smoozy_q13_sentinel_out_`, the same text the script printed for the active scene.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

**Answer.** Not in the sources, for the timing itself. The sources do not say whether one frame's files are completely written before the next frame begins rendering, whether they can overlap, or in what order a frame's own files are written (main output, File Output node files, cache).

What they do say:
- frames are "saved directly to disk as they are rendered"
- a script hook runs "directly after the frame is written"
- frame-change hooks run during rendering
- `-j` steps "forward after each rendered frame"
- render statistics carry a per-frame "saving time"

None of these settles how one frame's writes relate in time to the next frame's.

**Label.** Not in the sources (for the question asked). The facts listed above are documented.

**Quotes (the nearest documented facts).**

> "animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."

https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11

> "render_write … on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"
> "frame_change_pre … Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame."
> "render_stats … on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11

> "-j, --frame-jump <frames> … Set number of frames to step forward after each rendered frame."

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11

**Where I looked.**
- Read in full: the handlers page, the rendering animations page, the Output properties page, the File Output node page, the API pages for `RenderEngine` (`render_frame_finish`, `end_result`, `update_result`) and `RenderSettings`; the Cycles files `engine.py` and `__init__.py`.
- Searched the render, compositing and command-line pages, those API pages, the 5.3 release notes and the Cycles Python files for: "next frame", "following frame", "previous frame", "asynchron", "in parallel", "background thread", "separate thread", "while the next", "overlap", "concurrent", "as they are rendered", "after rendering", "when rendering is finished/complete".
- Hits were about motion vectors, plane tracking, the DLSS viewport denoiser and the interface's double buffer. None is about frame writes.

**Not covered.** The whole timing question. Also whether `render_write` runs for File Output node files or for video frames.

**Outside the interface.** The sources do not say. The handlers apply to rendering in general (inferred); the handler page does not separate background from interface renders. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window, in every route tried — Cycles and EEVEE, PNG and OpenEXR, with Placeholders, a File Output node, a preview picture, two views, a list of frames out of order, Cycles' tile file and a video (entries 39, 51, 61, 64 to 75) — each frame's files were finished before the next frame's render began, and the next frame's `render_pre` came after the previous frame's `render_write`. With Blender's window open the order was the same (entry 86).

**Operating system.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. The earlier versions' trace of every file call, made on Linux, has no macOS equivalent this sitting could run without changing a system setting.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 39 — the order of one frame's events against the next frame's.** Written out in full under question 16. In every animation route — from Python, with a File Output node, with the node alone, to a video, with Placeholders on, and from `-a` — the lines for each frame ran `render_pre`, Blender's own `Saved:` (or `Video append frame N`), `render_post`, `render_write`, and only then the next frame's `render_pre`. With Placeholders on, the next frame's empty file was already on disk at its `render_pre`, at 0 bytes.

**Entry 51 — four frames of an animation, watched on disk.** Written out in full under question 11. Cycles, 400 samples, 1920 × 1080, PNG, Placeholders off: each frame's file appeared at 0 bytes and finished growing before the next frame's file appeared — the last growth of `f_0001.png` at 1792229596.612418 and the first sight of `f_0002.png` at 1792229611.856035 — and the watcher found no file changing size after a later file had appeared.

**Entry 61 — a File Output node's file and the scene's own file, frame by frame.** Written out in full under question 11. Cycles, 400 samples, 1920 × 1080, frames 1 to 4: for every frame the node's file appeared and finished first, the scene's file second, and both were finished before the next frame's node file appeared — for example `n_Image0001.png` last changed at 1792231517.026837, `f_0001.png` appeared at 1792231517.047980 and last changed at 1792231517.435167, and `n_Image0002.png` appeared at 1792231534.780673. The watcher found no picture file changing after a later one appeared.

**How entries 64 to 73 and 86 were run.** One script sets every scene setting from environment variables and hangs a line on each of Blender's render events — a render starting (`render_init`), a frame's render beginning (`render_pre`) and finishing (`render_post`), a frame announced as written (`render_write`), the render completing or being cancelled, and the frame changing (`frame_change_pre`, `frame_change_post`) — each line stamped with the computer's clock to the microsecond. Blender's default scene, Cycles on the processor, 1 sample, denoising off, at factory settings with no window. Where a folder is watched, it is watched as described under question 11, from before Blender starts to after it exits, on the same clock. The script is `w14_run.py` of `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`, unchanged.

**Entry 64 — the order Blender itself reports, with no window.** 512 × 512, PNG, frames 1 to 5:

```
W14_OUT=…/work/e64/out W14_FMT=PNG W14_RES=512 W14_SAMPLES=1 W14_FSTART=1 W14_FEND=5 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

What came back, in full:

```
W14 1792232187.957778 BLENDER 5.3.0 Beta hash 5d8151335b6b
W14 1792232187.957858 PLATFORM darwin
W14 1792232187.957870 TEMPDIR …/tmp/blender_OmriS7/
W14 1792232187.958123 FORMAT PNG chosen from: AVIF,JPEG,OPEN_EXR,PNG,WEBP,BMP,CINEON,DPX,IRIS,JPEG2000,HDR,TARGA,TARGA_RAW,TIFF,OPEN_EXR_MULTILAYER,FFMPEG
W14 1792232187.958158 SETTINGS fmt PNG res 512 samples 1 frames 1 5 overwrite True placeholder False fileout False preview False stereo False exr_codec NONE
W14 1792232187.958173 RENDER-START
EVT 1792232187.959407 render_init frame=1
EVT 1792232187.959588 render_pre frame=1
EVT 1792232187.959865 frame_change_pre frame=1
EVT 1792232187.960450 frame_change_post frame=1
00:01.029  render           | Saved: '…/work/e64/out/frame_0001.png'
EVT 1792232188.105239 render_post frame=1
EVT 1792232188.105273 render_write frame=1
EVT 1792232188.105303 render_pre frame=2
EVT 1792232188.105402 frame_change_pre frame=2
EVT 1792232188.105704 frame_change_post frame=2
00:01.055  render           | Saved: '…/work/e64/out/frame_0002.png'
EVT 1792232188.131296 render_post frame=2
EVT 1792232188.131325 render_write frame=2
EVT 1792232188.131354 render_pre frame=3
EVT 1792232188.131434 frame_change_pre frame=3
EVT 1792232188.131698 frame_change_post frame=3
00:01.082  render           | Saved: '…/work/e64/out/frame_0003.png'
EVT 1792232188.158482 render_post frame=3
EVT 1792232188.158513 render_write frame=3
EVT 1792232188.158547 render_pre frame=4
EVT 1792232188.158636 frame_change_pre frame=4
EVT 1792232188.159072 frame_change_post frame=4
00:01.112  render           | Saved: '…/work/e64/out/frame_0004.png'
EVT 1792232188.188273 render_post frame=4
EVT 1792232188.188304 render_write frame=4
EVT 1792232188.188335 render_pre frame=5
EVT 1792232188.188418 frame_change_pre frame=5
EVT 1792232188.188767 frame_change_post frame=5
00:01.146  render           | Saved: '…/work/e64/out/frame_0005.png'
EVT 1792232188.222423 render_post frame=5
EVT 1792232188.222459 render_write frame=5
EVT 1792232188.222470 render_complete frame=1
EVT 1792232188.222527 frame_change_pre frame=1
EVT 1792232188.222545 frame_change_post frame=1
W14 1792232188.222589 RENDER-END
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
```

From each frame's `render_write` to the next frame's `render_pre`: +0.000030, +0.000029, +0.000034 and +0.000031 seconds. No frame's events came after the next frame's `render_pre`.

**Entry 65 — the disk itself, with a write slow enough to catch in the act.** As entry 64, but each frame a 3000 × 3000 OpenEXR, 32-bit, uncompressed — 144 megabytes — with the output folder watched:

```
W14_OUT=…/work/e65/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=5 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

Blender's lines, in full:

```
W14 1792232198.138511 BLENDER 5.3.0 Beta hash 5d8151335b6b
W14 1792232198.138588 PLATFORM darwin
W14 1792232198.138602 TEMPDIR …/tmp/blender_Y6PItQ/
W14 1792232198.138833 FORMAT OPEN_EXR chosen from: AVIF,JPEG,OPEN_EXR,PNG,WEBP,BMP,CINEON,DPX,IRIS,JPEG2000,HDR,TARGA,TARGA_RAW,TIFF,OPEN_EXR_MULTILAYER,FFMPEG
W14 1792232198.138874 SETTINGS fmt OPEN_EXR res 3000 samples 1 frames 1 5 overwrite True placeholder False fileout False preview False stereo False exr_codec NONE
W14 1792232198.138889 RENDER-START
EVT 1792232198.139017 render_init frame=1
EVT 1792232198.139130 render_pre frame=1
EVT 1792232198.139168 frame_change_pre frame=1
EVT 1792232198.139475 frame_change_post frame=1
00:01.578  render           | Saved: '…/work/e65/out/frame_0001.exr'
EVT 1792232199.246676 render_post frame=1
EVT 1792232199.246718 render_write frame=1
EVT 1792232199.246759 render_pre frame=2
EVT 1792232199.246863 frame_change_pre frame=2
EVT 1792232199.247195 frame_change_post frame=2
00:03.282  render           | Saved: '…/work/e65/out/frame_0002.exr'
EVT 1792232200.950525 render_post frame=2
EVT 1792232200.950580 render_write frame=2
EVT 1792232200.950621 render_pre frame=3
EVT 1792232200.950738 frame_change_pre frame=3
EVT 1792232200.951700 frame_change_post frame=3
00:04.426  render           | Saved: '…/work/e65/out/frame_0003.exr'
EVT 1792232202.094788 render_post frame=3
EVT 1792232202.094820 render_write frame=3
EVT 1792232202.094849 render_pre frame=4
EVT 1792232202.094934 frame_change_pre frame=4
EVT 1792232202.095264 frame_change_post frame=4
00:05.399  render           | Saved: '…/work/e65/out/frame_0004.exr'
EVT 1792232203.067332 render_post frame=4
EVT 1792232203.067366 render_write frame=4
EVT 1792232203.067398 render_pre frame=5
EVT 1792232203.067491 frame_change_pre frame=5
EVT 1792232203.067799 frame_change_post frame=5
00:06.407  render           | Saved: '…/work/e65/out/frame_0005.exr'
EVT 1792232204.075733 render_post frame=5
EVT 1792232204.075769 render_write frame=5
EVT 1792232204.075777 render_complete frame=1
EVT 1792232204.075847 frame_change_pre frame=1
EVT 1792232204.075865 frame_change_post frame=1
W14 1792232204.075922 RENDER-END
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
```

The watcher's summary, how many readings fell inside each file's write, and its first two and last two raw lines for the first frame:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001.exr                                       1792232198.955054     106770    108 1792232199.248575    144048850 -
frame_0002.exr                                       1792232200.346286       4946    176 1792232200.952636    144048850 -
frame_0003.exr                                       1792232201.852556    1487122     91 1792232202.096264    144048850 -
frame_0004.exr                                       1792232202.830184          0     87 1792232203.069359    144048850 -
frame_0005.exr                                       1792232203.828987          0     89 1792232204.076480    144048850 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
frame_0001.exr: 109 readings over 0.294 s, mean gap 0.0027 s
frame_0002.exr: 177 readings over 0.606 s, mean gap 0.0034 s
frame_0003.exr: 92 readings over 0.244 s, mean gap 0.0027 s
frame_0004.exr: 88 readings over 0.239 s, mean gap 0.0027 s
frame_0005.exr: 90 readings over 0.247 s, mean gap 0.0028 s
1792232198.955054 NEW   …/work/e65/out/frame_0001.exr 106770
1792232198.958272 GROW  …/work/e65/out/frame_0001.exr 475410
…
1792232199.245933 GROW  …/work/e65/out/frame_0001.exr 143900946
1792232199.248575 GROW  …/work/e65/out/frame_0001.exr 144048850
```

No name other than `frame_000N.exr` appeared in the folder. The watcher's reading that first showed each file at its final size came 1.9, 2.1, 1.4, 2.0 and 0.7 milliseconds after that frame's `render_write` line; the reading before it, a fraction of a millisecond before `render_write`, still showed the file short of its final size (for frame 1, 143,900,946 bytes at 1792232199.245933 against `render_write` at 1792232199.246718). A reading is taken every 2.7 milliseconds or so, so these two readings do not say on which side of `render_write` the last bytes went in; entry 66 asks that directly.

**Entry 66 — the file at the moment Blender announces it written.** Written out in full under question 16. Reading each 144,048,850-byte frame back inside its own `render_write` event found every byte there, for all five frames, before the next frame's `render_pre`.

**Entry 67 — Placeholders on and Overwrite off, in time.** As entry 65, with `W14_PLACEHOLDER=1 W14_OVERWRITE=0`, into an empty watched folder:

```
W14_OUT=…/work/e67/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=5 W14_PLACEHOLDER=1 W14_OVERWRITE=0 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

The watcher's summary:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001.exr                                       1792232257.496739          0     76 1792232258.457067    144048850 -
frame_0002.exr                                       1792232258.459570          0     76 1792232259.447812    144048850 -
frame_0003.exr                                       1792232259.447812          0     78 1792232260.275114    144048850 -
frame_0004.exr                                       1792232260.275114          0     73 1792232261.305761    144048850 -
frame_0005.exr                                       1792232261.305761          0     74 1792232262.073557    144048850 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

Blender's lines from the start of the render to the end (the script's own opening lines left out):

```
W14 1792232257.494550 RENDER-START
EVT 1792232257.495501 render_init frame=1
EVT 1792232257.496217 render_pre frame=1
EVT 1792232257.496329 frame_change_pre frame=1
EVT 1792232257.496665 frame_change_post frame=1
00:01.404  render           | Saved: '…/work/e67/out/frame_0001.exr'
EVT 1792232258.457460 render_post frame=1
EVT 1792232258.457491 render_write frame=1
EVT 1792232258.457648 render_pre frame=2
EVT 1792232258.457736 frame_change_pre frame=2
EVT 1792232258.458060 frame_change_post frame=2
00:02.394  render           | Saved: '…/work/e67/out/frame_0002.exr'
EVT 1792232259.447311 render_post frame=2
EVT 1792232259.447344 render_write frame=2
EVT 1792232259.447480 render_pre frame=3
EVT 1792232259.447568 frame_change_pre frame=3
EVT 1792232259.447923 frame_change_post frame=3
00:03.221  render           | Saved: '…/work/e67/out/frame_0003.exr'
EVT 1792232260.274492 render_post frame=3
EVT 1792232260.274522 render_write frame=3
EVT 1792232260.274676 render_pre frame=4
EVT 1792232260.274770 frame_change_pre frame=4
EVT 1792232260.275266 frame_change_post frame=4
00:04.251  render           | Saved: '…/work/e67/out/frame_0004.exr'
EVT 1792232261.304929 render_post frame=4
EVT 1792232261.304963 render_write frame=4
EVT 1792232261.305131 render_pre frame=5
EVT 1792232261.305229 frame_change_pre frame=5
EVT 1792232261.305625 frame_change_post frame=5
00:05.020  render           | Saved: '…/work/e67/out/frame_0005.exr'
EVT 1792232262.073219 render_post frame=5
EVT 1792232262.073252 render_write frame=5
EVT 1792232262.073260 render_complete frame=1
EVT 1792232262.073317 frame_change_pre frame=1
EVT 1792232262.073332 frame_change_post frame=1
W14 1792232262.073365 RENDER-END
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
```

So frame 1's empty file appeared 0.5 milliseconds after frame 1's `render_pre` and sat at 0 bytes until its picture was written into it; each later frame's empty file appeared in the reading after the previous frame's file reached its final size, which was also just after that frame's `render_write` and the next frame's `render_pre`. Only one empty file stood in the folder at a time, and `render_write` fired once per frame.

**Entry 68 — a File Output node's file against the main output file, in time.** As entry 65, three frames, with a File Output node added by the script — fed from the Render Layers node's first output, aimed at a second folder `…/node` — and both folders watched together:

```
W14_OUT=…/work/e68/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=3 W14_FILEOUT=1 W14_FILEOUTDIR=…/work/e68/node "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

The watcher's summary:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
Unsaved0001.exr                                      1792232276.441359     315577     85 1792232276.673431    144048761 -
frame_0001.exr                                       1792232276.673431    1151250     91 1792232276.921271    144048850 -
Unsaved0002.exr                                      1792232277.745401    1495225     81 1792232277.987128    144048761 -
frame_0002.exr                                       1792232277.987128      45330     75 1792232278.188674    144048850 -
Unsaved0003.exr                                      1792232279.018452          0     85 1792232279.255067    144048761 -
frame_0003.exr                                       1792232279.255067     721170     81 1792232279.536288    144048850 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

Blender's lines from the node's set-up to the end, in full apart from deprecation warnings:

```
W14 1792232275.529420 FILEOUT route scene.compositing_node_group
W14 1792232275.529705 FILEOUT directory …/work/e68/node/
W14 1792232275.529883 NOTE fileout format TypeError('bpy_struct: item.attr = val: enum "OPEN_EXR" not found in (\'OPEN_EXR_MULTILAYER\')')
W14 1792232275.530125 FILEOUT item added; inputs: W14Image,
W14 1792232275.530403 FILEOUT node made CompositorNodeOutputFile inputs now 2
W14 1792232275.530423 SETTINGS fmt OPEN_EXR res 3000 samples 1 frames 1 3 overwrite True placeholder False fileout True preview False stereo False exr_codec NONE
W14 1792232275.530436 RENDER-START
EVT 1792232275.530696 render_init frame=1
EVT 1792232275.530823 render_pre frame=1
EVT 1792232275.530855 frame_change_pre frame=1
EVT 1792232275.531108 frame_change_post frame=1
00:01.555  render           | Saved: '…/work/e68/node/Unsaved0001.exr'
00:01.803  render           | Saved: '…/work/e68/out/frame_0001.exr'
EVT 1792232276.919546 render_post frame=1
EVT 1792232276.919589 render_write frame=1
EVT 1792232276.920085 render_pre frame=2
EVT 1792232276.920278 frame_change_pre frame=2
EVT 1792232276.920740 frame_change_post frame=2
00:02.870  render           | Saved: '…/work/e68/node/Unsaved0002.exr'
00:03.071  render           | Saved: '…/work/e68/out/frame_0002.exr'
EVT 1792232278.187964 render_post frame=2
EVT 1792232278.188025 render_write frame=2
EVT 1792232278.188100 render_pre frame=3
EVT 1792232278.188294 frame_change_pre frame=3
EVT 1792232278.188813 frame_change_post frame=3
00:04.137  render           | Saved: '…/work/e68/node/Unsaved0003.exr'
00:04.420  render           | Saved: '…/work/e68/out/frame_0003.exr'
EVT 1792232279.536607 render_post frame=3
EVT 1792232279.536638 render_write frame=3
EVT 1792232279.536646 render_complete frame=1
EVT 1792232279.536769 frame_change_pre frame=1
EVT 1792232279.536784 frame_change_post frame=1
W14 1792232279.536901 RENDER-END
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
exit code: 0
```

The script's attempt to give the node plain OpenEXR was refused (`enum "OPEN_EXR" not found in ('OPEN_EXR_MULTILAYER')`), so the node wrote multi-layer OpenEXR, and with its name field as a new node has it, its files were named `Unsaved0001.exr` and on — the file had never been saved. For every frame the node's file appeared and finished first, and the main file appeared in the same reading as the node's last change and finished second; Blender's `Saved:` lines for both came before that frame's `render_write`, and the watcher's reading showing the main file at its final size fell within about a millisecond and a half of `render_write` (1.7 ms after it for frame 1, 0.6 ms after for frame 2, 0.3 ms before for frame 3). The next frame's files appeared only after that.

**Entry 69 — the preview picture saved beside a frame.** As entry 65, three frames, with `image_settings.use_preview` on (`W14_PREVIEW=1`):

```
W14_OUT=…/work/e69/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=3 W14_PREVIEW=1 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

The watcher's summary, then Blender's `Saved:` lines and the `render_write` lines:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001.exr                                       1792232310.611344     430354    423 1792232312.802709    144048850 -
frame_0001.jpg                                       1792232312.892076          0     21 1792232312.977735       236358 -
frame_0002.exr                                       1792232313.540437      86290    373 1792232315.475883    144048850 -
frame_0002.jpg                                       1792232315.542979       4096     23 1792232315.638635       236358 -
frame_0003.exr                                       1792232316.179423     962834     76 1792232316.381168    144048850 -
frame_0003.jpg                                       1792232316.447536          0     11 1792232316.476505       236358 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
00:03.338  render           | Saved: '…/work/e69/out/frame_0001.exr'
00:03.511  render           | Saved: '…/work/e69/out/frame_0001.jpg'
EVT 1792232312.975771 render_write frame=1
00:06.016  render           | Saved: '…/work/e69/out/frame_0002.exr'
00:06.173  render           | Saved: '…/work/e69/out/frame_0002.jpg'
EVT 1792232315.637293 render_write frame=2
00:06.915  render           | Saved: '…/work/e69/out/frame_0003.exr'
00:07.011  render           | Saved: '…/work/e69/out/frame_0003.jpg'
EVT 1792232316.475788 render_write frame=3
```

For each frame the main OpenEXR was written first and the `.jpg` preview second, both before the next frame's file appeared. Blender printed a `Saved:` line for each, then `render_write`.

**Entry 70 — two views in one frame.** As entry 65, three frames, with multi-view on, the stereo set-up, and each view saved to its own file (`W14_STEREO=1`):

```
W14_OUT=…/work/e70/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=3 W14_STEREO=1 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

The watcher's summary, then Blender's `Saved:` lines and the `render_write` lines:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001_L.exr                                     1792232320.082713          0    260 1792232321.298223    144048850 -
frame_0001_R.exr                                     1792232321.298223    1200402     78 1792232321.503371    144048850 -
frame_0002_L.exr                                     1792232322.812267          0     77 1792232323.015336    144048850 -
frame_0002_R.exr                                     1792232323.015336     946450    319 1792232324.615869    144048850 -
frame_0003_L.exr                                     1792232325.852084          0    490 1792232328.527148    144048850 -
frame_0003_R.exr                                     1792232328.530736        850    301 1792232330.031424    144048850 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
00:02.832  render           | Saved: '…/work/e70/out/frame_0001_L.exr'
00:03.039  render           | Saved: '…/work/e70/out/frame_0001_R.exr'
EVT 1792232321.503295 render_write frame=1
00:04.549  render           | Saved: '…/work/e70/out/frame_0002_L.exr'
00:06.150  render           | Saved: '…/work/e70/out/frame_0002_R.exr'
EVT 1792232324.614115 render_write frame=2
00:10.064  render           | Saved: '…/work/e70/out/frame_0003_L.exr'
00:11.566  render           | Saved: '…/work/e70/out/frame_0003_R.exr'
EVT 1792232330.030686 render_write frame=3
```

The left view was written first, then the right, both before the next frame's left view appeared; the right view's file first appeared in the same reading as the left view's last change.

**Entry 71 — a list of frames given out of order.** A script that only sets the scene up (Cycles on the processor, 1 sample, 800 × 800) and renders nothing; then Blender's own `-f` with the frames 5, 1 and 3, in that order, into a watched empty folder:

```
W14_RES=800 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_setup.py -o …/work/e71/out/f_#### -F OPEN_EXR -x 1 -f 5,1,3
```

The watcher's summary, then Blender's lines without the per-step progress lines (`Fra: N | Mem: … | Updating …` and the like, about forty per frame):

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
f_0005.exr                                           1792232345.829612          0      4 1792232345.852836       307167 -
f_0001.exr                                           1792232345.894793          0      5 1792232345.918708       307167 -
f_0003.exr                                           1792232345.973534        852      7 1792232346.016280       307167 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
W14 1792232345.769223 SETUP-ONLY engine=CYCLES res=800 samples=1
00:00.338  render           | Rendering single frame
00:00.339  render           | Rendering frame 5
00:00.339  render           | Start rendering: Scene, ViewLayer
00:00.339  render           | Engine: Cycles
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.426  render           | Saved: '…/work/e71/out/f_0005.exr'
00:00.426  render           | Time: 00:00.08 (Saving: 00:00.02)
00:00.426  render           | Rendering single frame
00:00.426  render           | Rendering frame 1
00:00.426  render           | Start rendering: Scene, ViewLayer
00:00.428  render           | Engine: Cycles
00:00.501  render           | Saved: '…/work/e71/out/f_0001.exr'
00:00.501  render           | Time: 00:00.07 (Saving: 00:00.03)
00:00.501  render           | Rendering single frame
00:00.501  render           | Rendering frame 3
00:00.501  render           | Start rendering: Scene, ViewLayer
00:00.505  render           | Engine: Cycles
00:00.587  render           | Saved: '…/work/e71/out/f_0003.exr'
00:00.587  render           | Time: 00:00.08 (Saving: 00:00.04)
exit code: 0
```

The frames were rendered and written in the order given — 5, then 1, then 3 — each finished before the next appeared.

**Entry 72 — Cycles' tile file against the frame's own file, and whether the tile file is gone before the frame file appears.** As entry 65, three frames, at 4000 × 4000 with tile size 256 (so Cycles works in tiles), the output folder and Blender's temporary folder watched together; and beside that watcher, a second small program that loops with no pause and in every pass looks at the output folder first and the temporary folder second, so that any pass finding a new frame file and then still finding a tile file would show the frame file had appeared before the tile file went. It made about 19,000 passes a second:

```
c2watch.py 120 …/tmp …/work/e72/out &
W14_OUT=…/work/e72/out W14_FMT=OPEN_EXR W14_RES=4000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=3 W14_TILE=256 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

The first watcher's summary, its `GONE` lines, and Blender's lines about the temporary folder, the tiles and each frame:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
quit.blend                                           1792232357.185909      95581      0 1792232357.185909        95581 -
cycles-tile-buffer-72403-39007243248-0-0.exr         1792232358.396162       8192    289 1792232360.051220      9564578 1792232360.258450
frame_0001.exr                                       1792232360.268836          0    698 1792232364.339717    256064850 -
cycles-tile-buffer-72403-39007243248-1-0.exr         1792232364.362910          0    282 1792232366.060322      9564578 1792232366.217897
frame_0002.exr                                       1792232366.217897          0    706 1792232369.982064    256064850 -
cycles-tile-buffer-72403-39007243248-2-0.exr         1792232370.008511       8192    279 1792232371.746963      9564578 1792232371.894913
frame_0003.exr                                       1792232371.897849     225362    722 1792232375.625791    256064850 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
1792232360.258450 GONE  …/tmp/blender_zlTKxv/cycles-tile-buffer-72403-39007243248-0-0.exr
1792232366.217897 GONE  …/tmp/blender_zlTKxv/cycles-tile-buffer-72403-39007243248-1-0.exr
1792232371.894913 GONE  …/tmp/blender_zlTKxv/cycles-tile-buffer-72403-39007243248-2-0.exr
W14 1792232358.352256 TEMPDIR …/tmp/blender_zlTKxv/
W14 1792232358.352426 TILE set True 256
EVT 1792232358.356374 render_pre frame=1
00:06.337  render           | Saved: '…/work/e72/out/frame_0001.exr'
EVT 1792232364.346539 render_write frame=1
EVT 1792232364.351114 render_pre frame=2
00:11.987  render           | Saved: '…/work/e72/out/frame_0002.exr'
EVT 1792232369.988734 render_write frame=2
EVT 1792232369.988773 render_pre frame=3
00:17.626  render           | Saved: '…/work/e72/out/frame_0003.exr'
EVT 1792232375.627878 render_write frame=3
exit code: 0
```

The second program's output, in full — each line is a pass at which what it saw changed:

```
1792232357.181864 pass 1 frames=() tiles=()
1792232358.395791 pass 22553 frames=() tiles=('cycles-tile-buffer-72403-39007243248-0-0.exr',)
1792232360.261269 pass 45461 frames=() tiles=()
1792232360.266345 pass 45462 frames=('frame_0001.exr',) tiles=()
1792232364.363166 pass 104554 frames=('frame_0001.exr',) tiles=('cycles-tile-buffer-72403-39007243248-1-0.exr',)
1792232366.216166 pass 126173 frames=('frame_0001.exr',) tiles=()
1792232366.220652 pass 126174 frames=('frame_0001.exr', 'frame_0002.exr') tiles=()
1792232370.006892 pass 179932 frames=('frame_0001.exr', 'frame_0002.exr') tiles=('cycles-tile-buffer-72403-39007243248-2-0.exr',)
1792232371.895519 pass 200669 frames=('frame_0001.exr', 'frame_0002.exr') tiles=()
1792232371.895884 pass 200671 frames=('frame_0001.exr', 'frame_0002.exr', 'frame_0003.exr') tiles=()
PASSES 2324501 in 120.0 seconds; passes that saw a new frame file and a tile file together: 0
```

So each frame's tile file was written while that frame rendered, finished, and was removed; at least one pass of the second program then saw neither the tile file nor the frame's file, and the frame's file appeared after that — 5.1 milliseconds after the tile file was last seen for frame 1, 4.5 for frame 2, 0.4 for frame 3. No pass saw a new frame file with a tile file still present, and no two frames' tile files were present at once. The first watcher alone stamped frame 2's tile file going and `frame_0002.exr` appearing with the same time, 1792232366.217897, and could not have told the two apart.

**Entry 73 — the same run with EEVEE.** As entry 65, with the engine set to EEVEE (the script picks whichever EEVEE name is offered and prints the list):

```
W14_OUT=…/work/e73/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=5 W14_ENGINE=EEVEE "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
```

The watcher's summary, then Blender's engine line, `Saved:` lines and `render_pre` and `render_write` lines:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001.exr                                       1792232504.719043          0    397 1792232506.791403    144048661 -
frame_0002.exr                                       1792232508.582209     655445    254 1792232509.701885    144048661 -
frame_0003.exr                                       1792232511.431166      21141    326 1792232513.041944    144048661 -
frame_0004.exr                                       1792232514.850457          0    436 1792232517.321356    144048661 -
frame_0005.exr                                       1792232519.190224          0    317 1792232520.860357    144048661 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
W14 1792232502.727309 EEVEE resolved to BLENDER_EEVEE available: BLENDER_EEVEE
EVT 1792232502.727757 render_pre frame=1
00:04.426  render           | Saved: '…/work/e73/out/frame_0001.exr'
EVT 1792232506.789205 render_write frame=1
EVT 1792232506.789237 render_pre frame=2
00:07.340  render           | Saved: '…/work/e73/out/frame_0002.exr'
EVT 1792232509.704135 render_write frame=2
EVT 1792232509.704168 render_pre frame=3
00:10.678  render           | Saved: '…/work/e73/out/frame_0003.exr'
EVT 1792232513.041228 render_write frame=3
EVT 1792232513.041302 render_pre frame=4
00:14.956  render           | Saved: '…/work/e73/out/frame_0004.exr'
EVT 1792232517.319865 render_write frame=4
EVT 1792232517.319916 render_pre frame=5
00:18.499  render           | Saved: '…/work/e73/out/frame_0005.exr'
EVT 1792232520.867196 render_write frame=5
exit code: 0
```

No file grew after a later file had appeared, and every frame's `render_write` came before the next frame's `render_pre`.

**Entry 74 — a video's frames over the run.** Written out in full under question 15. With forty frames going into one lossless video file, the file grew as the frames were written — 67,108,864 bytes at the 20th frame's `render_write`, 145,489,920 at the 40th — and took its last 7,698,769 bytes in five more changes after the last frame's `render_write`.

**Entry 75 — a video render stopped part way.** Written out in full under question 15. A Matroska H.264 render stopped with the terminate signal after 1257 `Video append frame` lines left a file a decoder could take 1153 frames from; a lossless render interrupted after 30 `render_write` lines left a file a decoder took 30 frames from.

**Entry 86 — frame order with Blender's own window open.** Blender 5.3 started **with** its window (the window settings folder described under question 6, no event simulation), and a script that sets the scene up as entry 65 (Cycles on the processor, 1 sample, 3000 × 3000 uncompressed OpenEXR, frames 1 to 5), stamps every render event, starts the render the way the Render Animation menu entry starts it (`render.render('INVOKE_DEFAULT', animation=True)`), and quits Blender at a fixed 25 seconds; the output folder watched throughout:

```
W14_OUT=…/work/e86/out W14_FMT=OPEN_EXR W14_RES=3000 W14_SAMPLES=1 W14_FSTART=1 W14_FEND=5 W14_QUIT_AFTER=25 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --python w14_window_fixed.py
```

What came back, in full, and the watcher's summary:

```
W14 1792233468.189044 BLENDER 5.3.0 Beta hash 5d8151335b6b
W14 1792233468.190923 PLATFORM darwin background False
W14 1792233468.190953 WINDOWS 1
W14 1792233468.191366 SETTINGS OPEN_EXR 3000 1 1 5
W14 1792233468.191577 RENDER-START-INVOKE
EVT 1792233468.214989 render_init frame=1
W14 1792233468.215044 INVOKE-RETURNED
EVT 1792233468.215750 render_pre frame=1
EVT 1792233468.218610 frame_change_pre frame=1
EVT 1792233468.219241 frame_change_post frame=1
00:03.568  render           | Saved: '…/work/e86/out/frame_0001.exr'
EVT 1792233471.173037 render_post frame=1
EVT 1792233471.173078 render_write frame=1
EVT 1792233471.173117 render_pre frame=2
EVT 1792233471.174108 frame_change_pre frame=2
EVT 1792233471.176864 frame_change_post frame=2
00:06.055  render           | Saved: '…/work/e86/out/frame_0002.exr'
EVT 1792233473.659542 render_post frame=2
EVT 1792233473.659602 render_write frame=2
EVT 1792233473.659700 render_pre frame=3
EVT 1792233473.660632 frame_change_pre frame=3
EVT 1792233473.661139 frame_change_post frame=3
00:07.709  render           | Saved: '…/work/e86/out/frame_0003.exr'
EVT 1792233475.313883 render_post frame=3
EVT 1792233475.313921 render_write frame=3
EVT 1792233475.313950 render_pre frame=4
EVT 1792233475.314068 frame_change_pre frame=4
EVT 1792233475.314339 frame_change_post frame=4
00:09.608  render           | Saved: '…/work/e86/out/frame_0004.exr'
EVT 1792233477.212759 render_post frame=4
EVT 1792233477.212801 render_write frame=4
EVT 1792233477.212839 render_pre frame=5
EVT 1792233477.217570 frame_change_pre frame=5
EVT 1792233477.222047 frame_change_post frame=5
00:11.600  render           | Saved: '…/work/e86/out/frame_0005.exr'
EVT 1792233479.205019 render_post frame=5
EVT 1792233479.205104 render_write frame=5
EVT 1792233479.208781 render_complete frame=1
EVT 1792233479.215611 frame_change_pre frame=1
EVT 1792233479.217597 frame_change_post frame=1
W14 1792233493.196621 RENDER-END
00:25.617  blend            | Saved session recovery to "…/tmp/quit.blend"
Blender quit
exit code: 0
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001.exr                                       1792233469.089148          0    389 1792233471.173502    144048850 -
frame_0002.exr                                       1792233471.816455      28946    346 1792233473.657173    144048850 -
frame_0003.exr                                       1792233474.249957     114962    242 1792233475.316717    144048850 -
frame_0004.exr                                       1792233475.866974     127250    265 1792233477.212562    144048850 -
frame_0005.exr                                       1792233477.887733          0    246 1792233479.205042    144048850 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
```

`background False` and `WINDOWS 1` say the window was there, and `INVOKE-RETURNED` printed straight after `render_init` says the render ran alongside the window rather than holding the script. From each frame's `render_write` to the next frame's `render_pre`: +0.000039, +0.000098, +0.000029 and +0.000038 seconds. No file grew after a later file had appeared.

---

## 15. While a render is writing an animation as a single video file, what is in that file at each point of the run, and what decides that?

**Answer.** Not in the sources, for the file's contents at each point of the run, and for what decides them.

What they do say:
- Video output "encodes each frame into a video container" and makes "one file that holds all the frames".
- If a problem happens while rendering, "the file might become unplayable" and must be re-rendered from the start. From that I infer that the file exists and is incomplete during the run.
- With Autosplit on, output is split into more files once the first reaches 2 GiB.
- Keyframe interval and B-frame settings influence "file size and seekability". That says nothing about what is in the file during the run.

**Label.** Not in the sources (for the question asked). The facts listed above are documented, except that the file exists during the run, which is inferred.

**Quotes.**

> "Video: Encodes each frame into a video container."
> "Autosplit Output … If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."
> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11

> "Blender creates one file that holds all the frames of your animation."

https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11

> "use_autosplit … Autosplit output at 2GB boundary (default False)" / "gopsize … Distance between key frames, also known as GOP size; influences file size and seekability"

https://docs.blender.org/api/5.3/bpy.types.FFmpegSettings.html — read 2026-10-11

**Step behind the inference.** "If a problem occurs while rendering, the file might become unplayable" can only be true if a file is already there while rendering is under way. So the file exists during the run and is not finished until the run ends. What it holds at any point is not said.

**Where I looked.**
- Read in full: Output properties (including Encoding), rendering animations, render output introduction, video formats, and the `FFmpegSettings` and `ImageFormatSettings` API pages.
- Searched the whole manual, API docs and 5.3 release notes for: "unplayable", "playable", "moov", "finaliz", "trailer", "flush", "append frame", "writing frame", "partially written", "partial file", "incomplete file", "incomplete video", "container header", "file header". The only relevant hit was the "might become unplayable" tip already quoted.
- I also looked for how the video file is named. I searched for frame-range naming such as "0001-0250" and "####-####", and for "start … end frame … name". Nothing about video naming was found.
- Nothing describes the file's contents over time.

**Not covered.** What is in the file at the start, middle and end of a run. Whether it can be played partway through. When container information is written. What happens on cancel versus crash. How the video file is named. The sequencer's 5.3 note about "seeking to codec key frames less" is about playback, so I ruled it out.

**Outside the interface.** The sources do not say this differs. The tip about watching encoding output in the System Console, and running `blender -d` to see more, is the only outside-related note: "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d." (https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11). The `--debug-ffmpeg` option "Enable debug messages from FFmpeg video input and output" also exists (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11). *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window: the one file is named for the frame range, present from the first frame at 0 bytes, and grows as frames are added — for a lossless video, about half its final size by the half-way frame and the rest within about 40 milliseconds of the last frame (entries 10, 74). Stopped with the terminate signal, a Matroska H.264 file had no duration and a decoder took 1153 frames of the 1257 Blender had appended; interrupted, a lossless file had a duration and a decoder took all 30 frames written; Blender's own movie loader read 0 frames from the stopped H.264 file (entry 75). What Blender's window shows during a video render was not looked at.

**Operating system.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine. The two-gigabyte split was not run (it would have left about six gigabytes this sitting could not delete).

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 10 — the name of a single video file, and how the video format is reached.** An animation of frames 1 to 5, Workbench at 64 × 64, with the media type set to Video, the format FFmpeg and the container Matroska, wrote one file named `mov_0001-0005.mkv` for the output path `…/plain/mov_`, and one named `mov_##.mkv` for the output path `…/hashes/mov_##.mkv`, printing `Video append frame 1` to `Video append frame 5` for each and no `Saved:` line. Setting the format to `'FFMPEG'` without first setting the media type to Video was refused. Both are written out in full under questions 8 and 6.

**Entry 74 — does the one video file grow as the run goes?** At factory settings, no window: Blender's default scene, Cycles on the processor, 1 sample, 1920 × 1920, frames 1 to 40, written as one video — the media type set to Video, then the FFmpeg format, the codec set to the lossless HUFFYUV (so the file is far larger than any write buffer), the container left as it comes — with the output folder watched every two milliseconds and every render event stamped, as described under question 14:

```
W14_OUT=…/work/e74/out W14_FMT=FFMPEG W14_FFCODEC=HUFFYUV W14_RES=1920 W14_SAMPLES=1 W14_FSTART=1 W14_FEND=40 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
ls -l …/work/e74/out
```

The script's lines about the format, and the 20th and 40th frames' lines:

```
W14 1792232539.641415 MEDIA TYPE set to VIDEO; file_format now FFMPEG
W14 1792232539.641651 FFMPEG codec set HUFFYUV container MKV
00:04.152  render           | Video append frame 20
EVT 1792232543.431307 render_write frame=20
00:07.825  render           | Video append frame 40
EVT 1792232547.107632 render_write frame=40
exit code: 0
```

The watcher's summary, its first lines, its lines either side of the 20th frame's `render_write`, and its last lines:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
frame_0001-0040.mkv                                  1792232539.659944          0     80 1792232547.149007    153188689 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
WATCH start 1792232538.424191 interval=0.002 dirs=…/work/e74/out
1792232539.659944 NEW   …/work/e74/out/frame_0001-0040.mkv 0
1792232540.262737 GROW  …/work/e74/out/frame_0001-0040.mkv 3670016
1792232540.435827 GROW  …/work/e74/out/frame_0001-0040.mkv 4194304
…
1792232543.048700 GROW  …/work/e74/out/frame_0001-0040.mkv 61079552
1792232543.229263 GROW  …/work/e74/out/frame_0001-0040.mkv 61865984
1792232543.232003 GROW  …/work/e74/out/frame_0001-0040.mkv 65011712
1792232543.428631 GROW  …/work/e74/out/frame_0001-0040.mkv 67108864
1792232543.433101 GROW  …/work/e74/out/frame_0001-0040.mkv 68681728
…
1792232547.097309 GROW  …/work/e74/out/frame_0001-0040.mkv 142606336
1792232547.105680 GROW  …/work/e74/out/frame_0001-0040.mkv 145489920
1792232547.113626 GROW  …/work/e74/out/frame_0001-0040.mkv 148897792
1792232547.123938 GROW  …/work/e74/out/frame_0001-0040.mkv 149159936
1792232547.132970 GROW  …/work/e74/out/frame_0001-0040.mkv 150994944
1792232547.135647 GROW  …/work/e74/out/frame_0001-0040.mkv 153092096
1792232547.149007 GROW  …/work/e74/out/frame_0001-0040.mkv 153188689
total 329216
-rw-r--r--@ 1 artist     wheel  153188689 Oct 17 13:22 frame_0001-0040.mkv
```

Read against Blender's clock: the file, `frame_0001-0040.mkv`, appeared at 0 bytes about 1.2 seconds after the render started and changed size 80 times. At the 20th frame's `render_write` it held 67,108,864 bytes; at the 40th, 145,489,920; after the 40th, it changed five more times, to its final 153,188,689 bytes, 41 milliseconds later.

**Entry 75 — a video render stopped part way: what the file holds, and what a copy taken mid-run holds.** Two runs. Both used a media reader already installed on the Mac, `/opt/homebrew/bin/ffprobe`, once to read the container and once with `-count_frames` to make a decoder walk the file; run A also handed each file to Blender itself (`bpy.data.movieclips.load`).

Run A — at factory settings, no window: Workbench, 1280 × 720, frames 1 to 4000, one video, Matroska with H.264. Blender started directly in the background; at a fixed 12 seconds the growing file was copied; at a fixed 25 seconds Blender was sent the ordinary terminate signal:

```
TMPDIR=… BLENDER_USER_RESOURCES=… "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e14_video.py -- …/work/e75A/out 4000 > …/blender.log 2>&1 &
B=$!; sleep 12; cp …/out/movie_0001-4000.mkv …/copy_at_12s.mkv; ls -l …/copy_at_12s.mkv
sleep 13; kill $B; wait $B; echo "blender exit code: $?"
ffprobe -v error -show_entries format=format_name,duration,size -show_entries stream=codec_name,nb_frames -of default=noprint_wrappers=1 <file>
ffprobe -v error -count_frames -select_streams v:0 -show_entries stream=nb_read_frames -of default=noprint_wrappers=1 <file>
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python e14_probe.py -- <file>
```

What came back, in full:

```
media_type = VIDEO  file_format = FFMPEG  container = MKV  codec = H264
frame_path(frame=1) = '…/work/e75A/out/movie_0001-4000.mkv'
-rw-r--r--@ 1 artist     wheel  524288 Oct 17 13:23 …/work/e75A/copy_at_12s.mkv
…/scripts/run_e75A.sh: line 7: 77312 Terminated: 15          TMPDIR=$W/tmp BLENDER_USER_RESOURCES=$W/res-5.3 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python $W/scripts/new/e14_video.py -- $E/out 4000 > $E/blender.log 2>&1
blender exit code: 143
total 2560
-rw-r--r--@ 1 artist     wheel  1310720 Oct 17 13:23 movie_0001-4000.mkv
=== …/work/e75A/copy_at_12s.mkv
codec_name=h264
nb_frames=N/A
format_name=matroska,webm
duration=N/A
size=524288
ffprobe exit 0
[matroska,webm @ 0xb21014000] File ended prematurely
nb_read_frames=468
ffprobe count exit 0
loaded as a movie: copy_at_12s.mkv  frames: 0  size: (256, 256)
=== …/work/e75A/out/movie_0001-4000.mkv
codec_name=h264
nb_frames=N/A
format_name=matroska,webm
duration=N/A
size=1310720
ffprobe exit 0
[matroska,webm @ 0xaf2810000] File ended prematurely
nb_read_frames=1153
ffprobe count exit 0
loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)
(count of `Video append frame` lines: 1257; the last five:)
00:24.516  render           | Video append frame 1253
00:24.533  render           | Video append frame 1254
00:24.556  render           | Video append frame 1255
00:24.571  render           | Video append frame 1256
00:24.587  render           | Video append frame 1257
```

The folder was watched through the run:

```
file                                                 first_seen            at_size  grows last_change         final_bytes gone
movie_0001-4000.mkv                                  1792232574.930088          0      5 1792232598.006651      1310720 -
OVERLAP -- a file's size changed strictly after a later file first appeared:
  none
1792232574.930088 NEW   …/work/e75A/out/movie_0001-4000.mkv 0
1792232580.770152 GROW  …/work/e75A/out/movie_0001-4000.mkv 262144
1792232585.559704 GROW  …/work/e75A/out/movie_0001-4000.mkv 524288
1792232589.661916 GROW  …/work/e75A/out/movie_0001-4000.mkv 786432
1792232593.855024 GROW  …/work/e75A/out/movie_0001-4000.mkv 1048576
1792232598.006651 GROW  …/work/e75A/out/movie_0001-4000.mkv 1310720
```

So the file was there under its final name, `movie_0001-4000.mkv` — the name `frame_path` had given for frame 1 — from 1.2 seconds in, at 0 bytes, and grew in steps of 262,144 bytes. Blender had printed `Video append frame` for 1257 frames when it was stopped. The file left behind, and the copy taken at 12 seconds, were each read by `ffprobe` as Matroska with H.264 but no duration, with `File ended prematurely`; its decoder got 1153 frames out of the file left behind and 468 out of the copy. Blender's own movie loader reported 0 frames for both.

Run B — as entry 74 (Cycles, 1920 × 1920, one lossless HUFFYUV video, frames 1 to 40), with Blender started directly in the background and sent an interrupt (`kill -INT`) at a fixed 6 seconds:

```
W14_OUT=…/work/e75B/out W14_FMT=FFMPEG W14_FFCODEC=HUFFYUV W14_RES=1920 W14_SAMPLES=1 W14_FSTART=1 W14_FEND=40 TMPDIR=… BLENDER_USER_RESOURCES=… "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py > …/blender.log 2>&1 &
B=$!; sleep 6; kill -INT $B; wait $B; echo "blender exit code: $?"
ls -l …/work/e75B/out; grep -c "render_write" …/blender.log
ffprobe … (the same two lines as run A)
```

What came back:

```
blender exit code: 1
total 230912
-rw-r--r--@ 1 artist     wheel  114891699 Oct 17 13:23 frame_0001-0040.mkv
30
codec_name=huffyuv
nb_frames=N/A
format_name=matroska,webm
duration=1.250000
size=114891699
ffprobe exit 0
nb_read_frames=30
ffprobe count exit 0
```

and the end of Blender's lines (the `frame_change` and `Video append frame` lines left out; there were 30 `Video append frame` lines):

```
EVT 1792232618.130168 render_post frame=28
EVT 1792232618.130198 render_write frame=28
EVT 1792232618.130214 render_pre frame=29
EVT 1792232618.301908 render_post frame=29
EVT 1792232618.301936 render_write frame=29
EVT 1792232618.301952 render_pre frame=30
EVT 1792232618.474497 render_post frame=30
EVT 1792232618.474526 render_write frame=30
EVT 1792232618.474543 render_pre frame=31
EVT 1792232618.629682 render_cancel frame=1
W14 1792232618.629811 RENDER-END
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Sent an internal break event. Press ^C again to kill Blender
Blender quit
```

So after the interrupt Blender printed `Sent an internal break event`, the render was cancelled at frame 31 (`render_cancel`), the script's own last line ran, and Blender ended with exit code 1; the file it left read with a duration of 1.25 seconds and gave up 30 frames to a decoder, the same number as the 30 `render_write` lines.

---

## 16. Under what conditions does a render report that a file has been written, and what is on disk at the moment it reports it?

**Answer.** The one documented report is the `render_write` script hook. It runs "on writing a render frame", "directly after the frame is written". So by the documentation, when it runs, that frame has been written.

The sources do not say:
- whether it runs for still renders saved with `write_still`, for File Output node files, for each frame of a video, or for placeholder files
- whether "written" means the data has reached the storage device rather than only having been handed to the system

No console or Info-editor message about a saved file is documented. The `render_stats` hook carries a "saving time", but it is a progress string, not a report that a file exists.

**Label.** Documented for the `render_write` hook's timing. Not in the sources for the rest.

**Quotes.**

> "render_write … on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"
> "render_stats … on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11

> "info: Information about devices, files, configuration, operations"

https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (under `--log-level`) — read 2026-10-11. This says logging at "info" level covers files in general. It does not say a message is logged when a render file is written.

**Where I looked.**
- Read in full: the handlers page, the Info editor page, the command-line arguments page (logging options), the Output properties page, the `RenderEngine` page (`report`, `update_stats`, `end_result`).
- Searched the whole manual, API docs, 5.3 release notes and Cycles Python files for: "Saved:", "Saved" (as a whole word), "Saving:", "saving time", "Fra:", "Append frame", "Writing frame", "written to disk", "saved to disk", "is written", "has been written", "been saved", "after the frame is written". Hits outside the two hook descriptions were about crash logs, keymaps, File Output inputs "will be written to disk", unsaved images and blend-file saving; none describes a report that a render file was written.
- The only hits on a render file being written are the `render_write` and `render_stats` descriptions.
- Cycles' denoise tool checks frame files with `os.path.isfile` (question 8). That is a later check, not a render's report, so I ruled it out.

**Not covered.**
- The conditions for `render_write` beyond "writing a render frame".
- The state of the file on disk when the hook runs, beyond "written": whether it is complete, flushed, or renamed into place.
- Any printed message.
- Whether File Output node writes or video frames trigger the hook.

**Outside the interface.** Script hooks are part of the Python API and are not described as interface-only. That they work with `-b` renders is inferred; it is not stated. *Brought up to date by the hand tests of 2026-10-17 (observed, on the Creator's Mac; the entries are written out below):* with no window: `render_write` fired once per frame for animations from Python and from `-a`, for a File Output node writing alone, and for each frame of a video (while the video file stood at 0 bytes); once for a still saved with `write_still` and for `-f 1`; not for a still not asked to save, nor for a still written only by a File Output node (entry 39). At each `render_write` the frame's file was complete, every byte readable (entry 66), and a File Output node's file for that frame was already finished (entry 68). With Blender's window open, `render_write` still came before the next frame began (entry 86).

**Operating system.** The sources do not say. *Brought up to date by the hand tests of 2026-10-17:* settled for macOS on Apple silicon only, by the runs below on the Creator's Mac (macOS 26.6.2). Linux and Windows were not run for 5.3 — this sitting had neither machine.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**LABEL: observed.** Run on 2026-10-17 on **the Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — in **Blender 5.3.0 Beta** (hash `5d8151335b6b`, built 2026-10-16), started from a terminal at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run had a temporary folder and a settings folder of its own, so nobody's own Blender settings were read or written; runs with no window used `-b --factory-startup`. In the output below, `…` stands for the working folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Writing`, outside this project; nothing else in any quoted line is changed. Another Blender, not started by these tests, was running on the Mac from before the first entry until about 12:35 that day — through entry 52 — so no time below is a clean time. **This settles macOS on Apple silicon only; Linux and Windows were not run.** The entry numbers are those of `Audits/Blender Research/Writing/Hand tests — 5.3.md`, which holds every command word for word.

**Entry 39 — which writes make Blender announce a written frame, and what is on disk when it does.** At factory settings, no window, Workbench at 64 × 64, frames 1 to 3, PNG, the output path `<route folder>/main_`. A script hung a small reporter on five of Blender's render events — a frame's render beginning (`render_pre`), finishing (`render_post`), being announced as written (`render_write`), the whole render completing, and a render being cancelled — and each report lists every file under the route's folder with its size at that moment. Ten routes: from Python, a still not asked to save; a still asked to save (`write_still=True`); an animation; an animation with a File Output node writing PNG beside the main output; the same with Save Output off, so the node is the only writer; a still with the node and not asked to save; an animation to a single video file (Matroska); an animation with Placeholders on and Overwrite off; and from the command line, a single frame (`-f 1`) and an animation (`-a`) started after the script had only set the scene up. The commands:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python writes.py -- …/work/e39 <route>     (for each of the eight Python routes)
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python writes.py -f 1 -- …/work/e39 cli
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python writes.py -a -- …/work/e39/cli_a cli
```

What came back, in full (one deprecation warning for `Scene.compositing_node_group`, printed by the three routes that make a node, is left out). Each `on disk` list is file name and size in bytes:

```
=== still_no_save
ROUTE still_no_save
EVT render_pre      frame=1 on disk=[]
EVT render_post     frame=1 on disk=[]
EVT render_complete frame=1 on disk=[]
returned: {'FINISHED'}
FINAL on disk: []
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== still_save
ROUTE still_save
EVT render_pre      frame=1 on disk=[]
00:00.622  render           | Saved: '…/work/e39/still_save/main_.png'
EVT render_post     frame=1 on disk=[('main_.png', 4074)]
EVT render_write    frame=1 on disk=[('main_.png', 4074)]
EVT render_complete frame=1 on disk=[('main_.png', 4074)]
returned: {'FINISHED'}
FINAL on disk: [('main_.png', 4074)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== anim
ROUTE anim
EVT render_pre      frame=1 on disk=[]
00:00.845  render           | Saved: '…/work/e39/anim/main_0001.png'
EVT render_post     frame=1 on disk=[('main_0001.png', 4074)]
EVT render_write    frame=1 on disk=[('main_0001.png', 4074)]
EVT render_pre      frame=2 on disk=[('main_0001.png', 4074)]
00:00.851  render           | Saved: '…/work/e39/anim/main_0002.png'
EVT render_post     frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
EVT render_write    frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
EVT render_pre      frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
00:00.858  render           | Saved: '…/work/e39/anim/main_0003.png'
EVT render_post     frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
EVT render_write    frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
EVT render_complete frame=1 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
returned: {'FINISHED'}
FINAL on disk: [('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== anim_node_and_main
ROUTE anim_node_and_main
EVT render_pre      frame=1 on disk=[]
00:00.603  render           | Saved: '…/work/e39/anim_node_and_main/node/n_Image0001.png'
00:00.604  render           | Saved: '…/work/e39/anim_node_and_main/main_0001.png'
EVT render_post     frame=1 on disk=[('main_0001.png', 4074), ('node/n_Image0001.png', 3478)]
EVT render_write    frame=1 on disk=[('main_0001.png', 4074), ('node/n_Image0001.png', 3478)]
EVT render_pre      frame=2 on disk=[('main_0001.png', 4074), ('node/n_Image0001.png', 3478)]
00:00.612  render           | Saved: '…/work/e39/anim_node_and_main/node/n_Image0002.png'
00:00.613  render           | Saved: '…/work/e39/anim_node_and_main/main_0002.png'
EVT render_post     frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478)]
EVT render_write    frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478)]
EVT render_pre      frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478)]
00:00.620  render           | Saved: '…/work/e39/anim_node_and_main/node/n_Image0003.png'
00:00.621  render           | Saved: '…/work/e39/anim_node_and_main/main_0003.png'
EVT render_post     frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
EVT render_write    frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
EVT render_complete frame=1 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
returned: {'FINISHED'}
FINAL on disk: [('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074), ('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== anim_node_only
ROUTE anim_node_only
save_output = False
EVT render_pre      frame=1 on disk=[]
00:00.636  render           | Saved: '…/work/e39/anim_node_only/node/n_Image0001.png'
EVT render_post     frame=1 on disk=[('node/n_Image0001.png', 3478)]
EVT render_write    frame=1 on disk=[('node/n_Image0001.png', 3478)]
EVT render_pre      frame=2 on disk=[('node/n_Image0001.png', 3478)]
00:00.643  render           | Saved: '…/work/e39/anim_node_only/node/n_Image0002.png'
EVT render_post     frame=2 on disk=[('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478)]
EVT render_write    frame=2 on disk=[('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478)]
EVT render_pre      frame=3 on disk=[('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478)]
00:00.652  render           | Saved: '…/work/e39/anim_node_only/node/n_Image0003.png'
EVT render_post     frame=3 on disk=[('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
EVT render_write    frame=3 on disk=[('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
EVT render_complete frame=1 on disk=[('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
returned: {'FINISHED'}
FINAL on disk: [('node/n_Image0001.png', 3478), ('node/n_Image0002.png', 3478), ('node/n_Image0003.png', 3478)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== still_node_only
ROUTE still_node_only
EVT render_pre      frame=1 on disk=[]
00:00.800  render           | Saved: '…/work/e39/still_node_only/node/n_Image.png'
EVT render_post     frame=1 on disk=[('node/n_Image.png', 3478)]
EVT render_complete frame=1 on disk=[('node/n_Image.png', 3478)]
returned: {'FINISHED'}
FINAL on disk: [('node/n_Image.png', 3478)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== anim_video
ROUTE anim_video
media_type = VIDEO file_format = FFMPEG
EVT render_pre      frame=1 on disk=[('main_0001-0003.mkv', 0)]
00:01.123  render           | Video append frame 1
EVT render_post     frame=1 on disk=[('main_0001-0003.mkv', 0)]
EVT render_write    frame=1 on disk=[('main_0001-0003.mkv', 0)]
EVT render_pre      frame=2 on disk=[('main_0001-0003.mkv', 0)]
00:01.132  render           | Video append frame 2
EVT render_post     frame=2 on disk=[('main_0001-0003.mkv', 0)]
EVT render_write    frame=2 on disk=[('main_0001-0003.mkv', 0)]
EVT render_pre      frame=3 on disk=[('main_0001-0003.mkv', 0)]
00:01.137  render           | Video append frame 3
EVT render_post     frame=3 on disk=[('main_0001-0003.mkv', 0)]
EVT render_write    frame=3 on disk=[('main_0001-0003.mkv', 0)]
EVT render_complete frame=1 on disk=[('main_0001-0003.mkv', 1455)]
returned: {'FINISHED'}
FINAL on disk: [('main_0001-0003.mkv', 1455)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== anim_placeholder
ROUTE anim_placeholder
EVT render_pre      frame=1 on disk=[('main_0001.png', 0)]
00:01.327  render           | Saved: '…/work/e39/anim_placeholder/main_0001.png'
EVT render_post     frame=1 on disk=[('main_0001.png', 4074)]
EVT render_write    frame=1 on disk=[('main_0001.png', 4074)]
EVT render_pre      frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 0)]
00:01.348  render           | Saved: '…/work/e39/anim_placeholder/main_0002.png'
EVT render_post     frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
EVT render_write    frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
EVT render_pre      frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 0)]
00:01.358  render           | Saved: '…/work/e39/anim_placeholder/main_0003.png'
EVT render_post     frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
EVT render_write    frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
EVT render_complete frame=1 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
returned: {'FINISHED'}
FINAL on disk: [('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

exit code: 0
=== cli -f 1
ROUTE cli
set up only; the render is started by the command line after this script
FINAL on disk: []
00:00.578  render           | Rendering single frame
00:00.578  render           | Rendering frame 1
EVT render_pre      frame=1 on disk=[]
00:00.579  render           | Start rendering: Scene, ViewLayer
00:00.579  render           | Engine: Workbench
00:01.006  render           | Saved: '…/work/e39/cli/main_0001.png'
00:01.006  render           | Time: 00:00.42 (Saving: 00:00.11)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
EVT render_post     frame=1 on disk=[('main_0001.png', 4074)]
EVT render_write    frame=1 on disk=[('main_0001.png', 4074)]
EVT render_complete frame=1 on disk=[('main_0001.png', 4074)]

exit code: 0
=== cli -a
ROUTE cli
set up only; the render is started by the command line after this script
FINAL on disk: []
00:00.661  render           | Rendering animation (frames 1..3)
00:00.661  render           | Rendering frame 1
EVT render_pre      frame=1 on disk=[]
00:00.662  render           | Start rendering: Scene, ViewLayer
00:00.662  render           | Engine: Workbench
00:00.932  render           | Saved: '…/work/e39/cli_a/cli/main_0001.png'
00:00.932  render           | Time: 00:00.27 (Saving: 00:00.08)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
EVT render_post     frame=1 on disk=[('main_0001.png', 4074)]
EVT render_write    frame=1 on disk=[('main_0001.png', 4074)]
00:00.932  render           | Rendering frame 2
EVT render_pre      frame=2 on disk=[('main_0001.png', 4074)]
00:00.932  render           | Start rendering: Scene, ViewLayer
00:00.932  render           | Engine: Workbench
00:00.937  render           | Saved: '…/work/e39/cli_a/cli/main_0002.png'
00:00.937  render           | Time: 00:00.00 (Saving: 00:00.00)
EVT render_post     frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
EVT render_write    frame=2 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
00:00.937  render           | Rendering frame 3
EVT render_pre      frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074)]
00:00.938  render           | Start rendering: Scene, ViewLayer
00:00.938  render           | Engine: Workbench
00:00.943  render           | Saved: '…/work/e39/cli_a/cli/main_0003.png'
00:00.943  render           | Time: 00:00.00 (Saving: 00:00.00)
EVT render_post     frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
EVT render_write    frame=3 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]
EVT render_complete frame=1 on disk=[('main_0001.png', 4074), ('main_0002.png', 4074), ('main_0003.png', 4074)]

exit code: 0
```

Counting the `render_write` lines: none for the still not asked to save; one for the still asked to save; one per frame for the animation, for the animation with a node beside the main output, for the node writing alone, for the video, and for Placeholders on; none for the still written only by the node; one for `-f 1`; one per frame for `-a`. At every `render_write` line that has a picture file, that frame's file was already on disk at its final size — and so were the node's file for that frame, where there was one. For the video, the one file stood at 0 bytes at each frame's `render_write` and at 1455 bytes when the render completed.

**Entry 66 — is the file finished at the moment Blender says it is written?** As entry 65 (3000 × 3000 uncompressed OpenEXR, frames 1 to 5), with the `render_write` line also asking Blender for that frame's file name (`scene.render.frame_path(frame=scene.frame_current)`), reading the size the file system reports, and reading the whole file and counting its bytes, inside the same event:

```
W14_OUT=…/work/e66/out W14_FMT=OPEN_EXR W14_RES=3000 W14_EXRCODEC=NONE W14_SAMPLES=1 W14_FSTART=1 W14_FEND=5 W14_READBACK=1 "/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python w14_run.py
grep readback …/blender.log; ls -l …/work/e66/out
```

What came back:

```
EVT 1792232242.172949 readback frame=1 stat=144048850 read=144048850 path=…/work/e66/out/frame_0001.exr
EVT 1792232243.235942 readback frame=2 stat=144048850 read=144048850 path=…/work/e66/out/frame_0002.exr
EVT 1792232244.483482 readback frame=3 stat=144048850 read=144048850 path=…/work/e66/out/frame_0003.exr
EVT 1792232245.427027 readback frame=4 stat=144048850 read=144048850 path=…/work/e66/out/frame_0004.exr
EVT 1792232246.541741 readback frame=5 stat=144048850 read=144048850 path=…/work/e66/out/frame_0005.exr
total 1406760
-rw-r--r--@ 1 artist     wheel  144048850 Oct 17 13:17 frame_0001.exr
-rw-r--r--@ 1 artist     wheel  144048850 Oct 17 13:17 frame_0002.exr
-rw-r--r--@ 1 artist     wheel  144048850 Oct 17 13:17 frame_0003.exr
-rw-r--r--@ 1 artist     wheel  144048850 Oct 17 13:17 frame_0004.exr
-rw-r--r--@ 1 artist     wheel  144048850 Oct 17 13:17 frame_0005.exr
```

At every `render_write`, the size reported and the bytes read were 144,048,850, the size each file had when the run ended.

**Entry 68 — the announcement against a File Output node's file.** Written out in full under question 14. With a File Output node writing beside the main output, 3000 × 3000 OpenEXR, each frame's `render_write` line came after Blender's `Saved:` lines for both the node's file and the main file. The watcher saw the node's file finished well before it; the reading that first showed the main file at its final size came just after it — for frame 1, `Unsaved0001.exr` last changed at 1792232276.673431, `render_write` at 1792232276.919589, and the reading showing `frame_0001.exr` at its final size at 1792232276.921271, 1.7 milliseconds later, about one reading's interval.

---

## Hand tests

*Pruned 2026-10-17.* All fifteen items that stood here were run on the Creator's Mac on 2026-10-17, together with every test the earlier versions carry a result for, and each result is written under the question it answers above as an observed finding; the list they were run from is `Audits/Blender Research/Writing/Hand tests — 5.3.md`. What is still open, and why:

1. **Anything on Windows** — the Windows halves of the old items 1, 4 and 6: an output path under `C:\`, `%ERRORLEVEL%` after a failed render, a folder denied through its security settings, and a scene named `a:b*c?` rendered on Windows (questions 3, 4, 5, 10, 12). No Windows machine was used; when one is, its edition has to be read and named.
2. **Anything on Linux**, including the trace of every file call a render makes (questions 11, 14). No Linux machine was used.
3. **A folder refused by its own permissions** (`chmod 555`), the first half of the old item 4 (questions 5, 12). The tool the tests ran in refused `chmod`; the tests were run against a read-only volume instead, which refuses for a different reason.
4. **A render on an NVIDIA or AMD card** (old item 8 asked for the graphics card; it was run on the Mac's own graphics only).
5. **A video passing two gigabytes, on the Mac** (question 15). Not run: it writes about six gigabytes, which the tests could not have deleted afterwards.
6. **The words of a tooltip, as shown on screen, and a File Output node's own panels in the node editor's sidebar** (questions 2, 3, 6, 8, 9). No tooltip appeared under a simulated pointer, and the sidebar could not be turned to the node's tab by script; both need a real pointer.
7. **Two Blenders rendering into one folder at once, and a folder shared over a network** (question 11). One Blender at a time was run, on one machine.

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Writing, Blender 5.3
